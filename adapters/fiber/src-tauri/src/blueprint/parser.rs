/**
 * Language-agnostic parser trait
 */
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub ast: serde_json::Value,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub trait Parser {
    fn parse(&self, source: &str) -> Result<ParseResult, String>;
    fn language(&self) -> &str;
}
