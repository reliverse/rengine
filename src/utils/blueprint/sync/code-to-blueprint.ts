/**
 * Code to Blueprint Sync
 * Handles syncing code changes to Blueprint representation
 */

import { invoke } from "@tauri-apps/api/core";
import type { ProgramNode } from "~/utils/blueprint/ast/ast-core";
import type { BlueprintGraph } from "~/utils/blueprint/graph/node-types";
import { convertASTToBlueprint } from "~/utils/blueprint/converters/ast-to-blueprint";
import type { SyncManager } from "./sync-manager";

/**
 * Options for code to Blueprint sync
 */
export interface CodeToBlueprintOptions {
  /** Language of the code */
  language: "pawn" | "typescript" | "rust" | "cpp";
  /** Whether to preserve node positions */
  preservePositions?: boolean;
  /** Sync manager instance */
  syncManager?: SyncManager;
}

/**
 * Parse code and convert to Blueprint
 */
export async function syncCodeToBlueprint(
  code: string,
  options: CodeToBlueprintOptions
): Promise<BlueprintGraph> {
  const { language, preservePositions = true, syncManager } = options;

  // Parse code to AST
  const parseResult = await invoke<{
    ast: any;
    errors: any[];
    warnings: any[];
    language: string;
  }>("parse_blueprint_code", {
    source: code,
    language,
  });

  if (parseResult.errors && parseResult.errors.length > 0) {
    throw new Error(`Parse errors: ${JSON.stringify(parseResult.errors)}`);
  }

  // Convert AST to Blueprint
  const ast = parseResult.ast as ProgramNode;
  const graph = convertASTToBlueprint(ast, {
    preservePositions,
  });

  // Notify sync manager if provided
  if (syncManager) {
    syncManager.handleCodeChange(ast);
  }

  return graph;
}

/**
 * Watch a code file and sync changes to Blueprint
 *
 * Note: This implementation uses polling for file changes since Tauri doesn't
 * have a built-in file watcher. For production, consider using a native file
 * watcher via a Tauri plugin or Rust backend command.
 */
export function watchCodeFile(
  filePath: string,
  options: CodeToBlueprintOptions,
  onGraphChange: (graph: BlueprintGraph) => void,
  onError?: (error: Error) => void
): () => void {
  let isWatching = true;
  let lastModified: number | null = null;
  let pollInterval: ReturnType<typeof setInterval> | null = null;

  const syncFile = async () => {
    if (!isWatching) return;

    try {
      // Check file modification time (would need Tauri command for this)
      // For now, we'll read the file and compare content hash
      const code = await invoke<string>("read_file", { path: filePath });

      // Simple content-based change detection
      const contentHash = code.length; // In production, use a proper hash
      if (lastModified !== null && lastModified === contentHash) {
        return; // No changes detected
      }

      lastModified = contentHash;

      // Parse and convert to Blueprint
      const graph = await syncCodeToBlueprint(code, options);
      onGraphChange(graph);
    } catch (error) {
      if (onError) {
        onError(error as Error);
      } else {
        console.error("Error syncing code file:", error);
      }
    }
  };

  // Initial sync
  syncFile();

  // Poll for changes every 2 seconds
  // In production, this should be replaced with a proper file watcher
  pollInterval = setInterval(() => {
    if (isWatching) {
      syncFile();
    }
  }, 2000);

  // Return cleanup function
  return () => {
    isWatching = false;
    if (pollInterval) {
      clearInterval(pollInterval);
      pollInterval = null;
    }
  };
}

/**
 * Sync code string to Blueprint (one-time)
 */
export async function syncCodeStringToBlueprint(
  code: string,
  options: CodeToBlueprintOptions
): Promise<BlueprintGraph> {
  return syncCodeToBlueprint(code, options);
}
