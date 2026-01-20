/**
 * Blueprint to Code Sync
 * Handles syncing Blueprint changes to code representation
 */

import { invoke } from "@tauri-apps/api/core";
import type { BlueprintGraph } from "~/utils/blueprint/graph/node-types";
import { convertBlueprintToAST } from "~/utils/blueprint/converters/blueprint-to-ast";
import type { SyncManager } from "./sync-manager";

/**
 * Options for Blueprint to code sync
 */
export interface BlueprintToCodeOptions {
  /** Language to generate code in */
  language: "pawn" | "typescript" | "rust" | "cpp";
  /** Whether to preserve formatting */
  preserveFormatting?: boolean;
  /** Sync manager instance */
  syncManager?: SyncManager;
}

/**
 * Convert Blueprint to code
 */
export async function syncBlueprintToCode(
  graph: BlueprintGraph,
  options: BlueprintToCodeOptions
): Promise<string> {
  const { language, preserveFormatting = true, syncManager } = options;

  // Convert Blueprint to AST
  const ast = convertBlueprintToAST(graph, {
    preservePositions: true,
  });

  // Notify sync manager if provided
  if (syncManager) {
    syncManager.handleBlueprintChange(graph);
  }

  // Generate code from AST
  const code = await invoke<string>("generate_blueprint_code", {
    ast: JSON.parse(JSON.stringify(ast)),
    language,
    options: {
      indentSize: 4,
      useTabs: false,
      preserveFormatting,
    },
  });

  return code;
}

/**
 * Watch a Blueprint file and sync changes to code
 *
 * Note: This implementation uses polling for file changes since Tauri doesn't
 * have a built-in file watcher. For production, consider using a native file
 * watcher via a Tauri plugin or Rust backend command.
 */
export function watchBlueprintFile(
  filePath: string,
  options: BlueprintToCodeOptions,
  onCodeChange: (code: string) => void,
  onError?: (error: Error) => void
): () => void {
  let isWatching = true;
  let lastModified: number | null = null;
  let pollInterval: ReturnType<typeof setInterval> | null = null;

  const syncFile = async () => {
    if (!isWatching) return;

    try {
      // Read Blueprint file
      const graphJson = await invoke<string>("read_file", { path: filePath });

      // Simple content-based change detection
      const contentHash = graphJson.length; // In production, use a proper hash
      if (lastModified !== null && lastModified === contentHash) {
        return; // No changes detected
      }

      lastModified = contentHash;

      // Parse and convert to code
      const graph = JSON.parse(graphJson) as BlueprintGraph;
      const code = await syncBlueprintToCode(graph, options);
      onCodeChange(code);
    } catch (error) {
      if (onError) {
        onError(error as Error);
      } else {
        console.error("Error syncing Blueprint file:", error);
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
 * Sync Blueprint graph to code string (one-time)
 */
export async function syncBlueprintGraphToCode(
  graph: BlueprintGraph,
  options: BlueprintToCodeOptions
): Promise<string> {
  return syncBlueprintToCode(graph, options);
}
