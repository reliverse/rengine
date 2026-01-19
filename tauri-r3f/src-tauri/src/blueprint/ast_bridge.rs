/**
 * Bridge between Rust AST and TypeScript AST
 * Converts between language-specific ASTs and the language-agnostic AST format
 */
use serde::{Deserialize, Serialize};

use crate::blueprint::pawn_parser::PawnAST;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAST {
    pub language: String,
    pub functions: Vec<UniversalFunction>,
    pub variables: Vec<UniversalVariable>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalFunction {
    pub id: String,
    pub name: String,
    pub parameters: Vec<UniversalParameter>,
    pub return_type: UniversalType,
    pub body: Vec<UniversalStatement>,
    pub modifiers: Vec<String>,
    pub position: SourcePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalParameter {
    pub id: String,
    pub name: String,
    pub param_type: UniversalType,
    pub default_value: Option<String>,
    pub position: SourcePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalVariable {
    pub id: String,
    pub name: String,
    pub var_type: UniversalType,
    pub initial_value: Option<String>,
    pub position: SourcePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalType {
    pub name: String,
    pub tag: Option<String>,
    pub is_array: bool,
    pub array_size: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalStatement {
    pub id: String,
    pub kind: String,
    pub content: serde_json::Value,
    pub position: SourcePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}

pub struct ASTBridge;

impl ASTBridge {
    /// Convert Pawn AST to Universal AST
    pub fn pawn_to_universal(pawn_ast: &PawnAST) -> UniversalAST {
        let mut functions = Vec::new();
        let mut variables = Vec::new();

        // Convert functions
        for func in &pawn_ast.functions {
            functions.push(UniversalFunction {
                id: format!("func_{}", func.name),
                name: func.name.clone(),
                parameters: func
                    .parameters
                    .iter()
                    .map(|p| UniversalParameter {
                        id: format!("param_{}", p.name),
                        name: p.name.clone(),
                        param_type: UniversalType {
                            name: p.param_type.clone(),
                            tag: None,
                            is_array: false,
                            array_size: None,
                        },
                        default_value: p.default_value.clone(),
                        position: SourcePosition {
                            line: p.line,
                            column: p.column,
                        },
                    })
                    .collect(),
                return_type: UniversalType {
                    name: func.return_type.clone(),
                    tag: None,
                    is_array: false,
                    array_size: None,
                },
                body: func
                    .body
                    .iter()
                    .map(|s| UniversalStatement {
                        id: format!("stmt_{}_{}", s.line, s.column),
                        kind: s.kind.clone(),
                        content: s.content.clone(),
                        position: SourcePosition {
                            line: s.line,
                            column: s.column,
                        },
                    })
                    .collect(),
                modifiers: func.modifiers.clone(),
                position: SourcePosition {
                    line: func.line,
                    column: func.column,
                },
            });
        }

        // Convert callbacks as functions
        for cb in &pawn_ast.callbacks {
            functions.push(UniversalFunction {
                id: format!("callback_{}", cb.name),
                name: cb.name.clone(),
                parameters: cb
                    .parameters
                    .iter()
                    .map(|p| UniversalParameter {
                        id: format!("param_{}", p.name),
                        name: p.name.clone(),
                        param_type: UniversalType {
                            name: p.param_type.clone(),
                            tag: None,
                            is_array: false,
                            array_size: None,
                        },
                        default_value: p.default_value.clone(),
                        position: SourcePosition {
                            line: p.line,
                            column: p.column,
                        },
                    })
                    .collect(),
                return_type: UniversalType {
                    name: "void".to_string(),
                    tag: None,
                    is_array: false,
                    array_size: None,
                },
                body: cb
                    .body
                    .iter()
                    .map(|s| UniversalStatement {
                        id: format!("stmt_{}_{}", s.line, s.column),
                        kind: s.kind.clone(),
                        content: s.content.clone(),
                        position: SourcePosition {
                            line: s.line,
                            column: s.column,
                        },
                    })
                    .collect(),
                modifiers: vec!["public".to_string()],
                position: SourcePosition {
                    line: cb.line,
                    column: cb.column,
                },
            });
        }

        // Convert variables
        for var in &pawn_ast.variables {
            variables.push(UniversalVariable {
                id: format!("var_{}", var.name),
                name: var.name.clone(),
                var_type: UniversalType {
                    name: var.var_type.clone(),
                    tag: None,
                    is_array: false,
                    array_size: None,
                },
                initial_value: var.initial_value.clone(),
                position: SourcePosition {
                    line: var.line,
                    column: var.column,
                },
            });
        }

        UniversalAST {
            language: "pawn".to_string(),
            functions,
            variables,
            metadata: serde_json::json!({
                "includes": pawn_ast.includes,
                "directives": pawn_ast.directives,
            }),
        }
    }

    /// Convert Universal AST to JSON for TypeScript
    pub fn universal_to_json(universal: &UniversalAST) -> serde_json::Value {
        serde_json::to_value(universal).unwrap_or(serde_json::json!({}))
    }
}
