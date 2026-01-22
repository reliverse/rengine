use rengine_core::renderware::analyzer::RwAnalyzer;
use std::fs;
use anyhow::{Result, anyhow};
use tracing::{info, error};

use crate::RwFormat;

pub struct RWHandler {
    verbose: bool,
    quiet: bool,
}

impl RWHandler {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub async fn analyze(&self, file_path: &str, depth: usize, format: RwFormat) -> Result<i32> {
        if self.verbose {
            info!("Analyzing RenderWare file: {}", file_path);
        }

        // Read file data
        let data = match fs::read(file_path) {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to read file {}: {}", file_path, e);
                return Err(anyhow!("Failed to read file: {}", e));
            }
        };

        let file_size = data.len() as u64;

        // Create analyzer
        let analyzer = RwAnalyzer::new();

        // Analyze the file
        let analysis = match analyzer.analyze_file(file_path, &data) {
            Ok(analysis) => analysis,
            Err(e) => {
                error!("Failed to analyze RenderWare file {}: {}", file_path, e);
                return Err(anyhow!("Failed to analyze RenderWare file: {}", e));
            }
        };

        let mut result = serde_json::json!({
            "file": file_path,
            "size": file_size,
            "format": analysis.format,
            "format_description": analysis.format_description,
            "rw_version": format!("0x{:08X}", analysis.rw_version),
            "total_chunks": analysis.total_chunks,
            "max_depth": analysis.max_depth,
            "analysis_time_ms": analysis.analysis_time_ms
        });

        if !analysis.corruption_warnings.is_empty() {
            result["corruption_warnings"] = serde_json::to_value(&analysis.corruption_warnings)?;
        }

        // Add chunk tree summary (limited depth for readability)
        result["chunk_tree"] = self.build_chunk_tree_summary(&analysis.root_chunk, 0, depth);

        match format {
            RwFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
            RwFormat::Tree | RwFormat::Text => {
                if !self.quiet {
                    println!("RenderWare Analysis: {}", result["file"]);
                    println!("Size: {} bytes", result["size"]);
                    println!("Format: {} ({})", result["format"], result["format_description"]);
                    println!("RenderWare Version: {}", result["rw_version"]);
                    println!("Total Chunks: {}", result["total_chunks"]);
                    println!("Max Depth: {}", result["max_depth"]);
                    println!("Analysis Time: {} ms", result["analysis_time_ms"]);

                    if let Some(warnings) = result.get("corruption_warnings") {
                        if let Some(warning_array) = warnings.as_array() {
                            if !warning_array.is_empty() {
                                println!("\nCorruption Warnings:");
                                for warning in warning_array {
                                    println!("  {}", warning);
                                }
                            }
                        }
                    }

                    println!("\nChunk Tree:");
                    self.print_chunk_tree(&result["chunk_tree"], 0);
                }
            }
        }

        Ok(0)
    }

    fn build_chunk_tree_summary(&self, node: &rengine_core::renderware::analyzer::ChunkNode, current_depth: usize, max_depth: usize) -> serde_json::Value {
        let mut result = serde_json::json!({
            "type": node.display_name,
            "type_id": node.header.chunk_type,
            "size": node.header.chunk_size,
            "version": format!("0x{:08X}", node.header.rw_version)
        });

        if node.is_corrupt {
            result["corrupt"] = serde_json::Value::Bool(true);
            if let Some(reason) = &node.corruption_reason {
                result["corruption_reason"] = serde_json::Value::String(reason.clone());
            }
        }

        if current_depth < max_depth && !node.children.is_empty() {
            let mut children = Vec::new();
            for child in &node.children {
                children.push(self.build_chunk_tree_summary(child, current_depth + 1, max_depth));
            }
            result["children"] = serde_json::Value::Array(children);
        } else if !node.children.is_empty() {
            result["children_count"] = serde_json::Value::Number(node.children.len().into());
        }

        result
    }

    fn print_chunk_tree(&self, tree: &serde_json::Value, depth: usize) {
        let indent = "  ".repeat(depth);

        if let Some(type_name) = tree.get("type") {
            let type_id = tree.get("type_id").and_then(|v| v.as_u64()).unwrap_or(0);
            let size = tree.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
            let version = tree.get("version").and_then(|v| v.as_str()).unwrap_or("unknown");

            print!("{}{} ({}) - {} bytes - {}", indent, type_name, type_id, size, version);

            if tree.get("corrupt").and_then(|v| v.as_bool()).unwrap_or(false) {
                print!(" [CORRUPT]");
                if let Some(reason) = tree.get("corruption_reason").and_then(|v| v.as_str()) {
                    print!(": {}", reason);
                }
            }

            println!();

            if let Some(children) = tree.get("children") {
                if let Some(children_array) = children.as_array() {
                    for child in children_array {
                        self.print_chunk_tree(child, depth + 1);
                    }
                }
            } else if let Some(count) = tree.get("children_count").and_then(|v| v.as_u64()) {
                if count > 0 {
                    println!("{}  ... and {} more children", indent, count);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rw_handler_creation() {
        let handler = RWHandler::new(false, false);
        assert!(!handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_rw_handler_with_verbose() {
        let handler = RWHandler::new(true, false);
        assert!(handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_rw_handler_quiet() {
        let handler = RWHandler::new(false, true);
        assert!(!handler.verbose);
        assert!(handler.quiet);
    }
}