import * as THREE from "three";

// Memory pool for frequently allocated objects to reduce GC pressure
export class MemoryPool<T> {
  private readonly pool: T[] = [];
  private readonly factory: () => T;
  private readonly reset?: (obj: T) => void;
  private readonly maxSize: number;

  constructor(factory: () => T, reset?: (obj: T) => void, maxSize = 1000) {
    this.factory = factory;
    this.reset = reset;
    this.maxSize = maxSize;
  }

  acquire(): T {
    const obj = this.pool.pop();
    if (obj) {
      this.reset?.(obj);
      return obj;
    }
    return this.factory();
  }

  release(obj: T): void {
    if (this.pool.length < this.maxSize) {
      this.reset?.(obj);
      this.pool.push(obj);
    }
  }

  clear(): void {
    this.pool.length = 0;
  }

  get size(): number {
    return this.pool.length;
  }
}

// Specialized pools for common Three.js objects
export class Vector3Pool extends MemoryPool<THREE.Vector3> {
  constructor(maxSize = 500) {
    super(
      () => new THREE.Vector3(),
      (v) => v.set(0, 0, 0),
      maxSize
    );
  }
}

export class Matrix4Pool extends MemoryPool<THREE.Matrix4> {
  constructor(maxSize = 200) {
    super(
      () => new THREE.Matrix4(),
      (m) => m.identity(),
      maxSize
    );
  }
}

export class QuaternionPool extends MemoryPool<THREE.Quaternion> {
  constructor(maxSize = 300) {
    super(
      () => new THREE.Quaternion(),
      (q) => q.set(0, 0, 0, 1),
      maxSize
    );
  }
}

export class Box3Pool extends MemoryPool<THREE.Box3> {
  constructor(maxSize = 100) {
    super(
      () => new THREE.Box3(),
      (b) => b.makeEmpty(),
      maxSize
    );
  }
}

export class SpherePool extends MemoryPool<THREE.Sphere> {
  constructor(maxSize = 100) {
    super(
      () => new THREE.Sphere(),
      (s) => s.set(new THREE.Vector3(), 0),
      maxSize
    );
  }
}

export class RaycasterPool extends MemoryPool<THREE.Raycaster> {
  constructor(maxSize = 50) {
    super(
      () => new THREE.Raycaster(),
      (r) => {
        r.set(new THREE.Vector3(), new THREE.Vector3(0, 0, 1));
      },
      maxSize
    );
  }
}

// Geometry pooling for dynamic geometry creation
export class GeometryPool {
  private readonly pools = new Map<string, MemoryPool<THREE.BufferGeometry>>();

  getGeometry(type: string): THREE.BufferGeometry {
    if (!this.pools.has(type)) {
      this.pools.set(
        type,
        new MemoryPool(
          () => this.createGeometry(type),
          (geom) => {
            // Reset geometry attributes
            if (geom.attributes.position) {
              geom.attributes.position.array.fill(0);
              geom.attributes.position.needsUpdate = true;
            }
            if (geom.attributes.normal) {
              geom.attributes.normal.array.fill(0);
              geom.attributes.normal.needsUpdate = true;
            }
            if (geom.attributes.uv) {
              geom.attributes.uv.array.fill(0);
              geom.attributes.uv.needsUpdate = true;
            }
            geom.computeBoundingBox();
            geom.computeBoundingSphere();
          },
          50
        )
      );
    }
    return (this.pools.get(type) as MemoryPool<THREE.BufferGeometry>).acquire();
  }

  releaseGeometry(type: string, geometry: THREE.BufferGeometry): void {
    this.pools.get(type)?.release(geometry);
  }

  private createGeometry(type: string): THREE.BufferGeometry {
    switch (type) {
      case "box":
        return new THREE.BoxGeometry(1, 1, 1);
      case "sphere":
        return new THREE.SphereGeometry(1);
      case "plane":
        return new THREE.PlaneGeometry(1, 1);
      case "cylinder":
        return new THREE.CylinderGeometry(1, 1, 1);
      case "capsule":
        return new THREE.CapsuleGeometry(1, 1);
      default:
        return new THREE.BufferGeometry();
    }
  }

  clear(): void {
    for (const pool of this.pools.values()) {
      pool.clear();
    }
    this.pools.clear();
  }
}

// Material pooling for dynamic material creation
export class MaterialPool {
  private readonly pools = new Map<string, MemoryPool<THREE.Material>>();

  getMaterial(
    type: string,
    options: Record<string, unknown> = {}
  ): THREE.Material {
    const key = `${type}_${JSON.stringify(options)}`;

    if (!this.pools.has(key)) {
      this.pools.set(
        key,
        new MemoryPool(
          () => this.createMaterial(type, options),
          (material) => {
            // Reset material properties to defaults
            material.opacity = 1;
            material.transparent = false;
            material.visible = true;
            material.needsUpdate = true;
          },
          100
        )
      );
    }

    return (this.pools.get(key) as MemoryPool<THREE.Material>).acquire();
  }

  releaseMaterial(
    type: string,
    options: Record<string, unknown>,
    material: THREE.Material
  ): void {
    const key = `${type}_${JSON.stringify(options)}`;
    this.pools.get(key)?.release(material);
  }

  private createMaterial(
    type: string,
    options: Record<string, unknown>
  ): THREE.Material {
    switch (type) {
      case "basic":
        return new THREE.MeshBasicMaterial(options);
      case "lambert":
        return new THREE.MeshLambertMaterial(options);
      case "phong":
        return new THREE.MeshPhongMaterial(options);
      case "standard":
        return new THREE.MeshStandardMaterial(options);
      case "physical":
        return new THREE.MeshPhysicalMaterial(options);
      default:
        return new THREE.MeshBasicMaterial(options);
    }
  }

  clear(): void {
    for (const pool of this.pools.values()) {
      pool.clear();
    }
    this.pools.clear();
  }
}

// Singleton instances for global use
export const vector3Pool = new Vector3Pool();
export const matrix4Pool = new Matrix4Pool();
export const quaternionPool = new QuaternionPool();
export const box3Pool = new Box3Pool();
export const spherePool = new SpherePool();
export const raycasterPool = new RaycasterPool();
export const geometryPool = new GeometryPool();
export const materialPool = new MaterialPool();

// Utility functions for common operations with pooled objects
export const withVector3 = <T>(
  fn: (v: THREE.Vector3) => T,
  v1?: THREE.Vector3 | number,
  v2?: number,
  v3?: number
): T => {
  const vec = vector3Pool.acquire();
  try {
    if (v1 !== undefined) {
      if (typeof v1 === "number") {
        vec.set(v1, v2 || 0, v3 || 0);
      } else {
        vec.copy(v1);
      }
    }
    return fn(vec);
  } finally {
    vector3Pool.release(vec);
  }
};

export const withMatrix4 = <T>(
  fn: (m: THREE.Matrix4) => T,
  matrix?: THREE.Matrix4
): T => {
  const mat = matrix4Pool.acquire();
  try {
    if (matrix) mat.copy(matrix);
    return fn(mat);
  } finally {
    matrix4Pool.release(mat);
  }
};

export const withQuaternion = <T>(
  fn: (q: THREE.Quaternion) => T,
  quaternion?: THREE.Quaternion
): T => {
  const quat = quaternionPool.acquire();
  try {
    if (quaternion) quat.copy(quaternion);
    return fn(quat);
  } finally {
    quaternionPool.release(quat);
  }
};

export const withBox3 = <T>(fn: (b: THREE.Box3) => T, box?: THREE.Box3): T => {
  const b3 = box3Pool.acquire();
  try {
    if (box) b3.copy(box);
    return fn(b3);
  } finally {
    box3Pool.release(b3);
  }
};

export const withRaycaster = <T>(
  fn: (r: THREE.Raycaster) => T,
  origin?: THREE.Vector3,
  direction?: THREE.Vector3
): T => {
  const raycaster = raycasterPool.acquire();
  try {
    if (origin && direction) {
      raycaster.set(origin, direction);
    }
    return fn(raycaster);
  } finally {
    raycasterPool.release(raycaster);
  }
};
