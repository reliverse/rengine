import * as THREE from "three";
import type { geometryFactories } from "./geometry-pool";

// Global geometry cache to reuse geometries across the scene
class GeometryCache {
  private readonly cache = new Map<string, THREE.BufferGeometry>();

  get(key: string): THREE.BufferGeometry | undefined {
    return this.cache.get(key);
  }

  set(key: string, geometry: THREE.BufferGeometry): void {
    this.cache.set(key, geometry);
  }

  getOrCreate(
    key: string,
    factory: () => THREE.BufferGeometry
  ): THREE.BufferGeometry {
    let geometry = this.cache.get(key);
    if (!geometry) {
      geometry = factory();
      this.cache.set(key, geometry);
    }
    return geometry;
  }

  // Pre-defined common geometries
  getCube(size = 1): THREE.BufferGeometry {
    return this.getOrCreate(
      `cube_${size}`,
      () => new THREE.BoxGeometry(size, size, size)
    );
  }

  getSphere(
    radius = 0.5,
    widthSegments = 32,
    heightSegments = 32
  ): THREE.BufferGeometry {
    return this.getOrCreate(
      `sphere_${radius}_${widthSegments}_${heightSegments}`,
      () => new THREE.SphereGeometry(radius, widthSegments, heightSegments)
    );
  }

  getPlane(
    width = 10,
    height = 10,
    widthSegments = 1,
    heightSegments = 1
  ): THREE.BufferGeometry {
    return this.getOrCreate(
      `plane_${width}_${height}_${widthSegments}_${heightSegments}`,
      () =>
        new THREE.PlaneGeometry(width, height, widthSegments, heightSegments)
    );
  }

  getCylinder(
    radiusTop = 1,
    radiusBottom = 1,
    height = 1,
    radialSegments = 32
  ): THREE.BufferGeometry {
    return this.getOrCreate(
      `cylinder_${radiusTop}_${radiusBottom}_${height}_${radialSegments}`,
      () =>
        new THREE.CylinderGeometry(
          radiusTop,
          radiusBottom,
          height,
          radialSegments
        )
    );
  }

  dispose(): void {
    for (const geometry of this.cache.values()) {
      geometry.dispose();
    }
    this.cache.clear();
  }
}

// Global material cache to reuse materials across the scene
class MaterialCache {
  private readonly cache = new Map<string, THREE.Material>();

  get(key: string): THREE.Material | undefined {
    return this.cache.get(key);
  }

  set(key: string, material: THREE.Material): void {
    this.cache.set(key, material);
  }

  getOrCreate(key: string, factory: () => THREE.Material): THREE.Material {
    let material = this.cache.get(key);
    if (!material) {
      material = factory();
      this.cache.set(key, material);
    }
    return material;
  }

  // Pre-defined common materials
  getBasicMaterial(
    color: string | number = 0xff_ff_ff
  ): THREE.MeshBasicMaterial {
    const colorStr = typeof color === "string" ? color : color.toString(16);
    return this.getOrCreate(
      `basic_${colorStr}`,
      () => new THREE.MeshBasicMaterial({ color })
    ) as THREE.MeshBasicMaterial;
  }

  getStandardMaterial(
    color: string | number = 0xff_ff_ff,
    options: Partial<THREE.MeshStandardMaterialParameters> = {}
  ): THREE.MeshStandardMaterial {
    const colorStr = typeof color === "string" ? color : color.toString(16);
    const optionsStr = JSON.stringify(options);
    return this.getOrCreate(
      `standard_${colorStr}_${optionsStr}`,
      () => new THREE.MeshStandardMaterial({ color, ...options })
    ) as THREE.MeshStandardMaterial;
  }

  getPhongMaterial(
    color: string | number = 0xff_ff_ff,
    options: Partial<THREE.MeshPhongMaterialParameters> = {}
  ): THREE.MeshPhongMaterial {
    const colorStr = typeof color === "string" ? color : color.toString(16);
    const optionsStr = JSON.stringify(options);
    return this.getOrCreate(
      `phong_${colorStr}_${optionsStr}`,
      () => new THREE.MeshPhongMaterial({ color, ...options })
    ) as THREE.MeshPhongMaterial;
  }

  getNormalMaterial(
    options: Partial<THREE.MeshNormalMaterialParameters> = {}
  ): THREE.MeshNormalMaterial {
    const optionsStr = JSON.stringify(options);
    return this.getOrCreate(
      `normal_${optionsStr}`,
      () => new THREE.MeshNormalMaterial(options)
    ) as THREE.MeshNormalMaterial;
  }

  dispose(): void {
    for (const material of this.cache.values()) {
      material.dispose();
    }
    this.cache.clear();
  }
}

// Global instances
export const geometryCache = new GeometryCache();
export const materialCache = new MaterialCache();

// Utility function to clean up caches when needed
export function disposeCaches(): void {
  geometryCache.dispose();
  materialCache.dispose();
}

// Hook for using cached geometries and materials in React components
export function useCachedGeometry(
  type: "cube" | "sphere" | "plane" | "cylinder",
  ...args: Parameters<
    (typeof geometryFactories)[keyof typeof geometryFactories]
  >
): THREE.BufferGeometry {
  switch (type) {
    case "cube":
      return geometryCache.getCube(...args);
    case "sphere":
      return geometryCache.getSphere(...args);
    case "plane":
      return geometryCache.getPlane(...args);
    case "cylinder":
      return geometryCache.getCylinder(...args);
    default:
      throw new Error(`Unknown geometry type: ${type}`);
  }
}

export function useCachedMaterial(
  type: "basic" | "standard" | "phong" | "normal",
  color?: string | number,
  options: Record<string, unknown> = {}
): THREE.Material {
  switch (type) {
    case "basic":
      return materialCache.getBasicMaterial(color);
    case "standard":
      return materialCache.getStandardMaterial(color, options);
    case "phong":
      return materialCache.getPhongMaterial(color, options);
    case "normal":
      return materialCache.getNormalMaterial(options);
    default:
      throw new Error(`Unknown material type: ${type}`);
  }
}
