import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export type RendererBackend = "r3f";

export interface RengineConfig {
  renderer: RendererBackend;
  target_fps: number;
  antialias: boolean;
  shadows: boolean;
  pixel_ratio: number;
  debug: boolean;
  log_level: string;
}

/**
 * Hook to read and monitor rengine.json configuration
 * Returns the current renderer selection and config
 */
export function useRendererConfig() {
  const [config, setConfig] = useState<RengineConfig | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    async function loadConfig() {
      try {
        const loadedConfig = await invoke<RengineConfig>(
          "load_rengine_config",
          {
            configPath: null,
          }
        );
        if (mounted) {
          setConfig(loadedConfig);
          setError(null);
        }
      } catch (err) {
        if (mounted) {
          setError(err instanceof Error ? err.message : String(err));
          // Use defaults on error
          setConfig({
            renderer: "r3f",
            target_fps: 60,
            antialias: true,
            shadows: true,
            pixel_ratio: 1.0,
            debug: false,
            log_level: "info",
          });
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    }

    loadConfig();

    return () => {
      mounted = false;
    };
  }, []);

  const effectiveRenderer = (): RendererBackend => {
    if (!config) return "r3f";
    return config.renderer;
  };

  return {
    config,
    renderer: effectiveRenderer(),
    loading,
    error,
    isR3f: effectiveRenderer() === "r3f",
  };
}
