import type * as THREE from "three";
import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";
import { applyLightingPreset } from "~/utils/lighting-presets";

export type TransformTool = "select" | "move" | "rotate" | "scale";

export type LightType =
  | "directional"
  | "point"
  | "spot"
  | "ambient"
  | "hemisphere";

export interface SceneLight {
  id: string;
  name: string;
  type: LightType;
  color: string;
  intensity: number;
  position: [number, number, number];
  target?: [number, number, number]; // For directional and spot lights
  visible: boolean;
  castShadow: boolean;

  // Point light specific
  distance?: number; // 0 = infinite
  decay?: number; // 2 = realistic

  // Spot light specific
  angle?: number; // radians
  penumbra?: number; // 0-1

  // Hemisphere light specific
  groundColor?: string;

  // Shadow properties
  shadowBias?: number;
  shadowMapSize?: number;
  shadowNear?: number;
  shadowFar?: number;
  shadowRadius?: number;
}

export interface SceneObject {
  id: string;
  name: string;
  type: "cube" | "sphere" | "plane" | "imported";
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
  color: string;
  visible: boolean;
  geometry?: THREE.BufferGeometry;
  material?: THREE.Material;
  importedModel?: THREE.Object3D;
  initialScale?: number; // For imported models, stores the initial scaling factor
}

export interface SceneState {
  objects: SceneObject[];
  lights: SceneLight[];
  selectedObjectIds: string[];
  selectedLightIds: string[];
  activeTool: TransformTool;
  cameraPosition: [number, number, number];
  cameraTarget: [number, number, number];
  gridVisible: boolean;
  snapToGrid: boolean;
  gridSize: number;
  placementMode: {
    active: boolean;
    objectType: SceneObject["type"] | null;
    previewPosition: [number, number, number] | null;
  };
  lightsVisible: boolean;
  transformDragging: boolean; // Track when transform controls are being dragged
  // File management
  currentFilePath: string | null;
  sceneMetadata: {
    name: string;
    description?: string;
    isModified: boolean;
    lastSavedAt?: Date;
  };
}

export interface SceneActions {
  // Object management
  addObject: (
    typeOrObject: SceneObject["type"] | SceneObject,
    position?: [number, number, number]
  ) => void;
  removeObject: (id: string) => void;
  updateObject: (id: string, updates: Partial<SceneObject>) => void;
  duplicateObject: (id: string) => void;

  // Light management
  addLight: (type: LightType, position?: [number, number, number]) => void;
  removeLight: (id: string) => void;
  updateLight: (id: string, updates: Partial<SceneLight>) => void;
  duplicateLight: (id: string) => void;

  // Selection
  selectObject: (id: string, multiSelect?: boolean) => void;
  selectLight: (id: string, multiSelect?: boolean) => void;
  clearSelection: () => void;
  selectAll: () => void;

  // Tools
  setActiveTool: (tool: TransformTool) => void;
  setTransformDragging: (dragging: boolean) => void;

  // Camera
  setCameraPosition: (position: [number, number, number]) => void;
  setCameraTarget: (target: [number, number, number]) => void;

  // Grid
  setGridVisible: (visible: boolean) => void;
  setSnapToGrid: (snap: boolean) => void;
  setGridSize: (size: number) => void;

  // Lights
  setLightsVisible: (visible: boolean) => void;
  applyLightingPreset: (presetId: string) => void;

  // Scene management
  clearScene: () => void;

  // Object placement
  startPlacement: (objectType: SceneObject["type"]) => void;
  updatePlacementPreview: (position: [number, number, number] | null) => void;
  confirmPlacement: () => void;
  cancelPlacement: () => void;

  // File management
  setCurrentFilePath: (path: string | null) => void;
  setSceneMetadata: (metadata: Partial<SceneState["sceneMetadata"]>) => void;
  markSceneModified: () => void;
  markSceneSaved: () => void;
  loadScene: (sceneState: Partial<SceneState>) => void;
}

const createDefaultObject = (
  type: SceneObject["type"],
  position: [number, number, number] = [0, 0, 0]
): SceneObject => {
  const baseObject: Omit<SceneObject, "id" | "name"> = {
    type,
    position,
    rotation: [0, 0, 0],
    scale: [1, 1, 1],
    color: "#ffffff",
    visible: true,
  };

  return {
    ...baseObject,
    id: `object_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    name: `${type.charAt(0).toUpperCase() + type.slice(1)} ${Date.now()}`,
  };
};

const createDefaultLight = (
  type: LightType,
  position: [number, number, number] = [0, 0, 0]
): SceneLight => {
  const baseLight: Omit<SceneLight, "id" | "name"> = {
    type,
    color: "#ffffff",
    intensity: 1,
    position,
    visible: true,
    castShadow: true,
  };

  switch (type) {
    case "directional":
      return {
        ...baseLight,
        target: [0, 0, 0],
        shadowBias: -0.0001,
        shadowMapSize: 2048,
        shadowNear: 0.1,
        shadowFar: 100,
        shadowRadius: 8,
        id: `light_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        name: `Directional Light ${Date.now()}`,
      };
    case "point":
      return {
        ...baseLight,
        distance: 0,
        decay: 2,
        shadowBias: 0.0001,
        shadowMapSize: 1024,
        shadowNear: 0.1,
        shadowFar: 50,
        shadowRadius: 4,
        id: `light_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        name: `Point Light ${Date.now()}`,
      };
    case "spot":
      return {
        ...baseLight,
        target: [0, 0, 0],
        angle: Math.PI / 6,
        penumbra: 0.1,
        distance: 0,
        decay: 2,
        shadowBias: 0.0001,
        shadowMapSize: 1024,
        shadowNear: 0.1,
        shadowFar: 50,
        shadowRadius: 4,
        id: `light_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        name: `Spot Light ${Date.now()}`,
      };
    case "ambient":
      return {
        ...baseLight,
        intensity: 0.6,
        id: `light_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        name: `Ambient Light ${Date.now()}`,
      };
    case "hemisphere":
      return {
        ...baseLight,
        groundColor: "#444444",
        intensity: 0.6,
        id: `light_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        name: `Hemisphere Light ${Date.now()}`,
      };
    default:
      throw new Error(`Unknown light type: ${type}`);
  }
};

export const useSceneStore = create<SceneState & SceneActions>()(
  subscribeWithSelector((set, get) => ({
    // Initial state
    objects: [],
    lights: [
      createDefaultLight("ambient", [0, 0, 0]),
      createDefaultLight("directional", [10, 10, 5]),
    ],
    selectedObjectIds: [],
    selectedLightIds: [],
    activeTool: "select",
    cameraPosition: [5, 5, 5],
    cameraTarget: [0, 0, 0],
    gridVisible: true,
    snapToGrid: false,
    gridSize: 1,
    placementMode: {
      active: false,
      objectType: null,
      previewPosition: null,
    },
    lightsVisible: true,
    transformDragging: false,
    currentFilePath: null,
    sceneMetadata: {
      name: "Untitled Scene",
      isModified: false,
    },

    // Actions
    addObject: (typeOrObject, position) => {
      let newObject: SceneObject;

      if (typeof typeOrObject === "string") {
        // Create a new object from type
        newObject = createDefaultObject(typeOrObject, position);
      } else {
        // Use the provided object (for imported models)
        newObject = typeOrObject;
      }

      set((state) => ({
        objects: [...state.objects, newObject],
        selectedObjectIds: [newObject.id],
      }));

      get().markSceneModified();
    },

    removeObject: (id) => {
      set((state) => ({
        objects: state.objects.filter((obj) => obj.id !== id),
        selectedObjectIds: state.selectedObjectIds.filter(
          (selectedId) => selectedId !== id
        ),
      }));

      get().markSceneModified();
    },

    updateObject: (id, updates) => {
      set((state) => ({
        objects: state.objects.map((obj) =>
          obj.id === id ? { ...obj, ...updates } : obj
        ),
      }));

      get().markSceneModified();
    },

    duplicateObject: (id) => {
      const originalObject = get().objects.find((obj) => obj.id === id);
      if (!originalObject) {
        return;
      }

      const duplicatedObject = {
        ...originalObject,
        id: `object_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        name: `${originalObject.name} Copy`,
        position: [
          originalObject.position[0] + 1,
          originalObject.position[1],
          originalObject.position[2] + 1,
        ] as [number, number, number],
      };

      set((state) => ({
        objects: [...state.objects, duplicatedObject],
        selectedObjectIds: [duplicatedObject.id],
      }));

      get().markSceneModified();
    },

    // Light management actions
    addLight: (type, position) => {
      const newLight = createDefaultLight(type, position);
      set((state) => ({
        lights: [...state.lights, newLight],
        selectedLightIds: [newLight.id],
        selectedObjectIds: [], // Clear object selection when selecting light
      }));

      get().markSceneModified();
    },

    removeLight: (id) => {
      set((state) => ({
        lights: state.lights.filter((light) => light.id !== id),
        selectedLightIds: state.selectedLightIds.filter(
          (selectedId) => selectedId !== id
        ),
      }));

      get().markSceneModified();
    },

    updateLight: (id, updates) => {
      set((state) => ({
        lights: state.lights.map((light) =>
          light.id === id ? { ...light, ...updates } : light
        ),
      }));

      get().markSceneModified();
    },

    duplicateLight: (id) => {
      const originalLight = get().lights.find((light) => light.id === id);
      if (!originalLight) {
        return;
      }

      const duplicatedLight = {
        ...originalLight,
        id: `light_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        name: `${originalLight.name} Copy`,
        position: [
          originalLight.position[0] + 1,
          originalLight.position[1],
          originalLight.position[2] + 1,
        ] as [number, number, number],
      };

      set((state) => ({
        lights: [...state.lights, duplicatedLight],
        selectedLightIds: [duplicatedLight.id],
        selectedObjectIds: [], // Clear object selection when selecting light
      }));

      get().markSceneModified();
    },

    selectObject: (id, multiSelect = false) => {
      set((state) => {
        if (multiSelect) {
          // Multi-select: toggle selection
          const isSelected = state.selectedObjectIds.includes(id);
          if (isSelected) {
            return {
              selectedObjectIds: state.selectedObjectIds.filter(
                (selectedId) => selectedId !== id
              ),
            };
          }
          return {
            selectedObjectIds: [...state.selectedObjectIds, id],
            selectedLightIds: [], // Clear light selection when selecting object
          };
        }
        // Single select: replace selection
        return {
          selectedObjectIds: [id],
          selectedLightIds: [], // Clear light selection when selecting object
        };
      });
    },

    selectLight: (id, multiSelect = false) => {
      set((state) => {
        if (multiSelect) {
          // Multi-select: toggle selection
          const isSelected = state.selectedLightIds.includes(id);
          if (isSelected) {
            return {
              selectedLightIds: state.selectedLightIds.filter(
                (selectedId) => selectedId !== id
              ),
            };
          }
          return {
            selectedLightIds: [...state.selectedLightIds, id],
            selectedObjectIds: [], // Clear object selection when selecting light
          };
        }
        // Single select: replace selection
        return {
          selectedLightIds: [id],
          selectedObjectIds: [], // Clear object selection when selecting light
        };
      });
    },

    clearSelection: () => {
      set({ selectedObjectIds: [], selectedLightIds: [] });
    },

    selectAll: () => {
      set((state) => ({
        selectedObjectIds: state.objects.map((obj) => obj.id),
        selectedLightIds: [], // Clear light selection when selecting all objects
      }));
    },

    setActiveTool: (tool) => {
      set({ activeTool: tool });
    },

    setTransformDragging: (dragging) => {
      set({ transformDragging: dragging });
    },

    setCameraPosition: (position) => {
      set({ cameraPosition: position });
      get().markSceneModified();
    },

    setCameraTarget: (target) => {
      set({ cameraTarget: target });
      get().markSceneModified();
    },

    setGridVisible: (visible) => {
      set({ gridVisible: visible });
      get().markSceneModified();
    },

    setSnapToGrid: (snap) => {
      set({ snapToGrid: snap });
      get().markSceneModified();
    },

    setGridSize: (size) => {
      set({ gridSize: size });
      get().markSceneModified();
    },

    setLightsVisible: (visible) => {
      set({ lightsVisible: visible });
    },

    applyLightingPreset: (presetId) => {
      try {
        const newLights = applyLightingPreset(presetId);
        set({
          lights: newLights,
          selectedLightIds: [], // Clear selection
          selectedObjectIds: [], // Clear object selection
        });
        get().markSceneModified();
      } catch (error) {
        console.error("Failed to apply lighting preset:", error);
      }
    },

    clearScene: () => {
      set({
        objects: [],
        lights: [
          createDefaultLight("ambient", [0, 0, 0]),
          createDefaultLight("directional", [10, 10, 5]),
        ],
        selectedObjectIds: [],
        selectedLightIds: [],
      });

      get().markSceneModified();
    },

    startPlacement: (objectType) => {
      set({
        placementMode: {
          active: true,
          objectType,
          previewPosition: null,
        },
        selectedObjectIds: [], // Clear selection when entering placement mode
      });
    },

    updatePlacementPreview: (position) => {
      set((state) => ({
        placementMode: {
          ...state.placementMode,
          previewPosition: position,
        },
      }));
    },

    confirmPlacement: () => {
      const state = get();
      if (
        state.placementMode.active &&
        state.placementMode.objectType &&
        state.placementMode.previewPosition
      ) {
        const newObject = createDefaultObject(
          state.placementMode.objectType,
          state.placementMode.previewPosition
        );
        set((state) => ({
          objects: [...state.objects, newObject],
          selectedObjectIds: [newObject.id],
          placementMode: {
            active: false,
            objectType: null,
            previewPosition: null,
          },
        }));
      }
    },

    cancelPlacement: () => {
      set({
        placementMode: {
          active: false,
          objectType: null,
          previewPosition: null,
        },
      });
    },

    // File management actions
    setCurrentFilePath: (path) => {
      set({ currentFilePath: path });
    },

    setSceneMetadata: (metadata) => {
      set((state) => ({
        sceneMetadata: { ...state.sceneMetadata, ...metadata },
      }));
    },

    markSceneModified: () => {
      set((state) => ({
        sceneMetadata: { ...state.sceneMetadata, isModified: true },
      }));
    },

    markSceneSaved: () => {
      set((state) => ({
        sceneMetadata: {
          ...state.sceneMetadata,
          isModified: false,
          lastSavedAt: new Date(),
        },
      }));
    },

    loadScene: (sceneState) => {
      set((state) => ({
        ...state,
        ...sceneState,
        selectedObjectIds: [], // Clear selection when loading
        selectedLightIds: [], // Clear light selection when loading
        placementMode: {
          active: false,
          objectType: null,
          previewPosition: null,
        },
        sceneMetadata: {
          ...state.sceneMetadata,
          isModified: false,
        },
      }));
    },
  }))
);

// Selectors for commonly used derived state
export const useSelectedObjects = () => {
  const selectedObjectIds = useSceneStore((state) => state.selectedObjectIds);
  const objects = useSceneStore((state) => state.objects);
  return objects.filter((obj) => selectedObjectIds.includes(obj.id));
};

export const useSelectedObject = () => {
  const selectedObjects = useSelectedObjects();
  return selectedObjects.length > 0 ? selectedObjects[0] : null;
};

export const useSelectedLights = () => {
  const selectedLightIds = useSceneStore((state) => state.selectedLightIds);
  const lights = useSceneStore((state) => state.lights);
  return lights.filter((light) => selectedLightIds.includes(light.id));
};

export const useSelectedLight = () => {
  const selectedLights = useSelectedLights();
  return selectedLights.length > 0 ? selectedLights[0] : null;
};

export const useObjectsCount = () => {
  return useSceneStore((state) => state.objects.length);
};

export const useLightsCount = () => {
  return useSceneStore((state) => state.lights.length);
};

export const useHasSelection = () => {
  return useSceneStore(
    (state) =>
      state.selectedObjectIds.length > 0 || state.selectedLightIds.length > 0
  );
};

export const useSelectionCount = () => {
  return useSceneStore(
    (state) => state.selectedObjectIds.length + state.selectedLightIds.length
  );
};
