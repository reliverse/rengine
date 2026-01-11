import { OrbitControls } from "@react-three/drei";
import { Canvas, useThree } from "@react-three/fiber";
import {
  Suspense,
  useCallback,
  useEffect,
  useRef,
  useState,
  useTransition,
} from "react";
import * as THREE from "three";
import { usePreemptiveRender } from "~/hooks/use-preemptive-render";
import { useToast } from "~/hooks/use-toast";
import { cn } from "~/lib/utils";
import { useSceneStore } from "~/stores/scene-store";
import { type ImportProgress, modelImporter } from "~/utils/model-import";
import { PerformanceFooter } from "./performance-footer";
import { LoadingFallback } from "./scene/loading-fallback";
import { SceneRenderer } from "./scene/scene-renderer";

// Enable THREE.js ColorManagement for correct color representation
// This is especially important when using global geometries and materials
THREE.ColorManagement.enabled = true;

// Component to handle invalidate() calls for orbit controls with optional movement regression
function ControlsInvalidator(
  props: React.ComponentProps<typeof OrbitControls>
) {
  const { performance } = useThree();
  const { scheduleInteraction } = usePreemptiveRender();
  const controlsRef = useRef<any>(null);
  const { performanceRegressionOnMove } = useSceneStore();

  useEffect(() => {
    const controls = controlsRef.current;
    if (controls) {
      const handleChange = () => {
        // Use preemptive rendering for smooth controls
        scheduleInteraction(() => {
          // Only trigger performance regression during movement if enabled in settings
          if (performanceRegressionOnMove) {
            performance.regress();
          }
        });
      };
      controls.addEventListener("change", handleChange);
      return () => controls.removeEventListener("change", handleChange);
    }
  }, [performance, scheduleInteraction, performanceRegressionOnMove]);

  return <OrbitControls ref={controlsRef} {...props} />;
}

// Adaptive pixel ratio component for performance scaling
function AdaptivePixelRatio() {
  const { performance } = useThree();
  const setPixelRatio = useThree((state) => state.setDpr);

  useEffect(() => {
    setPixelRatio(window.devicePixelRatio * performance.current);
  }, [performance.current, setPixelRatio]);

  return null;
}

// Simple DPR value
const FIXED_DPR = 1.5;

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
  const controlsRef = useRef<any>(null);

  // React 18 concurrency for expensive operations
  const [_isPending, startTransition] = useTransition();

  // Fixed DPR for now
  const dpr = FIXED_DPR;

  const { scheduleUpdate } = usePreemptiveRender();

  const handleControlsChange = useCallback(() => {
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

      // Use preemptive rendering for camera updates
      scheduleUpdate(() => {
        setCameraPosition(position);
        setCameraTarget(target);
      });
    }
  }, [setCameraPosition, setCameraTarget, scheduleUpdate]);

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

  const handleDrop = useCallback(
    (event: React.DragEvent) => {
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
        // Use startTransition for expensive import operations
        // This allows React to interrupt and prioritize more urgent updates
        startTransition(async () => {
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
        });
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
    },
    [isImporting, toast]
  );

  return (
    <>
      {/* biome-ignore lint/a11y/noStaticElementInteractions: Drag-drop area needs event handlers */}
      {/* biome-ignore lint/a11y/noNoninteractiveElementInteractions: Drag-drop area needs event handlers */}
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
          dpr={dpr}
          frameloop="demand"
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
          {fogEnabled && (
            <fog args={[fogColor, fogNear, fogFar]} attach="fog" />
          )}
          <Suspense fallback={<LoadingFallback />}>
            <SceneRenderer controlsRef={controlsRef} />
            <ControlsInvalidator
              dampingFactor={0.05}
              enableDamping={true}
              enabled={!transformDragging}
              enablePan={true}
              enableRotate={true}
              enableZoom={true}
              maxDistance={100}
              minDistance={1}
              onChange={handleControlsChange}
              target={cameraTarget}
            />
            <AdaptivePixelRatio />
          </Suspense>
        </Canvas>
      </div>

      {/* Performance Footer - Always visible */}
      <PerformanceFooter />
    </>
  );
}
