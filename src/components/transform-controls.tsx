import { TransformControls } from "@react-three/drei";
import { useRef } from "react";
import type { SceneObject } from "~/stores/scene-store";
import {
  useSceneStore,
  useSelectedLight,
  useSelectedObject,
} from "~/stores/scene-store";

export function SceneTransformControls() {
  const { activeTool, updateObject, updateLight, setTransformDragging } =
    useSceneStore();
  const selectedObject = useSelectedObject();
  const selectedLight = useSelectedLight();
  const transformControlsRef =
    useRef<React.ComponentRef<typeof TransformControls>>(null);

  const target = selectedObject || selectedLight;
  const isLight = !!selectedLight;

  if (!target || activeTool === "select") {
    return null;
  }

  const handleObjectChange = () => {
    if (!(transformControlsRef.current && target)) {
      return;
    }

    const newPosition = transformControlsRef.current.position.toArray();

    if (isLight) {
      updateLight(target.id, {
        position: [newPosition[0], newPosition[1], newPosition[2]],
      });
    } else {
      const rotationArray = transformControlsRef.current.rotation.toArray();
      const scaleArray = transformControlsRef.current.scale.toArray();

      updateObject(target.id, {
        position: [newPosition[0], newPosition[1], newPosition[2]],
        rotation: [
          (rotationArray[0] * 180) / Math.PI,
          (rotationArray[1] * 180) / Math.PI,
          (rotationArray[2] * 180) / Math.PI,
        ],
        scale: [scaleArray[0], scaleArray[1], scaleArray[2]],
      });
    }
  };

  const getMode = () => {
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

  const mode = getMode();

  const handleMouseDown = () => {
    setTransformDragging(true);
  };

  const handleMouseUp = () => {
    setTransformDragging(false);
  };

  return (
    <TransformControls
      mode={mode}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      onObjectChange={handleObjectChange}
      position={target.position}
      ref={transformControlsRef}
      {...(!isLight && {
        rotation: [
          ((target as SceneObject).rotation[0] * Math.PI) / 180,
          ((target as SceneObject).rotation[1] * Math.PI) / 180,
          ((target as SceneObject).rotation[2] * Math.PI) / 180,
        ],
        scale: (target as SceneObject).scale,
      })}
      showX
      showY
      showZ
      size={1}
    />
  );
}
