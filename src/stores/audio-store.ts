import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";

// Regex for file extension extraction
const FILE_EXTENSION_REGEX = /\.[^/.]+$/;

export type AudioFormat =
  | "wav"
  | "mp3"
  | "ogg"
  | "aac"
  | "flac"
  | "m4a"
  | "wma";

export interface AudioAsset {
  id: string;
  name: string;
  filePath: string;
  format: AudioFormat;
  fileSize: number;
  lastModified: number;
  duration?: number; // in seconds
  sampleRate?: number;
  channels?: number; // 1 = mono, 2 = stereo
  bitrate?: number;
  thumbnail?: string; // waveform visualization
  tags: string[];
  loaded: boolean;
  loading: boolean;
  error?: string;
}

export interface AudioState {
  // Audio library
  audioFiles: AudioAsset[];

  // UI state
  libraryVisible: boolean;
  filterFormat: string;
  filterSearch: string;
  sortBy: "name" | "format" | "size" | "date" | "duration";
  sortOrder: "asc" | "desc";
  showWaveforms: boolean;

  // Selected audio files
  selectedAudioIds: Set<string>;

  // Playback state
  currentlyPlaying: string | null;
  playbackVolume: number;
}

export interface AudioActions {
  // Audio management
  addAudioFile: (audio: Omit<AudioAsset, "id">) => string;
  removeAudioFile: (id: string) => void;
  updateAudioFile: (id: string, updates: Partial<AudioAsset>) => void;

  // Bulk operations
  bulkDeleteAudioFiles: (ids: string[]) => void;
  clearAllAudioFiles: () => void;

  // Import/Export
  importAudioFiles: (files: File[]) => Promise<void>;
  exportAudioFile: (id: string) => Promise<AudioAsset | null>;

  // UI actions
  setLibraryVisible: (visible: boolean) => void;
  setFilterFormat: (format: string) => void;
  setFilterSearch: (search: string) => void;
  setSortBy: (sortBy: "name" | "format" | "size" | "date" | "duration") => void;
  setSortOrder: (order: "asc" | "desc") => void;
  setShowWaveforms: (show: boolean) => void;

  // Selection
  selectAudioFile: (id: string, multiSelect?: boolean) => void;
  deselectAudioFile: (id: string) => void;
  clearSelection: () => void;

  // Playback
  playAudio: (id: string) => void;
  pauseAudio: () => void;
  stopAudio: () => void;
  setVolume: (volume: number) => void;

  // Utility functions
  getAudioFileById: (id: string) => AudioAsset | undefined;
  getFilteredAudioFiles: () => AudioAsset[];
  getAudioStats: () => AudioStats;
}

export interface AudioStats {
  totalFiles: number;
  totalFileSize: number;
  formats: Record<AudioFormat, number>;
  totalDuration: number;
  loadedFiles: number;
  loadingFiles: number;
  failedFiles: number;
}

export const useAudioStore = create<AudioState & AudioActions>()(
  subscribeWithSelector((set, get) => ({
    // Initial state
    audioFiles: [],
    libraryVisible: true,
    filterFormat: "",
    filterSearch: "",
    sortBy: "name",
    sortOrder: "asc",
    showWaveforms: true,
    selectedAudioIds: new Set(),
    currentlyPlaying: null,
    playbackVolume: 0.7,

    // Audio management
    addAudioFile: (audioData) => {
      const id = `audio_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      const audio: AudioAsset = {
        ...audioData,
        id,
      };

      set((state) => ({
        audioFiles: [...state.audioFiles, audio],
      }));

      return id;
    },

    removeAudioFile: (id) => {
      set((state) => ({
        audioFiles: state.audioFiles.filter((a) => a.id !== id),
        selectedAudioIds: new Set(
          Array.from(state.selectedAudioIds).filter(
            (selectedId) => selectedId !== id
          )
        ),
        currentlyPlaying:
          state.currentlyPlaying === id ? null : state.currentlyPlaying,
      }));
    },

    updateAudioFile: (id, updates) => {
      set((state) => ({
        audioFiles: state.audioFiles.map((audio) =>
          audio.id === id ? { ...audio, ...updates } : audio
        ),
      }));
    },

    // Bulk operations
    bulkDeleteAudioFiles: (ids) => {
      const idSet = new Set(ids);
      set((state) => ({
        audioFiles: state.audioFiles.filter((a) => !idSet.has(a.id)),
        selectedAudioIds: new Set(
          Array.from(state.selectedAudioIds).filter((id) => !idSet.has(id))
        ),
        currentlyPlaying:
          state.currentlyPlaying && idSet.has(state.currentlyPlaying)
            ? null
            : state.currentlyPlaying,
      }));
    },

    clearAllAudioFiles: () => {
      set({
        audioFiles: [],
        selectedAudioIds: new Set(),
        currentlyPlaying: null,
      });
    },

    // Import/Export
    importAudioFiles: async (files) => {
      const supportedFormats: AudioFormat[] = [
        "wav",
        "mp3",
        "ogg",
        "aac",
        "flac",
        "m4a",
        "wma",
      ];

      for (const file of files) {
        const extension = file.name
          .split(".")
          .pop()
          ?.toLowerCase() as AudioFormat;
        if (!(extension && supportedFormats.includes(extension))) {
          console.warn(`Unsupported audio format: ${extension}`);
          continue;
        }

        try {
          // Set loading state
          const tempId = `temp_${Date.now()}_${Math.random()}`;
          const loadingAudio: AudioAsset = {
            id: tempId,
            name: file.name.replace(FILE_EXTENSION_REGEX, ""),
            filePath: file.name,
            format: extension,
            fileSize: file.size,
            lastModified: file.lastModified,
            loaded: false,
            loading: true,
            tags: [],
          };

          set((state) => ({
            audioFiles: [...state.audioFiles, loadingAudio],
          }));

          // Simulate loading and metadata extraction (in real implementation, this would parse the audio file)
          await new Promise((resolve) => setTimeout(resolve, 1000));

          // Create final audio file with mock metadata
          const audioData: Omit<AudioAsset, "id"> = {
            name: file.name.replace(FILE_EXTENSION_REGEX, ""),
            filePath: file.name,
            format: extension,
            fileSize: file.size,
            lastModified: file.lastModified,
            duration: Math.floor(Math.random() * 300) + 10, // 10-310 seconds
            sampleRate: [44_100, 48_000, 96_000][Math.floor(Math.random() * 3)],
            channels: Math.random() > 0.3 ? 2 : 1, // 70% stereo, 30% mono
            bitrate: [128, 192, 256, 320][Math.floor(Math.random() * 4)],
            loaded: true,
            loading: false,
            tags: [],
          };

          // Remove loading audio and add final audio
          set((state) => ({
            audioFiles: [
              ...state.audioFiles.filter((a) => a.id !== tempId),
              { ...audioData, id: tempId },
            ],
          }));
        } catch (error) {
          console.error(`Failed to import audio ${file.name}:`, error);
          // Update audio with error state
          set((state) => ({
            audioFiles: state.audioFiles.map((audio) =>
              audio.filePath === file.name
                ? { ...audio, loading: false, error: (error as Error).message }
                : audio
            ),
          }));
        }
      }
    },

    exportAudioFile: (id) => {
      const audio = get().getAudioFileById(id);
      return Promise.resolve(audio || null);
    },

    // UI actions
    setLibraryVisible: (visible) => set({ libraryVisible: visible }),
    setFilterFormat: (format) => set({ filterFormat: format }),
    setFilterSearch: (search) => set({ filterSearch: search }),
    setSortBy: (sortBy) => set({ sortBy }),
    setSortOrder: (order) => set({ sortOrder: order }),
    setShowWaveforms: (show) => set({ showWaveforms: show }),

    // Selection
    selectAudioFile: (id, multiSelect = false) => {
      set((state) => {
        const newSelection = multiSelect
          ? new Set(state.selectedAudioIds)
          : new Set<string>();

        if (newSelection.has(id)) {
          newSelection.delete(id);
        } else {
          newSelection.add(id);
        }

        return { selectedAudioIds: newSelection };
      });
    },

    deselectAudioFile: (id) => {
      set((state) => ({
        selectedAudioIds: new Set(
          Array.from(state.selectedAudioIds).filter(
            (selectedId) => selectedId !== id
          )
        ),
      }));
    },

    clearSelection: () => {
      set({ selectedAudioIds: new Set() });
    },

    // Playback
    playAudio: (id) => {
      set({ currentlyPlaying: id });
    },

    pauseAudio: () => {
      // In a real implementation, this would pause the audio
      set({ currentlyPlaying: null });
    },

    stopAudio: () => {
      set({ currentlyPlaying: null });
    },

    setVolume: (volume) => {
      set({ playbackVolume: Math.max(0, Math.min(1, volume)) });
    },

    // Utility functions
    getAudioFileById: (id) => {
      return get().audioFiles.find((audio) => audio.id === id);
    },

    getFilteredAudioFiles: () => {
      const { audioFiles, filterFormat, filterSearch, sortBy, sortOrder } =
        get();

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
    },

    getAudioStats: (): AudioStats => {
      const audioFiles = get().audioFiles;
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
        totalFileSize: audioFiles.reduce(
          (sum, audio) => sum + audio.fileSize,
          0
        ),
        formats,
        totalDuration: audioFiles.reduce(
          (sum, audio) => sum + (audio.duration || 0),
          0
        ),
        loadedFiles: audioFiles.filter((a) => a.loaded).length,
        loadingFiles: audioFiles.filter((a) => a.loading).length,
        failedFiles: audioFiles.filter((a) => a.error).length,
      };
    },
  }))
);

// Convenience hooks
export const useFilteredAudioFiles = () =>
  useAudioStore((state) => state.getFilteredAudioFiles());
export const useAudioStats = () =>
  useAudioStore((state) => state.getAudioStats());
