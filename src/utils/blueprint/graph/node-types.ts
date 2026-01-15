/**
 * Blueprint node type definitions.
 * Defines the visual node types that can appear in a Blueprint graph.
 */

import type { DataType } from "../ast/ast-core";

export type BlueprintNodeType =
  | "function"
  | "variable"
  | "constant"
  | "call"
  | "expression"
  | "if"
  | "while"
  | "for"
  | "return"
  | "break"
  | "continue"
  | "assignment"
  | "literal"
  | "binary"
  | "unary"
  | "member"
  | "index"
  | "parameter"
  | "event"
  | "callback"
  | "native"
  | "comment"
  | "group";

export type PinDirection = "input" | "output" | "exec";

export type PinType = DataType | "exec" | "flow";

export interface Pin {
  id: string;
  name: string;
  type: PinType;
  direction: PinDirection;
  defaultValue?: any;
  required?: boolean;
  connected?: boolean;
  metadata?: Record<string, any>;
}

export interface BlueprintNode {
  id: string;
  type: BlueprintNodeType;
  title: string;
  category?: string;
  position: { x: number; y: number };
  size?: { width: number; height: number };
  inputs: Pin[];
  outputs: Pin[];
  properties: Record<string, any>;
  metadata?: Record<string, any>;
  selected?: boolean;
  collapsed?: boolean;
}

export interface BlueprintConnection {
  id: string;
  sourceNodeId: string;
  sourcePinId: string;
  targetNodeId: string;
  targetPinId: string;
  metadata?: Record<string, any>;
}

export interface BlueprintGraph {
  id: string;
  name: string;
  language: "pawn" | "typescript" | "rust" | "cpp";
  nodes: BlueprintNode[];
  connections: BlueprintConnection[];
  metadata: GraphMetadata;
  version: string;
}

export interface GraphMetadata {
  createdAt: string;
  modifiedAt: string;
  author?: string;
  description?: string;
  tags?: string[];
}

/**
 * Node category definitions for organization
 */
export const NODE_CATEGORIES = {
  FLOW: "Flow Control",
  VARIABLES: "Variables",
  FUNCTIONS: "Functions",
  OPERATORS: "Operators",
  LITERALS: "Literals",
  EVENTS: "Events",
  NATIVES: "Natives",
  UTILITIES: "Utilities",
} as const;

/**
 * Helper function to generate unique node IDs
 */
export function generateNodeId(prefix = "node"): string {
  return `${prefix}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * Helper function to generate unique pin IDs
 */
export function generatePinId(prefix = "pin"): string {
  return `${prefix}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * Helper function to generate unique connection IDs
 */
export function generateConnectionId(prefix = "conn"): string {
  return `${prefix}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * Create a basic Blueprint node
 */
export function createBlueprintNode(
  type: BlueprintNodeType,
  title: string,
  position: { x: number; y: number },
  inputs: Pin[] = [],
  outputs: Pin[] = [],
  properties: Record<string, any> = {}
): BlueprintNode {
  return {
    id: generateNodeId(),
    type,
    title,
    position,
    inputs,
    outputs,
    properties,
    selected: false,
    collapsed: false,
  };
}

/**
 * Create an execution pin (for flow control)
 */
export function createExecPin(direction: PinDirection, name = "Exec"): Pin {
  return {
    id: generatePinId(),
    name,
    type: "exec",
    direction,
    required: false,
    connected: false,
  };
}

/**
 * Create a data pin
 */
export function createDataPin(
  direction: PinDirection,
  name: string,
  type: DataType,
  required = false,
  defaultValue?: any
): Pin {
  return {
    id: generatePinId(),
    name,
    type,
    direction,
    required,
    defaultValue,
    connected: false,
  };
}
