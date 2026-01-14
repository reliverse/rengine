import { invoke } from "@tauri-apps/api/core";
import * as THREE from "three";
import { createMeshFromDffData } from "~/utils/dff-converter";

export interface DffImportStats {
  rw_version?: number;
  frame_count: number;
  geometry_count: number;
  atomic_count: number;
  material_count: number;
}

export interface DffImportResult {
  importedModel: THREE.Object3D;
  initialScale: number;
  stats: DffImportStats;
  raw: unknown;
}

function calculateInitialScale(object: THREE.Object3D): number {
  const box = new THREE.Box3().setFromObject(object);
  const size = box.getSize(new THREE.Vector3());
  const maxDimension = Math.max(size.x, size.y, size.z);
  return maxDimension > 0 ? Math.min(10 / maxDimension, 1) : 1;
}

export async function importDffAsThreeObject(filePath: string) {
  const raw = (await invoke("import_dff_file", { filePath })) as any;
  const importedModel = createMeshFromDffData(raw);

  const initialScale = calculateInitialScale(importedModel);
  importedModel.scale.setScalar(initialScale);

  const stats: DffImportStats = {
    rw_version: raw?.rw_version,
    frame_count: raw?.frames?.length || 0,
    geometry_count: raw?.geometries?.length || 0,
    atomic_count: raw?.atomics?.length || 0,
    material_count:
      raw?.geometries?.reduce(
        (sum: number, g: any) => sum + (g.materials?.length || 0),
        0
      ) || 0,
  };

  return { importedModel, initialScale, stats, raw } satisfies DffImportResult;
}
