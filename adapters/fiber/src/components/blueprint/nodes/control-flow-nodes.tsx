/**
 * Control flow node components
 * If/else, loops, and other control structures
 */

import type { BlueprintNode } from "~/utils/blueprint/graph/node-types";

/**
 * Node factory for control flow nodes
 */
export interface ControlFlowNodeConfig {
  type: "if" | "while" | "for" | "switch";
  position: { x: number; y: number };
  customProperties?: Record<string, any>;
}

/**
 * Create an enhanced if node with proper structure
 */
export function createIfNode(position: {
  x: number;
  y: number;
}): BlueprintNode {
  return {
    id: `node_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    type: "if",
    title: "If",
    category: "Flow Control",
    position,
    inputs: [
      {
        id: `pin_${Date.now()}_exec_in`,
        name: "Exec",
        type: "exec",
        direction: "input",
        required: false,
        connected: false,
      },
      {
        id: `pin_${Date.now()}_condition`,
        name: "Condition",
        type: "bool",
        direction: "input",
        required: true,
        connected: false,
      },
    ],
    outputs: [
      {
        id: `pin_${Date.now()}_exec_then`,
        name: "Then",
        type: "exec",
        direction: "output",
        required: false,
        connected: false,
      },
      {
        id: `pin_${Date.now()}_exec_else`,
        name: "Else",
        type: "exec",
        direction: "output",
        required: false,
        connected: false,
      },
    ],
    properties: {
      description:
        "Conditional branching - executes Then if condition is true, Else otherwise",
    },
    selected: false,
    collapsed: false,
  };
}

/**
 * Create an enhanced while loop node
 */
export function createWhileNode(position: {
  x: number;
  y: number;
}): BlueprintNode {
  return {
    id: `node_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    type: "while",
    title: "While Loop",
    category: "Flow Control",
    position,
    inputs: [
      {
        id: `pin_${Date.now()}_exec_in`,
        name: "Exec",
        type: "exec",
        direction: "input",
        required: false,
        connected: false,
      },
      {
        id: `pin_${Date.now()}_condition`,
        name: "Condition",
        type: "bool",
        direction: "input",
        required: true,
        connected: false,
      },
    ],
    outputs: [
      {
        id: `pin_${Date.now()}_exec_loop`,
        name: "Loop",
        type: "exec",
        direction: "output",
        required: false,
        connected: false,
      },
      {
        id: `pin_${Date.now()}_exec_completed`,
        name: "Completed",
        type: "exec",
        direction: "output",
        required: false,
        connected: false,
      },
    ],
    properties: {
      description:
        "Loops while condition is true. Loop output executes each iteration, Completed executes when condition becomes false",
    },
    selected: false,
    collapsed: false,
  };
}

/**
 * Create an enhanced for loop node
 */
export function createForNode(position: {
  x: number;
  y: number;
}): BlueprintNode {
  return {
    id: `node_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    type: "for",
    title: "For Loop",
    category: "Flow Control",
    position,
    inputs: [
      {
        id: `pin_${Date.now()}_exec_in`,
        name: "Exec",
        type: "exec",
        direction: "input",
        required: false,
        connected: false,
      },
      {
        id: `pin_${Date.now()}_start`,
        name: "Start",
        type: "int",
        direction: "input",
        required: true,
        connected: false,
      },
      {
        id: `pin_${Date.now()}_end`,
        name: "End",
        type: "int",
        direction: "input",
        required: true,
        connected: false,
      },
    ],
    outputs: [
      {
        id: `pin_${Date.now()}_exec_loop`,
        name: "Loop",
        type: "exec",
        direction: "output",
        required: false,
        connected: false,
      },
      {
        id: `pin_${Date.now()}_exec_completed`,
        name: "Completed",
        type: "exec",
        direction: "output",
        required: false,
        connected: false,
      },
      {
        id: `pin_${Date.now()}_index`,
        name: "Index",
        type: "int",
        direction: "output",
        required: false,
        connected: false,
      },
    ],
    properties: {
      description:
        "Iterates from Start to End. Index output provides current iteration value",
    },
    selected: false,
    collapsed: false,
  };
}
