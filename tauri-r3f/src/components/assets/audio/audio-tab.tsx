import { memo, useCallback, useMemo, useState } from "react";
import { Music, Filter, Search, SortAsc, SortDesc, Upload } from "lucide-react";
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
import { useAudioStore, type AudioFormat } from "~/stores/audio-store";
import { AudioCard } from "./audio-card";

// Audio tab component
export const AudioTab = memo(function AudioTab() {
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
