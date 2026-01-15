/**
 * Pawn code generator from AST
 */
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOptions {
    pub indent_size: usize,
    pub use_tabs: bool,
    pub preserve_comments: bool,
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            indent_size: 4,
            use_tabs: false,
            preserve_comments: true,
        }
    }
}

pub struct PawnGenerator {
    options: GenerationOptions,
}

impl PawnGenerator {
    pub fn new() -> Self {
        Self {
            options: GenerationOptions::default(),
        }
    }

    pub fn with_options(options: GenerationOptions) -> Self {
        Self { options }
    }

    pub fn generate(&self, ast: &serde_json::Value) -> Result<String, String> {
        let mut output = String::new();

        // Generate includes
        if let Some(includes) = ast.get("includes").and_then(|v| v.as_array()) {
            for include in includes {
                if let Some(include_str) = include.as_str() {
                    output.push_str(&format!("#include <{}>\n", include_str));
                }
            }
            if !includes.is_empty() {
                output.push('\n');
            }
        }

        // Generate functions
        if let Some(functions) = ast.get("functions").and_then(|v| v.as_array()) {
            for func in functions {
                output.push_str(&self.generate_function(func)?);
                output.push_str("\n\n");
            }
        }

        // Generate callbacks
        if let Some(callbacks) = ast.get("callbacks").and_then(|v| v.as_array()) {
            for callback in callbacks {
                output.push_str(&self.generate_callback(callback)?);
                output.push_str("\n\n");
            }
        }

        Ok(output)
    }

    fn generate_function(&self, func: &serde_json::Value) -> Result<String, String> {
        let mut output = String::new();

        // Modifiers
        if let Some(modifiers) = func.get("modifiers").and_then(|v| v.as_array()) {
            for modifier in modifiers {
                if let Some(mod_str) = modifier.as_str() {
                    output.push_str(mod_str);
                    output.push(' ');
                }
            }
        }

        // Return type
        if let Some(return_type) = func.get("return_type").and_then(|v| v.as_str()) {
            output.push_str(return_type);
            output.push(' ');
        }

        // Function name
        if let Some(name) = func.get("name").and_then(|v| v.as_str()) {
            output.push_str(name);
        }

        // Parameters
        output.push('(');
        if let Some(parameters) = func.get("parameters").and_then(|v| v.as_array()) {
            let param_strs: Vec<String> = parameters
                .iter()
                .filter_map(|p| self.generate_parameter(p).ok())
                .collect();
            output.push_str(&param_strs.join(", "));
        }
        output.push(')');

        // Body
        if let Some(body) = func.get("body").and_then(|v| v.as_array()) {
            output.push_str(" {\n");
            for stmt in body {
                output.push_str(&self.generate_statement(stmt, 1)?);
            }
            output.push('}');
        } else {
            output.push(';');
        }

        Ok(output)
    }

    fn generate_callback(&self, callback: &serde_json::Value) -> Result<String, String> {
        let mut output = String::new();

        output.push_str("public ");

        // Callback name
        if let Some(name) = callback.get("name").and_then(|v| v.as_str()) {
            output.push_str(name);
        }

        // Parameters
        output.push('(');
        if let Some(parameters) = callback.get("parameters").and_then(|v| v.as_array()) {
            let param_strs: Vec<String> = parameters
                .iter()
                .filter_map(|p| self.generate_parameter(p).ok())
                .collect();
            output.push_str(&param_strs.join(", "));
        }
        output.push(')');

        // Body
        if let Some(body) = callback.get("body").and_then(|v| v.as_array()) {
            output.push_str(" {\n");
            for stmt in body {
                output.push_str(&self.generate_statement(stmt, 1)?);
            }
            output.push('}');
        }

        Ok(output)
    }

    fn generate_parameter(&self, param: &serde_json::Value) -> Result<String, String> {
        let mut output = String::new();

        // Type
        if let Some(param_type) = param.get("param_type").and_then(|v| v.as_str()) {
            output.push_str(param_type);
            output.push(' ');
        }

        // Name
        if let Some(name) = param.get("name").and_then(|v| v.as_str()) {
            output.push_str(name);
        }

        // Default value
        if let Some(default) = param.get("default_value").and_then(|v| v.as_str()) {
            output.push_str(" = ");
            output.push_str(default);
        }

        Ok(output)
    }

    fn get_indent(&self, level: usize) -> String {
        if self.options.use_tabs {
            "\t".repeat(level)
        } else {
            " ".repeat(level * self.options.indent_size)
        }
    }

    fn generate_expression(&self, expr: &serde_json::Value) -> Result<String, String> {
        // Check if it's a structured expression or raw string
        if let Some(kind) = expr.get("kind").and_then(|v| v.as_str()) {
            match kind {
                "literal" => {
                    if let Some(value) = expr.get("value") {
                        if let Some(str_val) = value.as_str() {
                            return Ok(format!("\"{}\"", str_val));
                        }
                        if let Some(num_val) = value.as_f64() {
                            return Ok(num_val.to_string());
                        }
                        if let Some(bool_val) = value.as_bool() {
                            return Ok(if bool_val { "true" } else { "false" }.to_string());
                        }
                        if value.is_null() {
                            return Ok("NULL".to_string());
                        }
                    }
                    Ok(expr
                        .get("content")
                        .and_then(|c| c.get("raw"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string())
                }
                "variable" => Ok(expr
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()),
                "binary" => self.generate_binary_expression(expr),
                "unary" => self.generate_unary_expression(expr),
                "call" => self.generate_function_call(expr),
                "member" => {
                    let object =
                        self.generate_expression(expr.get("object").ok_or("Missing object")?)?;
                    let property = expr.get("property").and_then(|v| v.as_str()).unwrap_or("");
                    Ok(format!("{}.{}", object, property))
                }
                "index" => {
                    let object =
                        self.generate_expression(expr.get("object").ok_or("Missing object")?)?;
                    let index =
                        self.generate_expression(expr.get("index").ok_or("Missing index")?)?;
                    Ok(format!("{}[{}]", object, index))
                }
                _ => {
                    // Fallback to raw content if available
                    Ok(expr
                        .get("content")
                        .and_then(|c| c.get("raw"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string())
                }
            }
        } else if let Some(raw) = expr.get("raw").and_then(|v| v.as_str()) {
            Ok(raw.to_string())
        } else if let Some(str_val) = expr.as_str() {
            Ok(str_val.to_string())
        } else {
            Ok("".to_string())
        }
    }

    fn generate_binary_expression(&self, expr: &serde_json::Value) -> Result<String, String> {
        let operator = expr.get("operator").and_then(|v| v.as_str()).unwrap_or("");
        let left_expr = expr.get("left").ok_or("Missing left operand")?;
        let right_expr = expr.get("right").ok_or("Missing right operand")?;

        let left = self.generate_expression(left_expr)?;
        let right = self.generate_expression(right_expr)?;

        // Add parentheses for clarity (can be optimized later based on precedence)
        Ok(format!("({} {} {})", left, operator, right))
    }

    fn generate_unary_expression(&self, expr: &serde_json::Value) -> Result<String, String> {
        let operator = expr.get("operator").and_then(|v| v.as_str()).unwrap_or("");
        let operand_expr = expr.get("operand").ok_or("Missing operand")?;

        let operand = self.generate_expression(operand_expr)?;

        // Handle prefix vs postfix operators
        if operator == "++" || operator == "--" {
            // Check if it's prefix or postfix (default to prefix)
            let is_postfix = expr
                .get("postfix")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if is_postfix {
                Ok(format!("{}{}", operand, operator))
            } else {
                Ok(format!("{}{}", operator, operand))
            }
        } else {
            Ok(format!("{}{}", operator, operand))
        }
    }

    fn generate_function_call(&self, expr: &serde_json::Value) -> Result<String, String> {
        let callee = if let Some(callee_expr) = expr.get("callee") {
            self.generate_expression(callee_expr)?
        } else if let Some(callee_str) = expr.get("callee").and_then(|v| v.as_str()) {
            callee_str.to_string()
        } else {
            expr.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };

        let mut args = Vec::new();
        if let Some(arguments) = expr.get("arguments").and_then(|v| v.as_array()) {
            for arg in arguments {
                args.push(self.generate_expression(arg)?);
            }
        }

        Ok(format!("{}({})", callee, args.join(", ")))
    }

    fn generate_statement(
        &self,
        stmt: &serde_json::Value,
        indent_level: usize,
    ) -> Result<String, String> {
        let indent = self.get_indent(indent_level);

        let kind = stmt
            .get("kind")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        match kind {
            "variable" => self.generate_variable_declaration(stmt, &indent),
            "assignment" => self.generate_assignment_statement(stmt, &indent),
            "expression" => {
                let expr = stmt.get("content").ok_or("Missing expression content")?;
                let expr_str = self.generate_expression(expr)?;
                Ok(format!("{}{};\n", indent, expr_str))
            }
            "return" => self.generate_return_statement(stmt, &indent),
            "if" => self.generate_if_statement(stmt, indent_level),
            "while" => self.generate_while_statement(stmt, indent_level),
            "for" => self.generate_for_statement(stmt, indent_level),
            "break" => Ok(format!("{}break;\n", indent)),
            "continue" => Ok(format!("{}continue;\n", indent)),
            "block" => self.generate_block_statement(stmt, indent_level),
            _ => {
                // Fallback to raw content
                let content = stmt
                    .get("content")
                    .and_then(|c| c.get("raw"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                Ok(format!("{}{}\n", indent, content))
            }
        }
    }

    fn generate_variable_declaration(
        &self,
        stmt: &serde_json::Value,
        indent: &str,
    ) -> Result<String, String> {
        let var_type = stmt
            .get("var_type")
            .and_then(|v| v.as_str())
            .unwrap_or("int");
        let name = stmt.get("name").and_then(|v| v.as_str()).unwrap_or("");

        let mut output = format!("{}new {} {}", indent, var_type, name);

        if let Some(initial_value) = stmt.get("initial_value") {
            let value_str = self.generate_expression(initial_value)?;
            output.push_str(&format!(" = {}", value_str));
        } else if let Some(content) = stmt.get("content") {
            if let Some(raw) = content.get("raw").and_then(|v| v.as_str()) {
                // Try to extract initial value from raw content
                if raw.contains('=') {
                    output.push_str(&format!(
                        " = {}",
                        raw.split('=').nth(1).unwrap_or("").trim()
                    ));
                }
            }
        }

        Ok(format!("{};\n", output))
    }

    fn generate_assignment_statement(
        &self,
        stmt: &serde_json::Value,
        indent: &str,
    ) -> Result<String, String> {
        let target = if let Some(target_expr) = stmt.get("target") {
            self.generate_expression(target_expr)?
        } else {
            stmt.get("content")
                .and_then(|c| c.get("raw"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };

        let operator = stmt.get("operator").and_then(|v| v.as_str()).unwrap_or("=");
        let value = if let Some(value_expr) = stmt.get("value") {
            self.generate_expression(value_expr)?
        } else {
            "".to_string()
        };

        Ok(format!("{}{} {} {};\n", indent, target, operator, value))
    }

    fn generate_return_statement(
        &self,
        stmt: &serde_json::Value,
        indent: &str,
    ) -> Result<String, String> {
        if let Some(value) = stmt.get("value") {
            let value_str = self.generate_expression(value)?;
            Ok(format!("{}return {};\n", indent, value_str))
        } else if let Some(content) = stmt.get("content") {
            if let Some(raw) = content.get("raw").and_then(|v| v.as_str()) {
                if raw.trim().is_empty() {
                    Ok(format!("{}return;\n", indent))
                } else {
                    Ok(format!("{}return {};\n", indent, raw))
                }
            } else {
                Ok(format!("{}return;\n", indent))
            }
        } else {
            Ok(format!("{}return;\n", indent))
        }
    }

    fn generate_if_statement(
        &self,
        stmt: &serde_json::Value,
        indent_level: usize,
    ) -> Result<String, String> {
        let indent = self.get_indent(indent_level);

        let condition = if let Some(cond_expr) = stmt.get("condition") {
            self.generate_expression(cond_expr)?
        } else {
            stmt.get("content")
                .and_then(|c| c.get("raw"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };

        let then_body = if let Some(then_part) = stmt.get("then") {
            self.generate_statement_block(then_part, indent_level + 1)?
        } else {
            String::new()
        };

        let else_body = if let Some(else_part) = stmt.get("else") {
            format!(
                "\n{}else {}",
                indent,
                self.generate_statement_block(else_part, indent_level + 1)?
            )
        } else {
            String::new()
        };

        Ok(format!(
            "{}if ({}) {{\n{}{}}}{}\n",
            indent, condition, then_body, indent, else_body
        ))
    }

    fn generate_while_statement(
        &self,
        stmt: &serde_json::Value,
        indent_level: usize,
    ) -> Result<String, String> {
        let indent = self.get_indent(indent_level);

        let condition = if let Some(cond_expr) = stmt.get("condition") {
            self.generate_expression(cond_expr)?
        } else {
            stmt.get("content")
                .and_then(|c| c.get("raw"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };

        let body = if let Some(body_part) = stmt.get("body") {
            self.generate_statement_block(body_part, indent_level + 1)?
        } else {
            String::new()
        };

        Ok(format!(
            "{}while ({}) {{\n{}{}}}\n",
            indent, condition, body, indent
        ))
    }

    fn generate_for_statement(
        &self,
        stmt: &serde_json::Value,
        indent_level: usize,
    ) -> Result<String, String> {
        let indent = self.get_indent(indent_level);

        let init = if let Some(init_part) = stmt.get("init") {
            if let Some(init_stmt) = init_part.as_object() {
                if init_stmt.contains_key("kind") {
                    self.generate_statement(init_part, 0)?.trim().to_string()
                } else {
                    self.generate_expression(init_part)?.trim().to_string()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let condition = if let Some(cond_expr) = stmt.get("condition") {
            self.generate_expression(cond_expr)?
        } else {
            String::new()
        };

        let update = if let Some(update_expr) = stmt.get("update") {
            self.generate_expression(update_expr)?
        } else {
            String::new()
        };

        let body = if let Some(body_part) = stmt.get("body") {
            self.generate_statement_block(body_part, indent_level + 1)?
        } else {
            String::new()
        };

        Ok(format!(
            "{}for ({}; {}; {}) {{\n{}{}}}\n",
            indent, init, condition, update, body, indent
        ))
    }

    fn generate_block_statement(
        &self,
        stmt: &serde_json::Value,
        indent_level: usize,
    ) -> Result<String, String> {
        let indent = self.get_indent(indent_level);

        if let Some(statements) = stmt.get("statements").and_then(|v| v.as_array()) {
            let mut body = String::new();
            for stmt_node in statements {
                body.push_str(&self.generate_statement(stmt_node, indent_level + 1)?);
            }
            Ok(format!("{}{{\n{}{}}}\n", indent, body, indent))
        } else {
            let _indent = self.get_indent(indent_level);
            Ok(format!("{}{{\n{}}}\n", indent, indent))
        }
    }

    fn generate_statement_block(
        &self,
        block: &serde_json::Value,
        indent_level: usize,
    ) -> Result<String, String> {
        if let Some(statements) = block.as_array() {
            let mut body = String::new();
            for stmt in statements {
                body.push_str(&self.generate_statement(stmt, indent_level)?);
            }
            Ok(body)
        } else if block.get("kind").and_then(|v| v.as_str()) == Some("block") {
            self.generate_block_statement(block, indent_level)
        } else {
            // Single statement
            self.generate_statement(block, indent_level)
        }
    }
}

impl Default for PawnGenerator {
    fn default() -> Self {
        Self::new()
    }
}
