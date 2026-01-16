import { useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSceneStore } from "~/stores/scene-store";
import { useRendererConfig } from "./use-renderer-config";

/**
 * Hook to sync Zustand scene store to Bevy backend
 * Watches for changes and sends updates via Tauri IPC
 */
export function useBevySceneSync() {
  const { isBevy } = useRendererConfig();
  const sceneStore = useSceneStore();
  const syncTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Debounced scene sync function
  const syncScene = useCallback(async () => {
    if (!isBevy) return;

    try {
      const sceneState = {
        objects: sceneStore.objects.map((obj) => ({
          id: obj.id,
          name: obj.name,
          type: obj.type,
          position: obj.position,
          rotation: obj.rotation,
          scale: obj.scale,
          color: obj.color,
          visible: obj.visible,
        })),
        lights: sceneStore.lights.map((light) => ({
          id: light.id,
          name: light.name,
          type: light.type,
          position: light.position,
          color: light.color,
          intensity: light.intensity,
          visible: light.visible,
        })),
        camera_position: sceneStore.cameraPosition,
        camera_target: sceneStore.cameraTarget,
        backgroundColor: sceneStore.backgroundColor,
        fog_enabled: sceneStore.fogEnabled,
        fog_color: sceneStore.fogColor,
        fog_near: sceneStore.fogNear,
        fog_far: sceneStore.fogFar,
      };

      await invoke("sync_scene_to_bevy", { scene: sceneState });
    } catch (error) {
      console.error("Failed to sync scene to Bevy:", error);
    }
  }, [
    isBevy,
    sceneStore.objects,
    sceneStore.lights,
    sceneStore.cameraPosition,
    sceneStore.cameraTarget,
    sceneStore.backgroundColor,
    sceneStore.fogEnabled,
    sceneStore.fogColor,
    sceneStore.fogNear,
    sceneStore.fogFar,
  ]);

  // Sync scene when objects change
  useEffect(() => {
    if (!isBevy) return;

    // Debounce updates to avoid excessive IPC calls
    if (syncTimeoutRef.current) {
      clearTimeout(syncTimeoutRef.current);
    }

    syncTimeoutRef.current = setTimeout(() => {
      syncScene();
    }, 100); // 100ms debounce

    return () => {
      if (syncTimeoutRef.current) {
        clearTimeout(syncTimeoutRef.current);
      }
    };
  }, [isBevy, syncScene]);

  // Sync camera updates immediately (no debounce for responsiveness)
  useEffect(() => {
    if (!isBevy) return;

    const updateCamera = async () => {
      try {
        await invoke("update_bevy_camera", {
          position: sceneStore.cameraPosition,
          target: sceneStore.cameraTarget,
        });
      } catch (error) {
        console.error("Failed to update Bevy camera:", error);
      }
    };

    updateCamera();
  }, [sceneStore.cameraPosition, sceneStore.cameraTarget, isBevy]);

  // Initial sync on mount
  useEffect(() => {
    if (isBevy) {
      syncScene();
    }
  }, [isBevy, syncScene]); // Only sync when renderer changes
}
