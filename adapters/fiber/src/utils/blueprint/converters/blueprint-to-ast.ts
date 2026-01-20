/**
 * Blueprint to AST Converter
 * Converts Blueprint graph nodes back into AST nodes
 */

import type {
  ProgramNode,
  FunctionNode,
  StatementNode,
  ExpressionNode,
  VariableNode,
  ParameterNode,
  IfStatementNode,
  WhileStatementNode,
  TypeNode,
} from "~/utils/blueprint/ast/ast-core";
import type {
  BlueprintGraph,
  BlueprintNode,
  BlueprintConnection,
} from "~/utils/blueprint/graph/node-types";
import { generateASTNodeId } from "~/utils/blueprint/ast/ast-core";

/**
 * Configuration for Blueprint to AST conversion
 */
export interface BlueprintToASTConfig {
  /** Preserve node positions in AST metadata */
  preservePositions?: boolean;
  /** Generate source positions from node positions */
  generatePositions?: boolean;
}

const DEFAULT_CONFIG: Required<BlueprintToASTConfig> = {
  preservePositions: true,
  generatePositions: true,
};

/**
 * Convert a Blueprint Graph to an AST Program
 */
export function convertBlueprintToAST(
  graph: BlueprintGraph,
  config: BlueprintToASTConfig = {}
): ProgramNode {
  const cfg = { ...DEFAULT_CONFIG, ...config };
  const functions: FunctionNode[] = [];
  const variables: VariableNode[] = [];

  // Build a map of connections for quick lookup
  const connectionMap = buildConnectionMap(graph.connections);

  // Group nodes by function/callback
  const functionNodes = graph.nodes.filter(
    (node) => node.type === "function" || node.type === "callback"
  );

  // Convert each function node
  for (const funcNode of functionNodes) {
    const func = convertFunctionNodeToAST(funcNode, graph, connectionMap, cfg);
    if (func) {
      functions.push(func);
    }
  }

  // Convert variable nodes (top-level variables)
  const variableNodes = graph.nodes.filter((node) => node.type === "variable");
  for (const varNode of variableNodes) {
    const variable = convertVariableNodeToAST(
      varNode,
      graph,
      connectionMap,
      cfg
    );
    if (variable) {
      variables.push(variable);
    }
  }

  return {
    id: generateASTNodeId("program"),
    type: "program",
    language: graph.language,
    functions,
    variables,
    position: { line: 0, column: 0 },
    metadata: {
      blueprintId: graph.id,
      blueprintName: graph.name,
    },
  };
}

/**
 * Build a connection map for quick lookup
 */
function buildConnectionMap(
  connections: BlueprintConnection[]
): Map<string, Map<string, BlueprintConnection[]>> {
  const map = new Map<string, Map<string, BlueprintConnection[]>>();

  for (const conn of connections) {
    // Map by target node and pin
    let nodeMap = map.get(conn.targetNodeId);
    if (!nodeMap) {
      nodeMap = new Map();
      map.set(conn.targetNodeId, nodeMap);
    }
    if (!nodeMap.has(conn.targetPinId)) {
      nodeMap.set(conn.targetPinId, []);
    }
    nodeMap.get(conn.targetPinId)?.push(conn);
  }

  return map;
}

/**
 * Convert a function Blueprint node to an AST Function node
 */
function convertFunctionNodeToAST(
  funcNode: BlueprintNode,
  graph: BlueprintGraph,
  connectionMap: Map<string, Map<string, BlueprintConnection[]>>,
  config: Required<BlueprintToASTConfig>
): FunctionNode | null {
  if (funcNode.type !== "function" && funcNode.type !== "callback") {
    return null;
  }

  // Extract parameters from inputs (skip Exec pin)
  const parameters: ParameterNode[] = [];
  for (const input of funcNode.inputs) {
    if (input.type !== "exec") {
      parameters.push({
        id: generateASTNodeId("param"),
        type: "parameter",
        name: input.name,
        parameterType: {
          id: generateASTNodeId("type"),
          type: "type",
          name: input.type as string,
          position: getPositionFromNode(funcNode, config),
        },
        defaultValue: input.defaultValue
          ? convertValueToExpression(input.defaultValue, input.type as string)
          : undefined,
        position: getPositionFromNode(funcNode, config),
      });
    }
  }

  // Extract return type from outputs (skip Exec pin)
  const returnOutput = funcNode.outputs.find((p) => p.type !== "exec");
  const returnType: TypeNode = {
    id: generateASTNodeId("type"),
    type: "type",
    name: (returnOutput?.type as string) || "void",
    position: getPositionFromNode(funcNode, config),
  };

  // Find entry point (Exec input)
  const execInput = funcNode.inputs.find((p) => p.type === "exec");
  if (!execInput) {
    return null;
  }

  // Find connected nodes starting from Exec output
  const execOutput = funcNode.outputs.find((p) => p.type === "exec");
  const body: StatementNode[] = [];

  if (execOutput) {
    // Find nodes connected to Exec output
    const execConnections =
      connectionMap.get(funcNode.id)?.get(execOutput.id) || [];
    const visitedNodes = new Set<string>();

    // Traverse execution flow
    for (const conn of execConnections) {
      const targetNode = graph.nodes.find((n) => n.id === conn.targetNodeId);
      if (targetNode && !visitedNodes.has(targetNode.id)) {
        visitedNodes.add(targetNode.id);
        const statements = convertNodeToStatements(
          targetNode,
          graph,
          connectionMap,
          visitedNodes,
          config
        );
        body.push(...statements);
      }
    }
  }

  const isCallback =
    funcNode.type === "callback" || funcNode.properties.callbackName;

  return {
    id: funcNode.properties.astNodeId || generateASTNodeId("func"),
    type: "function",
    name: funcNode.title,
    parameters,
    returnType,
    body: body.length > 0 ? body : [],
    isPublic: isCallback || funcNode.properties.isPublic,
    isStatic: funcNode.properties.isStatic,
    modifiers: funcNode.properties.modifiers || [],
    position: getPositionFromNode(funcNode, config),
    metadata: {
      blueprintNodeId: funcNode.id,
      isCallback,
    },
  };
}

/**
 * Convert a Blueprint node to AST statements
 */
function convertNodeToStatements(
  node: BlueprintNode,
  graph: BlueprintGraph,
  connectionMap: Map<string, Map<string, BlueprintConnection[]>>,
  visitedNodes: Set<string>,
  config: Required<BlueprintToASTConfig>
): StatementNode[] {
  const statements: StatementNode[] = [];

  switch (node.type) {
    case "if": {
      const conditionPin = node.inputs.find((p) => p.name === "Condition");
      const thenPin = node.outputs.find((p) => p.name === "Then");
      const elsePin = node.outputs.find((p) => p.name === "Else");

      let condition: ExpressionNode | undefined;
      if (conditionPin) {
        const conditionConn = connectionMap
          .get(node.id)
          ?.get(conditionPin.id)?.[0];
        if (conditionConn) {
          const sourceNode = graph.nodes.find(
            (n) => n.id === conditionConn.sourceNodeId
          );
          if (sourceNode) {
            const expr = convertNodeToExpression(
              sourceNode,
              graph,
              connectionMap,
              config
            );
            condition = expr || undefined;
          }
        }
      }

      const thenStatements: StatementNode[] = [];
      const elseStatements: StatementNode[] = [];

      if (thenPin) {
        const thenConns = connectionMap.get(node.id)?.get(thenPin.id) || [];
        for (const conn of thenConns) {
          const targetNode = graph.nodes.find(
            (n) => n.id === conn.targetNodeId
          );
          if (targetNode && !visitedNodes.has(targetNode.id)) {
            visitedNodes.add(targetNode.id);
            thenStatements.push(
              ...convertNodeToStatements(
                targetNode,
                graph,
                connectionMap,
                visitedNodes,
                config
              )
            );
          }
        }
      }

      if (elsePin) {
        const elseConns = connectionMap.get(node.id)?.get(elsePin.id) || [];
        for (const conn of elseConns) {
          const targetNode = graph.nodes.find(
            (n) => n.id === conn.targetNodeId
          );
          if (targetNode && !visitedNodes.has(targetNode.id)) {
            visitedNodes.add(targetNode.id);
            elseStatements.push(
              ...convertNodeToStatements(
                targetNode,
                graph,
                connectionMap,
                visitedNodes,
                config
              )
            );
          }
        }
      }

      if (condition) {
        const ifStmt: IfStatementNode = {
          id: node.properties.astNodeId || generateASTNodeId("if"),
          type: "statement",
          kind: "if",
          condition,
          // biome-ignore lint/suspicious/noThenProperty: Required by AST type definition
          then:
            thenStatements.length === 1 ? thenStatements[0] : thenStatements,
          else:
            elseStatements.length > 0
              ? elseStatements.length === 1
                ? elseStatements[0]
                : elseStatements
              : undefined,
          position: getPositionFromNode(node, config),
          metadata: { blueprintNodeId: node.id },
        };
        statements.push(ifStmt as unknown as StatementNode);
      }
      break;
    }

    case "while": {
      const conditionPin = node.inputs.find((p) => p.name === "Condition");
      const loopPin = node.outputs.find((p) => p.name === "Loop");

      let condition: ExpressionNode | undefined;
      if (conditionPin) {
        const conditionConn = connectionMap
          .get(node.id)
          ?.get(conditionPin.id)?.[0];
        if (conditionConn) {
          const sourceNode = graph.nodes.find(
            (n) => n.id === conditionConn.sourceNodeId
          );
          if (sourceNode) {
            const expr = convertNodeToExpression(
              sourceNode,
              graph,
              connectionMap,
              config
            );
            condition = expr || undefined;
          }
        }
      }

      const bodyStatements: StatementNode[] = [];
      if (loopPin) {
        const loopConns = connectionMap.get(node.id)?.get(loopPin.id) || [];
        for (const conn of loopConns) {
          const targetNode = graph.nodes.find(
            (n) => n.id === conn.targetNodeId
          );
          if (targetNode && !visitedNodes.has(targetNode.id)) {
            visitedNodes.add(targetNode.id);
            bodyStatements.push(
              ...convertNodeToStatements(
                targetNode,
                graph,
                connectionMap,
                visitedNodes,
                config
              )
            );
          }
        }
      }

      if (condition) {
        const whileStmt: WhileStatementNode = {
          id: node.properties.astNodeId || generateASTNodeId("while"),
          type: "statement",
          kind: "while",
          condition,
          body:
            bodyStatements.length === 1 ? bodyStatements[0] : bodyStatements,
          position: getPositionFromNode(node, config),
          metadata: { blueprintNodeId: node.id },
        };
        statements.push(whileStmt as unknown as StatementNode);
      }
      break;
    }

    case "for": {
      const startPin = node.inputs.find((p) => p.name === "Start");
      const endPin = node.inputs.find((p) => p.name === "End");
      const loopPin = node.outputs.find((p) => p.name === "Loop");

      let init: ExpressionNode | undefined;
      let condition: ExpressionNode | undefined;

      if (startPin) {
        const startConn = connectionMap.get(node.id)?.get(startPin.id)?.[0];
        if (startConn) {
          const sourceNode = graph.nodes.find(
            (n) => n.id === startConn.sourceNodeId
          );
          if (sourceNode) {
            const expr = convertNodeToExpression(
              sourceNode,
              graph,
              connectionMap,
              config
            );
            init = expr || undefined;
          }
        }
      }

      if (endPin) {
        const endConn = connectionMap.get(node.id)?.get(endPin.id)?.[0];
        if (endConn) {
          const sourceNode = graph.nodes.find(
            (n) => n.id === endConn.sourceNodeId
          );
          if (sourceNode) {
            const expr = convertNodeToExpression(
              sourceNode,
              graph,
              connectionMap,
              config
            );
            condition = expr || undefined;
          }
        }
      }

      const bodyStatements: StatementNode[] = [];
      if (loopPin) {
        const loopConns = connectionMap.get(node.id)?.get(loopPin.id) || [];
        for (const conn of loopConns) {
          const targetNode = graph.nodes.find(
            (n) => n.id === conn.targetNodeId
          );
          if (targetNode && !visitedNodes.has(targetNode.id)) {
            visitedNodes.add(targetNode.id);
            bodyStatements.push(
              ...convertNodeToStatements(
                targetNode,
                graph,
                connectionMap,
                visitedNodes,
                config
              )
            );
          }
        }
      }

      const forStmt: StatementNode = {
        id: node.properties.astNodeId || generateASTNodeId("for"),
        type: "statement",
        kind: "for",
        statement: {
          init: init || undefined,
          condition: condition || undefined,
          update: undefined, // For loop blueprint nodes don't have an Update pin
          body:
            bodyStatements.length === 1 ? bodyStatements[0] : bodyStatements,
        },
        position: getPositionFromNode(node, config),
        metadata: { blueprintNodeId: node.id },
      };
      statements.push(forStmt);
      break;
    }

    case "return": {
      const valuePin = node.inputs.find((p) => p.name === "Value");
      let value: ExpressionNode | undefined;

      if (valuePin) {
        const valueConn = connectionMap.get(node.id)?.get(valuePin.id)?.[0];
        if (valueConn) {
          const sourceNode = graph.nodes.find(
            (n) => n.id === valueConn.sourceNodeId
          );
          if (sourceNode) {
            const expr = convertNodeToExpression(
              sourceNode,
              graph,
              connectionMap,
              config
            );
            value = expr || undefined;
          }
        }
      }

      const returnStmt: StatementNode = {
        id: node.properties.astNodeId || generateASTNodeId("return"),
        type: "statement",
        kind: "return",
        statement: {
          value,
        },
        position: getPositionFromNode(node, config),
        metadata: { blueprintNodeId: node.id },
      };
      statements.push(returnStmt);
      break;
    }

    case "assignment": {
      const variablePin = node.inputs.find((p) => p.name === "Variable");
      const valuePin = node.inputs.find((p) => p.name === "Value");

      let target: ExpressionNode | null = null;
      let value: ExpressionNode | null = null;

      if (variablePin) {
        const varConn = connectionMap.get(node.id)?.get(variablePin.id)?.[0];
        if (varConn) {
          const sourceNode = graph.nodes.find(
            (n) => n.id === varConn.sourceNodeId
          );
          if (sourceNode) {
            target = convertNodeToExpression(
              sourceNode,
              graph,
              connectionMap,
              config
            );
          }
        }
      }

      if (valuePin) {
        const valueConn = connectionMap.get(node.id)?.get(valuePin.id)?.[0];
        if (valueConn) {
          const sourceNode = graph.nodes.find(
            (n) => n.id === valueConn.sourceNodeId
          );
          if (sourceNode) {
            value = convertNodeToExpression(
              sourceNode,
              graph,
              connectionMap,
              config
            );
          }
        }
      }

      if (target && value) {
        const assignStmt: StatementNode = {
          id: node.properties.astNodeId || generateASTNodeId("assign"),
          type: "statement",
          kind: "assignment",
          statement: {
            target,
            operator: node.properties.operator || "=",
            value,
          },
          position: getPositionFromNode(node, config),
          metadata: { blueprintNodeId: node.id },
        };
        statements.push(assignStmt);
      }
      break;
    }

    case "call":
    case "native": {
      // Function calls are expressions, but we can treat them as expression statements
      const expr = convertNodeToExpression(node, graph, connectionMap, config);
      if (expr) {
        const exprStmt: StatementNode = {
          id: node.properties.astNodeId || generateASTNodeId("stmt"),
          type: "statement",
          kind: "expression",
          statement: expr,
          position: getPositionFromNode(node, config),
          metadata: { blueprintNodeId: node.id },
        };
        statements.push(exprStmt);
      }
      break;
    }

    default:
      // Unknown node type - skip or create placeholder
      break;
  }

  return statements;
}

/**
 * Convert a Blueprint node to an AST Expression
 */
function convertNodeToExpression(
  node: BlueprintNode,
  graph: BlueprintGraph,
  connectionMap: Map<string, Map<string, BlueprintConnection[]>>,
  config: Required<BlueprintToASTConfig>
): ExpressionNode | null {
  switch (node.type) {
    case "literal": {
      const value = node.properties.literalValue ?? node.title;
      const literalType = node.properties.literalType || "int";
      const literal: ExpressionNode = {
        id: node.properties.astNodeId || generateASTNodeId("literal"),
        type: "expression",
        kind: "literal",
        expression: {
          value,
          type: literalType,
        },
        position: getPositionFromNode(node, config),
        metadata: { blueprintNodeId: node.id },
      };
      return literal;
    }

    case "binary": {
      const operator = node.properties.operator || "+";
      const aPin = node.inputs.find((p) => p.name === "A");
      const bPin = node.inputs.find((p) => p.name === "B");

      let left: ExpressionNode | null = null;
      let right: ExpressionNode | null = null;

      if (aPin) {
        const aConn = connectionMap.get(node.id)?.get(aPin.id)?.[0];
        if (aConn) {
          const sourceNode = graph.nodes.find(
            (n) => n.id === aConn.sourceNodeId
          );
          if (sourceNode) {
            left = convertNodeToExpression(
              sourceNode,
              graph,
              connectionMap,
              config
            );
          }
        }
      }

      if (bPin) {
        const bConn = connectionMap.get(node.id)?.get(bPin.id)?.[0];
        if (bConn) {
          const sourceNode = graph.nodes.find(
            (n) => n.id === bConn.sourceNodeId
          );
          if (sourceNode) {
            right = convertNodeToExpression(
              sourceNode,
              graph,
              connectionMap,
              config
            );
          }
        }
      }

      if (left && right) {
        const binary: ExpressionNode = {
          id: node.properties.astNodeId || generateASTNodeId("binary"),
          type: "expression",
          kind: "binary",
          expression: {
            operator,
            left,
            right,
          },
          position: getPositionFromNode(node, config),
          metadata: { blueprintNodeId: node.id },
        };
        return binary;
      }
      break;
    }

    case "call":
    case "native": {
      const functionName =
        node.properties.functionName ||
        node.properties.nativeName ||
        node.title;
      const args: ExpressionNode[] = [];

      // Extract arguments from inputs (skip Exec pin)
      for (const input of node.inputs) {
        if (input.type !== "exec") {
          const argConn = connectionMap.get(node.id)?.get(input.id)?.[0];
          if (argConn) {
            const sourceNode = graph.nodes.find(
              (n) => n.id === argConn.sourceNodeId
            );
            if (sourceNode) {
              const arg = convertNodeToExpression(
                sourceNode,
                graph,
                connectionMap,
                config
              );
              if (arg) {
                args.push(arg);
              }
            }
          }
        }
      }

      const call: ExpressionNode = {
        id: node.properties.astNodeId || generateASTNodeId("call"),
        type: "expression",
        kind: "call",
        expression: {
          callee: functionName,
          arguments: args,
        },
        position: getPositionFromNode(node, config),
        metadata: {
          blueprintNodeId: node.id,
          isNative: node.type === "native",
        },
      };
      return call;
    }

    case "variable": {
      const varName = node.properties.variableName || node.title;
      const varExpr: ExpressionNode = {
        id: node.properties.astNodeId || generateASTNodeId("var"),
        type: "expression",
        kind: "variable",
        expression: { name: varName },
        position: getPositionFromNode(node, config),
        metadata: { blueprintNodeId: node.id },
      };
      return varExpr;
    }

    case "unary": {
      const operator = node.properties.operator || "-";
      const valuePin = node.inputs.find((p) => p.name === "Value");

      let operand: ExpressionNode | null = null;
      if (valuePin) {
        const valueConn = connectionMap.get(node.id)?.get(valuePin.id)?.[0];
        if (valueConn) {
          const sourceNode = graph.nodes.find(
            (n) => n.id === valueConn.sourceNodeId
          );
          if (sourceNode) {
            operand = convertNodeToExpression(
              sourceNode,
              graph,
              connectionMap,
              config
            );
          }
        }
      }

      if (operand) {
        const unary: ExpressionNode = {
          id: node.properties.astNodeId || generateASTNodeId("unary"),
          type: "expression",
          kind: "unary",
          expression: {
            operator,
            operand,
          },
          position: getPositionFromNode(node, config),
          metadata: { blueprintNodeId: node.id },
        };
        return unary;
      }
      break;
    }

    default:
      return null;
  }

  return null;
}

/**
 * Convert a variable Blueprint node to an AST Variable node
 */
function convertVariableNodeToAST(
  varNode: BlueprintNode,
  graph: BlueprintGraph,
  connectionMap: Map<string, Map<string, BlueprintConnection[]>>,
  config: Required<BlueprintToASTConfig>
): VariableNode | null {
  if (varNode.type !== "variable" && varNode.type !== "constant") {
    return null;
  }

  const varName = varNode.properties.variableName || varNode.title;
  const varType = varNode.properties.variableType || "any";
  const isConstant =
    varNode.type === "constant" || varNode.properties.isConstant;

  // Check for initial value connection
  const initialValuePin = varNode.inputs.find(
    (p) => p.name === "Initial Value"
  );
  let initialValue: ExpressionNode | undefined;

  if (initialValuePin) {
    const initConn = connectionMap
      .get(varNode.id)
      ?.get(initialValuePin.id)?.[0];
    if (initConn) {
      const sourceNode = graph.nodes.find(
        (n) => n.id === initConn.sourceNodeId
      );
      if (sourceNode) {
        const expr = convertNodeToExpression(
          sourceNode,
          graph,
          connectionMap,
          config
        );
        initialValue = expr || undefined;
      }
    }
  }

  return {
    id: varNode.properties.astNodeId || generateASTNodeId("var"),
    type: "variable",
    name: varName,
    variableType: {
      id: generateASTNodeId("type"),
      type: "type",
      name: varType,
      position: getPositionFromNode(varNode, config),
    },
    isConstant,
    initialValue,
    position: getPositionFromNode(varNode, config),
    metadata: { blueprintNodeId: varNode.id },
  };
}

/**
 * Get position from node (either from metadata or generate from node position)
 */
function getPositionFromNode(
  node: BlueprintNode,
  config: Required<BlueprintToASTConfig>
): { line: number; column: number } {
  if (config.preservePositions && node.metadata?.position) {
    return node.metadata.position;
  }

  if (config.generatePositions) {
    // Generate approximate position from node coordinates
    // Assuming 50 pixels per line, 10 pixels per column
    return {
      line: Math.floor(node.position.y / 50),
      column: Math.floor(node.position.x / 10),
    };
  }

  return { line: 0, column: 0 };
}

/**
 * Convert a value to an expression (for default values)
 */
function convertValueToExpression(value: any, type: string): ExpressionNode {
  return {
    id: generateASTNodeId("expr"),
    type: "expression",
    kind: "literal",
    expression: { value, type },
    position: { line: 0, column: 0 },
  };
}
