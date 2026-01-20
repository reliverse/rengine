import * as THREE from "three";

/**
 * Material types supported by the engine
 */
export type MaterialType =
  | "basic"
  | "standard"
  | "physical"
  | "lambert"
  | "phong"
  | "toon"
  | "normal"
  | "depth"
  | "subsurface"
  | "anisotropic"
  | "iridescent"
  | "velvet"
  | "holographic"
  | "gem"
  | "fabric"
  | "skin"
  | "advanced-toon"
  | "realistic-glass"
  | "metallic-paint";

/**
 * Texture channel configuration for PBR materials
 */
export interface TextureChannel {
  /** Texture UUID or null if no texture */
  texture: string | null;
  /** Texture offset [U, V] */
  offset: [number, number];
  /** Texture repeat [U, V] */
  repeat: [number, number];
  /** Texture rotation in radians */
  rotation: number;
  /** Texture wrapping mode */
  wrapS: THREE.Wrapping;
  /** Texture wrapping mode */
  wrapT: THREE.Wrapping;
  /** Texture filtering mode */
  magFilter: THREE.TextureFilter;
  /** Texture filtering mode */
  minFilter: THREE.TextureFilter;
  /** Texture anisotropy level */
  anisotropy: number;
  /** Texture encoding */
  encoding: THREE.ColorSpace;
  /** Flip Y coordinate */
  flipY: boolean;
}

/**
 * Base material properties shared across all material types
 */
export interface BaseMaterialProperties {
  /** Material unique identifier */
  id: string;
  /** Material name */
  name: string;
  /** Material type */
  type: MaterialType;
  /** Base color */
  color: string;
  /** Opacity (0-1) */
  opacity: number;
  /** Whether material is transparent */
  transparent: boolean;
  /** Alpha test threshold */
  alphaTest: number;
  /** Alpha hash threshold (for dithered transparency) */
  alphaHash: number;
  /** Whether to use vertex colors */
  vertexColors: boolean;
  /** Whether to use fog */
  fog: boolean;
  /** Side rendering mode */
  side: THREE.Side;
  /** Whether to render shadows */
  shadowSide: THREE.Side | null;
  /** Whether to receive shadows */
  receiveShadow: boolean;
  /** Whether to cast shadows */
  castShadow: boolean;
  /** Whether to use flat shading */
  flatShading: boolean;
  /** Whether to use wireframe rendering */
  wireframe: boolean;
  /** Wireframe line width */
  wireframeLinewidth: number;
  /** Wireframe line cap */
  wireframeLinecap: string;
  /** Wireframe line join */
  wireframeLinejoin: string;
}

/**
 * Standard material properties (MeshStandardMaterial)
 */
export interface StandardMaterialProperties extends BaseMaterialProperties {
  type: "standard";
  /** Metalness value (0-1) */
  metalness: number;
  /** Roughness value (0-1) */
  roughness: number;
  /** Bump scale */
  bumpScale: number;
  /** Displacement scale */
  displacementScale: number;
  /** Displacement bias */
  displacementBias: number;
  /** Environment map intensity */
  envMapIntensity: number;
  /** Light map intensity */
  lightMapIntensity: number;
  /** Ambient occlusion map intensity */
  aoMapIntensity: number;
  /** Emission intensity */
  emissiveIntensity: number;
  /** Emissive color */
  emissive: string;
  /** Normal map type */
  normalMapType: THREE.NormalMapTypes;
  /** Normal scale */
  normalScale: [number, number];
  /** Clearcoat level */
  clearcoat: number;
  /** Clearcoat roughness */
  clearcoatRoughness: number;
  /** IOR (Index of Refraction) */
  ior: number;
  /** Iridescence */
  iridescence: number;
  /** Iridescence IOR */
  iridescenceIOR: number;
  /** Iridescence thickness range */
  iridescenceThicknessRange: [number, number];
  /** Sheen color */
  sheenColor: string;
  /** Sheen roughness */
  sheenRoughness: number;
  /** Transmission */
  transmission: number;
  /** Thickness */
  thickness: number;
  /** Attenuation distance */
  attenuationDistance: number;
  /** Attenuation color */
  attenuationColor: string;
  /** Specular intensity */
  specularIntensity: number;
  /** Specular color */
  specularColor: string;
  /** Anisotropy */
  anisotropy: number;
  /** Anisotropy rotation */
  anisotropyRotation: number;

  // Texture channels
  /** Base color texture */
  map: TextureChannel;
  /** Alpha texture */
  alphaMap: TextureChannel;
  /** Ambient occlusion texture */
  aoMap: TextureChannel;
  /** Bump texture */
  bumpMap: TextureChannel;
  /** Displacement texture */
  displacementMap: TextureChannel;
  /** Emissive texture */
  emissiveMap: TextureChannel;
  /** Environment map */
  envMap: TextureChannel;
  /** Light map */
  lightMap: TextureChannel;
  /** Metalness texture */
  metalnessMap: TextureChannel;
  /** Normal texture */
  normalMap: TextureChannel;
  /** Roughness texture */
  roughnessMap: TextureChannel;
  /** Clearcoat texture */
  clearcoatMap: TextureChannel;
  /** Clearcoat roughness texture */
  clearcoatRoughnessMap: TextureChannel;
  /** Clearcoat normal texture */
  clearcoatNormalMap: TextureChannel;
  /** Iridescence texture */
  iridescenceMap: TextureChannel;
  /** Iridescence thickness texture */
  iridescenceThicknessMap: TextureChannel;
  /** Sheen color texture */
  sheenColorMap: TextureChannel;
  /** Sheen roughness texture */
  sheenRoughnessMap: TextureChannel;
  /** Specular intensity texture */
  specularIntensityMap: TextureChannel;
  /** Specular color texture */
  specularColorMap: TextureChannel;
  /** Transmission texture */
  transmissionMap: TextureChannel;
  /** Thickness texture */
  thicknessMap: TextureChannel;
  /** Anisotropy texture */
  anisotropyMap: TextureChannel;
}

/**
 * Physical material properties (MeshPhysicalMaterial) - extends standard
 */
export interface PhysicalMaterialProperties
  extends Omit<StandardMaterialProperties, "type"> {
  type: "physical";
  // All standard properties plus additional physical properties
}

/**
 * Basic material properties (MeshBasicMaterial)
 */
export interface BasicMaterialProperties extends BaseMaterialProperties {
  type: "basic";
  /** Whether to use alpha to coverage */
  alphaToCoverage: boolean;
  /** Whether to use color instead of texture */
  colorWrite: boolean;
  /** Whether to use depth test */
  depthTest: boolean;
  /** Whether to use depth write */
  depthWrite: boolean;
  /** Whether to use stencil test */
  stencilWrite: boolean;
  /** Stencil function */
  stencilFunc: THREE.StencilFunc;
  /** Stencil reference value */
  stencilRef: number;
  /** Stencil mask */
  stencilMask: number;
  /** Stencil fail operation */
  stencilFail: THREE.StencilOp;
  /** Stencil Z fail operation */
  stencilZFail: THREE.StencilOp;
  /** Stencil Z pass operation */
  stencilZPass: THREE.StencilOp;

  // Texture channels
  /** Base color texture */
  map: TextureChannel;
  /** Alpha texture */
  alphaMap: TextureChannel;
  /** Ambient occlusion texture */
  aoMap: TextureChannel;
  /** Environment map */
  envMap: TextureChannel;
  /** Light map */
  lightMap: TextureChannel;
  /** Specular map */
  specularMap: TextureChannel;
}

/**
 * Lambert material properties (MeshLambertMaterial)
 */
export interface LambertMaterialProperties extends BaseMaterialProperties {
  type: "lambert";
  /** Whether to use alpha to coverage */
  alphaToCoverage: boolean;
  /** Emissive color */
  emissive: string;
  /** Environment map intensity */
  envMapIntensity: number;
  /** Light map intensity */
  lightMapIntensity: number;
  /** Ambient occlusion map intensity */
  aoMapIntensity: number;
  /** Emissive intensity */
  emissiveIntensity: number;
  /** Whether to use color instead of texture */
  colorWrite: boolean;
  /** Whether to use depth test */
  depthTest: boolean;
  /** Whether to use depth write */
  depthWrite: boolean;

  // Texture channels
  /** Base color texture */
  map: TextureChannel;
  /** Alpha texture */
  alphaMap: TextureChannel;
  /** Ambient occlusion texture */
  aoMap: TextureChannel;
  /** Emissive texture */
  emissiveMap: TextureChannel;
  /** Environment map */
  envMap: TextureChannel;
  /** Light map */
  lightMap: TextureChannel;
  /** Specular map */
  specularMap: TextureChannel;
}

/**
 * Phong material properties (MeshPhongMaterial)
 */
export interface PhongMaterialProperties extends BaseMaterialProperties {
  type: "phong";
  /** Specular color */
  specular: string;
  /** Shininess */
  shininess: number;
  /** Emissive color */
  emissive: string;
  /** Environment map intensity */
  envMapIntensity: number;
  /** Light map intensity */
  lightMapIntensity: number;
  /** Ambient occlusion map intensity */
  aoMapIntensity: number;
  /** Emissive intensity */
  emissiveIntensity: number;
  /** Bump scale */
  bumpScale: number;
  /** Displacement scale */
  displacementScale: number;
  /** Displacement bias */
  displacementBias: number;
  /** Whether to use alpha to coverage */
  alphaToCoverage: boolean;
  /** Whether to use color instead of texture */
  colorWrite: boolean;
  /** Whether to use depth test */
  depthTest: boolean;
  /** Whether to use depth write */
  depthWrite: boolean;

  // Texture channels
  /** Base color texture */
  map: TextureChannel;
  /** Alpha texture */
  alphaMap: TextureChannel;
  /** Ambient occlusion texture */
  aoMap: TextureChannel;
  /** Bump texture */
  bumpMap: TextureChannel;
  /** Displacement texture */
  displacementMap: TextureChannel;
  /** Emissive texture */
  emissiveMap: TextureChannel;
  /** Environment map */
  envMap: TextureChannel;
  /** Light map */
  lightMap: TextureChannel;
  /** Normal texture */
  normalMap: TextureChannel;
  /** Specular texture */
  specularMap: TextureChannel;
}

/**
 * Toon material properties (MeshToonMaterial)
 */
export interface ToonMaterialProperties extends BaseMaterialProperties {
  type: "toon";
  /** Gradient map texture UUID */
  gradientMap: string | null;

  // Texture channels
  /** Base color texture */
  map: TextureChannel;
  /** Alpha texture */
  alphaMap: TextureChannel;
  /** Ambient occlusion texture */
  aoMap: TextureChannel;
  /** Light map */
  lightMap: TextureChannel;
}

/**
 * Normal material properties (MeshNormalMaterial)
 */
export interface NormalMaterialProperties extends BaseMaterialProperties {
  type: "normal";
  /** Normal map type */
  normalMapType: THREE.NormalMapTypes;
  /** Normal scale */
  normalScale: [number, number];

  // Texture channels
  /** Normal texture */
  normalMap: TextureChannel;
  /** Displacement texture */
  displacementMap: TextureChannel;
}

/**
 * Depth material properties (MeshDepthMaterial)
 */
export interface DepthMaterialProperties extends BaseMaterialProperties {
  type: "depth";
  /** Displacement scale */
  displacementScale: number;
  /** Displacement bias */
  displacementBias: number;

  // Texture channels
  /** Displacement texture */
  displacementMap: TextureChannel;
  /** Alpha texture */
  alphaMap: TextureChannel;
}

/**
 * Union type for all material property types
 */
export type MaterialProperties =
  | BasicMaterialProperties
  | StandardMaterialProperties
  | PhysicalMaterialProperties
  | LambertMaterialProperties
  | PhongMaterialProperties
  | ToonMaterialProperties
  | NormalMaterialProperties
  | DepthMaterialProperties;

/**
 * Material preset configuration
 */
export interface MaterialPreset {
  /** Preset unique identifier */
  id: string;
  /** Preset name */
  name: string;
  /** Preset category */
  category: string;
  /** Preset description */
  description?: string;
  /** Material properties */
  properties: Partial<MaterialProperties>;
  /** Preview color or texture */
  preview?: string;
}

/**
 * Material library entry
 */
export interface MaterialLibraryEntry {
  /** Material unique identifier */
  id: string;
  /** Material name */
  name: string;
  /** Material properties */
  properties: MaterialProperties;
  /** Creation timestamp */
  createdAt: Date;
  /** Last modified timestamp */
  modifiedAt: Date;
  /** Associated tags */
  tags: string[];
  /** Preview thumbnail */
  thumbnail?: string;
}

/**
 * Material assignment for scene objects
 */
export interface MaterialAssignment {
  /** Object ID */
  objectId: string;
  /** Material ID */
  materialId: string;
  /** Material slot index (for multi-material objects) */
  slotIndex?: number;
}

/**
 * Material system configuration
 */
export interface MaterialSystemConfig {
  /** Maximum texture size */
  maxTextureSize: number;
  /** Default texture anisotropy */
  defaultAnisotropy: number;
  /** Whether to enable texture compression */
  enableTextureCompression: boolean;
  /** Default texture format */
  defaultTextureFormat: THREE.PixelFormat;
  /** Whether to generate mipmaps */
  generateMipmaps: boolean;
  /** Whether to use premultiplied alpha */
  premultipliedAlpha: boolean;
  /** Material cache size */
  materialCacheSize: number;
  /** Texture cache size */
  textureCacheSize: number;
}

/**
 * Default texture channel configuration
 */
export const DEFAULT_TEXTURE_CHANNEL: TextureChannel = {
  texture: null,
  offset: [0, 0],
  repeat: [1, 1],
  rotation: 0,
  wrapS: THREE.RepeatWrapping,
  wrapT: THREE.RepeatWrapping,
  magFilter: THREE.LinearFilter,
  minFilter: THREE.LinearMipmapLinearFilter,
  anisotropy: 1,
  encoding: THREE.SRGBColorSpace,
  flipY: true,
};

/**
 * Default base material properties
 */
export const DEFAULT_BASE_MATERIAL_PROPERTIES: Omit<
  BaseMaterialProperties,
  "id" | "name" | "type"
> = {
  color: "#ffffff",
  opacity: 1.0,
  transparent: false,
  alphaTest: 0,
  alphaHash: 0,
  vertexColors: false,
  fog: true,
  side: THREE.FrontSide,
  shadowSide: null,
  receiveShadow: true,
  castShadow: true,
  flatShading: false,
  wireframe: false,
  wireframeLinewidth: 1,
  wireframeLinecap: "round",
  wireframeLinejoin: "round",
};

/**
 * Default standard material properties
 */
export const DEFAULT_STANDARD_MATERIAL_PROPERTIES: Omit<
  StandardMaterialProperties,
  keyof BaseMaterialProperties
> = {
  metalness: 0.0,
  roughness: 0.5,
  bumpScale: 1,
  displacementScale: 1,
  displacementBias: 0,
  envMapIntensity: 1,
  lightMapIntensity: 1,
  aoMapIntensity: 1,
  emissiveIntensity: 1,
  emissive: "#000000",
  normalMapType: THREE.TangentSpaceNormalMap,
  normalScale: [1, 1],
  clearcoat: 0,
  clearcoatRoughness: 0,
  ior: 1.5,
  iridescence: 0,
  iridescenceIOR: 1.3,
  iridescenceThicknessRange: [100, 400],
  sheenColor: "#000000",
  sheenRoughness: 1.0,
  transmission: 0,
  thickness: 0,
  attenuationDistance: 0,
  attenuationColor: "#ffffff",
  specularIntensity: 1.0,
  specularColor: "#ffffff",
  anisotropy: 0,
  anisotropyRotation: 0,
  map: { ...DEFAULT_TEXTURE_CHANNEL },
  alphaMap: { ...DEFAULT_TEXTURE_CHANNEL },
  aoMap: { ...DEFAULT_TEXTURE_CHANNEL },
  bumpMap: { ...DEFAULT_TEXTURE_CHANNEL },
  displacementMap: { ...DEFAULT_TEXTURE_CHANNEL },
  emissiveMap: { ...DEFAULT_TEXTURE_CHANNEL },
  envMap: { ...DEFAULT_TEXTURE_CHANNEL },
  lightMap: { ...DEFAULT_TEXTURE_CHANNEL },
  metalnessMap: { ...DEFAULT_TEXTURE_CHANNEL },
  normalMap: { ...DEFAULT_TEXTURE_CHANNEL },
  roughnessMap: { ...DEFAULT_TEXTURE_CHANNEL },
  clearcoatMap: { ...DEFAULT_TEXTURE_CHANNEL },
  clearcoatRoughnessMap: { ...DEFAULT_TEXTURE_CHANNEL },
  clearcoatNormalMap: { ...DEFAULT_TEXTURE_CHANNEL },
  iridescenceMap: { ...DEFAULT_TEXTURE_CHANNEL },
  iridescenceThicknessMap: { ...DEFAULT_TEXTURE_CHANNEL },
  sheenColorMap: { ...DEFAULT_TEXTURE_CHANNEL },
  sheenRoughnessMap: { ...DEFAULT_TEXTURE_CHANNEL },
  specularIntensityMap: { ...DEFAULT_TEXTURE_CHANNEL },
  specularColorMap: { ...DEFAULT_TEXTURE_CHANNEL },
  transmissionMap: { ...DEFAULT_TEXTURE_CHANNEL },
  thicknessMap: { ...DEFAULT_TEXTURE_CHANNEL },
  anisotropyMap: { ...DEFAULT_TEXTURE_CHANNEL },
};

/**
 * Default material system configuration
 */
export const DEFAULT_MATERIAL_SYSTEM_CONFIG: MaterialSystemConfig = {
  maxTextureSize: 2048,
  defaultAnisotropy: 1,
  enableTextureCompression: true,
  defaultTextureFormat: THREE.RGBAFormat,
  generateMipmaps: true,
  premultipliedAlpha: false,
  materialCacheSize: 100,
  textureCacheSize: 50,
};
