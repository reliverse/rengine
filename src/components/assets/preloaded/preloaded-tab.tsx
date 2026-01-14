import { memo, useCallback, useEffect, useState } from "react";
import { ChevronDown, ChevronRight, Search } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { readFile } from "@tauri-apps/plugin-fs";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { ScrollArea } from "~/components/ui/scroll-area";
import { PreloadedModelCard } from "./preloaded-model-card";
import { useSceneStore } from "~/stores/scene-store";
import { importDffAsThreeObject } from "~/utils/dff-import";
import { modelImporter } from "~/utils/model-import";
import * as THREE from "three";

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

interface PreloadedModelWithPaths {
  model: {
    id: number;
    name: string;
    modelFile: string; // Rust serializes with camelCase via serde rename
    radius: number;
  };
  model_file_path: string;
  materials_file_path?: string | null;
  preview_path?: string | null;
}

export const PreloadedTab = memo(function PreloadedTab() {
  const [databases, setDatabases] = useState<string[]>([]);
  const [expandedDatabases, setExpandedDatabases] = useState<Set<string>>(
    new Set()
  );
  const [modelsByDb, setModelsByDb] = useState<
    Map<string, PreloadedModelWithPaths[]>
  >(new Map());
  const [searchQueries, setSearchQueries] = useState<Map<string, string>>(
    new Map()
  );
  const [loadingDatabases, setLoadingDatabases] = useState(false);
  const [loadingModels, setLoadingModels] = useState<Set<string>>(new Set());
  const [selectedModel, setSelectedModel] = useState<{
    dbName: string;
    model: PreloadedModelWithPaths;
  } | null>(null);
  const [hoveredKey, setHoveredKey] = useState<string | null>(null);

  const startImportedPlacement = useSceneStore(
    (state) => state.startImportedPlacement
  );

  // Load databases on mount
  useEffect(() => {
    const loadDatabases = async () => {
      try {
        setLoadingDatabases(true);
        const dbList = (await invoke("list_preloaded_databases")) as string[];
        setDatabases(dbList);
        // Auto-expand first database
        if (dbList.length > 0) {
          setExpandedDatabases(new Set([dbList[0]]));
        }
      } catch (error) {
        console.error("Failed to load preloaded databases:", error);
      } finally {
        setLoadingDatabases(false);
      }
    };

    loadDatabases();
  }, []);

  // Load models when database is expanded
  const loadModelsForDatabase = useCallback(
    async (dbName: string) => {
      if (modelsByDb.has(dbName)) {
        return; // Already loaded
      }

      try {
        setLoadingModels((prev) => new Set(prev).add(dbName));
        const query = searchQueries.get(dbName) || "";
        const models = (await invoke("list_preloaded_models_with_resources", {
          dbName,
          query: query || null,
          offset: 0,
          limit: 1000, // Load first 1000 models
        })) as PreloadedModelWithPaths[];

        setModelsByDb((prev) => {
          const newMap = new Map(prev);
          newMap.set(dbName, models);
          return newMap;
        });
      } catch (error) {
        console.error(`Failed to load models for database ${dbName}:`, error);
      } finally {
        setLoadingModels((prev) => {
          const newSet = new Set(prev);
          newSet.delete(dbName);
          return newSet;
        });
      }
    },
    [modelsByDb, searchQueries]
  );

  // Load models when database is expanded
  useEffect(() => {
    for (const dbName of expandedDatabases) {
      if (!modelsByDb.has(dbName)) {
        loadModelsForDatabase(dbName);
      }
    }
  }, [expandedDatabases, modelsByDb, loadModelsForDatabase]);

  const toggleDatabase = useCallback((dbName: string) => {
    setExpandedDatabases((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(dbName)) {
        newSet.delete(dbName);
      } else {
        newSet.add(dbName);
      }
      return newSet;
    });
  }, []);

  const handleSearchChange = useCallback(
    async (dbName: string, query: string) => {
      setSearchQueries((prev) => {
        const newMap = new Map(prev);
        newMap.set(dbName, query);
        return newMap;
      });

      // Reload models with new query
      try {
        setLoadingModels((prev) => new Set(prev).add(dbName));
        const models = (await invoke("list_preloaded_models_with_resources", {
          dbName,
          query: query || null,
          offset: 0,
          limit: 1000,
        })) as PreloadedModelWithPaths[];

        setModelsByDb((prev) => {
          const newMap = new Map(prev);
          newMap.set(dbName, models);
          return newMap;
        });
      } catch (error) {
        console.error(`Failed to search models for database ${dbName}:`, error);
      } finally {
        setLoadingModels((prev) => {
          const newSet = new Set(prev);
          newSet.delete(dbName);
          return newSet;
        });
      }
    },
    []
  );

  const handleModelClick = useCallback(
    async (dbName: string, model: PreloadedModelWithPaths) => {
      try {
        setSelectedModel({ dbName, model });

        const fileExtension = model.model.modelFile
          .split(".")
          .pop()
          ?.toLowerCase();
        let importedModel: THREE.Object3D;
        let initialScale: number;

        if (fileExtension === "dff") {
          // Load the DFF file using the shared import helper (same logic as Toolbar Import Model)
          const dffResult = await importDffAsThreeObject(model.model_file_path);
          importedModel = dffResult.importedModel;
          initialScale = dffResult.initialScale;
        } else {
          // Load other formats (gltf, glb, obj, fbx) using the standard model importer
          const fileData = await readFile(model.model_file_path);
          const mimeType = getMimeType(model.model.modelFile);
          const blob = new Blob([fileData], { type: mimeType });
          const file = Object.assign(blob, {
            name: model.model.modelFile,
            lastModified: Date.now(),
          }) as File;

          const result = await modelImporter.importFromFile(file);
          if (!(result.success && result.object)) {
            throw new Error(
              `Failed to load ${fileExtension} model: ${result.error}`
            );
          }

          importedModel = (result.object as any).importedModel || result.object;
          // Calculate initial scale to fit the model reasonably in the scene
          const box = new THREE.Box3().setFromObject(importedModel);
          const size = box.getSize(new THREE.Vector3());
          const maxDimension = Math.max(size.x, size.y, size.z);
          initialScale = maxDimension > 0 ? Math.min(10 / maxDimension, 1) : 1;

          // Apply initial scale
          importedModel.scale.setScalar(initialScale);
        }

        // Start placement mode with imported model
        if (startImportedPlacement) {
          startImportedPlacement({
            kind: "preloaded",
            modelid: model.model.id,
            name: model.model.name,
            importedModel,
            initialScale,
          });
        }
      } catch (error) {
        console.error("Failed to load model for placement:", error);
      }
    },
    [startImportedPlacement]
  );

  if (loadingDatabases) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-muted-foreground text-sm">
          Loading databases...
        </div>
      </div>
    );
  }

  if (databases.length === 0) {
    return (
      <div className="flex h-full flex-col items-center justify-center p-4">
        <div className="text-center text-muted-foreground text-sm">
          No preloaded databases found.
          <br />
          Place CSV files in the app data directory.
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col">
      <ScrollArea className="flex-1">
        <div className="space-y-2 p-2">
          {databases.map((dbName) => {
            const isExpanded = expandedDatabases.has(dbName);
            const models = modelsByDb.get(dbName) || [];
            const isLoading = loadingModels.has(dbName);
            const searchQuery = searchQueries.get(dbName) || "";

            return (
              <div className="space-y-1" key={dbName}>
                <Button
                  className="w-full justify-start"
                  onClick={() => toggleDatabase(dbName)}
                  variant="ghost"
                >
                  {isExpanded ? (
                    <ChevronDown className="mr-2 h-4 w-4" />
                  ) : (
                    <ChevronRight className="mr-2 h-4 w-4" />
                  )}
                  <span className="font-medium">{dbName}</span>
                  <span className="ml-auto text-muted-foreground text-xs">
                    {isLoading ? "Loading..." : `${models.length} models`}
                  </span>
                </Button>

                {isExpanded && (
                  <div className="ml-4 space-y-2">
                    <div className="relative">
                      <Search className="absolute top-1/2 left-2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                      <Input
                        className="pl-8"
                        onChange={(e) =>
                          handleSearchChange(dbName, e.target.value)
                        }
                        placeholder="Search models..."
                        value={searchQuery}
                      />
                    </div>

                    {isLoading ? (
                      <div className="py-4 text-center text-muted-foreground text-sm">
                        Loading models...
                      </div>
                    ) : models.length === 0 ? (
                      <div className="py-4 text-center text-muted-foreground text-sm">
                        No models found
                      </div>
                    ) : (
                      <div className="grid grid-cols-3 gap-2">
                        {models.map((model) => (
                          <PreloadedModelCard
                            isSelected={
                              selectedModel?.dbName === dbName &&
                              selectedModel?.model.model.id === model.model.id
                            }
                            key={`${dbName}-${model.model.id}`}
                            model={model}
                            onClick={() => handleModelClick(dbName, model)}
                            onMouseEnter={() =>
                              setHoveredKey(`${dbName}-${model.model.id}`)
                            }
                            onMouseLeave={() => setHoveredKey(null)}
                            show3dPreview={
                              !model.preview_path &&
                              (hoveredKey === `${dbName}-${model.model.id}` ||
                                (selectedModel?.dbName === dbName &&
                                  selectedModel?.model.model.id ===
                                    model.model.id))
                            }
                          />
                        ))}
                      </div>
                    )}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </ScrollArea>
    </div>
  );
});
