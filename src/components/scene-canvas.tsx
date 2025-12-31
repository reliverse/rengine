import {
  GizmoHelper,
  GizmoViewport,
  Grid,
  OrbitControls,
  Stats,
} from "@react-three/drei";
import { Canvas, useThree } from "@react-three/fiber";
import { Suspense, useEffect, useRef, useState } from "react";
import * as THREE from "three";
import { Plane, Raycaster, Vector2, Vector3 } from "three";
import { useToast } from "~/hooks/use-toast";
import { cn } from "~/lib/utils";
import { useSceneStore } from "~/stores/scene-store";
import { type ImportProgress, modelImporter } from "~/utils/model-import";
import { PlacementPreview } from "./placement-preview";
import { SceneLights } from "./scene-lights";
import { SceneObjects } from "./scene-objects";

function MouseInteraction({
  controlsRef,
}: {
  // biome-ignore lint/suspicious/noExplicitAny: OrbitControls typing is complex
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

// biome-ignore lint/suspicious/noExplicitAny: OrbitControls typing is complex
function Scene({ controlsRef }: { controlsRef: React.RefObject<any> }) {
  const { gridVisible, gridSize, axesVisible, statsVisible } = useSceneStore();

  return (
    <>
      {/* Scene Lighting */}
      <SceneLights />

      {/* Grid */}
      {gridVisible && (
        <Grid
          args={[10.5, 10.5]}
          cellColor="#666666"
          cellSize={gridSize}
          cellThickness={0.5}
          fadeDistance={25}
          fadeStrength={1}
          followCamera={false}
          infiniteGrid={true}
          sectionColor="#888888"
          sectionSize={gridSize * 5}
          sectionThickness={1}
        />
      )}

      {/* Scene Objects */}
      <SceneObjects />

      {/* Scene Lights */}
      <SceneLights />

      {/* Placement Preview */}
      <PlacementPreview />

      {/* Mouse Interaction Handler */}
      <MouseInteraction controlsRef={controlsRef} />

      {/* Gizmos */}
      <GizmoHelper alignment="bottom-right" margin={[80, 80]}>
        <GizmoViewport
          axisColors={["red", "green", "blue"]}
          labelColor="black"
        />
      </GizmoHelper>

      {/* Additional helpers */}
      {axesVisible && <axesHelper args={[5]} />}
      {statsVisible && <Stats />}
    </>
  );
}

function LoadingFallback() {
  return (
    <div className="flex h-full items-center justify-center">
      <div className="text-center">
        <div className="mx-auto mb-2 h-8 w-8 animate-spin rounded-full border-primary border-b-2" />
        <p className="text-muted-foreground text-sm">Loading 3D scene...</p>
      </div>
    </div>
  );
}

export function SceneCanvas() {
  const {
    cameraPosition,
    cameraTarget,
    setCameraPosition,
    setCameraTarget,
    transformDragging,
    backgroundColor,
    fogEnabled,
    fogColor,
    fogNear,
    fogFar,
  } = useSceneStore();
  const [isDragOver, setIsDragOver] = useState(false);
  const [isImporting, setIsImporting] = useState(false);
  const { toast } = useToast();
  // biome-ignore lint/suspicious/noExplicitAny: OrbitControls typing is complex
  const controlsRef = useRef<any>(null);

  const handleControlsChange = () => {
    if (controlsRef.current) {
      const position: [number, number, number] = [
        controlsRef.current.object.position.x,
        controlsRef.current.object.position.y,
        controlsRef.current.object.position.z,
      ];
      const target: [number, number, number] = [
        controlsRef.current.target.x,
        controlsRef.current.target.y,
        controlsRef.current.target.z,
      ];

      setCameraPosition(position);
      setCameraTarget(target);
    }
  };

  const handleDragOver = (event: React.DragEvent) => {
    event.preventDefault();
    event.stopPropagation();
    if (!isImporting) {
      setIsDragOver(true);
    }
  };

  const handleDragLeave = (event: React.DragEvent) => {
    event.preventDefault();
    event.stopPropagation();
    setIsDragOver(false);
  };

  const handleDrop = async (event: React.DragEvent) => {
    event.preventDefault();
    event.stopPropagation();
    setIsDragOver(false);

    if (isImporting) {
      return;
    }

    const files = Array.from(event.dataTransfer.files);
    const modelFiles = files.filter((file) => {
      const ext = file.name.split(".").pop()?.toLowerCase();
      return ["gltf", "glb", "obj", "fbx"].includes(ext || "");
    });

    if (modelFiles.length === 0) {
      toast({
        title: "No compatible files",
        description: "Please drop GLTF, GLB, OBJ, or FBX files",
        variant: "destructive",
        duration: 3000,
      });
      return;
    }

    setIsImporting(true);

    try {
      for (const file of modelFiles) {
        // Calculate drop position based on camera and mouse position
        const dropPosition: [number, number, number] = [0, 0, 0]; // Default position, could be improved

        const result = await modelImporter.importFromFile(
          file,
          dropPosition,
          (_progress: ImportProgress) => {
            // Could show progress here if needed
          }
        );

        if (result.success && result.object) {
          useSceneStore.getState().addObject(result.object);

          toast({
            title: "Model imported",
            description: `${result.object.name} added to scene`,
            duration: 2000,
          });

          if (result.warnings && result.warnings.length > 0) {
            setTimeout(() => {
              toast({
                title: "Import warnings",
                description: result.warnings?.join("\n"),
                variant: "default",
                duration: 4000,
              });
            }, 1000);
          }
        } else {
          toast({
            title: "Import failed",
            description: result.error || `Failed to import ${file.name}`,
            variant: "destructive",
            duration: 4000,
          });
        }
      }
    } catch (error) {
      toast({
        title: "Import error",
        description: error instanceof Error ? error.message : "Unknown error",
        variant: "destructive",
        duration: 4000,
      });
    } finally {
      setIsImporting(false);
    }
  };

  return (
    // biome-ignore lint/a11y/noStaticElementInteractions: Drag-drop area needs event handlers
    // biome-ignore lint/a11y/noNoninteractiveElementInteractions: Drag-drop area needs event handlers
    <div
      className={cn(
        "relative h-full w-full",
        isDragOver && "ring-2 ring-blue-500 ring-opacity-50"
      )}
      onDragLeave={handleDragLeave}
      onDragOver={handleDragOver}
      onDrop={handleDrop}
      style={{ backgroundColor }}
    >
      {/* Drag overlay */}
      {isDragOver && (
        <div className="pointer-events-none absolute inset-0 z-10 flex items-center justify-center bg-blue-500 bg-opacity-10">
          <div className="rounded-lg bg-background/90 p-6 text-center backdrop-blur-sm">
            <div className="mb-2 text-4xl">📦</div>
            <div className="font-semibold text-lg">Drop 3D models here</div>
            <div className="text-muted-foreground text-sm">
              GLTF, GLB, OBJ, FBX supported
            </div>
          </div>
        </div>
      )}

      <Canvas
        camera={{
          position: cameraPosition,
          fov: 50,
          near: 0.1,
          far: 1000,
        }}
        gl={{
          antialias: true,
          alpha: false,
          powerPreference: "high-performance",
        }}
        onCreated={({ camera, scene }) => {
          camera.lookAt(...cameraTarget);
          // Set scene background color
          scene.background = new THREE.Color(backgroundColor);
        }}
        shadows
      >
        {/* Fog */}
        {fogEnabled && <fog args={[fogColor, fogNear, fogFar]} attach="fog" />}
        <Suspense fallback={null}>
          <Scene controlsRef={controlsRef} />
          <OrbitControls
            dampingFactor={0.05}
            enableDamping={true}
            enabled={!transformDragging}
            enablePan={true}
            enableRotate={true}
            enableZoom={true}
            maxDistance={100}
            minDistance={1}
            onChange={handleControlsChange}
            ref={controlsRef}
            target={cameraTarget}
          />
        </Suspense>
      </Canvas>

      {/* Overlay for when scene is loading */}
      <Suspense fallback={<LoadingFallback />}>
        <div />
      </Suspense>
    </div>
  );
}
