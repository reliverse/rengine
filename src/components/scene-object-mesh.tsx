import { TransformControls } from "@react-three/drei";
import { useFrame } from "@react-three/fiber";
import { memo, useCallback, useEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import { type Color, Mesh, MeshStandardMaterial } from "three";
import { useMaterialStore } from "~/stores/material-store";
import type { SceneObject } from "~/stores/scene-store";
import { useSceneStore } from "~/stores/scene-store";
import { cullingManager } from "~/utils/culling-manager";
import {
  type GeometryArgs,
  type geometryFactories,
  getGeometry,
  releaseGeometry,
} from "~/utils/geometry-pool";
import { createMaterialFromProperties } from "~/utils/material-utils";

// Note: Geometries are now managed by the geometry pool

export const SceneObjectMesh = memo(function SceneObjectMesh({
  object,
}: {
  object: SceneObject;
}) {
  const meshRef = useRef<Mesh>(null);
  const [targetObject, setTargetObject] = useState<THREE.Object3D | null>(null);

  // Use selectors to prevent unnecessary re-renders
  const selectedObjectIds = useSceneStore((state) => state.selectedObjectIds);
  const activeTool = useSceneStore((state) => state.activeTool);
  const transformDragging = useSceneStore((state) => state.transformDragging);
  const { selectObject, updateObject, setTransformDragging } = useSceneStore();

  const isSelected = selectedObjectIds.includes(object.id);

  // Memoize event handlers
  const handlePointerDown = useCallback(
    (event: React.PointerEvent) => {
      event.stopPropagation();
      const multiSelect =
        event.nativeEvent.ctrlKey || event.nativeEvent.metaKey;
      selectObject(object.id, multiSelect);
    },
    [object.id, selectObject]
  );

  const handleObjectChange = useCallback(() => {
    // Get the actual object being controlled (TransformControls manipulates this directly)
    const targetObj =
      object.type === "imported" && object.importedModel
        ? object.importedModel
        : meshRef.current;

    if (!targetObj) {
      return;
    }

    const positionArray = targetObj.position.toArray();
    const rotationArray = targetObj.rotation.toArray();
    const scaleArray = targetObj.scale.toArray();

    // For imported models, account for initial scale
    const finalScale: [number, number, number] =
      object.type === "imported" && object.initialScale
        ? [
            scaleArray[0] / object.initialScale,
            scaleArray[1] / object.initialScale,
            scaleArray[2] / object.initialScale,
          ]
        : [scaleArray[0], scaleArray[1], scaleArray[2]];

    updateObject(object.id, {
      position: [positionArray[0], positionArray[1], positionArray[2]],
      rotation: [
        (rotationArray[0] * 180) / Math.PI,
        (rotationArray[1] * 180) / Math.PI,
        (rotationArray[2] * 180) / Math.PI,
      ],
      scale: finalScale,
    });
  }, [
    object.id,
    object.type,
    object.initialScale,
    object.importedModel,
    updateObject,
  ]);

  // Memoize transform mode
  const transformMode = useMemo(() => {
    switch (activeTool) {
      case "move":
        return "translate";
      case "rotate":
        return "rotate";
      case "scale":
        return "scale";
      default:
        return "translate";
    }
  }, [activeTool]);

  // Optimize useFrame - only run when necessary properties change
  // Skip transform updates when dragging to allow TransformControls to work
  useFrame(() => {
    // Don't update transforms while dragging - let TransformControls handle it
    if (transformDragging && isSelected) {
      // Only update visibility and materials during drag
      if (object.type === "imported" && object.importedModel) {
        object.importedModel.visible = object.visible;
        if (isSelected !== object.importedModel.userData.lastSelected) {
          object.importedModel.userData.lastSelected = isSelected;
          object.importedModel.traverse((child) => {
            if (
              child instanceof Mesh &&
              child.material instanceof MeshStandardMaterial
            ) {
              (child.material.emissive as Color).setHex(
                isSelected ? 0x44_44_44 : 0x00_00_00
              );
              child.material.needsUpdate = true;
            }
          });
        }
      }
      return;
    }

    if (object.type === "imported" && object.importedModel) {
      // Batch position, rotation, scale updates
      object.importedModel.position.set(...object.position);
      const radX = (object.rotation[0] * Math.PI) / 180;
      const radY = (object.rotation[1] * Math.PI) / 180;
      const radZ = (object.rotation[2] * Math.PI) / 180;
      object.importedModel.rotation.set(radX, radY, radZ);

      const initialScale = object.initialScale || 1;
      object.importedModel.scale.set(
        object.scale[0] * initialScale,
        object.scale[1] * initialScale,
        object.scale[2] * initialScale
      );
      object.importedModel.visible = object.visible;

      // Only update materials if selection changed
      if (isSelected !== object.importedModel.userData.lastSelected) {
        object.importedModel.userData.lastSelected = isSelected;
        object.importedModel.traverse((child) => {
          if (
            child instanceof Mesh &&
            child.material instanceof MeshStandardMaterial
          ) {
            (child.material.emissive as Color).setHex(
              isSelected ? 0x44_44_44 : 0x00_00_00
            );
            child.material.needsUpdate = true;
          }
        });
      }
    } else if (meshRef.current) {
      // Batch updates for better performance
      meshRef.current.position.set(...object.position);

      const baseRotationX = object.type === "plane" ? -Math.PI / 2 : 0;
      meshRef.current.rotation.set(
        baseRotationX + (object.rotation[0] * Math.PI) / 180,
        (object.rotation[1] * Math.PI) / 180,
        (object.rotation[2] * Math.PI) / 180
      );
      meshRef.current.scale.set(...object.scale);
      meshRef.current.visible = object.visible;
    }
  });

  // Only update material emissive when selection changes
  useEffect(() => {
    if (
      object.type !== "imported" &&
      meshRef.current &&
      meshRef.current.material instanceof MeshStandardMaterial
    ) {
      (meshRef.current.material.emissive as Color).setHex(
        isSelected ? 0x44_44_44 : 0x00_00_00
      );
      meshRef.current.material.needsUpdate = true;
    }
  }, [isSelected, object.type]);

  // Use pooled geometry
  const { geometry, geometryType, geometryArgs } = useMemo((): {
    geometry: THREE.BufferGeometry;
    geometryType: keyof typeof geometryFactories;
    geometryArgs: GeometryArgs;
  } => {
    switch (object.type) {
      case "cube":
        return {
          geometry: getGeometry("box", 1, 1, 1),
          geometryType: "box" as const,
          geometryArgs: [1, 1, 1],
        };
      case "sphere":
        return {
          geometry: getGeometry("sphere", 0.5, 32, 32),
          geometryType: "sphere" as const,
          geometryArgs: [0.5, 32, 32],
        };
      case "plane":
        return {
          geometry: getGeometry("plane", 10, 10, 1, 1),
          geometryType: "plane" as const,
          geometryArgs: [10, 10, 1, 1],
        };
      default:
        return {
          geometry: getGeometry("box", 1, 1, 1),
          geometryType: "box" as const,
          geometryArgs: [1, 1, 1],
        };
    }
  }, [object.type]);

  // Get material from material system
  const material = useMemo(() => {
    // First try to use material assigned to object
    if (object.materialId) {
      const materialStore = useMaterialStore.getState();
      const materialEntry = materialStore.getMaterialById(object.materialId);
      if (materialEntry) {
        return createMaterialFromProperties(materialEntry.properties);
      }
    }

    // Fallback to legacy material or create default
    if (object.material) {
      return object.material;
    }

    // Create a default material based on object color
    return new MeshStandardMaterial({
      color: new THREE.Color(object.color),
      roughness: 0.5,
      metalness: 0.0,
      // Make planes double-sided so they're visible from both sides
      side: object.type === "plane" ? THREE.DoubleSide : THREE.FrontSide,
    });
  }, [object.materialId, object.material, object.color, object.type]);

  // Update material properties
  useEffect(() => {
    if (material) {
      material.transparent = !object.visible;
      material.opacity = object.visible ? 1 : 0.3;
      material.needsUpdate = true;
    }
  }, [material, object.visible]);

  // Cleanup geometry and material on unmount (only for non-imported objects)
  useEffect(() => {
    // Register with culling manager when mesh is created
    if (meshRef.current) {
      cullingManager.addObject(meshRef.current, false); // Regular objects can be culled
    }

    return () => {
      // Remove from culling manager
      if (meshRef.current) {
        cullingManager.removeObject(meshRef.current);
      }

      // Cleanup geometry (materials are now managed by material system)
      if (object.type !== "imported") {
        releaseGeometry(geometryType, ...geometryArgs);
      }
    };
  }, [object.type, geometryType, geometryArgs]);

  // Update target object when mesh ref or imported model changes
  useEffect(() => {
    if (object.type === "imported" && object.importedModel) {
      setTargetObject(object.importedModel);
    }
    // For regular objects, the ref callback will handle setting targetObject
  }, [object.type, object.importedModel]);

  // Update target object when mesh ref becomes available (using ref callback)
  const handleMeshRef = useCallback(
    (node: Mesh | null) => {
      if (node && object.type !== "imported") {
        setTargetObject(node);
      } else if (!node && object.type !== "imported") {
        setTargetObject(null);
      }
    },
    [object.type]
  );

  // Get the current target object (check ref directly for non-imported objects)
  const currentTargetObject = useMemo(() => {
    if (object.type === "imported" && object.importedModel) {
      return object.importedModel;
    }
    // For regular objects, use the ref directly if state hasn't updated yet
    return targetObject || meshRef.current;
  }, [object.type, object.importedModel, targetObject]);

  // Memoize transform controls to prevent unnecessary re-mounting
  const transformControls = useMemo(() => {
    if (!isSelected || activeTool === "select" || !currentTargetObject)
      return null;

    return (
      <TransformControls
        key={`transform-${object.id}`}
        mode={transformMode}
        object={currentTargetObject}
        onMouseDown={() => setTransformDragging(true)}
        onMouseUp={() => setTransformDragging(false)}
        onObjectChange={handleObjectChange}
        showX
        showY
        showZ
        size={1}
      />
    );
  }, [
    isSelected,
    activeTool,
    transformMode,
    handleObjectChange,
    setTransformDragging,
    object.id,
    currentTargetObject,
  ]);

  return (
    <>
      {transformControls}

      {object.type === "imported" && object.importedModel ? (
        <primitive
          object={object.importedModel}
          onPointerDown={handlePointerDown}
        />
      ) : (
        <mesh
          castShadow
          geometry={geometry}
          material={material}
          onPointerDown={handlePointerDown}
          receiveShadow
          ref={(node) => {
            meshRef.current = node;
            handleMeshRef(node);
          }}
        />
      )}
    </>
  );
});
