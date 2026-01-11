import { GizmoHelper, GizmoViewport, Grid, Stats } from "@react-three/drei";
import { useFrame, useThree } from "@react-three/fiber";
import { useEffect } from "react";
import { useSceneStore } from "~/stores/scene-store";
import { performanceMonitor } from "~/utils/performance-monitor";
import { PlacementPreview } from "../placement-preview";
import { SceneLights } from "../scene-lights";
import { SceneObjects } from "../scene-objects";
import { MouseInteraction } from "./mouse-interaction";

export function SceneRenderer({
  controlsRef,
}: {
  controlsRef: React.RefObject<any>;
}) {
  const { gridVisible, gridSize, axesVisible, statsVisible } = useSceneStore();
  const { gl } = useThree();

  // Set up performance monitoring
  useEffect(() => {
    performanceMonitor.setRenderer(gl);
  }, [gl]);

  // Update performance monitor every frame
  useFrame(() => {
    performanceMonitor.update();
  });

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
