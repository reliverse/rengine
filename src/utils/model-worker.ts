// Web Worker for model parsing to prevent UI blocking
// This runs in a separate thread to keep the main UI responsive

import * as THREE from "three";
// GLTF Extensions
import { DRACOLoader } from "three/examples/jsm/loaders/DRACOLoader.js";
import { FBXLoader } from "three/examples/jsm/loaders/FBXLoader.js";
// Import Three.js loaders (these need to be available in the worker context)
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { KTX2Loader } from "three/examples/jsm/loaders/KTX2Loader.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import {
  applyGLTFMaterialExtensions,
  extractGLTFCameras,
  extractGLTFLights,
  validateGLTFExtensions,
} from "./gltf-extensions.js";

interface WorkerMessage {
  type: "parse_gltf" | "parse_obj" | "parse_fbx" | "optimize_model";
  data: unknown;
  id: string;
}

interface WorkerResponse {
  type: "success" | "error" | "progress";
  data?: unknown;
  error?: string;
  progress?: number;
  id: string;
}

// Model optimization function (same as main thread)
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
          child.geometry.attributes.normal &&
          child.geometry.index !== null
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

// GLTF parsing function with extensions support
function parseGLTF(
  arrayBuffer: ArrayBuffer,
  onProgress?: (progress: number) => void
): Promise<{
  scene: THREE.Object3D;
  lights: THREE.Light[];
  cameras: THREE.Camera[];
  validation: any;
  animations: THREE.AnimationClip[];
  gltf: any;
}> {
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

    onProgress?.(25);

    loader.parse(
      arrayBuffer,
      "",
      (gltf) => {
        onProgress?.(50);

        // Validate extensions
        const validation = validateGLTFExtensions(gltf);

        // Apply extension-specific material properties
        gltf.scene.traverse((child: any) => {
          if (child.isMesh && child.material) {
            const materials = Array.isArray(child.material)
              ? child.material
              : [child.material];
            materials.forEach((material: any, index: number) => {
              if (material.userData?.gltfExtensions) {
                const newMaterial = applyGLTFMaterialExtensions(
                  material,
                  material.userData,
                  material.userData.gltfExtensions
                );
                if (Array.isArray(child.material)) {
                  child.material[index] = newMaterial;
                } else {
                  child.material = newMaterial;
                }
              }
            });
          }
        });

        // Extract lights and cameras
        const lights = extractGLTFLights(gltf);
        const cameras = extractGLTFCameras(gltf);

        onProgress?.(75);
        optimizeModel(gltf.scene);
        onProgress?.(100);

        resolve({
          scene: gltf.scene,
          lights,
          cameras,
          validation,
          animations: gltf.animations || [],
          gltf,
        });
      },
      (error) => {
        reject(new Error(`GLTF parse error: ${error}`));
      }
    );
  });
}

// OBJ parsing function
function parseOBJ(
  text: string,
  onProgress?: (progress: number) => void
): Promise<{
  scene: THREE.Object3D;
  lights: THREE.Light[];
  cameras: THREE.Camera[];
  validation: any;
  animations: THREE.AnimationClip[];
  gltf: any;
}> {
  return new Promise((resolve, reject) => {
    onProgress?.(50);
    const loader = new OBJLoader();

    try {
      const object = loader.parse(text);
      onProgress?.(75);
      optimizeModel(object);
      onProgress?.(100);
      resolve({
        scene: object,
        lights: [],
        cameras: [],
        validation: null,
        animations: [],
        gltf: null,
      });
    } catch (error) {
      reject(new Error(`OBJ parsing error: ${error}`));
    }
  });
}

// FBX parsing function
function parseFBX(
  arrayBuffer: ArrayBuffer,
  onProgress?: (progress: number) => void
): Promise<{
  scene: THREE.Object3D;
  lights: THREE.Light[];
  cameras: THREE.Camera[];
  validation: any;
  animations: THREE.AnimationClip[];
  gltf: any;
}> {
  return new Promise((resolve, reject) => {
    onProgress?.(50);
    const loader = new FBXLoader();

    try {
      const object = loader.parse(arrayBuffer, "");
      onProgress?.(75);
      optimizeModel(object);
      onProgress?.(100);
      resolve({
        scene: object,
        lights: [],
        cameras: [],
        validation: null,
        animations: [],
        gltf: null,
      });
    } catch (error) {
      reject(new Error(`FBX parsing error: ${error}`));
    }
  });
}

// Message handler
self.onmessage = async (event: MessageEvent<WorkerMessage>) => {
  const { type, data, id } = event.data;

  try {
    let result: unknown;

    switch (type) {
      case "parse_gltf":
        result = await parseGLTF(
          (data as { arrayBuffer: ArrayBuffer }).arrayBuffer,
          (progress) => {
            self.postMessage({
              type: "progress",
              progress,
              id,
            } as WorkerResponse);
          }
        );
        break;

      case "parse_obj":
        result = await parseOBJ((data as { text: string }).text, (progress) => {
          self.postMessage({
            type: "progress",
            progress,
            id,
          } as WorkerResponse);
        });
        break;

      case "parse_fbx":
        result = await parseFBX(
          (data as { arrayBuffer: ArrayBuffer }).arrayBuffer,
          (progress) => {
            self.postMessage({
              type: "progress",
              progress,
              id,
            } as WorkerResponse);
          }
        );
        break;

      default:
        throw new Error(`Unknown message type: ${type}`);
    }

    // Convert the Three.js object to transferable data
    const resultData = result as {
      scene: THREE.Object3D;
      lights: THREE.Light[];
      cameras: THREE.Camera[];
      validation: any;
      animations: THREE.AnimationClip[];
      gltf: any;
    };

    // Safely serialize GLTF data, excluding non-transferable objects
    const gltfData = resultData.gltf
      ? {
          animations:
            resultData.gltf.animations?.map((anim: any) => anim.toJSON()) || [],
          scenes: resultData.gltf.scenes?.length || 0,
          scene: resultData.gltf.scene ? 0 : null,
          // Only include basic metadata, exclude complex objects
          asset: resultData.gltf.asset
            ? {
                version: resultData.gltf.asset.version,
                generator: resultData.gltf.asset.generator,
              }
            : undefined,
        }
      : null;

    // Safely serialize lights and cameras, handling potential serialization issues
    let lightsData: any[] = [];
    let camerasData: any[] = [];

    try {
      lightsData = resultData.lights.map((light) => light.toJSON());
    } catch (error) {
      console.warn("Failed to serialize lights:", error);
      lightsData = [];
    }

    try {
      camerasData = resultData.cameras.map((camera) => camera.toJSON());
    } catch (error) {
      console.warn("Failed to serialize cameras:", error);
      camerasData = [];
    }

    const transferableData = {
      object: resultData.scene.toJSON(),
      boundingBox: resultData.scene.userData.boundingBox?.toJSON(),
      lights: lightsData,
      cameras: camerasData,
      validation: resultData.validation,
      animations: resultData.animations.map((anim) => anim.toJSON()),
      gltf: gltfData,
    };

    self.postMessage({
      type: "success",
      data: transferableData,
      id,
    } as WorkerResponse);
  } catch (error) {
    self.postMessage({
      type: "error",
      error: error instanceof Error ? error.message : "Unknown error",
      id,
    } as WorkerResponse);
  }
};
