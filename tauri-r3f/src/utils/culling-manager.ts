import * as THREE from "three";
import { withBox3, withMatrix4, withVector3 } from "./memory-pool";

// Culling configuration
export interface CullingConfig {
  frustumCulling: boolean;
  distanceCulling: boolean;
  occlusionCulling: boolean;
  maxDistance: number;
  minDistance: number;
  hysteresis: number; // Prevent flickering at culling boundaries
}

export interface CullableObject {
  object: THREE.Object3D;
  boundingBox?: THREE.Box3;
  boundingSphere?: THREE.Sphere;
  lastVisible: boolean;
  alwaysVisible?: boolean; // For objects that should never be culled
}

export class CullingManager {
  private readonly objects: CullableObject[] = [];
  private camera: THREE.Camera | null = null;
  private readonly frustum = new THREE.Frustum();
  private readonly cameraMatrix = new THREE.Matrix4();

  private config: CullingConfig = {
    frustumCulling: true,
    distanceCulling: true,
    occlusionCulling: false, // More expensive, disabled by default
    maxDistance: 24_000,
    minDistance: 0.1,
    hysteresis: 0.1,
  };

  setCamera(camera: THREE.Camera) {
    this.camera = camera;
  }

  setConfig(config: Partial<CullingConfig>) {
    this.config = { ...this.config, ...config };
  }

  // Add object to culling system
  addObject(object: THREE.Object3D, alwaysVisible = false): void {
    // Compute bounding volumes if not already present
    const boundingBox = object.userData.boundingBox
      ? new THREE.Box3().copy(object.userData.boundingBox)
      : new THREE.Box3().setFromObject(object);

    const boundingSphere = new THREE.Sphere();
    boundingBox.getBoundingSphere(boundingSphere);

    const cullableObject: CullableObject = {
      object,
      boundingBox,
      boundingSphere,
      lastVisible: true,
      alwaysVisible,
    };

    this.objects.push(cullableObject);
  }

  // Remove object from culling system
  removeObject(object: THREE.Object3D): void {
    const index = this.objects.findIndex((co) => co.object === object);
    if (index !== -1) {
      this.objects.splice(index, 1);
    }
  }

  // Update culling for all objects
  updateCulling(): void {
    if (!this.camera) return;

    // Update frustum
    if (this.config.frustumCulling && this.camera) {
      withMatrix4((cameraMatrix) => {
        if (!this.camera) return;
        cameraMatrix.multiplyMatrices(
          this.camera.projectionMatrix,
          this.camera.matrixWorldInverse
        );
        this.frustum.setFromProjectionMatrix(cameraMatrix);
      });
    }

    const cameraPosition = withVector3((pos) => {
      this.camera?.getWorldPosition(pos);
      return pos.clone();
    });

    for (const cullableObject of this.objects) {
      const { object, boundingBox, boundingSphere } = cullableObject;

      let shouldBeVisible = true;

      // Skip culling for always visible objects
      if (cullableObject.alwaysVisible) {
        shouldBeVisible = true;
      } else {
        // Frustum culling
        if (this.config.frustumCulling && boundingBox) {
          shouldBeVisible = withBox3((worldBox) => {
            worldBox.copy(boundingBox);
            object.updateMatrixWorld();
            worldBox.applyMatrix4(object.matrixWorld);
            return this.frustum.intersectsBox(worldBox);
          });
        }

        // Distance culling
        if (shouldBeVisible && this.config.distanceCulling && boundingSphere) {
          shouldBeVisible = withVector3((worldCenter) => {
            // Update bounding sphere to world space
            worldCenter.copy(boundingSphere.center);
            worldCenter.applyMatrix4(object.matrixWorld);

            const distance = cameraPosition.distanceTo(worldCenter);
            const effectiveRadius =
              (boundingSphere.radius * object.scale.length()) / Math.sqrt(3);

            // Add hysteresis to prevent flickering
            const hysteresisOffset = cullableObject.lastVisible
              ? -this.config.hysteresis
              : this.config.hysteresis;

            const cullDistance =
              this.config.maxDistance + effectiveRadius + hysteresisOffset;

            return (
              distance <= cullDistance && distance >= this.config.minDistance
            );
          });
        }

        // Occlusion culling (placeholder - would require more complex implementation)
        if (shouldBeVisible && this.config.occlusionCulling) {
          // This would require raycasting or more advanced occlusion queries
          // For now, we'll skip this as it's quite expensive and complex to implement properly
        }
      }

      // Apply visibility with hysteresis to prevent rapid toggling
      if (shouldBeVisible !== cullableObject.lastVisible) {
        object.visible = shouldBeVisible;
        cullableObject.lastVisible = shouldBeVisible;
      }
    }
  }

  // Force update of bounding volumes (call when object geometry changes)
  updateBoundingVolumes(object: THREE.Object3D): void {
    const cullableObject = this.objects.find((co) => co.object === object);
    if (!cullableObject) return;

    if (cullableObject.boundingBox) {
      cullableObject.boundingBox.setFromObject(object);
    }

    if (cullableObject.boundingSphere && cullableObject.boundingBox) {
      cullableObject.boundingBox.getBoundingSphere(
        cullableObject.boundingSphere
      );
    }
  }

  // Get culling statistics
  getCullingStats(): {
    totalObjects: number;
    visibleObjects: number;
    culledObjects: number;
    config: CullingConfig;
  } {
    const visibleCount = this.objects.filter((co) => co.lastVisible).length;

    return {
      totalObjects: this.objects.length,
      visibleObjects: visibleCount,
      culledObjects: this.objects.length - visibleCount,
      config: this.config,
    };
  }

  // Clear all objects
  clear(): void {
    this.objects.length = 0;
  }

  // Get objects within a certain distance for LOD or other optimizations
  getObjectsInRange(
    center: THREE.Vector3,
    maxDistance: number
  ): CullableObject[] {
    return this.objects.filter((cullableObject) => {
      if (!cullableObject.boundingSphere) return false;

      const boundingSphere = cullableObject.boundingSphere;
      return withVector3((worldCenter) => {
        worldCenter.copy(boundingSphere.center);
        worldCenter.applyMatrix4(cullableObject.object.matrixWorld);

        const distance = center.distanceTo(worldCenter);
        return distance <= maxDistance + boundingSphere.radius;
      });
    });
  }

  // Batch update for performance
  beginFrame(): void {
    // Pre-compute camera matrices and frustum if needed
    if (this.camera && this.config.frustumCulling) {
      this.cameraMatrix.multiplyMatrices(
        this.camera.projectionMatrix,
        this.camera.matrixWorldInverse
      );
      this.frustum.setFromProjectionMatrix(this.cameraMatrix);
    }
  }

  // Get recommended culling settings based on scene complexity
  static getRecommendedSettings(objectCount: number): Partial<CullingConfig> {
    if (objectCount < 100) {
      return {
        frustumCulling: true,
        distanceCulling: false,
        occlusionCulling: false,
        maxDistance: 24_000,
      };
    }
    if (objectCount < 1000) {
      return {
        frustumCulling: true,
        distanceCulling: true,
        occlusionCulling: false,
        maxDistance: 24_000,
        hysteresis: 0.2,
      };
    }
    return {
      frustumCulling: true,
      distanceCulling: true,
      occlusionCulling: false, // Still disabled for performance
      maxDistance: 24_000,
      hysteresis: 0.5,
    };
  }
}

// Singleton instance
export const cullingManager = new CullingManager();
