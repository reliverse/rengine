import { memo, useCallback, useMemo, useState } from "react";
import { Box, Filter, Search, SortAsc, SortDesc, Upload } from "lucide-react";
import { useShallow } from "zustand/react/shallow";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { Separator } from "~/components/ui/separator";
import { ScrollArea } from "~/components/ui/scroll-area";
import { cn } from "~/lib/utils";
import { useModelsStore, type ModelFormat } from "~/stores/models-store";
import { ModelCard } from "./model-card";

// Models tab component
export const ModelsTab = memo(function ModelsTab() {
  const {
    models,
    filterFormat,
    filterSearch,
    sortBy,
    sortOrder,
    showThumbnails,
    selectedModelIds,
    setFilterFormat,
    setFilterSearch,
    setSortBy,
    setSortOrder,
    importModels,
    selectModel,
    removeModel,
    clearSelection,
  } = useModelsStore(
    useShallow((state) => ({
      models: state.models,
      filterFormat: state.filterFormat,
      filterSearch: state.filterSearch,
      sortBy: state.sortBy,
      sortOrder: state.sortOrder,
      showThumbnails: state.showThumbnails,
      selectedModelIds: state.selectedModelIds,
      setFilterFormat: state.setFilterFormat,
      setFilterSearch: state.setFilterSearch,
      setSortBy: state.setSortBy,
      setSortOrder: state.setSortOrder,
      importModels: state.importModels,
      selectModel: state.selectModel,
      removeModel: state.removeModel,
      clearSelection: state.clearSelection,
    }))
  );

  const filteredModels = useMemo(() => {
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
  }, [models, filterFormat, filterSearch, sortBy, sortOrder]);

  const stats = useMemo(() => {
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
  }, [models]);

  const [dragOver, setDragOver] = useState(false);

  const handleFileDrop = useCallback(
    async (e: React.DragEvent) => {
      e.preventDefault();
      setDragOver(false);

      const files = Array.from(e.dataTransfer.files).filter((file) => {
        const extension = file.name.toLowerCase().split(".").pop();
        return [
          "gltf",
          "glb",
          "obj",
          "fbx",
          "dae",
          "3ds",
          "stl",
          "ply",
        ].includes(extension || "");
      });

      if (files.length > 0) {
        try {
          await importModels(files);
        } catch (error) {
          console.error("Failed to import models:", error);
        }
      }
    },
    [importModels]
  );

  const handleFileSelect = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const files = Array.from(e.target.files || []);
      if (files.length > 0) {
        try {
          await importModels(files);
        } catch (error) {
          console.error("Failed to import models:", error);
        }
      }
      e.target.value = "";
    },
    [importModels]
  );

  const handleModelClick = useCallback(
    (modelId: string, e: React.MouseEvent) => {
      if (e.ctrlKey || e.metaKey) {
        selectModel(modelId, true);
      } else {
        clearSelection();
        selectModel(modelId);
      }
    },
    [selectModel, clearSelection]
  );

  const handleBulkDelete = useCallback(() => {
    for (const id of selectedModelIds) {
      removeModel(id);
    }
    clearSelection();
  }, [selectedModelIds, removeModel, clearSelection]);

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
  };

  return (
    <div className="flex h-full flex-col">
      {/* Header with stats */}
      <div className="mb-4 flex items-center justify-between">
        <div className="flex items-center gap-4 text-muted-foreground text-sm">
          <span>{stats.totalModels} models</span>
          <span>{formatFileSize(stats.totalFileSize)} total</span>
          {stats.loadingModels > 0 && (
            <span className="text-blue-500">{stats.loadingModels} loading</span>
          )}
        </div>
      </div>

      {/* Toolbar */}
      <div className="mb-4 flex flex-wrap items-center gap-2">
        {/* Search */}
        <div className="relative min-w-48 flex-1">
          <Search className="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 transform text-muted-foreground" />
          <Input
            className="pl-9"
            onChange={(e) => setFilterSearch(e.target.value)}
            placeholder="Search models..."
            value={filterSearch}
          />
        </div>

        {/* Filter by format */}
        <Select
          onValueChange={(value) => setFilterFormat(value || "")}
          value={filterFormat}
        >
          <SelectTrigger className="w-32">
            <Filter className="mr-2 h-4 w-4" />
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">All Formats</SelectItem>
            <SelectItem value="gltf">GLTF</SelectItem>
            <SelectItem value="glb">GLB</SelectItem>
            <SelectItem value="obj">OBJ</SelectItem>
            <SelectItem value="fbx">FBX</SelectItem>
            <SelectItem value="dae">DAE</SelectItem>
            <SelectItem value="3ds">3DS</SelectItem>
            <SelectItem value="stl">STL</SelectItem>
            <SelectItem value="ply">PLY</SelectItem>
          </SelectContent>
        </Select>

        {/* Sort */}
        <Select onValueChange={(value: any) => setSortBy(value)} value={sortBy}>
          <SelectTrigger className="w-32">
            {sortOrder === "asc" ? (
              <SortAsc className="mr-2 h-4 w-4" />
            ) : (
              <SortDesc className="mr-2 h-4 w-4" />
            )}
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="name">Name</SelectItem>
            <SelectItem value="format">Format</SelectItem>
            <SelectItem value="size">Size</SelectItem>
            <SelectItem value="date">Date</SelectItem>
          </SelectContent>
        </Select>

        <Button
          onClick={() => setSortOrder(sortOrder === "asc" ? "desc" : "asc")}
          size="sm"
          variant="outline"
        >
          {sortOrder === "asc" ? (
            <SortAsc className="h-4 w-4" />
          ) : (
            <SortDesc className="h-4 w-4" />
          )}
        </Button>

        {/* Import */}
        <div className="relative">
          <input
            accept=".gltf,.glb,.obj,.fbx,.dae,.3ds,.stl,.ply"
            className="absolute inset-0 h-full w-full cursor-pointer opacity-0"
            multiple
            onChange={handleFileSelect}
            type="file"
          />
          <Button size="sm" variant="outline">
            <Upload className="mr-2 h-4 w-4" />
            Import
          </Button>
        </div>

        {/* Bulk actions */}
        {selectedModelIds.size > 0 && (
          <>
            <Separator className="h-6" orientation="vertical" />
            <span className="text-muted-foreground text-sm">
              {selectedModelIds.size} selected
            </span>
            <Button onClick={handleBulkDelete} size="sm" variant="destructive">
              Delete
            </Button>
          </>
        )}
      </div>

      {/* Models Grid */}
      <ScrollArea className="flex-1">
        <div
          className={cn(
            "grid gap-4 rounded-lg border-2 border-dashed p-4 transition-colors",
            dragOver ? "border-primary bg-primary/5" : "border-muted",
            "grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8"
          )}
          onDragLeave={() => setDragOver(false)}
          onDragOver={(e) => {
            e.preventDefault();
            setDragOver(true);
          }}
          onDrop={handleFileDrop}
        >
          {filteredModels.map((model) => (
            <ModelCard
              isSelected={selectedModelIds.has(model.id)}
              key={model.id}
              model={model}
              onClick={(e) => handleModelClick(model.id, e)}
              onDelete={() => removeModel(model.id)}
              showThumbnail={showThumbnails}
            />
          ))}

          {filteredModels.length === 0 && (
            <div className="col-span-full flex flex-col items-center justify-center py-12 text-center">
              <Box className="mb-4 h-12 w-12 text-muted-foreground" />
              <h3 className="mb-2 font-medium text-lg">No models found</h3>
              <p className="mb-4 text-muted-foreground">
                {filterSearch || filterFormat
                  ? "Try adjusting your filters or search terms."
                  : "Import some 3D models to get started."}
              </p>
              <div className="relative">
                <input
                  accept=".gltf,.glb,.obj,.fbx,.dae,.3ds,.stl,.ply"
                  className="absolute inset-0 h-full w-full cursor-pointer opacity-0"
                  multiple
                  onChange={handleFileSelect}
                  type="file"
                />
                <Button>
                  <Upload className="mr-2 h-4 w-4" />
                  Import Models
                </Button>
              </div>
              <p className="mt-2 text-muted-foreground text-xs">
                Supported: GLTF, GLB, OBJ, FBX, DAE, 3DS, STL, PLY
              </p>
            </div>
          )}
        </div>
      </ScrollArea>
    </div>
  );
});
