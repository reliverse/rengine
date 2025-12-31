import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface SettingsState {
  precision: number;
}

export interface SettingsActions {
  setPrecision: (precision: number) => void;
}

export const useSettingsStore = create<SettingsState & SettingsActions>()(
  persist(
    (set) => ({
      // Default precision
      precision: 4,

      setPrecision: (precision: number) => {
        set({ precision: Math.max(0, Math.min(10, precision)) }); // Clamp between 0-10
      },
    }),
    {
      name: "rengine-settings",
    }
  )
);

// Selectors for commonly used derived state
export const usePrecision = () => useSettingsStore((state) => state.precision);

// Utility function to get step value based on precision
export const useStepValue = () => {
  const precision = usePrecision();
  return 1 / 10 ** precision;
};
