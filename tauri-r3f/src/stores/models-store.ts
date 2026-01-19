import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";

// Regex for removing file extension
const FILE_EXTENSION_REGEX = /\.[^/.]+$/;

export type ModelFormat =
  | "gltf"
  | "glb"
  | "obj"
  | "fbx"
  | "dae"
  | "3ds"
  | "stl"
  | "ply";

export interface ModelAsset {
  id: string;
  name: string;
  filePath: string;
  format: ModelFormat;
  fileSize: number;
  lastModified: number;
  thumbnail?: string;
  metadata: {
    vertices?: number;
    triangles?: number;
    materials?: number;
    animations?: number;
    bones?: number;
    dimensions?: {
      width: number;
      height: number;
      depth: number;
    };
    center?: {
      x: number;
      y: number;
      z: number;
    };
  };
  loaded: boolean;
  loading: boolean;
  error?: string;
  tags: string[];
}

export interface ModelsState {
  // Model library
  models: ModelAsset[];

  // UI state
  libraryVisible: boolean;
  filterFormat: string;
  filterSearch: string;
  sortBy: "name" | "format" | "size" | "date";
  sortOrder: "asc" | "desc";
  showThumbnails: boolean;

  // Selected models
  selectedModelIds: Set<string>;
}

export interface ModelsActions {
  // Model management
  addModel: (model: Omit<ModelAsset, "id">) => string;
  removeModel: (id: string) => void;
  updateModel: (id: string, updates: Partial<ModelAsset>) => void;

  // Bulk operations
  bulkDeleteModels: (ids: string[]) => void;
  clearAllModels: () => void;

  // Import/Export
  importModels: (files: File[]) => Promise<void>;
  exportModel: (id: string) => ModelAsset | null;

  // UI actions
  setLibraryVisible: (visible: boolean) => void;
  setFilterFormat: (format: string) => void;
  setFilterSearch: (search: string) => void;
  setSortBy: (sortBy: "name" | "format" | "size" | "date") => void;
  setSortOrder: (order: "asc" | "desc") => void;
  setShowThumbnails: (show: boolean) => void;

  // Selection
  selectModel: (id: string, multiSelect?: boolean) => void;
  deselectModel: (id: string) => void;
  clearSelection: () => void;

  // Utility functions
  getModelById: (id: string) => ModelAsset | undefined;
  getFilteredModels: () => ModelAsset[];
  getModelsStats: () => ModelsStats;
}

export interface ModelsStats {
  totalModels: number;
  totalFileSize: number;
  formats: Record<ModelFormat, number>;
  loadedModels: number;
  loadingModels: number;
  failedModels: number;
}

export const useModelsStore = create<ModelsState & ModelsActions>()(
  subscribeWithSelector((set, get) => ({
    // Initial state
    models: [],
    libraryVisible: true,
    filterFormat: "",
    filterSearch: "",
    sortBy: "name",
    sortOrder: "asc",
    showThumbnails: true,
    selectedModelIds: new Set(),

    // Model management
    addModel: (modelData) => {
      const id = `model_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      const model: ModelAsset = {
        ...modelData,
        id,
      };

      set((state) => ({
        models: [...state.models, model],
      }));

      return id;
    },

    removeModel: (id) => {
      set((state) => ({
        models: state.models.filter((m) => m.id !== id),
        selectedModelIds: new Set(
          Array.from(state.selectedModelIds).filter(
            (selectedId) => selectedId !== id
          )
        ),
      }));
    },

    updateModel: (id, updates) => {
      set((state) => ({
        models: state.models.map((model) =>
          model.id === id ? { ...model, ...updates } : model
        ),
      }));
    },

    // Bulk operations
    bulkDeleteModels: (ids) => {
      set((state) => ({
        models: state.models.filter((m) => !ids.includes(m.id)),
        selectedModelIds: new Set(
          Array.from(state.selectedModelIds).filter((id) => !ids.includes(id))
        ),
      }));
    },

    clearAllModels: () => {
      set({
        models: [],
        selectedModelIds: new Set(),
      });
    },

    // Import/Export
    importModels: async (files) => {
      const supportedFormats: ModelFormat[] = [
        "gltf",
        "glb",
        "obj",
        "fbx",
        "dae",
        "3ds",
        "stl",
        "ply",
      ];

      for (const file of files) {
        const extension = file.name
          .toLowerCase()
          .split(".")
          .pop() as ModelFormat;
        if (!supportedFormats.includes(extension)) {
          console.warn(`Unsupported model format: ${extension}`);
          continue;
        }

        try {
          // Set loading state
          const tempId = `temp_${Date.now()}_${Math.random()}`;
          const loadingModel: ModelAsset = {
            id: tempId,
            name: file.name.replace(FILE_EXTENSION_REGEX, ""),
            filePath: file.name,
            format: extension,
            fileSize: file.size,
            lastModified: file.lastModified,
            metadata: {},
            loaded: false,
            loading: true,
            tags: [],
          };

          set((state) => ({
            models: [...state.models, loadingModel],
          }));

          // Simulate loading (in real implementation, this would load and parse the model)
          await new Promise((resolve) => setTimeout(resolve, 1000));

          // Create final model
          const modelData: Omit<ModelAsset, "id"> = {
            name: file.name.replace(FILE_EXTENSION_REGEX, ""),
            filePath: file.name,
            format: extension,
            fileSize: file.size,
            lastModified: file.lastModified,
            metadata: {
              // Placeholder metadata - in real implementation this would be parsed from the file
              vertices: Math.floor(Math.random() * 10_000) + 1000,
              triangles: Math.floor(Math.random() * 5000) + 500,
              materials: Math.floor(Math.random() * 5) + 1,
            },
            loaded: true,
            loading: false,
            tags: [],
          };

          // Remove loading model and add final model
          set((state) => ({
            models: [
              ...state.models.filter((m) => m.id !== tempId),
              { ...modelData, id: tempId },
            ],
          }));
        } catch (error) {
          console.error(`Failed to import model ${file.name}:`, error);
          // Update model with error state
          set((state) => ({
            models: state.models.map((model) =>
              model.filePath === file.name
                ? { ...model, loading: false, error: (error as Error).message }
                : model
            ),
          }));
        }
      }
    },

    exportModel: (id) => {
      const model = get().getModelById(id);
      return model || null;
    },

    // UI actions
    setLibraryVisible: (visible) => set({ libraryVisible: visible }),
    setFilterFormat: (format) => set({ filterFormat: format }),
    setFilterSearch: (search) => set({ filterSearch: search }),
    setSortBy: (sortBy) => set({ sortBy }),
    setSortOrder: (order) => set({ sortOrder: order }),
    setShowThumbnails: (show) => set({ showThumbnails: show }),

    // Selection
    selectModel: (id, multiSelect = false) => {
      set((state) => {
        const newSelection = multiSelect
          ? new Set(state.selectedModelIds)
          : new Set<string>();

        if (newSelection.has(id)) {
          newSelection.delete(id);
        } else {
          newSelection.add(id);
        }

        return { selectedModelIds: newSelection };
      });
    },

    deselectModel: (id) => {
      set((state) => ({
        selectedModelIds: new Set(
          Array.from(state.selectedModelIds).filter(
            (selectedId) => selectedId !== id
          )
        ),
      }));
    },

    clearSelection: () => {
      set({ selectedModelIds: new Set() });
    },

    // Utility functions
    getModelById: (id) => {
      return get().models.find((model) => model.id === id);
    },

    getFilteredModels: () => {
      const { models, filterFormat, filterSearch, sortBy, sortOrder } = get();

      const filtered = models.filter((model) => {
        const matchesFormat = !filterFormat || model.format === filterFormat;
        const matchesSearch =
          !filterSearch ||
          model.name.toLowerCase().includes(filterSearch.toLowerCase()) ||
          model.format.toLowerCase().includes(filterSearch.toLowerCase());

        return matchesFormat && matchesSearch;
      });

      // Sort
      filtered.sort((a, b) => {
        let comparison = 0;

        switch (sortBy) {
          case "name":
            comparison = a.name.localeCompare(b.name);
            break;
          case "format":
            comparison = a.format.localeCompare(b.format);
            break;
          case "size":
            comparison = a.fileSize - b.fileSize;
            break;
          case "date":
            comparison = a.lastModified - b.lastModified;
            break;
        }

        return sortOrder === "asc" ? comparison : -comparison;
      });

      return filtered;
    },

    getModelsStats: (): ModelsStats => {
      const models = get().models;
      const formats: Record<ModelFormat, number> = {
        gltf: 0,
        glb: 0,
        obj: 0,
        fbx: 0,
        dae: 0,
        "3ds": 0,
        stl: 0,
        ply: 0,
      };

      for (const model of models) {
        formats[model.format]++;
      }

      return {
        totalModels: models.length,
        totalFileSize: models.reduce((sum, model) => sum + model.fileSize, 0),
        formats,
        loadedModels: models.filter((m) => m.loaded).length,
        loadingModels: models.filter((m) => m.loading).length,
        failedModels: models.filter((m) => m.error).length,
      };
    },
  }))
);

// Convenience hooks
export const useFilteredModels = () =>
  useModelsStore((state) => state.getFilteredModels());
export const useModelsStats = () =>
  useModelsStore((state) => state.getModelsStats());
