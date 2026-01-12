import { useFrame, useThree } from "@react-three/fiber";
import { useEffect, useMemo, useRef } from "react";
import {
  CubeCamera,
  CubeTextureLoader,
  LinearMipmapLinearFilter,
  RGBFormat,
  WebGLCubeRenderTarget,
} from "three";
import { useSceneStore } from "~/stores/scene-store";
import { setReflectionTexture } from "~/utils/reflection-manager";

// Skybox preset definitions
export const SKYBOX_PRESETS = {
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
  // Add more presets here in the future
} as const;

export type SkyboxPreset = keyof typeof SKYBOX_PRESETS;

export function Skybox() {
  const { scene, gl } = useThree();
  const { skyboxEnabled, skyboxPreset, skyboxIntensity } = useSceneStore();

  // Cube camera for reflections
  const cubeCameraRef = useRef<CubeCamera | null>(null);
  const cubeRenderTargetRef = useRef<WebGLCubeRenderTarget | null>(null);

  // Initialize cube camera and render target for reflections
  useEffect(() => {
    if (skyboxEnabled && !cubeRenderTargetRef.current) {
      cubeRenderTargetRef.current = new WebGLCubeRenderTarget(256, {
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
    } else if (!skyboxEnabled && cubeCameraRef.current) {
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
  }, [scene, skyboxEnabled]);

  // Update cube camera every frame for dynamic reflections
  useFrame(() => {
    if (cubeCameraRef.current && skyboxEnabled) {
      cubeCameraRef.current.update(gl, scene);
      // Update global reflection texture
      setReflectionTexture(cubeCameraRef.current.renderTarget.texture);
    } else if (!skyboxEnabled) {
      setReflectionTexture(null);
    }
  });

  // Load skybox texture
  const skyboxTexture = useMemo(() => {
    if (!(skyboxEnabled && SKYBOX_PRESETS[skyboxPreset as SkyboxPreset])) {
      return null;
    }

    const preset = SKYBOX_PRESETS[skyboxPreset as SkyboxPreset];
    const loader = new CubeTextureLoader();
    return loader.load(preset.urls);
  }, [skyboxEnabled, skyboxPreset]);

  // Apply skybox to scene background
  useEffect(() => {
    if (skyboxTexture && skyboxEnabled) {
      scene.background = skyboxTexture;
      scene.backgroundIntensity = skyboxIntensity;
    } else if (!skyboxEnabled) {
      // Reset to solid background color when skybox is disabled
      scene.background = null; // This will let the canvas background show through
      scene.backgroundIntensity = 1;
    }
  }, [scene, skyboxTexture, skyboxEnabled, skyboxIntensity]);

  // Cleanup texture on unmount
  useEffect(() => {
    return () => {
      if (skyboxTexture) {
        skyboxTexture.dispose();
      }
    };
  }, [skyboxTexture]);

  return null;
}
