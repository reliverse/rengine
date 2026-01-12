import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";
import type { ColFile } from "~/types/col";
import type { DffModel } from "~/types/dff";
import type { IdeDocument } from "~/types/ide";
import type { ImgArchive } from "~/types/img";
import type { RwAnalysis } from "~/types/rw-analyzer";
import type { TxdArchive } from "~/types/txd";

export type RenderWareAssetType =
  | "img"
  | "txd"
  | "dff"
  | "col"
  | "ide"
  | "rw-analysis";

export interface RenderWareAsset {
  id: string;
  name: string;
  assetType: RenderWareAssetType;
  filePath: string;
  fileSize: number;
  lastModified: number;
  renderWareVersion?: string;
  gameInfo?: string;
  thumbnail?: string;
  metadata: Record<string, any>;
  loaded: boolean;
  loading: boolean;
  error?: string;
}

export interface AssetRegistry {
  // Asset collections
  imgArchives: Map<string, ImgArchive>;
  txdArchives: Map<string, TxdArchive>;
  dffModels: Map<string, DffModel>;
  colFiles: Map<string, ColFile>;
  ideDocuments: Map<string, IdeDocument>;
  rwAnalyses: Map<string, RwAnalysis>;

  // Asset registry (for quick lookup and management)
  assets: Map<string, RenderWareAsset>;

  // Actions
  registerAsset: (asset: RenderWareAsset) => void;
  unregisterAsset: (assetId: string) => void;
  updateAsset: (assetId: string, updates: Partial<RenderWareAsset>) => void;

  // IMG Archive management
  addImgArchive: (archive: ImgArchive) => void;
  removeImgArchive: (archivePath: string) => void;
  getImgArchive: (archivePath: string) => ImgArchive | undefined;

  // TXD Archive management
  addTxdArchive: (archive: TxdArchive) => void;
  removeTxdArchive: (archivePath: string) => void;
  getTxdArchive: (archivePath: string) => TxdArchive | undefined;

  // DFF Model management
  addDffModel: (model: DffModel) => void;
  removeDffModel: (modelPath: string) => void;
  getDffModel: (modelPath: string) => DffModel | undefined;

  // COL File management
  addColFile: (colFile: ColFile) => void;
  removeColFile: (filePath: string) => void;
  getColFile: (filePath: string) => ColFile | undefined;

  // IDE Document management
  addIdeDocument: (document: IdeDocument) => void;
  removeIdeDocument: (filePath: string) => void;
  getIdeDocument: (filePath: string) => IdeDocument | undefined;

  // RW Analysis management
  addRwAnalysis: (analysis: RwAnalysis) => void;
  removeRwAnalysis: (filePath: string) => void;
  getRwAnalysis: (filePath: string) => RwAnalysis | undefined;

  // Utility functions
  getAssetsByType: (assetType: RenderWareAssetType) => RenderWareAsset[];
  getAssetByPath: (filePath: string) => RenderWareAsset | undefined;
  clearAllAssets: () => void;
  getStatistics: () => AssetStatistics;
}

export interface AssetStatistics {
  totalAssets: number;
  imgArchives: number;
  txdArchives: number;
  dffModels: number;
  colFiles: number;
  ideDocuments: number;
  rwAnalyses: number;
  totalFileSize: number;
  loadedAssets: number;
  loadingAssets: number;
  failedAssets: number;
}

export const useAssetStore = create<AssetRegistry>()(
  subscribeWithSelector((set, get) => ({
    // Initial state
    imgArchives: new Map(),
    txdArchives: new Map(),
    dffModels: new Map(),
    colFiles: new Map(),
    ideDocuments: new Map(),
    rwAnalyses: new Map(),
    assets: new Map(),

    // Asset registry actions
    registerAsset: (asset: RenderWareAsset) => {
      set((state) => {
        const newAssets = new Map(state.assets);
        newAssets.set(asset.id, asset);
        return { assets: newAssets };
      });
    },

    unregisterAsset: (assetId: string) => {
      set((state) => {
        const newAssets = new Map(state.assets);
        newAssets.delete(assetId);
        return { assets: newAssets };
      });
    },

    updateAsset: (assetId: string, updates: Partial<RenderWareAsset>) => {
      set((state) => {
        const newAssets = new Map(state.assets);
        const existingAsset = newAssets.get(assetId);
        if (existingAsset) {
          newAssets.set(assetId, { ...existingAsset, ...updates });
        }
        return { assets: newAssets };
      });
    },

    // IMG Archive management
    addImgArchive: (archive: ImgArchive) => {
      set((state) => {
        const newArchives = new Map(state.imgArchives);
        newArchives.set(archive.file_path, archive);

        // Register in asset registry
        const asset: RenderWareAsset = {
          id: `img-${archive.file_path}`,
          name: archive.file_path.split("/").pop() || "IMG Archive",
          assetType: "img",
          filePath: archive.file_path,
          fileSize: 0, // Would need to get from fs
          lastModified: Date.now(),
          renderWareVersion: archive.version === "V1" ? "3.1.0.1" : "3.6.0.3",
          metadata: {
            totalEntries: archive.total_entries,
            version: archive.version,
          },
          loaded: true,
          loading: false,
        };

        const newAssets = new Map(state.assets);
        newAssets.set(asset.id, asset);

        return { imgArchives: newArchives, assets: newAssets };
      });
    },

    removeImgArchive: (archivePath: string) => {
      set((state) => {
        const newArchives = new Map(state.imgArchives);
        newArchives.delete(archivePath);

        // Remove from asset registry
        const newAssets = new Map(state.assets);
        newAssets.delete(`img-${archivePath}`);

        return { imgArchives: newArchives, assets: newAssets };
      });
    },

    getImgArchive: (archivePath: string) => {
      return get().imgArchives.get(archivePath);
    },

    // TXD Archive management
    addTxdArchive: (archive: TxdArchive) => {
      set((state) => {
        const newArchives = new Map(state.txdArchives);
        newArchives.set(archive.file_path, archive);

        // Register in asset registry
        const asset: RenderWareAsset = {
          id: `txd-${archive.file_path}`,
          name: archive.file_path.split("/").pop() || "TXD Archive",
          assetType: "txd",
          filePath: archive.file_path,
          fileSize: 0,
          lastModified: Date.now(),
          renderWareVersion: "3.6.0.3", // Default for SA
          metadata: {
            totalTextures: archive.total_textures,
          },
          loaded: true,
          loading: false,
        };

        const newAssets = new Map(state.assets);
        newAssets.set(asset.id, asset);

        return { txdArchives: newArchives, assets: newAssets };
      });
    },

    removeTxdArchive: (archivePath: string) => {
      set((state) => {
        const newArchives = new Map(state.txdArchives);
        newArchives.delete(archivePath);

        const newAssets = new Map(state.assets);
        newAssets.delete(`txd-${archivePath}`);

        return { txdArchives: newArchives, assets: newAssets };
      });
    },

    getTxdArchive: (archivePath: string) => {
      return get().txdArchives.get(archivePath);
    },

    // DFF Model management
    addDffModel: (model: DffModel) => {
      set((state) => {
        const newModels = new Map(state.dffModels);
        newModels.set(model.file_path, model);

        const asset: RenderWareAsset = {
          id: `dff-${model.file_path}`,
          name: model.file_path.split("/").pop() || "DFF Model",
          assetType: "dff",
          filePath: model.file_path,
          fileSize: 0,
          lastModified: Date.now(),
          renderWareVersion: model.rw_version.toString(),
          metadata: {
            frames: model.frames.length,
            geometries: model.geometries.length,
            atomics: model.atomics.length,
          },
          loaded: true,
          loading: false,
        };

        const newAssets = new Map(state.assets);
        newAssets.set(asset.id, asset);

        return { dffModels: newModels, assets: newAssets };
      });
    },

    removeDffModel: (modelPath: string) => {
      set((state) => {
        const newModels = new Map(state.dffModels);
        newModels.delete(modelPath);

        const newAssets = new Map(state.assets);
        newAssets.delete(`dff-${modelPath}`);

        return { dffModels: newModels, assets: newAssets };
      });
    },

    getDffModel: (modelPath: string) => {
      return get().dffModels.get(modelPath);
    },

    // COL File management
    addColFile: (colFile: ColFile) => {
      set((state) => {
        const newFiles = new Map(state.colFiles);
        newFiles.set(colFile.file_path, colFile);

        const asset: RenderWareAsset = {
          id: `col-${colFile.file_path}`,
          name: colFile.file_path.split("/").pop() || "COL File",
          assetType: "col",
          filePath: colFile.file_path,
          fileSize: 0,
          lastModified: Date.now(),
          gameInfo: colFile.version.toString(),
          metadata: {
            modelCount: colFile.models.length,
            version: colFile.version,
          },
          loaded: true,
          loading: false,
        };

        const newAssets = new Map(state.assets);
        newAssets.set(asset.id, asset);

        return { colFiles: newFiles, assets: newAssets };
      });
    },

    removeColFile: (filePath: string) => {
      set((state) => {
        const newFiles = new Map(state.colFiles);
        newFiles.delete(filePath);

        const newAssets = new Map(state.assets);
        newAssets.delete(`col-${filePath}`);

        return { colFiles: newFiles, assets: newAssets };
      });
    },

    getColFile: (filePath: string) => {
      return get().colFiles.get(filePath);
    },

    // IDE Document management
    addIdeDocument: (document: IdeDocument) => {
      set((state) => {
        const newDocuments = new Map(state.ideDocuments);
        newDocuments.set(document.filePath, document);

        const asset: RenderWareAsset = {
          id: `ide-${document.filePath}`,
          name: document.filePath.split("/").pop() || "IDE Document",
          assetType: "ide",
          filePath: document.filePath,
          fileSize: 0,
          lastModified: Date.now(),
          metadata: {
            sections: Object.keys(document.sections).length,
          },
          loaded: true,
          loading: false,
        };

        const newAssets = new Map(state.assets);
        newAssets.set(asset.id, asset);

        return { ideDocuments: newDocuments, assets: newAssets };
      });
    },

    removeIdeDocument: (filePath: string) => {
      set((state) => {
        const newDocuments = new Map(state.ideDocuments);
        newDocuments.delete(filePath);

        const newAssets = new Map(state.assets);
        newAssets.delete(`ide-${filePath}`);

        return { ideDocuments: newDocuments, assets: newAssets };
      });
    },

    getIdeDocument: (filePath: string) => {
      return get().ideDocuments.get(filePath);
    },

    // RW Analysis management
    addRwAnalysis: (analysis: RwAnalysis) => {
      set((state) => {
        const newAnalyses = new Map(state.rwAnalyses);
        newAnalyses.set(analysis.file_path, analysis);

        const asset: RenderWareAsset = {
          id: `rw-${analysis.file_path}`,
          name: analysis.file_path.split("/").pop() || "RW Analysis",
          assetType: "rw-analysis",
          filePath: analysis.file_path,
          fileSize: 0,
          lastModified: Date.now(),
          metadata: {
            totalChunks: analysis.total_chunks,
            maxDepth: analysis.max_depth,
          },
          loaded: true,
          loading: false,
        };

        const newAssets = new Map(state.assets);
        newAssets.set(asset.id, asset);

        return { rwAnalyses: newAnalyses, assets: newAssets };
      });
    },

    removeRwAnalysis: (filePath: string) => {
      set((state) => {
        const newAnalyses = new Map(state.rwAnalyses);
        newAnalyses.delete(filePath);

        const newAssets = new Map(state.assets);
        newAssets.delete(`rw-${filePath}`);

        return { rwAnalyses: newAnalyses, assets: newAssets };
      });
    },

    getRwAnalysis: (filePath: string) => {
      return get().rwAnalyses.get(filePath);
    },

    // Utility functions
    getAssetsByType: (assetType: RenderWareAssetType) => {
      const assets = Array.from(get().assets.values());
      return assets.filter((asset) => asset.assetType === assetType);
    },

    getAssetByPath: (filePath: string) => {
      const assets = Array.from(get().assets.values());
      return assets.find((asset) => asset.filePath === filePath);
    },

    clearAllAssets: () => {
      set({
        imgArchives: new Map(),
        txdArchives: new Map(),
        dffModels: new Map(),
        colFiles: new Map(),
        ideDocuments: new Map(),
        rwAnalyses: new Map(),
        assets: new Map(),
      });
    },

    getStatistics: (): AssetStatistics => {
      const state = get();
      const assets = Array.from(state.assets.values());

      return {
        totalAssets: assets.length,
        imgArchives: state.imgArchives.size,
        txdArchives: state.txdArchives.size,
        dffModels: state.dffModels.size,
        colFiles: state.colFiles.size,
        ideDocuments: state.ideDocuments.size,
        rwAnalyses: state.rwAnalyses.size,
        totalFileSize: assets.reduce((sum, asset) => sum + asset.fileSize, 0),
        loadedAssets: assets.filter((asset) => asset.loaded).length,
        loadingAssets: assets.filter((asset) => asset.loading).length,
        failedAssets: assets.filter((asset) => asset.error).length,
      };
    },
  }))
);

// Export convenience hooks
export const useImgArchives = () => useAssetStore((state) => state.imgArchives);
export const useTxdArchives = () => useAssetStore((state) => state.txdArchives);
export const useDffModels = () => useAssetStore((state) => state.dffModels);
export const useColFiles = () => useAssetStore((state) => state.colFiles);
export const useIdeDocuments = () =>
  useAssetStore((state) => state.ideDocuments);
export const useRwAnalyses = () => useAssetStore((state) => state.rwAnalyses);
export const useAssetStatistics = () =>
  useAssetStore((state) => state.getStatistics());
