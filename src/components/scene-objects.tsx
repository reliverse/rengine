import * as THREE from "three";
import { useSceneStore } from "~/stores/scene-store";
import Polyhedron from "./polyhedron";
import { SceneObjectMesh } from "./scene-object-mesh";

function EnhancedPolyhedron({
  material,
  name,
  position,
  rotationSpeed,
}: {
  material: THREE.Material;
  name: string;
  position: [number, number, number];
  rotationSpeed: { x: number; y: number };
}) {
  return (
    <Polyhedron
      material={material}
      name={name}
      position={position}
      rotationSpeed={rotationSpeed}
    />
  );
}

export function SceneObjects() {
  const objects = useSceneStore((state) => state.objects);

  // Demo object rotation controls - using default values
  const rotationControls = {
    "Basic Material - X Rotation": 0.2,
    "Basic Material - Y Rotation": 0.05,
    "Normal Material - X Rotation": 0.2,
    "Normal Material - Y Rotation": 0.05,
    "Phong Material - X Rotation": 0.2,
    "Phong Material - Y Rotation": 0.05,
    "Standard Material - X Rotation": 0.2,
    "Standard Material - Y Rotation": 0.05,
  };

  return (
    <>
      {/* Regular scene objects */}
      {objects.map((object) => (
        <SceneObjectMesh key={object.id} object={object} />
      ))}

      {/* Demo objects for lighting showcase */}
      <EnhancedPolyhedron
        material={new THREE.MeshBasicMaterial({ color: "yellow" })}
        name="Basic Material"
        position={[-3, 1, 0]}
        rotationSpeed={{
          x: rotationControls["Basic Material - X Rotation"],
          y: rotationControls["Basic Material - Y Rotation"],
        }}
      />
      <EnhancedPolyhedron
        material={new THREE.MeshNormalMaterial({ flatShading: true })}
        name="Normal Material"
        position={[-1, 1, 0]}
        rotationSpeed={{
          x: rotationControls["Normal Material - X Rotation"],
          y: rotationControls["Normal Material - Y Rotation"],
        }}
      />
      <EnhancedPolyhedron
        material={
          new THREE.MeshPhongMaterial({ color: "lime", flatShading: true })
        }
        name="Phong Material"
        position={[1, 1, 0]}
        rotationSpeed={{
          x: rotationControls["Phong Material - X Rotation"],
          y: rotationControls["Phong Material - Y Rotation"],
        }}
      />
      <EnhancedPolyhedron
        material={
          new THREE.MeshStandardMaterial({
            color: 0xff_00_33,
            flatShading: true,
          })
        }
        name="Standard Material"
        position={[3, 1, 0]}
        rotationSpeed={{
          x: rotationControls["Standard Material - X Rotation"],
          y: rotationControls["Standard Material - Y Rotation"],
        }}
      />
    </>
  );
}
