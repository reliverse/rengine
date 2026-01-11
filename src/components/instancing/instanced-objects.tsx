import { useFrame } from "@react-three/fiber";
import { useEffect, useMemo, useRef } from "react";
import * as THREE from "three";

// Instanced mesh component for rendering many identical objects efficiently
// Reduces draw calls from N objects to 1 draw call

interface InstancedObjectsProps {
  count: number;
  geometry: THREE.BufferGeometry;
  material: THREE.Material;
  positions?: [number, number, number][];
  rotations?: [number, number, number][];
  scales?: [number, number, number][];
  colors?: THREE.Color[];
  animate?: boolean;
  animationSpeed?: number;
}

export function InstancedObjects({
  count,
  geometry,
  material,
  positions,
  rotations,
  scales,
  colors,
  animate = false,
  animationSpeed = 1,
}: InstancedObjectsProps) {
  const meshRef = useRef<THREE.InstancedMesh>(null);
  const dummy = useMemo(() => new THREE.Object3D(), []);

  // Generate random positions if not provided
  const instancePositions = useMemo(() => {
    if (positions && positions.length >= count) {
      return positions.slice(0, count);
    }
    return Array.from(
      { length: count },
      () =>
        [
          (Math.random() - 0.5) * 100,
          (Math.random() - 0.5) * 100,
          (Math.random() - 0.5) * 100,
        ] as [number, number, number]
    );
  }, [positions, count]);

  // Generate random rotations if not provided
  const instanceRotations = useMemo(() => {
    if (rotations && rotations.length >= count) {
      return rotations.slice(0, count);
    }
    return Array.from(
      { length: count },
      () =>
        [
          Math.random() * Math.PI * 2,
          Math.random() * Math.PI * 2,
          Math.random() * Math.PI * 2,
        ] as [number, number, number]
    );
  }, [rotations, count]);

  // Generate scales if not provided
  const instanceScales = useMemo(() => {
    if (scales && scales.length >= count) {
      return scales.slice(0, count);
    }
    return Array.from(
      { length: count },
      () =>
        [
          0.5 + Math.random() * 0.5,
          0.5 + Math.random() * 0.5,
          0.5 + Math.random() * 0.5,
        ] as [number, number, number]
    );
  }, [scales, count]);

  // Generate colors if not provided
  const instanceColors = useMemo(() => {
    if (colors && colors.length >= count) {
      return colors.slice(0, count);
    }
    return Array.from({ length: count }, () =>
      new THREE.Color().setHSL(Math.random(), 0.7, 0.5)
    );
  }, [colors, count]);

  // Set up instances
  useEffect(() => {
    if (!meshRef.current) return;

    const mesh = meshRef.current;

    for (let i = 0; i < count; i++) {
      const [x, y, z] = instancePositions[i];
      const [rx, ry, rz] = instanceRotations[i];
      const [sx, sy, sz] = instanceScales[i];

      dummy.position.set(x, y, z);
      dummy.rotation.set(rx, ry, rz);
      dummy.scale.set(sx, sy, sz);
      dummy.updateMatrix();

      mesh.setMatrixAt(i, dummy.matrix);

      // Set color if material supports it
      if (mesh.instanceColor) {
        mesh.setColorAt(i, instanceColors[i]);
      }
    }

    mesh.instanceMatrix.needsUpdate = true;
    if (mesh.instanceColor) {
      mesh.instanceColor.needsUpdate = true;
    }
  }, [
    count,
    instancePositions,
    instanceRotations,
    instanceScales,
    instanceColors,
    dummy,
  ]);

  // Animation loop
  useFrame((state) => {
    if (!(animate && meshRef.current)) return;

    const mesh = meshRef.current;
    const time = state.clock.elapsedTime * animationSpeed;

    for (let i = 0; i < count; i++) {
      const [x, y, z] = instancePositions[i];
      const [rx, ry, rz] = instanceRotations[i];
      const [sx, sy, sz] = instanceScales[i];

      dummy.position.set(
        x + Math.sin(time + i * 0.1) * 0.5,
        y + Math.cos(time + i * 0.1) * 0.5,
        z + Math.sin(time * 0.5 + i * 0.1) * 0.5
      );
      dummy.rotation.set(
        rx + time * 0.2 + i * 0.01,
        ry + time * 0.1 + i * 0.01,
        rz + time * 0.15 + i * 0.01
      );
      dummy.scale.set(sx, sy, sz);
      dummy.updateMatrix();

      mesh.setMatrixAt(i, dummy.matrix);
    }

    mesh.instanceMatrix.needsUpdate = true;
  });

  return (
    <instancedMesh
      args={[geometry, material, count]}
      frustumCulled={false}
      ref={meshRef} // Disable frustum culling for performance with many instances
    />
  );
}

// Utility function to create a grid of instanced objects
export function createInstancedGrid({
  count,
  spacing = 2,
  geometry,
  material,
  animate = false,
}: {
  count: number;
  spacing?: number;
  geometry: THREE.BufferGeometry;
  material: THREE.Material;
  animate?: boolean;
}) {
  const gridSize = Math.ceil(Math.sqrt(count));
  const positions: [number, number, number][] = [];

  for (let i = 0; i < count; i++) {
    const x = ((i % gridSize) - gridSize / 2) * spacing;
    const z = (Math.floor(i / gridSize) - gridSize / 2) * spacing;
    positions.push([x, 0, z]);
  }

  return (
    <InstancedObjects
      animate={animate}
      count={count}
      geometry={geometry}
      material={material}
      positions={positions}
    />
  );
}

// Performance-optimized instancing for static objects
export function InstancedStaticObjects({
  count,
  geometry,
  material,
  positions,
}: {
  count: number;
  geometry: THREE.BufferGeometry;
  material: THREE.Material;
  positions: [number, number, number][];
}) {
  const meshRef = useRef<THREE.InstancedMesh>(null);
  const dummy = useMemo(() => new THREE.Object3D(), []);

  useEffect(() => {
    if (!meshRef.current) return;

    const mesh = meshRef.current;

    for (let i = 0; i < Math.min(count, positions.length); i++) {
      const [x, y, z] = positions[i];

      dummy.position.set(x, y, z);
      dummy.rotation.set(0, 0, 0);
      dummy.scale.set(1, 1, 1);
      dummy.updateMatrix();

      mesh.setMatrixAt(i, dummy.matrix);
    }

    mesh.instanceMatrix.needsUpdate = true;
  }, [count, positions, dummy]);

  return (
    <instancedMesh
      args={[geometry, material, count]}
      frustumCulled={true}
      ref={meshRef} // Enable frustum culling for static objects
    />
  );
}
