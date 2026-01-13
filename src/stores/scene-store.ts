import * as THREE from "three";
import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";
import {
  CAMERA_DEFAULTS,
  GRID_DEFAULTS,
  OBJECT_DEFAULTS,
  PLACEMENT_DEFAULTS,
  SCENE_METADATA_DEFAULTS,
  TOOL_DEFAULTS,
  UI_DEFAULTS,
} from "~/lib/defaults";
import { useMaterialStore } from "~/stores/material-store";
import { clearPools } from "~/utils/geometry-pool";
import {
  applyLightingPreset,
  type LightingPresetId,
} from "~/utils/lighting-presets";
import { clearModelCache } from "~/utils/model-import";
import type { EnvironmentPreset } from "~/components/skybox";

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
  materialId?: string; // Reference to material in material system
  material?: THREE.Material; // Fallback for legacy compatibility
  importedModel?: THREE.Object3D;
  initialScale?: number; // For imported models, stores the initial scaling factor
  modelid?: number; // SAMP model ID for export
  animationController?: any; // Animation controller for GLTF models
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

  // Environment settings
  backgroundColor: string;
  fogEnabled: boolean;
  fogColor: string;
  fogNear: number;
  fogFar: number;

  // Environment settings (expanded from skybox)
  skyboxEnabled: boolean;
  skyboxPreset: EnvironmentPreset;
  skyboxIntensity: number;
  environmentIntensity: number;
  backgroundBlurriness: number;
  environmentRotation: [number, number, number];
  backgroundRotation: [number, number, number];
  groundProjection: boolean;
  groundProjectionHeight: number;
  groundProjectionRadius: number;
  groundProjectionScale: number;
  customEnvironmentFile: string | null;
  liveEnvironment: boolean;
  liveEnvironmentResolution: number;
  cameraShakeEnabled: boolean;
  cameraShakeIntensity: number;
  cameraShakeSpeed: number;

  // Scene helpers
  axesVisible: boolean;
  statsVisible: boolean;

  // Performance settings
  performanceRegressionOnMove: boolean;

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
  addLight: (
    typeOrLight: LightType | Omit<SceneLight, "id" | "name">,
    position?: [number, number, number]
  ) => void;
  removeLight: (id: string) => void;
  updateLight: (id: string, updates: Partial<SceneLight>) => void;
  duplicateLight: (id: string) => void;
  clearLights: () => void;

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
  applyLightingPreset: (presetId: LightingPresetId) => void;

  // Environment
  setBackgroundColor: (color: string) => void;
  setFogEnabled: (enabled: boolean) => void;
  setFogColor: (color: string) => void;
  setFogNear: (near: number) => void;
  setFogFar: (far: number) => void;

  // Environment (expanded skybox)
  setSkyboxEnabled: (enabled: boolean) => void;
  setSkyboxPreset: (preset: EnvironmentPreset) => void;
  setSkyboxIntensity: (intensity: number) => void;
  setEnvironmentIntensity: (intensity: number) => void;
  setBackgroundBlurriness: (blurriness: number) => void;
  setEnvironmentRotation: (rotation: [number, number, number]) => void;
  setBackgroundRotation: (rotation: [number, number, number]) => void;
  setGroundProjection: (enabled: boolean) => void;
  setGroundProjectionHeight: (height: number) => void;
  setGroundProjectionRadius: (radius: number) => void;
  setGroundProjectionScale: (scale: number) => void;
  setCustomEnvironmentFile: (file: string | null) => void;
  setLiveEnvironment: (enabled: boolean) => void;
  setLiveEnvironmentResolution: (resolution: number) => void;
  setCameraShakeEnabled: (enabled: boolean) => void;
  setCameraShakeIntensity: (intensity: number) => void;
  setCameraShakeSpeed: (speed: number) => void;

  // Scene helpers
  setAxesVisible: (visible: boolean) => void;
  setStatsVisible: (visible: boolean) => void;

  // Performance settings
  setPerformanceRegressionOnMove: (enabled: boolean) => void;

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

  // Memory management
  cleanupSceneResources: () => void;
  forceGarbageCollection: () => void;
}

const createDefaultObject = (
  type: SceneObject["type"],
  position: [number, number, number] = OBJECT_DEFAULTS.POSITION
): SceneObject => {
  const baseObject: Omit<SceneObject, "id" | "name"> = {
    type,
    position,
    rotation: OBJECT_DEFAULTS.ROTATION,
    scale: OBJECT_DEFAULTS.SCALE,
    color: OBJECT_DEFAULTS.COLOR,
    visible: OBJECT_DEFAULTS.VISIBLE,
  };

  return {
    ...baseObject,
    id: `object_${Math.random().toString(36).substr(2, 9)}`,
    name: `${type.charAt(0).toUpperCase()}${type.slice(1)}`,
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
        id: `light_${Math.random().toString(36).substr(2, 9)}`,
        name: "Directional Light",
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
        id: `light_${Math.random().toString(36).substr(2, 9)}`,
        name: "Point Light",
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
        id: `light_${Math.random().toString(36).substr(2, 9)}`,
        name: "Spot Light",
      };
    case "ambient":
      return {
        ...baseLight,
        intensity: 0.6,
        id: `light_${Math.random().toString(36).substr(2, 9)}`,
        name: "Ambient Light",
      };
    case "hemisphere":
      return {
        ...baseLight,
        groundColor: "#444444",
        intensity: 0.6,
        id: `light_${Math.random().toString(36).substr(2, 9)}`,
        name: "Hemisphere Light",
      };
    default:
      throw new Error(`Unknown light type: ${type}`);
  }
};

export const useSceneStore = create<SceneState & SceneActions>()(
  subscribeWithSelector((set, get) => ({
    // Initial state with basic studio lighting for new projects
    objects: [],
    lights: applyLightingPreset("studio"),
    selectedObjectIds: [],
    selectedLightIds: [],
    activeTool: TOOL_DEFAULTS.ACTIVE_TOOL,
    cameraPosition: CAMERA_DEFAULTS.POSITION,
    cameraTarget: CAMERA_DEFAULTS.TARGET,
    gridVisible: GRID_DEFAULTS.VISIBLE,
    snapToGrid: GRID_DEFAULTS.SNAP_TO_GRID,
    gridSize: GRID_DEFAULTS.SIZE,
    placementMode: PLACEMENT_DEFAULTS,
    lightsVisible: UI_DEFAULTS.LIGHTS_VISIBLE,
    transformDragging: UI_DEFAULTS.TRANSFORM_DRAGGING,

    // Environment defaults
    backgroundColor: "#0a0a0a", // very dark gray (alternatives: "#1e293b")
    fogEnabled: false,
    fogColor: "#0a0a0a", // very dark gray (alternatives: "#1e293b")
    fogNear: 1,
    fogFar: 100,

    // Environment defaults
    skyboxEnabled: true,
    skyboxPreset: "cosmic",
    skyboxIntensity: 1,
    environmentIntensity: 1,
    backgroundBlurriness: 0,
    environmentRotation: [0, 0, 0],
    backgroundRotation: [0, 0, 0],
    groundProjection: false,
    groundProjectionHeight: 15,
    groundProjectionRadius: 60,
    groundProjectionScale: 1000,
    customEnvironmentFile: null,
    liveEnvironment: false,
    liveEnvironmentResolution: 256,
    cameraShakeEnabled: false,
    cameraShakeIntensity: 0.1,
    cameraShakeSpeed: 1,

    // Scene helpers defaults
    axesVisible: true,
    statsVisible: true,

    // Performance settings defaults
    performanceRegressionOnMove: UI_DEFAULTS.PERFORMANCE_REGRESSION_ON_MOVE,

    currentFilePath: null,
    sceneMetadata: SCENE_METADATA_DEFAULTS,

    // Actions
    addObject: (typeOrObject, position) => {
      let newObject: SceneObject;

      if (typeof typeOrObject === "string") {
        newObject = createDefaultObject(typeOrObject, position);

        // Create and assign a default material for the new object
        const materialStore = useMaterialStore.getState();
        const materialName = `${newObject.name} Material`;
        const materialId = materialStore.createMaterial(
          materialName,
          "standard",
          {
            color: newObject.color,
            // Planes should be double-sided by default
            side:
              newObject.type === "plane" ? THREE.DoubleSide : THREE.FrontSide,
          }
        );

        // Assign the material to the object
        newObject.materialId = materialId;
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
      const objectToRemove = get().objects.find((obj) => obj.id === id);

      set((state) => ({
        objects: state.objects.filter((obj) => obj.id !== id),
        selectedObjectIds: state.selectedObjectIds.filter(
          (selectedId) => selectedId !== id
        ),
      }));

      // Clean up material assignments
      if (objectToRemove?.materialId) {
        const materialStore = useMaterialStore.getState();
        materialStore.removeMaterialFromObject(id);
        // Optionally cleanup unused materials
        materialStore.cleanupUnusedMaterials();
      }

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

      // Duplicate the material if one is assigned
      if (originalObject.materialId) {
        const materialStore = useMaterialStore.getState();
        const newMaterialId = materialStore.duplicateMaterial(
          originalObject.materialId
        );
        duplicatedObject.materialId = newMaterialId;
      }

      set((state) => ({
        objects: [...state.objects, duplicatedObject],
        selectedObjectIds: [duplicatedObject.id],
      }));

      get().markSceneModified();
    },

    // Light management actions
    addLight: (typeOrLight, position) => {
      let newLight: SceneLight;

      if (typeof typeOrLight === "string") {
        newLight = createDefaultLight(typeOrLight, position);
      } else {
        newLight = {
          ...typeOrLight,
          id: `light_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
          name: `${typeOrLight.type.charAt(0).toUpperCase() + typeOrLight.type.slice(1)} Light ${Date.now()}`,
        };
      }

      set((state) => ({
        lights: [...state.lights, newLight],
        selectedLightIds: [newLight.id],
        selectedObjectIds: [], // Clear object selection when selecting light
      }));

      get().markSceneModified();
    },

    clearLights: () => {
      set({
        lights: [],
        selectedLightIds: [],
      });
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

      // Auto-select material for the selected object if it has one
      const object = get().objects.find((obj) => obj.id === id);
      if (object?.materialId) {
        const materialStore = useMaterialStore.getState();
        materialStore.selectMaterial(object.materialId);
      } else {
        // Clear material selection if object has no material
        const materialStore = useMaterialStore.getState();
        materialStore.selectMaterial(null);
      }
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

      // Clear material selection when selecting light
      const materialStore = useMaterialStore.getState();
      materialStore.selectMaterial(null);
    },

    clearSelection: () => {
      set({ selectedObjectIds: [], selectedLightIds: [] });
      // Also clear material selection
      const materialStore = useMaterialStore.getState();
      materialStore.selectMaterial(null);
    },

    selectAll: () => {
      set((state) => ({
        selectedObjectIds: state.objects.map((obj) => obj.id),
        selectedLightIds: [], // Clear light selection when selecting all objects
      }));

      // Clear material selection when selecting all objects
      const materialStore = useMaterialStore.getState();
      materialStore.selectMaterial(null);
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

    // Environment actions
    setBackgroundColor: (color) => {
      set({ backgroundColor: color });
      get().markSceneModified();
    },
    setFogEnabled: (enabled) => {
      set({ fogEnabled: enabled });
      get().markSceneModified();
    },
    setFogColor: (color) => {
      set({ fogColor: color });
      get().markSceneModified();
    },
    setFogNear: (near) => {
      set({ fogNear: near });
      get().markSceneModified();
    },
    setFogFar: (far) => {
      set({ fogFar: far });
      get().markSceneModified();
    },

    // Skybox actions
    setSkyboxEnabled: (enabled) => {
      set({ skyboxEnabled: enabled });
      get().markSceneModified();
    },
    setSkyboxPreset: (preset: EnvironmentPreset) => {
      set({ skyboxPreset: preset });
      get().markSceneModified();
    },
    setSkyboxIntensity: (intensity) => {
      set({ skyboxIntensity: intensity });
      get().markSceneModified();
    },
    setEnvironmentIntensity: (intensity) => {
      set({ environmentIntensity: intensity });
      get().markSceneModified();
    },
    setBackgroundBlurriness: (blurriness) => {
      set({ backgroundBlurriness: blurriness });
      get().markSceneModified();
    },
    setEnvironmentRotation: (rotation) => {
      set({ environmentRotation: rotation });
      get().markSceneModified();
    },
    setBackgroundRotation: (rotation) => {
      set({ backgroundRotation: rotation });
      get().markSceneModified();
    },
    setGroundProjection: (enabled) => {
      set({ groundProjection: enabled });
      get().markSceneModified();
    },
    setGroundProjectionHeight: (height) => {
      set({ groundProjectionHeight: height });
      get().markSceneModified();
    },
    setGroundProjectionRadius: (radius) => {
      set({ groundProjectionRadius: radius });
      get().markSceneModified();
    },
    setGroundProjectionScale: (scale) => {
      set({ groundProjectionScale: scale });
      get().markSceneModified();
    },
    setCustomEnvironmentFile: (file) => {
      set({ customEnvironmentFile: file });
      get().markSceneModified();
    },
    setLiveEnvironment: (enabled) => {
      set({ liveEnvironment: enabled });
      get().markSceneModified();
    },
    setLiveEnvironmentResolution: (resolution) => {
      set({ liveEnvironmentResolution: resolution });
      get().markSceneModified();
    },
    setCameraShakeEnabled: (enabled) => {
      set({ cameraShakeEnabled: enabled });
      get().markSceneModified();
    },
    setCameraShakeIntensity: (intensity) => {
      set({ cameraShakeIntensity: intensity });
      get().markSceneModified();
    },
    setCameraShakeSpeed: (speed) => {
      set({ cameraShakeSpeed: speed });
      get().markSceneModified();
    },

    // Scene helpers actions
    setAxesVisible: (visible) => {
      set({ axesVisible: visible });
      get().markSceneModified();
    },
    setStatsVisible: (visible) => {
      set({ statsVisible: visible });
      get().markSceneModified();
    },

    // Performance settings actions
    setPerformanceRegressionOnMove: (enabled) => {
      set({ performanceRegressionOnMove: enabled });
    },

    clearScene: () => {
      set({
        objects: [],
        lights: [],
        selectedObjectIds: [],
        selectedLightIds: [],
      });

      // Also clear material selection
      const materialStore = useMaterialStore.getState();
      materialStore.selectMaterial(null);

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
      // Cleanup resources from current scene before loading new one
      get().cleanupSceneResources();

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

    // Memory management functions
    cleanupSceneResources: () => {
      const state = get();

      // Dispose of imported model geometries and materials
      for (const object of state.objects) {
        if (object.type === "imported" && object.importedModel) {
          object.importedModel.traverse((child) => {
            if (child instanceof THREE.Mesh) {
              if (child.geometry) {
                child.geometry.dispose();
              }
              if (child.material) {
                if (Array.isArray(child.material)) {
                  for (const material of child.material) {
                    material.dispose();
                  }
                } else {
                  child.material.dispose();
                }
              }
            }
          });
        }
      }

      // Clear model cache
      clearModelCache();

      // Clear geometry and material pools
      clearPools();
    },

    forceGarbageCollection: () => {
      // Cleanup scene resources
      get().cleanupSceneResources();

      // Force garbage collection if available (only in development/debugging)
      if (typeof window !== "undefined" && "gc" in window) {
        (window as Window & { gc?: () => void }).gc?.();
      }
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
