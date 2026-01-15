/**
 * Conflict Resolution
 * Handles resolution of sync conflicts between code and Blueprint
 */

import type { SyncConflict } from "./sync-manager";
import type { ProgramNode } from "~/utils/blueprint/ast/ast-core";
import type { BlueprintGraph } from "~/utils/blueprint/graph/node-types";

/**
 * Conflict resolution strategy
 */
export type ConflictResolutionStrategy =
  | "code_wins"
  | "blueprint_wins"
  | "merge"
  | "ask_user";

/**
 * Conflict resolution result
 */
export interface ConflictResolutionResult {
  /** The resolution strategy chosen */
  strategy: ConflictResolutionStrategy;
  /** The resolved code AST (if applicable) */
  resolvedAST?: ProgramNode;
  /** The resolved Blueprint graph (if applicable) */
  resolvedGraph?: BlueprintGraph;
  /** Whether the conflict was resolved */
  resolved: boolean;
}

/**
 * Merge changes from both code and Blueprint
 * This is a simplified merge - in production, you'd want more sophisticated merging
 */
function mergeChanges(
  _conflict: SyncConflict,
  currentAST?: ProgramNode,
  currentGraph?: BlueprintGraph
): ConflictResolutionResult {
  // For now, merge strategy prefers code for functions and Blueprint for layout
  // This is a placeholder - real merging would be more complex
  if (currentAST && currentGraph) {
    // Merge: Use code structure but preserve Blueprint node positions
    return {
      strategy: "merge",
      resolved: true,
      resolvedAST: currentAST,
      resolvedGraph: currentGraph,
    };
  }

  return {
    strategy: "merge",
    resolved: false,
  };
}

/**
 * Resolve a conflict using the specified strategy
 */
export function resolveConflict(
  conflict: SyncConflict,
  strategy: ConflictResolutionStrategy,
  currentAST?: ProgramNode,
  currentGraph?: BlueprintGraph
): ConflictResolutionResult {
  switch (strategy) {
    case "code_wins":
      return {
        strategy: "code_wins",
        resolved: true,
        resolvedAST: currentAST,
        // Blueprint will be regenerated from code
      };

    case "blueprint_wins":
      return {
        strategy: "blueprint_wins",
        resolved: true,
        resolvedGraph: currentGraph,
        // Code will be regenerated from Blueprint
      };

    case "merge":
      return mergeChanges(conflict, currentAST, currentGraph);

    case "ask_user":
      return {
        strategy: "ask_user",
        resolved: false,
      };

    default:
      return {
        strategy: "ask_user",
        resolved: false,
      };
  }
}

/**
 * Detect if a conflict can be auto-resolved
 */
export function canAutoResolve(conflict: SyncConflict): boolean {
  // Simple heuristic: if only one side changed significantly, we can auto-resolve
  const codeChangeTime = conflict.codeChange.timestamp;
  const blueprintChangeTime = conflict.blueprintChange.timestamp;
  const timeDiff = Math.abs(codeChangeTime - blueprintChangeTime);

  // If changes are more than 5 seconds apart, likely separate edits
  return timeDiff > 5000;
}

/**
 * Suggest a resolution strategy based on conflict analysis
 */
export function suggestResolution(
  conflict: SyncConflict
): ConflictResolutionStrategy {
  const codeChangeTime = conflict.codeChange.timestamp;
  const blueprintChangeTime = conflict.blueprintChange.timestamp;

  // If code changed more recently, suggest code wins
  if (codeChangeTime > blueprintChangeTime) {
    return "code_wins";
  }

  // If Blueprint changed more recently, suggest Blueprint wins
  if (blueprintChangeTime > codeChangeTime) {
    return "blueprint_wins";
  }

  // Default to asking user
  return "ask_user";
}

/**
 * Create a user-friendly conflict message
 */
export function createConflictMessage(conflict: SyncConflict): string {
  const codeTime = new Date(conflict.codeChange.timestamp).toLocaleTimeString();
  const blueprintTime = new Date(
    conflict.blueprintChange.timestamp
  ).toLocaleTimeString();

  return `Conflict detected: Both code and Blueprint have been modified.

Code changed at: ${codeTime}
Blueprint changed at: ${blueprintTime}

Please choose how to resolve this conflict:
- Use code changes (overwrite Blueprint)
- Use Blueprint changes (overwrite code)
- Merge both changes (experimental)
- Cancel and resolve manually`;
}

/**
 * Analyze conflict to provide more details
 */
export function analyzeConflict(
  _conflict: SyncConflict,
  currentAST?: ProgramNode,
  currentGraph?: BlueprintGraph
): {
  codeChanges: string[];
  blueprintChanges: string[];
  severity: "low" | "medium" | "high";
} {
  const codeChanges: string[] = [];
  const blueprintChanges: string[] = [];

  if (currentAST) {
    codeChanges.push(`${currentAST.functions.length} functions`);
    codeChanges.push(`${currentAST.variables.length} variables`);
  }

  if (currentGraph) {
    blueprintChanges.push(`${currentGraph.nodes.length} nodes`);
    blueprintChanges.push(`${currentGraph.connections.length} connections`);
  }

  // Determine severity based on change magnitude
  let severity: "low" | "medium" | "high" = "low";
  if (currentAST && currentGraph) {
    const totalChanges =
      currentAST.functions.length + currentGraph.nodes.length;
    if (totalChanges > 20) {
      severity = "high";
    } else if (totalChanges > 10) {
      severity = "medium";
    }
  }

  return {
    codeChanges,
    blueprintChanges,
    severity,
  };
}

/**
 * @deprecated Use individual functions instead. This export is kept for backwards compatibility.
 */
export const ConflictResolver = {
  resolve: resolveConflict,
  canAutoResolve,
  suggestResolution,
  createConflictMessage,
  analyzeConflict,
};
