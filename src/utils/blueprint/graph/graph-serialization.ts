/**
 * Blueprint graph serialization and deserialization.
 * Handles saving and loading Blueprint graphs to/from JSON.
 */

import type {
  BlueprintGraph,
  BlueprintNode,
  BlueprintConnection,
} from "./node-types";

/**
 * Serialize a Blueprint graph to JSON
 */
export function serializeGraph(graph: BlueprintGraph): string {
  return JSON.stringify(graph, null, 2);
}

/**
 * Deserialize a Blueprint graph from JSON
 */
export function deserializeGraph(json: string): BlueprintGraph {
  try {
    const parsed = JSON.parse(json);
    return validateAndNormalizeGraph(parsed);
  } catch (error) {
    throw new Error(`Failed to deserialize graph: ${error}`);
  }
}

/**
 * Validate and normalize a graph object
 */
function validateAndNormalizeGraph(obj: any): BlueprintGraph {
  // Ensure required fields exist
  if (!obj.id) {
    obj.id = `graph_${Date.now()}`;
  }
  if (!obj.name) {
    obj.name = "Untitled Blueprint";
  }
  if (!obj.language) {
    obj.language = "pawn";
  }
  if (!obj.nodes) {
    obj.nodes = [];
  }
  if (!obj.connections) {
    obj.connections = [];
  }
  if (!obj.metadata) {
    obj.metadata = {
      createdAt: new Date().toISOString(),
      modifiedAt: new Date().toISOString(),
    };
  }
  if (!obj.version) {
    obj.version = "1.0.0";
  }

  // Validate nodes
  obj.nodes = obj.nodes.map((node: any) => validateAndNormalizeNode(node));

  // Validate connections
  obj.connections = obj.connections.map((conn: any) =>
    validateAndNormalizeConnection(conn)
  );

  return obj as BlueprintGraph;
}

/**
 * Validate and normalize a node object
 */
function validateAndNormalizeNode(obj: any): BlueprintNode {
  if (!obj.id) {
    obj.id = `node_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }
  if (!obj.type) {
    obj.type = "expression";
  }
  if (!obj.title) {
    obj.title = "Untitled Node";
  }
  if (!obj.position) {
    obj.position = { x: 0, y: 0 };
  }
  if (!obj.inputs) {
    obj.inputs = [];
  }
  if (!obj.outputs) {
    obj.outputs = [];
  }
  if (!obj.properties) {
    obj.properties = {};
  }
  if (obj.selected === undefined) {
    obj.selected = false;
  }
  if (obj.collapsed === undefined) {
    obj.collapsed = false;
  }

  return obj as BlueprintNode;
}

/**
 * Validate and normalize a connection object
 */
function validateAndNormalizeConnection(obj: any): BlueprintConnection {
  if (!obj.id) {
    obj.id = `conn_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }
  if (
    !(
      obj.sourceNodeId &&
      obj.sourcePinId &&
      obj.targetNodeId &&
      obj.targetPinId
    )
  ) {
    throw new Error("Invalid connection: missing required fields");
  }

  return obj as BlueprintConnection;
}

/**
 * Export graph to a file-friendly format
 */
export function exportGraph(graph: BlueprintGraph): {
  format: string;
  version: string;
  data: BlueprintGraph;
} {
  return {
    format: "rengine-blueprint",
    version: "1.0.0",
    data: graph,
  };
}

/**
 * Import graph from a file-friendly format
 */
export function importGraph(data: {
  format: string;
  version: string;
  data: any;
}): BlueprintGraph {
  if (data.format !== "rengine-blueprint") {
    throw new Error(`Unsupported format: ${data.format}`);
  }

  return validateAndNormalizeGraph(data.data);
}

/**
 * Create a graph snapshot for undo/redo
 */
export function createGraphSnapshot(graph: BlueprintGraph): BlueprintGraph {
  // Deep clone the graph
  return JSON.parse(JSON.stringify(graph));
}

/**
 * Compare two graphs for equality
 */
export function graphsEqual(
  graph1: BlueprintGraph,
  graph2: BlueprintGraph
): boolean {
  return serializeGraph(graph1) === serializeGraph(graph2);
}
