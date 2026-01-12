use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use crate::RengineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeColumn {
    pub name: String,
    pub r#type: String,
    #[serde(rename = "itemsType")]
    pub items_type: Option<String>,
    pub required: bool,
    pub default: Option<serde_json::Value>,
    pub description: Option<String>,
    #[serde(rename = "variableLength")]
    pub variable_length: Option<bool>,
    #[serde(rename = "dependsOn")]
    pub depends_on: Option<String>,
    pub format: Option<String>,
    pub games: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeSectionType {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub columns: Option<Vec<IdeColumn>>,
    pub games: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeSection {
    #[serde(rename = "supportedGames")]
    pub supported_games: Vec<String>,
    #[serde(rename = "primaryKeys")]
    pub primary_keys: Vec<String>,
    pub description: String,
    pub columns: Vec<IdeColumn>,
    #[serde(rename = "commonPrefix")]
    pub common_prefix: Option<Vec<IdeColumn>>,
    pub types: Option<HashMap<String, IdeSectionType>>,
    pub discriminator: Option<bool>,
    #[serde(rename = "parseHints")]
    pub parse_hints: Option<HashMap<String, serde_json::Value>>,
    pub variants: Option<HashMap<String, IdeVariant>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeVariant {
    #[serde(rename = "insertAfter")]
    pub insert_after: String,
    pub description: String,
    #[serde(rename = "extraColumns")]
    pub extra_columns: Vec<IdeColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeSchema {
    pub sections: HashMap<String, IdeSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeRow {
    pub data: HashMap<String, serde_json::Value>,
    pub extra_fields: Option<Vec<String>>,
    pub raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeSectionData {
    pub rows: Vec<IdeRow>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeDocument {
    pub file_path: String,
    pub sections: HashMap<String, IdeSectionData>,
}

pub struct IdeParser {
    schema: IdeSchema,
}

impl IdeParser {
    pub fn new(schema_json: &str) -> Result<Self, RengineError> {
        let schema: IdeSchema = serde_json::from_str(schema_json)
            .map_err(|e| RengineError::ParseError {
                path: "schema".to_string(),
                details: format!("Failed to parse IDE schema: {}", e),
            })?;

        Ok(IdeParser { schema })
    }

    pub fn parse_file(&self, file_path: &str) -> Result<IdeDocument, RengineError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| RengineError::FileReadFailed {
                path: file_path.to_string(),
                details: e.to_string(),
            })?;

        self.parse_content(&content, file_path)
    }

    pub fn parse_content(&self, content: &str, file_path: &str) -> Result<IdeDocument, RengineError> {
        let lines: Vec<&str> = content.lines().collect();
        let mut parsed_data: HashMap<String, IdeSectionData> = HashMap::new();
        let mut current_section: Option<String> = None;

        // Initialize all sections from schema
        for section_key in self.schema.sections.keys() {
            parsed_data.insert(section_key.clone(), IdeSectionData {
                rows: Vec::new(),
                errors: Vec::new(),
            });
        }

        for line in lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Check for section headers
            if self.schema.sections.contains_key(&line.to_lowercase()) {
                current_section = Some(line.to_lowercase());
                continue;
            }

            // Check for section end
            if line.to_lowercase() == "end" {
                current_section = None;
                continue;
            }

            // Parse row if we're in a section
            if let Some(ref section_key) = current_section {
                if let Some(section_data) = parsed_data.get_mut(section_key) {
                    self.parse_row(line, section_key, section_data);
                }
            }
        }

        Ok(IdeDocument {
            file_path: file_path.to_string(),
            sections: parsed_data,
        })
    }

    fn parse_row(&self, line: &str, section_key: &str, section_data: &mut IdeSectionData) {
        let clean_line = line.split('#').next().unwrap_or("").trim();
        let tokens: Vec<String> = clean_line
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if tokens.is_empty() {
            return;
        }

        let schema_section = match self.schema.sections.get(section_key) {
            Some(s) => s,
            None => return,
        };

        let mut row_data: HashMap<String, serde_json::Value> = HashMap::new();
        let mut token_index = 0;
        let columns = &schema_section.columns;

        // Handle special sections
        if let Some(parse_hints) = &schema_section.parse_hints {
            if parse_hints.contains_key("note") {
                // Store as raw for complex sections like path
                row_data.insert("raw".to_string(), serde_json::Value::String(line.to_string()));
                section_data.rows.push(IdeRow {
                    data: row_data,
                    extra_fields: None,
                    raw: Some(line.to_string()),
                });
                return;
            }
        }

        let mut col_index = 0;
        while col_index < columns.len() && token_index < tokens.len() {
            let col_schema = &columns[col_index];
            let col_name = &col_schema.name;
            let _col_type = &col_schema.r#type;

            match self.parse_column_value(col_schema, &tokens, &mut token_index, &row_data) {
                Ok(value) => {
                    row_data.insert(col_name.clone(), value);
                }
                Err(e) => {
                    let error_msg = format!("Error parsing column '{}' in line '{}': {}", col_name, line, e);
                    section_data.errors.push(error_msg);
                    col_index += 1;
                    continue;
                }
            }

            col_index += 1;
        }

        // Store leftover tokens
        let extra_fields = if token_index < tokens.len() {
            Some(tokens[token_index..].to_vec())
        } else {
            None
        };

        section_data.rows.push(IdeRow {
            data: row_data,
            extra_fields,
            raw: Some(line.to_string()),
        });
    }

    fn parse_column_value(
        &self,
        col_schema: &IdeColumn,
        tokens: &[String],
        token_index: &mut usize,
        row_data: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let col_type = &col_schema.r#type;

        match col_type.as_str() {
            "array" => {
                let mut count = 1;
                if let Some(depends_on) = &col_schema.depends_on {
                    if let Some(serde_json::Value::Number(dep_val)) = row_data.get(depends_on) {
                        if let Some(dep_int) = dep_val.as_i64() {
                            count = dep_int as usize;
                        }
                    }
                }

                let items_type = col_schema.items_type.as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("string");

                let end_slice = std::cmp::min(*token_index + count, tokens.len());
                let array_tokens = &tokens[*token_index..end_slice];

                let array_values: Vec<serde_json::Value> = array_tokens.iter()
                    .map(|token| self.parse_single_value(items_type, token))
                    .collect::<Result<Vec<_>, _>>()?;

                *token_index += array_tokens.len();
                Ok(serde_json::Value::Array(array_values))
            }
            _ => {
                if *token_index >= tokens.len() {
                    return Err("Not enough tokens".to_string());
                }
                let value = self.parse_single_value(col_type, &tokens[*token_index])?;
                *token_index += 1;
                Ok(value)
            }
        }
    }

    fn parse_single_value(&self, value_type: &str, token: &str) -> Result<serde_json::Value, String> {
        match value_type {
            "int" => {
                token.parse::<i64>()
                    .map(|n| serde_json::Value::Number(n.into()))
                    .map_err(|e| format!("Invalid integer '{}': {}", token, e))
            }
            "float" => {
                token.parse::<f64>()
                    .map(|n| serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap_or(serde_json::Number::from(0))))
                    .map_err(|e| format!("Invalid float '{}': {}", token, e))
            }
            "string" => Ok(serde_json::Value::String(token.to_string())),
            _ => Ok(serde_json::Value::String(token.to_string())),
        }
    }

    pub fn serialize_to_string(&self, document: &IdeDocument) -> Result<String, RengineError> {
        let mut lines = Vec::new();

        for (section_key, section_data) in &document.sections {
            if section_data.rows.is_empty() {
                continue;
            }

            // Add section header
            lines.push(section_key.clone());

            // Serialize each row
            for row in &section_data.rows {
                if let Some(serialized) = self.serialize_row(row, section_key) {
                    lines.push(serialized);
                }
            }

            // Add section end
            lines.push("end".to_string());
            lines.push(String::new()); // Empty line after section
        }

        Ok(lines.join("\n"))
    }

    fn serialize_row(&self, row: &IdeRow, section_key: &str) -> Option<String> {
        // For raw rows (complex sections), return the raw content
        if let Some(raw) = &row.raw {
            return Some(raw.clone());
        }

        let schema_section = self.schema.sections.get(section_key)?;
        let mut tokens = Vec::new();

        // Serialize regular columns
        for col in &schema_section.columns {
            let col_name = &col.name;
            if let Some(value) = row.data.get(col_name) {
                match value {
                    serde_json::Value::Array(arr) => {
                        // Handle arrays
                        for item in arr {
                            tokens.push(self.serialize_value(item));
                        }
                    }
                    _ => {
                        tokens.push(self.serialize_value(value));
                    }
                }
            }
        }

        // Add extra fields if present
        if let Some(extra) = &row.extra_fields {
            tokens.extend(extra.clone());
        }

        if tokens.is_empty() {
            None
        } else {
            Some(tokens.join(", "))
        }
    }

    fn serialize_value(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            _ => value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let schema_json = r#"{
            "sections": {
                "objs": {
                    "supportedGames": ["GTA III"],
                    "primaryKeys": ["id"],
                    "description": "Test section",
                    "columns": [
                        {"name": "id", "type": "int", "required": true},
                        {"name": "modelName", "type": "string", "required": true}
                    ]
                }
            }
        }"#;

        let parser = IdeParser::new(schema_json).unwrap();
        let content = "objs\n123, testmodel\nend\n";

        let result = parser.parse_content(&content, "test.ide").unwrap();
        assert_eq!(result.sections["objs"].rows.len(), 1);
        assert_eq!(result.sections["objs"].rows[0].data["id"], serde_json::json!(123));
        assert_eq!(result.sections["objs"].rows[0].data["modelName"], serde_json::json!("testmodel"));
    }
}