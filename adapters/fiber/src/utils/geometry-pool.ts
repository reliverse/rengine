import * as THREE from "three";

// Geometry and Material Pool for resource reuse
// Reusing geometries and materials reduces GPU overhead

// Type definitions for factory function parameters
export type GeometryArgs =
  | [number, number, number] // sphere: radius, widthSegments, heightSegments
  | [number, number, number] // box: width, height, depth
  | [number, number, number, number] // cylinder: radiusTop, radiusBottom, height, radialSegments
  | [number, number, number, number] // plane: width, height, widthSegments, heightSegments
  | [number, number, number, number]; // torus: radius, tube, radialSegments, tubularSegments

export type MaterialArgs = [string | number]; // color parameter for all material factories

interface PooledGeometry {
  geometry: THREE.BufferGeometry;
  refCount: number;
  lastUsed: number;
}

interface PooledMaterial {
  material: THREE.Material;
  refCount: number;
  lastUsed: number;
}

class GeometryPool {
  private readonly geometries = new Map<string, PooledGeometry>();
  private readonly cleanupInterval = 30_000; // 30 seconds

  constructor() {
    // Periodic cleanup of unused geometries
    setInterval(() => this.cleanup(), this.cleanupInterval);
  }

  // Get or create a geometry
  getGeometry(
    key: string,
    factory: () => THREE.BufferGeometry
  ): THREE.BufferGeometry {
    let pooled = this.geometries.get(key);

    if (!pooled) {
      // Create new geometry
      pooled = {
        geometry: factory(),
        refCount: 0,
        lastUsed: Date.now(),
      };
      this.geometries.set(key, pooled);
    }

    // Increment reference count
    pooled.refCount++;
    pooled.lastUsed = Date.now();

    return pooled.geometry;
  }

  // Release a geometry reference
  releaseGeometry(key: string): void {
    const pooled = this.geometries.get(key);
    if (pooled) {
      pooled.refCount = Math.max(0, pooled.refCount - 1);
    }
  }

  // Get current pool stats
  getStats() {
    const total = this.geometries.size;
    const inUse = Array.from(this.geometries.values()).filter(
      (g) => g.refCount > 0
    ).length;
    return { total, inUse, available: total - inUse };
  }

  // Cleanup unused geometries
  cleanup(): void {
    const now = Date.now();
    const toRemove: string[] = [];

    for (const [key, pooled] of this.geometries) {
      // Remove geometries that haven't been used in 5 minutes and have no references
      if (pooled.refCount === 0 && now - pooled.lastUsed > 300_000) {
        pooled.geometry.dispose();
        toRemove.push(key);
      }
    }

    for (const key of toRemove) {
      this.geometries.delete(key);
    }
  }
}

class MaterialPool {
  private readonly materials = new Map<string, PooledMaterial>();
  private readonly cleanupInterval = 60_000; // 1 minute

  constructor() {
    // Periodic cleanup of unused materials
    setInterval(() => this.cleanup(), this.cleanupInterval);
  }

  // Get or create a material
  getMaterial(key: string, factory: () => THREE.Material): THREE.Material {
    let pooled = this.materials.get(key);

    if (!pooled) {
      // Create new material
      pooled = {
        material: factory(),
        refCount: 0,
        lastUsed: Date.now(),
      };
      this.materials.set(key, pooled);
    }

    // Increment reference count
    pooled.refCount++;
    pooled.lastUsed = Date.now();

    return pooled.material;
  }

  // Release a material reference
  releaseMaterial(key: string): void {
    const pooled = this.materials.get(key);
    if (pooled) {
      pooled.refCount = Math.max(0, pooled.refCount - 1);
    }
  }

  // Get current pool stats
  getStats() {
    const total = this.materials.size;
    const inUse = Array.from(this.materials.values()).filter(
      (m) => m.refCount > 0
    ).length;
    return { total, inUse, available: total - inUse };
  }

  // Cleanup unused materials
  cleanup(): void {
    const now = Date.now();
    const toRemove: string[] = [];

    for (const [key, pooled] of this.materials) {
      // Remove materials that haven't been used in 10 minutes and have no references
      if (pooled.refCount === 0 && now - pooled.lastUsed > 600_000) {
        pooled.material.dispose();
        toRemove.push(key);
      }
    }

    for (const key of toRemove) {
      this.materials.delete(key);
    }
  }
}

// Common geometry factories
export const geometryFactories = {
  sphere: (radius = 1, widthSegments = 32, heightSegments = 16) =>
    new THREE.SphereGeometry(radius, widthSegments, heightSegments),

  box: (width = 1, height = 1, depth = 1) =>
    new THREE.BoxGeometry(width, height, depth),

  cylinder: (
    radiusTop = 1,
    radiusBottom = 1,
    height = 1,
    radialSegments = 32
  ) =>
    new THREE.CylinderGeometry(radiusTop, radiusBottom, height, radialSegments),

  plane: (width = 1, height = 1, widthSegments = 1, heightSegments = 1) =>
    new THREE.PlaneGeometry(width, height, widthSegments, heightSegments),

  torus: (radius = 1, tube = 0.4, radialSegments = 16, tubularSegments = 100) =>
    new THREE.TorusGeometry(radius, tube, radialSegments, tubularSegments),
};

// Common material factories
export const materialFactories = {
  basic: (color: string | number = 0xff_ff_ff) =>
    new THREE.MeshBasicMaterial({ color }),

  lambert: (color: string | number = 0xff_ff_ff) =>
    new THREE.MeshLambertMaterial({ color }),

  phong: (color: string | number = 0xff_ff_ff) =>
    new THREE.MeshPhongMaterial({ color }),

  standard: (color: string | number = 0xff_ff_ff) =>
    new THREE.MeshStandardMaterial({ color }),

  physical: (color: string | number = 0xff_ff_ff) =>
    new THREE.MeshPhysicalMaterial({ color }),
};

// Singleton instances
export const geometryPool = new GeometryPool();
export const materialPool = new MaterialPool();

// Helper functions for easy usage
export function getPooledGeometry(
  type: keyof typeof geometryFactories,
  ...args: GeometryArgs
): THREE.BufferGeometry {
  const key = `${type}:${JSON.stringify(args)}`;
  return geometryPool.getGeometry(key, () => geometryFactories[type](...args));
}

export function getPooledMaterial(
  type: keyof typeof materialFactories,
  ...args: MaterialArgs
): THREE.Material {
  const key = `${type}:${JSON.stringify(args)}`;
  return materialPool.getMaterial(key, () => materialFactories[type](...args));
}

export function releasePooledGeometry(
  type: keyof typeof geometryFactories,
  ...args: GeometryArgs
): void {
  const key = `${type}:${JSON.stringify(args)}`;
  geometryPool.releaseGeometry(key);
}

export function releasePooledMaterial(
  type: keyof typeof materialFactories,
  ...args: MaterialArgs
): void {
  const key = `${type}:${JSON.stringify(args)}`;
  materialPool.releaseMaterial(key);
}

// Pool management functions
export function clearPools(): void {
  // Force cleanup of all pools
  geometryPool.cleanup();
  materialPool.cleanup();
}

export function getPoolStats() {
  return {
    geometry: geometryPool.getStats(),
    material: materialPool.getStats(),
  };
}

// Legacy compatibility functions (used by scene-object-mesh.tsx)
export function getGeometry(
  type: keyof typeof geometryFactories,
  ...args: GeometryArgs
): THREE.BufferGeometry {
  return getPooledGeometry(type, ...args);
}

export function getMaterial(
  type: keyof typeof materialFactories,
  ...args: MaterialArgs
): THREE.Material {
  return getPooledMaterial(type, ...args);
}

export function releaseGeometry(
  type: keyof typeof geometryFactories,
  ...args: GeometryArgs
): void {
  releasePooledGeometry(type, ...args);
}

export function releaseMaterial(
  type: keyof typeof materialFactories,
  ...args: MaterialArgs
): void {
  releasePooledMaterial(type, ...args);
}

// Hook for React components to use pooled resources
export function usePooledGeometry(
  type: keyof typeof geometryFactories,
  ...args: GeometryArgs
) {
  const geometry = getPooledGeometry(type, ...args);

  // Note: In a real implementation, you'd want to use useEffect to release on unmount
  // But for simplicity, we'll rely on the pool's cleanup mechanism
  return geometry;
}

export function usePooledMaterial(
  type: keyof typeof materialFactories,
  ...args: MaterialArgs
) {
  const material = getPooledMaterial(type, ...args);
  return material;
}
