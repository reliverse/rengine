/**
 * Blueprint File Management
 * Handles creating, opening, saving, and exporting Blueprint files
 */

import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { BlueprintGraph } from "~/utils/blueprint/graph/node-types";
import {
  serializeGraph,
  deserializeGraph,
} from "~/utils/blueprint/graph/graph-serialization";
import { createBlueprintGraph } from "~/utils/blueprint/graph/blueprint-graph";
import { syncBlueprintToCode } from "./sync/blueprint-to-code";

/**
 * File management options
 */
export interface BlueprintFileOptions {
  /** Default file name */
  defaultName?: string;
  /** File filters for dialog */
  filters?: Array<{ name: string; extensions: string[] }>;
}

const DEFAULT_OPTIONS: BlueprintFileOptions = {
  defaultName: "untitled",
  filters: [
    { name: "Blueprint Files", extensions: ["blueprint", "json"] },
    { name: "All Files", extensions: ["*"] },
  ],
};

// Regex patterns for file path parsing (moved to top level for performance)
const PATH_SEPARATOR_REGEX = /[/\\]/;
const FILE_EXTENSION_REGEX = /\.[^/.]+$/;

/**
 * Create a new Blueprint file
 */
export function createNewBlueprint(
  language: BlueprintGraph["language"] = "pawn"
): BlueprintGraph {
  return createBlueprintGraph("New Blueprint", language);
}

/**
 * Open a Blueprint file from disk
 */
export async function openBlueprintFile(
  filePath?: string
): Promise<BlueprintGraph | null> {
  try {
    let selectedPath = filePath;

    // If no path provided, show open dialog
    if (!selectedPath) {
      const result = await open({
        filters: DEFAULT_OPTIONS.filters,
        title: "Open Blueprint File",
      });

      if (!result || Array.isArray(result)) {
        return null;
      }

      selectedPath = result;
    }

    // Read file
    const content = await invoke<string>("read_file", {
      path: selectedPath,
    });

    // Parse JSON
    const graphData = JSON.parse(content);

    // Deserialize graph
    const graph = deserializeGraph(graphData);

    return graph;
  } catch (error) {
    console.error("Failed to open Blueprint file:", error);
    throw error;
  }
}

/**
 * Save a Blueprint file to disk
 */
export async function saveBlueprintFile(
  graph: BlueprintGraph,
  filePath?: string
): Promise<string | null> {
  try {
    let selectedPath = filePath;

    // If no path provided, show save dialog
    if (!selectedPath) {
      const result = await save({
        filters: DEFAULT_OPTIONS.filters,
        title: "Save Blueprint File",
        defaultPath: `${DEFAULT_OPTIONS.defaultName}.blueprint`,
      });

      if (!result) {
        return null;
      }

      selectedPath = result;
    }

    // Serialize graph
    const graphData = serializeGraph(graph);

    // Write file
    await invoke("write_file", {
      filePath: selectedPath,
      content: JSON.stringify(graphData, null, 2),
    });

    return selectedPath;
  } catch (error) {
    console.error("Failed to save Blueprint file:", error);
    throw error;
  }
}

/**
 * Export Blueprint to Pawn code (.pwn file)
 */
export async function exportBlueprintToPwn(
  graph: BlueprintGraph,
  filePath?: string
): Promise<string | null> {
  try {
    let selectedPath = filePath;

    // If no path provided, show save dialog
    if (!selectedPath) {
      const result = await save({
        filters: [
          { name: "Pawn Files", extensions: ["pwn"] },
          { name: "All Files", extensions: ["*"] },
        ],
        title: "Export Blueprint to Pawn",
        defaultPath: `${graph.name || "blueprint"}.pwn`,
      });

      if (!result) {
        return null;
      }

      selectedPath = result;
    }

    // Convert Blueprint to code
    const code = await syncBlueprintToCode(graph, {
      language: "pawn",
      preserveFormatting: true,
    });

    // Write file
    await invoke("write_file", {
      filePath: selectedPath,
      content: code,
    });

    return selectedPath;
  } catch (error) {
    console.error("Failed to export Blueprint to Pawn:", error);
    throw error;
  }
}

/**
 * Import Pawn code file and convert to Blueprint
 */
export async function importPwnToBlueprint(
  filePath?: string
): Promise<BlueprintGraph | null> {
  try {
    let selectedPath = filePath;

    // If no path provided, show open dialog
    if (!selectedPath) {
      const result = await open({
        filters: [
          { name: "Pawn Files", extensions: ["pwn"] },
          { name: "All Files", extensions: ["*"] },
        ],
        title: "Import Pawn File",
      });

      if (!result || Array.isArray(result)) {
        return null;
      }

      selectedPath = result;
    }

    // Read file
    const code = await invoke<string>("read_file", {
      path: selectedPath,
    });

    // Parse code to AST
    const parseResult = await invoke<{
      ast: any;
      errors: any[];
      warnings: any[];
      language: string;
    }>("parse_blueprint_code", {
      source: code,
      language: "pawn",
    });

    if (parseResult.errors && parseResult.errors.length > 0) {
      throw new Error(`Parse errors: ${JSON.stringify(parseResult.errors)}`);
    }

    // Convert AST to Blueprint
    const { convertASTToBlueprint } = await import(
      "./converters/ast-to-blueprint"
    );
    const graph = convertASTToBlueprint(parseResult.ast as any, {
      preservePositions: true,
    });

    // Update graph name from file name
    if (selectedPath) {
      const fileName =
        selectedPath
          .split(PATH_SEPARATOR_REGEX)
          .pop()
          ?.replace(FILE_EXTENSION_REGEX, "") || "Imported Blueprint";
      graph.name = fileName;
    }

    return graph;
  } catch (error) {
    console.error("Failed to import Pawn file:", error);
    throw error;
  }
}
