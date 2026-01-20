/**
 * Blueprint graph data structure and operations.
 * Manages nodes, connections, and graph-level operations.
 */

import type {
  BlueprintNode,
  BlueprintConnection,
  BlueprintGraph,
} from "./node-types";
import {
  generateNodeId,
  generateConnectionId,
  generatePinId,
} from "./node-types";

/**
 * Create a new empty Blueprint graph
 */
export function createBlueprintGraph(
  name: string,
  language: BlueprintGraph["language"] = "pawn"
): BlueprintGraph {
  const now = new Date().toISOString();
  return {
    id: generateNodeId("graph"),
    name,
    language,
    nodes: [],
    connections: [],
    metadata: {
      createdAt: now,
      modifiedAt: now,
    },
    version: "1.0.0",
  };
}

/**
 * Add a node to the graph
 */
export function addNode(
  graph: BlueprintGraph,
  node: BlueprintNode
): BlueprintGraph {
  return {
    ...graph,
    nodes: [...graph.nodes, node],
    metadata: {
      ...graph.metadata,
      modifiedAt: new Date().toISOString(),
    },
  };
}

/**
 * Remove a node from the graph
 */
export function removeNode(
  graph: BlueprintGraph,
  nodeId: string
): BlueprintGraph {
  // Remove the node
  const nodes = graph.nodes.filter((n) => n.id !== nodeId);

  // Remove all connections involving this node
  const connections = graph.connections.filter(
    (conn) => conn.sourceNodeId !== nodeId && conn.targetNodeId !== nodeId
  );

  return {
    ...graph,
    nodes,
    connections,
    metadata: {
      ...graph.metadata,
      modifiedAt: new Date().toISOString(),
    },
  };
}

/**
 * Update a node in the graph
 */
export function updateNode(
  graph: BlueprintGraph,
  nodeId: string,
  updates: Partial<BlueprintNode>
): BlueprintGraph {
  return {
    ...graph,
    nodes: graph.nodes.map((node) =>
      node.id === nodeId ? { ...node, ...updates } : node
    ),
    metadata: {
      ...graph.metadata,
      modifiedAt: new Date().toISOString(),
    },
  };
}

/**
 * Move a node to a new position
 */
export function moveNode(
  graph: BlueprintGraph,
  nodeId: string,
  position: { x: number; y: number }
): BlueprintGraph {
  return updateNode(graph, nodeId, { position });
}

/**
 * Add a connection between two pins
 */
export function addConnection(
  graph: BlueprintGraph,
  sourceNodeId: string,
  sourcePinId: string,
  targetNodeId: string,
  targetPinId: string
): BlueprintGraph {
  // Check if connection already exists
  const exists = graph.connections.some(
    (conn) =>
      conn.sourceNodeId === sourceNodeId &&
      conn.sourcePinId === sourcePinId &&
      conn.targetNodeId === targetNodeId &&
      conn.targetPinId === targetPinId
  );

  if (exists) {
    return graph;
  }

  // Validate connection
  const sourceNode = graph.nodes.find((n) => n.id === sourceNodeId);
  const targetNode = graph.nodes.find((n) => n.id === targetNodeId);

  if (!(sourceNode && targetNode)) {
    return graph;
  }

  const sourcePin = [...sourceNode.outputs, ...sourceNode.inputs].find(
    (p) => p.id === sourcePinId
  );
  const targetPin = [...targetNode.inputs, ...targetNode.outputs].find(
    (p) => p.id === targetPinId
  );

  if (!(sourcePin && targetPin)) {
    return graph;
  }

  // Check type compatibility (exec pins can only connect to exec pins)
  if (sourcePin.type === "exec" && targetPin.type !== "exec") {
    return graph;
  }
  if (targetPin.type === "exec" && sourcePin.type !== "exec") {
    return graph;
  }

  // Check if target pin is an input
  if (targetPin.direction !== "input") {
    return graph;
  }

  // Check if source pin is an output
  if (sourcePin.direction !== "output") {
    return graph;
  }

  const connection: BlueprintConnection = {
    id: generateConnectionId(),
    sourceNodeId,
    sourcePinId,
    targetNodeId,
    targetPinId,
  };

  // Update pin connected status
  const updatedSourceNode = {
    ...sourceNode,
    outputs: sourceNode.outputs.map((p) =>
      p.id === sourcePinId ? { ...p, connected: true } : p
    ),
  };

  const updatedTargetNode = {
    ...targetNode,
    inputs: targetNode.inputs.map((p) =>
      p.id === targetPinId ? { ...p, connected: true } : p
    ),
  };

  return {
    ...graph,
    nodes: graph.nodes.map((node) => {
      if (node.id === sourceNodeId) return updatedSourceNode;
      if (node.id === targetNodeId) return updatedTargetNode;
      return node;
    }),
    connections: [...graph.connections, connection],
    metadata: {
      ...graph.metadata,
      modifiedAt: new Date().toISOString(),
    },
  };
}

/**
 * Remove a connection
 */
export function removeConnection(
  graph: BlueprintGraph,
  connectionId: string
): BlueprintGraph {
  const connection = graph.connections.find((c) => c.id === connectionId);
  if (!connection) {
    return graph;
  }

  // Update pin connected status
  const sourceNode = graph.nodes.find((n) => n.id === connection.sourceNodeId);
  const targetNode = graph.nodes.find((n) => n.id === connection.targetNodeId);

  let updatedNodes = graph.nodes;

  if (sourceNode) {
    updatedNodes = updatedNodes.map((node) => {
      if (node.id === sourceNode.id) {
        return {
          ...node,
          outputs: node.outputs.map((p) =>
            p.id === connection.sourcePinId ? { ...p, connected: false } : p
          ),
        };
      }
      return node;
    });
  }

  if (targetNode) {
    updatedNodes = updatedNodes.map((node) => {
      if (node.id === targetNode.id) {
        return {
          ...node,
          inputs: node.inputs.map((p) =>
            p.id === connection.targetPinId ? { ...p, connected: false } : p
          ),
        };
      }
      return node;
    });
  }

  return {
    ...graph,
    nodes: updatedNodes,
    connections: graph.connections.filter((c) => c.id !== connectionId),
    metadata: {
      ...graph.metadata,
      modifiedAt: new Date().toISOString(),
    },
  };
}

/**
 * Get all connections for a node
 */
export function getNodeConnections(
  graph: BlueprintGraph,
  nodeId: string
): BlueprintConnection[] {
  return graph.connections.filter(
    (conn) => conn.sourceNodeId === nodeId || conn.targetNodeId === nodeId
  );
}

/**
 * Get all nodes connected to a specific node
 */
export function getConnectedNodes(
  graph: BlueprintGraph,
  nodeId: string
): BlueprintNode[] {
  const connections = getNodeConnections(graph, nodeId);
  const connectedNodeIds = new Set<string>();

  for (const conn of connections) {
    if (conn.sourceNodeId === nodeId) {
      connectedNodeIds.add(conn.targetNodeId);
    }
    if (conn.targetNodeId === nodeId) {
      connectedNodeIds.add(conn.sourceNodeId);
    }
  }

  return graph.nodes.filter((node) => connectedNodeIds.has(node.id));
}

/**
 * Select nodes
 */
export function selectNodes(
  graph: BlueprintGraph,
  nodeIds: string[]
): BlueprintGraph {
  return {
    ...graph,
    nodes: graph.nodes.map((node) => ({
      ...node,
      selected: nodeIds.includes(node.id),
    })),
  };
}

/**
 * Clear selection
 */
export function clearSelection(graph: BlueprintGraph): BlueprintGraph {
  return {
    ...graph,
    nodes: graph.nodes.map((node) => ({ ...node, selected: false })),
  };
}

/**
 * Duplicate nodes
 */
export function duplicateNodes(
  graph: BlueprintGraph,
  nodeIds: string[],
  offset: { x: number; y: number } = { x: 100, y: 100 }
): BlueprintGraph {
  const nodesToDuplicate = graph.nodes.filter((node) =>
    nodeIds.includes(node.id)
  );
  const nodeIdMap = new Map<string, string>();

  // Create new nodes with new IDs
  const newNodes = nodesToDuplicate.map((node) => {
    const newId = generateNodeId();
    nodeIdMap.set(node.id, newId);
    return {
      ...node,
      id: newId,
      position: {
        x: node.position.x + offset.x,
        y: node.position.y + offset.y,
      },
      selected: false,
      inputs: node.inputs.map((pin) => ({
        ...pin,
        id: generatePinId(),
        connected: false,
      })),
      outputs: node.outputs.map((pin) => ({
        ...pin,
        id: generatePinId(),
        connected: false,
      })),
    };
  });

  // Create new connections for duplicated nodes
  const newConnections: BlueprintConnection[] = [];
  for (const conn of graph.connections) {
    const newSourceId = nodeIdMap.get(conn.sourceNodeId);
    const newTargetId = nodeIdMap.get(conn.targetNodeId);

    if (newSourceId && newTargetId) {
      // Find corresponding pins in new nodes
      const sourceNode = newNodes.find((n) => n.id === newSourceId);
      const targetNode = newNodes.find((n) => n.id === newTargetId);

      if (sourceNode && targetNode) {
        const sourcePin = sourceNode.outputs.find(
          (p) => p.name === conn.sourcePinId
        );
        const targetPin = targetNode.inputs.find(
          (p) => p.name === conn.targetPinId
        );

        if (sourcePin && targetPin) {
          newConnections.push({
            id: generateConnectionId(),
            sourceNodeId: newSourceId,
            sourcePinId: sourcePin.id,
            targetNodeId: newTargetId,
            targetPinId: targetPin.id,
          });
        }
      }
    }
  }

  return {
    ...graph,
    nodes: [...graph.nodes, ...newNodes],
    connections: [...graph.connections, ...newConnections],
    metadata: {
      ...graph.metadata,
      modifiedAt: new Date().toISOString(),
    },
  };
}
