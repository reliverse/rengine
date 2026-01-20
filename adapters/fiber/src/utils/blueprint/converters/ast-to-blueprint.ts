/**
 * AST to Blueprint Converter
 * Converts AST nodes into Blueprint graph nodes
 */

import type {
  ProgramNode,
  FunctionNode,
  StatementNode,
  ExpressionNode,
  VariableNode,
  IfStatementNode,
  WhileStatementNode,
  ForStatementNode,
  ReturnStatementNode,
  BlockStatementNode,
  AssignmentStatementNode,
  LiteralNode,
  BinaryExpressionNode,
  CallExpressionNode,
  UnaryExpressionNode,
} from "~/utils/blueprint/ast/ast-core";
import type {
  BlueprintGraph,
  BlueprintNode,
  BlueprintConnection,
} from "~/utils/blueprint/graph/node-types";
import {
  createBlueprintNode,
  createExecPin,
  createDataPin,
  generateConnectionId,
} from "~/utils/blueprint/graph/node-types";
import { createBlueprintGraph } from "~/utils/blueprint/graph/blueprint-graph";

/**
 * Configuration for AST to Blueprint conversion
 */
export interface ASTToBlueprintConfig {
  /** Starting X position for nodes */
  startX?: number;
  /** Starting Y position for nodes */
  startY?: number;
  /** Horizontal spacing between nodes */
  nodeSpacingX?: number;
  /** Vertical spacing between nodes */
  nodeSpacingY?: number;
  /** Preserve original AST node positions if available */
  preservePositions?: boolean;
}

const DEFAULT_CONFIG: Required<ASTToBlueprintConfig> = {
  startX: 100,
  startY: 100,
  nodeSpacingX: 300,
  nodeSpacingY: 150,
  preservePositions: false,
};

/**
 * Convert an AST Program to a Blueprint Graph
 */
export function convertASTToBlueprint(
  ast: ProgramNode,
  config: ASTToBlueprintConfig = {}
): BlueprintGraph {
  const cfg = { ...DEFAULT_CONFIG, ...config };
  const nodes: BlueprintNode[] = [];
  const connections: BlueprintConnection[] = [];
  const currentX = cfg.startX;
  let currentY = cfg.startY;

  // Convert functions
  for (const func of ast.functions) {
    const funcNode = convertFunctionToBlueprint(
      func,
      { x: currentX, y: currentY },
      cfg
    );
    nodes.push(funcNode);
    currentY += cfg.nodeSpacingY * 2; // Functions take more space
  }

  // Convert top-level variables
  for (const variable of ast.variables) {
    const varNode = convertVariableToBlueprint(
      variable,
      { x: currentX, y: currentY },
      cfg
    );
    nodes.push(varNode);
    currentY += cfg.nodeSpacingY;
  }

  const graph = createBlueprintGraph(
    `blueprint_${Date.now()}`,
    ast.language as any
  );
  return {
    ...graph,
    nodes,
    connections,
  };
}

/**
 * Convert a Function AST node to a Blueprint node
 */
function convertFunctionToBlueprint(
  func: FunctionNode,
  position: { x: number; y: number },
  _config: Required<ASTToBlueprintConfig>
): BlueprintNode {
  const inputs = [createExecPin("input", "Exec")];
  const outputs = [createExecPin("output", "Exec")];

  // Add parameter inputs
  for (const param of func.parameters) {
    inputs.push(
      createDataPin(
        "input",
        param.name,
        param.parameterType.name as any,
        !param.defaultValue
      )
    );
  }

  // Add return value output if function has a return type
  if (func.returnType.name !== "void") {
    outputs.push(
      createDataPin("output", "Return", func.returnType.name as any)
    );
  }

  return createBlueprintNode(
    func.name.startsWith("On") ? "callback" : "function",
    func.name,
    position,
    inputs,
    outputs,
    {
      functionName: func.name,
      returnType: func.returnType.name,
      isPublic: func.isPublic,
      isStatic: func.isStatic,
      modifiers: func.modifiers,
      astNodeId: func.id,
    }
  );
}

/**
 * Convert a Statement AST node to Blueprint nodes
 */
export function convertStatementToBlueprint(
  statement: StatementNode,
  position: { x: number; y: number },
  config: Required<ASTToBlueprintConfig> = DEFAULT_CONFIG
): {
  nodes: BlueprintNode[];
  connections: BlueprintConnection[];
  nextPosition: { x: number; y: number };
} {
  const nodes: BlueprintNode[] = [];
  const connections: BlueprintConnection[] = [];
  const currentPos = { ...position };

  switch (statement.kind) {
    case "if": {
      const ifStmt = statement as unknown as IfStatementNode;
      const ifNode = convertIfStatementToBlueprint(ifStmt, currentPos, config);
      nodes.push(ifNode.node);
      nodes.push(...ifNode.childNodes);
      connections.push(...ifNode.connections);
      currentPos.y += config.nodeSpacingY * 2;
      break;
    }

    case "while": {
      const whileStmt = statement as unknown as WhileStatementNode;
      const whileNode = convertWhileStatementToBlueprint(
        whileStmt,
        currentPos,
        config
      );
      nodes.push(whileNode.node);
      nodes.push(...whileNode.childNodes);
      connections.push(...whileNode.connections);
      currentPos.y += config.nodeSpacingY * 2;
      break;
    }

    case "for": {
      const forStmt = statement as unknown as ForStatementNode;
      const forNode = convertForStatementToBlueprint(
        forStmt,
        currentPos,
        config
      );
      nodes.push(forNode.node);
      nodes.push(...forNode.childNodes);
      connections.push(...forNode.connections);
      currentPos.y += config.nodeSpacingY * 2;
      break;
    }

    case "return": {
      const returnStmt = statement as unknown as ReturnStatementNode;
      const returnNode = convertReturnStatementToBlueprint(
        returnStmt,
        currentPos,
        config
      );
      nodes.push(returnNode.node);
      nodes.push(...returnNode.childNodes);
      connections.push(...returnNode.connections);
      currentPos.y += config.nodeSpacingY;
      break;
    }

    case "assignment": {
      const assignStmt = statement as unknown as AssignmentStatementNode;
      const assignNode = convertAssignmentStatementToBlueprint(
        assignStmt,
        currentPos,
        config
      );
      nodes.push(assignNode.node);
      nodes.push(...assignNode.childNodes);
      connections.push(...assignNode.connections);
      currentPos.y += config.nodeSpacingY;
      break;
    }

    case "block": {
      const blockStmt = statement as unknown as BlockStatementNode;
      const blockResult = convertBlockStatementToBlueprint(
        blockStmt,
        currentPos,
        config
      );
      nodes.push(...blockResult.nodes);
      connections.push(...blockResult.connections);
      currentPos.y += config.nodeSpacingY * blockStmt.statements.length;
      break;
    }

    case "expression": {
      const exprStmt = statement as StatementNode & {
        statement: ExpressionNode;
      };
      const exprResult = convertExpressionToBlueprint(
        exprStmt.statement,
        currentPos,
        config
      );
      nodes.push(...exprResult.nodes);
      connections.push(...exprResult.connections);
      currentPos.y += config.nodeSpacingY;
      break;
    }

    default:
      // Unknown statement type - create a placeholder
      nodes.push(
        createBlueprintNode(
          "expression",
          `Statement: ${statement.kind}`,
          currentPos,
          [],
          [],
          { astNodeId: statement.id }
        )
      );
      currentPos.y += config.nodeSpacingY;
  }

  return { nodes, connections, nextPosition: currentPos };
}

/**
 * Convert an If statement to Blueprint nodes
 */
function convertIfStatementToBlueprint(
  ifStmt: IfStatementNode,
  position: { x: number; y: number },
  config: Required<ASTToBlueprintConfig>
): {
  node: BlueprintNode;
  childNodes: BlueprintNode[];
  connections: BlueprintConnection[];
} {
  const childNodes: BlueprintNode[] = [];
  const connections: BlueprintConnection[] = [];

  // Convert condition expression
  const conditionResult = convertExpressionToBlueprint(
    ifStmt.condition,
    { x: position.x - config.nodeSpacingX, y: position.y },
    config
  );
  childNodes.push(...conditionResult.nodes);
  connections.push(...conditionResult.connections);

  // Create if node
  const ifNode = createBlueprintNode(
    "if",
    "If",
    position,
    [
      createExecPin("input", "Exec"),
      createDataPin("input", "Condition", "bool", true),
    ],
    [createExecPin("output", "Then"), createExecPin("output", "Else")],
    { astNodeId: ifStmt.id }
  );

  // Connect condition to if node
  if (conditionResult.nodes.length > 0) {
    const conditionNode = conditionResult.nodes.at(-1);
    if (conditionNode) {
      const conditionOutputPin = conditionNode.outputs.find(
        (p) => p.type === "bool"
      );
      if (conditionOutputPin) {
        connections.push({
          id: generateConnectionId(),
          sourceNodeId: conditionNode.id,
          sourcePinId: conditionOutputPin.id,
          targetNodeId: ifNode.id,
          targetPinId: ifNode.inputs[1].id, // Condition input
        });
      }
    }
  }

  return { node: ifNode, childNodes, connections };
}

/**
 * Convert a While statement to Blueprint nodes
 */
function convertWhileStatementToBlueprint(
  whileStmt: WhileStatementNode,
  position: { x: number; y: number },
  config: Required<ASTToBlueprintConfig>
): {
  node: BlueprintNode;
  childNodes: BlueprintNode[];
  connections: BlueprintConnection[];
} {
  const childNodes: BlueprintNode[] = [];
  const connections: BlueprintConnection[] = [];

  // Convert condition expression
  const conditionResult = convertExpressionToBlueprint(
    whileStmt.condition,
    { x: position.x - config.nodeSpacingX, y: position.y },
    config
  );
  childNodes.push(...conditionResult.nodes);
  connections.push(...conditionResult.connections);

  // Create while node
  const whileNode = createBlueprintNode(
    "while",
    "While Loop",
    position,
    [
      createExecPin("input", "Exec"),
      createDataPin("input", "Condition", "bool", true),
    ],
    [createExecPin("output", "Loop"), createExecPin("output", "Completed")],
    { astNodeId: whileStmt.id }
  );

  // Connect condition to while node
  if (conditionResult.nodes.length > 0) {
    const conditionNode = conditionResult.nodes.at(-1);
    if (conditionNode) {
      const conditionOutputPin = conditionNode.outputs.find(
        (p) => p.type === "bool"
      );
      if (conditionOutputPin) {
        connections.push({
          id: generateConnectionId(),
          sourceNodeId: conditionNode.id,
          sourcePinId: conditionOutputPin.id,
          targetNodeId: whileNode.id,
          targetPinId: whileNode.inputs[1].id, // Condition input
        });
      }
    }
  }

  return { node: whileNode, childNodes, connections };
}

/**
 * Convert a For statement to Blueprint nodes
 */
function convertForStatementToBlueprint(
  forStmt: ForStatementNode,
  position: { x: number; y: number },
  _config: Required<ASTToBlueprintConfig>
): {
  node: BlueprintNode;
  childNodes: BlueprintNode[];
  connections: BlueprintConnection[];
} {
  const childNodes: BlueprintNode[] = [];
  const connections: BlueprintConnection[] = [];

  // Create for node
  const forNode = createBlueprintNode(
    "for",
    "For Loop",
    position,
    [
      createExecPin("input", "Exec"),
      createDataPin("input", "Start", "int", true),
      createDataPin("input", "End", "int", true),
    ],
    [
      createExecPin("output", "Loop"),
      createExecPin("output", "Completed"),
      createDataPin("output", "Index", "int"),
    ],
    { astNodeId: forStmt.id }
  );

  return { node: forNode, childNodes, connections };
}

/**
 * Convert a Return statement to Blueprint nodes
 */
function convertReturnStatementToBlueprint(
  returnStmt: ReturnStatementNode,
  position: { x: number; y: number },
  config: Required<ASTToBlueprintConfig>
): {
  node: BlueprintNode;
  childNodes: BlueprintNode[];
  connections: BlueprintConnection[];
} {
  const childNodes: BlueprintNode[] = [];
  const connections: BlueprintConnection[] = [];

  const inputs = [createExecPin("input", "Exec")];
  if (returnStmt.value) {
    inputs.push(createDataPin("input", "Value", "any", false));
  }

  const returnNode = createBlueprintNode(
    "return",
    "Return",
    position,
    inputs,
    [],
    { astNodeId: returnStmt.id }
  );

  // Convert return value expression if present
  if (returnStmt.value) {
    const valueResult = convertExpressionToBlueprint(
      returnStmt.value,
      { x: position.x - config.nodeSpacingX, y: position.y },
      config
    );
    childNodes.push(...valueResult.nodes);
    connections.push(...valueResult.connections);

    // Connect value to return node
    if (valueResult.nodes.length > 0) {
      const valueNode = valueResult.nodes.at(-1);
      if (valueNode) {
        const valueOutputPin = valueNode.outputs[0];
        if (valueOutputPin) {
          connections.push({
            id: generateConnectionId(),
            sourceNodeId: valueNode.id,
            sourcePinId: valueOutputPin.id,
            targetNodeId: returnNode.id,
            targetPinId: returnNode.inputs[1].id, // Value input
          });
        }
      }
    }
  }

  return { node: returnNode, childNodes, connections };
}

/**
 * Convert an Assignment statement to Blueprint nodes
 */
function convertAssignmentStatementToBlueprint(
  assignStmt: AssignmentStatementNode,
  position: { x: number; y: number },
  config: Required<ASTToBlueprintConfig>
): {
  node: BlueprintNode;
  childNodes: BlueprintNode[];
  connections: BlueprintConnection[];
} {
  const childNodes: BlueprintNode[] = [];
  const connections: BlueprintConnection[] = [];

  // Convert value expression
  const valueResult = convertExpressionToBlueprint(
    assignStmt.value,
    { x: position.x - config.nodeSpacingX, y: position.y },
    config
  );
  childNodes.push(...valueResult.nodes);
  connections.push(...valueResult.connections);

  // Create assignment node
  const assignNode = createBlueprintNode(
    "assignment",
    "Assignment",
    position,
    [
      createExecPin("input", "Exec"),
      createDataPin("input", "Variable", "any", true),
      createDataPin("input", "Value", "any", true),
    ],
    [createExecPin("output", "Exec")],
    {
      operator: assignStmt.operator || "=",
      astNodeId: assignStmt.id,
    }
  );

  // Connect value to assignment node
  if (valueResult.nodes.length > 0) {
    const valueNode = valueResult.nodes.at(-1);
    if (valueNode) {
      const valueOutputPin = valueNode.outputs[0];
      if (valueOutputPin) {
        connections.push({
          id: generateConnectionId(),
          sourceNodeId: valueNode.id,
          sourcePinId: valueOutputPin.id,
          targetNodeId: assignNode.id,
          targetPinId: assignNode.inputs[2].id, // Value input
        });
      }
    }
  }

  return { node: assignNode, childNodes, connections };
}

/**
 * Convert a Block statement to Blueprint nodes
 */
function convertBlockStatementToBlueprint(
  blockStmt: BlockStatementNode,
  position: { x: number; y: number },
  config: Required<ASTToBlueprintConfig>
): { nodes: BlueprintNode[]; connections: BlueprintConnection[] } {
  const nodes: BlueprintNode[] = [];
  const connections: BlueprintConnection[] = [];
  let currentPos = { ...position };

  // Convert each statement in the block
  for (let i = 0; i < blockStmt.statements.length; i++) {
    const stmt = blockStmt.statements[i];
    const result = convertStatementToBlueprint(stmt, currentPos, config);
    nodes.push(...result.nodes);
    connections.push(...result.connections);

    // Connect statements in sequence
    if (i > 0 && result.nodes.length > 0 && nodes.length > 1) {
      const prevNode = nodes.at(result.nodes.length + 1);
      if (prevNode) {
        const currNode = result.nodes[0];
        const prevExecOut = prevNode.outputs.find((p) => p.type === "exec");
        const currExecIn = currNode.inputs.find((p) => p.type === "exec");
        if (prevExecOut && currExecIn) {
          connections.push({
            id: generateConnectionId(),
            sourceNodeId: prevNode.id,
            sourcePinId: prevExecOut.id,
            targetNodeId: currNode.id,
            targetPinId: currExecIn.id,
          });
        }
      }
    }

    currentPos = result.nextPosition;
  }

  return { nodes, connections };
}

/**
 * Convert an Expression AST node to Blueprint nodes
 */
export function convertExpressionToBlueprint(
  expr: ExpressionNode,
  position: { x: number; y: number },
  config: Required<ASTToBlueprintConfig> = DEFAULT_CONFIG
): { nodes: BlueprintNode[]; connections: BlueprintConnection[] } {
  const nodes: BlueprintNode[] = [];
  const connections: BlueprintConnection[] = [];

  switch (expr.kind) {
    case "literal": {
      const literal = expr as unknown as LiteralNode;
      const literalNode = createBlueprintNode(
        "literal",
        String(literal.value),
        position,
        [],
        [createDataPin("output", "Value", literal.literalType)],
        { astNodeId: expr.id }
      );
      nodes.push(literalNode);
      break;
    }

    case "binary": {
      const binary = expr as unknown as BinaryExpressionNode;
      const leftResult = convertExpressionToBlueprint(
        binary.left,
        {
          x: position.x - config.nodeSpacingX,
          y: position.y - config.nodeSpacingY / 2,
        },
        config
      );
      const rightResult = convertExpressionToBlueprint(
        binary.right,
        {
          x: position.x - config.nodeSpacingX,
          y: position.y + config.nodeSpacingY / 2,
        },
        config
      );

      nodes.push(...leftResult.nodes);
      nodes.push(...rightResult.nodes);
      connections.push(...leftResult.connections);
      connections.push(...rightResult.connections);

      // Determine output type
      let outputType = "any";
      if (
        ["==", "!=", "<", ">", "<=", ">=", "&&", "||"].includes(
          (binary as any).operator
        )
      ) {
        outputType = "bool";
      }

      const binaryNode = createBlueprintNode(
        "binary",
        (binary as any).operator || "Binary Op",
        position,
        [
          createDataPin("input", "A", "any", true),
          createDataPin("input", "B", "any", true),
        ],
        [createDataPin("output", "Result", outputType as any)],
        {
          operator: (binary as any).operator,
          astNodeId: expr.id,
        }
      );
      nodes.push(binaryNode);

      // Connect left and right expressions
      if (leftResult.nodes.length > 0) {
        const leftNode = leftResult.nodes.at(-1);
        if (leftNode) {
          const leftOutput = leftNode.outputs[0];
          if (leftOutput) {
            connections.push({
              id: generateConnectionId(),
              sourceNodeId: leftNode.id,
              sourcePinId: leftOutput.id,
              targetNodeId: binaryNode.id,
              targetPinId: binaryNode.inputs[0].id,
            });
          }
        }
      }
      if (rightResult.nodes.length > 0) {
        const rightNode = rightResult.nodes.at(-1);
        if (rightNode) {
          const rightOutput = rightNode.outputs[0];
          if (rightOutput) {
            connections.push({
              id: generateConnectionId(),
              sourceNodeId: rightNode.id,
              sourcePinId: rightOutput.id,
              targetNodeId: binaryNode.id,
              targetPinId: binaryNode.inputs[1].id,
            });
          }
        }
      }
      break;
    }

    case "call": {
      const call = expr as unknown as CallExpressionNode;
      const argNodes: BlueprintNode[] = [];
      const argConnections: BlueprintConnection[] = [];

      // Convert arguments
      for (let i = 0; i < call.arguments.length; i++) {
        const arg = call.arguments[i];
        const argResult = convertExpressionToBlueprint(
          arg,
          {
            x: position.x - config.nodeSpacingX,
            y:
              position.y +
              (i - call.arguments.length / 2) * config.nodeSpacingY,
          },
          config
        );
        argNodes.push(...argResult.nodes);
        argConnections.push(...argResult.connections);
      }

      nodes.push(...argNodes);
      connections.push(...argConnections);

      // Create call node
      const callNode = createBlueprintNode(
        "call",
        typeof call.callee === "string" ? call.callee : "Call Function",
        position,
        [
          createExecPin("input", "Exec"),
          ...call.arguments.map((_, i) =>
            createDataPin("input", `Arg${i + 1}`, "any", true)
          ),
        ],
        [
          createExecPin("output", "Exec"),
          createDataPin("output", "Result", "any"),
        ],
        {
          functionName:
            typeof call.callee === "string" ? call.callee : undefined,
          astNodeId: expr.id,
        }
      );
      nodes.push(callNode);

      // Connect arguments
      for (let i = 0; i < call.arguments.length && i < argNodes.length; i++) {
        const argNode = argNodes[argNodes.length - call.arguments.length + i];
        const argOutput = argNode.outputs[0];
        if (argOutput) {
          connections.push({
            id: generateConnectionId(),
            sourceNodeId: argNode.id,
            sourcePinId: argOutput.id,
            targetNodeId: callNode.id,
            targetPinId: callNode.inputs[i + 1].id, // +1 for Exec input
          });
        }
      }
      break;
    }

    case "variable": {
      const varNode = createBlueprintNode(
        "variable",
        (expr as any).name || "Variable",
        position,
        [],
        [createDataPin("output", "Value", "any")],
        {
          variableName: (expr as any).name,
          astNodeId: expr.id,
        }
      );
      nodes.push(varNode);
      break;
    }

    case "unary": {
      const unary = expr as unknown as UnaryExpressionNode;
      const operandResult = convertExpressionToBlueprint(
        unary.operand,
        { x: position.x - config.nodeSpacingX, y: position.y },
        config
      );
      nodes.push(...operandResult.nodes);
      connections.push(...operandResult.connections);

      const unaryNode = createBlueprintNode(
        "unary",
        (unary as any).operator || "Unary Op",
        position,
        [createDataPin("input", "Value", "any", true)],
        [createDataPin("output", "Result", "any")],
        {
          operator: (unary as any).operator,
          astNodeId: expr.id,
        }
      );
      nodes.push(unaryNode);

      // Connect operand
      if (operandResult.nodes.length > 0) {
        const operandNode = operandResult.nodes.at(-1);
        if (operandNode) {
          const operandOutput = operandNode.outputs[0];
          if (operandOutput) {
            connections.push({
              id: generateConnectionId(),
              sourceNodeId: operandNode.id,
              sourcePinId: operandOutput.id,
              targetNodeId: unaryNode.id,
              targetPinId: unaryNode.inputs[0].id,
            });
          }
        }
      }
      break;
    }

    default: {
      // Unknown expression type - create placeholder
      nodes.push(
        createBlueprintNode(
          "expression",
          `Expression: ${expr.kind}`,
          position,
          [],
          [createDataPin("output", "Value", "any")],
          { astNodeId: expr.id }
        )
      );
    }
  }

  return { nodes, connections };
}

/**
 * Convert a Variable AST node to a Blueprint node
 */
function convertVariableToBlueprint(
  variable: VariableNode,
  position: { x: number; y: number },
  _config: Required<ASTToBlueprintConfig>
): BlueprintNode {
  const outputs = [
    createDataPin("output", "Value", variable.variableType.name as any),
  ];

  // If variable has initial value, add it as an input
  const inputs = variable.initialValue
    ? [
        createDataPin(
          "input",
          "Initial Value",
          variable.variableType.name as any,
          false
        ),
      ]
    : [];

  return createBlueprintNode(
    variable.isConstant ? "constant" : "variable",
    variable.name,
    position,
    inputs,
    outputs,
    {
      variableName: variable.name,
      variableType: variable.variableType.name,
      isConstant: variable.isConstant,
      astNodeId: variable.id,
    }
  );
}
