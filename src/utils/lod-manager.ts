import * as THREE from "three";
import { withVector3 } from "./memory-pool";

// LOD (Level of Detail) configuration
export interface LODLevel {
  distance: number;
  geometry?: THREE.BufferGeometry;
  segments?: number; // For procedural geometry simplification
}

export interface LODConfig {
  levels: LODLevel[];
  hysteresis?: number; // Prevent rapid switching between levels
}

export interface LODData {
  object: THREE.Object3D;
  config: LODConfig;
  currentLevel: number;
  lastDistance: number;
}

export class LODManager {
  private readonly lodObjects = new Map<string, LODData>();

  private camera: THREE.Camera | null = null;

  setCamera(camera: THREE.Camera) {
    this.camera = camera;
  }

  // Add object with LOD configuration
  addLODObject(id: string, object: THREE.Object3D, config: LODConfig) {
    this.lodObjects.set(id, {
      object,
      config,
      currentLevel: 0,
      lastDistance: 0,
    });
  }

  // Remove LOD object
  removeLODObject(id: string) {
    this.lodObjects.delete(id);
  }

  // Update LOD levels based on camera distance
  updateLODLevels() {
    if (!this.camera) return;

    const cameraPosition = withVector3((camPos) => {
      this.camera?.getWorldPosition(camPos);
      return camPos.clone();
    });

    for (const [_id, lodData] of this.lodObjects) {
      const { object, config } = lodData;

      const distance = withVector3((objPos) => {
        object.getWorldPosition(objPos);
        return cameraPosition.distanceTo(objPos);
      });

      // Apply hysteresis to prevent rapid switching
      const hysteresis = config.hysteresis || 0;
      const effectiveDistance =
        distance + hysteresis * (lodData.currentLevel > 0 ? 1 : -1);

      // Find appropriate LOD level
      let newLevel = 0;
      for (let i = 0; i < config.levels.length; i++) {
        if (effectiveDistance >= config.levels[i].distance) {
          newLevel = i;
        } else {
          break;
        }
      }

      // Update geometry if level changed
      if (newLevel !== lodData.currentLevel) {
        this.switchLODLevel(lodData, newLevel);
        lodData.currentLevel = newLevel;
      }

      lodData.lastDistance = distance;
    }
  }

  private switchLODLevel(lodData: LODData, levelIndex: number) {
    const { object, config } = lodData;
    const level = config.levels[levelIndex];

    // For Mesh objects, update geometry
    if (level?.geometry && object instanceof THREE.Mesh) {
      object.geometry = level.geometry;
    }
  }

  // Create LOD geometries for common shapes
  static createLODSphere(radius: number): LODLevel[] {
    return [
      { distance: 0, segments: 32 }, // High detail up close
      { distance: 10, segments: 16 }, // Medium detail
      { distance: 25, segments: 8 }, // Low detail
      { distance: 50, segments: 4 }, // Very low detail
    ].map((level) => ({
      ...level,
      geometry: new THREE.SphereGeometry(
        radius,
        level.segments,
        level.segments
      ),
    }));
  }

  static createLODCube(size: number): LODLevel[] {
    return [
      { distance: 0, segments: 1 }, // Full detail
      { distance: 20, segments: 1 }, // Same detail, just further
      { distance: 50, segments: 1 }, // Could add simplified versions
    ].map((level) => ({
      ...level,
      geometry: new THREE.BoxGeometry(size, size, size),
    }));
  }

  // Get LOD statistics
  getLODStats() {
    const stats = {
      totalObjects: this.lodObjects.size,
      levelDistribution: new Array(10).fill(0), // Up to 10 levels
    };

    for (const lodData of this.lodObjects.values()) {
      stats.levelDistribution[lodData.currentLevel]++;
    }

    return stats;
  }

  // Clear all LOD objects
  clear() {
    this.lodObjects.clear();
  }
}

// Singleton instance
export const lodManager = new LODManager();
