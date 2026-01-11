import * as THREE from "three";
import type {
  BasicMaterialProperties,
  DepthMaterialProperties,
  LambertMaterialProperties,
  MaterialProperties,
  NormalMaterialProperties,
  PhongMaterialProperties,
  PhysicalMaterialProperties,
  StandardMaterialProperties,
  ToonMaterialProperties,
} from "~/types/materials";
import {
  applyTextureChannelProperties,
  getCachedTexture,
} from "~/utils/texture-manager";

// Material cache to avoid recreating materials unnecessarily
const materialCache = new Map<string, THREE.Material>();

/**
 * Create a Three.js BasicMaterial from properties
 */
export function createBasicMaterial(
  properties: BasicMaterialProperties
): THREE.MeshBasicMaterial {
  const material = new THREE.MeshBasicMaterial({
    color: new THREE.Color(properties.color),
    opacity: properties.opacity,
    transparent: properties.transparent,
    alphaTest: properties.alphaTest,
    alphaHash: Boolean(properties.alphaHash),
    vertexColors: Boolean(properties.vertexColors),
    fog: properties.fog,
    side: properties.side,
    shadowSide: properties.shadowSide,
    alphaToCoverage: Boolean(properties.alphaToCoverage),
    colorWrite: Boolean(properties.colorWrite),
    depthTest: Boolean(properties.depthTest),
    depthWrite: Boolean(properties.depthWrite),
    stencilWrite: properties.stencilWrite,
    stencilFunc: properties.stencilFunc,
    stencilRef: properties.stencilRef,
    stencilFail: properties.stencilFail,
    stencilZFail: properties.stencilZFail,
    stencilZPass: properties.stencilZPass,
    wireframe: properties.wireframe,
    wireframeLinewidth: properties.wireframeLinewidth,
    wireframeLinecap:
      properties.wireframeLinecap === "round" ||
      properties.wireframeLinecap === "bevel" ||
      properties.wireframeLinecap === "miter"
        ? properties.wireframeLinecap
        : "round",
    wireframeLinejoin:
      (properties.wireframeLinejoin as "round" | "bevel" | "miter") || "round",
  });

  // Apply textures
  const mapTexture = getCachedTexture(properties.map.texture);
  if (mapTexture) applyTextureChannelProperties(mapTexture, properties.map);
  material.map = mapTexture;

  const alphaMapTexture = getCachedTexture(properties.alphaMap.texture);
  if (alphaMapTexture)
    applyTextureChannelProperties(alphaMapTexture, properties.alphaMap);
  material.alphaMap = alphaMapTexture;

  const aoMapTexture = getCachedTexture(properties.aoMap.texture);
  if (aoMapTexture)
    applyTextureChannelProperties(aoMapTexture, properties.aoMap);
  material.aoMap = aoMapTexture;

  const envMapTexture = getCachedTexture(properties.envMap.texture);
  if (envMapTexture)
    applyTextureChannelProperties(envMapTexture, properties.envMap);
  material.envMap = envMapTexture;

  const lightMapTexture = getCachedTexture(properties.lightMap.texture);
  if (lightMapTexture)
    applyTextureChannelProperties(lightMapTexture, properties.lightMap);
  material.lightMap = lightMapTexture;

  const specularMapTexture = getCachedTexture(properties.specularMap.texture);
  if (specularMapTexture)
    applyTextureChannelProperties(specularMapTexture, properties.specularMap);
  material.specularMap = specularMapTexture;

  material.needsUpdate = true;
  return material;
}

/**
 * Create a Three.js StandardMaterial from properties
 */
export function createStandardMaterial(
  properties: StandardMaterialProperties
): THREE.MeshPhysicalMaterial {
  const material = new THREE.MeshPhysicalMaterial({
    color: new THREE.Color(properties.color),
    opacity: properties.opacity,
    transparent: properties.transparent,
    alphaTest: properties.alphaTest,
    alphaHash: Boolean(properties.alphaHash),
    vertexColors: Boolean(properties.vertexColors),
    fog: properties.fog,
    side: properties.side,
    shadowSide: properties.shadowSide,
    flatShading: properties.flatShading,
    wireframe: properties.wireframe,
    wireframeLinewidth: properties.wireframeLinewidth,
    wireframeLinecap:
      properties.wireframeLinecap === "round" ||
      properties.wireframeLinecap === "bevel" ||
      properties.wireframeLinecap === "miter"
        ? properties.wireframeLinecap
        : "round",
    wireframeLinejoin:
      (properties.wireframeLinejoin as "round" | "bevel" | "miter") || "round",
    metalness: properties.metalness,
    roughness: properties.roughness,
    bumpScale: properties.bumpScale,
    displacementScale: properties.displacementScale,
    displacementBias: properties.displacementBias,
    envMapIntensity: properties.envMapIntensity,
    lightMapIntensity: properties.lightMapIntensity,
    aoMapIntensity: properties.aoMapIntensity,
    emissiveIntensity: properties.emissiveIntensity,
    emissive: new THREE.Color(properties.emissive),
    normalMapType: properties.normalMapType,
    normalScale: new THREE.Vector2(
      properties.normalScale[0],
      properties.normalScale[1]
    ),
    clearcoat: properties.clearcoat,
    clearcoatRoughness: properties.clearcoatRoughness,
    ior: properties.ior,
    iridescence: properties.iridescence,
    iridescenceIOR: properties.iridescenceIOR,
    iridescenceThicknessRange: properties.iridescenceThicknessRange,
    sheenColor: new THREE.Color(properties.sheenColor),
    sheenRoughness: properties.sheenRoughness,
    transmission: properties.transmission,
    thickness: properties.thickness,
    attenuationDistance: properties.attenuationDistance,
    attenuationColor: new THREE.Color(properties.attenuationColor),
  });

  // Apply textures
  const mapTexture = getCachedTexture(properties.map.texture);
  if (mapTexture) applyTextureChannelProperties(mapTexture, properties.map);
  material.map = mapTexture;

  const alphaMapTexture = getCachedTexture(properties.alphaMap.texture);
  if (alphaMapTexture)
    applyTextureChannelProperties(alphaMapTexture, properties.alphaMap);
  material.alphaMap = alphaMapTexture;

  const aoMapTexture = getCachedTexture(properties.aoMap.texture);
  if (aoMapTexture)
    applyTextureChannelProperties(aoMapTexture, properties.aoMap);
  material.aoMap = aoMapTexture;

  const bumpMapTexture = getCachedTexture(properties.bumpMap.texture);
  if (bumpMapTexture)
    applyTextureChannelProperties(bumpMapTexture, properties.bumpMap);
  material.bumpMap = bumpMapTexture;

  const displacementMapTexture = getCachedTexture(
    properties.displacementMap.texture
  );
  if (displacementMapTexture)
    applyTextureChannelProperties(
      displacementMapTexture,
      properties.displacementMap
    );
  material.displacementMap = displacementMapTexture;

  const emissiveMapTexture = getCachedTexture(properties.emissiveMap.texture);
  if (emissiveMapTexture)
    applyTextureChannelProperties(emissiveMapTexture, properties.emissiveMap);
  material.emissiveMap = emissiveMapTexture;

  const envMapTexture = getCachedTexture(properties.envMap.texture);
  if (envMapTexture)
    applyTextureChannelProperties(envMapTexture, properties.envMap);
  material.envMap = envMapTexture;

  const lightMapTexture = getCachedTexture(properties.lightMap.texture);
  if (lightMapTexture)
    applyTextureChannelProperties(lightMapTexture, properties.lightMap);
  material.lightMap = lightMapTexture;

  const metalnessMapTexture = getCachedTexture(properties.metalnessMap.texture);
  if (metalnessMapTexture)
    applyTextureChannelProperties(metalnessMapTexture, properties.metalnessMap);
  material.metalnessMap = metalnessMapTexture;

  const normalMapTexture = getCachedTexture(properties.normalMap.texture);
  if (normalMapTexture)
    applyTextureChannelProperties(normalMapTexture, properties.normalMap);
  material.normalMap = normalMapTexture;

  const roughnessMapTexture = getCachedTexture(properties.roughnessMap.texture);
  if (roughnessMapTexture)
    applyTextureChannelProperties(roughnessMapTexture, properties.roughnessMap);
  material.roughnessMap = roughnessMapTexture;

  const clearcoatMapTexture = getCachedTexture(properties.clearcoatMap.texture);
  if (clearcoatMapTexture)
    applyTextureChannelProperties(clearcoatMapTexture, properties.clearcoatMap);
  material.clearcoatMap = clearcoatMapTexture;

  const clearcoatRoughnessMapTexture = getCachedTexture(
    properties.clearcoatRoughnessMap.texture
  );
  if (clearcoatRoughnessMapTexture)
    applyTextureChannelProperties(
      clearcoatRoughnessMapTexture,
      properties.clearcoatRoughnessMap
    );
  material.clearcoatRoughnessMap = clearcoatRoughnessMapTexture;

  const clearcoatNormalMapTexture = getCachedTexture(
    properties.clearcoatNormalMap.texture
  );
  if (clearcoatNormalMapTexture)
    applyTextureChannelProperties(
      clearcoatNormalMapTexture,
      properties.clearcoatNormalMap
    );
  material.clearcoatNormalMap = clearcoatNormalMapTexture;

  const iridescenceMapTexture = getCachedTexture(
    properties.iridescenceMap.texture
  );
  if (iridescenceMapTexture)
    applyTextureChannelProperties(
      iridescenceMapTexture,
      properties.iridescenceMap
    );
  material.iridescenceMap = iridescenceMapTexture;

  const iridescenceThicknessMapTexture = getCachedTexture(
    properties.iridescenceThicknessMap.texture
  );
  if (iridescenceThicknessMapTexture)
    applyTextureChannelProperties(
      iridescenceThicknessMapTexture,
      properties.iridescenceThicknessMap
    );
  material.iridescenceThicknessMap = iridescenceThicknessMapTexture;

  const sheenColorMapTexture = getCachedTexture(
    properties.sheenColorMap.texture
  );
  if (sheenColorMapTexture)
    applyTextureChannelProperties(
      sheenColorMapTexture,
      properties.sheenColorMap
    );
  material.sheenColorMap = sheenColorMapTexture;

  const sheenRoughnessMapTexture = getCachedTexture(
    properties.sheenRoughnessMap.texture
  );
  if (sheenRoughnessMapTexture)
    applyTextureChannelProperties(
      sheenRoughnessMapTexture,
      properties.sheenRoughnessMap
    );
  material.sheenRoughnessMap = sheenRoughnessMapTexture;

  const specularIntensityMapTexture = getCachedTexture(
    properties.specularIntensityMap.texture
  );
  if (specularIntensityMapTexture)
    applyTextureChannelProperties(
      specularIntensityMapTexture,
      properties.specularIntensityMap
    );
  material.specularIntensityMap = specularIntensityMapTexture;

  const specularColorMapTexture = getCachedTexture(
    properties.specularColorMap.texture
  );
  if (specularColorMapTexture)
    applyTextureChannelProperties(
      specularColorMapTexture,
      properties.specularColorMap
    );
  material.specularColorMap = specularColorMapTexture;

  const transmissionMapTexture = getCachedTexture(
    properties.transmissionMap.texture
  );
  if (transmissionMapTexture)
    applyTextureChannelProperties(
      transmissionMapTexture,
      properties.transmissionMap
    );
  material.transmissionMap = transmissionMapTexture;

  const thicknessMapTexture = getCachedTexture(properties.thicknessMap.texture);
  if (thicknessMapTexture)
    applyTextureChannelProperties(thicknessMapTexture, properties.thicknessMap);
  material.thicknessMap = thicknessMapTexture;

  const anisotropyMapTexture = getCachedTexture(
    properties.anisotropyMap.texture
  );
  if (anisotropyMapTexture)
    applyTextureChannelProperties(
      anisotropyMapTexture,
      properties.anisotropyMap
    );
  material.anisotropyMap = anisotropyMapTexture;

  material.needsUpdate = true;
  return material;
}

/**
 * Create a Three.js PhysicalMaterial from properties
 */
export function createPhysicalMaterial(
  properties: PhysicalMaterialProperties
): THREE.MeshPhysicalMaterial {
  // Physical material extends standard material
  return createStandardMaterial(
    properties as unknown as StandardMaterialProperties
  ) as THREE.MeshPhysicalMaterial;
}

/**
 * Create a Three.js LambertMaterial from properties
 */
export function createLambertMaterial(
  properties: LambertMaterialProperties
): THREE.MeshLambertMaterial {
  const material = new THREE.MeshLambertMaterial({
    color: new THREE.Color(properties.color),
    opacity: properties.opacity,
    transparent: properties.transparent,
    alphaTest: properties.alphaTest,
    alphaHash: Boolean(properties.alphaHash),
    vertexColors: Boolean(properties.vertexColors),
    fog: properties.fog,
    side: properties.side,
    shadowSide: properties.shadowSide,
    alphaToCoverage: Boolean(properties.alphaToCoverage),
    colorWrite: Boolean(properties.colorWrite),
    depthTest: Boolean(properties.depthTest),
    depthWrite: Boolean(properties.depthWrite),
    flatShading: properties.flatShading,
    wireframe: properties.wireframe,
    wireframeLinewidth: properties.wireframeLinewidth,
    wireframeLinecap:
      properties.wireframeLinecap === "round" ||
      properties.wireframeLinecap === "bevel" ||
      properties.wireframeLinecap === "miter"
        ? properties.wireframeLinecap
        : "round",
    wireframeLinejoin:
      (properties.wireframeLinejoin as "round" | "bevel" | "miter") || "round",
    emissive: new THREE.Color(properties.emissive),
    lightMapIntensity: properties.lightMapIntensity,
    aoMapIntensity: properties.aoMapIntensity,
    emissiveIntensity: properties.emissiveIntensity,
  });

  // Apply textures
  const lambertMapTexture = getCachedTexture(properties.map.texture);
  if (lambertMapTexture)
    applyTextureChannelProperties(lambertMapTexture, properties.map);
  material.map = lambertMapTexture;

  const lambertAlphaMapTexture = getCachedTexture(properties.alphaMap.texture);
  if (lambertAlphaMapTexture)
    applyTextureChannelProperties(lambertAlphaMapTexture, properties.alphaMap);
  material.alphaMap = lambertAlphaMapTexture;

  const lambertAoMapTexture = getCachedTexture(properties.aoMap.texture);
  if (lambertAoMapTexture)
    applyTextureChannelProperties(lambertAoMapTexture, properties.aoMap);
  material.aoMap = lambertAoMapTexture;

  const lambertEmissiveMapTexture = getCachedTexture(
    properties.emissiveMap.texture
  );
  if (lambertEmissiveMapTexture)
    applyTextureChannelProperties(
      lambertEmissiveMapTexture,
      properties.emissiveMap
    );
  material.emissiveMap = lambertEmissiveMapTexture;

  const lambertEnvMapTexture = getCachedTexture(properties.envMap.texture);
  if (lambertEnvMapTexture)
    applyTextureChannelProperties(lambertEnvMapTexture, properties.envMap);
  material.envMap = lambertEnvMapTexture;

  const lambertLightMapTexture = getCachedTexture(properties.lightMap.texture);
  if (lambertLightMapTexture)
    applyTextureChannelProperties(lambertLightMapTexture, properties.lightMap);
  material.lightMap = lambertLightMapTexture;

  const lambertSpecularMapTexture = getCachedTexture(
    properties.specularMap.texture
  );
  if (lambertSpecularMapTexture)
    applyTextureChannelProperties(
      lambertSpecularMapTexture,
      properties.specularMap
    );
  material.specularMap = lambertSpecularMapTexture;

  material.needsUpdate = true;
  return material;
}

/**
 * Create a Three.js PhongMaterial from properties
 */
export function createPhongMaterial(
  properties: PhongMaterialProperties
): THREE.MeshPhongMaterial {
  const material = new THREE.MeshPhongMaterial({
    color: new THREE.Color(properties.color),
    opacity: properties.opacity,
    transparent: properties.transparent,
    alphaTest: properties.alphaTest,
    alphaHash: Boolean(properties.alphaHash),
    vertexColors: Boolean(properties.vertexColors),
    fog: properties.fog,
    side: properties.side,
    shadowSide: properties.shadowSide,
    alphaToCoverage: Boolean(properties.alphaToCoverage),
    colorWrite: Boolean(properties.colorWrite),
    depthTest: Boolean(properties.depthTest),
    depthWrite: Boolean(properties.depthWrite),
    flatShading: properties.flatShading,
    wireframe: properties.wireframe,
    wireframeLinewidth: properties.wireframeLinewidth,
    wireframeLinecap:
      properties.wireframeLinecap === "round" ||
      properties.wireframeLinecap === "bevel" ||
      properties.wireframeLinecap === "miter"
        ? properties.wireframeLinecap
        : "round",
    wireframeLinejoin:
      (properties.wireframeLinejoin as "round" | "bevel" | "miter") || "round",
    specular: new THREE.Color(properties.specular),
    shininess: properties.shininess,
    emissive: new THREE.Color(properties.emissive),
    lightMapIntensity: properties.lightMapIntensity,
    aoMapIntensity: properties.aoMapIntensity,
    emissiveIntensity: properties.emissiveIntensity,
    bumpScale: properties.bumpScale,
    displacementScale: properties.displacementScale,
    displacementBias: properties.displacementBias,
  });

  // Apply textures
  const phongMapTexture = getCachedTexture(properties.map.texture);
  if (phongMapTexture)
    applyTextureChannelProperties(phongMapTexture, properties.map);
  material.map = phongMapTexture;

  const phongAlphaMapTexture = getCachedTexture(properties.alphaMap.texture);
  if (phongAlphaMapTexture)
    applyTextureChannelProperties(phongAlphaMapTexture, properties.alphaMap);
  material.alphaMap = phongAlphaMapTexture;

  const phongAoMapTexture = getCachedTexture(properties.aoMap.texture);
  if (phongAoMapTexture)
    applyTextureChannelProperties(phongAoMapTexture, properties.aoMap);
  material.aoMap = phongAoMapTexture;

  const phongBumpMapTexture = getCachedTexture(properties.bumpMap.texture);
  if (phongBumpMapTexture)
    applyTextureChannelProperties(phongBumpMapTexture, properties.bumpMap);
  material.bumpMap = phongBumpMapTexture;

  const phongDisplacementMapTexture = getCachedTexture(
    properties.displacementMap.texture
  );
  if (phongDisplacementMapTexture)
    applyTextureChannelProperties(
      phongDisplacementMapTexture,
      properties.displacementMap
    );
  material.displacementMap = phongDisplacementMapTexture;

  const phongEmissiveMapTexture = getCachedTexture(
    properties.emissiveMap.texture
  );
  if (phongEmissiveMapTexture)
    applyTextureChannelProperties(
      phongEmissiveMapTexture,
      properties.emissiveMap
    );
  material.emissiveMap = phongEmissiveMapTexture;

  const phongEnvMapTexture = getCachedTexture(properties.envMap.texture);
  if (phongEnvMapTexture)
    applyTextureChannelProperties(phongEnvMapTexture, properties.envMap);
  material.envMap = phongEnvMapTexture;

  const phongLightMapTexture = getCachedTexture(properties.lightMap.texture);
  if (phongLightMapTexture)
    applyTextureChannelProperties(phongLightMapTexture, properties.lightMap);
  material.lightMap = phongLightMapTexture;

  const phongNormalMapTexture = getCachedTexture(properties.normalMap.texture);
  if (phongNormalMapTexture)
    applyTextureChannelProperties(phongNormalMapTexture, properties.normalMap);
  material.normalMap = phongNormalMapTexture;

  const phongSpecularMapTexture = getCachedTexture(
    properties.specularMap.texture
  );
  if (phongSpecularMapTexture)
    applyTextureChannelProperties(
      phongSpecularMapTexture,
      properties.specularMap
    );
  material.specularMap = phongSpecularMapTexture;

  material.needsUpdate = true;
  return material;
}

/**
 * Create a Three.js ToonMaterial from properties
 */
export function createToonMaterial(
  properties: ToonMaterialProperties
): THREE.MeshToonMaterial {
  const material = new THREE.MeshToonMaterial({
    color: new THREE.Color(properties.color),
    opacity: properties.opacity,
    transparent: properties.transparent,
    alphaTest: properties.alphaTest,
    alphaHash: Boolean(properties.alphaHash),
    vertexColors: Boolean(properties.vertexColors),
    fog: properties.fog,
    side: properties.side,
    shadowSide: properties.shadowSide,
    wireframe: properties.wireframe,
    wireframeLinewidth: properties.wireframeLinewidth,
    wireframeLinecap:
      properties.wireframeLinecap === "round" ||
      properties.wireframeLinecap === "bevel" ||
      properties.wireframeLinecap === "miter"
        ? properties.wireframeLinecap
        : "round",
    wireframeLinejoin:
      (properties.wireframeLinejoin as "round" | "bevel" | "miter") || "round",
  });

  // Apply textures
  const toonMapTexture = getCachedTexture(properties.map.texture);
  if (toonMapTexture)
    applyTextureChannelProperties(toonMapTexture, properties.map);
  material.map = toonMapTexture;

  const toonAlphaMapTexture = getCachedTexture(properties.alphaMap.texture);
  if (toonAlphaMapTexture)
    applyTextureChannelProperties(toonAlphaMapTexture, properties.alphaMap);
  material.alphaMap = toonAlphaMapTexture;

  const toonAoMapTexture = getCachedTexture(properties.aoMap.texture);
  if (toonAoMapTexture)
    applyTextureChannelProperties(toonAoMapTexture, properties.aoMap);
  material.aoMap = toonAoMapTexture;

  const toonLightMapTexture = getCachedTexture(properties.lightMap.texture);
  if (toonLightMapTexture)
    applyTextureChannelProperties(toonLightMapTexture, properties.lightMap);
  material.lightMap = toonLightMapTexture;

  material.gradientMap = getCachedTexture(properties.gradientMap);

  material.needsUpdate = true;
  return material;
}

/**
 * Create a Three.js NormalMaterial from properties
 */
export function createNormalMaterial(
  properties: NormalMaterialProperties
): THREE.MeshNormalMaterial {
  const material = new THREE.MeshNormalMaterial({
    opacity: properties.opacity,
    transparent: properties.transparent,
    alphaTest: properties.alphaTest,
    alphaHash: Boolean(properties.alphaHash),
    vertexColors: Boolean(properties.vertexColors),
    fog: properties.fog,
    side: properties.side,
    shadowSide: properties.shadowSide,
    flatShading: properties.flatShading,
    wireframe: properties.wireframe,
    wireframeLinewidth: properties.wireframeLinewidth,
    normalMapType: properties.normalMapType,
    normalScale: new THREE.Vector2(
      properties.normalScale[0],
      properties.normalScale[1]
    ),
  });

  // Apply textures
  const normalMapTexture = getCachedTexture(properties.normalMap.texture);
  if (normalMapTexture)
    applyTextureChannelProperties(normalMapTexture, properties.normalMap);
  material.normalMap = normalMapTexture;

  const displacementMapTexture = getCachedTexture(
    properties.displacementMap.texture
  );
  if (displacementMapTexture)
    applyTextureChannelProperties(
      displacementMapTexture,
      properties.displacementMap
    );
  material.displacementMap = displacementMapTexture;

  material.needsUpdate = true;
  return material;
}

/**
 * Create a Three.js DepthMaterial from properties
 */
export function createDepthMaterial(
  properties: DepthMaterialProperties
): THREE.MeshDepthMaterial {
  const material = new THREE.MeshDepthMaterial({
    opacity: properties.opacity,
    transparent: properties.transparent,
    alphaTest: properties.alphaTest,
    alphaHash: Boolean(properties.alphaHash),
    vertexColors: Boolean(properties.vertexColors),
    side: properties.side,
    shadowSide: properties.shadowSide,
    wireframe: properties.wireframe,
    wireframeLinewidth: properties.wireframeLinewidth,
    displacementScale: properties.displacementScale,
    displacementBias: properties.displacementBias,
  });

  // Apply textures
  const depthDisplacementMapTexture = getCachedTexture(
    properties.displacementMap.texture
  );
  if (depthDisplacementMapTexture)
    applyTextureChannelProperties(
      depthDisplacementMapTexture,
      properties.displacementMap
    );
  material.displacementMap = depthDisplacementMapTexture;

  const depthAlphaMapTexture = getCachedTexture(properties.alphaMap.texture);
  if (depthAlphaMapTexture)
    applyTextureChannelProperties(depthAlphaMapTexture, properties.alphaMap);
  material.alphaMap = depthAlphaMapTexture;

  material.needsUpdate = true;
  return material;
}

/**
 * Create a custom material for advanced material types
 */
export function createCustomMaterial(
  _type: string,
  options: { color?: THREE.Color; opacity?: number; transparent?: boolean }
): THREE.Material {
  // For now, create a basic material as fallback for custom types
  return new THREE.MeshBasicMaterial({
    color: options.color || new THREE.Color(0xff_ff_ff),
    opacity: options.opacity || 1,
    transparent: options.transparent,
  });
}

/**
 * Create a Three.js material from material properties
 */
export function createMaterialFromProperties(
  properties: MaterialProperties
): THREE.Material {
  const cacheKey = `${properties.id}_${JSON.stringify(properties)}`;

  // Check cache first
  if (materialCache.has(cacheKey)) {
    const cachedMaterial = materialCache.get(cacheKey);
    if (cachedMaterial) {
      return cachedMaterial;
    }
  }

  let material: THREE.Material;

  switch (properties.type) {
    case "basic":
      material = createBasicMaterial(properties as BasicMaterialProperties);
      break;
    case "standard":
      material = createStandardMaterial(
        properties as StandardMaterialProperties
      );
      break;
    case "physical":
      material = createPhysicalMaterial(
        properties as PhysicalMaterialProperties
      );
      break;
    case "lambert":
      material = createLambertMaterial(properties as LambertMaterialProperties);
      break;
    case "phong":
      material = createPhongMaterial(properties as PhongMaterialProperties);
      break;
    case "toon":
      material = createToonMaterial(properties as ToonMaterialProperties);
      break;
    case "normal":
      material = createNormalMaterial(properties as NormalMaterialProperties);
      break;
    case "depth":
      material = createDepthMaterial(properties as DepthMaterialProperties);
      break;

    // Custom materials
    default: {
      material = createCustomMaterial((properties as any).type, {
        color: (properties as any).color
          ? new THREE.Color((properties as any).color)
          : new THREE.Color(0xff_ff_ff),
        opacity: (properties as any).opacity,
        transparent: (properties as any).transparent,
      });
      break;
    }
  }

  // Set common properties
  material.name = properties.name;
  material.userData.materialId = properties.id;

  // Cache the material
  materialCache.set(cacheKey, material);

  return material;
}

/**
 * Update an existing Three.js material with new properties
 */
export function updateMaterialFromProperties(
  material: THREE.Material,
  properties: MaterialProperties
): THREE.Material {
  const cacheKey = `${properties.id}_${JSON.stringify(properties)}`;

  // Clear cache entry for this material
  for (const [key, cachedMaterial] of materialCache) {
    if (cachedMaterial === material) {
      materialCache.delete(key);
      break;
    }
  }

  // Dispose of the old material
  material.dispose();

  // Create new material
  const newMaterial = createMaterialFromProperties(properties);

  // Cache the new material
  materialCache.set(cacheKey, newMaterial);

  return newMaterial;
}

/**
 * Clear material cache
 */
export function clearMaterialCache(): void {
  for (const material of materialCache.values()) {
    material.dispose();
  }
  materialCache.clear();
}

/**
 * Get material cache statistics
 */
export function getMaterialCacheStats(): {
  count: number;
  memoryUsage: number;
} {
  return {
    count: materialCache.size,
    memoryUsage: 0, // Would need to calculate actual memory usage
  };
}

/**
 * Create a material preview texture for thumbnails
 */
export function createMaterialPreview(
  properties: MaterialProperties,
  size = 64
): Promise<THREE.Texture> {
  return new Promise((resolve) => {
    // Create a small canvas for material preview
    const canvas = document.createElement("canvas");
    canvas.width = size;
    canvas.height = size;
    const ctx = canvas.getContext("2d");
    if (!ctx) {
      // Fallback: create a simple texture
      const fallbackCanvas = document.createElement("canvas");
      fallbackCanvas.width = size;
      fallbackCanvas.height = size;
      const fallbackTexture = new THREE.CanvasTexture(fallbackCanvas);
      resolve(fallbackTexture);
      return;
    }

    // Fill with base color
    ctx.fillStyle = properties.color || "#ffffff";
    ctx.fillRect(0, 0, size, size);

    // Add some pattern based on material type
    switch (properties.type) {
      case "standard":
      case "physical": {
        // Add a metallic sheen effect
        const stdProps = properties as StandardMaterialProperties;
        if (stdProps.metalness > 0.5) {
          ctx.fillStyle = `rgba(255, 255, 255, ${stdProps.metalness * 0.3})`;
          ctx.fillRect(0, 0, size, size);
        }
        break;
      }
      case "phong": {
        // Add a glossy effect
        const phongProps = properties as PhongMaterialProperties;
        ctx.fillStyle = `rgba(255, 255, 255, ${phongProps.shininess / 100})`;
        ctx.fillRect(0, 0, size, size);
        break;
      }
    }

    // Create texture from canvas
    const texture = new THREE.CanvasTexture(canvas);
    resolve(texture);
  });
}
