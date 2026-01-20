import { Html, Line, Sphere } from "@react-three/drei";
import type React from "react";
import { useEffect, useMemo, useRef, useState } from "react";
import type * as THREE from "three";
import {
  Color,
  type DirectionalLight as ThreeDirectionalLight,
  type SpotLight as ThreeSpotLight,
} from "three";
import { type SceneLight, useSceneStore } from "~/stores/scene-store";

interface LightGizmoProps {
  light: SceneLight;
  isSelected: boolean;
  onSelect: (id: string) => void;
}

function LightGizmo({ light, isSelected, onSelect }: LightGizmoProps) {
  const [hovered, setHovered] = useState(false);

  const handleClick = (e: React.MouseEvent<THREE.Group>) => {
    e.stopPropagation();
    onSelect(light.id);
  };

  const handlePointerOver = () => setHovered(true);
  const handlePointerOut = () => setHovered(false);

  return (
    <group
      onClick={handleClick}
      onPointerOut={handlePointerOut}
      onPointerOver={handlePointerOver}
      position={light.position}
    >
      {/* Light position indicator */}
      <Sphere args={[0.05]} onClick={handleClick}>
        <meshBasicMaterial
          color={isSelected || hovered ? "#ff6b35" : light.color}
          opacity={0.8}
          transparent
        />
      </Sphere>

      {/* Light direction indicator for directional and spot lights */}
      {light.type === "directional" && light.target && (
        <Line
          color={isSelected || hovered ? "#ff6b35" : light.color}
          lineWidth={2}
          points={[
            [0, 0, 0],
            [
              light.target[0] - light.position[0],
              light.target[1] - light.position[1],
              light.target[2] - light.position[2],
            ],
          ]}
        />
      )}

      {light.type === "spot" && light.target && (
        <Line
          color={isSelected || hovered ? "#ff6b35" : light.color}
          lineWidth={2}
          points={[
            [0, 0, 0],
            [
              light.target[0] - light.position[0],
              light.target[1] - light.position[1],
              light.target[2] - light.position[2],
            ],
          ]}
        />
      )}

      {/* Light name label */}
      {(isSelected || hovered) && (
        <Html distanceFactor={10}>
          <div className="whitespace-nowrap rounded bg-background/90 px-2 py-1 font-medium text-foreground text-xs">
            {light.name}
          </div>
        </Html>
      )}
    </group>
  );
}

interface DirectionalLightComponentProps {
  light: SceneLight;
}

function DirectionalLightComponent({ light }: DirectionalLightComponentProps) {
  const lightRef = useRef<ThreeDirectionalLight>(null);

  useEffect(() => {
    if (lightRef.current && light.target) {
      lightRef.current.target.position.set(...light.target);
      lightRef.current.target.updateMatrixWorld();
    }
  }, [light.target]);

  if (!light.visible) {
    return null;
  }

  return (
    <directionalLight
      castShadow={light.castShadow}
      color={new Color(light.color)}
      intensity={light.intensity}
      position={light.position}
      ref={lightRef}
      shadow-bias={light.shadowBias}
      shadow-camera-far={light.shadowFar}
      shadow-camera-near={light.shadowNear}
      shadow-mapSize={[
        light.shadowMapSize || 2048,
        light.shadowMapSize || 2048,
      ]}
      shadow-radius={light.shadowRadius}
    />
  );
}

interface PointLightComponentProps {
  light: SceneLight;
}

function PointLightComponent({ light }: PointLightComponentProps) {
  if (!light.visible) {
    return null;
  }

  return (
    <pointLight
      castShadow={light.castShadow}
      color={new Color(light.color)}
      decay={light.decay}
      distance={light.distance}
      intensity={light.intensity}
      position={light.position}
      shadow-bias={light.shadowBias}
      shadow-camera-far={light.shadowFar}
      shadow-camera-near={light.shadowNear}
      shadow-mapSize={[
        light.shadowMapSize || 1024,
        light.shadowMapSize || 1024,
      ]}
      shadow-radius={light.shadowRadius}
    />
  );
}

interface SpotLightComponentProps {
  light: SceneLight;
}

function SpotLightComponent({ light }: SpotLightComponentProps) {
  const lightRef = useRef<ThreeSpotLight>(null);

  useEffect(() => {
    if (lightRef.current && light.target) {
      lightRef.current.target.position.set(...light.target);
      lightRef.current.target.updateMatrixWorld();
    }
  }, [light.target]);

  if (!light.visible) {
    return null;
  }

  return (
    <spotLight
      angle={light.angle}
      castShadow={light.castShadow}
      color={new Color(light.color)}
      decay={light.decay}
      distance={light.distance}
      intensity={light.intensity}
      penumbra={light.penumbra}
      position={light.position}
      ref={lightRef}
      shadow-bias={light.shadowBias}
      shadow-camera-far={light.shadowFar}
      shadow-camera-near={light.shadowNear}
      shadow-mapSize={[
        light.shadowMapSize || 1024,
        light.shadowMapSize || 1024,
      ]}
      shadow-radius={light.shadowRadius}
    />
  );
}

interface AmbientLightComponentProps {
  light: SceneLight;
}

function AmbientLightComponent({ light }: AmbientLightComponentProps) {
  if (!light.visible) {
    return null;
  }

  return (
    <ambientLight color={new Color(light.color)} intensity={light.intensity} />
  );
}

interface HemisphereLightComponentProps {
  light: SceneLight;
}

function HemisphereLightComponent({ light }: HemisphereLightComponentProps) {
  if (!light.visible) {
    return null;
  }

  return (
    <hemisphereLight
      color={new Color(light.color)}
      groundColor={new Color(light.groundColor || "#444444")}
      intensity={light.intensity}
    />
  );
}

interface SceneLightComponentProps {
  light: SceneLight;
}

function SceneLightComponent({ light }: SceneLightComponentProps) {
  const { selectedLightIds, selectLight } = useSceneStore();
  const isSelected = selectedLightIds.includes(light.id);

  const lightComponent = useMemo(() => {
    switch (light.type) {
      case "directional":
        return <DirectionalLightComponent light={light} />;
      case "point":
        return <PointLightComponent light={light} />;
      case "spot":
        return <SpotLightComponent light={light} />;
      case "ambient":
        return <AmbientLightComponent light={light} />;
      case "hemisphere":
        return <HemisphereLightComponent light={light} />;
      default:
        return null;
    }
  }, [light]);

  return (
    <>
      {lightComponent}
      <LightGizmo
        isSelected={isSelected}
        light={light}
        onSelect={selectLight}
      />
    </>
  );
}

export function SceneLights() {
  const lights = useSceneStore((state) => state.lights);
  const lightsVisible = useSceneStore((state) => state.lightsVisible);

  if (!lightsVisible) {
    // Still render ambient and hemisphere lights when lights are hidden
    return (
      <>
        {lights
          .filter(
            (light) => light.type === "ambient" || light.type === "hemisphere"
          )
          .map((light) => (
            <SceneLightComponent key={light.id} light={light} />
          ))}
      </>
    );
  }

  return (
    <>
      {lights.map((light) => (
        <SceneLightComponent key={light.id} light={light} />
      ))}
    </>
  );
}
