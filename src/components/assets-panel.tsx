import {
  Box,
  Code,
  Filter,
  Folder,
  Globe,
  Image,
  Music,
  Package,
  Palette,
  Search,
  Settings,
  SortAsc,
  SortDesc,
  Type,
  Upload,
  X,
} from "lucide-react";
import { memo, useCallback, useMemo, useState } from "react";
import { useShallow } from "zustand/react/shallow";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";

import { Input } from "~/components/ui/input";
import { ScrollArea } from "~/components/ui/scroll-area";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { Separator } from "~/components/ui/separator";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { cn } from "~/lib/utils";
import { useAudioStore, type AudioFormat } from "~/stores/audio-store";
import { useModelsStore, type ModelFormat } from "~/stores/models-store";
import { useSceneStore } from "~/stores/scene-store";
import { RemoteAssetsTab } from "./remote-assets-tab";
import { TextureLibrary } from "./texture-library";

// Regex patterns for file path parsing
const PATH_SEPARATOR_REGEX = /[/\\]/;
const FILE_EXTENSION_REGEX = /\.(dff|obj|txd|col|ipl)$/i;

// Asset types supported by the engine
export type AssetType =
  | "models"
  | "textures"
  | "materials"
  | "audio"
  | "scripts"
  | "fonts"
  | "prefabs"
  | "renderware"
  | "remote";

interface AssetsPanelProps {
  className?: string;
}

// Asset type configurations
const ASSET_TYPES = {
  models: {
    label: "Models",
    icon: Box,
    accept: ".gltf,.glb,.obj,.fbx,.dae,.3ds,.stl,.ply",
    description: "3D models and meshes",
    color: "text-blue-500",
  },
  textures: {
    label: "Textures",
    icon: Image,
    accept: "image/*,.tga,.dds",
    description: "Image textures and materials",
    color: "text-green-500",
  },
  materials: {
    label: "Materials",
    icon: Palette,
    accept: ".json,.mat",
    description: "Material definitions and shaders",
    color: "text-purple-500",
  },
  audio: {
    label: "Audio",
    icon: Music,
    accept: ".wav,.mp3,.ogg,.aac,.flac",
    description: "Sound effects and music",
    color: "text-orange-500",
  },
  scripts: {
    label: "Scripts",
    icon: Code,
    accept: ".ts,.tsx,.js,.jsx,.lua,.py",
    description: "Game logic and behaviors",
    color: "text-yellow-500",
  },
  fonts: {
    label: "Fonts",
    icon: Type,
    accept: ".ttf,.otf,.woff,.woff2",
    description: "Typography and text rendering",
    color: "text-pink-500",
  },
  prefabs: {
    label: "Prefabs",
    icon: Package,
    accept: ".prefab,.json",
    description: "Reusable object templates",
    color: "text-indigo-500",
  },
  renderware: {
    label: "RW/GTA",
    icon: Settings,
    accept: ".img,.txd,.dff,.col,.ide,.pwn",
    description: "GTA engine assets and PWN scripts",
    color: "text-red-500",
  },
  remote: {
    label: "Remote",
    icon: Globe,
    accept: "",
    description: "Third-party asset libraries",
    color: "text-cyan-500",
  },
} as const;

// Placeholder components for asset types not yet implemented
const PlaceholderAssetTab = memo(function PlaceholderAssetTab({
  assetType,
}: {
  assetType: AssetType;
}) {
  const config = ASSET_TYPES[assetType];

  return (
    <div className="flex h-full flex-col items-center justify-center p-8 text-center">
      <div className={cn("mb-4 rounded-full bg-muted p-4", config.color)}>
        <config.icon className="h-8 w-8" />
      </div>
      <h3 className="mb-2 font-medium text-lg">{config.label}</h3>
      <p className="mb-4 text-muted-foreground">{config.description}</p>
      {config.accept && (
        <div className="relative">
          <input
            accept={config.accept}
            className="absolute inset-0 h-full w-full cursor-pointer opacity-0"
            multiple
            type="file"
          />
          <Button>
            <Upload className="mr-2 h-4 w-4" />
            Import {config.label}
          </Button>
        </div>
      )}
      {config.accept && (
        <p className="mt-2 text-muted-foreground text-xs">
          Supported formats: {config.accept}
        </p>
      )}
    </div>
  );
});

// Models tab component
const ModelsTab = memo(function ModelsTab() {
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

// Audio tab component
const AudioTab = memo(function AudioTab() {
  const {
    audioFiles,
    filterFormat,
    filterSearch,
    sortBy,
    sortOrder,
    showWaveforms,
    selectedAudioIds,
    currentlyPlaying,
    playbackVolume,
    setFilterFormat,
    setFilterSearch,
    setSortBy,
    setSortOrder,
    importAudioFiles,
    selectAudioFile,
    removeAudioFile,
    clearSelection,
    playAudio,
    pauseAudio,
    setVolume,
  } = useAudioStore(
    useShallow((state) => ({
      audioFiles: state.audioFiles,
      filterFormat: state.filterFormat,
      filterSearch: state.filterSearch,
      sortBy: state.sortBy,
      sortOrder: state.sortOrder,
      showWaveforms: state.showWaveforms,
      selectedAudioIds: state.selectedAudioIds,
      currentlyPlaying: state.currentlyPlaying,
      playbackVolume: state.playbackVolume,
      setFilterFormat: state.setFilterFormat,
      setFilterSearch: state.setFilterSearch,
      setSortBy: state.setSortBy,
      setSortOrder: state.setSortOrder,
      importAudioFiles: state.importAudioFiles,
      selectAudioFile: state.selectAudioFile,
      removeAudioFile: state.removeAudioFile,
      clearSelection: state.clearSelection,
      playAudio: state.playAudio,
      pauseAudio: state.pauseAudio,
      setVolume: state.setVolume,
    }))
  );

  const filteredAudioFiles = useMemo(() => {
    const filtered = audioFiles.filter((audio) => {
      const matchesFormat = !filterFormat || audio.format === filterFormat;
      const matchesSearch =
        !filterSearch ||
        audio.name.toLowerCase().includes(filterSearch.toLowerCase()) ||
        audio.format.toLowerCase().includes(filterSearch.toLowerCase());

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
        case "duration":
          comparison = (a.duration || 0) - (b.duration || 0);
          break;
      }

      return sortOrder === "asc" ? comparison : -comparison;
    });

    return filtered;
  }, [audioFiles, filterFormat, filterSearch, sortBy, sortOrder]);

  const stats = useMemo(() => {
    const formats: Record<AudioFormat, number> = {
      wav: 0,
      mp3: 0,
      ogg: 0,
      aac: 0,
      flac: 0,
      m4a: 0,
      wma: 0,
    };

    for (const audio of audioFiles) {
      formats[audio.format]++;
    }

    return {
      totalFiles: audioFiles.length,
      totalFileSize: audioFiles.reduce((sum, audio) => sum + audio.fileSize, 0),
      formats,
      totalDuration: audioFiles.reduce(
        (sum, audio) => sum + (audio.duration || 0),
        0
      ),
      loadedFiles: audioFiles.filter((a) => a.loaded).length,
      loadingFiles: audioFiles.filter((a) => a.loading).length,
      failedFiles: audioFiles.filter((a) => a.error).length,
    };
  }, [audioFiles]);

  const [dragOver, setDragOver] = useState(false);

  const handleFileDrop = useCallback(
    async (e: React.DragEvent) => {
      e.preventDefault();
      setDragOver(false);

      const files = Array.from(e.dataTransfer.files).filter((file) => {
        const extension = file.name.toLowerCase().split(".").pop();
        return ["wav", "mp3", "ogg", "aac", "flac", "m4a", "wma"].includes(
          extension || ""
        );
      });

      if (files.length > 0) {
        try {
          await importAudioFiles(files);
        } catch (error) {
          console.error("Failed to import audio files:", error);
        }
      }
    },
    [importAudioFiles]
  );

  const handleFileSelect = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const files = Array.from(e.target.files || []);
      if (files.length > 0) {
        try {
          await importAudioFiles(files);
        } catch (error) {
          console.error("Failed to import audio files:", error);
        }
      }
      e.target.value = "";
    },
    [importAudioFiles]
  );

  const handleAudioClick = useCallback(
    (audioId: string, e: React.MouseEvent) => {
      if (e.ctrlKey || e.metaKey) {
        selectAudioFile(audioId, true);
      } else {
        clearSelection();
        selectAudioFile(audioId, false);
      }
    },
    [selectAudioFile, clearSelection]
  );

  const handlePlayPause = useCallback(
    (audioId: string, e: React.MouseEvent) => {
      e.stopPropagation();
      if (currentlyPlaying === audioId) {
        pauseAudio();
      } else {
        playAudio(audioId);
      }
    },
    [currentlyPlaying, playAudio, pauseAudio]
  );

  const handleBulkDelete = useCallback(() => {
    for (const id of selectedAudioIds) {
      removeAudioFile(id);
    }
    clearSelection();
  }, [selectedAudioIds, removeAudioFile, clearSelection]);

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
  };

  const formatDuration = (seconds?: number) => {
    if (!seconds) return "0:00";
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  };

  return (
    <div className="flex h-full flex-col">
      {/* Header with stats */}
      <div className="mb-4 flex items-center justify-between">
        <div className="flex items-center gap-4 text-muted-foreground text-sm">
          <span>{stats.totalFiles} audio files</span>
          <span>{formatFileSize(stats.totalFileSize)} total</span>
          <span>{formatDuration(stats.totalDuration)} total duration</span>
          {stats.loadingFiles > 0 && (
            <span className="text-blue-500">{stats.loadingFiles} loading</span>
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
            placeholder="Search audio files..."
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
            <SelectItem value="wav">WAV</SelectItem>
            <SelectItem value="mp3">MP3</SelectItem>
            <SelectItem value="ogg">OGG</SelectItem>
            <SelectItem value="aac">AAC</SelectItem>
            <SelectItem value="flac">FLAC</SelectItem>
            <SelectItem value="m4a">M4A</SelectItem>
            <SelectItem value="wma">WMA</SelectItem>
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
            <SelectItem value="duration">Duration</SelectItem>
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

        {/* Volume control */}
        <div className="flex items-center gap-2">
          <span className="text-muted-foreground text-xs">Vol:</span>
          <input
            className="w-16"
            max={1}
            min={0}
            onChange={(e) => setVolume(Number.parseFloat(e.target.value))}
            step={0.1}
            type="range"
            value={playbackVolume}
          />
        </div>

        {/* Import */}
        <div className="relative">
          <input
            accept=".wav,.mp3,.ogg,.aac,.flac,.m4a,.wma"
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
        {selectedAudioIds.size > 0 && (
          <>
            <Separator className="h-6" orientation="vertical" />
            <span className="text-muted-foreground text-sm">
              {selectedAudioIds.size} selected
            </span>
            <Button onClick={handleBulkDelete} size="sm" variant="destructive">
              Delete
            </Button>
          </>
        )}
      </div>

      {/* Audio Grid */}
      <ScrollArea className="flex-1">
        <div
          className={cn(
            "grid gap-4 rounded-lg border-2 border-dashed p-4 transition-colors",
            dragOver ? "border-primary bg-primary/5" : "border-muted",
            "grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4"
          )}
          onDragLeave={() => setDragOver(false)}
          onDragOver={(e) => {
            e.preventDefault();
            setDragOver(true);
          }}
          onDrop={handleFileDrop}
        >
          {filteredAudioFiles.map((audio) => (
            <AudioCard
              audio={audio}
              currentlyPlaying={currentlyPlaying}
              isSelected={selectedAudioIds.has(audio.id)}
              key={audio.id}
              onClick={(e) => handleAudioClick(audio.id, e)}
              onDelete={() => removeAudioFile(audio.id)}
              onPlayPause={(e) => handlePlayPause(audio.id, e)}
              showWaveform={showWaveforms}
            />
          ))}

          {filteredAudioFiles.length === 0 && (
            <div className="col-span-full flex flex-col items-center justify-center py-12 text-center">
              <Music className="mb-4 h-12 w-12 text-muted-foreground" />
              <h3 className="mb-2 font-medium text-lg">No audio files found</h3>
              <p className="mb-4 text-muted-foreground">
                {filterSearch || filterFormat
                  ? "Try adjusting your filters or search terms."
                  : "Import some audio files to get started."}
              </p>
              <div className="relative">
                <input
                  accept=".wav,.mp3,.ogg,.aac,.flac,.m4a,.wma"
                  className="absolute inset-0 h-full w-full cursor-pointer opacity-0"
                  multiple
                  onChange={handleFileSelect}
                  type="file"
                />
                <Button>
                  <Upload className="mr-2 h-4 w-4" />
                  Import Audio
                </Button>
              </div>
              <p className="mt-2 text-muted-foreground text-xs">
                Supported: WAV, MP3, OGG, AAC, FLAC, M4A, WMA
              </p>
            </div>
          )}
        </div>
      </ScrollArea>
    </div>
  );
});

// Scripts tab component (placeholder for now)
const ScriptsTab = memo(function ScriptsTab() {
  return <PlaceholderAssetTab assetType="scripts" />;
});

// Fonts tab component (placeholder for now)
const FontsTab = memo(function FontsTab() {
  return <PlaceholderAssetTab assetType="fonts" />;
});

// Prefabs tab component (placeholder for now)
const PrefabsTab = memo(function PrefabsTab() {
  return <PlaceholderAssetTab assetType="prefabs" />;
});

// IMG Archive interface
interface ImgArchive {
  path: string;
  entries: Array<{
    name: string;
    size: number;
    offset: number;
    is_compressed?: boolean;
    rw_version?: string;
  }>;
  version: string;
  total_entries: number;
  file_size: number;
}

// Imported asset interfaces
interface ImportedDffAsset {
  type: "dff";
  file_path: string;
  rw_version: number;
  frame_count: number;
  geometry_count: number;
  atomic_count: number;
  material_count?: number;
  texture_count?: number;
  samp_model_id?: number | null;
  samp_model_name?: string | null;
  loaded_at: number;
}

interface ImportedTxdAsset {
  type: "txd";
  file_path: string;
  texture_count: number;
  textures: Array<{
    name: string;
    width: number;
    height: number;
    format: string;
  }>;
  renderware_version?: string;
  samp_model_id?: number | null;
  samp_model_name?: string | null;
  loaded_at: number;
}

interface ImportedColAsset {
  type: "col";
  file_path: string;
  version: string;
  model_count: number;
  models: Array<{
    name: string;
    face_count: number;
    vertex_count: number;
  }>;
  samp_model_id?: number | null;
  samp_model_name?: string | null;
  loaded_at: number;
}

interface ImportedIplAsset {
  type: "ipl";
  file_path: string;
  instance_count: number;
  zone_count: number;
  cull_count: number;
  pick_count: number;
  samp_model_id?: number | null;
  samp_model_name?: string | null;
  loaded_at: number;
}

type ImportedAsset =
  | ImportedDffAsset
  | ImportedTxdAsset
  | ImportedColAsset
  | ImportedIplAsset;

// RenderWare tab component
const RenderWareTab = memo(function RenderWareTab() {
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
    Array<{
      id: number;
      name: string;
      dff: string;
      txd: string;
      radius: number;
    }>
  >([]);
  const [isSearchingModels, setIsSearchingModels] = useState(false);
  const addObject = useSceneStore((state) => state.addObject);

  const loadImgArchive = useCallback(async (filePath: string) => {
    try {
      console.log("Loading IMG archive:", filePath);
      const archive = (await invoke("load_img_archive", {
        path: filePath,
      })) as ImgArchive;
      setLoadedArchives((prev) => new Map(prev.set(filePath, archive)));
      console.log("IMG archive loaded:", archive);
      return archive;
    } catch (error) {
      console.error("Failed to load IMG archive:", error);
      throw error;
    }
  }, []);

  const extractImgEntry = useCallback(
    async (archivePath: string, entryName: string, outputPath: string) => {
      try {
        await invoke("extract_img_entry", {
          archivePath,
          entryName,
          outputPath,
        });
        console.log(`Extracted ${entryName} to ${outputPath}`);
      } catch (error) {
        console.error(`Failed to extract ${entryName}:`, error);
        throw error;
      }
    },
    []
  );

  const importIndividualFile = useCallback(
    async (fileType: "dff" | "txd" | "col" | "ipl") => {
      try {
        const filters = {
          dff: [{ name: "DFF Model", extensions: ["dff", "obj"] }], // Support both DFF and OBJ
          txd: [{ name: "TXD Texture Archive", extensions: ["txd"] }],
          col: [{ name: "COL Collision", extensions: ["col"] }],
          ipl: [{ name: "IPL Placement", extensions: ["ipl"] }],
        };

        const filePath = await open({
          filters: filters[fileType],
        });

        if (!filePath || typeof filePath !== "string") {
          return;
        }

        setIsImporting(true);

        // Extract base name for SA:MP model lookup
        const fileNameOnly = filePath.split(PATH_SEPARATOR_REGEX).pop() || "";
        const baseName = fileNameOnly.replace(FILE_EXTENSION_REGEX, "");

        // Try to find matching SA:MP model
        let sampModel: {
          id: number;
          name: string;
          dff: string;
          txd: string;
          radius: number;
        } | null = null;
        try {
          const result = await invoke("get_samp_model_by_name", {
            name: baseName,
          });
          if (result) {
            sampModel = result as {
              id: number;
              name: string;
              dff: string;
              txd: string;
              radius: number;
            };
            console.log(
              `✅ Auto-matched "${baseName}" to SA:MP model ID ${sampModel.id} (${sampModel.dff})`
            );
          } else {
            console.log(`ℹ️ No SA:MP model found for "${baseName}"`);
          }
        } catch (error) {
          console.log(`⚠️ SA:MP model lookup failed for "${baseName}":`, error);
        }

        const command = `import_${fileType}_file`;
        const result = (await invoke(command, { filePath })) as any;

        console.log(`${fileType.toUpperCase()} file imported:`, result);

        // Store the imported asset with SA:MP model info if found
        let asset: ImportedAsset;
        const assetId = `${fileType}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

        if (fileType === "dff") {
          asset = {
            type: "dff",
            file_path: filePath,
            rw_version: result.rw_version,
            frame_count: result.frames?.length || 0,
            geometry_count: result.geometries?.length || 0,
            atomic_count: result.atomics?.length || 0,
            material_count:
              result.geometries?.reduce(
                (sum: number, g: any) => sum + (g.materials?.length || 0),
                0
              ) || 0,
            texture_count:
              result.geometries?.reduce(
                (sum: number, g: any) => sum + (g.textures?.length || 0),
                0
              ) || 0,
            samp_model_id: sampModel?.id || null,
            samp_model_name: sampModel?.name || null,
            loaded_at: Date.now(),
          } as ImportedDffAsset;
        } else if (fileType === "txd") {
          asset = {
            type: "txd",
            file_path: filePath,
            texture_count:
              result.total_textures || result.textures?.length || 0,
            textures: result.textures || [],
            renderware_version: result.renderware_version,
            samp_model_id: sampModel?.id || null,
            samp_model_name: sampModel?.name || null,
            loaded_at: Date.now(),
          } as ImportedTxdAsset;
        } else if (fileType === "col") {
          asset = {
            type: "col",
            file_path: filePath,
            version: result.version || "Unknown",
            model_count: result.models?.length || 0,
            models:
              result.models?.map((m: any) => ({
                name: m.name || "Unknown",
                face_count: m.faces?.length || 0,
                vertex_count: m.vertices?.length || 0,
              })) || [],
            samp_model_id: sampModel?.id || null,
            samp_model_name: sampModel?.name || null,
            loaded_at: Date.now(),
          } as ImportedColAsset;
        } else if (fileType === "ipl") {
          asset = {
            type: "ipl",
            file_path: filePath,
            instance_count: result.instances?.length || 0,
            zone_count: result.zones?.length || 0,
            cull_count: result.culls?.length || 0,
            pick_count: result.picks?.length || 0,
            samp_model_id: sampModel?.id || null,
            samp_model_name: sampModel?.name || null,
            loaded_at: Date.now(),
          } as ImportedIplAsset;
        } else {
          throw new Error(`Unknown asset type: ${fileType}`);
        }

        setImportedAssets((prev) => new Map(prev).set(assetId, asset));

        // Show notification if SA:MP model was found
        if (sampModel) {
          console.log(
            `✅ Auto-matched ${baseName} to SA:MP model ID ${sampModel.id}`
          );
        }
      } catch (error) {
        console.error(`${fileType.toUpperCase()} import error:`, error);
      } finally {
        setIsImporting(false);
      }
    },
    []
  );

  const toggleArchiveExpansion = useCallback((archivePath: string) => {
    setExpandedArchives((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(archivePath)) {
        newSet.delete(archivePath);
      } else {
        newSet.add(archivePath);
      }
      return newSet;
    });
  }, []);

  const toggleAssetExpansion = useCallback((assetId: string) => {
    setExpandedAssets((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(assetId)) {
        newSet.delete(assetId);
      } else {
        newSet.add(assetId);
      }
      return newSet;
    });
  }, []);

  const removeImportedAsset = useCallback((assetId: string) => {
    setImportedAssets((prev) => {
      const newMap = new Map(prev);
      newMap.delete(assetId);
      return newMap;
    });
  }, []);

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
  };

  const searchSampModels = useCallback(async (query: string) => {
    if (!query.trim()) {
      setModelSearchResults([]);
      return;
    }

    setIsSearchingModels(true);
    try {
      const results = (await invoke("search_samp_models_by_name", {
        query: query.trim(),
        limit: 20,
      })) as Array<{
        id: number;
        name: string;
        dff: string;
        txd: string;
        radius: number;
      }>;

      setModelSearchResults(results);
    } catch (error) {
      console.error("Error searching SA:MP models:", error);
      setModelSearchResults([]);
    } finally {
      setIsSearchingModels(false);
    }
  }, []);

  const handleModelSearchInput = useCallback(
    (value: string) => {
      setModelSearchQuery(value);
      // Debounce search
      const timeoutId = setTimeout(() => {
        searchSampModels(value);
      }, 300);

      return () => clearTimeout(timeoutId);
    },
    [searchSampModels]
  );

  const handleImportFiles = useCallback((files: File[]) => {
    // For drag-and-dropped files, we can't get the file path
    // This would require additional handling - for now, show message
    for (const file of files) {
      const extension = file.name.toLowerCase().split(".").pop();
      console.log(
        `${extension?.toUpperCase()} file dropped: ${file.name} - use the dedicated import button for this file type`
      );
    }
  }, []);

  const handleFileDrop = useCallback(
    async (e: React.DragEvent) => {
      e.preventDefault();
      setDragOver(false);

      const files = Array.from(e.dataTransfer.files).filter((file) => {
        const extension = file.name.toLowerCase().split(".").pop();
        return ["img", "txd", "dff", "col", "ide", "pwn"].includes(
          extension || ""
        );
      });

      if (files.length > 0) {
        await handleImportFiles(files);
      }
    },
    [handleImportFiles]
  );

  const handlePwnImport = useCallback(async () => {
    try {
      const filePath = await open({
        filters: [
          {
            name: "Pawn Script",
            extensions: ["pwn"],
          },
        ],
      });

      if (!filePath || typeof filePath !== "string") {
        return;
      }

      setIsImporting(true);

      // Use backend PWN parsing
      const result = await invoke("parse_pwn_file", {
        filePath,
      });

      const pwnResult = result as any;

      if (pwnResult.errors.length > 0) {
        console.warn(
          "PWN parsing warnings:",
          pwnResult.errors.slice(0, 3).join(", ")
        );
      }

      if (pwnResult.objects.length === 0) {
        console.warn("No valid CreateDynamicObject calls found in the file");
        return;
      }

      // Convert and add objects to scene
      for (const pwnObj of pwnResult.objects) {
        const sceneObject = {
          id: `pwn_import_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
          name: `Object_${pwnObj.modelid}`,
          type: "imported" as const,
          modelid: pwnObj.modelid,
          position: [pwnObj.x, pwnObj.y, pwnObj.z] as [number, number, number],
          rotation: [pwnObj.rx, pwnObj.ry, pwnObj.rz] as [
            number,
            number,
            number,
          ],
          scale: [1, 1, 1] as [number, number, number],
          color: "#ffffff",
          visible: true,
          pwnData: {
            worldid: pwnObj.worldid,
            interiorid: pwnObj.interiorid,
            playerid: pwnObj.playerid,
            streamdistance: pwnObj.streamdistance,
            drawdistance: pwnObj.drawdistance,
            areaid: pwnObj.areaid,
            priority: pwnObj.priority,
          },
        };

        addObject(sceneObject);
      }

      console.log(`Imported ${pwnResult.objects.length} objects from PWN file`);
    } catch (error) {
      console.error("PWN import error:", error);
    } finally {
      setIsImporting(false);
    }
  }, [addObject]);

  const config = ASSET_TYPES.renderware;

  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <div className="mb-4 flex items-center justify-between">
        <div>
          <h3 className="mb-1 font-medium text-lg">{config.label}</h3>
          <p className="text-muted-foreground text-sm">{config.description}</p>
        </div>
        <div className="text-right text-muted-foreground text-xs">
          <div>{loadedArchives.size} archives loaded</div>
          <div>
            {Array.from(loadedArchives.values()).reduce(
              (sum, archive) => sum + archive.total_entries,
              0
            )}{" "}
            archive entries
          </div>
          <div>{importedAssets.size} imported assets</div>
        </div>
      </div>

      {/* Toolbar */}
      <div className="mb-4 flex flex-wrap gap-2">
        {/* SA:MP Model Search */}
        <div className="relative min-w-64 flex-1">
          <Search className="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 transform text-muted-foreground" />
          <Input
            className="pl-9"
            onChange={(e) => handleModelSearchInput(e.target.value)}
            placeholder="Search SA:MP models by name..."
            value={modelSearchQuery}
          />
          {isSearchingModels && (
            <div className="absolute top-1/2 right-3 -translate-y-1/2">
              <div className="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent" />
            </div>
          )}
        </div>

        <Button
          disabled={isImporting}
          onClick={async () => {
            try {
              setIsImporting(true);
              const filePath = await open({
                filters: [{ name: "IMG Archive", extensions: ["img"] }],
              });
              if (filePath && typeof filePath === "string") {
                const archive = await loadImgArchive(filePath);
                console.log("Archive loaded:", archive);
              }
            } catch (error) {
              console.error("IMG archive load error:", error);
            } finally {
              setIsImporting(false);
            }
          }}
          size="sm"
          variant="outline"
        >
          <Upload className="mr-2 h-4 w-4" />
          Load IMG Archive
        </Button>

        <Button
          disabled={isImporting}
          onClick={() => importIndividualFile("dff")}
          size="sm"
          variant="outline"
        >
          Import DFF
        </Button>

        <Button
          disabled={isImporting}
          onClick={() => importIndividualFile("txd")}
          size="sm"
          variant="outline"
        >
          Import TXD
        </Button>

        <Button
          disabled={isImporting}
          onClick={() => importIndividualFile("col")}
          size="sm"
          variant="outline"
        >
          Import COL
        </Button>

        <Button
          disabled={isImporting}
          onClick={() => importIndividualFile("ipl")}
          size="sm"
          variant="outline"
        >
          Import IPL
        </Button>

        <Button
          disabled={isImporting}
          onClick={async () => {
            try {
              setIsImporting(true);

              // Select IDE file
              const idePath = await open({
                filters: [{ name: "IDE Definitions", extensions: ["ide"] }],
              });

              if (!idePath || typeof idePath !== "string") {
                setIsImporting(false);
                return;
              }

              // Select IMG archive
              const imgPath = await open({
                filters: [{ name: "IMG Archive", extensions: ["img"] }],
              });

              if (!imgPath || typeof imgPath !== "string") {
                setIsImporting(false);
                return;
              }

              // Import via IDE
              const result = (await invoke("import_via_ide", {
                imgArchivePath: imgPath,
                ideFilePath: idePath,
                modelsDirectory: undefined, // Will use IDE file's directory
              })) as any;

              console.log("IDE-based import completed:", result);
            } catch (error) {
              console.error("IDE-based import error:", error);
            } finally {
              setIsImporting(false);
            }
          }}
          size="sm"
          variant="outline"
        >
          <Settings className="mr-2 h-4 w-4" />
          IDE Import
        </Button>

        <Button
          disabled={isImporting}
          onClick={handlePwnImport}
          size="sm"
          variant="outline"
        >
          <Code className="mr-2 h-4 w-4" />
          Import PWN
        </Button>
      </div>

      {/* Loaded Archives */}
      {loadedArchives.size > 0 && (
        <div className="mb-4">
          <h4 className="mb-2 font-medium text-sm">Loaded Archives</h4>
          <div className="space-y-2">
            {Array.from(loadedArchives.entries()).map(([path, archive]) => (
              <Card className="p-3" key={path}>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Button
                      className="h-6 w-6 p-0"
                      onClick={() => toggleArchiveExpansion(path)}
                      size="sm"
                      variant="ghost"
                    >
                      {expandedArchives.has(path) ? "▼" : "▶"}
                    </Button>
                    <div>
                      <div className="font-medium text-sm">
                        {path.split("/").pop()?.split("\\").pop()}
                      </div>
                      <div className="text-muted-foreground text-xs">
                        {archive.total_entries} entries •{" "}
                        {formatFileSize(archive.file_size)} • {archive.version}
                      </div>
                    </div>
                  </div>
                  <Button
                    disabled={isImporting}
                    onClick={async () => {
                      try {
                        const outputDir = await open({
                          directory: true,
                        });
                        if (outputDir && typeof outputDir === "string") {
                          setIsImporting(true);
                          // Extract all entries
                          for (const entry of archive.entries.slice(0, 10)) {
                            // Limit to first 10 for demo
                            try {
                              await extractImgEntry(
                                path,
                                entry.name,
                                `${outputDir}/${entry.name}`
                              );
                            } catch (error) {
                              console.error(
                                `Failed to extract ${entry.name}:`,
                                error
                              );
                            }
                          }
                        }
                      } catch (error) {
                        console.error("Batch extraction error:", error);
                      } finally {
                        setIsImporting(false);
                      }
                    }}
                    size="sm"
                    variant="outline"
                  >
                    Extract All
                  </Button>
                </div>

                {expandedArchives.has(path) && (
                  <div className="mt-3 max-h-64 overflow-y-auto">
                    <div className="space-y-1">
                      {archive.entries.slice(0, 50).map((entry) => (
                        <div
                          className="flex items-center justify-between rounded bg-muted/50 px-2 py-1 text-xs"
                          key={entry.name}
                        >
                          <div className="flex items-center gap-2">
                            <span className="font-mono">{entry.name}</span>
                            {entry.is_compressed && (
                              <span className="rounded bg-blue-100 px-1 text-blue-700">
                                compressed
                              </span>
                            )}
                            {entry.rw_version && (
                              <span className="rounded bg-green-100 px-1 text-green-700">
                                {entry.rw_version}
                              </span>
                            )}
                          </div>
                          <div className="flex items-center gap-2">
                            <span className="text-muted-foreground">
                              {formatFileSize(entry.size)}
                            </span>
                            <Button
                              className="h-6 w-6 p-0"
                              onClick={async () => {
                                try {
                                  const outputPath = await open({
                                    defaultPath: entry.name,
                                  });
                                  if (
                                    outputPath &&
                                    typeof outputPath === "string"
                                  ) {
                                    await extractImgEntry(
                                      path,
                                      entry.name,
                                      outputPath
                                    );
                                  }
                                } catch (error) {
                                  console.error(
                                    `Failed to extract ${entry.name}:`,
                                    error
                                  );
                                }
                              }}
                              size="sm"
                              variant="ghost"
                            >
                              <Upload className="h-3 w-3" />
                            </Button>
                          </div>
                        </div>
                      ))}
                      {archive.entries.length > 50 && (
                        <div className="py-1 text-center text-muted-foreground text-xs">
                          ... and {archive.entries.length - 50} more entries
                        </div>
                      )}
                    </div>
                  </div>
                )}
              </Card>
            ))}
          </div>
        </div>
      )}

      {/* Imported Assets */}
      {importedAssets.size > 0 && (
        <div className="mb-4">
          <h4 className="mb-2 font-medium text-sm">Imported Assets</h4>
          <div className="space-y-2">
            {Array.from(importedAssets.entries()).map(([assetId, asset]) => (
              <Card className="p-3" key={assetId}>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Button
                      className="h-6 w-6 p-0"
                      onClick={() => toggleAssetExpansion(assetId)}
                      size="sm"
                      variant="ghost"
                    >
                      {expandedAssets.has(assetId) ? "▼" : "▶"}
                    </Button>
                    <div>
                      <div className="flex items-center gap-2 font-medium text-sm">
                        <span className="rounded bg-primary/10 px-1 py-0.5 text-primary text-xs uppercase">
                          {asset.type}
                        </span>
                        {asset.file_path.split("/").pop()?.split("\\").pop()}
                        {asset.samp_model_id && (
                          <span className="rounded bg-green-100 px-1 py-0.5 font-bold text-green-700 text-xs">
                            ID: {asset.samp_model_id}
                          </span>
                        )}
                      </div>
                      <div className="text-muted-foreground text-xs">
                        {asset.type === "dff" && (
                          <>
                            {asset.frame_count} frames • {asset.geometry_count}{" "}
                            geometries • {asset.atomic_count} atomics
                            {asset.samp_model_name && (
                              <> • SA:MP: {asset.samp_model_name}</>
                            )}
                          </>
                        )}
                        {asset.type === "txd" && (
                          <>
                            {asset.texture_count} textures
                            {asset.samp_model_name && (
                              <> • SA:MP: {asset.samp_model_name}</>
                            )}
                          </>
                        )}
                        {asset.type === "col" && (
                          <>
                            {asset.model_count} models • {asset.version}
                            {asset.samp_model_name && (
                              <> • SA:MP: {asset.samp_model_name}</>
                            )}
                          </>
                        )}
                        {asset.type === "ipl" && (
                          <>
                            {asset.instance_count} instances •{" "}
                            {asset.zone_count} zones
                            {asset.samp_model_name && (
                              <> • SA:MP: {asset.samp_model_name}</>
                            )}
                          </>
                        )}
                      </div>
                    </div>
                  </div>
                  <Button
                    className="mr-1 h-6 w-6 p-0"
                    onClick={() => {
                      // Add asset to scene with SA:MP model ID
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

                      // Use the scene store to add the object
                      const { addObject } = useSceneStore.getState();
                      addObject(sceneObject);
                      console.log(
                        `Added ${asset.type} to scene with ${asset.samp_model_id ? `SA:MP ID ${asset.samp_model_id}` : "no SA:MP ID"}`
                      );
                    }}
                    size="sm"
                    title="Add to Scene"
                    variant="ghost"
                  >
                    <Upload className="h-3 w-3" />
                  </Button>
                  <Button
                    className="h-6 w-6 p-0"
                    onClick={() => removeImportedAsset(assetId)}
                    size="sm"
                    title="Remove Asset"
                    variant="ghost"
                  >
                    <X className="h-3 w-3" />
                  </Button>
                </div>

                {expandedAssets.has(assetId) && (
                  <div className="mt-3">
                    <div className="space-y-1 text-xs">
                      <div className="flex justify-between">
                        <span>File:</span>
                        <span className="font-mono text-muted-foreground">
                          {asset.file_path}
                        </span>
                      </div>
                      <div className="flex justify-between">
                        <span>Imported:</span>
                        <span className="text-muted-foreground">
                          {new Date(asset.loaded_at).toLocaleString()}
                        </span>
                      </div>

                      {asset.type === "dff" && (
                        <>
                          <div className="flex justify-between">
                            <span>RW Version:</span>
                            <span className="text-muted-foreground">
                              {asset.rw_version}
                            </span>
                          </div>
                          <div className="flex justify-between">
                            <span>Materials:</span>
                            <span className="text-muted-foreground">
                              {asset.material_count}
                            </span>
                          </div>
                          <div className="flex justify-between">
                            <span>Textures:</span>
                            <span className="text-muted-foreground">
                              {asset.texture_count}
                            </span>
                          </div>
                          {asset.samp_model_id && (
                            <>
                              <div className="flex justify-between">
                                <span>SA:MP ID:</span>
                                <span className="font-bold text-green-600">
                                  {asset.samp_model_id}
                                </span>
                              </div>
                              <div className="flex justify-between">
                                <span>SA:MP Name:</span>
                                <span className="text-green-600">
                                  {asset.samp_model_name}
                                </span>
                              </div>
                            </>
                          )}
                        </>
                      )}

                      {asset.type === "txd" && (
                        <>
                          {asset.samp_model_id && (
                            <>
                              <div className="flex justify-between">
                                <span>SA:MP ID:</span>
                                <span className="font-bold text-green-600">
                                  {asset.samp_model_id}
                                </span>
                              </div>
                              <div className="flex justify-between">
                                <span>SA:MP Name:</span>
                                <span className="text-green-600">
                                  {asset.samp_model_name}
                                </span>
                              </div>
                            </>
                          )}
                          {asset.textures.length > 0 && (
                            <div className="mt-2">
                              <div className="mb-1 font-medium">Textures:</div>
                              <div className="max-h-32 space-y-1 overflow-y-auto">
                                {asset.textures.slice(0, 10).map((texture) => (
                                  <div
                                    className="flex justify-between text-xs"
                                    key={`${assetId}_texture_${texture.name}`}
                                  >
                                    <span className="font-mono">
                                      {texture.name}
                                    </span>
                                    <span className="text-muted-foreground">
                                      {texture.width}×{texture.height}{" "}
                                      {texture.format}
                                    </span>
                                  </div>
                                ))}
                                {asset.textures.length > 10 && (
                                  <div className="py-1 text-center text-muted-foreground text-xs">
                                    ... and {asset.textures.length - 10} more
                                    textures
                                  </div>
                                )}
                              </div>
                            </div>
                          )}
                        </>
                      )}

                      {asset.type === "col" && (
                        <>
                          {asset.samp_model_id && (
                            <>
                              <div className="flex justify-between">
                                <span>SA:MP ID:</span>
                                <span className="font-bold text-green-600">
                                  {asset.samp_model_id}
                                </span>
                              </div>
                              <div className="flex justify-between">
                                <span>SA:MP Name:</span>
                                <span className="text-green-600">
                                  {asset.samp_model_name}
                                </span>
                              </div>
                            </>
                          )}
                          {asset.models.length > 0 && (
                            <div className="mt-2">
                              <div className="mb-1 font-medium">Models:</div>
                              <div className="max-h-32 space-y-1 overflow-y-auto">
                                {asset.models.slice(0, 10).map((model) => (
                                  <div
                                    className="flex justify-between text-xs"
                                    key={`${assetId}_model_${model.name}`}
                                  >
                                    <span className="font-mono">
                                      {model.name}
                                    </span>
                                    <span className="text-muted-foreground">
                                      {model.vertex_count} verts •{" "}
                                      {model.face_count} faces
                                    </span>
                                  </div>
                                ))}
                                {asset.models.length > 10 && (
                                  <div className="py-1 text-center text-muted-foreground text-xs">
                                    ... and {asset.models.length - 10} more
                                    models
                                  </div>
                                )}
                              </div>
                            </div>
                          )}
                        </>
                      )}

                      {asset.type === "ipl" && (
                        <>
                          {asset.samp_model_id && (
                            <>
                              <div className="flex justify-between">
                                <span>SA:MP ID:</span>
                                <span className="font-bold text-green-600">
                                  {asset.samp_model_id}
                                </span>
                              </div>
                              <div className="flex justify-between">
                                <span>SA:MP Name:</span>
                                <span className="text-green-600">
                                  {asset.samp_model_name}
                                </span>
                              </div>
                            </>
                          )}
                          <div className="flex justify-between">
                            <span>Zones:</span>
                            <span className="text-muted-foreground">
                              {asset.zone_count}
                            </span>
                          </div>
                          <div className="flex justify-between">
                            <span>Cull zones:</span>
                            <span className="text-muted-foreground">
                              {asset.cull_count}
                            </span>
                          </div>
                          <div className="flex justify-between">
                            <span>Pick zones:</span>
                            <span className="text-muted-foreground">
                              {asset.pick_count}
                            </span>
                          </div>
                        </>
                      )}
                    </div>
                  </div>
                )}
              </Card>
            ))}
          </div>
        </div>
      )}

      {/* SA:MP Model Search Results */}
      {modelSearchResults.length > 0 && (
        <div className="mb-4">
          <h4 className="mb-2 font-medium text-sm">
            SA:MP Model Search Results ({modelSearchResults.length})
          </h4>
          <div className="max-h-64 space-y-2 overflow-y-auto">
            {modelSearchResults.map((model) => (
              <Card className="p-3" key={model.id}>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="font-medium text-primary text-sm">
                      ID: {model.id}
                    </div>
                    <div className="font-mono text-sm">{model.name}</div>
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="text-muted-foreground text-xs">
                      Radius: {model.radius}
                    </div>
                    <Button
                      onClick={() => {
                        // Copy model ID to clipboard or use it somewhere
                        navigator.clipboard?.writeText(model.id.toString());
                        console.log(`Copied model ID ${model.id} to clipboard`);
                      }}
                      size="sm"
                      variant="outline"
                    >
                      Copy ID
                    </Button>
                  </div>
                </div>
                <div className="mt-2 text-muted-foreground text-xs">
                  <div>DFF: {model.dff}</div>
                  <div>TXD: {model.txd}</div>
                </div>
              </Card>
            ))}
          </div>
        </div>
      )}

      {/* Drop zone */}
      <div className="flex-1">
        <div
          className={cn(
            "flex h-full flex-col items-center justify-center rounded-lg border-2 border-dashed p-8 transition-colors",
            dragOver ? "border-primary bg-primary/5" : "border-muted",
            isImporting && "pointer-events-none opacity-50"
          )}
          onDragLeave={() => setDragOver(false)}
          onDragOver={(e) => {
            e.preventDefault();
            setDragOver(true);
          }}
          onDrop={handleFileDrop}
        >
          <div className={cn("mb-4 rounded-full bg-muted p-4", config.color)}>
            <config.icon className="h-8 w-8" />
          </div>
          <h3 className="mb-2 font-medium text-lg">
            {isImporting ? "Importing..." : `Import ${config.label} Assets`}
          </h3>
          <p className="mb-4 text-center text-muted-foreground">
            Drag and drop {config.label} files here, or use the buttons above
          </p>

          <p className="text-center text-muted-foreground text-xs">
            Supported formats: {config.accept}
          </p>

          {/* PWN specific info */}
          <div className="mt-4 rounded bg-muted p-3 text-left text-xs">
            <p className="mb-2 font-medium">PWN Script Support:</p>
            <p className="text-muted-foreground">
              Import objects from SA-MP Pawn scripts containing
              CreateDynamicObject calls.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
});

// Materials tab component (placeholder for now)
const MaterialsTab = memo(function MaterialsTab() {
  return <PlaceholderAssetTab assetType="materials" />;
});

// Model card component
interface ModelCardProps {
  model: any;
  isSelected: boolean;
  showThumbnail: boolean;
  onClick: (e: React.MouseEvent) => void;
  onDelete: () => void;
}

function ModelCard({
  model,
  isSelected,
  showThumbnail,
  onClick,
  onDelete,
}: ModelCardProps) {
  const [showDetails, setShowDetails] = useState(false);

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
  };

  return (
    <div
      aria-label={`Model: ${model.name}`}
      className={cn(
        "group relative cursor-pointer rounded-lg border-2 transition-all hover:shadow-md",
        isSelected ? "border-primary shadow-md" : "border-border",
        "aspect-square overflow-hidden"
      )}
      onClick={onClick}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onClick(e as any);
        }
      }}
      onMouseEnter={() => setShowDetails(true)}
      onMouseLeave={() => setShowDetails(false)}
      role="button"
      tabIndex={0}
    >
      {/* Thumbnail */}
      <div className="flex h-full w-full items-center justify-center bg-muted">
        {showThumbnail && model.thumbnail ? (
          <img
            alt={model.name}
            className="h-full w-full object-cover"
            height={256}
            src={model.thumbnail}
            style={{ imageRendering: "pixelated" }}
            width={256}
          />
        ) : (
          <div className="flex flex-col items-center justify-center text-muted-foreground">
            <Box className="mb-2 h-8 w-8" />
            <span className="px-2 text-center text-xs">{model.name}</span>
          </div>
        )}
      </div>

      {/* Format badge */}
      <div className="absolute top-1 left-1 rounded bg-background/80 px-1 py-0.5 font-medium text-xs">
        {model.format.toUpperCase()}
      </div>

      {/* Selection indicator */}
      {isSelected && (
        <div className="absolute top-1 right-1 flex h-4 w-4 items-center justify-center rounded-full bg-primary">
          <div className="h-2 w-2 rounded-full bg-primary-foreground" />
        </div>
      )}

      {/* Details overlay */}
      {showDetails && (
        <div className="absolute inset-0 flex flex-col justify-between bg-black/75 p-2 text-white text-xs">
          <div>
            <div className="truncate font-medium">{model.name}</div>
            <div className="text-muted-foreground">
              {model.format.toUpperCase()}
            </div>
          </div>

          <div className="space-y-1">
            <div>{formatFileSize(model.fileSize)}</div>
            {model.metadata.vertices && (
              <div className="flex items-center gap-1">
                <span>{model.metadata.vertices.toLocaleString()} verts</span>
              </div>
            )}
            {model.metadata.triangles && (
              <div className="flex items-center gap-1">
                <span>{model.metadata.triangles.toLocaleString()} tris</span>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Delete button */}
      <Button
        className="absolute top-1 right-1 h-6 w-6 p-0 opacity-0 transition-opacity group-hover:opacity-100"
        onClick={(e) => {
          e.stopPropagation();
          onDelete();
        }}
        size="sm"
        variant="destructive"
      >
        <X className="h-3 w-3" />
      </Button>
    </div>
  );
}

// Audio card component
interface AudioCardProps {
  audio: any;
  isSelected: boolean;
  showWaveform: boolean;
  currentlyPlaying: string | null;
  onClick: (e: React.MouseEvent) => void;
  onPlayPause: (e: React.MouseEvent) => void;
  onDelete: () => void;
}

function AudioCard({
  audio,
  isSelected,
  showWaveform,
  currentlyPlaying,
  onClick,
  onPlayPause,
  onDelete,
}: AudioCardProps) {
  const [showDetails, setShowDetails] = useState(false);

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
  };

  const formatDuration = (seconds?: number) => {
    if (!seconds) return "0:00";
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  };

  const isPlaying = currentlyPlaying === audio.id;

  return (
    <div
      className={cn(
        "group relative cursor-pointer rounded-lg border-2 p-4 transition-all hover:shadow-md",
        isSelected ? "border-primary bg-primary/5 shadow-md" : "border-border"
      )}
      onClick={onClick}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onClick(e as any);
        }
      }}
      onMouseEnter={() => setShowDetails(true)}
      onMouseLeave={() => setShowDetails(false)}
    >
      {/* Header */}
      <div className="mb-2 flex items-center justify-between">
        <div className="flex min-w-0 flex-1 items-center gap-2">
          <Music className="h-4 w-4 shrink-0 text-muted-foreground" />
          <span className="truncate font-medium text-sm" title={audio.name}>
            {audio.name}
          </span>
        </div>

        {/* Play/Pause button */}
        <Button
          className="h-8 w-8 shrink-0 p-0"
          onClick={onPlayPause}
          size="sm"
          variant="ghost"
        >
          {isPlaying ? (
            <div className="flex gap-0.5">
              <div className="w-1 animate-pulse rounded-sm bg-current" />
              <div
                className="w-1 animate-pulse rounded-sm bg-current"
                style={{ animationDelay: "0.1s" }}
              />
              <div
                className="w-1 animate-pulse rounded-sm bg-current"
                style={{ animationDelay: "0.2s" }}
              />
            </div>
          ) : (
            <div className="ml-0.5 h-0 w-0 border-t border-t-transparent border-b border-b-transparent border-l-2 border-l-current" />
          )}
        </Button>
      </div>

      {/* Waveform visualization */}
      {showWaveform && (
        <div className="mb-3 h-8 overflow-hidden rounded bg-muted">
          <div className="flex h-full items-end gap-0.5 p-1">
            {Array.from({ length: 20 }, (_, i) => (
              <div
                className={cn(
                  "rounded-sm bg-primary/60 transition-all duration-300",
                  isPlaying ? "animate-pulse" : ""
                )}
                key={`${audio.id}-wave-${i}`}
                style={{
                  height: `${Math.random() * 100}%`,
                  width: "2px",
                  animationDelay: `${i * 0.1}s`,
                }}
              />
            ))}
          </div>
        </div>
      )}

      {/* Metadata */}
      <div className="space-y-1 text-muted-foreground text-xs">
        <div className="flex justify-between">
          <span>{audio.format.toUpperCase()}</span>
          <span>{formatDuration(audio.duration)}</span>
        </div>
        <div className="flex justify-between">
          <span>{formatFileSize(audio.fileSize)}</span>
          {audio.channels && (
            <span>{audio.channels === 1 ? "Mono" : "Stereo"}</span>
          )}
        </div>
        {audio.sampleRate && (
          <div className="text-right">{audio.sampleRate / 1000}kHz</div>
        )}
      </div>

      {/* Selection indicator */}
      {isSelected && (
        <div className="absolute top-2 right-2 flex h-4 w-4 items-center justify-center rounded-full bg-primary">
          <div className="h-2 w-2 rounded-full bg-primary-foreground" />
        </div>
      )}

      {/* Details overlay */}
      {showDetails && (
        <div className="absolute inset-0 flex flex-col justify-between rounded-lg bg-black/75 p-3 text-white text-xs">
          <div>
            <div className="mb-1 truncate font-medium">{audio.name}</div>
            <div className="text-muted-foreground">
              {audio.format.toUpperCase()} • {formatDuration(audio.duration)}
            </div>
          </div>

          <div className="space-y-1">
            <div className="flex justify-between">
              <span>Size:</span>
              <span>{formatFileSize(audio.fileSize)}</span>
            </div>
            {audio.channels && (
              <div className="flex justify-between">
                <span>Channels:</span>
                <span>{audio.channels === 1 ? "Mono" : "Stereo"}</span>
              </div>
            )}
            {audio.sampleRate && (
              <div className="flex justify-between">
                <span>Sample Rate:</span>
                <span>{audio.sampleRate}Hz</span>
              </div>
            )}
            {audio.bitrate && (
              <div className="flex justify-between">
                <span>Bitrate:</span>
                <span>{audio.bitrate}kbps</span>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Delete button */}
      <Button
        className="absolute top-2 right-10 h-6 w-6 p-0 opacity-0 transition-opacity group-hover:opacity-100"
        onClick={(e) => {
          e.stopPropagation();
          onDelete();
        }}
        size="sm"
        variant="destructive"
      >
        <X className="h-3 w-3" />
      </Button>
    </div>
  );
}

export const AssetsPanel = memo(function AssetsPanel({
  className,
}: AssetsPanelProps) {
  const [activeTab, setActiveTab] = useState<AssetType>("textures");

  const handleTabChange = useCallback((value: string) => {
    setActiveTab(value as AssetType);
  }, []);

  return (
    <Card className={cn("flex h-full w-full flex-col", className)}>
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center gap-2">
          <Folder className="h-5 w-5" />
          Assets
        </CardTitle>
      </CardHeader>

      <CardContent className="flex flex-1 flex-col gap-4 p-4 pt-0">
        <Tabs
          className="flex h-full flex-col"
          onValueChange={handleTabChange}
          value={activeTab}
        >
          <TabsList className="mb-42 grid w-full grid-cols-3">
            {Object.entries(ASSET_TYPES).map(([key, config]) => (
              <TabsTrigger
                className="flex flex-col gap-1 px-2 py-2 text-xs"
                key={key}
                value={key}
              >
                <config.icon className="h-4 w-4" />
                <span className="hidden sm:inline">{config.label}</span>
              </TabsTrigger>
            ))}
          </TabsList>

          <TabsContent className="mt-0 flex-1" value="models">
            <ScrollArea className="h-full">
              <ModelsTab />
            </ScrollArea>
          </TabsContent>

          <TabsContent className="mt-0 flex-1 overflow-hidden" value="textures">
            <ScrollArea className="h-full overflow-x-auto">
              <TextureLibrary />
            </ScrollArea>
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="materials">
            <ScrollArea className="h-full">
              <MaterialsTab />
            </ScrollArea>
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="audio">
            <ScrollArea className="h-full">
              <AudioTab />
            </ScrollArea>
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="scripts">
            <ScrollArea className="h-full">
              <ScriptsTab />
            </ScrollArea>
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="fonts">
            <ScrollArea className="h-full">
              <FontsTab />
            </ScrollArea>
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="prefabs">
            <ScrollArea className="h-full">
              <PrefabsTab />
            </ScrollArea>
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="renderware">
            <ScrollArea className="h-full">
              <RenderWareTab />
            </ScrollArea>
          </TabsContent>

          <TabsContent className="mt-0 flex-1 overflow-hidden" value="remote">
            <ScrollArea className="h-full overflow-x-auto">
              <RemoteAssetsTab />
            </ScrollArea>
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  );
});
