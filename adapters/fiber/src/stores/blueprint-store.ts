/**
 * Blueprint Store
 * Manages Blueprint editor state
 */

import { create } from "zustand";
import type { BlueprintGraph } from "~/utils/blueprint/graph/node-types";

export interface BlueprintState {
  /** Current Blueprint graph */
  currentGraph: BlueprintGraph | null;
  /** Current file path */
  currentFilePath: string | null;
  /** Whether the Blueprint has unsaved changes */
  isModified: boolean;
  /** Last saved timestamp */
  lastSavedAt: Date | null;
}

export interface BlueprintActions {
  /** Set the current Blueprint graph */
  setCurrentGraph: (graph: BlueprintGraph | null) => void;
  /** Update the current graph */
  updateGraph: (updates: Partial<BlueprintGraph>) => void;
  /** Set the current file path */
  setCurrentFilePath: (path: string | null) => void;
  /** Mark Blueprint as modified */
  markModified: () => void;
  /** Mark Blueprint as saved */
  markSaved: () => void;
  /** Clear the current Blueprint */
  clearBlueprint: () => void;
}

const initialState: BlueprintState = {
  currentGraph: null,
  currentFilePath: null,
  isModified: false,
  lastSavedAt: null,
};

export const useBlueprintStore = create<BlueprintState & BlueprintActions>(
  (set) => ({
    ...initialState,

    setCurrentGraph: (graph) => {
      set({
        currentGraph: graph,
        isModified: false,
        lastSavedAt: graph ? new Date() : null,
      });
    },

    updateGraph: (updates) => {
      set((state) => {
        if (!state.currentGraph) {
          return state;
        }
        return {
          currentGraph: { ...state.currentGraph, ...updates },
          isModified: true,
        };
      });
    },

    setCurrentFilePath: (path) => {
      set({ currentFilePath: path });
    },

    markModified: () => {
      set({ isModified: true });
    },

    markSaved: () => {
      set({
        isModified: false,
        lastSavedAt: new Date(),
      });
    },

    clearBlueprint: () => {
      set(initialState);
    },
  })
);
