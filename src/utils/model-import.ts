import {
  Box3,
  type Box3JSON,
  Mesh,
  type Object3D,
  type Object3DJSON,
  ObjectLoader,
  Vector3,
} from "three";
import type { SceneObject } from "~/stores/scene-store";
import { textureOptimizer } from "./texture-optimizer";
import { workerManager } from "./worker-manager";

// Type for the result from worker manager parseModel
interface ParseResult {
  object: Object3DJSON; // THREE.Object3D JSON representation (full structure with metadata)
  boundingBox?: Box3JSON; // THREE.Box3 JSON representation
}

const FILE_EXTENSION_REGEX = /\.[^/.]+$/;

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
  object?: SceneObject;
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
  const maxFileSize = 50 * 1024 * 1024; // 50MB limit
  const fileExtension = file.name.split(".").pop()?.toLowerCase();

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

const createSceneObjectFromModel = (
  model: Object3D,
  fileName: string,
  position: [number, number, number],
  warnings: string[]
): SceneObject => {
  let vertexCount = 0;
  let geometryCount = 0;

  model.traverse((child) => {
    if (child instanceof Mesh && child.geometry) {
      geometryCount++;
      if (child.geometry.attributes.position) {
        vertexCount += child.geometry.attributes.position.count;
      }
    }
  });

  if (vertexCount > 100_000) {
    warnings.push(
      `High vertex count (${vertexCount.toLocaleString()}) - may impact performance`
    );
  } else if (vertexCount > 50_000) {
    warnings.push(`Large model (${vertexCount.toLocaleString()} vertices)`);
  }

  if (geometryCount > 50) {
    warnings.push(
      `Many geometry objects (${geometryCount}) - consider optimizing`
    );
  }

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

  if (!box.isEmpty() && center.length() > 0.001) {
    model.position.sub(center);
  }

  const maxDimension = Math.max(size.x, size.y, size.z);
  const scale = maxDimension > 0 ? 2 / maxDimension : 1;

  const safeScale = Number.isNaN(scale) || !Number.isFinite(scale) ? 1 : scale;

  return {
    id: `imported_${Math.random().toString(36).substr(2, 9)}`,
    name: fileName.replace(FILE_EXTENSION_REGEX, ""),
    type: "imported",
    position,
    rotation: [0, 0, 0],
    scale: [1, 1, 1],
    color: "#ffffff",
    visible: true,
    importedModel: model,
    initialScale: safeScale,
  };
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

    const validation = validateFile(file);
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

    onProgress?.({ loaded: 90, total: 100, stage: "Creating scene object" });

    const sceneObject = createSceneObjectFromModel(
      object3D,
      file.name,
      position,
      warnings
    );

    onProgress?.({ loaded: 100, total: 100, stage: "Import complete" });

    return {
      success: true,
      object: sceneObject,
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
