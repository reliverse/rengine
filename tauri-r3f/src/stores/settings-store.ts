import { create } from "zustand";
import { persist } from "zustand/middleware";
import { DEFAULT_PRECISION } from "~/lib/defaults";
import type { QualityLevelType } from "~/utils/adaptive-quality";

export interface GraphicsSettings {
  qualityLevel: QualityLevelType;
  adaptiveQualityEnabled: boolean;
  pixelRatio: number;
  antialias: boolean;
  shadows: boolean;
  postProcessing: boolean;
  textureQuality: number;
  lodDistance: number;
  cullingDistance: number;
  particleCount: number;
}

export interface SettingsState {
  precision: number;
  performanceMonitorEnabled: boolean;
  graphics: GraphicsSettings;
}

export interface SettingsActions {
  setPrecision: (precision: number) => void;
  setPerformanceMonitorEnabled: (enabled: boolean) => void;
  setGraphicsSettings: (settings: Partial<GraphicsSettings>) => void;
  setQualityLevel: (level: QualityLevelType) => void;
  setAdaptiveQualityEnabled: (enabled: boolean) => void;
}

export const useSettingsStore = create<SettingsState & SettingsActions>()(
  persist(
    (set) => ({
      // Default precision
      precision: DEFAULT_PRECISION,
      performanceMonitorEnabled: true, // Enabled by default

      // Graphics settings with defaults
      graphics: {
        qualityLevel: 2, // MEDIUM quality by default
        adaptiveQualityEnabled: false, // Disabled by default
        pixelRatio: 1,
        antialias: false,
        shadows: true,
        postProcessing: false,
        textureQuality: 0.75,
        lodDistance: 0.8,
        cullingDistance: 0.8,
        particleCount: 0.7,
      },

      setPrecision: (precision: number) => {
        set({ precision: Math.max(0, Math.min(10, precision)) }); // Clamp between 0-10
      },

      setPerformanceMonitorEnabled: (enabled: boolean) => {
        set({ performanceMonitorEnabled: enabled });
      },

      setGraphicsSettings: (settings: Partial<GraphicsSettings>) => {
        set((state) => ({
          graphics: { ...state.graphics, ...settings },
        }));
      },

      setQualityLevel: (level: QualityLevelType) => {
        set((state) => ({
          graphics: { ...state.graphics, qualityLevel: level },
        }));
      },

      setAdaptiveQualityEnabled: (enabled: boolean) => {
        set((state) => ({
          graphics: { ...state.graphics, adaptiveQualityEnabled: enabled },
        }));
      },
    }),
    {
      name: "rengine-settings",
    }
  )
);

// Selectors for commonly used derived state
export const usePrecision = () => useSettingsStore((state) => state.precision);
export const usePerformanceMonitorEnabled = () =>
  useSettingsStore((state) => state.performanceMonitorEnabled);
export const useGraphicsSettings = () =>
  useSettingsStore((state) => state.graphics);
export const useQualityLevel = () =>
  useSettingsStore((state) => state.graphics.qualityLevel);
export const useAdaptiveQualityEnabled = () =>
  useSettingsStore((state) => state.graphics.adaptiveQualityEnabled);

// Utility function to get step value based on precision
export const useStepValue = () => {
  const precision = usePrecision();
  return 1 / 10 ** precision;
};
