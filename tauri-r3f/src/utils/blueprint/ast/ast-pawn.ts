/**
 * Pawn-specific AST extensions and transformations.
 * Extends the core AST with Pawn language-specific features.
 */

import type {
  FunctionNode,
  TypeNode,
  ParameterNode,
  ExpressionNode,
  StatementNode,
} from "./ast-core";
import { generateASTNodeId } from "./ast-core";

/**
 * Pawn-specific type tags
 */
export type PawnTag = "Float" | "String" | "bool" | "File" | "Text" | string;

export interface PawnTypeNode extends TypeNode {
  tag?: PawnTag;
  isTagged?: boolean;
}

/**
 * Pawn-specific function modifiers
 */
export type PawnFunctionModifier =
  | "public"
  | "stock"
  | "forward"
  | "native"
  | "static";

export interface PawnFunctionNode extends FunctionNode {
  modifiers?: PawnFunctionModifier[];
  isPublic?: boolean;
  isStock?: boolean;
  isForward?: boolean;
  isNative?: boolean;
  isStatic?: boolean;
}

/**
 * Pawn callback function (SAMP-specific)
 */
export interface PawnCallbackNode extends PawnFunctionNode {
  callbackName: string; // e.g., 'OnPlayerConnect', 'OnGameModeInit'
  isCallback: true;
}

/**
 * Pawn native function call
 */
export interface PawnNativeCallNode extends ExpressionNode {
  kind: "call";
  nativeName: string; // e.g., 'CreateDynamicObject', 'SetPlayerPos'
  isNative: true;
}

/**
 * Pawn array declaration
 */
export interface PawnArrayNode extends TypeNode {
  type: "type";
  name: string;
  isArray: true;
  arraySize: number;
  elementType: TypeNode;
}

/**
 * Helper functions for Pawn-specific AST operations
 */

export function createPawnTypeNode(
  name: string,
  tag?: PawnTag,
  isArray = false,
  arraySize?: number
): PawnTypeNode {
  return {
    id: generateASTNodeId("pawn_type"),
    type: "type",
    name,
    tag,
    isTagged: !!tag,
    isArray,
    arraySize,
    position: { line: 0, column: 0 },
  };
}

export function createPawnFunctionNode(
  name: string,
  parameters: ParameterNode[],
  returnType: PawnTypeNode,
  body: StatementNode[],
  modifiers?: PawnFunctionModifier[]
): PawnFunctionNode {
  return {
    id: generateASTNodeId("pawn_func"),
    type: "function",
    name,
    parameters,
    returnType,
    body,
    modifiers,
    isPublic: modifiers?.includes("public"),
    isStock: modifiers?.includes("stock"),
    isForward: modifiers?.includes("forward"),
    isNative: modifiers?.includes("native"),
    isStatic: modifiers?.includes("static"),
    position: { line: 0, column: 0 },
  };
}

export function createPawnCallbackNode(
  callbackName: string,
  parameters: ParameterNode[],
  returnType: PawnTypeNode,
  body: StatementNode[]
): PawnCallbackNode {
  return {
    ...createPawnFunctionNode(callbackName, parameters, returnType, body, [
      "public",
    ]),
    callbackName,
    isCallback: true,
  };
}

export function createPawnNativeCallNode(
  nativeName: string,
  args: ExpressionNode[]
): PawnNativeCallNode {
  return {
    id: generateASTNodeId("pawn_native"),
    type: "expression",
    kind: "call",
    expression: {
      callee: nativeName,
      arguments: args,
    },
    nativeName,
    isNative: true,
    position: { line: 0, column: 0 },
  };
}

/**
 * Common Pawn types
 */
export const PAWN_TYPES = {
  INT: createPawnTypeNode("int"),
  FLOAT: createPawnTypeNode("Float", "Float"),
  BOOL: createPawnTypeNode("bool"),
  STRING: createPawnTypeNode("String", "String"),
  VOID: createPawnTypeNode("void"),
} as const;

/**
 * Check if a type node is a Pawn-specific type
 */
export function isPawnTypeNode(node: TypeNode): node is PawnTypeNode {
  return "tag" in node || "isTagged" in node;
}

/**
 * Check if a function node is a Pawn callback
 */
export function isPawnCallbackNode(
  node: FunctionNode
): node is PawnCallbackNode {
  return "isCallback" in node && (node as any).isCallback === true;
}

/**
 * Check if an expression is a Pawn native call
 */
export function isPawnNativeCallNode(
  node: ExpressionNode
): node is PawnNativeCallNode {
  return (
    "isNative" in node &&
    (node as any).isNative === true &&
    node.kind === "call"
  );
}
