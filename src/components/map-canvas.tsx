import { Box, Grid, Plane, Sphere } from "@react-three/drei";
import { Canvas, useFrame } from "@react-three/fiber";
import { invoke } from "@tauri-apps/api/core";
import { useRef, useState } from "react";
import { MathUtils, Mesh, Vector3 as THREEVector3 } from "three";
import { toast } from "~/lib/toast";
import { useEditorStore } from "~/store/editor-store";
import { MapObject, Vector3 } from "~/types/map";
import MarqueeSelection from "./marquee-selection";
import TransformGizmos from "./transform-gizmos";
import ViewportControls from "./viewport-controls";
import ViewportDisplayControls from "./viewport-display-controls";

interface MapObjectProps {
  object: MapObject;
  isSelected: boolean;
  onSelect: () => void;
}

function MapObjectComponent({ object, isSelected, onSelect }: MapObjectProps) {
  const meshRef = useRef<Mesh>(null);

  useFrame(() => {
    if (meshRef.current) {
      meshRef.current.position.set(object.position.x, object.position.y, object.position.z);
      meshRef.current.rotation.set(
        MathUtils.degToRad(object.rotation.x),
        MathUtils.degToRad(object.rotation.y),
        MathUtils.degToRad(object.rotation.z),
      );
      meshRef.current.scale.set(object.scale.x, object.scale.y, object.scale.z);
    }
  });

  const handleClick = (e: any) => {
    e.stopPropagation();
    onSelect();
  };

  const getGeometry = () => {
    switch (object.object_type) {
      case "cube":
        return <Box args={[1, 1, 1]} />;
      case "sphere":
        return <Sphere args={[0.5, 32, 32]} />;
      case "plane":
        return <Plane args={[1, 1]} rotation={[-Math.PI / 2, 0, 0]} />;
      default:
        return <Box args={[1, 1, 1]} />;
    }
  };

  return (
    <mesh
      onClick={handleClick}
      ref={meshRef}
      userData={{ objectId: object.id }}
      visible={object.visible}
    >
      {getGeometry()}
      <meshStandardMaterial
        color={object.color}
        opacity={isSelected ? 0.7 : 1.0}
        transparent={isSelected}
        wireframe={isSelected}
      />
    </mesh>
  );
}

function Scene() {
  const { mapData, selectedObjectId, selectObject, isGridVisible, gridSize, tool, updateObject } =
    useEditorStore();

  const [, setSelectedObjects] = useState<string[]>([]);

  const handleTransform = (updates: Partial<MapObject>) => {
    if (selectedObjectId) {
      updateObject(selectedObjectId, updates);
    }
  };

  const handleSelectionChange = (selectedIds: string[]) => {
    setSelectedObjects(selectedIds);
    if (selectedIds.length > 0) {
      selectObject(selectedIds[0] ?? null); // Select first object as primary
    } else {
      selectObject(null);
    }
  };

  const selectedObject = mapData?.objects.find((obj) => obj.id === selectedObjectId);

  return (
    <>
      <ambientLight intensity={0.5} />
      <directionalLight intensity={1} position={[10, 10, 5]} />

      {/* Invisible ground plane for object placement */}
      <mesh
        position={[0, 0, 0]}
        receiveShadow
        rotation={[-Math.PI / 2, 0, 0]}
        userData={{ isGroundPlane: true }}
        visible={false}
      >
        <planeGeometry args={[1000, 1000]} />
        <meshBasicMaterial
          opacity={0}
          side={2}
          transparent // DoubleSide to ensure raycasting works from both sides
        />
      </mesh>

      {isGridVisible && (
        <Grid
          args={[100, 100]}
          cellColor="#6f6f6f"
          cellSize={gridSize}
          cellThickness={0.5}
          fadeDistance={50}
          fadeStrength={1}
          followCamera={false}
          infiniteGrid
          sectionColor="#9d4edd"
          sectionSize={10}
          sectionThickness={1}
        />
      )}

      {mapData?.objects.map((object) => (
        <MapObjectComponent
          isSelected={selectedObjectId === object.id}
          key={object.id}
          object={object}
          onSelect={() => selectObject(object.id)}
        />
      ))}

      {/* Transform gizmos for selected object */}
      {selectedObject && tool !== "select" && (
        <TransformGizmos onTransform={handleTransform} selectedObject={selectedObject} />
      )}

      {/* Marquee selection for orthographic views */}
      <MarqueeSelection onSelectionChange={handleSelectionChange} />
    </>
  );
}

export default function MapCanvas() {
  const {
    camera,
    tool,
    pendingObjectType,
    setPendingObjectType,
    setTool,
    addObject,
    syncWithBackend,
    selectObject,
    mapData,
  } = useEditorStore();

  const handleCanvasClick = (event: any) => {
    console.log("Canvas clicked", { tool, pendingObjectType, intersections: event.intersections });

    // Don't handle clicks when Shift is pressed (let marquee selection handle it)
    if (event.nativeEvent?.shiftKey) return;

    if (tool === "add" && pendingObjectType) {
      console.log("Adding object", pendingObjectType);

      // Get the intersection point for object placement
      const intersections = event.intersections || [];
      console.log("All intersections:", intersections);
      console.log(
        "Intersection details:",
        intersections.map((i: any) => ({
          object: i.object?.userData,
          point: i.point,
          distance: i.distance,
        })),
      );

      // Find the best intersection for object placement
      let intersection = null;

      // First, try to find ground plane intersection
      intersection = intersections.find(
        (intersection: any) => intersection.object?.userData?.isGroundPlane === true,
      );

      // If no ground plane intersection, find any non-object intersection
      if (!intersection) {
        intersection = intersections.find((intersection: any) => {
          // Check if this intersection is with an existing map object
          const isExistingObject = mapData?.objects.some(
            (obj) => intersection.object?.userData?.objectId === obj.id,
          );
          return !isExistingObject;
        });
      }

      // If still no intersection, use the first available intersection
      if (!intersection) {
        intersection = intersections[0];
      }

      let position;

      if (intersection && intersection.point) {
        // Use intersection point if available
        position = {
          x: intersection.point.x,
          y: intersection.point.y,
          z: intersection.point.z,
        };
        console.log("Using intersection point:", position);
      } else {
        // If no intersection, try to place object in front of camera
        // This is a better fallback than always using (0,0,0)
        const cameraPosition = event.camera?.position;
        const cameraDirection = event.camera?.getWorldDirection?.(new THREEVector3());

        if (cameraPosition && cameraDirection) {
          // Place object 5 units in front of camera
          const distance = 5;
          position = {
            x: cameraPosition.x + cameraDirection.x * distance,
            y: cameraPosition.y + cameraDirection.y * distance,
            z: cameraPosition.z + cameraDirection.z * distance,
          };
          console.log("No intersection, placing in front of camera:", position);
        } else {
          // Last resort: place at origin
          position = {
            x: 0,
            y: 0,
            z: 0,
          };
          console.log("No intersection and no camera info, using origin:", position);
        }
      }

      // Add the pending object type at the clicked position
      addObjectToMap(pendingObjectType, position);

      // Clear pending object type and switch back to select tool
      setPendingObjectType(null);
      setTool("select");
    } else if (tool === "select") {
      // Only clear selection if not clicking on an object
      const intersections = event.intersections || [];
      if (intersections.length === 0) {
        selectObject(null);
      }
    }
  };

  const addObjectToMap = async (type: "cube" | "sphere" | "plane", position: Vector3) => {
    try {
      console.log("Calling add_object with:", { object_type: type, position });
      const newObject = await invoke<MapObject>("add_object", {
        params: {
          object_type: type,
          position: position,
        },
      });
      console.log("Received new object:", newObject);
      addObject(newObject);
      await syncWithBackend();
      toast.success(
        `${type.charAt(0).toUpperCase() + type.slice(1)} added successfully!`,
        `Position: (${position.x.toFixed(1)}, ${position.y.toFixed(1)}, ${position.z.toFixed(1)})`,
      );
    } catch (error) {
      console.error("Failed to add object:", error);
      toast.error("Failed to add object");
    }
  };

  return (
    <div className="w-full h-full bg-gray-900 relative">
      <Canvas
        camera={{
          position: [camera.position.x, camera.position.y, camera.position.z],
          fov: 75,
        }}
        dpr={[1, 2]}
        gl={{
          antialias: true,
          alpha: false,
          powerPreference: "high-performance",
          stencil: false,
          depth: true,
        }}
        onClick={handleCanvasClick}
        onCreated={({ gl }) => {
          try {
            // Ensure the canvas has a proper background
            gl.setClearColor("#1a1a1a", 1.0);
            gl.shadowMap.enabled = true;
            gl.shadowMap.type = 2; // PCFSoftShadowMap
          } catch (error) {
            console.error("Error setting canvas properties:", error);
          }
        }}
        onError={(error) => {
          console.error("Canvas error:", error);
          toast.error("3D Editor failed to initialize");
        }}
        performance={{ min: 0.5 }} // Device pixel ratio
        raycaster={{
          params: {
            Line: { threshold: 0.1 },
            Points: { threshold: 0.1 },
            Mesh: { threshold: 0.1 },
            LOD: {},
            Sprite: {},
          },
          far: 1000, // Increase far distance for raycasting
          near: 0.1, // Set near distance for raycasting
        }} // Performance settings
        style={{
          background: "#1a1a1a",
          backgroundColor: "#1a1a1a", // Fallback for older browsers
        }}
      >
        <ViewportControls>
          <ViewportDisplayControls>
            <Scene />
          </ViewportDisplayControls>
        </ViewportControls>
      </Canvas>
    </div>
  );
}
