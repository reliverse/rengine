import type { MaterialPreset } from "~/types/materials";

/**
 * Material presets for common materials
 */
export const MATERIAL_PRESETS: MaterialPreset[] = [
  // Basic Materials
  {
    id: "basic-white",
    name: "White Basic",
    category: "Basic",
    description: "Simple white material with no special effects",
    properties: {
      type: "basic",
      color: "#ffffff",
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2, // DoubleSide
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "basic-black",
    name: "Black Basic",
    category: "Basic",
    description: "Simple black material",
    properties: {
      type: "basic",
      color: "#000000",
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  // Standard PBR Materials
  {
    id: "pbr-plastic-white",
    name: "White Plastic",
    category: "Plastic",
    description: "Smooth white plastic material",
    properties: {
      type: "standard",
      color: "#ffffff",
      metalness: 0.0,
      roughness: 0.3,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "pbr-plastic-red",
    name: "Red Plastic",
    category: "Plastic",
    description: "Smooth red plastic material",
    properties: {
      type: "standard",
      color: "#ff0000",
      metalness: 0.0,
      roughness: 0.3,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "pbr-plastic-blue",
    name: "Blue Plastic",
    category: "Plastic",
    description: "Smooth blue plastic material",
    properties: {
      type: "standard",
      color: "#0088ff",
      metalness: 0.0,
      roughness: 0.3,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  // Metal Materials
  {
    id: "metal-gold",
    name: "Gold",
    category: "Metal",
    description: "Shiny gold metal",
    properties: {
      type: "standard",
      color: "#ffd700",
      metalness: 1.0,
      roughness: 0.1,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "metal-silver",
    name: "Silver",
    category: "Metal",
    description: "Polished silver metal",
    properties: {
      type: "standard",
      color: "#c0c0c0",
      metalness: 1.0,
      roughness: 0.05,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "metal-steel",
    name: "Steel",
    category: "Metal",
    description: "Brushed steel metal",
    properties: {
      type: "standard",
      color: "#8c8c8c",
      metalness: 0.8,
      roughness: 0.2,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "metal-copper",
    name: "Copper",
    category: "Metal",
    description: "Aged copper metal",
    properties: {
      type: "standard",
      color: "#b87333",
      metalness: 0.9,
      roughness: 0.15,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  // Wood Materials
  {
    id: "wood-oak",
    name: "Oak Wood",
    category: "Wood",
    description: "Natural oak wood texture",
    properties: {
      type: "standard",
      color: "#8b4513",
      metalness: 0.0,
      roughness: 0.8,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "wood-pine",
    name: "Pine Wood",
    category: "Wood",
    description: "Light pine wood",
    properties: {
      type: "standard",
      color: "#deb887",
      metalness: 0.0,
      roughness: 0.7,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  // Glass Materials
  {
    id: "glass-clear",
    name: "Clear Glass",
    category: "Glass",
    description: "Transparent clear glass",
    properties: {
      type: "physical",
      color: "#ffffff",
      metalness: 0.0,
      roughness: 0.0,
      opacity: 0.1,
      transparent: true,
      transmission: 1.0,
      thickness: 0.1,
      ior: 1.5,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: false,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "glass-tinted",
    name: "Tinted Glass",
    category: "Glass",
    description: "Blue tinted glass",
    properties: {
      type: "physical",
      color: "#87ceeb",
      metalness: 0.0,
      roughness: 0.0,
      opacity: 0.3,
      transparent: true,
      transmission: 0.8,
      thickness: 0.1,
      ior: 1.5,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: false,
      flatShading: false,
      wireframe: false,
    },
  },

  // Fabric Materials
  {
    id: "fabric-cloth",
    name: "Cloth",
    category: "Fabric",
    description: "Soft cloth material",
    properties: {
      type: "standard",
      color: "#f5f5dc",
      metalness: 0.0,
      roughness: 0.9,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "fabric-leather",
    name: "Leather",
    category: "Fabric",
    description: "Brown leather material",
    properties: {
      type: "standard",
      color: "#8b4513",
      metalness: 0.0,
      roughness: 0.6,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  // Stone Materials
  {
    id: "stone-marble",
    name: "Marble",
    category: "Stone",
    description: "White marble stone",
    properties: {
      type: "standard",
      color: "#f8f8ff",
      metalness: 0.0,
      roughness: 0.1,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "stone-concrete",
    name: "Concrete",
    category: "Stone",
    description: "Gray concrete material",
    properties: {
      type: "standard",
      color: "#808080",
      metalness: 0.0,
      roughness: 0.9,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  // Special Materials
  {
    id: "special-emissive",
    name: "Emissive",
    category: "Special",
    description: "Glowing material",
    properties: {
      type: "standard",
      color: "#ffffff",
      metalness: 0.0,
      roughness: 0.5,
      emissive: "#ffffff",
      emissiveIntensity: 1.0,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "special-mirror",
    name: "Mirror",
    category: "Special",
    description: "Highly reflective mirror surface",
    properties: {
      type: "standard",
      color: "#ffffff",
      metalness: 1.0,
      roughness: 0.0,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "toon-basic",
    name: "Toon Shader",
    category: "Toon",
    description: "Cartoon-style shading",
    properties: {
      type: "toon",
      color: "#ff6b6b",
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  // Custom Material Presets
  {
    id: "skin-realistic",
    name: "Realistic Skin",
    category: "Organic",
    description: "Human skin with subsurface scattering",
    properties: {
      type: "standard" as const,
      color: "#ffdbac",
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "glass-crystal",
    name: "Crystal Glass",
    category: "Glass",
    description: "Clear crystal glass with refraction",
    properties: {
      type: "physical" as const,
      color: "#ffffff",
      opacity: 0.1,
      transparent: true,
      transmission: 1.0,
      ior: 1.5,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: false,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "fabric-silk",
    name: "Silk Fabric",
    category: "Fabric",
    description: "Smooth, shiny silk material",
    properties: {
      type: "lambert" as const,
      color: "#ffffff",
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "gem-diamond",
    name: "Diamond",
    category: "Gem",
    description: "Brilliant diamond gemstone",
    properties: {
      type: "physical" as const,
      color: "#ffffff",
      opacity: 0.9,
      transparent: true,
      transmission: 0.1,
      ior: 2.42,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "paint-metallic-red",
    name: "Metallic Red Paint",
    category: "Paint",
    description: "Automotive metallic red paint",
    properties: {
      type: "standard" as const,
      color: "#cc0000",
      metalness: 0.8,
      roughness: 0.2,
      clearcoat: 1.0,
      clearcoatRoughness: 0.1,
      opacity: 1.0,
      transparent: false,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: true,
      flatShading: false,
      wireframe: false,
    },
  },

  {
    id: "hologram-rainbow",
    name: "Rainbow Hologram",
    category: "Special",
    description: "Color-shifting holographic material",
    properties: {
      type: "physical" as const,
      color: "#ffffff",
      opacity: 0.8,
      transparent: true,
      transmission: 0.8,
      vertexColors: false,
      fog: true,
      side: 2,
      receiveShadow: true,
      castShadow: false,
      flatShading: false,
      wireframe: false,
    },
  },
];

/**
 * Get all material preset categories
 */
export function getMaterialPresetCategories(): string[] {
  const categories = new Set<string>();
  for (const preset of MATERIAL_PRESETS) {
    categories.add(preset.category);
  }
  return Array.from(categories).sort();
}

/**
 * Get material presets by category
 */
export function getMaterialPresetsByCategory(
  category: string
): MaterialPreset[] {
  return MATERIAL_PRESETS.filter((preset) => preset.category === category);
}

/**
 * Get material preset by ID
 */
export function getMaterialPresetById(id: string): MaterialPreset | undefined {
  return MATERIAL_PRESETS.find((preset) => preset.id === id);
}

/**
 * Search material presets by name or description
 */
export function searchMaterialPresets(query: string): MaterialPreset[] {
  const lowercaseQuery = query.toLowerCase();
  return MATERIAL_PRESETS.filter(
    (preset) =>
      preset.name.toLowerCase().includes(lowercaseQuery) ||
      preset.description?.toLowerCase().includes(lowercaseQuery) ||
      preset.category.toLowerCase().includes(lowercaseQuery)
  );
}
