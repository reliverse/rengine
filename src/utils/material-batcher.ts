import * as THREE from "three";
import {
  matrix4Pool,
  quaternionPool,
  vector3Pool,
  withMatrix4,
  withQuaternion,
  withVector3,
} from "./memory-pool";

// Material batching configuration
export interface MaterialBatch {
  material: THREE.Material;
  objects: THREE.Object3D[];
  geometryGroups: Map<string, THREE.BufferGeometry>;
  instanceCount: number;
  instancedMeshes: THREE.InstancedMesh[];
  lastUpdateTime: number;
  needsUpdate: boolean;
}

export interface MaterialBatchOptions {
  enableInstancing?: boolean;
  maxBatchSize?: number;
  sortByMaterial?: boolean;
  enableMerging?: boolean;
  instancingThreshold?: number; // Minimum objects for instancing
  maxInstancesPerMesh?: number; // Maximum instances per InstancedMesh
  enableFrustumCulling?: boolean;
  enableLODInstancing?: boolean; // Use instancing with LOD
}

export class MaterialBatcher {
  private readonly batches = new Map<string, MaterialBatch>();
  private readonly options: Required<MaterialBatchOptions>;
  private scene: THREE.Scene | null = null;
  private readonly instancedMeshPool = new Map<string, THREE.InstancedMesh[]>();
  private frameCount = 0;

  constructor(options: MaterialBatchOptions = {}) {
    this.options = {
      enableInstancing: true,
      maxBatchSize: 1000,
      sortByMaterial: true,
      enableMerging: false,
      instancingThreshold: 3, // Minimum 3 objects for instancing
      maxInstancesPerMesh: 1000, // Max instances per InstancedMesh
      enableFrustumCulling: true,
      enableLODInstancing: false,
      ...options,
    };
  }

  setScene(scene: THREE.Scene) {
    this.scene = scene;
  }

  // Add object to batching system
  addObject(object: THREE.Object3D): void {
    if (!this.scene) return;

    // Only batch mesh objects
    if (!(object instanceof THREE.Mesh)) return;

    const mesh = object as THREE.Mesh;
    const material = mesh.material;
    const geometry = mesh.geometry;

    // Skip objects with multiple materials for now
    if (Array.isArray(material)) return;

    // Generate material key for batching
    const materialKey = this.getMaterialKey(material);

    if (!this.batches.has(materialKey)) {
      this.batches.set(materialKey, {
        material: material instanceof THREE.Material ? material : material,
        objects: [],
        geometryGroups: new Map(),
        instanceCount: 0,
        instancedMeshes: [],
        lastUpdateTime: 0,
        needsUpdate: true,
      });
    }

    const batch = this.batches.get(materialKey) as MaterialBatch;
    batch.objects.push(object);
    batch.needsUpdate = true;

    // Group by geometry for potential merging
    const geometryKey = this.getGeometryKey(geometry);
    if (!batch.geometryGroups.has(geometryKey)) {
      batch.geometryGroups.set(geometryKey, geometry);
    }
  }

  // Remove object from batching system
  removeObject(object: THREE.Object3D): void {
    for (const [key, batch] of this.batches) {
      const index = batch.objects.indexOf(object);
      if (index !== -1) {
        batch.objects.splice(index, 1);
        batch.instanceCount = Math.max(0, batch.instanceCount - 1);
        batch.needsUpdate = true;

        // Remove batch if empty
        if (batch.objects.length === 0) {
          this.clearBatchInstancedMeshes(batch);
          this.batches.delete(key);
        }
        break;
      }
    }
  }

  // Update batches and create optimized render calls
  updateBatches(): void {
    if (!this.scene) return;

    this.frameCount++;

    // Only update batches that need it (every few frames or when marked)
    for (const batch of this.batches.values()) {
      if (batch.objects.length === 0) continue;

      // Update every 10 frames or when explicitly needed
      if (this.frameCount % 10 === 0 || batch.needsUpdate) {
        this.updateBatch(batch);
        batch.needsUpdate = false;
        batch.lastUpdateTime = this.frameCount;
      }
    }
  }

  private updateBatch(batch: MaterialBatch): void {
    // Clear previous instanced meshes for this batch
    this.clearBatchInstancedMeshes(batch);

    const objectCount = batch.objects.length;

    // Decide on batching strategy based on object count and options
    if (
      this.options.enableInstancing &&
      objectCount >= this.options.instancingThreshold
    ) {
      this.createOptimizedInstancedBatch(batch);
    } else if (this.options.enableMerging && objectCount > 1) {
      this.createMergedBatch(batch);
    } else {
      // Keep individual objects but optimize their materials
      this.optimizeIndividualMaterials(batch);
    }
  }

  private createOptimizedInstancedBatch(batch: MaterialBatch): void {
    if (!this.scene || batch.objects.length < this.options.instancingThreshold)
      return;

    // Group objects by geometry for better instancing
    const geometryGroups = new Map<string, THREE.Mesh[]>();

    for (const object of batch.objects) {
      const mesh = object as THREE.Mesh;
      const geometryKey = this.getGeometryKey(mesh.geometry);

      if (!geometryGroups.has(geometryKey)) {
        geometryGroups.set(geometryKey, []);
      }
      geometryGroups.get(geometryKey)?.push(mesh);
    }

    // Create optimized instanced meshes
    for (const [geometryKey, meshes] of geometryGroups) {
      if (meshes.length < this.options.instancingThreshold) continue;

      this.createInstancedMeshesForGroup(batch, geometryKey, meshes);
    }
  }

  private createInstancedMeshesForGroup(
    batch: MaterialBatch,
    geometryKey: string,
    meshes: THREE.Mesh[]
  ): void {
    const maxInstances = this.options.maxInstancesPerMesh;
    const geometry = meshes[0].geometry;

    // Split into multiple InstancedMesh if needed
    for (let i = 0; i < meshes.length; i += maxInstances) {
      const chunkSize = Math.min(maxInstances, meshes.length - i);
      const meshChunk = meshes.slice(i, i + chunkSize);

      const instancedMesh = this.getOrCreateInstancedMesh(
        geometry,
        batch.material,
        chunkSize
      );

      // Set instance matrices efficiently
      this.setInstanceMatrices(instancedMesh, meshChunk);

      // Configure instanced mesh
      instancedMesh.frustumCulled = this.options.enableFrustumCulling;
      instancedMesh.userData.isBatched = true;
      instancedMesh.userData.batchType = "instanced";
      instancedMesh.userData.originalObjects = meshChunk;
      instancedMesh.userData.geometryKey = geometryKey;

      // Add LOD support if enabled
      if (this.options.enableLODInstancing) {
        this.setupLODInstancing(instancedMesh, meshChunk);
      }

      this.scene?.add(instancedMesh);
      batch.instancedMeshes.push(instancedMesh);

      // Hide original meshes
      for (const mesh of meshChunk) {
        mesh.visible = false;
      }
    }

    batch.instanceCount = meshes.length;
  }

  private getOrCreateInstancedMesh(
    geometry: THREE.BufferGeometry,
    material: THREE.Material,
    count: number
  ): THREE.InstancedMesh {
    const key = `${geometry.uuid}_${material.uuid}_${count}`;

    // Try to reuse from pool
    const pooled = this.instancedMeshPool.get(key);
    if (pooled && pooled.length > 0) {
      const mesh = pooled.at(-1) as THREE.InstancedMesh;
      pooled.length -= 1;
      // Reset instance count
      mesh.count = count;
      return mesh;
    }

    // Create new instanced mesh
    return new THREE.InstancedMesh(geometry, material, count);
  }

  private setInstanceMatrices(
    instancedMesh: THREE.InstancedMesh,
    meshes: THREE.Mesh[]
  ): void {
    const tempMatrix = matrix4Pool.acquire();
    const tempPosition = vector3Pool.acquire();
    const tempQuaternion = quaternionPool.acquire();
    const tempScale = vector3Pool.acquire();

    try {
      meshes.forEach((mesh, index) => {
        mesh.getWorldPosition(tempPosition);
        mesh.getWorldQuaternion(tempQuaternion);
        mesh.getWorldScale(tempScale);

        tempMatrix.compose(tempPosition, tempQuaternion, tempScale);
        instancedMesh.setMatrixAt(index, tempMatrix);
      });

      instancedMesh.instanceMatrix.needsUpdate = true;
    } finally {
      matrix4Pool.release(tempMatrix);
      vector3Pool.release(tempPosition);
      quaternionPool.release(tempQuaternion);
      vector3Pool.release(tempScale);
    }
  }

  private setupLODInstancing(
    instancedMesh: THREE.InstancedMesh,
    meshes: THREE.Mesh[]
  ): void {
    // Add LOD information to instanced mesh for distance-based LOD
    const distances = meshes.map((mesh) => mesh.position.length());
    const avgDistance = distances.reduce((a, b) => a + b, 0) / distances.length;

    instancedMesh.userData.lodDistance = avgDistance;
    instancedMesh.userData.lodLevel = this.calculateLODLevel(avgDistance);
  }

  private calculateLODLevel(distance: number): number {
    // Simple distance-based LOD calculation
    if (distance < 10) return 0; // High detail
    if (distance < 25) return 1; // Medium detail
    if (distance < 50) return 2; // Low detail
    return 3; // Very low detail
  }

  private clearBatchInstancedMeshes(batch: MaterialBatch): void {
    // Return instanced meshes to pool and remove from scene
    for (const instancedMesh of batch.instancedMeshes) {
      this.scene?.remove(instancedMesh);

      // Return to pool for reuse
      const materialUuid = Array.isArray(instancedMesh.material)
        ? instancedMesh.material[0]?.uuid || "unknown"
        : instancedMesh.material.uuid;
      const key = `${instancedMesh.geometry.uuid}_${materialUuid}_${instancedMesh.count}`;
      if (!this.instancedMeshPool.has(key)) {
        this.instancedMeshPool.set(key, []);
      }
      this.instancedMeshPool.get(key)?.push(instancedMesh);
    }

    batch.instancedMeshes.length = 0;

    // Show original objects again
    for (const object of batch.objects) {
      object.visible = true;
    }
  }

  private createMergedBatch(batch: MaterialBatch): void {
    if (!this.scene || batch.objects.length < 2) return;

    // For now, keep objects separate but mark them for potential future merging
    // Full geometry merging is complex and would require:
    // 1. Transforming geometries to world space
    // 2. Merging vertex buffers
    // 3. Handling different materials/textures

    for (const obj of batch.objects) {
      obj.userData.isBatched = true;
      obj.userData.batchType = "individual";
    }
  }

  private optimizeIndividualMaterials(batch: MaterialBatch): void {
    // Optimize material properties for better performance
    for (const obj of batch.objects) {
      if (obj instanceof THREE.Mesh) {
        const material = obj.material;

        // Enable frustum culling
        obj.frustumCulled = true;

        // Optimize material settings
        if (
          material instanceof THREE.MeshStandardMaterial &&
          obj.position.length() > 50
        ) {
          // Reduce shader complexity for distant objects
          material.envMapIntensity = Math.min(material.envMapIntensity, 0.5);
          material.aoMapIntensity = Math.min(material.aoMapIntensity, 0.5);
        }

        obj.userData.isBatched = true;
        obj.userData.batchType = "individual";
      }
    }
  }

  private clearBatchedObjects(): void {
    if (!this.scene) return;

    // Clear all batch instanced meshes
    for (const batch of this.batches.values()) {
      this.clearBatchInstancedMeshes(batch);
    }

    // Clear instanced mesh pool
    for (const meshes of this.instancedMeshPool.values()) {
      for (const mesh of meshes) {
        mesh.dispose();
      }
    }
    this.instancedMeshPool.clear();
  }

  // Generate unique key for material batching
  private getMaterialKey(material: THREE.Material | THREE.Material[]): string {
    if (Array.isArray(material)) {
      // For multi-material objects, create key from all materials
      return material.map((m) => this.getSingleMaterialKey(m)).join("_");
    }
    return this.getSingleMaterialKey(material);
  }

  private getSingleMaterialKey(material: THREE.Material): string {
    const props: string[] = [material.type];

    if (
      material instanceof THREE.MeshBasicMaterial ||
      material instanceof THREE.MeshLambertMaterial ||
      material instanceof THREE.MeshPhongMaterial ||
      material instanceof THREE.MeshStandardMaterial
    ) {
      if (material.color) {
        props.push(material.color.getHexString());
      }

      if ("map" in material && material.map) {
        props.push(material.map.uuid);
      }

      if ("normalMap" in material && material.normalMap) {
        props.push(`normal_${material.normalMap.uuid}`);
      }

      if ("roughnessMap" in material && material.roughnessMap) {
        props.push(`roughness_${material.roughnessMap.uuid}`);
      }

      if ("metalnessMap" in material && material.metalnessMap) {
        props.push(`metalness_${material.metalnessMap.uuid}`);
      }

      props.push(`transparent_${material.transparent}`);
      props.push(`opacity_${material.opacity}`);
    }

    return props.join("_");
  }

  // Generate unique key for geometry batching
  private getGeometryKey(geometry: THREE.BufferGeometry): string {
    // Use geometry UUID for uniqueness
    return geometry.uuid;
  }

  // Get batching statistics
  getStats(): {
    totalBatches: number;
    totalObjects: number;
    instancedObjects: number;
    individualObjects: number;
    averageBatchSize: number;
  } {
    let totalObjects = 0;
    let instancedObjects = 0;

    for (const batch of this.batches.values()) {
      totalObjects += batch.objects.length;
      instancedObjects += batch.instanceCount;
    }

    const individualObjects = totalObjects - instancedObjects;

    return {
      totalBatches: this.batches.size,
      totalObjects,
      instancedObjects,
      individualObjects,
      averageBatchSize: totalObjects / Math.max(this.batches.size, 1),
    };
  }

  // Clear all batches
  clear(): void {
    this.clearBatchedObjects();
    this.batches.clear();
  }

  // Update instance matrices for dynamic objects
  updateInstanceMatrices(object: THREE.Object3D): void {
    if (!this.scene) return;

    // Find which batch contains this object
    for (const batch of this.batches.values()) {
      const index = batch.objects.indexOf(object);
      if (index !== -1 && batch.instancedMeshes.length > 0) {
        // Find which instanced mesh contains this object
        for (const instancedMesh of batch.instancedMeshes) {
          const originalObjects = instancedMesh.userData
            .originalObjects as THREE.Mesh[];
          const instanceIndex = originalObjects.indexOf(object as THREE.Mesh);

          if (instanceIndex !== -1) {
            // Update the instance matrix
            withMatrix4((matrix) => {
              withVector3((position) => {
                withQuaternion((quaternion) => {
                  withVector3((scale) => {
                    object.getWorldPosition(position);
                    object.getWorldQuaternion(quaternion);
                    object.getWorldScale(scale);

                    matrix.compose(position, quaternion, scale);
                    instancedMesh.setMatrixAt(instanceIndex, matrix);
                    instancedMesh.instanceMatrix.needsUpdate = true;
                  });
                });
              });
            });
            break;
          }
        }
        break;
      }
    }
  }

  // Force rebuild of all batches
  rebuild(): void {
    for (const batch of this.batches.values()) {
      batch.needsUpdate = true;
    }
    this.updateBatches();
  }
}

// Singleton instance
export const materialBatcher = new MaterialBatcher();
