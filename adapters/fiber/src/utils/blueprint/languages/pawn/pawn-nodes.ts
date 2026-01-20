/**
 * Pawn-specific node type definitions and factories
 */

import type { BlueprintNode, Pin } from "~/utils/blueprint/graph/node-types";
import {
  createBlueprintNode,
  createExecPin,
  createDataPin,
} from "~/utils/blueprint/graph/node-types";
import type { SampCallback } from "./pawn-callbacks";
import type { SampNativeFunction } from "./pawn-functions";

/**
 * Create a SAMP callback node
 */
export function createSampCallbackNode(
  callback: SampCallback,
  position: { x: number; y: number }
): BlueprintNode {
  const inputs: Pin[] = [createExecPin("input", "Exec")];
  const outputs: Pin[] = [createExecPin("output", "Exec")];

  // Add parameter inputs
  for (const param of callback.parameters) {
    inputs.push(createDataPin("input", param.name, param.type as any, true));
  }

  // Add return value output if callback returns something
  if (callback.returnType !== "void") {
    outputs.push(createDataPin("output", "Return", callback.returnType as any));
  }

  return createBlueprintNode(
    "callback",
    callback.name,
    position,
    inputs,
    outputs,
    {
      callbackName: callback.name,
      description: callback.description,
      returnType: callback.returnType,
    }
  );
}

/**
 * Create a SAMP native function call node
 */
export function createSampNativeNode(
  native: SampNativeFunction,
  position: { x: number; y: number }
): BlueprintNode {
  const inputs: Pin[] = [createExecPin("input", "Exec")];
  const outputs: Pin[] = [createExecPin("output", "Exec")];

  // Add parameter inputs
  for (const param of native.parameters) {
    inputs.push(
      createDataPin("input", param.name, param.type as any, !param.optional)
    );
  }

  // Add return value output
  if (native.returnType !== "void") {
    outputs.push(createDataPin("output", "Result", native.returnType as any));
  }

  return createBlueprintNode("native", native.name, position, inputs, outputs, {
    nativeName: native.name,
    description: native.description,
    category: native.category,
    returnType: native.returnType,
  });
}

/**
 * Create a variable getter node
 */
export function createVariableGetterNode(
  variableName: string,
  variableType: string,
  position: { x: number; y: number }
): BlueprintNode {
  return createBlueprintNode(
    "variable",
    `Get ${variableName}`,
    position,
    [],
    [createDataPin("output", "Value", variableType as any)],
    {
      variableName,
      variableType,
      operation: "get",
    }
  );
}

/**
 * Create a variable setter node
 */
export function createVariableSetterNode(
  variableName: string,
  variableType: string,
  position: { x: number; y: number }
): BlueprintNode {
  return createBlueprintNode(
    "variable",
    `Set ${variableName}`,
    position,
    [
      createExecPin("input", "Exec"),
      createDataPin("input", "Value", variableType as any, true),
    ],
    [createExecPin("output", "Exec")],
    {
      variableName,
      variableType,
      operation: "set",
    }
  );
}

/**
 * Create a literal value node
 */
export function createLiteralNode(
  value: any,
  valueType: string,
  position: { x: number; y: number }
): BlueprintNode {
  return createBlueprintNode(
    "literal",
    String(value),
    position,
    [],
    [createDataPin("output", "Value", valueType as any)],
    {
      literalValue: value,
      literalType: valueType,
    }
  );
}

/**
 * Create a binary operator node
 */
export function createBinaryOperatorNode(
  operator: string,
  position: { x: number; y: number }
): BlueprintNode {
  // Determine output type based on operator
  let outputType = "any";
  if (["==", "!=", "<", ">", "<=", ">=", "&&", "||"].includes(operator)) {
    outputType = "bool";
  } else if (["+", "-", "*", "/", "%"].includes(operator)) {
    outputType = "int"; // Could be Float, but default to int
  }

  return createBlueprintNode(
    "binary",
    operator,
    position,
    [
      createDataPin("input", "A", "any", true),
      createDataPin("input", "B", "any", true),
    ],
    [createDataPin("output", "Result", outputType as any)],
    {
      operator,
    }
  );
}

/**
 * Create a comparison node (specialized binary operator)
 */
export function createComparisonNode(
  operator: "==" | "!=" | "<" | ">" | "<=" | ">=",
  position: { x: number; y: number }
): BlueprintNode {
  return createBlueprintNode(
    "binary",
    `Compare ${operator}`,
    position,
    [
      createDataPin("input", "A", "any", true),
      createDataPin("input", "B", "any", true),
    ],
    [createDataPin("output", "Result", "bool")],
    {
      operator,
      comparison: true,
    }
  );
}

/**
 * Create a logical operator node
 */
export function createLogicalOperatorNode(
  operator: "&&" | "||",
  position: { x: number; y: number }
): BlueprintNode {
  return createBlueprintNode(
    "binary",
    operator === "&&" ? "AND" : "OR",
    position,
    [
      createDataPin("input", "A", "bool", true),
      createDataPin("input", "B", "bool", true),
    ],
    [createDataPin("output", "Result", "bool")],
    {
      operator,
      logical: true,
    }
  );
}

/**
 * Create a math operator node
 */
export function createMathOperatorNode(
  operator: "+" | "-" | "*" | "/" | "%",
  position: { x: number; y: number }
): BlueprintNode {
  const operatorNames: Record<string, string> = {
    "+": "Add",
    "-": "Subtract",
    "*": "Multiply",
    "/": "Divide",
    "%": "Modulo",
  };

  return createBlueprintNode(
    "binary",
    operatorNames[operator] || operator,
    position,
    [
      createDataPin("input", "A", "int", true),
      createDataPin("input", "B", "int", true),
    ],
    [createDataPin("output", "Result", "int")],
    {
      operator,
      math: true,
    }
  );
}
