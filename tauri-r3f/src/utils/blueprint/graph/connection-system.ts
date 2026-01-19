/**
 * Connection system for Blueprint nodes.
 * Handles pin connections, validation, and connection queries.
 */

import type {
  BlueprintGraph,
  BlueprintNode,
  BlueprintConnection,
  Pin,
} from "./node-types";

/**
 * Check if two pins can be connected
 */
export function canConnectPins(sourcePin: Pin, targetPin: Pin): boolean {
  // Pins must have opposite directions
  if (sourcePin.direction === targetPin.direction) {
    return false;
  }

  // Source must be output, target must be input
  if (sourcePin.direction !== "output" || targetPin.direction !== "input") {
    return false;
  }

  // Exec pins can only connect to exec pins
  if (sourcePin.type === "exec" && targetPin.type !== "exec") {
    return false;
  }
  if (targetPin.type === "exec" && sourcePin.type !== "exec") {
    return false;
  }

  // Flow pins can only connect to flow pins
  if (sourcePin.type === "flow" && targetPin.type !== "flow") {
    return false;
  }
  if (targetPin.type === "flow" && sourcePin.type !== "flow") {
    return false;
  }

  // Data types must be compatible (exact match for now, can be extended)
  if (
    sourcePin.type !== "exec" &&
    sourcePin.type !== "flow" &&
    targetPin.type !== "exec" &&
    targetPin.type !== "flow"
  ) {
    // Allow 'any' type to connect to anything
    if (sourcePin.type === "any" || targetPin.type === "any") {
      return true;
    }

    // Types must match
    if (sourcePin.type !== targetPin.type) {
      return false;
    }
  }

  return true;
}

/**
 * Find a pin by ID in a node
 */
export function findPinInNode(
  node: BlueprintNode,
  pinId: string
): Pin | undefined {
  return [...node.inputs, ...node.outputs].find((pin) => pin.id === pinId);
}

/**
 * Get all input pins for a node
 */
export function getInputPins(node: BlueprintNode): Pin[] {
  return node.inputs.filter((pin) => pin.direction === "input");
}

/**
 * Get all output pins for a node
 */
export function getOutputPins(node: BlueprintNode): Pin[] {
  return node.outputs.filter((pin) => pin.direction === "output");
}

/**
 * Get all execution pins for a node
 */
export function getExecPins(node: BlueprintNode): Pin[] {
  return [...node.inputs, ...node.outputs].filter((pin) => pin.type === "exec");
}

/**
 * Get all data pins for a node
 */
export function getDataPins(node: BlueprintNode): Pin[] {
  return [...node.inputs, ...node.outputs].filter(
    (pin) => pin.type !== "exec" && pin.type !== "flow"
  );
}

/**
 * Check if a pin is connected
 */
export function isPinConnected(
  graph: BlueprintGraph,
  nodeId: string,
  pinId: string
): boolean {
  return graph.connections.some(
    (conn) =>
      (conn.sourceNodeId === nodeId && conn.sourcePinId === pinId) ||
      (conn.targetNodeId === nodeId && conn.targetPinId === pinId)
  );
}

/**
 * Get all connections for a specific pin
 */
export function getPinConnections(
  graph: BlueprintGraph,
  nodeId: string,
  pinId: string
): BlueprintConnection[] {
  return graph.connections.filter(
    (conn) =>
      (conn.sourceNodeId === nodeId && conn.sourcePinId === pinId) ||
      (conn.targetNodeId === nodeId && conn.targetPinId === pinId)
  );
}

/**
 * Get the node connected to a pin
 */
export function getConnectedNode(
  graph: BlueprintGraph,
  nodeId: string,
  pinId: string
): BlueprintNode | undefined {
  const connection = graph.connections.find(
    (conn) =>
      (conn.sourceNodeId === nodeId && conn.sourcePinId === pinId) ||
      (conn.targetNodeId === nodeId && conn.targetPinId === pinId)
  );

  if (!connection) {
    return undefined;
  }

  const connectedNodeId =
    connection.sourceNodeId === nodeId
      ? connection.targetNodeId
      : connection.sourceNodeId;

  return graph.nodes.find((node) => node.id === connectedNodeId);
}

/**
 * Get all nodes that provide input to a specific node
 */
export function getInputNodes(
  graph: BlueprintGraph,
  nodeId: string
): BlueprintNode[] {
  const node = graph.nodes.find((n) => n.id === nodeId);
  if (!node) {
    return [];
  }

  const inputConnections = graph.connections.filter(
    (conn) => conn.targetNodeId === nodeId
  );
  const inputNodeIds = new Set(
    inputConnections.map((conn) => conn.sourceNodeId)
  );

  return graph.nodes.filter((node) => inputNodeIds.has(node.id));
}

/**
 * Get all nodes that receive output from a specific node
 */
export function getOutputNodes(
  graph: BlueprintGraph,
  nodeId: string
): BlueprintNode[] {
  const node = graph.nodes.find((n) => n.id === nodeId);
  if (!node) {
    return [];
  }

  const outputConnections = graph.connections.filter(
    (conn) => conn.sourceNodeId === nodeId
  );
  const outputNodeIds = new Set(
    outputConnections.map((conn) => conn.targetNodeId)
  );

  return graph.nodes.filter((node) => outputNodeIds.has(node.id));
}

/**
 * Validate a connection before adding it
 */
export function validateConnection(
  graph: BlueprintGraph,
  sourceNodeId: string,
  sourcePinId: string,
  targetNodeId: string,
  targetPinId: string
): { valid: boolean; error?: string } {
  // Check if nodes exist
  const sourceNode = graph.nodes.find((n) => n.id === sourceNodeId);
  const targetNode = graph.nodes.find((n) => n.id === targetNodeId);

  if (!sourceNode) {
    return { valid: false, error: "Source node not found" };
  }

  if (!targetNode) {
    return { valid: false, error: "Target node not found" };
  }

  // Check if pins exist
  const sourcePin = findPinInNode(sourceNode, sourcePinId);
  const targetPin = findPinInNode(targetNode, targetPinId);

  if (!sourcePin) {
    return { valid: false, error: "Source pin not found" };
  }

  if (!targetPin) {
    return { valid: false, error: "Target pin not found" };
  }

  // Check if pins can be connected
  if (!canConnectPins(sourcePin, targetPin)) {
    return { valid: false, error: "Pins are not compatible" };
  }

  // Check if connection already exists
  const exists = graph.connections.some(
    (conn) =>
      conn.sourceNodeId === sourceNodeId &&
      conn.sourcePinId === sourcePinId &&
      conn.targetNodeId === targetNodeId &&
      conn.targetPinId === targetPinId
  );

  if (exists) {
    return { valid: false, error: "Connection already exists" };
  }

  // Check if target pin already has a connection (for non-exec pins)
  if (
    targetPin.type !== "exec" &&
    isPinConnected(graph, targetNodeId, targetPinId)
  ) {
    return { valid: false, error: "Target pin is already connected" };
  }

  return { valid: true };
}

/**
 * Get the execution flow starting from a node
 */
export function getExecutionFlow(
  graph: BlueprintGraph,
  startNodeId: string
): BlueprintNode[] {
  const visited = new Set<string>();
  const flow: BlueprintNode[] = [];

  function traverse(nodeId: string) {
    if (visited.has(nodeId)) {
      return;
    }

    visited.add(nodeId);
    const node = graph.nodes.find((n) => n.id === nodeId);
    if (node) {
      flow.push(node);
    }

    // Follow execution connections
    const execOutputs = graph.connections.filter(
      (conn) => conn.sourceNodeId === nodeId
    );

    for (const conn of execOutputs) {
      const sourceNode = graph.nodes.find((n) => n.id === nodeId);
      if (sourceNode) {
        const sourcePin = findPinInNode(sourceNode, conn.sourcePinId);
        if (sourcePin?.type === "exec") {
          traverse(conn.targetNodeId);
        }
      }
    }
  }

  traverse(startNodeId);
  return flow;
}
