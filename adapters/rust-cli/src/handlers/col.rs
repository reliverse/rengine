use rengine_core::renderware::col::ColFile;
use std::fs;
use anyhow::{Result, anyhow};
use tracing::{info, error};

use crate::OutputFormat;

pub struct COLHandler {
    verbose: bool,
    quiet: bool,
}

impl COLHandler {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub async fn info(&self, file_path: &str, detailed: bool, format: OutputFormat) -> Result<i32> {
        if self.verbose {
            info!("Loading COL file: {}", file_path);
        }

        let col_file = match ColFile::load_from_path(file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to load COL file {}: {}", file_path, e);
                return Err(anyhow!("Failed to load COL file: {}", e));
            }
        };

        let file_size = fs::metadata(file_path)?.len();

        let mut info = serde_json::json!({
            "file": file_path,
            "size": file_size,
            "version": format!("{:?}", col_file.version),
            "models": col_file.models.len()
        });

        if detailed {
            // Count collision primitives
            let mut total_spheres = 0;
            let mut total_boxes = 0;
            let mut total_faces = 0;

            for model in &col_file.models {
                total_spheres += model.spheres.len();
                total_boxes += model.boxes.len();
                total_faces += model.faces.len();
            }

            info["spheres"] = serde_json::Value::Number(total_spheres.into());
            info["boxes"] = serde_json::Value::Number(total_boxes.into());
            info["faces"] = serde_json::Value::Number(total_faces.into());

            // Add model details
            let mut model_details = Vec::new();
            for (i, model) in col_file.models.iter().enumerate() {
                model_details.push(serde_json::json!({
                    "index": i,
                    "id": model.model_id,
                    "name": model.model_name,
                    "spheres": model.spheres.len(),
                    "boxes": model.boxes.len(),
                    "faces": model.faces.len(),
                    "vertices": model.vertices.len()
                }));
            }
            info["model_details"] = serde_json::Value::Array(model_details);
        }

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&info)?);
            }
            OutputFormat::Text => {
                if !self.quiet {
                    println!("COL File: {}", info["file"]);
                    println!("Size: {} bytes", info["size"]);
                    println!("Version: {}", info["version"]);
                    println!("Models: {}", info["models"]);

                    if detailed {
                        if let Some(spheres) = info["spheres"].as_u64() {
                            println!("Spheres: {}", spheres);
                        }
                        if let Some(boxes) = info["boxes"].as_u64() {
                            println!("Boxes: {}", boxes);
                        }
                        if let Some(faces) = info["faces"].as_u64() {
                            println!("Faces: {}", faces);
                        }

                        if let Some(model_details) = info.get("model_details") {
                            if let Some(models) = model_details.as_array() {
                                println!("\nModel Details:");
                                for model in models {
                                    let index = model["index"].as_u64().unwrap_or(0);
                                    let name = model["name"].as_str().unwrap_or("unknown");
                                    let spheres = model["spheres"].as_u64().unwrap_or(0);
                                    let boxes = model["boxes"].as_u64().unwrap_or(0);
                                    let faces = model["faces"].as_u64().unwrap_or(0);
                                    println!("  Model {} ({}): {} spheres, {} boxes, {} faces",
                                            index, name, spheres, boxes, faces);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(0)
    }

    pub async fn analyze(&self, file_path: &str, format: OutputFormat) -> Result<i32> {
        if self.verbose {
            info!("Analyzing COL file: {}", file_path);
        }

        let col_file = match ColFile::load_from_path(file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to load COL file {}: {}", file_path, e);
                return Err(anyhow!("Failed to load COL file: {}", e));
            }
        };

        let file_size = fs::metadata(file_path)?.len();

        let mut analysis = serde_json::json!({
            "file": file_path,
            "size": file_size,
            "version": format!("{:?}", col_file.version),
            "models": col_file.models.len(),
            "sections": {
                "collision_models": col_file.models.len()
            }
        });

        // Analyze model complexity
        let mut complexity_stats = serde_json::json!({
            "simple_models": 0,
            "medium_models": 0,
            "complex_models": 0
        });

        for model in &col_file.models {
            let total_primitives = model.spheres.len() + model.boxes.len() + model.faces.len();
            if total_primitives <= 5 {
                complexity_stats["simple_models"] = serde_json::Value::Number(
                    (complexity_stats["simple_models"].as_u64().unwrap_or(0) + 1).into()
                );
            } else if total_primitives <= 20 {
                complexity_stats["medium_models"] = serde_json::Value::Number(
                    (complexity_stats["medium_models"].as_u64().unwrap_or(0) + 1).into()
                );
            } else {
                complexity_stats["complex_models"] = serde_json::Value::Number(
                    (complexity_stats["complex_models"].as_u64().unwrap_or(0) + 1).into()
                );
            }
        }

        analysis["complexity_stats"] = complexity_stats;

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&analysis)?);
            }
            OutputFormat::Text => {
                if !self.quiet {
                    println!("COL Analysis: {}", analysis["file"]);
                    println!("Size: {} bytes", analysis["size"]);
                    println!("Version: {}", analysis["version"]);
                    println!("Models: {}", analysis["models"]);

                    if let Some(complexity) = analysis.get("complexity_stats") {
                        println!("\nComplexity Statistics:");
                        println!("  Simple models (≤5 primitives): {}", complexity["simple_models"]);
                        println!("  Medium models (6-20 primitives): {}", complexity["medium_models"]);
                        println!("  Complex models (>20 primitives): {}", complexity["complex_models"]);
                    }
                }
            }
        }

        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_col_handler_creation() {
        let handler = COLHandler::new(false, false);
        assert!(!handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_col_handler_with_verbose() {
        let handler = COLHandler::new(true, false);
        assert!(handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_col_handler_quiet() {
        let handler = COLHandler::new(false, true);
        assert!(!handler.verbose);
        assert!(handler.quiet);
    }
}