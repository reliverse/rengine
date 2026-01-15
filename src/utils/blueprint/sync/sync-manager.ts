/**
 * Blueprint Sync Manager
 * Handles bidirectional synchronization between code and Blueprint visual representation
 */

import type { BlueprintGraph } from "~/utils/blueprint/graph/node-types";
import type { ProgramNode } from "~/utils/blueprint/ast/ast-core";
import { convertASTToBlueprint } from "~/utils/blueprint/converters/ast-to-blueprint";
import { convertBlueprintToAST } from "~/utils/blueprint/converters/blueprint-to-ast";

/**
 * Sync state tracking
 */
export interface SyncState {
  /** Last known code hash */
  lastCodeHash: string;
  /** Last known Blueprint hash */
  lastBlueprintHash: string;
  /** Timestamp of last code sync */
  lastCodeSync: number;
  /** Timestamp of last Blueprint sync */
  lastBlueprintSync: number;
  /** Whether sync is currently in progress */
  syncing: boolean;
}

/**
 * Sync conflict information
 */
export interface SyncConflict {
  type: "code_changed" | "blueprint_changed" | "both_changed";
  codeChange: {
    hash: string;
    timestamp: number;
  };
  blueprintChange: {
    hash: string;
    timestamp: number;
  };
  message: string;
}

/**
 * Sync preferences
 */
export interface SyncPreferences {
  /** Auto-sync on code changes */
  autoSyncOnCodeChange: boolean;
  /** Auto-sync on Blueprint changes */
  autoSyncOnBlueprintChange: boolean;
  /** Conflict resolution strategy */
  conflictResolution: "code_wins" | "blueprint_wins" | "ask_user" | "merge";
  /** Debounce delay for auto-sync (ms) */
  debounceDelay: number;
  /** Whether to preserve node positions when syncing from code */
  preservePositions: boolean;
}

const DEFAULT_PREFERENCES: SyncPreferences = {
  autoSyncOnCodeChange: true,
  autoSyncOnBlueprintChange: true,
  conflictResolution: "ask_user",
  debounceDelay: 500,
  preservePositions: true,
};

/**
 * Sync Manager class
 */
export class SyncManager {
  private state: SyncState;
  private preferences: SyncPreferences;
  private readonly codeChangeCallbacks: Set<(ast: ProgramNode) => void> =
    new Set();
  private readonly blueprintChangeCallbacks: Set<
    (graph: BlueprintGraph) => void
  > = new Set();
  private readonly conflictCallbacks: Set<(conflict: SyncConflict) => void> =
    new Set();
  private debounceTimer: ReturnType<typeof setTimeout> | null = null;

  constructor(preferences: Partial<SyncPreferences> = {}) {
    this.preferences = { ...DEFAULT_PREFERENCES, ...preferences };
    this.state = {
      lastCodeHash: "",
      lastBlueprintHash: "",
      lastCodeSync: 0,
      lastBlueprintSync: 0,
      syncing: false,
    };
  }

  /**
   * Update sync preferences
   */
  updatePreferences(preferences: Partial<SyncPreferences>): void {
    this.preferences = { ...this.preferences, ...preferences };
  }

  /**
   * Get current preferences
   */
  getPreferences(): SyncPreferences {
    return { ...this.preferences };
  }

  /**
   * Get current sync state
   */
  getState(): SyncState {
    return { ...this.state };
  }

  /**
   * Register callback for code changes
   */
  onCodeChange(callback: (ast: ProgramNode) => void): () => void {
    this.codeChangeCallbacks.add(callback);
    return () => {
      this.codeChangeCallbacks.delete(callback);
    };
  }

  /**
   * Register callback for Blueprint changes
   */
  onBlueprintChange(callback: (graph: BlueprintGraph) => void): () => void {
    this.blueprintChangeCallbacks.add(callback);
    return () => {
      this.blueprintChangeCallbacks.delete(callback);
    };
  }

  /**
   * Register callback for sync conflicts
   */
  onConflict(callback: (conflict: SyncConflict) => void): () => void {
    this.conflictCallbacks.add(callback);
    return () => {
      this.conflictCallbacks.delete(callback);
    };
  }

  /**
   * Sync code changes to Blueprint
   */
  async syncCodeToBlueprint(ast: ProgramNode): Promise<BlueprintGraph> {
    if (this.state.syncing) {
      throw new Error("Sync already in progress");
    }

    this.state.syncing = true;
    try {
      const codeHash = this.hashAST(ast);
      const hasConflict = this.detectConflict(
        codeHash,
        this.state.lastBlueprintHash
      );

      if (hasConflict) {
        const conflict = this.createConflict(
          "code_changed",
          codeHash,
          this.state.lastBlueprintHash
        );
        this.notifyConflict(conflict);

        // Handle conflict based on preferences
        if (this.preferences.conflictResolution === "code_wins") {
          // Continue with sync
        } else if (this.preferences.conflictResolution === "blueprint_wins") {
          this.state.syncing = false;
          throw new Error("Blueprint changes take precedence");
        } else if (this.preferences.conflictResolution === "ask_user") {
          // Wait for user decision (handled by conflict callback)
          this.state.syncing = false;
          throw new Error("User decision required");
        }
      }

      // Convert AST to Blueprint
      const graph = convertASTToBlueprint(ast, {
        preservePositions: this.preferences.preservePositions,
      });

      // Update state
      this.state.lastCodeHash = codeHash;
      this.state.lastCodeSync = Date.now();

      // Notify Blueprint change callbacks
      this.notifyBlueprintChange(graph);

      return graph;
    } finally {
      this.state.syncing = false;
    }
  }

  /**
   * Sync Blueprint changes to code
   */
  async syncBlueprintToCode(graph: BlueprintGraph): Promise<ProgramNode> {
    if (this.state.syncing) {
      throw new Error("Sync already in progress");
    }

    this.state.syncing = true;
    try {
      const blueprintHash = this.hashBlueprint(graph);
      const hasConflict = this.detectConflict(
        this.state.lastCodeHash,
        blueprintHash
      );

      if (hasConflict) {
        const conflict = this.createConflict(
          "blueprint_changed",
          this.state.lastCodeHash,
          blueprintHash
        );
        this.notifyConflict(conflict);

        // Handle conflict based on preferences
        if (this.preferences.conflictResolution === "blueprint_wins") {
          // Continue with sync
        } else if (this.preferences.conflictResolution === "code_wins") {
          this.state.syncing = false;
          throw new Error("Code changes take precedence");
        } else if (this.preferences.conflictResolution === "ask_user") {
          // Wait for user decision (handled by conflict callback)
          this.state.syncing = false;
          throw new Error("User decision required");
        }
      }

      // Convert Blueprint to AST
      const ast = convertBlueprintToAST(graph, {
        preservePositions: this.preferences.preservePositions,
      });

      // Update state
      this.state.lastBlueprintHash = blueprintHash;
      this.state.lastBlueprintSync = Date.now();

      // Notify code change callbacks
      this.notifyCodeChange(ast);

      return ast;
    } finally {
      this.state.syncing = false;
    }
  }

  /**
   * Handle code change with debouncing
   */
  handleCodeChange(ast: ProgramNode): void {
    if (!this.preferences.autoSyncOnCodeChange) {
      return;
    }

    // Clear existing timer
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer);
    }

    // Set new timer
    this.debounceTimer = setTimeout(() => {
      this.syncCodeToBlueprint(ast).catch((error) => {
        console.error("Failed to sync code to Blueprint:", error);
      });
    }, this.preferences.debounceDelay);
  }

  /**
   * Handle Blueprint change with debouncing
   */
  handleBlueprintChange(graph: BlueprintGraph): void {
    if (!this.preferences.autoSyncOnBlueprintChange) {
      return;
    }

    // Clear existing timer
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer);
    }

    // Set new timer
    this.debounceTimer = setTimeout(() => {
      this.syncBlueprintToCode(graph).catch((error) => {
        console.error("Failed to sync Blueprint to code:", error);
      });
    }, this.preferences.debounceDelay);
  }

  /**
   * Resolve conflict by choosing a side
   */
  async resolveConflict(
    conflict: SyncConflict,
    resolution: "code_wins" | "blueprint_wins"
  ): Promise<void> {
    // Update preferences temporarily
    const oldResolution = this.preferences.conflictResolution;
    this.preferences.conflictResolution = resolution;

    try {
      // Retry sync based on resolution
      if (resolution === "code_wins" && conflict.type === "code_changed") {
        // Code changes will be synced when code is provided again
      } else if (
        resolution === "blueprint_wins" &&
        conflict.type === "blueprint_changed"
      ) {
        // Blueprint changes will be synced when Blueprint is provided again
      }
    } finally {
      this.preferences.conflictResolution = oldResolution;
    }
  }

  /**
   * Detect if there's a conflict
   */
  private detectConflict(codeHash: string, blueprintHash: string): boolean {
    const codeChanged =
      codeHash !== this.state.lastCodeHash && this.state.lastCodeHash !== "";
    const blueprintChanged =
      blueprintHash !== this.state.lastBlueprintHash &&
      this.state.lastBlueprintHash !== "";

    return codeChanged && blueprintChanged;
  }

  /**
   * Create conflict object
   */
  private createConflict(
    type: SyncConflict["type"],
    codeHash: string,
    blueprintHash: string
  ): SyncConflict {
    return {
      type,
      codeChange: {
        hash: codeHash,
        timestamp: Date.now(),
      },
      blueprintChange: {
        hash: blueprintHash,
        timestamp: Date.now(),
      },
      message: `Conflict detected: ${type}. Both code and Blueprint have been modified.`,
    };
  }

  /**
   * Notify conflict callbacks
   */
  private notifyConflict(conflict: SyncConflict): void {
    for (const callback of this.conflictCallbacks) {
      try {
        callback(conflict);
      } catch (error) {
        console.error("Error in conflict callback:", error);
      }
    }
  }

  /**
   * Notify code change callbacks
   */
  private notifyCodeChange(ast: ProgramNode): void {
    for (const callback of this.codeChangeCallbacks) {
      try {
        callback(ast);
      } catch (error) {
        console.error("Error in code change callback:", error);
      }
    }
  }

  /**
   * Notify Blueprint change callbacks
   */
  private notifyBlueprintChange(graph: BlueprintGraph): void {
    for (const callback of this.blueprintChangeCallbacks) {
      try {
        callback(graph);
      } catch (error) {
        console.error("Error in Blueprint change callback:", error);
      }
    }
  }

  /**
   * Generate hash for AST
   */
  private hashAST(ast: ProgramNode): string {
    // Simple hash based on AST structure
    const str = JSON.stringify({
      functions: ast.functions.map((f) => ({
        name: f.name,
        parameters: f.parameters.length,
        bodyLength: f.body.length,
      })),
      variables: ast.variables.map((v) => v.name),
    });
    return this.simpleHash(str);
  }

  /**
   * Generate hash for Blueprint
   */
  private hashBlueprint(graph: BlueprintGraph): string {
    // Simple hash based on graph structure
    const str = JSON.stringify({
      nodes: graph.nodes.length,
      connections: graph.connections.length,
      nodeTypes: graph.nodes.map((n) => n.type),
    });
    return this.simpleHash(str);
  }

  /**
   * Simple string hash function
   */
  private simpleHash(str: string): string {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      const char = str.charCodeAt(i);
      // Use multiplication instead of bitwise shift
      hash = hash * 32 - hash + char;
      // Convert to 32-bit integer using Math.floor and modulo
      hash = Math.floor(hash % 2_147_483_648);
    }
    return hash.toString(36);
  }

  /**
   * Reset sync state (useful for testing or manual reset)
   */
  reset(): void {
    this.state = {
      lastCodeHash: "",
      lastBlueprintHash: "",
      lastCodeSync: 0,
      lastBlueprintSync: 0,
      syncing: false,
    };
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer);
      this.debounceTimer = null;
    }
  }
}
