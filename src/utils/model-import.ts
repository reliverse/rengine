import * as THREE from "three";
import {
  Box3,
  type Box3JSON,
  Mesh,
  MeshBasicMaterial,
  MeshStandardMaterial,
  type Object3D,
  type Object3DJSON,
  ObjectLoader,
  Vector3,
} from "three";
import type { SceneLight, SceneObject } from "~/stores/scene-store";
import { createAnimationController } from "./animation-system";
import { validateGLTF } from "./gltf-extensions";
import { textureOptimizer } from "./texture-optimizer";
import { workerManager } from "./worker-manager";
import { invoke } from "@tauri-apps/api/core";

// Type for the result from worker manager parseModel
interface ParseResult {
  object: Object3DJSON; // THREE.Object3D JSON representation (full structure with metadata)
  boundingBox?: Box3JSON; // THREE.Box3 JSON representation
  lights?: any[]; // Extracted lights from GLTF
  cameras?: any[]; // Extracted cameras from GLTF
  validation?: any; // GLTF extension validation results
  animations?: any[]; // GLTF animations JSON
  gltf?: any; // Full GLTF object for animation access
}

// Regex patterns for file path parsing (consistent with assets-panel.tsx)
const PATH_SEPARATOR_REGEX = /[/\\]/;
const FILE_EXTENSION_REGEX = /\.(dff|obj|txd|col|ipl)$/i;

// Asset cache for loaded models
const MODEL_CACHE = new Map<
  string,
  { model: Object3D; lastAccessed: number }
>();
const MAX_CACHE_SIZE = 10; // Maximum number of cached models
const CACHE_TTL = 5 * 60 * 1000; // 5 minutes TTL

// Worker pool for off-main-thread processing (if available) - reserved for future use
// const workerPool: Worker[] | null = null;

export interface ImportProgress {
  loaded: number;
  total: number;
  stage: string;
}

export interface ImportResult {
  success: boolean;
  objects?: SceneObject[]; // Now returns array of objects for hierarchy support
  object?: SceneObject; // Keep for backward compatibility
  lights?: SceneLight[];
  cameras?: any[]; // Camera objects from GLTF
  error?: string;
  warnings?: string[];
}

// Loaders kept for potential future fallback use

const validateFile = (
  file: File
): {
  valid: boolean;
  error?: string;
  warnings: string[];
} => {
  const warnings: string[] = [];
  const fileExtension = file.name.split(".").pop()?.toLowerCase();

  // For GLTF/GLB files, do comprehensive validation
  if (fileExtension === "gltf" || fileExtension === "glb") {
    try {
      // Quick file-level validation first
      const basicValidation = validateGLTF(file);
      warnings.push(...basicValidation.warnings);

      if (!basicValidation.valid) {
        return {
          valid: false,
          error: basicValidation.errors.join("; "),
          warnings,
        };
      }
    } catch (error) {
      // If validation fails, continue with basic checks
      console.warn("GLTF validation failed:", error);
    }
  }

  const maxFileSize = 50 * 1024 * 1024; // 50MB limit
  if (file.size > maxFileSize) {
    return {
      valid: false,
      error: `File size (${(file.size / (1024 * 1024)).toFixed(1)}MB) exceeds maximum limit of 50MB`,
      warnings,
    };
  }

  if (file.size > 10 * 1024 * 1024) {
    warnings.push("Large file detected - loading may take longer");
  }

  if (file.size === 0) {
    return {
      valid: false,
      error: "File is empty",
      warnings,
    };
  }

  const supportedFormats = ["gltf", "glb", "obj", "fbx"];
  if (!(fileExtension && supportedFormats.includes(fileExtension))) {
    return {
      valid: false,
      error: `Unsupported format: ${fileExtension}. Supported: ${supportedFormats.join(", ")}`,
      warnings,
    };
  }

  return { valid: true, warnings };
};

// Cache management functions
const cleanCache = () => {
  const now = Date.now();
  const entries = Array.from(MODEL_CACHE.entries());

  // Remove expired entries
  for (const [key, value] of entries) {
    if (now - value.lastAccessed > CACHE_TTL) {
      MODEL_CACHE.delete(key);
    }
  }

  // If still too many entries, remove oldest
  if (MODEL_CACHE.size > MAX_CACHE_SIZE) {
    const sortedEntries = entries
      .filter(([key]) => MODEL_CACHE.has(key))
      .sort((a, b) => a[1].lastAccessed - b[1].lastAccessed);

    const toRemove = sortedEntries.slice(0, MODEL_CACHE.size - MAX_CACHE_SIZE);
    for (const [key] of toRemove) {
      MODEL_CACHE.delete(key);
    }
  }
};

const getCacheKey = (file: File): string => {
  return `${file.name}_${file.size}_${file.lastModified}`;
};

const getCachedModel = (file: File): Object3D | null => {
  const key = getCacheKey(file);
  const cached = MODEL_CACHE.get(key);

  if (cached) {
    cached.lastAccessed = Date.now();
    // Return a clone to avoid modifying the cached version
    return cached.model.clone();
  }

  return null;
};

const cacheModel = (file: File, model: Object3D): void => {
  const key = getCacheKey(file);
  MODEL_CACHE.set(key, { model: model.clone(), lastAccessed: Date.now() });
  cleanCache();
};

// Optimized loading functions with caching and web workers
const loadGLTF = (
  file: File,
  onProgress?: (progress: ImportProgress) => void
): Promise<Object3D> => {
  // Check cache first
  const cached = getCachedModel(file);
  if (cached) {
    onProgress?.({ loaded: 100, total: 100, stage: "Loaded from cache" });
    return Promise.resolve(cached);
  }

  return new Promise((resolve, reject) => {
    onProgress?.({ loaded: 0, total: 100, stage: "Reading file" });

    const reader = new FileReader();
    reader.onload = async (event) => {
      try {
        const arrayBuffer = event.target?.result as ArrayBuffer;

        onProgress?.({
          loaded: 10,
          total: 100,
          stage: "Processing with worker",
        });

        // Use web worker for parsing
        const result: ParseResult = await workerManager.parseModel(
          "gltf",
          arrayBuffer,
          onProgress
        );

        // Convert JSON back to Three.js object
        const loader = new ObjectLoader();
        const object3D = loader.parse(result.object);

        // Restore bounding box if it exists
        if (result.boundingBox) {
          const box = new Box3();
          box.fromJSON(result.boundingBox);
          object3D.userData.boundingBox = box;
        }

        // Store GLTF metadata
        if (result.lights) {
          object3D.userData.gltfLights = result.lights;
        }
        if (result.cameras) {
          object3D.userData.gltfCameras = result.cameras;
        }
        if (result.validation) {
          object3D.userData.gltfValidation = result.validation;
        }
        if (result.animations) {
          object3D.userData.gltfAnimations = result.animations;
        }
        if (result.gltf) {
          object3D.userData.gltf = result.gltf;
        }

        // Optimize textures in the model
        optimizeModelTextures(object3D);

        // Cache the model
        cacheModel(file, object3D);

        resolve(object3D);
      } catch (error) {
        console.error("GLTF parse error:", error);
        reject(new Error("Failed to parse GLTF file"));
      }
    };
    reader.onerror = () => reject(new Error("Failed to read file"));
    reader.readAsArrayBuffer(file);
  });
};

const loadOBJ = (
  file: File,
  onProgress?: (progress: ImportProgress) => void
): Promise<Object3D> => {
  // Check cache first
  const cached = getCachedModel(file);
  if (cached) {
    onProgress?.({ loaded: 100, total: 100, stage: "Loaded from cache" });
    return Promise.resolve(cached);
  }

  return new Promise((resolve, reject) => {
    onProgress?.({ loaded: 0, total: 100, stage: "Reading file" });

    const reader = new FileReader();
    reader.onload = async (event) => {
      try {
        const text = event.target?.result as string;

        onProgress?.({
          loaded: 25,
          total: 100,
          stage: "Processing with worker",
        });

        // Use web worker for parsing
        const result: ParseResult = await workerManager.parseModel(
          "obj",
          text,
          onProgress
        );

        // Convert JSON back to Three.js object
        const loader = new ObjectLoader();
        const object3D = loader.parse(result.object);

        // Restore bounding box if it exists
        if (result.boundingBox) {
          const box = new Box3();
          box.fromJSON(result.boundingBox);
          object3D.userData.boundingBox = box;
        }

        // Optimize textures in the model
        optimizeModelTextures(object3D);

        // Cache the model
        cacheModel(file, object3D);

        resolve(object3D);
      } catch (error) {
        console.error("OBJ parsing error:", error);
        reject(new Error("Failed to parse OBJ file - invalid format"));
      }
    };
    reader.onerror = () => reject(new Error("Failed to read OBJ file"));
    reader.readAsText(file);
  });
};

const loadFBX = (
  file: File,
  onProgress?: (progress: ImportProgress) => void
): Promise<Object3D> => {
  // Check cache first
  const cached = getCachedModel(file);
  if (cached) {
    onProgress?.({ loaded: 100, total: 100, stage: "Loaded from cache" });
    return Promise.resolve(cached);
  }

  return new Promise((resolve, reject) => {
    onProgress?.({ loaded: 0, total: 100, stage: "Reading file" });

    const reader = new FileReader();
    reader.onload = async (event) => {
      try {
        const arrayBuffer = event.target?.result as ArrayBuffer;

        onProgress?.({
          loaded: 25,
          total: 100,
          stage: "Processing with worker",
        });

        // Use web worker for parsing
        const result: ParseResult = await workerManager.parseModel(
          "fbx",
          arrayBuffer,
          onProgress
        );

        // Convert JSON back to Three.js object
        const loader = new ObjectLoader();
        const object3D = loader.parse(result.object);

        // Restore bounding box if it exists
        if (result.boundingBox) {
          const box = new Box3();
          box.fromJSON(result.boundingBox);
          object3D.userData.boundingBox = box;
        }

        // Optimize textures in the model
        optimizeModelTextures(object3D);

        // Cache the model
        cacheModel(file, object3D);

        resolve(object3D);
      } catch (error) {
        console.error("FBX parsing error:", error);
        reject(new Error("Failed to parse FBX file - invalid format"));
      }
    };
    reader.onerror = () => reject(new Error("Failed to read FBX file"));
    reader.readAsArrayBuffer(file);
  });
};

// Model optimization is now handled in the web worker

// Material optimization is now handled in the web worker

// Optimize textures in imported models
const optimizeModelTextures = (model: Object3D): void => {
  model.traverse((child) => {
    if (child instanceof Mesh && child.material) {
      const materials = Array.isArray(child.material)
        ? child.material
        : [child.material];

      for (const material of materials) {
        // Optimize textures if they exist
        if ("map" in material && material.map) {
          textureOptimizer.optimizeTexture(material.map);
        }
        if ("normalMap" in material && material.normalMap) {
          textureOptimizer.optimizeTexture(material.normalMap);
        }
        if ("roughnessMap" in material && material.roughnessMap) {
          textureOptimizer.optimizeTexture(material.roughnessMap);
        }
        if ("metalnessMap" in material && material.metalnessMap) {
          textureOptimizer.optimizeTexture(material.metalnessMap);
        }
        if ("emissiveMap" in material && material.emissiveMap) {
          textureOptimizer.optimizeTexture(material.emissiveMap);
        }
        if ("aoMap" in material && material.aoMap) {
          textureOptimizer.optimizeTexture(material.aoMap);
        }
      }
    }
  });
};

const createSceneObjectsFromModel = async (
  model: Object3D,
  fileName: string,
  position: [number, number, number],
  warnings: string[]
): Promise<SceneObject[]> => {
  const sceneObjects: SceneObject[] = [];
  let totalVertexCount = 0;
  let totalGeometryCount = 0;

  // Count meshes and vertices
  model.traverse((child) => {
    if (child instanceof Mesh && child.geometry) {
      totalGeometryCount++;
      if (child.geometry.attributes.position) {
        totalVertexCount += child.geometry.attributes.position.count;
      }
    }
  });

  // Calculate overall bounding box for scaling
  const box = new Box3().setFromObject(model);
  const size = box.getSize(new Vector3());
  const center = box.getCenter(new Vector3());

  if (
    !box.isEmpty() &&
    (Number.isNaN(size.x) || Number.isNaN(size.y) || Number.isNaN(size.z))
  ) {
    warnings.push(
      "Model has invalid geometry - scaling may not work correctly"
    );
  }

  // Center the model
  if (!box.isEmpty() && center.length() > 0.001) {
    model.position.sub(center);
  }

  const maxDimension = Math.max(size.x, size.y, size.z);
  const globalScale = maxDimension > 0 ? 2 / maxDimension : 1;
  const safeGlobalScale =
    Number.isNaN(globalScale) || !Number.isFinite(globalScale)
      ? 1
      : globalScale;

  // Apply initial scaling to the model
  model.scale.setScalar(safeGlobalScale);

  // Ensure all meshes have materials for proper rendering and selection
  model.traverse((child) => {
    if (child instanceof Mesh) {
      if (!child.material) {
        child.material = new MeshStandardMaterial({
          color: 0xff_ff_ff,
          roughness: 0.5,
          metalness: 0.0,
        });
      }
      // Ensure material is an array or single material for proper raycasting
      if (Array.isArray(child.material)) {
        for (const mat of child.material) {
          if (
            mat instanceof MeshStandardMaterial ||
            mat instanceof MeshBasicMaterial
          ) {
            mat.transparent = false;
          }
        }
      } else if (
        child.material instanceof MeshStandardMaterial ||
        child.material instanceof MeshBasicMaterial
      ) {
        child.material.transparent = false;
      }
    }
  });

  if (totalVertexCount > 100_000) {
    warnings.push(
      `High vertex count (${totalVertexCount.toLocaleString()}) - may impact performance`
    );
  } else if (totalVertexCount > 50_000) {
    warnings.push(
      `Large model (${totalVertexCount.toLocaleString()} vertices)`
    );
  }

  if (totalGeometryCount > 50) {
    warnings.push(
      `Complex model with ${totalGeometryCount} geometry objects - rendered as single selectable object`
    );
  }

  // Extract base name for SA:MP model lookup
  const fileNameOnly = fileName.split(PATH_SEPARATOR_REGEX).pop() || "";
  const baseName = fileNameOnly.replace(FILE_EXTENSION_REGEX, "");

  // Try to find matching SA:MP model (only if invoke is available and baseName is valid)
  let sampModelId: number | undefined;
  if (baseName && baseName.length > 0) {
    try {
      // Check if we're in an environment where invoke is available
      if (typeof invoke !== "undefined") {
        const sampModel = (await invoke("get_samp_model_by_name", {
          name: baseName,
        })) as { id: number; name: string; dff: string; txd: string } | null;

        if (sampModel) {
          sampModelId = sampModel.id;
          console.log(
            `✅ Auto-matched "${baseName}" to SA:MP model ID ${sampModelId} (${sampModel.dff})`
          );
        } else {
          console.log(`ℹ️ No SA:MP model found for "${baseName}"`);
        }
      }
    } catch (error) {
      // SA:MP database not available or other error - continue without model ID
      console.log(`⚠️ SA:MP model lookup failed for "${baseName}":`, error);
    }
  }

  // Create a single scene object for the entire model (better for selection and performance)
  const sceneObject: SceneObject = {
    id: `imported_${Math.random().toString(36).substr(2, 9)}`,
    name: baseName,
    type: "imported",
    position,
    rotation: [0, 0, 0],
    scale: [1, 1, 1], // Scaling is applied to the model itself
    color: "#ffffff",
    visible: true,
    importedModel: model,
    initialScale: safeGlobalScale,
    modelid: sampModelId, // Auto-set SA:MP model ID if found
  };

  sceneObjects.push(sceneObject);

  return sceneObjects;
};

export const importFromFile = async (
  file: File,
  position: [number, number, number] = [0, 0, 0],
  onProgress?: (progress: ImportProgress) => void
): Promise<ImportResult> => {
  const fileExtension = file.name.split(".").pop()?.toLowerCase();
  const warnings: string[] = [];

  try {
    onProgress?.({ loaded: 0, total: 100, stage: "Validating file" });

    const validation = await validateFile(file);
    if (!validation.valid) {
      return {
        success: false,
        error: validation.error,
      };
    }
    warnings.push(...validation.warnings);

    onProgress?.({ loaded: 10, total: 100, stage: "Loading model" });

    let object3D: Object3D | null = null;

    switch (fileExtension) {
      case "gltf":
      case "glb":
        object3D = await loadGLTF(file, onProgress);
        // Add GLTF extension warnings
        if (object3D?.userData.gltfValidation) {
          warnings.push(...object3D.userData.gltfValidation.warnings);
        }
        break;
      case "obj":
        object3D = await loadOBJ(file, onProgress);
        break;
      case "fbx":
        object3D = await loadFBX(file, onProgress);
        break;
      default:
        return {
          success: false,
          error: `Unsupported file format: ${fileExtension}. Supported formats: GLTF, GLB, OBJ, FBX`,
        };
    }

    if (!object3D) {
      return {
        success: false,
        error: "Failed to load model",
      };
    }

    onProgress?.({ loaded: 90, total: 100, stage: "Creating scene objects" });

    const sceneObjects = await createSceneObjectsFromModel(
      object3D,
      file.name,
      position,
      warnings
    );

    // Create animation controller if animations are present
    if (
      object3D.userData.gltfAnimations &&
      object3D.userData.gltfAnimations.length > 0
    ) {
      try {
        // Convert animation JSON back to AnimationClips
        const animations = object3D.userData.gltfAnimations.map(
          (animData: any) => {
            return THREE.AnimationClip.parse(animData);
          }
        );

        // Create a temporary GLTF-like object for the animation controller
        const gltfWithAnimations = {
          animations,
          // Add minimal GLTF structure that the animation controller might expect
          scene: object3D,
        };

        const animationController = createAnimationController(
          gltfWithAnimations,
          object3D
        );
        // Attach animation controller to the first scene object
        if (sceneObjects.length > 0) {
          sceneObjects[0].animationController = animationController;
        }

        if (animations.length > 0) {
          warnings.push(
            `Loaded ${animations.length} animation(s): ${animations.map((a: THREE.AnimationClip) => a.name || "Unnamed").join(", ")}`
          );
        }
      } catch (error) {
        warnings.push(`Failed to create animation controller: ${error}`);
      }
    }

    // Create scene lights from GLTF lights
    let sceneLights: SceneLight[] | undefined;
    if (
      object3D.userData.gltfLights &&
      object3D.userData.gltfLights.length > 0
    ) {
      try {
        sceneLights = createSceneLightsFromGLTF(
          object3D.userData.gltfLights,
          position
        );
        if (sceneLights.length > 0) {
          warnings.push(`Imported ${sceneLights.length} light(s) from GLTF`);
        }
      } catch (error) {
        warnings.push(`Failed to import lights: ${error}`);
      }
    }

    // Create scene cameras from GLTF cameras
    let sceneCameras: any[] | undefined;
    if (
      object3D.userData.gltfCameras &&
      object3D.userData.gltfCameras.length > 0
    ) {
      try {
        sceneCameras = createSceneCamerasFromGLTF(
          object3D.userData.gltfCameras
        );
        if (sceneCameras.length > 0) {
          warnings.push(`Imported ${sceneCameras.length} camera(s) from GLTF`);
        }
      } catch (error) {
        warnings.push(`Failed to import cameras: ${error}`);
      }
    }

    onProgress?.({ loaded: 100, total: 100, stage: "Import complete" });

    return {
      success: true,
      objects: sceneObjects,
      object: sceneObjects[0], // Keep for backward compatibility
      lights: sceneLights,
      cameras: sceneCameras,
      warnings: warnings.length > 0 ? warnings : undefined,
    };
  } catch (error) {
    console.error("Model import error:", error);
    return {
      success: false,
      error: `Failed to import model: ${error instanceof Error ? error.message : "Unknown error"}`,
      warnings,
    };
  }
};

// Convert GLTF lights to scene lights
function createSceneLightsFromGLTF(
  gltfLights: any[],
  basePosition: [number, number, number]
): SceneLight[] {
  if (!gltfLights || gltfLights.length === 0) return [];

  return gltfLights.map((lightData: any, index: number) => {
    // Parse the light data (it's stored as JSON)
    const light = new ObjectLoader().parse(lightData) as THREE.Light;

    // Determine light type and properties
    let lightType: SceneLight["type"] = "directional";
    let color = "#ffffff";
    let intensity = 1;
    let distance = 0;
    let decay = 1;

    if (light instanceof THREE.DirectionalLight) {
      lightType = "directional";
      color = `#${light.color.getHexString()}`;
      intensity = light.intensity || 1;
    } else if (light instanceof THREE.PointLight) {
      lightType = "point";
      color = `#${light.color.getHexString()}`;
      intensity = light.intensity || 1;
      distance = light.distance || 0;
      decay = light.decay || 1;
    } else if (light instanceof THREE.SpotLight) {
      lightType = "spot";
      color = `#${light.color.getHexString()}`;
      intensity = light.intensity || 1;
      distance = light.distance || 0;
      decay = light.decay || 1;
    }

    // Calculate position relative to the imported model
    const position: [number, number, number] = [
      basePosition[0] + (light.position?.x || 0),
      basePosition[1] + (light.position?.y || 0),
      basePosition[2] + (light.position?.z || 0),
    ];

    // Calculate target for directional and spot lights
    let target: [number, number, number] | undefined;
    if (
      (lightType === "directional" || lightType === "spot") &&
      light instanceof THREE.DirectionalLight &&
      light.target
    ) {
      target = [
        basePosition[0] + light.target.position.x,
        basePosition[1] + light.target.position.y,
        basePosition[2] + light.target.position.z,
      ];
    } else if (
      lightType === "spot" &&
      light instanceof THREE.SpotLight &&
      light.target
    ) {
      target = [
        basePosition[0] + light.target.position.x,
        basePosition[1] + light.target.position.y,
        basePosition[2] + light.target.position.z,
      ];
    }

    return {
      id: `gltf_light_${Date.now()}_${index}`,
      name: `GLTF Light ${index + 1}`,
      type: lightType,
      position,
      target,
      color,
      intensity,
      distance,
      decay,
      castShadow: light.castShadow,
      shadowMapSize:
        light.shadow?.mapSize instanceof THREE.Vector2
          ? light.shadow.mapSize.x
          : typeof light.shadow?.mapSize === "number"
            ? light.shadow.mapSize
            : 1024,
      shadowCameraNear:
        (
          light.shadow?.camera as
            | THREE.PerspectiveCamera
            | THREE.OrthographicCamera
        )?.near || 0.1,
      shadowCameraFar:
        (
          light.shadow?.camera as
            | THREE.PerspectiveCamera
            | THREE.OrthographicCamera
        )?.far || 100,
      shadowCameraFov:
        (light.shadow?.camera as THREE.PerspectiveCamera)?.fov || 50,
      shadowBias: light.shadow?.bias || 0,
      visible: true,
    };
  });
}

// Convert GLTF cameras to scene cameras (for now, we'll store them as metadata)
function createSceneCamerasFromGLTF(gltfCameras: any[]): any[] {
  if (!gltfCameras || gltfCameras.length === 0) return [];

  return gltfCameras.map((cameraData: any, index: number) => {
    // Parse the camera data
    const camera = new ObjectLoader().parse(cameraData) as THREE.Camera;

    return {
      id: `gltf_camera_${Date.now()}_${index}`,
      name: `GLTF Camera ${index + 1}`,
      camera,
      type:
        camera instanceof THREE.PerspectiveCamera
          ? "perspective"
          : "orthographic",
    };
  });
}

// Utility function to clear cache (useful for memory management)
export const clearModelCache = (): void => {
  MODEL_CACHE.clear();
};

// Get cache statistics
export const getCacheStats = () => ({
  size: MODEL_CACHE.size,
  maxSize: MAX_CACHE_SIZE,
  ttl: CACHE_TTL,
});

export const modelImporter = { importFromFile };
export default modelImporter;
