import { invoke } from "@tauri-apps/api/core";
import { create } from "zustand";
import { immer } from "zustand/middleware/immer";
import { toast } from "../lib/toast";
import { EditorState, MapData, MapObject, Vector3 } from "../types/map";

interface EditorStore extends EditorState {
  // Map data
  mapData: MapData | null;
  pendingObjectType: "cube" | "sphere" | "plane" | null;

  // Actions
  setMapData: (mapData: MapData | null) => void;
  addObject: (object: MapObject) => void;
  updateObject: (id: string, updates: Partial<MapObject>) => void;
  deleteObject: (id: string) => void;
  selectObject: (id: string | null) => void;

  // Tool actions
  setTool: (tool: EditorState["tool"]) => void;
  setGridVisible: (visible: boolean) => void;
  setSnapToGrid: (snap: boolean) => void;
  setGridSize: (size: number) => void;
  setPendingObjectType: (type: "cube" | "sphere" | "plane" | null) => void;

  // Camera actions
  setCameraPosition: (position: Vector3) => void;
  setCameraTarget: (target: Vector3) => void;
  setCameraZoom: (zoom: number) => void;

  // Utility actions
  reset: () => void;
  syncWithBackend: () => Promise<void>;
}

const initialState: EditorState = {
  selectedObjectId: null,
  tool: "select",
  isGridVisible: true,
  snapToGrid: true,
  gridSize: 1,
  camera: {
    position: { x: 10, y: 10, z: 10 },
    target: { x: 0, y: 0, z: 0 },
    zoom: 1,
  },
};

export const useEditorStore = create<EditorStore>()(
  immer((set, _get) => ({
    ...initialState,
    mapData: null,
    pendingObjectType: null,

    setMapData: (mapData) =>
      set((state) => {
        state.mapData = mapData;
      }),

    addObject: (object) =>
      set((state) => {
        if (state.mapData) {
          state.mapData.objects.push(object);
          state.mapData.updatedAt = new Date().toISOString();
        }
      }),

    updateObject: (id, updates) =>
      set((state) => {
        if (state.mapData) {
          const object = state.mapData.objects.find((obj) => obj.id === id);
          if (object) {
            Object.assign(object, updates);
            state.mapData.updatedAt = new Date().toISOString();
          }
        }
      }),

    deleteObject: (id) =>
      set((state) => {
        if (state.mapData) {
          state.mapData.objects = state.mapData.objects.filter((obj) => obj.id !== id);
          state.mapData.updatedAt = new Date().toISOString();
          if (state.selectedObjectId === id) {
            state.selectedObjectId = null;
          }
        }
      }),

    selectObject: (id) =>
      set((state) => {
        state.selectedObjectId = id;
      }),

    setTool: (tool) =>
      set((state) => {
        state.tool = tool;
      }),

    setGridVisible: (visible) =>
      set((state) => {
        state.isGridVisible = visible;
      }),

    setSnapToGrid: (snap) =>
      set((state) => {
        state.snapToGrid = snap;
      }),

    setGridSize: (size) =>
      set((state) => {
        state.gridSize = size;
      }),

    setPendingObjectType: (type) =>
      set((state) => {
        state.pendingObjectType = type;
      }),

    setCameraPosition: (position) =>
      set((state) => {
        state.camera.position = position;
      }),

    setCameraTarget: (target) =>
      set((state) => {
        state.camera.target = target;
      }),

    setCameraZoom: (zoom) =>
      set((state) => {
        state.camera.zoom = zoom;
      }),

    reset: () =>
      set((state) => {
        Object.assign(state, initialState);
        state.mapData = null;
        state.pendingObjectType = null;
      }),

    syncWithBackend: async () => {
      const state = useEditorStore.getState();
      if (state.mapData) {
        try {
          await invoke("sync_map_data", { mapData: state.mapData });
        } catch (error) {
          console.error("Failed to sync map data:", error);
          toast.error("Failed to sync map data");
        }
      }
    },
  })),
);
