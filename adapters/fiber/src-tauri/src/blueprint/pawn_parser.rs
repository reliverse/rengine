/**
 * Pawn language parser implementation
 */
use pest::Parser;
use serde::{Deserialize, Serialize};

use crate::blueprint::parser::{ParseResult, Parser as ParserTrait};

#[derive(pest_derive::Parser)]
#[grammar = "blueprint/grammar/pawn.pest"]
pub struct PawnParserImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PawnAST {
    pub functions: Vec<FunctionNode>,
    pub callbacks: Vec<CallbackNode>,
    pub variables: Vec<VariableNode>,
    pub includes: Vec<String>,
    pub directives: Vec<DirectiveNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionNode {
    pub name: String,
    pub modifiers: Vec<String>,
    pub return_type: String,
    pub parameters: Vec<ParameterNode>,
    pub body: Vec<StatementNode>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackNode {
    pub name: String,
    pub parameters: Vec<ParameterNode>,
    pub body: Vec<StatementNode>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterNode {
    pub name: String,
    pub param_type: String,
    pub default_value: Option<String>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableNode {
    pub name: String,
    pub var_type: String,
    pub initial_value: Option<String>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementNode {
    pub kind: String,
    pub content: serde_json::Value,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectiveNode {
    pub kind: String,
    pub content: String,
    pub line: usize,
    pub column: usize,
}

pub struct PawnParser;

impl PawnParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_program(&self, source: &str) -> Result<PawnAST, Vec<String>> {
        let pairs = PawnParserImpl::parse(Rule::program, source)
            .map_err(|e| vec![format!("Parse error: {}", e)])?;

        let mut ast = PawnAST {
            functions: Vec::new(),
            callbacks: Vec::new(),
            variables: Vec::new(),
            includes: Vec::new(),
            directives: Vec::new(),
        };

        let mut errors = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::function => match self.parse_function(pair) {
                    Ok(func) => ast.functions.push(func),
                    Err(e) => errors.push(e),
                },
                Rule::callback => match self.parse_callback(pair) {
                    Ok(cb) => ast.callbacks.push(cb),
                    Err(e) => errors.push(e),
                },
                Rule::directive => match self.parse_directive(pair) {
                    Ok(dir) => {
                        if dir.kind == "include" {
                            ast.includes.push(dir.content);
                        } else {
                            ast.directives.push(dir);
                        }
                    }
                    Err(e) => errors.push(e),
                },
                Rule::variable_declaration => match self.parse_variable(pair) {
                    Ok(var) => ast.variables.push(var),
                    Err(e) => errors.push(e),
                },
                _ => {}
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(ast)
    }

    fn parse_function(&self, pair: pest::iterators::Pair<Rule>) -> Result<FunctionNode, String> {
        let span = pair.as_span();
        let inner = pair.into_inner();

        let mut modifiers = Vec::new();
        let mut return_type = "void".to_string();
        let mut name = String::new();
        let mut parameters = Vec::new();
        let mut body = Vec::new();

        for pair in inner {
            match pair.as_rule() {
                Rule::function_modifier => {
                    modifiers.push(pair.as_str().to_string());
                }
                Rule::type_name => {
                    return_type = pair.as_str().to_string();
                }
                Rule::identifier => {
                    if name.is_empty() {
                        name = pair.as_str().to_string();
                    }
                }
                Rule::parameter_list => {
                    parameters = self.parse_parameter_list(pair)?;
                }
                Rule::block_statement => {
                    body = self.parse_block(pair)?;
                }
                _ => {}
            }
        }

        Ok(FunctionNode {
            name,
            modifiers,
            return_type,
            parameters,
            body,
            line: span.start_pos().line_col().0,
            column: span.start_pos().line_col().1,
        })
    }

    fn parse_callback(&self, pair: pest::iterators::Pair<Rule>) -> Result<CallbackNode, String> {
        let span = pair.as_span();
        let inner = pair.into_inner();

        let mut name = String::new();
        let mut parameters = Vec::new();
        let mut body = Vec::new();

        for pair in inner {
            match pair.as_rule() {
                Rule::identifier => {
                    if name.is_empty() {
                        name = format!("On{}", pair.as_str());
                    }
                }
                Rule::parameter_list => {
                    parameters = self.parse_parameter_list(pair)?;
                }
                Rule::block_statement => {
                    body = self.parse_block(pair)?;
                }
                _ => {}
            }
        }

        Ok(CallbackNode {
            name,
            parameters,
            body,
            line: span.start_pos().line_col().0,
            column: span.start_pos().line_col().1,
        })
    }

    fn parse_parameter_list(
        &self,
        pair: pest::iterators::Pair<Rule>,
    ) -> Result<Vec<ParameterNode>, String> {
        let mut parameters = Vec::new();

        for param_pair in pair.into_inner() {
            if param_pair.as_rule() == Rule::parameter {
                let span = param_pair.as_span();
                let inner = param_pair.into_inner();

                let mut param_type = String::new();
                let mut name = String::new();
                let mut default_value = None;

                for p in inner {
                    match p.as_rule() {
                        Rule::type_name => {
                            param_type = p.as_str().to_string();
                        }
                        Rule::identifier => {
                            if name.is_empty() {
                                name = p.as_str().to_string();
                            }
                        }
                        Rule::expression => {
                            default_value = Some(p.as_str().to_string());
                        }
                        _ => {}
                    }
                }

                parameters.push(ParameterNode {
                    name,
                    param_type,
                    default_value,
                    line: span.start_pos().line_col().0,
                    column: span.start_pos().line_col().1,
                });
            }
        }

        Ok(parameters)
    }

    fn parse_variable(&self, pair: pest::iterators::Pair<Rule>) -> Result<VariableNode, String> {
        let span = pair.as_span();
        let inner = pair.into_inner();

        let mut var_type = String::new();
        let mut name = String::new();
        let mut initial_value = None;

        for p in inner {
            match p.as_rule() {
                Rule::type_name => {
                    var_type = p.as_str().to_string();
                }
                Rule::identifier => {
                    if name.is_empty() {
                        name = p.as_str().to_string();
                    }
                }
                Rule::expression => {
                    initial_value = Some(p.as_str().to_string());
                }
                _ => {}
            }
        }

        Ok(VariableNode {
            name,
            var_type,
            initial_value,
            line: span.start_pos().line_col().0,
            column: span.start_pos().line_col().1,
        })
    }

    fn parse_block(&self, pair: pest::iterators::Pair<Rule>) -> Result<Vec<StatementNode>, String> {
        let mut statements = Vec::new();

        for stmt_pair in pair.into_inner() {
            if stmt_pair.as_rule() == Rule::statement {
                match self.parse_statement(stmt_pair) {
                    Ok(stmt) => statements.push(stmt),
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(statements)
    }

    fn parse_statement(&self, pair: pest::iterators::Pair<Rule>) -> Result<StatementNode, String> {
        let span = pair.as_span();
        let inner = pair.into_inner().next();

        if let Some(inner_pair) = inner {
            let kind = match inner_pair.as_rule() {
                Rule::expression_statement => "expression",
                Rule::variable_declaration => "variable",
                Rule::assignment_statement => "assignment",
                Rule::if_statement => "if",
                Rule::while_statement => "while",
                Rule::for_statement => "for",
                Rule::return_statement => "return",
                Rule::break_statement => "break",
                Rule::continue_statement => "continue",
                Rule::block_statement => "block",
                _ => "unknown",
            };

            Ok(StatementNode {
                kind: kind.to_string(),
                content: serde_json::json!({ "raw": inner_pair.as_str() }),
                line: span.start_pos().line_col().0,
                column: span.start_pos().line_col().1,
            })
        } else {
            Err("Empty statement".to_string())
        }
    }

    fn parse_directive(&self, pair: pest::iterators::Pair<Rule>) -> Result<DirectiveNode, String> {
        let span = pair.as_span();
        let content = pair.as_str().to_string();
        let inner = pair.into_inner();

        let kind = if let Some(first) = inner.peek() {
            match first.as_rule() {
                Rule::include => "include",
                Rule::define => "define",
                Rule::pragma => "pragma",
                _ => "unknown",
            }
        } else {
            "unknown"
        };

        Ok(DirectiveNode {
            kind: kind.to_string(),
            content,
            line: span.start_pos().line_col().0,
            column: span.start_pos().line_col().1,
        })
    }
}

impl ParserTrait for PawnParser {
    fn parse(&self, source: &str) -> Result<ParseResult, String> {
        match self.parse_program(source) {
            Ok(ast) => {
                let ast_json = serde_json::to_value(&ast)
                    .map_err(|e| format!("Failed to serialize AST: {}", e))?;

                Ok(ParseResult {
                    ast: ast_json,
                    errors: Vec::new(),
                    warnings: Vec::new(),
                })
            }
            Err(errors) => {
                let ast_json = serde_json::json!({});

                Ok(ParseResult {
                    ast: ast_json,
                    errors,
                    warnings: Vec::new(),
                })
            }
        }
    }

    fn language(&self) -> &str {
        "pawn"
    }
}
