import { Html } from "@react-three/drei";
import { useThree } from "@react-three/fiber";
import { useCallback, useEffect, useRef, useState } from "react";
import { Vector2, Vector3 } from "three";
import { useEditorStore } from "~/store/editor-store";

interface MarqueeSelectionProps {
  onSelectionChange: (selectedIds: string[]) => void;
}

interface SelectionBox {
  start: Vector2;
  end: Vector2;
  isActive: boolean;
}

export default function MarqueeSelection({ onSelectionChange }: MarqueeSelectionProps) {
  const { camera, gl } = useThree();
  const { mapData, selectedObjectId } = useEditorStore();

  const [selectionBox, setSelectionBox] = useState<SelectionBox>({
    start: new Vector2(),
    end: new Vector2(),
    isActive: false,
  });

  const [isMarqueeSelecting, setIsMarqueeSelecting] = useState(false);
  const [isCtrlDown, setIsCtrlDown] = useState(false);
  const [isShiftDown, setIsShiftDown] = useState(false);

  const overlayRef = useRef<HTMLDivElement>(null);

  // Convert screen coordinates to normalized device coordinates
  const screenToNDC = useCallback(
    (screenX: number, screenY: number) => {
      const rect = gl.domElement.getBoundingClientRect();
      if (!rect) return new Vector2();

      const x = ((screenX - rect.left) / rect.width) * 2 - 1;
      const y = (-(screenY - rect.top) / rect.height) * 2 + 1;
      return new Vector2(x, y);
    },
    [gl.domElement],
  );

  // Check if a point is inside the selection box
  const isPointInSelectionBox = useCallback((point: Vector2, box: SelectionBox) => {
    const minX = Math.min(box.start.x, box.end.x);
    const maxX = Math.max(box.start.x, box.end.x);
    const minY = Math.min(box.start.y, box.end.y);
    const maxY = Math.max(box.start.y, box.end.y);

    return point.x >= minX && point.x <= maxX && point.y >= minY && point.y <= maxY;
  }, []);

  // Get objects within selection box
  const getObjectsInSelectionBox = useCallback(
    (box: SelectionBox) => {
      if (!mapData) return [];

      const selectedObjects: string[] = [];

      mapData.objects.forEach((obj) => {
        // Convert object position to screen coordinates
        const worldPos = new Vector3(obj.position.x, obj.position.y, obj.position.z);
        const screenPos = worldPos.clone().project(camera);

        if (isPointInSelectionBox(new Vector2(screenPos.x, screenPos.y), box)) {
          selectedObjects.push(obj.id);
        }
      });

      return selectedObjects;
    },
    [mapData, camera, isPointInSelectionBox],
  );

  // Handle mouse down
  const handleMouseDown = useCallback(
    (event: MouseEvent) => {
      // Only work with Shift + Left mouse button
      if (event.button !== 0 || !event.shiftKey) return;

      const rect = gl.domElement.getBoundingClientRect();
      if (!rect) return;

      const mouseX = event.clientX - rect.left;
      const mouseY = event.clientY - rect.top;

      const ndc = screenToNDC(mouseX, mouseY);

      setSelectionBox({
        start: ndc.clone(),
        end: ndc.clone(),
        isActive: true,
      });

      setIsMarqueeSelecting(true);
    },
    [screenToNDC, gl.domElement],
  );

  // Handle mouse move
  const handleMouseMove = useCallback(
    (event: MouseEvent) => {
      if (!isMarqueeSelecting) return;

      const rect = gl.domElement.getBoundingClientRect();
      if (!rect) return;

      const mouseX = event.clientX - rect.left;
      const mouseY = event.clientY - rect.top;

      const ndc = screenToNDC(mouseX, mouseY);

      setSelectionBox((prev) => ({
        ...prev,
        end: ndc.clone(),
      }));
    },
    [isMarqueeSelecting, screenToNDC, gl.domElement],
  );

  // Handle mouse up
  const handleMouseUp = useCallback(() => {
    if (!isMarqueeSelecting) return;

    setIsMarqueeSelecting(false);

    const selectedObjects = getObjectsInSelectionBox(selectionBox);

    if (selectedObjects.length > 0) {
      if (isCtrlDown) {
        // Add to selection with Ctrl
        const currentSelection = selectedObjectId ? [selectedObjectId] : [];
        const newSelection = [...new Set([...currentSelection, ...selectedObjects])];
        onSelectionChange(newSelection);
      } else {
        // Replace selection (Shift+LMB marquee selection)
        onSelectionChange(selectedObjects);
      }
    } else if (!isCtrlDown && !isShiftDown) {
      // Clear selection if clicking on empty space (regular LMB)
      onSelectionChange([]);
    }

    setSelectionBox((prev) => ({
      ...prev,
      isActive: false,
    }));
  }, [
    isMarqueeSelecting,
    selectionBox,
    getObjectsInSelectionBox,
    isCtrlDown,
    isShiftDown,
    selectedObjectId,
    onSelectionChange,
  ]);

  // Handle keyboard events
  const handleKeyDown = useCallback((event: KeyboardEvent) => {
    if (event.key === "Control") setIsCtrlDown(true);
    if (event.key === "Shift") setIsShiftDown(true);
  }, []);

  const handleKeyUp = useCallback((event: KeyboardEvent) => {
    if (event.key === "Control") setIsCtrlDown(false);
    if (event.key === "Shift") setIsShiftDown(false);
  }, []);

  // Event listeners
  useEffect(() => {
    const canvas = gl.domElement;

    canvas.addEventListener("mousedown", handleMouseDown);
    canvas.addEventListener("mousemove", handleMouseMove);
    canvas.addEventListener("mouseup", handleMouseUp);

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);

    return () => {
      canvas.removeEventListener("mousedown", handleMouseDown);
      canvas.removeEventListener("mousemove", handleMouseMove);
      canvas.removeEventListener("mouseup", handleMouseUp);

      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [handleMouseDown, handleMouseMove, handleMouseUp, handleKeyDown, handleKeyUp, gl.domElement]);

  // Render selection box overlay
  const renderSelectionBox = () => {
    if (!selectionBox.isActive) return null;

    const rect = gl.domElement.getBoundingClientRect();
    if (!rect) return null;

    // Convert NDC back to screen coordinates for accurate positioning
    const startX = ((selectionBox.start.x + 1) / 2) * rect.width;
    const startY = ((1 - selectionBox.start.y) / 2) * rect.height;
    const endX = ((selectionBox.end.x + 1) / 2) * rect.width;
    const endY = ((1 - selectionBox.end.y) / 2) * rect.height;

    const left = Math.min(startX, endX);
    const top = Math.min(startY, endY);
    const width = Math.abs(endX - startX);
    const height = Math.abs(endY - startY);

    // Only render if the selection box has some size
    if (width < 2 || height < 2) return null;

    return (
      <div
        className="absolute pointer-events-none"
        ref={overlayRef}
        style={{
          left: left + rect.left,
          top: top + rect.top,
          width,
          height,
          border: "2px solid #00aaff",
          backgroundColor: "rgba(0, 170, 255, 0.1)",
          boxSizing: "border-box",
          zIndex: 1000,
        }}
      />
    );
  };

  return <Html>{renderSelectionBox()}</Html>;
}
