// Web Worker for model parsing to prevent UI blocking
// This runs in a separate thread to keep the main UI responsive

import * as THREE from "three";
import { FBXLoader } from "three/examples/jsm/loaders/FBXLoader.js";
// Import Three.js loaders (these need to be available in the worker context)
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";

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

// GLTF parsing function
function parseGLTF(
  arrayBuffer: ArrayBuffer,
  onProgress?: (progress: number) => void
): Promise<THREE.Object3D> {
  return new Promise((resolve, reject) => {
    const loader = new GLTFLoader();
    onProgress?.(25);

    loader.parse(
      arrayBuffer,
      "",
      (gltf) => {
        onProgress?.(75);
        optimizeModel(gltf.scene);
        onProgress?.(100);
        resolve(gltf.scene);
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
): Promise<THREE.Object3D> {
  return new Promise((resolve, reject) => {
    onProgress?.(50);
    const loader = new OBJLoader();

    try {
      const object = loader.parse(text);
      onProgress?.(75);
      optimizeModel(object);
      onProgress?.(100);
      resolve(object);
    } catch (error) {
      reject(new Error(`OBJ parsing error: ${error}`));
    }
  });
}

// FBX parsing function
function parseFBX(
  arrayBuffer: ArrayBuffer,
  onProgress?: (progress: number) => void
): Promise<THREE.Object3D> {
  return new Promise((resolve, reject) => {
    onProgress?.(50);
    const loader = new FBXLoader();

    try {
      const object = loader.parse(arrayBuffer, "");
      onProgress?.(75);
      optimizeModel(object);
      onProgress?.(100);
      resolve(object);
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
    const transferableData = {
      object: (result as THREE.Object3D).toJSON(),
      boundingBox: (result as THREE.Object3D).userData.boundingBox?.toJSON(),
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
