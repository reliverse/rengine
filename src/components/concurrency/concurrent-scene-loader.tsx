import { useGLTF } from "@react-three/drei";
import { useFrame } from "@react-three/fiber";
import { Suspense, useMemo, useState } from "react";
import { useConcurrentAsync } from "~/utils/react-concurrency";

// Concurrent scene loader for expensive 3D operations
// Demonstrates React 18 concurrency with 3D model loading

interface ModelData {
  url: string;
  position: [number, number, number];
  scale?: number;
}

interface ConcurrentSceneLoaderProps {
  models: ModelData[];
  onProgress?: (loaded: number, total: number) => void;
}

/**
 * ConcurrentSceneLoader loads multiple 3D models concurrently
 * Uses React 18 concurrency to prevent blocking the main thread
 */
export function ConcurrentSceneLoader({
  models,
  onProgress,
}: ConcurrentSceneLoaderProps) {
  const [loadedModels, setLoadedModels] = useState<Record<string, unknown>>({});
  const { execute, isPending } = useConcurrentAsync();

  // Load all models concurrently
  useMemo(() => {
    const loadPromises = models.map(async (model) => {
      try {
        const gltf = await execute(() =>
          Promise.resolve(useGLTF.preload(model.url))
        );
        return { url: model.url, gltf };
      } catch (error) {
        console.error(`Failed to load model ${model.url}:`, error);
        return { url: model.url, gltf: null };
      }
    });

    // Load models concurrently and update progress
    Promise.all(loadPromises).then((results) => {
      const newLoadedModels: Record<string, unknown> = {};
      for (const { url, gltf } of results) {
        if (gltf) {
          newLoadedModels[url] = gltf;
        }
      }
      setLoadedModels(newLoadedModels);
      onProgress?.(Object.keys(newLoadedModels).length, models.length);
    });
  }, [models, execute, onProgress]);

  return (
    <group>
      {models.map((model) => (
        <Suspense
          fallback={<LoadingPlaceholder position={model.position} />}
          key={model.url}
        >
          <ModelRenderer
            position={model.position}
            scale={model.scale}
            url={model.url}
          />
        </Suspense>
      ))}

      {isPending && (
        <LoadingIndicator
          loaded={Object.keys(loadedModels).length}
          total={models.length}
        />
      )}
    </group>
  );
}

// Individual model renderer
function ModelRenderer({
  url,
  position,
  scale = 1,
}: {
  url: string;
  position: [number, number, number];
  scale?: number;
}) {
  const { scene } = useGLTF(url);

  return <primitive object={scene} position={position} scale={scale} />;
}

// Loading placeholder for individual models
function LoadingPlaceholder({
  position,
}: {
  position: [number, number, number];
}) {
  const [rotation, setRotation] = useState(0);

  useFrame((state) => {
    setRotation(state.clock.elapsedTime * 2);
  });

  return (
    <mesh position={position}>
      <boxGeometry args={[1, 1, 1]} />
      <meshBasicMaterial color="gray" wireframe />
      <mesh rotation={[0, rotation, 0]}>
        <boxGeometry args={[0.5, 0.5, 0.5]} />
        <meshBasicMaterial color="blue" />
      </mesh>
    </mesh>
  );
}

// Global loading indicator
function LoadingIndicator({
  loaded,
  total,
}: {
  loaded: number;
  total: number;
}) {
  const progress = (loaded / total) * 100;

  return (
    <div className="absolute top-4 right-4 rounded-lg bg-background/90 p-4 backdrop-blur-sm">
      <div className="font-medium text-sm">Loading Models</div>
      <div className="text-muted-foreground text-xs">
        {loaded} / {total} loaded
      </div>
      <div className="mt-2 h-2 w-32 rounded-full bg-muted">
        <div
          className="h-full rounded-full bg-primary transition-all duration-300"
          style={{ width: `${progress}%` }}
        />
      </div>
    </div>
  );
}

// Hook for concurrent expensive computations
export function useConcurrentExpensiveComputation<T>(
  computation: () => T,
  deps: React.DependencyList,
  enabled = true
) {
  const { data, isPending, execute } = useConcurrentAsync<T>();

  useMemo(() => {
    if (enabled) {
      execute(() => Promise.resolve(computation()));
    }
  }, [enabled, execute, computation, ...deps]); // eslint-disable-line react-hooks/exhaustive-deps

  return { result: data, isComputing: isPending };
}

// Concurrent geometry generation for expensive procedural content
export function useConcurrentGeometryGeneration(
  generator: () => THREE.BufferGeometry,
  deps: React.DependencyList
) {
  const { result: geometry, isComputing } = useConcurrentExpensiveComputation(
    generator,
    deps
  );

  return { geometry, isGenerating: isComputing };
}

// Concurrent texture processing
export function useConcurrentTextureProcessing(
  processor: () => Promise<HTMLImageElement>,
  deps: React.DependencyList
) {
  const {
    data: texture,
    isPending,
    execute,
  } = useConcurrentAsync<HTMLImageElement>();

  useMemo(() => {
    execute(processor);
  }, [execute, processor, ...deps]); // eslint-disable-line react-hooks/exhaustive-deps

  return { texture, isProcessing: isPending };
}

// Utility for batching concurrent operations
export function useConcurrentBatchOperations() {
  const { batchUpdate, isPending } = useConcurrentBatch();

  const batchLoadModels = (modelUrls: string[]) => {
    const updates = modelUrls.map((url) => () => useGLTF.preload(url));
    batchUpdate(updates);
  };

  return { batchLoadModels, isPending };
}

// Import required modules
import type * as THREE from "three";
import { useConcurrentBatch } from "~/utils/react-concurrency";
