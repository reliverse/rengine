import * as THREE from "three";

type CustomMaterialParameters<T extends THREE.MaterialParameters> = T & {
  // Custom properties that can be added to any material
  subsurfaceColor?: THREE.Color | number | string;
  subsurfaceRadius?: number;
  subsurfaceStrength?: number;
  anisotropyMap?: THREE.Texture | null;
  anisotropyStrength?: number;
  iridescenceMap?: THREE.Texture | null;
  iridescenceThicknessMap?: THREE.Texture | null;
  velvetStrength?: number;
  velvetColor?: THREE.Color | number | string;
  hologramStrength?: number;
  hologramSpeed?: number;
  hologramDensity?: number;
  dispersion?: number;
  gemType?: string;
  weaveDensity?: number;
  weaveStrength?: number;
  fabricType?: string;
  skinTone?: THREE.Color | number | string;
  oiliness?: number;
  shadingLevels?: number;
  shadowStrength?: number;
  highlightStrength?: number;
  rimLightStrength?: number;
  tintColor?: THREE.Color | number | string;
  distortionStrength?: number;
  chromaticAberration?: number;
  flakeColor?: THREE.Color | number | string;
  flakeSize?: number;
  flakeDensity?: number;
  baseColor?: THREE.Color | number | string;
};

/**
 * Custom material with subsurface scattering effect
 */
export class SubsurfaceScatteringMaterial extends THREE.MeshPhysicalMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshPhysicalMaterialParameters> = {}
  ) {
    super(parameters);

    // Enable transmission for subsurface scattering
    this.transmission = parameters.transmission || 0.1;
    this.thickness = parameters.thickness || 0.5;
    this.ior = parameters.ior || 1.5;
    this.attenuationColor = parameters.attenuationColor
      ? new THREE.Color(parameters.attenuationColor)
      : new THREE.Color(0xff_ff_ff);
    this.attenuationDistance = parameters.attenuationDistance || 0.5;

    // Custom uniforms for enhanced subsurface scattering
    this.userData = {
      ...this.userData,
      subsurfaceColor:
        parameters.subsurfaceColor || new THREE.Color(0xff_cc_cc),
      subsurfaceRadius: parameters.subsurfaceRadius || 1.0,
      subsurfaceStrength: parameters.subsurfaceStrength || 0.5,
    };
  }
}

/**
 * Anisotropic material with directional highlights
 */
export class AnisotropicMaterial extends THREE.MeshPhysicalMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshPhysicalMaterialParameters> = {}
  ) {
    super(parameters);

    // Enable anisotropy
    this.anisotropy = parameters.anisotropy || 0.5;
    this.anisotropyRotation = parameters.anisotropyRotation || 0;

    // Custom uniforms for enhanced anisotropy
    this.userData = {
      ...this.userData,
      anisotropyMap: parameters.anisotropyMap || null,
      anisotropyStrength: parameters.anisotropyStrength || 1.0,
    };
  }
}

/**
 * Iridescent material with color-shifting effect
 */
export class IridescentMaterial extends THREE.MeshPhysicalMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshPhysicalMaterialParameters> = {}
  ) {
    super(parameters);

    // Enable iridescence
    this.iridescence = parameters.iridescence || 1.0;
    this.iridescenceIOR = parameters.iridescenceIOR || 1.3;
    this.iridescenceThicknessRange = parameters.iridescenceThicknessRange || [
      100, 400,
    ];

    // Custom uniforms for enhanced iridescence
    this.userData = {
      ...this.userData,
      iridescenceMap: parameters.iridescenceMap || null,
      iridescenceThicknessMap: parameters.iridescenceThicknessMap || null,
    };
  }
}

/**
 * Velvet material with soft, diffuse appearance
 */
export class VelvetMaterial extends THREE.MeshPhysicalMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshPhysicalMaterialParameters> = {}
  ) {
    super(parameters);

    // Velvet has very high roughness and low metalness
    this.roughness = Math.max(parameters.roughness || 0.9, 0.8);
    this.metalness = Math.min(parameters.metalness || 0.0, 0.1);

    // Custom velvet properties
    this.userData = {
      ...this.userData,
      velvetStrength: parameters.velvetStrength || 0.8,
      velvetColor: parameters.velvetColor || new THREE.Color(0x00_00_00),
    };
  }
}

/**
 * Holographic material with rainbow effects
 */
export class HolographicMaterial extends THREE.MeshPhysicalMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshPhysicalMaterialParameters> = {}
  ) {
    super(parameters);

    // Holographic materials are transparent with rainbow effects
    this.transparent = true;
    this.transmission = parameters.transmission || 0.8;
    this.roughness = parameters.roughness || 0.0;
    this.metalness = parameters.metalness || 0.0;

    // Custom holographic properties
    this.userData = {
      ...this.userData,
      hologramStrength: parameters.hologramStrength || 1.0,
      hologramSpeed: parameters.hologramSpeed || 1.0,
      hologramDensity: parameters.hologramDensity || 10.0,
    };
  }
}

/**
 * Gem material with high refractive index and dispersion
 */
export class GemMaterial extends THREE.MeshPhysicalMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshPhysicalMaterialParameters> = {}
  ) {
    super(parameters);

    // Gems have high IOR and low roughness
    this.ior = parameters.ior || 2.4; // Diamond-like
    this.roughness = parameters.roughness || 0.0;
    this.metalness = 0.0; // Gems are dielectric
    this.transmission = 0.0; // Opaque gems, use transmission for transparent gems

    // Custom gem properties
    this.userData = {
      ...this.userData,
      dispersion: parameters.dispersion || 0.1,
      gemType: parameters.gemType || "diamond",
    };
  }
}

/**
 * Fabric material with weave pattern
 */
export class FabricMaterial extends THREE.MeshLambertMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshLambertMaterialParameters> = {}
  ) {
    super(parameters);

    // Fabric properties
    this.color = parameters.color
      ? new THREE.Color(parameters.color)
      : new THREE.Color(0xf5_f5_dc);

    // Custom fabric properties
    this.userData = {
      ...this.userData,
      weaveDensity: parameters.weaveDensity || 50.0,
      weaveStrength: parameters.weaveStrength || 0.3,
      fabricType: parameters.fabricType || "cotton",
    };
  }
}

/**
 * Skin material with subsurface scattering
 */
export class SkinMaterial extends THREE.MeshPhysicalMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshPhysicalMaterialParameters> = {}
  ) {
    super(parameters);

    // Skin properties
    this.roughness = parameters.roughness || 0.4;
    this.metalness = 0.0;
    this.transmission = parameters.transmission || 0.1;
    this.thickness = parameters.thickness || 0.8;

    // Custom skin properties
    this.userData = {
      ...this.userData,
      skinTone: parameters.skinTone || new THREE.Color(0xff_db_ac),
      subsurfaceColor:
        parameters.subsurfaceColor || new THREE.Color(0xff_6b_6b),
      oiliness: parameters.oiliness || 0.2,
    };
  }
}

/**
 * Custom toon material with adjustable shading levels
 */
export class AdvancedToonMaterial extends THREE.MeshToonMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshToonMaterialParameters> = {}
  ) {
    super(parameters);

    // Custom toon properties
    this.userData = {
      ...this.userData,
      shadingLevels: parameters.shadingLevels || 3,
      shadowStrength: parameters.shadowStrength || 0.8,
      highlightStrength: parameters.highlightStrength || 1.2,
      rimLightStrength: parameters.rimLightStrength || 0.5,
    };
  }
}

/**
 * Glass material with realistic refraction
 */
export class RealisticGlassMaterial extends THREE.MeshPhysicalMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshPhysicalMaterialParameters> = {}
  ) {
    super(parameters);

    // Glass properties
    this.transparent = true;
    this.transmission = 1.0;
    this.opacity = parameters.opacity || 0.9;
    this.roughness = parameters.roughness || 0.0;
    this.metalness = 0.0;
    this.ior = parameters.ior || 1.5;
    this.thickness = parameters.thickness || 0.1;

    // Custom glass properties
    this.userData = {
      ...this.userData,
      tintColor: parameters.tintColor || new THREE.Color(0xff_ff_ff),
      distortionStrength: parameters.distortionStrength || 0.1,
      chromaticAberration: parameters.chromaticAberration || 0.01,
    };
  }
}

/**
 * Metallic paint material with flake effects
 */
export class MetallicPaintMaterial extends THREE.MeshPhysicalMaterial {
  constructor(
    parameters: CustomMaterialParameters<THREE.MeshPhysicalMaterialParameters> = {}
  ) {
    super(parameters);

    // Metallic paint properties
    this.metalness = parameters.metalness || 0.8;
    this.roughness = parameters.roughness || 0.2;
    this.clearcoat = parameters.clearcoat || 1.0;
    this.clearcoatRoughness = parameters.clearcoatRoughness || 0.1;

    // Custom metallic paint properties
    this.userData = {
      ...this.userData,
      flakeColor: parameters.flakeColor || new THREE.Color(0xff_ff_ff),
      flakeSize: parameters.flakeSize || 0.1,
      flakeDensity: parameters.flakeDensity || 0.5,
      baseColor: parameters.baseColor || new THREE.Color(0x33_33_33),
    };
  }
}

/**
 * Create custom material based on type and parameters
 */
export function createCustomMaterial(
  type: string,
  parameters: CustomMaterialParameters<THREE.MaterialParameters> = {}
): THREE.Material {
  switch (type.toLowerCase()) {
    case "subsurface":
      return new SubsurfaceScatteringMaterial(parameters);
    case "anisotropic":
      return new AnisotropicMaterial(parameters);
    case "iridescent":
      return new IridescentMaterial(parameters);
    case "velvet":
      return new VelvetMaterial(parameters);
    case "holographic":
      return new HolographicMaterial(parameters);
    case "gem":
      return new GemMaterial(parameters);
    case "fabric":
      return new FabricMaterial(parameters);
    case "skin":
      return new SkinMaterial(parameters);
    case "advanced-toon":
      return new AdvancedToonMaterial(parameters);
    case "realistic-glass":
      return new RealisticGlassMaterial(parameters);
    case "metallic-paint":
      return new MetallicPaintMaterial(parameters);
    default:
      return new THREE.MeshPhysicalMaterial(parameters);
  }
}

/**
 * Get available custom material types
 */
export function getCustomMaterialTypes(): Array<{
  type: string;
  name: string;
  description: string;
  category: string;
}> {
  return [
    {
      type: "subsurface",
      name: "Subsurface Scattering",
      description: "Material with realistic subsurface light scattering",
      category: "Advanced",
    },
    {
      type: "anisotropic",
      name: "Anisotropic",
      description: "Material with directional surface highlights",
      category: "Advanced",
    },
    {
      type: "iridescent",
      name: "Iridescent",
      description: "Material with color-shifting rainbow effects",
      category: "Special",
    },
    {
      type: "velvet",
      name: "Velvet",
      description: "Soft, diffuse fabric-like material",
      category: "Fabric",
    },
    {
      type: "holographic",
      name: "Holographic",
      description: "Rainbow holographic effect material",
      category: "Special",
    },
    {
      type: "gem",
      name: "Gem",
      description: "Precious gem material with high refraction",
      category: "Special",
    },
    {
      type: "fabric",
      name: "Fabric",
      description: "Textured fabric material",
      category: "Fabric",
    },
    {
      type: "skin",
      name: "Skin",
      description: "Human skin material with subsurface scattering",
      category: "Organic",
    },
    {
      type: "advanced-toon",
      name: "Advanced Toon",
      description: "Enhanced cartoon shading with adjustable levels",
      category: "Stylized",
    },
    {
      type: "realistic-glass",
      name: "Realistic Glass",
      description: "Physically accurate glass material",
      category: "Glass",
    },
    {
      type: "metallic-paint",
      name: "Metallic Paint",
      description: "Automotive metallic paint with flake effects",
      category: "Paint",
    },
  ];
}

/**
 * Update custom material properties at runtime
 */
export function updateCustomMaterial(
  material: THREE.Material,
  property: string,
  value: unknown
): void {
  if (material.userData && material.userData[property] !== undefined) {
    material.userData[property] = value;
  } else if (material[property as keyof THREE.Material] !== undefined) {
    (material as unknown as Record<string, unknown>)[property] = value;
  }

  material.needsUpdate = true;
}
