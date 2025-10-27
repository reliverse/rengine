import { useRef, useState } from "react";
import { Group, Vector3 } from "three";
import { useEditorStore } from "~/store/editor-store";
import { MapObject } from "~/types/map";

interface TransformGizmosProps {
  selectedObject: MapObject | null;
  onTransform: (updates: Partial<MapObject>) => void;
}

interface GizmoAxis {
  name: "x" | "y" | "z";
  color: string;
  position: [number, number, number];
  rotation: [number, number, number];
}

const axes: GizmoAxis[] = [
  { name: "x", color: "#ff0000", position: [1, 0, 0], rotation: [0, 0, -Math.PI / 2] },
  { name: "y", color: "#00ff00", position: [0, 1, 0], rotation: [0, 0, 0] },
  { name: "z", color: "#0000ff", position: [0, 0, 1], rotation: [Math.PI / 2, 0, 0] },
];

export default function TransformGizmos({ selectedObject, onTransform }: TransformGizmosProps) {
  const { tool } = useEditorStore();
  const [hoveredAxis, setHoveredAxis] = useState<string | null>(null);
  const [isDragging, setIsDragging] = useState(false);
  const [dragAxis, setDragAxis] = useState<string | null>(null);
  const [dragStart, setDragStart] = useState<Vector3 | null>(null);
  const [dragStartMouse, setDragStartMouse] = useState<Vector3 | null>(null);

  const gizmoRef = useRef<Group>(null);

  if (!selectedObject || tool === "select") {
    return null;
  }

  const handlePointerDown = (event: any, axis: string) => {
    event.stopPropagation();
    setIsDragging(true);
    setDragAxis(axis);
    setDragStart(
      new Vector3(selectedObject.position.x, selectedObject.position.y, selectedObject.position.z),
    );

    // Convert mouse position to world coordinates
    const mouseVector = new Vector3();
    mouseVector.x = (event.clientX / window.innerWidth) * 2 - 1;
    mouseVector.y = -(event.clientY / window.innerHeight) * 2 + 1;
    setDragStartMouse(mouseVector);
  };

  const handlePointerMove = (event: any) => {
    if (!isDragging || !dragAxis || !dragStart || !dragStartMouse) return;

    const mouseVector = new Vector3();
    mouseVector.x = (event.clientX / window.innerWidth) * 2 - 1;
    mouseVector.y = -(event.clientY / window.innerHeight) * 2 + 1;

    const deltaMouse = mouseVector.clone().sub(dragStartMouse);

    if (tool === "move") {
      handleMoveTransform(deltaMouse, dragAxis);
    } else if (tool === "rotate") {
      handleRotateTransform(deltaMouse, dragAxis);
    } else if (tool === "scale") {
      handleScaleTransform(deltaMouse, dragAxis);
    }
  };

  const handleMoveTransform = (deltaMouse: Vector3, axis: string) => {
    const moveSpeed = 2;
    const delta = deltaMouse.multiplyScalar(moveSpeed);

    const updates: Partial<MapObject> = { position: { ...selectedObject.position } };

    switch (axis) {
      case "x":
        updates.position!.x = dragStart!.x + delta.x;
        break;
      case "y":
        updates.position!.y = dragStart!.y + delta.y;
        break;
      case "z":
        updates.position!.z = dragStart!.z + delta.z;
        break;
    }

    onTransform(updates);
  };

  const handleRotateTransform = (deltaMouse: Vector3, axis: string) => {
    const rotateSpeed = 100;
    const delta = deltaMouse.x * rotateSpeed;

    const updates: Partial<MapObject> = { rotation: { ...selectedObject.rotation } };

    switch (axis) {
      case "x":
        updates.rotation!.x = selectedObject.rotation.x + delta;
        break;
      case "y":
        updates.rotation!.y = selectedObject.rotation.y + delta;
        break;
      case "z":
        updates.rotation!.z = selectedObject.rotation.z + delta;
        break;
    }

    onTransform(updates);
  };

  const handleScaleTransform = (deltaMouse: Vector3, axis: string) => {
    const scaleSpeed = 0.5;
    const delta = 1 + deltaMouse.x * scaleSpeed;

    const updates: Partial<MapObject> = { scale: { ...selectedObject.scale } };

    switch (axis) {
      case "x":
        updates.scale!.x = Math.max(0.1, selectedObject.scale.x * delta);
        break;
      case "y":
        updates.scale!.y = Math.max(0.1, selectedObject.scale.y * delta);
        break;
      case "z":
        updates.scale!.z = Math.max(0.1, selectedObject.scale.z * delta);
        break;
    }

    onTransform(updates);
  };

  const handlePointerUp = () => {
    setIsDragging(false);
    setDragAxis(null);
    setDragStart(null);
    setDragStartMouse(null);
  };

  const renderMoveGizmo = () => (
    <group ref={gizmoRef}>
      {axes.map((axis) => (
        <group key={axis.name}>
          {/* Arrow shaft */}
          <mesh
            onPointerDown={(e) => handlePointerDown(e, axis.name)}
            onPointerEnter={() => setHoveredAxis(axis.name)}
            onPointerLeave={() => setHoveredAxis(null)}
            onPointerMove={handlePointerMove}
            onPointerUp={handlePointerUp}
            position={[axis.position[0] * 0.5, axis.position[1] * 0.5, axis.position[2] * 0.5]}
            rotation={axis.rotation}
          >
            <cylinderGeometry args={[0.02, 0.02, 1, 8]} />
            <meshBasicMaterial
              color={hoveredAxis === axis.name ? "#ffff00" : axis.color}
              opacity={0.8}
              transparent
            />
          </mesh>
          {/* Arrow head */}
          <mesh
            onPointerDown={(e) => handlePointerDown(e, axis.name)}
            onPointerEnter={() => setHoveredAxis(axis.name)}
            onPointerLeave={() => setHoveredAxis(null)}
            onPointerMove={handlePointerMove}
            onPointerUp={handlePointerUp}
            position={axis.position}
            rotation={axis.rotation}
          >
            <coneGeometry args={[0.08, 0.2, 8]} />
            <meshBasicMaterial
              color={hoveredAxis === axis.name ? "#ffff00" : axis.color}
              opacity={0.8}
              transparent
            />
          </mesh>
        </group>
      ))}
      {/* Center sphere for uniform movement */}
      <mesh
        onPointerDown={(e) => handlePointerDown(e, "all")}
        onPointerEnter={() => setHoveredAxis("center")}
        onPointerLeave={() => setHoveredAxis(null)}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerUp}
        position={[0, 0, 0]}
      >
        <sphereGeometry args={[0.1, 8, 8]} />
        <meshBasicMaterial
          color={hoveredAxis === "center" ? "#ffff00" : "#ffffff"}
          opacity={0.8}
          transparent
        />
      </mesh>
    </group>
  );

  const renderRotateGizmo = () => (
    <group ref={gizmoRef}>
      {axes.map((axis) => (
        <mesh
          key={axis.name}
          onPointerDown={(e) => handlePointerDown(e, axis.name)}
          onPointerEnter={() => setHoveredAxis(axis.name)}
          onPointerLeave={() => setHoveredAxis(null)}
          onPointerMove={handlePointerMove}
          onPointerUp={handlePointerUp}
          position={[0, 0, 0]}
          rotation={axis.rotation}
        >
          <torusGeometry args={[1, 0.02, 8, 32]} />
          <meshBasicMaterial
            color={hoveredAxis === axis.name ? "#ffff00" : axis.color}
            opacity={0.8}
            transparent
          />
        </mesh>
      ))}
    </group>
  );

  const renderScaleGizmo = () => (
    <group ref={gizmoRef}>
      {axes.map((axis) => (
        <group key={axis.name}>
          {/* Scale handle */}
          <mesh
            onPointerDown={(e) => handlePointerDown(e, axis.name)}
            onPointerEnter={() => setHoveredAxis(axis.name)}
            onPointerLeave={() => setHoveredAxis(null)}
            onPointerMove={handlePointerMove}
            onPointerUp={handlePointerUp}
            position={axis.position}
            rotation={axis.rotation}
          >
            <boxGeometry args={[0.1, 0.1, 0.1]} />
            <meshBasicMaterial
              color={hoveredAxis === axis.name ? "#ffff00" : axis.color}
              opacity={0.8}
              transparent
            />
          </mesh>
          {/* Scale line */}
          <mesh
            onPointerDown={(e) => handlePointerDown(e, axis.name)}
            onPointerEnter={() => setHoveredAxis(axis.name)}
            onPointerLeave={() => setHoveredAxis(null)}
            onPointerMove={handlePointerMove}
            onPointerUp={handlePointerUp}
            position={[axis.position[0] * 0.5, axis.position[1] * 0.5, axis.position[2] * 0.5]}
            rotation={axis.rotation}
          >
            <cylinderGeometry args={[0.02, 0.02, 1, 8]} />
            <meshBasicMaterial
              color={hoveredAxis === axis.name ? "#ffff00" : axis.color}
              opacity={0.8}
              transparent
            />
          </mesh>
        </group>
      ))}
      {/* Center cube for uniform scaling */}
      <mesh
        onPointerDown={(e) => handlePointerDown(e, "all")}
        onPointerEnter={() => setHoveredAxis("center")}
        onPointerLeave={() => setHoveredAxis(null)}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerUp}
        position={[0, 0, 0]}
      >
        <boxGeometry args={[0.15, 0.15, 0.15]} />
        <meshBasicMaterial
          color={hoveredAxis === "center" ? "#ffff00" : "#ffffff"}
          opacity={0.8}
          transparent
        />
      </mesh>
    </group>
  );

  const renderGizmo = () => {
    switch (tool) {
      case "move":
        return renderMoveGizmo();
      case "rotate":
        return renderRotateGizmo();
      case "scale":
        return renderScaleGizmo();
      default:
        return null;
    }
  };

  return (
    <group
      position={[selectedObject.position.x, selectedObject.position.y, selectedObject.position.z]}
    >
      {renderGizmo()}
    </group>
  );
}
