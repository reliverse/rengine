import * as THREE from "three";
import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";
import {
  getImageMetadata,
  isTauri,
  readImageAsBase64,
  saveTextureToFile,
  showTextureFileDialog,
  showTextureSaveDialog,
} from "~/utils/tauri-texture-api";
import {
  applyTextureChannelProperties,
  applyTextureOptions,
  clearTextureCache,
  createCheckerboardTexture,
  createColorTexture,
  createMetalnessTexture,
  createNormalTexture,
  createRoughnessTexture,
  createTexturePreview,
  getTextureCacheStats,
  loadTexture,
  loadTextureFromFile,
  validateTexture,
} from "~/utils/texture-manager";
import { textureOptimizer } from "~/utils/texture-optimizer";

// Regex constants for performance
const EXTENSION_REGEX = /\.[^/.]+$/;
const PATH_SEPARATOR_REGEX = /[/\\]/;

export type TextureType =
  | "albedo" // Base color/diffuse
  | "normal" // Normal map
  | "roughness" // Roughness/metallic
  | "metallic" // Metallic
  | "ao" // Ambient occlusion
  | "emissive" // Emissive
  | "height" // Height/displacement
  | "opacity" // Alpha/opacity
  | "environment" // Environment/cubemap
  | "custom"; // User-defined

export interface TextureAsset {
  id: string;
  name: string;
  type: TextureType;
  path?: string; // Filesystem path for imported textures
  texture: THREE.Texture | null; // Three.js texture object
  thumbnail?: string; // Base64 thumbnail data URL
  metadata: {
    width: number;
    height: number;
    format: THREE.PixelFormat;
    encoding: THREE.ColorSpace;
    sizeBytes: number;
    createdAt: Date;
    modifiedAt: Date;
    source: "file" | "generated" | "procedural";
    originalFilename?: string;
    tags: string[];
  };
  settings: {
    wrapS: THREE.Wrapping;
    wrapT: THREE.Wrapping;
    magFilter: THREE.MagnificationTextureFilter;
    minFilter: THREE.MinificationTextureFilter;
    anisotropy: number;
    flipY: boolean;
    generateMipmaps: boolean;
  };
  usage: {
    materialCount: number;
    lastUsed?: Date;
    references: string[]; // Material IDs that use this texture
  };
}

export interface TextureLibraryState {
  // Texture assets
  textures: Map<string, TextureAsset>;

  // UI state
  selectedTextureId: string | null;
  libraryVisible: boolean;
  filterType: TextureType | "all";
  filterSearch: string;
  sortBy: "name" | "type" | "size" | "date" | "usage";
  sortOrder: "asc" | "desc";

  // Import/export state
  importingTextures: boolean;
  importProgress: number;
  exportProgress: number;

  // Preview settings
  previewSize: number;
  showThumbnails: boolean;

  // Performance stats
  stats: {
    totalTextures: number;
    totalMemoryUsage: number;
    cacheHitRate: number;
    averageLoadTime: number;
  };
}

export interface TextureLibraryActions {
  // Texture CRUD operations
  addTexture: (textureAsset: Omit<TextureAsset, "id">) => string;
  updateTexture: (id: string, updates: Partial<TextureAsset>) => void;
  removeTexture: (id: string) => void;
  duplicateTexture: (id: string) => string;

  // Texture loading and creation
  loadTextureFromFile: (file: File, type?: TextureType) => Promise<string>;
  loadTextureFromUrl: (
    url: string,
    name: string,
    type?: TextureType
  ) => Promise<string>;
  createProceduralTexture: (
    type: "color" | "checkerboard" | "normal" | "roughness" | "metallic",
    options: any,
    name: string
  ) => string;

  // UI state management
  selectTexture: (id: string | null) => void;
  setLibraryVisible: (visible: boolean) => void;
  setFilterType: (type: TextureType | "all") => void;
  setFilterSearch: (search: string) => void;
  setSortBy: (sortBy: "name" | "type" | "size" | "date" | "usage") => void;
  setSortOrder: (order: "asc" | "desc") => void;

  // Import/Export
  importTextures: (files: File[]) => Promise<string[]>;
  importTexturesFromDialog: () => Promise<string[]>;
  exportTexture: (id: string, format: "png" | "jpg" | "webp") => Promise<void>;
  exportAllTextures: (format: "png" | "jpg" | "webp") => Promise<void>;

  // Texture operations
  updateTextureSettings: (
    id: string,
    settings: Partial<TextureAsset["settings"]>
  ) => void;
  generateThumbnail: (id: string) => Promise<void>;
  validateTextureAsset: (id: string) => {
    isValid: boolean;
    errors: string[];
    warnings: string[];
  };
  optimizeTexture: (id: string) => Promise<void>;

  // Usage tracking
  trackTextureUsage: (textureId: string, materialId: string) => void;
  untrackTextureUsage: (textureId: string, materialId: string) => void;

  // Utility functions
  getTextureById: (id: string) => TextureAsset | undefined;
  getTexturesByType: (type: TextureType) => TextureAsset[];
  getFilteredTextures: () => TextureAsset[];
  getSelectedTexture: () => TextureAsset | undefined;
  searchTextures: (query: string) => TextureAsset[];

  // Bulk operations
  bulkUpdateTextures: (
    updates: { id: string; changes: Partial<TextureAsset> }[]
  ) => void;
  bulkDeleteTextures: (ids: string[]) => void;
  bulkOptimizeTextures: (ids: string[]) => Promise<void>;

  // Performance and cleanup
  updateStats: () => void;
  cleanupUnusedTextures: () => void;
  clearTextureLibrary: () => void;
}

const createDefaultTextureSettings = () => ({
  wrapS: THREE.RepeatWrapping as THREE.Wrapping,
  wrapT: THREE.RepeatWrapping as THREE.Wrapping,
  magFilter: THREE.LinearFilter as THREE.MagnificationTextureFilter,
  minFilter: THREE.LinearMipmapLinearFilter as THREE.MinificationTextureFilter,
  anisotropy: 1,
  flipY: true,
  generateMipmaps: true,
});

const createDefaultTextureMetadata = (
  source: "file" | "generated" | "procedural"
) => ({
  width: 0,
  height: 0,
  format: THREE.RGBAFormat as THREE.PixelFormat,
  encoding: THREE.SRGBColorSpace as THREE.ColorSpace,
  sizeBytes: 0,
  createdAt: new Date(),
  modifiedAt: new Date(),
  source,
  tags: [] as string[],
});

const createDefaultTextureUsage = () => ({
  materialCount: 0,
  references: [] as string[],
});

export const useTextureStore = create<
  TextureLibraryState & TextureLibraryActions
>()(
  subscribeWithSelector((set, get) => ({
    // Initial state
    textures: new Map(),
    selectedTextureId: null,
    libraryVisible: false,
    filterType: "all",
    filterSearch: "",
    sortBy: "name",
    sortOrder: "asc",
    importingTextures: false,
    importProgress: 0,
    exportProgress: 0,
    previewSize: 128,
    showThumbnails: true,
    stats: {
      totalTextures: 0,
      totalMemoryUsage: 0,
      cacheHitRate: 0,
      averageLoadTime: 0,
    },

    // Texture CRUD operations
    addTexture: (textureAsset) => {
      const id = `texture_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      const fullTextureAsset: TextureAsset = {
        ...textureAsset,
        id,
        metadata: {
          ...createDefaultTextureMetadata("generated"),
          ...textureAsset.metadata,
        },
        settings: {
          ...createDefaultTextureSettings(),
          ...textureAsset.settings,
        },
        usage: { ...createDefaultTextureUsage(), ...textureAsset.usage },
      };

      set((state) => ({
        textures: new Map(state.textures).set(id, fullTextureAsset),
      }));

      get().updateStats();
      return id;
    },

    updateTexture: (id, updates) => {
      set((state) => {
        const texture = state.textures.get(id);
        if (!texture) return state;

        const updatedTexture: TextureAsset = {
          ...texture,
          ...updates,
          metadata: {
            ...texture.metadata,
            ...updates.metadata,
            modifiedAt: new Date(),
          },
          settings: { ...texture.settings, ...updates.settings },
          usage: { ...texture.usage, ...updates.usage },
        };

        return {
          textures: new Map(state.textures).set(id, updatedTexture),
        };
      });

      get().updateStats();
    },

    removeTexture: (id) => {
      set((state) => {
        const textures = new Map(state.textures);
        const texture = textures.get(id);

        if (texture?.texture) {
          texture.texture.dispose();
        }

        textures.delete(id);

        return {
          textures,
          selectedTextureId:
            state.selectedTextureId === id ? null : state.selectedTextureId,
        };
      });

      get().updateStats();
    },

    duplicateTexture: (id) => {
      const originalTexture = get().textures.get(id);
      if (!originalTexture) {
        throw new Error(`Texture with id ${id} not found`);
      }

      const newId = `texture_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      const duplicatedTexture: TextureAsset = {
        ...originalTexture,
        id: newId,
        name: `${originalTexture.name} Copy`,
        texture: originalTexture.texture?.clone() || null,
        metadata: {
          ...originalTexture.metadata,
          createdAt: new Date(),
          modifiedAt: new Date(),
        },
        usage: { ...createDefaultTextureUsage() },
      };

      set((state) => ({
        textures: new Map(state.textures).set(newId, duplicatedTexture),
      }));

      get().updateStats();
      return newId;
    },

    // Texture loading and creation
    loadTextureFromFile: async (file, type = "custom") => {
      try {
        const texture = await loadTextureFromFile(file, {
          generateMipmaps: true,
          anisotropy: 1,
        });

        const textureAsset: Omit<TextureAsset, "id"> = {
          name: file.name.replace(EXTENSION_REGEX, ""), // Remove extension
          type,
          texture,
          metadata: {
            ...createDefaultTextureMetadata("file"),
            width: (texture.image as any)?.width || 0,
            height: (texture.image as any)?.height || 0,
            format: texture.format as THREE.PixelFormat,
            encoding: (texture as any).colorSpace || THREE.SRGBColorSpace,
            sizeBytes: file.size,
            originalFilename: file.name,
          },
          settings: createDefaultTextureSettings(),
          usage: createDefaultTextureUsage(),
        };

        const id = get().addTexture(textureAsset);
        await get().generateThumbnail(id);

        return id;
      } catch (error) {
        console.error("Failed to load texture from file:", error);
        throw error;
      }
    },

    loadTextureFromUrl: async (url, name, type = "custom") => {
      try {
        const texture = await loadTexture(url, {
          generateMipmaps: true,
          anisotropy: 1,
        });

        const textureAsset: Omit<TextureAsset, "id"> = {
          name,
          type,
          texture,
          metadata: {
            ...createDefaultTextureMetadata("file"),
            width: (texture.image as any)?.width || 0,
            height: (texture.image as any)?.height || 0,
            format: texture.format as THREE.PixelFormat,
            encoding: (texture as any).colorSpace || THREE.SRGBColorSpace,
            sizeBytes: 0, // Unknown for URL textures
          },
          settings: createDefaultTextureSettings(),
          usage: createDefaultTextureUsage(),
        };

        const id = get().addTexture(textureAsset);
        await get().generateThumbnail(id);

        return id;
      } catch (error) {
        console.error("Failed to load texture from URL:", error);
        throw error;
      }
    },

    createProceduralTexture: (type, options, name) => {
      let texture: THREE.Texture;

      switch (type) {
        case "color":
          texture = createColorTexture(options.color, options.size);
          break;
        case "checkerboard":
          texture = createCheckerboardTexture(
            options.color1,
            options.color2,
            options.size
          );
          break;
        case "normal":
          texture = createNormalTexture(options.size);
          break;
        case "roughness":
          texture = createRoughnessTexture(options.roughness, options.size);
          break;
        case "metallic":
          texture = createMetalnessTexture(options.metalness, options.size);
          break;
        default:
          throw new Error(`Unknown procedural texture type: ${type}`);
      }

      const textureAsset: Omit<TextureAsset, "id"> = {
        name,
        type:
          type === "color" || type === "checkerboard"
            ? "albedo"
            : (type as TextureType),
        texture,
        metadata: {
          ...createDefaultTextureMetadata("procedural"),
          width: options.size,
          height: options.size,
          format: texture.format as THREE.PixelFormat,
          encoding: (texture as any).colorSpace || THREE.SRGBColorSpace,
          sizeBytes: options.size * options.size * 4, // Rough estimate
        },
        settings: createDefaultTextureSettings(),
        usage: createDefaultTextureUsage(),
      };

      const id = get().addTexture(textureAsset);
      get().generateThumbnail(id);

      return id;
    },

    // UI state management
    selectTexture: (id) => {
      set({ selectedTextureId: id });
    },

    setLibraryVisible: (visible) => {
      set({ libraryVisible: visible });
    },

    setFilterType: (type) => {
      set({ filterType: type });
    },

    setFilterSearch: (search) => {
      set({ filterSearch: search });
    },

    setSortBy: (sortBy) => {
      set({ sortBy });
    },

    setSortOrder: (order) => {
      set({ sortOrder: order });
    },

    // Import/Export
    importTexturesFromDialog: async () => {
      try {
        const filePaths = await showTextureFileDialog({ multiple: true });
        if (!filePaths || filePaths.length === 0) return [];

        const textureIds: string[] = [];

        for (const filePath of filePaths) {
          try {
            // Load image as base64 using Tauri
            const dataUrl = await readImageAsBase64(filePath);

            // Create image element from data URL
            const img = new Image();
            await new Promise((resolve, reject) => {
              img.onload = resolve;
              img.onerror = reject;
              img.src = dataUrl;
            });

            // Create texture from image
            const texture = new THREE.Texture(img);
            applyTextureOptions(texture, {
              generateMipmaps: true,
              anisotropy: 1,
            });
            texture.needsUpdate = true;

            // Get metadata
            const metadata = await getImageMetadata(filePath);

            // Create texture asset
            const type = getTextureTypeFromFilename(filePath);
            const textureAsset: Omit<TextureAsset, "id"> = {
              name:
                filePath
                  .split(PATH_SEPARATOR_REGEX)
                  .pop()
                  ?.replace(EXTENSION_REGEX, "") || "texture",
              type,
              path: filePath,
              texture,
              metadata: {
                ...createDefaultTextureMetadata("file"),
                width: img.width,
                height: img.height,
                format:
                  metadata.format === "PNG"
                    ? THREE.RGBAFormat
                    : metadata.format === "JPG" || metadata.format === "JPEG"
                      ? THREE.RGBFormat
                      : THREE.RGBAFormat,
                encoding: THREE.SRGBColorSpace,
                sizeBytes: metadata.sizeBytes,
                originalFilename: filePath.split(PATH_SEPARATOR_REGEX).pop(),
              },
              settings: createDefaultTextureSettings(),
              usage: createDefaultTextureUsage(),
            };

            const id = get().addTexture(textureAsset);
            await get().generateThumbnail(id);
            textureIds.push(id);
          } catch (error) {
            console.error(`Failed to import texture ${filePath}:`, error);
          }
        }

        return textureIds;
      } catch (error) {
        console.error("Failed to import textures from dialog:", error);
        throw error;
      }
    },

    importTextures: async (files) => {
      set({ importingTextures: true, importProgress: 0 });

      const textureIds: string[] = [];
      const totalFiles = files.length;

      for (let i = 0; i < files.length; i++) {
        try {
          const file = files[i];
          const type = getTextureTypeFromFilename(file.name);
          const id = await get().loadTextureFromFile(file, type);
          textureIds.push(id);
        } catch (error) {
          console.error(`Failed to import texture ${files[i].name}:`, error);
        }

        set({ importProgress: ((i + 1) / totalFiles) * 100 });
      }

      set({ importingTextures: false, importProgress: 0 });
      return textureIds;
    },

    exportTexture: async (id, format) => {
      const texture = get().textures.get(id);
      if (!texture?.texture) return;

      // Helper function for web fallback
      const fallbackWebExport = (texture: any, format: string) => {
        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d");
        if (!(ctx && texture.texture.image)) return;

        canvas.width = (texture.texture.image as any).width || 512;
        canvas.height = (texture.texture.image as any).height || 512;

        ctx.drawImage(texture.texture.image as CanvasImageSource, 0, 0);

        const link = document.createElement("a");
        link.download = `${texture.name}.${format}`;
        link.href = canvas.toDataURL(`image/${format}`);
        link.click();
      };

      if (isTauri()) {
        try {
          // Use Tauri native save dialog
          const savePath = await showTextureSaveDialog({
            defaultPath: `${texture.name}.${format}`,
            filters: [
              {
                name: `${format.toUpperCase()} Image`,
                extensions: [format],
              },
            ],
          });

          if (savePath) {
            // Create a temporary canvas to get the image data
            const canvas = document.createElement("canvas");
            const ctx = canvas.getContext("2d");
            if (!(ctx && texture.texture.image)) return;

            canvas.width = (texture.texture.image as any).width || 512;
            canvas.height = (texture.texture.image as any).height || 512;

            ctx.drawImage(texture.texture.image as CanvasImageSource, 0, 0);
            const dataUrl = canvas.toDataURL(`image/${format}`);

            await saveTextureToFile(savePath, dataUrl, format);
          }
        } catch (error) {
          console.error("Failed to export texture with Tauri:", error);
          // Fallback to web download
          fallbackWebExport(texture, format);
        }
      } else {
        // Web fallback
        fallbackWebExport(texture, format);
      }
    },

    exportAllTextures: async (format) => {
      const textures = Array.from(get().textures.values());
      set({ exportProgress: 0 });

      for (let i = 0; i < textures.length; i++) {
        await get().exportTexture(textures[i].id, format);
        set({ exportProgress: ((i + 1) / textures.length) * 100 });
      }

      set({ exportProgress: 0 });
    },

    // Texture operations
    updateTextureSettings: (id, settings) => {
      const texture = get().textures.get(id);
      if (!texture?.texture) return;

      applyTextureChannelProperties(texture.texture, {
        texture: null, // We're only updating settings
        offset: [0, 0],
        repeat: [1, 1],
        rotation: 0,
        ...settings,
      } as any);

      get().updateTexture(id, {
        settings: { ...texture.settings, ...settings },
      });
    },

    generateThumbnail: async (id) => {
      const texture = get().textures.get(id);
      if (!texture?.texture) return;

      try {
        const thumbnail = await createTexturePreview(texture.texture, 128);
        get().updateTexture(id, { thumbnail });
      } catch (error) {
        console.error("Failed to generate thumbnail:", error);
      }
    },

    validateTextureAsset: (id) => {
      const texture = get().textures.get(id);
      if (!texture?.texture) {
        return { isValid: false, errors: ["Texture not found"], warnings: [] };
      }

      return validateTexture(texture.texture);
    },

    optimizeTexture: async (id) => {
      const texture = get().textures.get(id);
      if (!texture?.texture) return;

      // Use the texture optimizer to optimize the texture
      textureOptimizer.optimizeTexture(texture.texture);
      get().updateTexture(id, {
        settings: {
          ...texture.settings,
          anisotropy: texture.texture.anisotropy,
          generateMipmaps: texture.texture.generateMipmaps,
        },
      });
      await Promise.resolve();
    },

    // Usage tracking
    trackTextureUsage: (textureId, materialId) => {
      const texture = get().textures.get(textureId);
      if (!texture) return;

      const references = [...texture.usage.references];
      if (!references.includes(materialId)) {
        references.push(materialId);
      }

      get().updateTexture(textureId, {
        usage: {
          ...texture.usage,
          references,
          materialCount: references.length,
          lastUsed: new Date(),
        },
      });
    },

    untrackTextureUsage: (textureId, materialId) => {
      const texture = get().textures.get(textureId);
      if (!texture) return;

      const references = texture.usage.references.filter(
        (ref) => ref !== materialId
      );

      get().updateTexture(textureId, {
        usage: {
          ...texture.usage,
          references,
          materialCount: references.length,
        },
      });
    },

    // Utility functions
    getTextureById: (id) => {
      return get().textures.get(id);
    },

    getTexturesByType: (type) => {
      return Array.from(get().textures.values()).filter(
        (texture) => texture.type === type
      );
    },

    getFilteredTextures: () => {
      const { textures, filterType, filterSearch, sortBy, sortOrder } = get();
      let filtered = Array.from(textures.values());

      // Filter by type
      if (filterType !== "all") {
        filtered = filtered.filter((texture) => texture.type === filterType);
      }

      // Filter by search
      if (filterSearch) {
        const search = filterSearch.toLowerCase();
        filtered = filtered.filter(
          (texture) =>
            texture.name.toLowerCase().includes(search) ||
            texture.metadata.tags.some((tag) =>
              tag.toLowerCase().includes(search)
            )
        );
      }

      // Sort
      filtered.sort((a, b) => {
        let comparison = 0;

        switch (sortBy) {
          case "name":
            comparison = a.name.localeCompare(b.name);
            break;
          case "type":
            comparison = a.type.localeCompare(b.type);
            break;
          case "size":
            comparison = a.metadata.sizeBytes - b.metadata.sizeBytes;
            break;
          case "date":
            comparison =
              a.metadata.createdAt.getTime() - b.metadata.createdAt.getTime();
            break;
          case "usage":
            comparison = a.usage.materialCount - b.usage.materialCount;
            break;
        }

        return sortOrder === "desc" ? -comparison : comparison;
      });

      return filtered;
    },

    getSelectedTexture: () => {
      const selectedId = get().selectedTextureId;
      return selectedId ? get().textures.get(selectedId) : undefined;
    },

    searchTextures: (query) => {
      const search = query.toLowerCase();
      return Array.from(get().textures.values()).filter(
        (texture) =>
          texture.name.toLowerCase().includes(search) ||
          texture.type.toLowerCase().includes(search) ||
          texture.metadata.tags.some((tag) =>
            tag.toLowerCase().includes(search)
          )
      );
    },

    // Bulk operations
    bulkUpdateTextures: (updates) => {
      set((state) => {
        const newTextures = new Map(state.textures);

        for (const update of updates) {
          const texture = newTextures.get(update.id);
          if (texture) {
            newTextures.set(update.id, {
              ...texture,
              ...update.changes,
              metadata: { ...texture.metadata, ...update.changes.metadata },
              settings: { ...texture.settings, ...update.changes.settings },
              usage: { ...texture.usage, ...update.changes.usage },
            });
          }
        }

        return { textures: newTextures };
      });

      get().updateStats();
    },

    bulkDeleteTextures: (ids) => {
      set((state) => {
        const newTextures = new Map(state.textures);

        for (const id of ids) {
          const texture = newTextures.get(id);
          if (texture?.texture) {
            texture.texture.dispose();
          }
          newTextures.delete(id);
        }

        return {
          textures: newTextures,
          selectedTextureId:
            state.selectedTextureId && ids.includes(state.selectedTextureId)
              ? null
              : state.selectedTextureId,
        };
      });

      get().updateStats();
    },

    bulkOptimizeTextures: async (ids) => {
      for (const id of ids) {
        await get().optimizeTexture(id);
      }
    },

    // Performance and cleanup
    updateStats: () => {
      const textures = Array.from(get().textures.values());
      const cacheStats = getTextureCacheStats();

      set({
        stats: {
          totalTextures: textures.length,
          totalMemoryUsage: cacheStats.memoryUsage,
          cacheHitRate:
            cacheStats.count > 0
              ? (cacheStats.count / textures.length) * 100
              : 0,
          averageLoadTime: 0, // Would need to track load times
        },
      });
    },

    cleanupUnusedTextures: () => {
      set((state) => {
        const newTextures = new Map(state.textures);

        for (const [id, texture] of newTextures) {
          if (texture.usage.materialCount === 0) {
            if (texture.texture) {
              texture.texture.dispose();
            }
            newTextures.delete(id);
          }
        }

        return { textures: newTextures };
      });

      get().updateStats();
    },

    clearTextureLibrary: () => {
      // Dispose of all textures
      for (const texture of get().textures.values()) {
        if (texture.texture) {
          texture.texture.dispose();
        }
      }

      set({
        textures: new Map(),
        selectedTextureId: null,
      });

      clearTextureCache();
      get().updateStats();
    },
  }))
);

// Helper function to determine texture type from filename
function getTextureTypeFromFilename(filename: string): TextureType {
  const name = filename.toLowerCase();

  if (name.includes("normal")) return "normal";
  if (name.includes("roughness") || name.includes("rough")) return "roughness";
  if (name.includes("metallic") || name.includes("metal")) return "metallic";
  if (name.includes("ao") || name.includes("ambient")) return "ao";
  if (name.includes("emissive") || name.includes("emission")) return "emissive";
  if (name.includes("height") || name.includes("displacement")) return "height";
  if (name.includes("opacity") || name.includes("alpha")) return "opacity";
  if (name.includes("env") || name.includes("environment"))
    return "environment";

  return "albedo"; // Default to albedo/diffuse
}

// Selectors for commonly used derived state
export const useSelectedTexture = () => {
  return useTextureStore((state) => state.getSelectedTexture());
};

export const useFilteredTextures = () => {
  return useTextureStore((state) => state.getFilteredTextures());
};

export const useTextureStats = () => {
  return useTextureStore((state) => state.stats);
};

export const useTexturesByType = (type: TextureType) => {
  return useTextureStore((state) => state.getTexturesByType(type));
};
