/**
 * Core AST (Abstract Syntax Tree) types for language-agnostic code representation.
 * This forms the foundation for the Blueprint Visual Scripting system.
 */

export type NodeType =
  | "program"
  | "function"
  | "variable"
  | "parameter"
  | "statement"
  | "expression"
  | "assignment"
  | "call"
  | "if"
  | "while"
  | "for"
  | "return"
  | "break"
  | "continue"
  | "block"
  | "literal"
  | "binary"
  | "unary"
  | "member"
  | "index"
  | "type"
  | "comment";

export type DataType =
  | "void"
  | "int"
  | "float"
  | "bool"
  | "string"
  | "array"
  | "object"
  | "function"
  | "any";

export interface SourcePosition {
  line: number;
  column: number;
  offset?: number;
}

export interface SourceRange {
  start: SourcePosition;
  end: SourcePosition;
}

export interface ASTNode {
  id: string;
  type: NodeType;
  position: SourcePosition;
  range?: SourceRange;
  metadata?: Record<string, any>;
}

export interface TypeNode extends ASTNode {
  type: "type";
  name: string;
  isArray?: boolean;
  arraySize?: number;
  genericTypes?: TypeNode[];
  isNullable?: boolean;
}

export interface LiteralNode extends ASTNode {
  type: "literal";
  value: string | number | boolean | null;
  literalType: DataType;
}

export interface VariableNode extends ASTNode {
  type: "variable";
  name: string;
  variableType: TypeNode;
  isConstant?: boolean;
  initialValue?: ExpressionNode;
}

export interface ParameterNode extends ASTNode {
  type: "parameter";
  name: string;
  parameterType: TypeNode;
  defaultValue?: ExpressionNode;
  isReference?: boolean;
  isConst?: boolean;
}

export interface ExpressionNode extends ASTNode {
  type: "expression";
  kind:
    | "literal"
    | "variable"
    | "call"
    | "binary"
    | "unary"
    | "member"
    | "index"
    | "ternary";
  expression: any; // Will be one of the specific expression types
}

export interface BinaryExpressionNode extends ASTNode {
  type: "expression";
  kind: "binary";
  operator: string;
  left: ExpressionNode;
  right: ExpressionNode;
}

export interface UnaryExpressionNode extends ASTNode {
  type: "expression";
  kind: "unary";
  operator: string;
  operand: ExpressionNode;
}

export interface CallExpressionNode extends ASTNode {
  type: "expression";
  kind: "call";
  callee: ExpressionNode | string; // Function name or expression
  arguments: ExpressionNode[];
}

export interface MemberExpressionNode extends ASTNode {
  type: "expression";
  kind: "member";
  object: ExpressionNode;
  property: string;
}

export interface IndexExpressionNode extends ASTNode {
  type: "expression";
  kind: "index";
  object: ExpressionNode;
  index: ExpressionNode;
}

export interface StatementNode extends ASTNode {
  type: "statement";
  kind:
    | "expression"
    | "assignment"
    | "if"
    | "while"
    | "for"
    | "return"
    | "break"
    | "continue"
    | "block"
    | "variable";
  statement: any; // Will be one of the specific statement types
}

export interface AssignmentStatementNode extends ASTNode {
  type: "statement";
  kind: "assignment";
  target: ExpressionNode;
  operator?: string; // '=', '+=', '-=', etc.
  value: ExpressionNode;
}

export interface IfStatementNode extends ASTNode {
  type: "statement";
  kind: "if";
  condition: ExpressionNode;
  then: StatementNode | StatementNode[];
  else?: StatementNode | StatementNode[];
}

export interface WhileStatementNode extends ASTNode {
  type: "statement";
  kind: "while";
  condition: ExpressionNode;
  body: StatementNode | StatementNode[];
}

export interface ForStatementNode extends ASTNode {
  type: "statement";
  kind: "for";
  init?: StatementNode | ExpressionNode;
  condition?: ExpressionNode;
  update?: ExpressionNode;
  body: StatementNode | StatementNode[];
}

export interface ReturnStatementNode extends ASTNode {
  type: "statement";
  kind: "return";
  value?: ExpressionNode;
}

export interface BlockStatementNode extends ASTNode {
  type: "statement";
  kind: "block";
  statements: StatementNode[];
}

export interface FunctionNode extends ASTNode {
  type: "function";
  name: string;
  parameters: ParameterNode[];
  returnType: TypeNode;
  body: StatementNode[];
  isPublic?: boolean;
  isStatic?: boolean;
  isAsync?: boolean;
  modifiers?: string[];
}

export interface CommentNode extends ASTNode {
  type: "comment";
  text: string;
  isBlock?: boolean; // true for /* */, false for //
}

export interface ProgramNode extends ASTNode {
  type: "program";
  language: string;
  functions: FunctionNode[];
  variables: VariableNode[];
  imports?: string[];
  comments?: CommentNode[];
}

/**
 * Helper function to generate unique IDs for AST nodes
 */
export function generateASTNodeId(prefix = "ast"): string {
  return `${prefix}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * Type guard functions
 */
export function isFunctionNode(node: ASTNode): node is FunctionNode {
  return node.type === "function";
}

export function isStatementNode(node: ASTNode): node is StatementNode {
  return node.type === "statement";
}

export function isExpressionNode(node: ASTNode): node is ExpressionNode {
  return node.type === "expression";
}

export function isVariableNode(node: ASTNode): node is VariableNode {
  return node.type === "variable";
}

export function isTypeNode(node: ASTNode): node is TypeNode {
  return node.type === "type";
}

export function isLiteralNode(node: ASTNode): node is LiteralNode {
  return node.type === "literal";
}

export function isCommentNode(node: ASTNode): node is CommentNode {
  return node.type === "comment";
}
