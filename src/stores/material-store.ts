import * as THREE from "three";
import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";
import {
  type BasicMaterialProperties,
  DEFAULT_BASE_MATERIAL_PROPERTIES,
  DEFAULT_MATERIAL_SYSTEM_CONFIG,
  DEFAULT_STANDARD_MATERIAL_PROPERTIES,
  type DepthMaterialProperties,
  type LambertMaterialProperties,
  type MaterialAssignment,
  type MaterialLibraryEntry,
  type MaterialProperties,
  type MaterialSystemConfig,
  type MaterialType,
  type NormalMaterialProperties,
  type PhongMaterialProperties,
  type PhysicalMaterialProperties,
  type StandardMaterialProperties,
  type ToonMaterialProperties,
} from "~/types/materials";
import { createMaterialFromProperties } from "~/utils/material-utils";

/**
 * Material system state
 */
export interface MaterialState {
  /** Material library entries */
  materials: MaterialLibraryEntry[];
  /** Material assignments to scene objects */
  assignments: MaterialAssignment[];
  /** Currently selected material ID */
  selectedMaterialId: string | null;
  /** Material system configuration */
  config: MaterialSystemConfig;
  /** Whether material panel is visible */
  materialPanelVisible: boolean;
  /** Material filter/search term */
  materialFilter: string;
  /** Material category filter */
  materialCategoryFilter: string;
}

/**
 * Material system actions
 */
export interface MaterialActions {
  // Material CRUD operations
  createMaterial: (
    name: string,
    type: MaterialType,
    properties?: Partial<MaterialProperties>
  ) => string;
  updateMaterial: (id: string, updates: Partial<MaterialProperties>) => void;
  duplicateMaterial: (id: string) => string;
  deleteMaterial: (id: string) => void;
  importMaterial: (
    materialEntry: Omit<MaterialLibraryEntry, "id" | "createdAt" | "modifiedAt">
  ) => string;

  // Material selection and UI
  selectMaterial: (id: string | null) => void;
  setMaterialPanelVisible: (visible: boolean) => void;
  setMaterialFilter: (filter: string) => void;
  setMaterialCategoryFilter: (category: string) => void;

  // Material assignments
  assignMaterialToObject: (
    objectId: string,
    materialId: string,
    slotIndex?: number
  ) => void;
  removeMaterialFromObject: (objectId: string, slotIndex?: number) => void;
  getMaterialsForObject: (objectId: string) => MaterialAssignment[];
  getObjectForMaterial: (materialId: string) => MaterialAssignment[];

  // Material system configuration
  updateConfig: (config: Partial<MaterialSystemConfig>) => void;

  // Utility functions
  getMaterialById: (id: string) => MaterialLibraryEntry | undefined;
  getSelectedMaterial: () => MaterialLibraryEntry | undefined;
  getFilteredMaterials: () => MaterialLibraryEntry[];
  createThreeMaterial: (id: string) => THREE.Material | null;
  exportMaterial: (id: string) => MaterialLibraryEntry | null;

  // Bulk operations
  bulkUpdateMaterials: (
    updates: { id: string; changes: Partial<MaterialProperties> }[]
  ) => void;
  bulkDeleteMaterials: (ids: string[]) => void;

  // Cleanup and memory management
  cleanupUnusedMaterials: () => void;
  clearMaterialCache: () => void;
}

/**
 * Create default material properties for a given type
 */
function createDefaultMaterialProperties(
  type: MaterialType,
  overrides: Partial<MaterialProperties> = {}
): MaterialProperties {
  const baseProps = {
    ...DEFAULT_BASE_MATERIAL_PROPERTIES,
    ...overrides,
  };

  switch (type) {
    case "standard":
      return {
        ...baseProps,
        type: "standard",
        ...DEFAULT_STANDARD_MATERIAL_PROPERTIES,
        ...overrides,
      } as StandardMaterialProperties;

    case "physical":
      return {
        ...baseProps,
        type: "physical",
        ...DEFAULT_STANDARD_MATERIAL_PROPERTIES,
        ...overrides,
      } as PhysicalMaterialProperties;

    case "basic":
      return {
        ...baseProps,
        type: "basic",
        alphaToCoverage: false,
        colorWrite: true,
        depthTest: true,
        depthWrite: true,
        stencilWrite: false,
        stencilFunc: THREE.AlwaysStencilFunc,
        stencilRef: 0,
        stencilMask: 0xff,
        stencilFail: THREE.KeepStencilOp,
        stencilZFail: THREE.KeepStencilOp,
        stencilZPass: THREE.KeepStencilOp,
        map: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        alphaMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        aoMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        envMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        lightMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        specularMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        ...overrides,
      } as BasicMaterialProperties;

    case "lambert":
      return {
        ...baseProps,
        type: "lambert",
        alphaToCoverage: false,
        emissive: "#000000",
        envMapIntensity: 1,
        lightMapIntensity: 1,
        aoMapIntensity: 1,
        emissiveIntensity: 1,
        colorWrite: true,
        depthTest: true,
        depthWrite: true,
        map: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        alphaMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        aoMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        emissiveMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        envMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        lightMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        specularMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        ...overrides,
      } as LambertMaterialProperties;

    case "phong":
      return {
        ...baseProps,
        type: "phong",
        specular: "#111111",
        shininess: 30,
        emissive: "#000000",
        envMapIntensity: 1,
        lightMapIntensity: 1,
        aoMapIntensity: 1,
        emissiveIntensity: 1,
        bumpScale: 1,
        displacementScale: 1,
        displacementBias: 0,
        alphaToCoverage: false,
        colorWrite: true,
        depthTest: true,
        depthWrite: true,
        map: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        alphaMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        aoMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        bumpMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        displacementMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        emissiveMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        envMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        lightMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        normalMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        specularMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        ...overrides,
      } as PhongMaterialProperties;

    case "toon":
      return {
        ...baseProps,
        type: "toon",
        gradientMap: null,
        map: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        alphaMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        aoMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        lightMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        ...overrides,
      } as ToonMaterialProperties;

    case "normal":
      return {
        ...baseProps,
        type: "normal",
        normalMapType: THREE.TangentSpaceNormalMap,
        normalScale: [1, 1],
        normalMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        displacementMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        ...overrides,
      } as NormalMaterialProperties;

    case "depth":
      return {
        ...baseProps,
        type: "depth",
        displacementScale: 1,
        displacementBias: 0,
        displacementMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        alphaMap: {
          texture: null,
          offset: [0, 0],
          repeat: [1, 1],
          rotation: 0,
          wrapS: THREE.RepeatWrapping,
          wrapT: THREE.RepeatWrapping,
          magFilter: THREE.LinearFilter,
          minFilter: THREE.LinearMipmapLinearFilter,
          anisotropy: 1,
          encoding: THREE.SRGBColorSpace,
          flipY: true,
        },
        ...overrides,
      } as DepthMaterialProperties;

    default:
      throw new Error(`Unknown material type: ${type}`);
  }
}

/**
 * Material store implementation
 */
export const useMaterialStore = create<MaterialState & MaterialActions>()(
  subscribeWithSelector((set, get) => ({
    // Initial state
    materials: [],
    assignments: [],
    selectedMaterialId: null,
    config: DEFAULT_MATERIAL_SYSTEM_CONFIG,
    materialPanelVisible: false,
    materialFilter: "",
    materialCategoryFilter: "",

    // Material CRUD operations
    createMaterial: (name, type, properties = {}) => {
      const id = `material_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      const materialProperties = createDefaultMaterialProperties(type, {
        id,
        name,
        ...properties,
      });

      const materialEntry: MaterialLibraryEntry = {
        id,
        name,
        properties: materialProperties,
        createdAt: new Date(),
        modifiedAt: new Date(),
        tags: [],
      };

      set((state) => ({
        materials: [...state.materials, materialEntry],
        selectedMaterialId: id,
      }));

      return id;
    },

    updateMaterial: (id, updates) => {
      set(
        (state) =>
          ({
            materials: state.materials.map((material) =>
              material.id === id
                ? {
                    ...material,
                    properties: { ...material.properties, ...updates },
                    modifiedAt: new Date(),
                  }
                : material
            ),
          }) as Partial<MaterialState & MaterialActions>
      );
    },

    duplicateMaterial: (id) => {
      const originalMaterial = get().materials.find((m) => m.id === id);
      if (!originalMaterial) {
        throw new Error(`Material with id ${id} not found`);
      }

      const newId = `material_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      const duplicatedMaterial: MaterialLibraryEntry = {
        ...originalMaterial,
        id: newId,
        name: `${originalMaterial.name} Copy`,
        properties: {
          ...originalMaterial.properties,
          id: newId,
          name: `${originalMaterial.name} Copy`,
        },
        createdAt: new Date(),
        modifiedAt: new Date(),
      };

      set((state) => ({
        materials: [...state.materials, duplicatedMaterial],
        selectedMaterialId: newId,
      }));

      return newId;
    },

    deleteMaterial: (id) => {
      set((state) => ({
        materials: state.materials.filter((m) => m.id !== id),
        assignments: state.assignments.filter((a) => a.materialId !== id),
        selectedMaterialId:
          state.selectedMaterialId === id ? null : state.selectedMaterialId,
      }));
    },

    importMaterial: (materialData) => {
      const id = `material_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      const materialEntry: MaterialLibraryEntry = {
        ...materialData,
        id,
        createdAt: new Date(),
        modifiedAt: new Date(),
      };

      set((state) => ({
        materials: [...state.materials, materialEntry],
        selectedMaterialId: id,
      }));

      return id;
    },

    // Material selection and UI
    selectMaterial: (id) => {
      set({ selectedMaterialId: id });
    },

    setMaterialPanelVisible: (visible) => {
      set({ materialPanelVisible: visible });
    },

    setMaterialFilter: (filter) => {
      set({ materialFilter: filter });
    },

    setMaterialCategoryFilter: (category) => {
      set({ materialCategoryFilter: category });
    },

    // Material assignments
    assignMaterialToObject: (objectId, materialId, slotIndex = 0) => {
      set((state) => {
        // Remove existing assignment for this object/slot
        const filteredAssignments = state.assignments.filter(
          (a) => !(a.objectId === objectId && a.slotIndex === slotIndex)
        );

        return {
          assignments: [
            ...filteredAssignments,
            { objectId, materialId, slotIndex },
          ],
        };
      });
    },

    removeMaterialFromObject: (objectId, slotIndex = 0) => {
      set((state) => ({
        assignments: state.assignments.filter(
          (a) => !(a.objectId === objectId && a.slotIndex === slotIndex)
        ),
      }));
    },

    getMaterialsForObject: (objectId) => {
      return get().assignments.filter((a) => a.objectId === objectId);
    },

    getObjectForMaterial: (materialId) => {
      return get().assignments.filter((a) => a.materialId === materialId);
    },

    // Material system configuration
    updateConfig: (config) => {
      set((state) => ({
        config: { ...state.config, ...config },
      }));
    },

    // Utility functions
    getMaterialById: (id) => {
      return get().materials.find((m) => m.id === id);
    },

    getSelectedMaterial: () => {
      const selectedId = get().selectedMaterialId;
      return selectedId ? get().getMaterialById(selectedId) : undefined;
    },

    getFilteredMaterials: () => {
      const { materials, materialFilter, materialCategoryFilter } = get();

      return materials.filter((material) => {
        const matchesFilter =
          materialFilter === "" ||
          material.name.toLowerCase().includes(materialFilter.toLowerCase()) ||
          material.properties.type
            .toLowerCase()
            .includes(materialFilter.toLowerCase());

        const matchesCategory =
          materialCategoryFilter === "" ||
          material.tags.includes(materialCategoryFilter);

        return matchesFilter && matchesCategory;
      });
    },

    createThreeMaterial: (id) => {
      const material = get().getMaterialById(id);
      if (!material) return null;

      return createMaterialFromProperties(material.properties);
    },

    exportMaterial: (id) => {
      const material = get().getMaterialById(id);
      return material || null;
    },

    // Bulk operations
    bulkUpdateMaterials: (updates) => {
      set(
        (state) =>
          ({
            materials: state.materials.map((material) => {
              const update = updates.find((u) => u.id === material.id);
              if (update) {
                return {
                  ...material,
                  properties: { ...material.properties, ...update.changes },
                  modifiedAt: new Date(),
                };
              }
              return material;
            }),
          }) as Partial<MaterialState & MaterialActions>
      );
    },

    bulkDeleteMaterials: (ids) => {
      set((state) => ({
        materials: state.materials.filter((m) => !ids.includes(m.id)),
        assignments: state.assignments.filter(
          (a) => !ids.includes(a.materialId)
        ),
        selectedMaterialId:
          state.selectedMaterialId && ids.includes(state.selectedMaterialId)
            ? null
            : state.selectedMaterialId,
      }));
    },

    // Cleanup and memory management
    cleanupUnusedMaterials: () => {
      const { assignments } = get();
      const usedMaterialIds = new Set(assignments.map((a) => a.materialId));

      set((state) => ({
        materials: state.materials.filter((m) => usedMaterialIds.has(m.id)),
      }));
    },

    clearMaterialCache: () => {
      // Clear any cached Three.js materials
      // This would be implemented with a material cache system
      set({ materials: [] });
    },
  }))
);

// Selectors for commonly used derived state
export const useSelectedMaterial = () => {
  return useMaterialStore((state) => state.getSelectedMaterial());
};

export const useMaterialsCount = () => {
  return useMaterialStore((state) => state.materials.length);
};

export const useFilteredMaterials = () => {
  return useMaterialStore((state) => state.getFilteredMaterials());
};

export const useMaterialAssignments = () => {
  return useMaterialStore((state) => state.assignments);
};

export const useMaterialForObject = (objectId: string) => {
  return useMaterialStore((state) => {
    const assignments = state.getMaterialsForObject(objectId);
    return assignments.length > 0
      ? state.getMaterialById(assignments[0].materialId)
      : undefined;
  });
};

// Re-export types for convenience
export type { MaterialType } from "~/types/materials";
