import { useFrame } from "@react-three/fiber";
import { useRef } from "react";
import {
  BoxGeometry,
  Mesh,
  MeshStandardMaterial,
  PlaneGeometry,
  SphereGeometry,
} from "three";
import { useSceneStore } from "~/stores/scene-store";

export function PlacementPreview() {
  const { placementMode } = useSceneStore();
  const meshRef = useRef<Mesh>(null);

  useFrame(() => {
    if (meshRef.current && placementMode.previewPosition) {
      meshRef.current.position.set(...placementMode.previewPosition);

      if (placementMode.objectType === "plane") {
        meshRef.current.rotation.set(-Math.PI / 2, 0, 0);
      } else {
        meshRef.current.rotation.set(0, 0, 0);
      }
    }
  });

  if (
    !(
      placementMode.active &&
      placementMode.objectType &&
      placementMode.previewPosition
    )
  ) {
    return null;
  }

  let geometry: BoxGeometry | SphereGeometry | PlaneGeometry;
  switch (placementMode.objectType) {
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
    color: "#ffffff",
    transparent: true,
    opacity: 0.7,
    wireframe: true,
  });

  return <primitive object={new Mesh(geometry, material)} ref={meshRef} />;
}
