/**
 * React hook for using the Blueprint Sync Manager
 */

import { useEffect, useRef, useState, useCallback } from "react";
import type { BlueprintGraph } from "~/utils/blueprint/graph/node-types";
import type { ProgramNode } from "~/utils/blueprint/ast/ast-core";
import {
  SyncManager,
  type SyncPreferences,
  type SyncConflict,
  type SyncState,
} from "./sync-manager";

/**
 * Hook options
 */
export interface UseSyncManagerOptions {
  /** Initial sync preferences */
  preferences?: Partial<SyncPreferences>;
  /** Callback when code changes */
  onCodeChange?: (ast: ProgramNode) => void;
  /** Callback when Blueprint changes */
  onBlueprintChange?: (graph: BlueprintGraph) => void;
  /** Callback when conflict occurs */
  onConflict?: (conflict: SyncConflict) => void;
}

/**
 * Hook return value
 */
export interface UseSyncManagerReturn {
  /** Sync manager instance */
  syncManager: SyncManager;
  /** Current sync state */
  state: SyncState;
  /** Sync code to Blueprint */
  syncCodeToBlueprint: (ast: ProgramNode) => Promise<BlueprintGraph>;
  /** Sync Blueprint to code */
  syncBlueprintToCode: (graph: BlueprintGraph) => Promise<ProgramNode>;
  /** Handle code change (with debouncing) */
  handleCodeChange: (ast: ProgramNode) => void;
  /** Handle Blueprint change (with debouncing) */
  handleBlueprintChange: (graph: BlueprintGraph) => void;
  /** Update preferences */
  updatePreferences: (preferences: Partial<SyncPreferences>) => void;
  /** Resolve conflict */
  resolveConflict: (
    conflict: SyncConflict,
    resolution: "code_wins" | "blueprint_wins"
  ) => Promise<void>;
}

/**
 * React hook for Blueprint sync manager
 */
export function useSyncManager(
  options: UseSyncManagerOptions = {}
): UseSyncManagerReturn {
  const { preferences, onCodeChange, onBlueprintChange, onConflict } = options;

  // Create sync manager instance (persist across renders)
  const syncManagerRef = useRef<SyncManager | null>(null);
  if (!syncManagerRef.current) {
    syncManagerRef.current = new SyncManager(preferences);
  }

  const syncManager = syncManagerRef.current;

  // State for sync status
  const [state, setState] = useState<SyncState>(syncManager.getState());

  // Register callbacks
  useEffect(() => {
    const unsubscribers: (() => void)[] = [];

    if (onCodeChange) {
      const unsubscribe = syncManager.onCodeChange((ast) => {
        setState(syncManager.getState());
        onCodeChange(ast);
      });
      unsubscribers.push(unsubscribe);
    }

    if (onBlueprintChange) {
      const unsubscribe = syncManager.onBlueprintChange((graph) => {
        setState(syncManager.getState());
        onBlueprintChange(graph);
      });
      unsubscribers.push(unsubscribe);
    }

    if (onConflict) {
      const unsubscribe = syncManager.onConflict((conflict) => {
        onConflict(conflict);
      });
      unsubscribers.push(unsubscribe);
    }

    return () => {
      for (const unsub of unsubscribers) {
        unsub();
      }
    };
  }, [syncManager, onCodeChange, onBlueprintChange, onConflict]);

  // Update preferences when they change
  useEffect(() => {
    if (preferences) {
      syncManager.updatePreferences(preferences);
    }
  }, [syncManager, preferences]);

  // Sync functions
  const syncCodeToBlueprint = useCallback(
    async (ast: ProgramNode): Promise<BlueprintGraph> => {
      const graph = await syncManager.syncCodeToBlueprint(ast);
      setState(syncManager.getState());
      return graph;
    },
    [syncManager]
  );

  const syncBlueprintToCode = useCallback(
    async (graph: BlueprintGraph): Promise<ProgramNode> => {
      const ast = await syncManager.syncBlueprintToCode(graph);
      setState(syncManager.getState());
      return ast;
    },
    [syncManager]
  );

  const handleCodeChange = useCallback(
    (ast: ProgramNode) => {
      syncManager.handleCodeChange(ast);
      setState(syncManager.getState());
    },
    [syncManager]
  );

  const handleBlueprintChange = useCallback(
    (graph: BlueprintGraph) => {
      syncManager.handleBlueprintChange(graph);
      setState(syncManager.getState());
    },
    [syncManager]
  );

  const updatePreferences = useCallback(
    (newPreferences: Partial<SyncPreferences>) => {
      syncManager.updatePreferences(newPreferences);
    },
    [syncManager]
  );

  const resolveConflict = useCallback(
    async (
      conflict: SyncConflict,
      resolution: "code_wins" | "blueprint_wins"
    ) => {
      await syncManager.resolveConflict(conflict, resolution);
      setState(syncManager.getState());
    },
    [syncManager]
  );

  return {
    syncManager,
    state,
    syncCodeToBlueprint,
    syncBlueprintToCode,
    handleCodeChange,
    handleBlueprintChange,
    updatePreferences,
    resolveConflict,
  };
}
