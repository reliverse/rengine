/**
 * TypeScript wrapper for Pawn code generator
 */

import { invoke } from "@tauri-apps/api/core";

export interface GenerationOptions {
  indentSize?: number;
  useTabs?: boolean;
  preserveComments?: boolean;
}

/**
 * Generate Pawn code from AST
 */
export async function generatePawnCode(
  ast: any,
  options?: GenerationOptions
): Promise<string> {
  try {
    const rustOptions = options
      ? {
          indent_size: options.indentSize ?? 4,
          use_tabs: options.useTabs ?? false,
          preserve_comments: options.preserveComments ?? true,
        }
      : undefined;

    const result = await invoke<string>("generate_blueprint_code", {
      ast,
      language: "pawn",
      options: rustOptions,
    });

    return result;
  } catch (error) {
    throw new Error(
      `Failed to generate Pawn code: ${error instanceof Error ? error.message : String(error)}`
    );
  }
}

/**
 * Generate code with default options
 */
export async function generatePawnCodeDefault(ast: any): Promise<string> {
  return generatePawnCode(ast);
}

/**
 * Generate code with custom indentation
 */
export async function generatePawnCodeWithIndent(
  ast: any,
  indentSize: number,
  useTabs = false
): Promise<string> {
  return generatePawnCode(ast, {
    indentSize,
    useTabs,
    preserveComments: true,
  });
}
