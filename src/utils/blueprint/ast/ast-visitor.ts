/**
 * Visitor pattern implementation for AST traversal.
 * Allows for easy traversal and transformation of AST nodes.
 */

import type {
  ASTNode,
  ProgramNode,
  FunctionNode,
  StatementNode,
  ExpressionNode,
  VariableNode,
  ParameterNode,
  CommentNode,
} from "./ast-core";

export interface ASTVisitor {
  visitProgram?(node: ProgramNode): void;
  visitFunction?(node: FunctionNode): void;
  visitStatement?(node: StatementNode): void;
  visitExpression?(node: ExpressionNode): void;
  visitVariable?(node: VariableNode): void;
  visitParameter?(node: ParameterNode): void;
  visitComment?(node: CommentNode): void;
  visitNode?(node: ASTNode): void;
}

export interface ASTTransformer<T extends ASTNode = ASTNode> {
  transformProgram?(node: ProgramNode): ProgramNode;
  transformFunction?(node: FunctionNode): FunctionNode;
  transformStatement?(node: StatementNode): StatementNode;
  transformExpression?(node: ExpressionNode): ExpressionNode;
  transformVariable?(node: VariableNode): VariableNode;
  transformParameter?(node: ParameterNode): ParameterNode;
  transformComment?(node: CommentNode): CommentNode;
  transformNode?(node: ASTNode): T;
}

/**
 * Base AST traverser that visits all nodes in an AST
 */
export class ASTTraverser {
  private readonly visitor: ASTVisitor;

  constructor(visitor: ASTVisitor) {
    this.visitor = visitor;
  }

  traverse(node: ASTNode): void {
    this.visit(node);
  }

  private visit(node: ASTNode): void {
    // Call generic visitNode first
    this.visitor.visitNode?.(node);

    // Then call specific visitor methods
    switch (node.type) {
      case "program":
        this.visitor.visitProgram?.(node as ProgramNode);
        this.visitProgram(node as ProgramNode);
        break;
      case "function":
        this.visitor.visitFunction?.(node as FunctionNode);
        this.visitFunction(node as FunctionNode);
        break;
      case "statement":
        this.visitor.visitStatement?.(node as StatementNode);
        this.visitStatement(node as StatementNode);
        break;
      case "expression":
        this.visitor.visitExpression?.(node as ExpressionNode);
        this.visitExpression(node as ExpressionNode);
        break;
      case "variable":
        this.visitor.visitVariable?.(node as VariableNode);
        this.visitVariable(node as VariableNode);
        break;
      case "parameter":
        this.visitor.visitParameter?.(node as ParameterNode);
        this.visitParameter(node as ParameterNode);
        break;
      case "comment":
        this.visitor.visitComment?.(node as CommentNode);
        break;
      default:
        // For other node types, just visit children if they exist
        this.visitChildren(node);
    }
  }

  private visitProgram(node: ProgramNode): void {
    for (const fn of node.functions) {
      this.visit(fn);
    }
    for (const varNode of node.variables) {
      this.visit(varNode);
    }
    if (node.comments) {
      for (const comment of node.comments) {
        this.visit(comment);
      }
    }
  }

  private visitFunction(node: FunctionNode): void {
    for (const param of node.parameters) {
      this.visit(param);
    }
    for (const stmt of node.body) {
      this.visit(stmt);
    }
  }

  private visitStatement(node: StatementNode): void {
    const stmt = node.statement;
    if (!stmt) return;

    switch (node.kind) {
      case "if": {
        const ifStmt = stmt as any;
        if (ifStmt.condition) this.visit(ifStmt.condition);
        if (Array.isArray(ifStmt.then)) {
          for (const s of ifStmt.then) {
            this.visit(s);
          }
        } else if (ifStmt.then) {
          this.visit(ifStmt.then);
        }
        if (ifStmt.else) {
          if (Array.isArray(ifStmt.else)) {
            for (const s of ifStmt.else) {
              this.visit(s);
            }
          } else {
            this.visit(ifStmt.else);
          }
        }
        break;
      }
      case "while": {
        const whileStmt = stmt as any;
        if (whileStmt.condition) this.visit(whileStmt.condition);
        if (Array.isArray(whileStmt.body)) {
          for (const s of whileStmt.body) {
            this.visit(s);
          }
        } else if (whileStmt.body) {
          this.visit(whileStmt.body);
        }
        break;
      }
      case "for": {
        const forStmt = stmt as any;
        if (forStmt.init) this.visit(forStmt.init);
        if (forStmt.condition) this.visit(forStmt.condition);
        if (forStmt.update) this.visit(forStmt.update);
        if (Array.isArray(forStmt.body)) {
          for (const s of forStmt.body) {
            this.visit(s);
          }
        } else if (forStmt.body) {
          this.visit(forStmt.body);
        }
        break;
      }
      case "return": {
        const returnStmt = stmt as any;
        if (returnStmt.value) this.visit(returnStmt.value);
        break;
      }
      case "assignment": {
        const assignStmt = stmt as any;
        if (assignStmt.target) this.visit(assignStmt.target);
        if (assignStmt.value) this.visit(assignStmt.value);
        break;
      }
      case "expression": {
        if (stmt) this.visit(stmt as ExpressionNode);
        break;
      }
      case "block": {
        const blockStmt = stmt as any;
        if (blockStmt.statements) {
          for (const s of blockStmt.statements) {
            this.visit(s);
          }
        }
        break;
      }
      case "variable": {
        const varStmt = stmt as any;
        if (varStmt.initialValue) this.visit(varStmt.initialValue);
        break;
      }
    }
  }

  private visitExpression(node: ExpressionNode): void {
    const expr = node.expression;
    if (!expr) return;

    switch (node.kind) {
      case "binary": {
        const binExpr = expr as any;
        if (binExpr.left) this.visit(binExpr.left);
        if (binExpr.right) this.visit(binExpr.right);
        break;
      }
      case "unary": {
        const unaryExpr = expr as any;
        if (unaryExpr.operand) this.visit(unaryExpr.operand);
        break;
      }
      case "call": {
        const callExpr = expr as any;
        if (callExpr.callee && typeof callExpr.callee !== "string") {
          this.visit(callExpr.callee);
        }
        if (callExpr.arguments) {
          for (const arg of callExpr.arguments) {
            this.visit(arg);
          }
        }
        break;
      }
      case "member": {
        const memberExpr = expr as any;
        if (memberExpr.object) this.visit(memberExpr.object);
        break;
      }
      case "index": {
        const indexExpr = expr as any;
        if (indexExpr.object) this.visit(indexExpr.object);
        if (indexExpr.index) this.visit(indexExpr.index);
        break;
      }
      case "ternary": {
        const ternaryExpr = expr as any;
        if (ternaryExpr.condition) this.visit(ternaryExpr.condition);
        if (ternaryExpr.then) this.visit(ternaryExpr.then);
        if (ternaryExpr.else) this.visit(ternaryExpr.else);
        break;
      }
    }
  }

  private visitVariable(node: VariableNode): void {
    if (node.initialValue) {
      this.visit(node.initialValue);
    }
  }

  private visitParameter(node: ParameterNode): void {
    if (node.defaultValue) {
      this.visit(node.defaultValue);
    }
  }

  private visitChildren(node: ASTNode): void {
    // Generic children visitor for nodes with unknown structure
    if ("children" in node && Array.isArray((node as any).children)) {
      for (const child of (node as any).children) {
        this.visit(child);
      }
    }
  }
}

/**
 * Transformer that applies transformations to AST nodes
 */
export class ASTTransformerClass {
  private readonly transformer: ASTTransformer;

  constructor(transformer: ASTTransformer) {
    this.transformer = transformer;
  }

  transform<T extends ASTNode>(node: ASTNode): T {
    return this.transformNode(node) as T;
  }

  private transformNode(node: ASTNode): ASTNode {
    // Call generic transformNode first
    if (this.transformer.transformNode) {
      const transformed = this.transformer.transformNode(node);
      if (transformed) return transformed;
    }

    // Then call specific transformer methods
    switch (node.type) {
      case "program":
        return this.transformer.transformProgram?.(node as ProgramNode) ?? node;
      case "function":
        return (
          this.transformer.transformFunction?.(node as FunctionNode) ?? node
        );
      case "statement":
        return (
          this.transformer.transformStatement?.(node as StatementNode) ?? node
        );
      case "expression":
        return (
          this.transformer.transformExpression?.(node as ExpressionNode) ?? node
        );
      case "variable":
        return (
          this.transformer.transformVariable?.(node as VariableNode) ?? node
        );
      case "parameter":
        return (
          this.transformer.transformParameter?.(node as ParameterNode) ?? node
        );
      case "comment":
        return this.transformer.transformComment?.(node as CommentNode) ?? node;
      default:
        return node;
    }
  }
}
