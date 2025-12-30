import { TransformControls } from "@react-three/drei";
import { useFrame } from "@react-three/fiber";
import { useEffect, useRef } from "react";
import {
  BoxGeometry,
  Mesh,
  MeshStandardMaterial,
  PlaneGeometry,
  SphereGeometry,
} from "three";
import type { SceneObject } from "~/stores/scene-store";
import { useSceneStore } from "~/stores/scene-store";

export function SceneObjectMesh({ object }: { object: SceneObject }) {
  const meshRef = useRef<Mesh>(null);
  const transformControlsRef =
    useRef<React.ComponentRef<typeof TransformControls>>(null);
  const selectedObjectIds = useSceneStore((state) => state.selectedObjectIds);
  const isSelected = selectedObjectIds.includes(object.id);
  const { selectObject, activeTool, updateObject, setTransformDragging } =
    useSceneStore();

  const handlePointerDown = (event: React.PointerEvent) => {
    event.stopPropagation();
    const multiSelect = event.nativeEvent.ctrlKey || event.nativeEvent.metaKey;
    selectObject(object.id, multiSelect);
  };

  const handleObjectChange = () => {
    if (!(transformControlsRef.current && meshRef.current)) {
      return;
    }

    const positionArray = meshRef.current.position.toArray();
    const rotationArray = meshRef.current.rotation.toArray();
    const scaleArray = meshRef.current.scale.toArray();

    updateObject(object.id, {
      position: [positionArray[0], positionArray[1], positionArray[2]],
      rotation: [
        (rotationArray[0] * 180) / Math.PI,
        (rotationArray[1] * 180) / Math.PI,
        (rotationArray[2] * 180) / Math.PI,
      ],
      scale: [scaleArray[0], scaleArray[1], scaleArray[2]],
    });
  };

  const getTransformMode = () => {
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
  };

  useFrame(() => {
    if (object.type === "imported" && object.importedModel) {
      object.importedModel.position.set(...object.position);
      object.importedModel.rotation.set(
        (object.rotation[0] * Math.PI) / 180,
        (object.rotation[1] * Math.PI) / 180,
        (object.rotation[2] * Math.PI) / 180
      );
      const initialScale = object.initialScale || 1;
      object.importedModel.scale.set(
        object.scale[0] * initialScale,
        object.scale[1] * initialScale,
        object.scale[2] * initialScale
      );
      object.importedModel.visible = object.visible;

      object.importedModel.traverse((child) => {
        if (
          child instanceof Mesh &&
          child.material instanceof MeshStandardMaterial
        ) {
          if (isSelected) {
            child.material.emissive.setHex(0x44_44_44);
          } else {
            child.material.emissive.setHex(0x00_00_00);
          }
          child.material.needsUpdate = true;
        }
      });
    } else if (meshRef.current) {
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

  useEffect(() => {
    if (
      object.type !== "imported" &&
      meshRef.current &&
      meshRef.current.material instanceof MeshStandardMaterial
    ) {
      if (isSelected) {
        meshRef.current.material.emissive.setHex(0x44_44_44);
      } else {
        meshRef.current.material.emissive.setHex(0x00_00_00);
      }
      meshRef.current.material.needsUpdate = true;
    }
  }, [isSelected, object.type]);

  let geometry: BoxGeometry | SphereGeometry | PlaneGeometry;
  switch (object.type) {
    case "cube":
      geometry = new BoxGeometry(1, 1, 1);
      break;
    case "sphere":
      geometry = new SphereGeometry(0.5, 32, 32);
      break;
    case "plane":
      geometry = new PlaneGeometry(10, 10);
      break;
    default:
      geometry = new BoxGeometry(1, 1, 1);
  }

  const material = new MeshStandardMaterial({
    color: object.color,
    transparent: !object.visible,
    opacity: object.visible ? 1 : 0.3,
  });

  return (
    <>
      {isSelected && activeTool !== "select" && (
        <TransformControls
          mode={getTransformMode()}
          object={meshRef.current || undefined}
          onMouseDown={() => setTransformDragging(true)}
          onMouseUp={() => setTransformDragging(false)}
          onObjectChange={handleObjectChange}
          ref={transformControlsRef}
          showX
          showY
          showZ
          size={1}
        />
      )}

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
          ref={meshRef}
        />
      )}
    </>
  );
}
