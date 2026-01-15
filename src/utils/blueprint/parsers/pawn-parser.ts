/**
 * TypeScript wrapper for Pawn parser
 */

import { invoke } from "@tauri-apps/api/core";

export interface ParseResult {
  ast: any;
  errors: string[];
  warnings: string[];
  language: string;
}

/**
 * Parse Pawn code into AST
 */
export async function parsePawnCode(source: string): Promise<ParseResult> {
  try {
    const result = await invoke<ParseResult>("parse_blueprint_code", {
      source,
      language: "pawn",
    });
    return result;
  } catch (error) {
    throw new Error(
      `Failed to parse Pawn code: ${error instanceof Error ? error.message : String(error)}`
    );
  }
}
