import { Box3, Mesh, type Object3D, Vector3 } from "three";
import { FBXLoader } from "three/examples/jsm/loaders/FBXLoader.js";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import type { SceneObject } from "~/stores/scene-store";

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

class ModelImporter {
  private readonly gltfLoader = new GLTFLoader();
  private readonly objLoader = new OBJLoader();
  private readonly fbxLoader = new FBXLoader();

  async importFromFile(
    file: File,
    position: [number, number, number] = [0, 0, 0],
    onProgress?: (progress: ImportProgress) => void
  ): Promise<ImportResult> {
    const fileExtension = file.name.split(".").pop()?.toLowerCase();
    const warnings: string[] = [];

    try {
      // Validate file
      onProgress?.({ loaded: 0, total: 100, stage: "Validating file..." });
      const validation = this.validateFile(file);
      if (!validation.valid) {
        return {
          success: false,
          error: validation.error,
        };
      }
      warnings.push(...validation.warnings);

      onProgress?.({ loaded: 10, total: 100, stage: "Loading model..." });

      let object3D: Object3D | null = null;

      switch (fileExtension) {
        case "gltf":
        case "glb":
          object3D = await this.loadGLTF(file, (progress) =>
            onProgress?.({
              loaded: 10 + progress * 0.7,
              total: 100,
              stage: "Loading GLTF...",
            })
          );
          break;
        case "obj":
          object3D = await this.loadOBJ(file, (progress) =>
            onProgress?.({
              loaded: 10 + progress * 0.7,
              total: 100,
              stage: "Parsing OBJ...",
            })
          );
          break;
        case "fbx":
          object3D = await this.loadFBX(file, (progress) =>
            onProgress?.({
              loaded: 10 + progress * 0.7,
              total: 100,
              stage: "Loading FBX...",
            })
          );
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

      onProgress?.({ loaded: 80, total: 100, stage: "Processing model..." });

      // Create a scene object from the loaded model
      const sceneObject = this.createSceneObjectFromModel(
        object3D,
        file.name,
        position,
        warnings
      );

      onProgress?.({ loaded: 100, total: 100, stage: "Complete" });

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
  }

  private validateFile(file: File): {
    valid: boolean;
    error?: string;
    warnings: string[];
  } {
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
      // 10MB warning
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
  }

  private loadGLTF(
    file: File,
    onProgress?: (progress: number) => void
  ): Promise<Object3D> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = (event) => {
        const arrayBuffer = event.target?.result as ArrayBuffer;
        this.gltfLoader.parse(
          arrayBuffer,
          "",
          (gltf) => {
            resolve(gltf.scene);
          },
          (error) => {
            console.error("GLTF parse error:", error);
            reject(new Error("Failed to parse GLTF file"));
          }
        );
      };
      reader.onerror = () => reject(new Error("Failed to read file"));
      reader.onprogress = (event) => {
        if (event.lengthComputable && onProgress) {
          onProgress(event.loaded / event.total);
        }
      };
      reader.readAsArrayBuffer(file);
    });
  }

  private loadOBJ(
    file: File,
    onProgress?: (progress: number) => void
  ): Promise<Object3D> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = (event) => {
        const text = event.target?.result as string;
        try {
          const object = this.objLoader.parse(text);
          resolve(object);
        } catch (error) {
          console.error("OBJ parsing error:", error);
          reject(new Error("Failed to parse OBJ file - invalid format"));
        }
      };
      reader.onerror = () => reject(new Error("Failed to read OBJ file"));
      reader.onprogress = (event) => {
        if (event.lengthComputable && onProgress) {
          onProgress(event.loaded / event.total);
        }
      };
      reader.readAsText(file);
    });
  }

  private loadFBX(
    file: File,
    onProgress?: (progress: number) => void
  ): Promise<Object3D> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = (event) => {
        try {
          const arrayBuffer = event.target?.result as ArrayBuffer;
          const object = this.fbxLoader.parse(arrayBuffer, "");
          resolve(object);
        } catch (error) {
          console.error("FBX parsing error:", error);
          reject(new Error("Failed to parse FBX file - invalid format"));
        }
      };
      reader.onerror = () => reject(new Error("Failed to read FBX file"));
      reader.onprogress = (event) => {
        if (event.lengthComputable && onProgress) {
          onProgress(event.loaded / event.total);
        }
      };
      reader.readAsArrayBuffer(file);
    });
  }

  private createSceneObjectFromModel(
    model: Object3D,
    fileName: string,
    position: [number, number, number],
    warnings: string[]
  ): SceneObject {
    // Count geometry and validate complexity
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

    // Add warnings for complex models
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

    // Calculate bounding box to get proper scaling and centering
    const box = new Box3().setFromObject(model);
    const size = box.getSize(new Vector3());
    const center = box.getCenter(new Vector3());

    // Check for invalid bounding box
    if (
      !box.isEmpty() &&
      (Number.isNaN(size.x) || Number.isNaN(size.y) || Number.isNaN(size.z))
    ) {
      warnings.push(
        "Model has invalid geometry - scaling may not work correctly"
      );
    }

    // Center the model at origin (don't apply initial scaling to the model itself)
    if (!box.isEmpty() && center.length() > 0.001) {
      model.position.sub(center);
    }

    const maxDimension = Math.max(size.x, size.y, size.z);
    const scale = maxDimension > 0 ? 2 / maxDimension : 1; // Scale to fit within ~2 units

    // Ensure scale is valid
    const safeScale =
      Number.isNaN(scale) || !Number.isFinite(scale) ? 1 : scale;

    return {
      id: `imported_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      name: fileName.replace(FILE_EXTENSION_REGEX, ""), // Remove file extension
      type: "imported",
      position,
      rotation: [0, 0, 0],
      scale: [1, 1, 1], // Start with neutral scale, initial scaling is stored separately
      color: "#ffffff",
      visible: true,
      importedModel: model,
      initialScale: safeScale, // Store the initial scaling factor separately
    };
  }
}

// Top-level regex for performance
const FILE_EXTENSION_REGEX = /\.[^/.]+$/;

// Singleton instance
const modelImporter = new ModelImporter();

export { modelImporter };
export default modelImporter;
