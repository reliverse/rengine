import * as THREE from "three";

// Scene graph optimization utilities
export interface SceneGraphStats {
  totalObjects: number;
  visibleObjects: number;
  meshCount: number;
  lightCount: number;
  materialCount: number;
  geometryCount: number;
  textureCount: number;
  totalVertices: number;
  totalTriangles: number;
  depth: number;
  averageDepth: number;
}

export interface UpdateContext {
  camera: THREE.Camera;
  frustum?: THREE.Frustum;
  frameCount: number;
  deltaTime: number;
}

export class SceneGraphOptimizer {
  private scene: THREE.Scene | null = null;
  private readonly updateQueue: Set<THREE.Object3D> = new Set();
  private readonly dirtyObjects: Set<THREE.Object3D> = new Set();
  private readonly transformCache = new Map<THREE.Object3D, THREE.Matrix4>();
  private readonly boundsCache = new Map<THREE.Object3D, THREE.Box3>();
  private lastUpdateFrame = 0;

  setScene(scene: THREE.Scene): void {
    this.scene = scene;
    this.clearCaches();
  }

  // Efficient scene traversal with caching
  traverseOptimized(
    callback: (object: THREE.Object3D, context: UpdateContext) => void,
    context: UpdateContext
  ): void {
    if (!this.scene) return;

    // Only update if this is a new frame
    if (context.frameCount === this.lastUpdateFrame) return;
    this.lastUpdateFrame = context.frameCount;

    // Process dirty objects first
    this.processDirtyObjects();

    // Traverse scene with optimized path
    this.traverseScene(this.scene, callback, context);

    // Update caches for next frame
    this.updateCaches();
  }

  private traverseScene(
    object: THREE.Object3D,
    callback: (object: THREE.Object3D, context: UpdateContext) => void,
    context: UpdateContext,
    depth = 0
  ): void {
    // Skip invisible objects early
    if (!object.visible) return;

    // Check frustum culling if available
    if (context.frustum && !this.isInFrustum(object, context.frustum)) {
      return;
    }

    // Call callback for this object
    callback(object, context);

    // Recursively traverse children
    if (object.children.length > 0) {
      // Use for...of for better readability
      for (const child of object.children) {
        this.traverseScene(child, callback, context, depth + 1);
      }
    }
  }

  private isInFrustum(object: THREE.Object3D, frustum: THREE.Frustum): boolean {
    // Check cached bounds first
    const cachedBounds = this.boundsCache.get(object);
    if (cachedBounds) {
      return frustum.intersectsBox(cachedBounds);
    }

    // Compute bounds if not cached
    const bounds = new THREE.Box3().setFromObject(object);
    this.boundsCache.set(object, bounds);
    return frustum.intersectsBox(bounds);
  }

  // Mark objects as dirty when their transform changes
  markDirty(object: THREE.Object3D): void {
    this.dirtyObjects.add(object);
    this.invalidateChildCaches(object);
  }

  private invalidateChildCaches(object: THREE.Object3D): void {
    // Remove cached data for this object and all children
    this.transformCache.delete(object);
    this.boundsCache.delete(object);

    for (const child of object.children) {
      this.invalidateChildCaches(child);
    }
  }

  private processDirtyObjects(): void {
    for (const object of this.dirtyObjects) {
      // Force matrix update
      object.updateMatrix();
      object.updateMatrixWorld(true);

      // Update cached transform
      const matrix = this.transformCache.get(object);
      if (matrix) {
        matrix.copy(object.matrixWorld);
      } else {
        this.transformCache.set(object, object.matrixWorld.clone());
      }

      // Invalidate bounds cache
      this.boundsCache.delete(object);
    }

    this.dirtyObjects.clear();
  }

  private updateCaches(): void {
    // Limit cache size to prevent memory leaks
    if (this.transformCache.size > 1000) {
      // Clear oldest entries (simple LRU approximation)
      const entries = Array.from(this.transformCache.entries());
      const toRemove = entries.slice(0, entries.length - 500);
      for (const [key] of toRemove) {
        this.transformCache.delete(key);
      }
    }

    if (this.boundsCache.size > 500) {
      const entries = Array.from(this.boundsCache.entries());
      const toRemove = entries.slice(0, entries.length - 250);
      for (const [key] of toRemove) {
        this.boundsCache.delete(key);
      }
    }
  }

  // Get optimized scene statistics
  getSceneStats(): SceneGraphStats {
    if (!this.scene) {
      return this.getEmptyStats();
    }

    const stats = {
      totalObjects: 0,
      visibleObjects: 0,
      meshCount: 0,
      lightCount: 0,
      materialCount: 0,
      geometryCount: 0,
      textureCount: 0,
      totalVertices: 0,
      totalTriangles: 0,
      depth: 0,
      averageDepth: 0,
    };

    const depths: number[] = [];
    const materials = new Set<THREE.Material>();
    const geometries = new Set<THREE.BufferGeometry>();
    const textures = new Set<THREE.Texture>();

    const collectStats = (object: THREE.Object3D, depth: number) => {
      stats.totalObjects++;
      if (object.visible) stats.visibleObjects++;

      if (object instanceof THREE.Mesh) {
        stats.meshCount++;
        if (object.geometry) {
          geometries.add(object.geometry);
          if (object.geometry.attributes.position) {
            stats.totalVertices += object.geometry.attributes.position.count;
          }
          if (object.geometry.index) {
            stats.totalTriangles += object.geometry.index.count / 3;
          } else if (object.geometry.attributes.position) {
            stats.totalVertices +=
              object.geometry.attributes.position.count / 3;
          }
        }
        if (object.material) {
          if (Array.isArray(object.material)) {
            for (const mat of object.material) {
              materials.add(mat);
            }
          } else {
            materials.add(object.material);
          }
        }
      } else if (object instanceof THREE.Light) {
        stats.lightCount++;
      }

      // Collect texture references
      object.traverse((child) => {
        if (child instanceof THREE.Mesh && child.material) {
          const mats = Array.isArray(child.material)
            ? child.material
            : [child.material];
          for (const mat of mats) {
            if ("map" in mat && mat.map) textures.add(mat.map);
            if ("normalMap" in mat && mat.normalMap)
              textures.add(mat.normalMap);
            if ("roughnessMap" in mat && mat.roughnessMap)
              textures.add(mat.roughnessMap);
            if ("metalnessMap" in mat && mat.metalnessMap)
              textures.add(mat.metalnessMap);
            if ("emissiveMap" in mat && mat.emissiveMap)
              textures.add(mat.emissiveMap);
          }
        }
      });

      depths.push(depth);
      stats.depth = Math.max(stats.depth, depth);

      for (const child of object.children) {
        collectStats(child, depth + 1);
      }
    };

    collectStats(this.scene, 0);

    stats.materialCount = materials.size;
    stats.geometryCount = geometries.size;
    stats.textureCount = textures.size;
    stats.averageDepth = depths.reduce((a, b) => a + b, 0) / depths.length;

    return stats;
  }

  private getEmptyStats(): SceneGraphStats {
    return {
      totalObjects: 0,
      visibleObjects: 0,
      meshCount: 0,
      lightCount: 0,
      materialCount: 0,
      geometryCount: 0,
      textureCount: 0,
      totalVertices: 0,
      totalTriangles: 0,
      depth: 0,
      averageDepth: 0,
    };
  }

  // Batch transform updates for better performance
  batchUpdateTransforms(objects: THREE.Object3D[]): void {
    for (const object of objects) {
      this.markDirty(object);
    }
  }

  // Optimize object ordering for better cache coherence
  optimizeRenderOrder(): void {
    if (!this.scene) return;

    const opaqueObjects: THREE.Object3D[] = [];
    const transparentObjects: THREE.Object3D[] = [];

    this.scene.traverse((object) => {
      if (object instanceof THREE.Mesh && object.material) {
        const material = Array.isArray(object.material)
          ? object.material[0]
          : object.material;
        if (material.transparent) {
          transparentObjects.push(object);
        } else {
          opaqueObjects.push(object);
        }
      }
    });

    // Sort opaque objects front-to-back for better early-Z
    opaqueObjects.sort((a, b) => {
      const distA = a.position.distanceToSquared(
        this.scene?.position ?? a.position
      );
      const distB = b.position.distanceToSquared(
        this.scene?.position ?? b.position
      );
      return distA - distB;
    });

    // Sort transparent objects back-to-front
    transparentObjects.sort((a, b) => {
      const distA = a.position.distanceToSquared(
        this.scene?.position ?? a.position
      );
      const distB = b.position.distanceToSquared(
        this.scene?.position ?? b.position
      );
      return distB - distA;
    });

    // Update renderOrder
    for (let index = 0; index < opaqueObjects.length; index++) {
      opaqueObjects[index].renderOrder = index;
    }

    for (let index = 0; index < transparentObjects.length; index++) {
      transparentObjects[index].renderOrder = 1000 + index; // Ensure transparent objects render after opaque
    }
  }

  clearCaches(): void {
    this.transformCache.clear();
    this.boundsCache.clear();
    this.dirtyObjects.clear();
    this.updateQueue.clear();
  }

  // Memory usage estimation
  getMemoryUsage(): { transforms: number; bounds: number; total: number } {
    const transformMemory = this.transformCache.size * 16 * 4; // 16 floats * 4 bytes each
    const boundsMemory = this.boundsCache.size * 6 * 4; // 6 floats * 4 bytes each

    return {
      transforms: transformMemory,
      bounds: boundsMemory,
      total: transformMemory + boundsMemory,
    };
  }
}

// Singleton instance
export const sceneGraphOptimizer = new SceneGraphOptimizer();
