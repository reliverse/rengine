import { useState } from "react";
import type { ImgArchive, ImportedAsset } from "../../types";

export interface ModelSearchResult {
  id: number;
  name: string;
  dff: string;
  txd: string;
  radius: number;
}

export function useRenderWareState() {
  const [dragOver, setDragOver] = useState(false);
  const [isImporting, setIsImporting] = useState(false);
  const [loadedArchives, setLoadedArchives] = useState<Map<string, ImgArchive>>(
    new Map()
  );
  const [expandedArchives, setExpandedArchives] = useState<Set<string>>(
    new Set()
  );
  const [importedAssets, setImportedAssets] = useState<
    Map<string, ImportedAsset>
  >(new Map());
  const [expandedAssets, setExpandedAssets] = useState<Set<string>>(new Set());
  const [modelSearchQuery, setModelSearchQuery] = useState("");
  const [modelSearchResults, setModelSearchResults] = useState<
    ModelSearchResult[]
  >([]);
  const [isSearchingModels, setIsSearchingModels] = useState(false);

  const toggleArchiveExpansion = (archivePath: string) => {
    setExpandedArchives((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(archivePath)) {
        newSet.delete(archivePath);
      } else {
        newSet.add(archivePath);
      }
      return newSet;
    });
  };

  const toggleAssetExpansion = (assetId: string) => {
    setExpandedAssets((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(assetId)) {
        newSet.delete(assetId);
      } else {
        newSet.add(assetId);
      }
      return newSet;
    });
  };

  const removeImportedAsset = (assetId: string) => {
    setImportedAssets((prev) => {
      const newMap = new Map(prev);
      newMap.delete(assetId);
      return newMap;
    });
  };

  return {
    // State
    dragOver,
    setDragOver,
    isImporting,
    setIsImporting,
    loadedArchives,
    setLoadedArchives,
    expandedArchives,
    importedAssets,
    setImportedAssets,
    expandedAssets,
    modelSearchQuery,
    setModelSearchQuery,
    modelSearchResults,
    setModelSearchResults,
    isSearchingModels,
    setIsSearchingModels,

    // Actions
    toggleArchiveExpansion,
    toggleAssetExpansion,
    removeImportedAsset,
  };
}
