import { Environment } from "@react-three/drei";
import { useFrame, useThree } from "@react-three/fiber";
import { Suspense, useEffect, useMemo, useRef, useState } from "react";
import {
  CubeCamera,
  CubeTextureLoader,
  Euler,
  LinearMipmapLinearFilter,
  RGBFormat,
  Vector3,
  WebGLCubeRenderTarget,
} from "three";
import { useSceneStore } from "~/stores/scene-store";
import { setReflectionTexture } from "~/utils/reflection-manager";

// Valid drei Environment preset names
const DREI_PRESETS = [
  "apartment",
  "city",
  "dawn",
  "forest",
  "lobby",
  "night",
  "park",
  "studio",
  "sunset",
  "warehouse",
] as const;

// Environment preset definitions (drei presets + custom ones)
export const ENVIRONMENT_PRESETS = {
  // Valid drei presets (these use preset prop)
  apartment: "lebombo_1k.hdr",
  city: "potsdamer_platz_1k.hdr",
  dawn: "kiara_1_dawn_1k.hdr",
  forest: "forest_slope_1k.hdr",
  lobby: "st_fagans_interior_1k.hdr",
  night: "dikhololo_night_1k.hdr",
  park: "rooitou_park_1k.hdr",
  studio: "studio_small_03_1k.hdr",
  sunset: "venice_sunset_1k.hdr",
  warehouse: "empty_warehouse_01_1k.hdr",
  // Custom presets (these use files prop)
  citrus: "/skyboxes/citrus/citrus_orchard_road_puresky_4k.hdr",
  custom: null, // For custom HDRI files
  cosmic: {
    name: "Cosmic",
    urls: [
      "/skyboxes/cosmic/4.jpg", // positiveX
      "/skyboxes/cosmic/3.jpg", // negativeX
      "/skyboxes/cosmic/5.jpg", // positiveY
      "/skyboxes/cosmic/2.jpg", // negativeY
      "/skyboxes/cosmic/1.jpg", // positiveZ
      "/skyboxes/cosmic/6.jpg", // negativeZ
    ],
  },
} as const;

// Type for all available environment presets
export type EnvironmentPreset = keyof typeof ENVIRONMENT_PRESETS;

// Legacy cube environment component
function LegacyCubeEnvironment({
  urls,
  background,
  backgroundBlurriness,
  backgroundIntensity,
  backgroundRotation,
  environmentIntensity,
  environmentRotation,
}: {
  urls: readonly string[];
  background: boolean;
  backgroundBlurriness: number;
  backgroundIntensity: number;
  backgroundRotation: [number, number, number];
  environmentIntensity: number;
  environmentRotation: [number, number, number];
}) {
  const { scene } = useThree();
  const [cubeTexture, setCubeTexture] = useState<any>(null);

  useEffect(() => {
    const loader = new CubeTextureLoader();
    loader.load(
      [...urls], // Convert readonly array to mutable
      (texture) => {
        setCubeTexture(texture);
      },
      undefined,
      (error) => {
        console.error("Failed to load cube texture:", error);
      }
    );
  }, [urls]);

  useEffect(() => {
    if (cubeTexture) {
      if (background) {
        scene.background = cubeTexture;
        scene.backgroundIntensity = backgroundIntensity;
        scene.backgroundBlurriness = backgroundBlurriness;
        scene.backgroundRotation = new Euler(...backgroundRotation);
      }

      // Set environment for reflections
      scene.environment = cubeTexture;
      scene.environmentIntensity = environmentIntensity;
      if (environmentRotation) {
        // Note: Three.js scene.environment doesn't have rotation property
        // This would need custom handling if rotation is needed
      }
    }

    return () => {
      if (cubeTexture) {
        cubeTexture.dispose();
      }
    };
  }, [
    cubeTexture,
    background,
    backgroundIntensity,
    backgroundBlurriness,
    backgroundRotation,
    environmentIntensity,
    environmentRotation,
    scene,
  ]);

  return null;
}

export function Skybox() {
  const { scene, gl, camera } = useThree();
  const {
    skyboxEnabled,
    skyboxPreset,
    skyboxIntensity,
    environmentIntensity,
    backgroundBlurriness,
    environmentRotation,
    backgroundRotation,
    groundProjection,
    groundProjectionHeight,
    groundProjectionRadius,
    groundProjectionScale,
    customEnvironmentFile,
    liveEnvironment,
    liveEnvironmentResolution,
    cameraShakeEnabled,
    cameraShakeIntensity,
    cameraShakeSpeed,
  } = useSceneStore();

  // Cube camera for reflections and live environments
  const cubeCameraRef = useRef<CubeCamera | null>(null);
  const cubeRenderTargetRef = useRef<WebGLCubeRenderTarget | null>(null);

  // Camera shake state
  const [cameraShakeOffset] = useState(() => new Vector3());
  const [cameraShakeTime, setCameraShakeTime] = useState(0);

  // Initialize cube camera and render target for reflections
  useEffect(() => {
    if ((skyboxEnabled || liveEnvironment) && !cubeRenderTargetRef.current) {
      const resolution = liveEnvironment ? liveEnvironmentResolution : 256;
      cubeRenderTargetRef.current = new WebGLCubeRenderTarget(resolution, {
        format: RGBFormat,
        generateMipmaps: true,
        minFilter: LinearMipmapLinearFilter,
      });
      cubeCameraRef.current = new CubeCamera(
        1,
        1000,
        cubeRenderTargetRef.current
      );
      cubeCameraRef.current.position.set(0, 100, 0);
      scene.add(cubeCameraRef.current);
    } else if (!(skyboxEnabled || liveEnvironment) && cubeCameraRef.current) {
      scene.remove(cubeCameraRef.current);
      cubeCameraRef.current = null;
      if (cubeRenderTargetRef.current) {
        cubeRenderTargetRef.current.dispose();
        cubeRenderTargetRef.current = null;
      }
    }

    return () => {
      if (cubeCameraRef.current) {
        scene.remove(cubeCameraRef.current);
      }
      if (cubeRenderTargetRef.current) {
        cubeRenderTargetRef.current.dispose();
      }
    };
  }, [scene, skyboxEnabled, liveEnvironment, liveEnvironmentResolution]);

  // Update cube camera every frame for dynamic reflections and live environments
  useFrame((_state, delta) => {
    // Camera shake
    if (cameraShakeEnabled) {
      setCameraShakeTime((prev) => prev + delta * cameraShakeSpeed);
      const shakeX = Math.sin(cameraShakeTime * 2) * cameraShakeIntensity;
      const shakeY = Math.cos(cameraShakeTime * 1.5) * cameraShakeIntensity;
      const shakeZ =
        Math.sin(cameraShakeTime * 0.8) * cameraShakeIntensity * 0.5;

      cameraShakeOffset.set(shakeX, shakeY, shakeZ);
      camera.position.add(cameraShakeOffset);
      camera.lookAt(0, 0, 0); // Re-center look direction
      camera.position.sub(cameraShakeOffset);
    }

    // Update cube camera for live environments
    if (cubeCameraRef.current && liveEnvironment) {
      cubeCameraRef.current.update(gl, scene);
      // Update global reflection texture
      setReflectionTexture(cubeCameraRef.current.renderTarget.texture);
    } else if (!liveEnvironment) {
      setReflectionTexture(null);
    }
  });

  // Determine environment source and type
  const { source, type } = useMemo(() => {
    if (!skyboxEnabled) return { source: null, type: "none" as const };

    // Check if it's a legacy preset (cosmic)
    if (skyboxPreset === "cosmic") {
      return { source: ENVIRONMENT_PRESETS.cosmic, type: "legacy" as const };
    }

    // Check if it's a drei preset (one of the valid preset names)
    if (DREI_PRESETS.includes(skyboxPreset as any)) {
      return { source: skyboxPreset, type: "preset" as const };
    }

    // Check for custom file presets (citrus) or user uploaded files
    const presetFile =
      ENVIRONMENT_PRESETS[skyboxPreset as keyof typeof ENVIRONMENT_PRESETS];
    if (typeof presetFile === "string" && presetFile.startsWith("/")) {
      return { source: presetFile, type: "file" as const };
    }

    // Check for user-uploaded custom files
    if (customEnvironmentFile || skyboxPreset === "custom") {
      return { source: customEnvironmentFile, type: "file" as const };
    }

    return { source: null, type: "none" as const };
  }, [skyboxEnabled, skyboxPreset, customEnvironmentFile]);

  // Ground projection configuration
  const groundProjectionConfig = useMemo(() => {
    if (!groundProjection) return undefined;

    return {
      height: groundProjectionHeight,
      radius: groundProjectionRadius,
      scale: groundProjectionScale,
    };
  }, [
    groundProjection,
    groundProjectionHeight,
    groundProjectionRadius,
    groundProjectionScale,
  ]);

  // Only render Environment component if enabled
  if (!skyboxEnabled) {
    return null;
  }

  return (
    <Suspense fallback={null}>
      {type === "legacy" ? (
        // Legacy cube texture support
        <LegacyCubeEnvironment
          background={true}
          backgroundBlurriness={backgroundBlurriness}
          backgroundIntensity={skyboxIntensity}
          backgroundRotation={backgroundRotation}
          environmentIntensity={environmentIntensity}
          environmentRotation={environmentRotation}
          urls={source.urls}
        />
      ) : (
        // Standard drei Environment with presets or custom files
        <Environment
          background={true}
          backgroundBlurriness={backgroundBlurriness}
          backgroundIntensity={skyboxIntensity}
          backgroundRotation={backgroundRotation}
          environmentIntensity={environmentIntensity}
          environmentRotation={environmentRotation}
          files={type === "file" ? (source as string) : undefined}
          ground={groundProjectionConfig}
          preset={type === "preset" ? (source as any) : undefined}
        >
          {/* Support for live/custom environments */}
          {liveEnvironment && (
            <mesh scale={100}>
              <sphereGeometry args={[1, 64, 64]} />
              <meshBasicMaterial
                map={cubeCameraRef.current?.renderTarget.texture || undefined}
                side={2} // THREE.BackSide
              />
            </mesh>
          )}
        </Environment>
      )}
    </Suspense>
  );
}
