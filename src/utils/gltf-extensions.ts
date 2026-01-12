import * as THREE from "three";

// GLTF Extension Support
export interface GLTFExtension {
  name: string;
  supported: boolean;
  description: string;
  required?: boolean;
}

// Supported GLTF extensions
export const SUPPORTED_EXTENSIONS: Record<string, GLTFExtension> = {
  // Core extensions
  KHR_materials_pbrSpecularGlossiness: {
    name: "KHR_materials_pbrSpecularGlossiness",
    supported: true,
    description: "PBR materials with specular/glossiness workflow",
  },
  KHR_materials_unlit: {
    name: "KHR_materials_unlit",
    supported: true,
    description: "Unlit materials",
  },
  KHR_materials_transmission: {
    name: "KHR_materials_transmission",
    supported: true,
    description: "Transmission/refraction properties",
  },
  KHR_materials_volume: {
    name: "KHR_materials_volume",
    supported: true,
    description: "Volume scattering properties",
  },
  KHR_materials_ior: {
    name: "KHR_materials_ior",
    supported: true,
    description: "Index of refraction",
  },
  KHR_materials_specular: {
    name: "KHR_materials_specular",
    supported: true,
    description: "Specular properties",
  },
  KHR_materials_clearcoat: {
    name: "KHR_materials_clearcoat",
    supported: true,
    description: "Clearcoat materials",
  },
  KHR_materials_sheen: {
    name: "KHR_materials_sheen",
    supported: true,
    description: "Sheen materials",
  },
  KHR_materials_anisotropy: {
    name: "KHR_materials_anisotropy",
    supported: true,
    description: "Anisotropic materials",
  },
  KHR_materials_emissive_strength: {
    name: "KHR_materials_emissive_strength",
    supported: true,
    description: "Emissive strength multiplier",
  },

  // Texture extensions
  KHR_texture_transform: {
    name: "KHR_texture_transform",
    supported: true,
    description: "Texture coordinate transformations",
  },
  KHR_texture_basisu: {
    name: "KHR_texture_basisu",
    supported: true,
    description: "Basis Universal texture compression",
  },

  // Geometry extensions
  KHR_draco_mesh_compression: {
    name: "KHR_draco_mesh_compression",
    supported: true,
    description: "Draco mesh compression",
  },
  KHR_mesh_quantization: {
    name: "KHR_mesh_quantization",
    supported: true,
    description: "Mesh quantization",
  },

  // Animation extensions
  KHR_animation_pointer: {
    name: "KHR_animation_pointer",
    supported: false,
    description: "Animation pointer targets",
  },

  // Scene extensions
  KHR_lights_punctual: {
    name: "KHR_lights_punctual",
    supported: true,
    description: "Punctual lights",
  },
  KHR_materials_variants: {
    name: "KHR_materials_variants",
    supported: false,
    description: "Material variants",
  },

  // Deprecated extensions
  KHR_techniques_webgl: {
    name: "KHR_techniques_webgl",
    supported: false,
    description: "WebGL techniques (deprecated)",
  },
  KHR_blend: {
    name: "KHR_blend",
    supported: false,
    description: "Blending (deprecated)",
  },
};

export interface GLTFValidationResult {
  valid: boolean;
  supportedExtensions: string[];
  unsupportedExtensions: string[];
  warnings: string[];
  errors: string[];
}

/**
 * Comprehensive GLTF/GLB validation
 */
export function validateGLTF(file: File, gltf?: any): GLTFValidationResult {
  const result: GLTFValidationResult = {
    valid: true,
    supportedExtensions: [],
    unsupportedExtensions: [],
    warnings: [],
    errors: [],
  };

  // File-level validation
  if (file.size > 100 * 1024 * 1024) {
    // 100MB limit
    result.errors.push(
      `File size (${(file.size / (1024 * 1024)).toFixed(1)}MB) exceeds maximum limit of 100MB`
    );
    result.valid = false;
  }

  if (file.size === 0) {
    result.errors.push("File is empty");
    result.valid = false;
  }

  // GLTF-specific validation
  if (gltf) {
    // Check for basic GLTF structure
    if (!(gltf.scene || gltf.scenes)) {
      result.errors.push("Invalid GLTF: No scenes found");
      result.valid = false;
    }

    if (gltf.scenes && gltf.scenes.length === 0) {
      result.warnings.push("GLTF contains no scenes");
    }

    // Validate meshes
    if (gltf.meshes) {
      for (let i = 0; i < gltf.meshes.length; i++) {
        const mesh = gltf.meshes[i];
        if (!mesh.primitives || mesh.primitives.length === 0) {
          result.warnings.push(`Mesh ${i} has no primitives`);
        }

        for (const primitive of mesh.primitives) {
          if (!primitive.attributes) {
            result.errors.push(`Mesh ${i} primitive missing attributes`);
            result.valid = false;
          } else if (!primitive.attributes.POSITION) {
            result.warnings.push(
              `Mesh ${i} primitive missing POSITION attribute`
            );
          }
        }
      }
    }

    // Validate materials
    if (gltf.materials) {
      for (const _material of gltf.materials) {
        // Material validation can be added here
      }
    }

    // Extension validation
    const extensionsResult = validateGLTFExtensions(gltf);
    result.supportedExtensions = extensionsResult.supportedExtensions;
    result.unsupportedExtensions = extensionsResult.unsupportedExtensions;
    result.warnings.push(...extensionsResult.warnings);
    result.errors.push(...extensionsResult.errors);

    if (extensionsResult.errors.length > 0) {
      result.valid = false;
    }
  }

  return result;
}

/**
 * Validate GLTF extensions and report compatibility
 */
export function validateGLTFExtensions(gltf: any): GLTFValidationResult {
  const result: GLTFValidationResult = {
    valid: true,
    supportedExtensions: [],
    unsupportedExtensions: [],
    warnings: [],
    errors: [],
  };

  if (!gltf?.parser) {
    return result;
  }

  const extensionsUsed = gltf.parser.extensionsUsed || [];
  const extensionsRequired = gltf.parser.extensionsRequired || [];

  for (const ext of extensionsUsed) {
    const extensionInfo = SUPPORTED_EXTENSIONS[ext];

    if (extensionInfo) {
      if (extensionInfo.supported) {
        result.supportedExtensions.push(ext);
      } else {
        result.unsupportedExtensions.push(ext);
        result.warnings.push(
          `Extension ${ext} is not fully supported: ${extensionInfo.description}`
        );
      }
    } else {
      result.unsupportedExtensions.push(ext);
      result.warnings.push(`Unknown extension: ${ext}`);
    }
  }

  // Check required extensions
  for (const ext of extensionsRequired) {
    if (!SUPPORTED_EXTENSIONS[ext]?.supported) {
      result.errors.push(`Required extension ${ext} is not supported`);
      result.valid = false;
    }
  }

  return result;
}

/**
 * Apply extension-specific material properties
 */
export function applyGLTFMaterialExtensions(
  material: THREE.Material,
  _gltfMaterial: any,
  extensions: any
): THREE.Material {
  if (
    !(
      material instanceof THREE.MeshStandardMaterial ||
      material instanceof THREE.MeshPhysicalMaterial
    )
  ) {
    return material;
  }

  // KHR_materials_pbrSpecularGlossiness
  if (extensions?.KHR_materials_pbrSpecularGlossiness) {
    const specGloss = extensions.KHR_materials_pbrSpecularGlossiness;
    // Convert specular/glossiness to metallic/roughness
    if (specGloss.diffuseFactor) {
      material.color.fromArray(specGloss.diffuseFactor);
    }
    if (specGloss.specularFactor) {
      // Approximate conversion - this is not perfect
      const specular = specGloss.specularFactor;
      const glossiness = specGloss.glossinessFactor || 1.0;
      material.metalness = Math.min(specular[0], specular[1], specular[2]);
      material.roughness = 1.0 - glossiness;
    }
  }

  // KHR_materials_transmission
  if (
    extensions?.KHR_materials_transmission &&
    material instanceof THREE.MeshPhysicalMaterial
  ) {
    const transmission = extensions.KHR_materials_transmission;
    material.transmission = transmission.transmissionFactor || 0;
    if (transmission.transmissionTexture) {
      // Handle transmission texture
      material.transmissionMap = transmission.transmissionTexture;
    }
  }

  // KHR_materials_volume
  if (
    extensions?.KHR_materials_volume &&
    material instanceof THREE.MeshPhysicalMaterial
  ) {
    const volume = extensions.KHR_materials_volume;
    material.thickness = volume.thicknessFactor || 0;
    material.attenuationDistance = volume.attenuationDistance || 0;
    if (volume.attenuationColor) {
      material.attenuationColor = new THREE.Color().fromArray(
        volume.attenuationColor
      );
    }
    if (volume.thicknessTexture) {
      material.thicknessMap = volume.thicknessTexture;
    }
  }

  // KHR_materials_clearcoat
  if (
    extensions?.KHR_materials_clearcoat &&
    material instanceof THREE.MeshPhysicalMaterial
  ) {
    const clearcoat = extensions.KHR_materials_clearcoat;
    material.clearcoat = clearcoat.clearcoatFactor || 0;
    material.clearcoatRoughness = clearcoat.clearcoatRoughnessFactor || 0;
    if (clearcoat.clearcoatTexture) {
      material.clearcoatMap = clearcoat.clearcoatTexture;
    }
    if (clearcoat.clearcoatRoughnessTexture) {
      material.clearcoatRoughnessMap = clearcoat.clearcoatRoughnessTexture;
    }
    if (clearcoat.clearcoatNormalTexture) {
      material.clearcoatNormalMap = clearcoat.clearcoatNormalTexture;
    }
  }

  // KHR_materials_sheen
  if (
    extensions?.KHR_materials_sheen &&
    material instanceof THREE.MeshPhysicalMaterial
  ) {
    const sheen = extensions.KHR_materials_sheen;
    material.sheen = sheen.sheenColorFactor || 0;
    material.sheenRoughness = sheen.sheenRoughnessFactor || 1;
    if (sheen.sheenColorTexture) {
      material.sheenColorMap = sheen.sheenColorTexture;
    }
    if (sheen.sheenRoughnessTexture) {
      material.sheenRoughnessMap = sheen.sheenRoughnessTexture;
    }
  }

  // KHR_materials_anisotropy
  if (
    extensions?.KHR_materials_anisotropy &&
    material instanceof THREE.MeshPhysicalMaterial
  ) {
    const anisotropy = extensions.KHR_materials_anisotropy;
    material.anisotropy = anisotropy.anisotropyStrength || 0;
    if (anisotropy.anisotropyRotation) {
      material.anisotropyRotation = anisotropy.anisotropyRotation;
    }
    if (anisotropy.anisotropyTexture) {
      material.anisotropyMap = anisotropy.anisotropyTexture;
    }
  }

  // KHR_materials_emissive_strength
  if (extensions?.KHR_materials_emissive_strength) {
    const emissiveStrength = extensions.KHR_materials_emissive_strength;
    const strength = emissiveStrength.emissiveStrength || 1;
    material.emissive.multiplyScalar(strength);
  }

  // KHR_materials_ior
  if (
    extensions?.KHR_materials_ior &&
    material instanceof THREE.MeshPhysicalMaterial
  ) {
    const ior = extensions.KHR_materials_ior;
    material.ior = ior.ior || 1.5;
  }

  // KHR_materials_specular
  if (
    extensions?.KHR_materials_specular &&
    material instanceof THREE.MeshPhysicalMaterial
  ) {
    const specular = extensions.KHR_materials_specular;
    material.specularIntensity = specular.specularFactor || 1;
    if (specular.specularTexture) {
      material.specularIntensityMap = specular.specularTexture;
    }
    if (specular.specularColorFactor) {
      material.specularColor = new THREE.Color().fromArray(
        specular.specularColorFactor
      );
    }
    if (specular.specularColorTexture) {
      material.specularColorMap = specular.specularColorTexture;
    }
  }

  return material;
}

/**
 * Extract lights from GLTF scene
 */
export function extractGLTFLights(gltf: any): THREE.Light[] {
  const lights: THREE.Light[] = [];

  if (!gltf?.scene) return lights;

  gltf.scene.traverse((child: any) => {
    if (child.isLight) {
      lights.push(child);
    }
  });

  return lights;
}

/**
 * Extract cameras from GLTF scene
 */
export function extractGLTFCameras(gltf: any): THREE.Camera[] {
  const cameras: THREE.Camera[] = [];

  if (!gltf?.cameras) return cameras;

  // GLTF cameras are stored in gltf.cameras array
  for (const camera of gltf.cameras) {
    cameras.push(camera);
  }

  return cameras;
}
