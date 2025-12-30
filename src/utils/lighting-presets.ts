import type { SceneLight } from "~/stores/scene-store";

export interface LightingPreset {
  name: string;
  description: string;
  lights: Omit<SceneLight, "id" | "name">[];
}

export const LIGHTING_PRESETS: Record<string, LightingPreset> = {
  default: {
    name: "Default",
    description: "Basic ambient and directional lighting",
    lights: [
      {
        type: "ambient",
        color: "#ffffff",
        intensity: 0.6,
        position: [0, 0, 0],
        visible: true,
        castShadow: false,
      },
      {
        type: "directional",
        color: "#ffffff",
        intensity: 1,
        position: [10, 10, 5],
        target: [0, 0, 0],
        visible: true,
        castShadow: true,
        shadowBias: -0.0001,
        shadowMapSize: 2048,
        shadowNear: 0.1,
        shadowFar: 100,
        shadowRadius: 8,
      },
    ],
  },

  studio: {
    name: "Studio",
    description: "Soft, even lighting for product photography",
    lights: [
      {
        type: "ambient",
        color: "#f0f0f0",
        intensity: 0.4,
        position: [0, 0, 0],
        visible: true,
        castShadow: false,
      },
      {
        type: "directional",
        color: "#ffffff",
        intensity: 0.8,
        position: [5, 8, 5],
        target: [0, 0, 0],
        visible: true,
        castShadow: true,
        shadowBias: -0.000_05,
        shadowMapSize: 2048,
        shadowNear: 0.1,
        shadowFar: 50,
        shadowRadius: 4,
      },
      {
        type: "directional",
        color: "#e0e0ff",
        intensity: 0.3,
        position: [-5, 3, 5],
        target: [0, 0, 0],
        visible: true,
        castShadow: false,
      },
      {
        type: "point",
        color: "#fff0e0",
        intensity: 0.5,
        position: [3, 2, 3],
        visible: true,
        castShadow: true,
        distance: 20,
        decay: 2,
        shadowBias: 0.0001,
        shadowMapSize: 1024,
        shadowNear: 0.1,
        shadowFar: 20,
        shadowRadius: 2,
      },
    ],
  },

  outdoor: {
    name: "Outdoor",
    description: "Bright sunlight with sky lighting",
    lights: [
      {
        type: "hemisphere",
        color: "#87CEEB", // Sky blue
        intensity: 0.6,
        position: [0, 0, 0],
        visible: true,
        castShadow: false,
        groundColor: "#8B4513", // Brown ground
      },
      {
        type: "directional",
        color: "#FFF8DC", // Warm sunlight
        intensity: 1.2,
        position: [10, 20, 10],
        target: [0, 0, 0],
        visible: true,
        castShadow: true,
        shadowBias: -0.0002,
        shadowMapSize: 4096,
        shadowNear: 0.1,
        shadowFar: 200,
        shadowRadius: 16,
      },
    ],
  },

  indoor: {
    name: "Indoor",
    description: "Warm indoor lighting with multiple light sources",
    lights: [
      {
        type: "ambient",
        color: "#ffeedd",
        intensity: 0.3,
        position: [0, 0, 0],
        visible: true,
        castShadow: false,
      },
      {
        type: "point",
        color: "#fff4e0",
        intensity: 1,
        position: [0, 4, 0],
        visible: true,
        castShadow: true,
        distance: 15,
        decay: 2,
        shadowBias: 0.0001,
        shadowMapSize: 1024,
        shadowNear: 0.1,
        shadowFar: 15,
        shadowRadius: 2,
      },
      {
        type: "point",
        color: "#ffe4b5",
        intensity: 0.6,
        position: [3, 2, 3],
        visible: true,
        castShadow: true,
        distance: 10,
        decay: 2,
        shadowBias: 0.0001,
        shadowMapSize: 1024,
        shadowNear: 0.1,
        shadowFar: 10,
        shadowRadius: 1,
      },
      {
        type: "point",
        color: "#ffe4b5",
        intensity: 0.6,
        position: [-3, 2, 3],
        visible: true,
        castShadow: true,
        distance: 10,
        decay: 2,
        shadowBias: 0.0001,
        shadowMapSize: 1024,
        shadowNear: 0.1,
        shadowFar: 10,
        shadowRadius: 1,
      },
    ],
  },

  sunset: {
    name: "Sunset",
    description: "Warm golden hour lighting",
    lights: [
      {
        type: "hemisphere",
        color: "#FF6B35", // Orange sky
        intensity: 0.4,
        position: [0, 0, 0],
        visible: true,
        castShadow: false,
        groundColor: "#2D1B69", // Dark blue ground
      },
      {
        type: "directional",
        color: "#FF8C42", // Warm orange
        intensity: 1.5,
        position: [5, 8, -10],
        target: [0, 0, 0],
        visible: true,
        castShadow: true,
        shadowBias: -0.0003,
        shadowMapSize: 4096,
        shadowNear: 0.1,
        shadowFar: 150,
        shadowRadius: 12,
      },
      {
        type: "directional",
        color: "#FFE5B4", // Soft warm fill
        intensity: 0.3,
        position: [-8, 5, 8],
        target: [0, 0, 0],
        visible: true,
        castShadow: false,
      },
    ],
  },

  dramatic: {
    name: "Dramatic",
    description: "High contrast lighting with strong shadows",
    lights: [
      {
        type: "directional",
        color: "#ffffff",
        intensity: 2,
        position: [8, 12, 6],
        target: [0, 0, 0],
        visible: true,
        castShadow: true,
        shadowBias: -0.0005,
        shadowMapSize: 4096,
        shadowNear: 0.1,
        shadowFar: 100,
        shadowRadius: 20,
      },
      {
        type: "ambient",
        color: "#404040",
        intensity: 0.1,
        position: [0, 0, 0],
        visible: true,
        castShadow: false,
      },
    ],
  },

  neon: {
    name: "Neon",
    description: "Colorful neon-style lighting",
    lights: [
      {
        type: "ambient",
        color: "#2a2a4a",
        intensity: 0.2,
        position: [0, 0, 0],
        visible: true,
        castShadow: false,
      },
      {
        type: "point",
        color: "#ff0080",
        intensity: 1.5,
        position: [5, 3, 5],
        visible: true,
        castShadow: true,
        distance: 20,
        decay: 2,
        shadowBias: 0.0001,
        shadowMapSize: 1024,
        shadowNear: 0.1,
        shadowFar: 20,
        shadowRadius: 3,
      },
      {
        type: "point",
        color: "#00ffff",
        intensity: 1.5,
        position: [-5, 3, 5],
        visible: true,
        castShadow: true,
        distance: 20,
        decay: 2,
        shadowBias: 0.0001,
        shadowMapSize: 1024,
        shadowNear: 0.1,
        shadowFar: 20,
        shadowRadius: 3,
      },
      {
        type: "point",
        color: "#ffff00",
        intensity: 1.5,
        position: [0, 3, -5],
        visible: true,
        castShadow: true,
        distance: 20,
        decay: 2,
        shadowBias: 0.0001,
        shadowMapSize: 1024,
        shadowNear: 0.1,
        shadowFar: 20,
        shadowRadius: 3,
      },
    ],
  },

  moonlight: {
    name: "Moonlight",
    description: "Cool blue moonlight with subtle shadows",
    lights: [
      {
        type: "hemisphere",
        color: "#4A90E2", // Cool blue sky
        intensity: 0.3,
        position: [0, 0, 0],
        visible: true,
        castShadow: false,
        groundColor: "#2C3E50", // Dark ground
      },
      {
        type: "directional",
        color: "#E8F4FD", // Cool white moonlight
        intensity: 0.8,
        position: [10, 15, 8],
        target: [0, 0, 0],
        visible: true,
        castShadow: true,
        shadowBias: -0.0001,
        shadowMapSize: 2048,
        shadowNear: 0.1,
        shadowFar: 100,
        shadowRadius: 8,
      },
    ],
  },
};

export function applyLightingPreset(presetId: string): SceneLight[] {
  const preset = LIGHTING_PRESETS[presetId];
  if (!preset) {
    throw new Error(`Lighting preset "${presetId}" not found`);
  }

  return preset.lights.map((light, index) => ({
    ...light,
    id: `light_preset_${presetId}_${index}_${Date.now()}`,
    name: `${preset.name} Light ${index + 1}`,
  }));
}

export function getPresetList(): Array<{
  id: string;
  name: string;
  description: string;
}> {
  return Object.entries(LIGHTING_PRESETS).map(([id, preset]) => ({
    id,
    name: preset.name,
    description: preset.description,
  }));
}
