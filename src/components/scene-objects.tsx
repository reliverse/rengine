import { useSceneStore } from "~/stores/scene-store";
import { SceneObjectMesh } from "./scene-object-mesh";

export function SceneObjects() {
  const objects = useSceneStore((state) => state.objects);

  return (
    <>
      {objects.map((object) => (
        <SceneObjectMesh key={object.id} object={object} />
      ))}
    </>
  );
}
