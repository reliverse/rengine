import { memo, useMemo, useRef } from "react";
import * as THREE from "three";
import type { SceneObject } from "~/stores/scene-store";
import { useSceneStore } from "~/stores/scene-store";
import { geometryCache, materialCache } from "~/utils/geometry-cache";
import { GeometryLOD } from "./level-of-detail";
import { SceneObjectMesh } from "./scene-object-mesh";

// Memoized scene object mesh
const MemoizedSceneObjectMesh = memo(SceneObjectMesh);

// Instanced mesh component for efficient rendering of multiple similar objects
const InstancedObjects = memo(function InstancedObjects({
  objects,
}: {
  objects: SceneObject[];
}) {
  // Group objects by geometry type and material for proper instancing
  const instanceGroups = useMemo(() => {
    const groups: Array<{
      geometryType: string;
      materialKey: string;
      objects: SceneObject[];
      geometry: THREE.BufferGeometry;
      material: THREE.Material;
    }> = [];

    // Group by geometry and material combination
    const groupMap = new Map<string, SceneObject[]>();

    for (const obj of objects) {
      if (obj.type !== "cube" && obj.type !== "sphere" && obj.type !== "plane")
        continue;

      const key = `${obj.type}_${obj.color}`;
      if (!groupMap.has(key)) {
        groupMap.set(key, []);
      }
      const group = groupMap.get(key);
      if (group) {
        group.push(obj);
      }
    }

    // Create instance groups only for groups with multiple objects
    for (const [key, groupObjects] of groupMap) {
      if (groupObjects.length > 1) {
        const [geometryType, color] = key.split("_");
        let geometry: THREE.BufferGeometry;

        // Get cached geometry and material
        switch (geometryType) {
          case "cube":
            geometry = geometryCache.getCube(1);
            break;
          case "sphere":
            geometry = geometryCache.getSphere(0.5, 32, 32);
            break;
          case "plane":
            geometry = geometryCache.getPlane(10, 10);
            break;
          default:
            continue;
        }

        const material = materialCache.getStandardMaterial(color);

        groups.push({
          geometryType,
          materialKey: key,
          objects: groupObjects,
          geometry,
          material,
        });
      }
    }

    return groups;
  }, [objects]);

  // Don't render if no instance groups
  if (instanceGroups.length === 0) return null;

  return (
    <>
      {instanceGroups.map((group) => (
        <InstancedMeshGroup group={group} key={group.materialKey} />
      ))}
    </>
  );
});

// Individual instanced mesh group component
const InstancedMeshGroup = memo(function InstancedMeshGroup({
  group,
}: {
  group: {
    geometryType: string;
    materialKey: string;
    objects: SceneObject[];
    geometry: THREE.BufferGeometry;
    material: THREE.Material;
  };
}) {
  const meshRef = useRef<THREE.InstancedMesh>(null);

  // Update instance matrices only when objects change
  useMemo(() => {
    if (!meshRef.current) return;

    const matrix = new THREE.Matrix4();
    const quaternion = new THREE.Quaternion();
    const scale = new THREE.Vector3();

    for (let i = 0; i < group.objects.length; i++) {
      const obj = group.objects[i];

      // Set position
      matrix.setPosition(obj.position[0], obj.position[1], obj.position[2]);

      // Set rotation (convert degrees to radians)
      quaternion.setFromEuler(
        new THREE.Euler(
          (obj.rotation[0] * Math.PI) / 180,
          (obj.rotation[1] * Math.PI) / 180,
          (obj.rotation[2] * Math.PI) / 180
        )
      );
      matrix.makeRotationFromQuaternion(quaternion);

      // Set scale
      scale.set(obj.scale[0], obj.scale[1], obj.scale[2]);
      matrix.scale(scale);

      meshRef.current.setMatrixAt(i, matrix);
    }

    if (meshRef.current.instanceMatrix) {
      meshRef.current.instanceMatrix.needsUpdate = true;
    }
  }, [group.objects]);

  return (
    <instancedMesh
      args={[group.geometry, group.material, group.objects.length]}
      frustumCulled={true}
      ref={meshRef}
    />
  );
});

export const SceneObjects = memo(function SceneObjects() {
  const objects = useSceneStore((state) => state.objects);

  return (
    <>
      {/* Instanced rendering for multiple similar objects */}
      <InstancedObjects objects={objects} />

      {/* Individual rendering for unique objects and imported models */}
      {objects.map((object) => {
        // Use LOD for basic geometry types that benefit from distance-based quality reduction
        if (object.type === "sphere") {
          return (
            <GeometryLOD
              color={object.color}
              key={object.id}
              size={Math.max(...object.scale)}
              type={object.type}
            />
          );
        }

        // Use regular mesh for other types
        return <MemoizedSceneObjectMesh key={object.id} object={object} />;
      })}
    </>
  );
});
