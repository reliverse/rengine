import { useThree } from "@react-three/fiber";
import { useEffect, useRef } from "react";
import { Plane, Raycaster, Vector2, Vector3 } from "three";
import { useSceneStore } from "~/stores/scene-store";

export function MouseInteraction({
  controlsRef,
}: {
  controlsRef: React.RefObject<any>;
}) {
  const { camera, gl } = useThree();
  const {
    placementMode,
    updatePlacementPreview,
    confirmPlacement,
    cancelPlacement,
    cameraPosition,
    cameraTarget,
    clearSelection,
    selectAll,
    removeObject,
    selectedObjectIds,
  } = useSceneStore();
  const raycaster = useRef(new Raycaster());
  const plane = useRef(new Plane(new Vector3(0, 1, 0), 0)); // Ground plane

  // Restore camera position on mount
  useEffect(() => {
    camera.position.set(...cameraPosition);

    if (controlsRef.current) {
      controlsRef.current.target.set(...cameraTarget);
      controlsRef.current.update();
    }
  }, [camera, cameraPosition, cameraTarget, controlsRef]);

  useEffect(() => {
    if (!placementMode.active) {
      return;
    }

    const handleMouseMove = (event: MouseEvent) => {
      const rect = gl.domElement.getBoundingClientRect();
      const x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
      const y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

      raycaster.current.setFromCamera(new Vector2(x, y), camera);
      const intersection = new Vector3();
      raycaster.current.ray.intersectPlane(plane.current, intersection);

      if (intersection) {
        updatePlacementPreview([
          intersection.x,
          intersection.y,
          intersection.z,
        ]);
      }
    };

    const handleClick = (event: MouseEvent) => {
      if (placementMode.active && placementMode.previewPosition) {
        event.stopPropagation();
        confirmPlacement();
      } else if (selectedObjectIds.length > 0) {
        // Click-to-clear: Clear all object selections when clicking on empty 3D space
        // This only triggers if no object was clicked (objects call stopPropagation)
        clearSelection();
      }
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        if (placementMode.active) {
          cancelPlacement();
        } else {
          clearSelection();
        }
      } else if (event.key === "a" && (event.ctrlKey || event.metaKey)) {
        event.preventDefault();
        selectAll();
      } else if (event.key === "Delete" || event.key === "Backspace") {
        // Remove selected objects
        for (const id of selectedObjectIds) {
          removeObject(id);
        }
      }
    };

    gl.domElement.addEventListener("mousemove", handleMouseMove);
    gl.domElement.addEventListener("click", handleClick);
    window.addEventListener("keydown", handleKeyDown);

    return () => {
      gl.domElement.removeEventListener("mousemove", handleMouseMove);
      gl.domElement.removeEventListener("click", handleClick);
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [
    placementMode.active,
    placementMode.previewPosition,
    camera,
    gl,
    updatePlacementPreview,
    confirmPlacement,
    cancelPlacement,
    clearSelection,
    selectAll,
    removeObject,
    selectedObjectIds,
  ]);

  return null;
}
