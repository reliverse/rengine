import { memo, useCallback } from "react";
import { useSceneStore } from "~/stores/scene-store";
import { ASSET_TYPES, FILE_EXTENSION_REGEX } from "../constants";
import type { ImportedAsset } from "../types";
import { useRenderWareState } from "./hooks/use-renderware-state";
import { useModelSearch } from "./hooks/use-model-search";
import { RenderWareHeader } from "./components/renderware-header";
import { RenderWareToolbar } from "./components/renderware-toolbar";
import { LoadedArchivesSection } from "./components/loaded-archives-section";
import { ImportedAssetsSection } from "./components/imported-assets-section";
import { ModelSearchResults } from "./components/model-search-results";
import { RenderWareDropZone } from "./components/renderware-drop-zone";

// RenderWare tab component
export const RenderWareTab = memo(function RenderWareTab() {
  const {
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
    toggleArchiveExpansion,
    toggleAssetExpansion,
    removeImportedAsset,
  } = useRenderWareState();

  const { searchSampModels, handleModelSearchInput } = useModelSearch();

  const addObject = useSceneStore((state) => state.addObject);

  const handleAssetImported = useCallback(
    (asset: ImportedAsset) => {
      const assetId = `${asset.type}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      setImportedAssets((prev) => new Map(prev).set(assetId, asset));
    },
    [setImportedAssets]
  );

  const handleModelSearchInputWrapper = useCallback(
    (value: string) => {
      handleModelSearchInput(
        value,
        setModelSearchQuery,
        searchSampModels,
        setModelSearchResults,
        setIsSearchingModels
      );
    },
    [
      handleModelSearchInput,
      searchSampModels,
      setModelSearchQuery,
      setModelSearchResults,
      setIsSearchingModels,
    ]
  );

  const handleAddToScene = useCallback(
    (asset: ImportedAsset) => {
      const sceneObject = {
        id: `scene_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        name:
          asset.file_path
            .split("/")
            .pop()
            ?.split("\\")
            .pop()
            ?.replace(FILE_EXTENSION_REGEX, "") || asset.type,
        type: "imported" as const,
        position: [0, 0, 0] as [number, number, number],
        rotation: [0, 0, 0] as [number, number, number],
        scale: [1, 1, 1] as [number, number, number],
        color: "#ffffff",
        visible: true,
        modelid: asset.samp_model_id || undefined,
      };

      addObject(sceneObject);
      console.log(
        `Added ${asset.type} to scene with ${asset.samp_model_id ? `SA:MP ID ${asset.samp_model_id}` : "no SA:MP ID"}`
      );
    },
    [addObject]
  );

  const config = ASSET_TYPES.renderware;

  return (
    <div className="flex h-full flex-col">
      <RenderWareHeader
        config={config}
        importedAssets={importedAssets}
        loadedArchives={loadedArchives}
      />

      <RenderWareToolbar
        isImporting={isImporting}
        isSearchingModels={isSearchingModels}
        modelSearchQuery={modelSearchQuery}
        onAssetImported={handleAssetImported}
        onModelSearchInput={handleModelSearchInputWrapper}
        setIsImporting={setIsImporting}
        setLoadedArchives={setLoadedArchives}
      />

      <LoadedArchivesSection
        expandedArchives={expandedArchives}
        isImporting={isImporting}
        loadedArchives={loadedArchives}
        onToggleArchiveExpansion={toggleArchiveExpansion}
        setIsImporting={setIsImporting}
      />

      <ImportedAssetsSection
        expandedAssets={expandedAssets}
        importedAssets={importedAssets}
        onAddToScene={handleAddToScene}
        onRemoveAsset={removeImportedAsset}
        onToggleAssetExpansion={toggleAssetExpansion}
      />

      <ModelSearchResults modelSearchResults={modelSearchResults} />

      <RenderWareDropZone
        config={config}
        dragOver={dragOver}
        isImporting={isImporting}
        setDragOver={setDragOver}
      />
    </div>
  );
});
