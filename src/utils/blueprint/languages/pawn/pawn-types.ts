/**
 * Pawn type system definitions
 */

export type PawnType = "int" | "Float" | "bool" | "String" | "void";

export interface PawnTypeDefinition {
  name: PawnType;
  tag?: string;
  isArray: boolean;
  arraySize?: number;
}

export const PAWN_TYPES: Record<PawnType, PawnTypeDefinition> = {
  int: {
    name: "int",
    isArray: false,
  },
  Float: {
    name: "Float",
    tag: "Float",
    isArray: false,
  },
  bool: {
    name: "bool",
    isArray: false,
  },
  String: {
    name: "String",
    tag: "String",
    isArray: false,
  },
  void: {
    name: "void",
    isArray: false,
  },
};

/**
 * Create a Pawn type definition
 */
export function createPawnType(
  name: PawnType,
  isArray = false,
  arraySize?: number
): PawnTypeDefinition {
  return {
    name,
    tag: PAWN_TYPES[name]?.tag,
    isArray,
    arraySize,
  };
}
