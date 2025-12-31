import { useFrame } from "@react-three/fiber";
import { useRef } from "react";
import type * as THREE from "three";

interface PolyhedronProps {
  name?: string;
  position?: [number, number, number];
  material?: THREE.Material;
  rotationSpeed?: { x: number; y: number };
}

export default function Polyhedron({
  rotationSpeed = { x: 0.2, y: 0.05 },
  ...props
}: PolyhedronProps) {
  const ref = useRef<THREE.Mesh>(null);

  useFrame((_, delta) => {
    if (ref.current) {
      ref.current.rotation.x += rotationSpeed.x * delta;
      ref.current.rotation.y += rotationSpeed.y * delta;
    }
  });

  return (
    <mesh {...props} ref={ref}>
      <icosahedronGeometry args={[1, 1]} />
    </mesh>
  );
}
