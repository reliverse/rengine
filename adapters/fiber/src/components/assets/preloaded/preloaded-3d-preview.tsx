import { Canvas } from "@react-three/fiber";
import { memo, useEffect, useMemo, useState } from "react";
import { readFile } from "@tauri-apps/plugin-fs";
import * as THREE from "three";
import { invoke } from "@tauri-apps/api/core";
import { createMeshFromDffData } from "~/utils/dff-converter";
import { modelImporter } from "~/utils/model-import";

interface CachedEntry {
  object: THREE.Object3D;
  loadedAt: number;
}

// Small in-memory cache: enough to keep the UI snappy while browsing.
const MODEL_CACHE = new Map<string, CachedEntry>();
const CACHE_TTL_MS = 10 * 60 * 1000;
const MAX_CACHE_ENTRIES = 64;

// Simple concurrency limiter for DFF loading (avoid spiking CPU with many hovers).
let inFlight = 0;
const MAX_IN_FLIGHT = 2;
const queue: Array<() => void> = [];

function enqueue<T>(task: () => Promise<T>): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    const run = () => {
      inFlight++;
      task()
        .then(resolve, reject)
        .finally(() => {
          inFlight--;
          const next = queue.shift();
          if (next) next();
        });
    };

    if (inFlight < MAX_IN_FLIGHT) run();
    else queue.push(run);
  });
}

function cleanCache() {
  const now = Date.now();
  for (const [key, entry] of MODEL_CACHE.entries()) {
    if (now - entry.loadedAt > CACHE_TTL_MS) MODEL_CACHE.delete(key);
  }
  if (MODEL_CACHE.size <= MAX_CACHE_ENTRIES) return;

  const sorted = [...MODEL_CACHE.entries()].sort(
    (a, b) => a[1].loadedAt - b[1].loadedAt
  );
  for (const [key] of sorted.slice(0, MODEL_CACHE.size - MAX_CACHE_ENTRIES)) {
    MODEL_CACHE.delete(key);
  }
}

// Helper function to get MIME type from filename
function getMimeType(filename: string): string {
  const extension = filename.split(".").pop()?.toLowerCase();
  switch (extension) {
    case "gltf":
      return "model/gltf+json";
    case "glb":
      return "model/gltf-binary";
    case "obj":
      return "model/obj";
    case "fbx":
      return "application/octet-stream"; // FBX doesn't have a standard MIME type
    case "dae":
      return "model/vnd.collada+xml";
    case "3ds":
      return "application/x-3ds";
    case "stl":
      return "model/stl";
    case "ply":
      return "application/octet-stream";
    default:
      return "application/octet-stream";
  }
}

async function loadModelAsObject3D(modelPath: string): Promise<THREE.Object3D> {
  cleanCache();

  const cached = MODEL_CACHE.get(modelPath);
  if (cached) return cached.object.clone();

  const fileExtension = modelPath.split(".").pop()?.toLowerCase();
  let obj: THREE.Object3D;

  if (fileExtension === "dff") {
    // Load DFF file using RenderWare pipeline
    const dffResult = (await enqueue(() =>
      invoke("import_dff_file", { filePath: modelPath })
    )) as any;

    obj = createMeshFromDffData(dffResult);
  } else {
    // Load other formats using standard model importer
    const fileData = await readFile(modelPath);
    const mimeType = getMimeType(modelPath);
    const blob = new Blob([fileData], { type: mimeType });
    const file = Object.assign(blob, {
      name: modelPath.split("/").pop() || "model",
      lastModified: Date.now(),
    }) as File;

    const result = await modelImporter.importFromFile(file);
    if (!(result.success && result.object)) {
      throw new Error(`Failed to load ${fileExtension} model: ${result.error}`);
    }

    obj = (result.object as any).importedModel || result.object;
  }

  // Fit-ish: keep it centered and not enormous in preview.
  const box = new THREE.Box3().setFromObject(obj);
  const center = box.getCenter(new THREE.Vector3());
  const size = box.getSize(new THREE.Vector3());
  obj.position.sub(center);
  const maxDim = Math.max(size.x, size.y, size.z);
  const s = maxDim > 0 ? 1 / maxDim : 1;
  obj.scale.setScalar(s);

  MODEL_CACHE.set(modelPath, { object: obj, loadedAt: Date.now() });
  return obj.clone();
}

export const Preloaded3DPreview = memo(function Preloaded3DPreview({
  modelPath,
}: {
  modelPath: string;
}) {
  const [object, setObject] = useState<THREE.Object3D | null>(null);

  useEffect(() => {
    let cancelled = false;
    setObject(null);

    loadModelAsObject3D(modelPath)
      .then((obj) => {
        if (!cancelled) setObject(obj);
      })
      .catch(() => {
        if (!cancelled) setObject(null);
      });

    return () => {
      cancelled = true;
    };
  }, [modelPath]);

  const sceneObject = useMemo(() => {
    if (!object) return null;
    const clone = object.clone();
    clone.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        const materials = Array.isArray(child.material)
          ? child.material
          : child.material
            ? [child.material]
            : [];
        for (const mat of materials) {
          if (mat instanceof THREE.MeshStandardMaterial) {
            mat.transparent = true;
            mat.opacity = 0.95;
          }
        }
      }
    });
    return clone;
  }, [object]);

  return (
    <Canvas
      camera={{ position: [1.8, 1.2, 1.8], fov: 35 }}
      dpr={1}
      frameloop="demand"
      gl={{ antialias: true, alpha: true }}
      style={{ width: "100%", height: "100%" }}
    >
      <ambientLight intensity={0.8} />
      <directionalLight intensity={1.2} position={[2, 3, 2]} />
      {sceneObject ? <primitive object={sceneObject} /> : null}
    </Canvas>
  );
});
