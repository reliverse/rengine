import * as THREE from "three";
import type { ImportProgress } from "./model-import";

interface WorkerTask {
  id: string;
  resolve: (value: ParseResult) => void;
  reject: (error: unknown) => void;
  onProgress?: (progress: ImportProgress) => void;
}

interface ParseResult {
  object: THREE.Object3DJSON; // THREE.Object3D JSON representation (full structure with metadata)
  boundingBox?: THREE.Box3JSON; // THREE.Box3 JSON representation
  lights?: any[]; // Extracted lights from GLTF
  cameras?: any[]; // Extracted cameras from GLTF
  validation?: any; // GLTF extension validation results
  animations?: any[]; // GLTF animations JSON
  gltf?: any; // Full GLTF object for animation access
}

export class WorkerManager {
  private worker: Worker | null = null;
  private readonly tasks = new Map<string, WorkerTask>();
  private isInitialized = false;

  constructor() {
    this.initializeWorker();
  }

  private initializeWorker() {
    try {
      // Create worker from the model-worker.ts file
      this.worker = new Worker(new URL("./model-worker.ts", import.meta.url), {
        type: "module",
      });

      this.worker.onmessage = (event) => {
        const { type, data, error, progress, id } = event.data;
        const task = this.tasks.get(id);

        if (!task) return;

        switch (type) {
          case "progress":
            task.onProgress?.({
              loaded: progress,
              total: 100,
              stage: "Processing in worker",
            });
            break;

          case "success":
            // Convert JSON back to Three.js object
            task.resolve(data);
            this.tasks.delete(id);
            break;

          case "error":
            task.reject(new Error(error));
            this.tasks.delete(id);
            break;
          default:
            console.warn(`Unknown worker message type: ${type}`);
            break;
        }
      };

      this.worker.onerror = (error) => {
        console.error("Worker error:", error);
        // Reject all pending tasks
        for (const [id, task] of this.tasks) {
          task.reject(new Error("Worker error"));
          this.tasks.delete(id);
        }
      };

      this.isInitialized = true;
    } catch (error) {
      console.warn(
        "Web Workers not supported, falling back to main thread:",
        error
      );
      this.isInitialized = false;
    }
  }

  // Parse model using worker
  parseModel(
    type: "gltf" | "obj" | "fbx",
    data: ArrayBuffer | string,
    onProgress?: (progress: ImportProgress) => void
  ): Promise<ParseResult> {
    // Fallback to main thread if worker not available
    if (!(this.isInitialized && this.worker)) {
      return this.fallbackParse(type, data, onProgress);
    }

    return new Promise((resolve, reject) => {
      const id = Math.random().toString(36).substr(2, 9);

      this.tasks.set(id, {
        id,
        resolve,
        reject,
        onProgress,
      });

      // Send message to worker
      const message = {
        type: `parse_${type}`,
        data: type === "obj" ? { text: data } : { arrayBuffer: data },
        id,
      };

      this.worker?.postMessage(message);

      // Transfer ArrayBuffer ownership to worker for better performance
      if (type !== "obj" && data instanceof ArrayBuffer) {
        this.worker?.postMessage(message, [data]);
      }
    });
  }

  // Fallback parsing on main thread
  private async fallbackParse(
    type: "gltf" | "obj" | "fbx",
    data: ArrayBuffer | string,
    onProgress?: (progress: ImportProgress) => void
  ): Promise<ParseResult> {
    // Import the original parsing functions dynamically
    const { GLTFLoader } = await import(
      "three/examples/jsm/loaders/GLTFLoader.js"
    );
    const { DRACOLoader } = await import(
      "three/examples/jsm/loaders/DRACOLoader.js"
    );
    const { KTX2Loader } = await import(
      "three/examples/jsm/loaders/KTX2Loader.js"
    );
    const { OBJLoader } = await import(
      "three/examples/jsm/loaders/OBJLoader.js"
    );
    const { FBXLoader } = await import(
      "three/examples/jsm/loaders/FBXLoader.js"
    );
    const { validateGLTFExtensions, extractGLTFLights, extractGLTFCameras } =
      await import("./gltf-extensions.js");

    switch (type) {
      case "gltf":
        return new Promise((resolve, reject) => {
          const loader = new GLTFLoader();

          // Configure Draco loader for mesh compression
          const dracoLoader = new DRACOLoader();
          dracoLoader.setDecoderPath(
            "https://www.gstatic.com/draco/versioned/decoders/1.5.6/"
          );
          loader.setDRACOLoader(dracoLoader);

          // Configure KTX2 loader for texture compression
          const ktx2Loader = new KTX2Loader();
          ktx2Loader.setTranscoderPath(
            "https://unpkg.com/three@0.182.0/examples/jsm/libs/basis/"
          );
          loader.setKTX2Loader(ktx2Loader);

          onProgress?.({ loaded: 25, total: 100, stage: "Parsing GLTF" });
          loader.parse(
            data as ArrayBuffer,
            "",
            (gltf) => {
              onProgress?.({
                loaded: 100,
                total: 100,
                stage: "Optimizing model",
              });

              // Log supported extensions for debugging
              if (gltf.parser?.extensions) {
                console.log("GLTF extensions:", gltf.parser.extensions);
              }

              // Validate extensions
              const validation = validateGLTFExtensions(gltf);

              // Extract lights and cameras
              const lights = extractGLTFLights(gltf);
              const cameras = extractGLTFCameras(gltf);

              // Optimize model like in worker
              optimizeModel(gltf.scene);
              // Return transferable data format
              resolve({
                object: gltf.scene.toJSON(),
                boundingBox: gltf.scene.userData.boundingBox?.toJSON(),
                lights: lights.map((light) => light.toJSON()),
                cameras: cameras.map((camera) => camera.toJSON()),
                validation,
                animations:
                  gltf.animations?.map((anim: THREE.AnimationClip) =>
                    anim.toJSON()
                  ) || [],
                gltf,
              });
            },
            (error) => reject(error)
          );
        });

      case "obj":
        return new Promise((resolve, reject) => {
          onProgress?.({ loaded: 50, total: 100, stage: "Parsing OBJ" });
          const loader = new OBJLoader();
          try {
            const object = loader.parse(data as string);
            onProgress?.({
              loaded: 100,
              total: 100,
              stage: "Optimizing model",
            });
            // Optimize model like in worker
            optimizeModel(object);
            // Return transferable data format
            resolve({
              object: object.toJSON(),
              boundingBox: object.userData.boundingBox?.toJSON(),
            });
          } catch (error) {
            reject(error);
          }
        });

      case "fbx":
        return new Promise((resolve, reject) => {
          onProgress?.({ loaded: 50, total: 100, stage: "Parsing FBX" });
          const loader = new FBXLoader();
          try {
            const object = loader.parse(data as ArrayBuffer, "");
            onProgress?.({
              loaded: 100,
              total: 100,
              stage: "Optimizing model",
            });
            // Optimize model like in worker
            optimizeModel(object);
            // Return transferable data format
            resolve({
              object: object.toJSON(),
              boundingBox: object.userData.boundingBox?.toJSON(),
            });
          } catch (error) {
            reject(error);
          }
        });

      default:
        throw new Error(`Unsupported format: ${type}`);
    }
  }

  // Terminate worker and clean up
  terminate() {
    if (this.worker) {
      this.worker.terminate();
      this.worker = null;
      this.isInitialized = false;
    }

    // Reject any pending tasks
    for (const [id, task] of this.tasks) {
      task.reject(new Error("Worker terminated"));
      this.tasks.delete(id);
    }
  }

  // Get worker status
  getStatus() {
    return {
      isInitialized: this.isInitialized,
      activeTasks: this.tasks.size,
      workerAvailable: !!this.worker,
    };
  }
}

// Model optimization function (same as in worker)
function optimizeModel(model: THREE.Object3D): void {
  model.traverse((child) => {
    if (child instanceof THREE.Mesh) {
      // Optimize geometry
      if (child.geometry) {
        child.geometry.computeBoundingBox();
        child.geometry.computeBoundingSphere();

        // Merge vertices if possible (reduces vertex count)
        if (
          child.geometry.attributes.position &&
          child.geometry.attributes.normal
        ) {
          child.geometry = child.geometry.toNonIndexed();
        }
      }

      // Enable frustum culling
      child.frustumCulled = true;

      // Set render order for transparent objects
      if (
        child.material &&
        "transparent" in child.material &&
        child.material.transparent
      ) {
        child.renderOrder = 1;
      }
    }
  });

  // Compute overall bounding box
  const box = new THREE.Box3().setFromObject(model);
  model.userData.boundingBox = box;
}

// Singleton instance
export const workerManager = new WorkerManager();
