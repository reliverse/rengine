use rengine_core::renderware::dff::DffModel;
use std::fs;
use anyhow::{Result, anyhow};
use tracing::{info, error};

use crate::OutputFormat;

pub struct DFFHandler {
    verbose: bool,
    quiet: bool,
}

impl DFFHandler {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub async fn info(&self, file_path: &str, detailed: bool, format: OutputFormat) -> Result<i32> {
        if self.verbose {
            info!("Loading DFF file: {}", file_path);
        }

        let model = match DffModel::load_from_path(file_path) {
            Ok(model) => model,
            Err(e) => {
                error!("Failed to load DFF file {}: {}", file_path, e);
                return Err(anyhow!("Failed to load DFF file: {}", e));
            }
        };

        let file_size = fs::metadata(file_path)?.len();

        let mut info = serde_json::json!({
            "file": file_path,
            "size": file_size,
            "rw_version": format!("0x{:08X}", model.rw_version),
            "frames": model.frames.len(),
            "geometries": model.geometries.len(),
            "atomics": model.atomics.len(),
            "uv_animations": model.uv_animations.len()
        });

        if detailed {
            // Add detailed geometry information
            let mut geometry_info = Vec::new();
            for (i, geom) in model.geometries.iter().enumerate() {
                geometry_info.push(serde_json::json!({
                    "index": i,
                    "vertices": geom.vertices.len(),
                    "triangles": geom.triangles.len(),
                    "materials": geom.materials.len(),
                    "uv_layers": geom.uv_layers.len(),
                    "has_normals": !geom.normals.is_empty(),
                    "has_native_geometry": geom.native_geometry.is_some()
                }));
            }
            info["geometry_details"] = serde_json::Value::Array(geometry_info);

            // Add material summary
            let mut material_summary = serde_json::json!({
                "total_materials": 0,
                "materials_with_textures": 0,
                "materials_with_effects": 0
            });

            let mut total_materials = 0;
            let mut materials_with_textures = 0;
            let mut materials_with_effects = 0;

            for geom in &model.geometries {
                total_materials += geom.materials.len();
                for material in &geom.materials {
                    if !material.textures.is_empty() {
                        materials_with_textures += 1;
                    }
                    if !material.effects.is_empty() {
                        materials_with_effects += 1;
                    }
                }
            }

            material_summary["total_materials"] = serde_json::Value::Number(total_materials.into());
            material_summary["materials_with_textures"] = serde_json::Value::Number(materials_with_textures.into());
            material_summary["materials_with_effects"] = serde_json::Value::Number(materials_with_effects.into());
            info["material_summary"] = material_summary;
        }

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&info)?);
            }
            OutputFormat::Text => {
                if !self.quiet {
                    println!("DFF File: {}", info["file"]);
                    println!("Size: {} bytes", info["size"]);
                    println!("RenderWare Version: {}", info["rw_version"]);
                    println!("Frames: {}", info["frames"]);
                    println!("Geometries: {}", info["geometries"]);
                    println!("Atomics: {}", info["atomics"]);
                    println!("UV Animations: {}", info["uv_animations"]);

                    if detailed {
                        if let Some(geometry_details) = info.get("geometry_details") {
                            if let Some(geoms) = geometry_details.as_array() {
                                println!("\nGeometry Details:");
                                for geom in geoms {
                                    let index = geom["index"].as_u64().unwrap_or(0);
                                    let vertices = geom["vertices"].as_u64().unwrap_or(0);
                                    let triangles = geom["triangles"].as_u64().unwrap_or(0);
                                    let materials = geom["materials"].as_u64().unwrap_or(0);
                                    println!("  Geometry {}: {} vertices, {} triangles, {} materials",
                                            index, vertices, triangles, materials);
                                }
                            }
                        }

                        if let Some(material_summary) = info.get("material_summary") {
                            println!("\nMaterial Summary:");
                            println!("  Total Materials: {}", material_summary["total_materials"]);
                            println!("  Materials with Textures: {}", material_summary["materials_with_textures"]);
                            println!("  Materials with Effects: {}", material_summary["materials_with_effects"]);
                        }
                    }
                }
            }
        }

        Ok(0)
    }

    pub async fn analyze(&self, file_path: &str, format: OutputFormat) -> Result<i32> {
        let model = DffModel::load_from_path(file_path)?;

        let file_size = fs::metadata(file_path)?.len();

        let mut analysis = serde_json::json!({
            "file": file_path,
            "size": file_size,
            "rw_version": format!("0x{:08X}", model.rw_version),
            "chunks": Vec::<serde_json::Value>::new()
        });

        // Analyze chunks structure (simplified for now)
        let mut chunks = Vec::new();

        // Add frame list chunk info
        if !model.frames.is_empty() {
            chunks.push(serde_json::json!({
                "type": "Frame List",
                "type_id": 14,
                "count": model.frames.len(),
                "description": "Contains frame hierarchy and transformations"
            }));
        }

        // Add geometry list chunk info
        if !model.geometries.is_empty() {
            chunks.push(serde_json::json!({
                "type": "Geometry List",
                "type_id": 26,
                "count": model.geometries.len(),
                "description": "Contains geometry definitions"
            }));
        }

        // Add atomic chunk info
        if !model.atomics.is_empty() {
            chunks.push(serde_json::json!({
                "type": "Atomic",
                "type_id": 20,
                "count": model.atomics.len(),
                "description": "Links frames to geometries"
            }));
        }

        analysis["chunks"] = serde_json::Value::Array(chunks);

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&analysis)?);
            }
            OutputFormat::Text => {
                if !self.quiet {
                    println!("DFF Analysis: {}", analysis["file"]);
                    println!("Size: {} bytes", analysis["size"]);
                    println!("RenderWare Version: {}", analysis["rw_version"]);

                    if let Some(chunks) = analysis.get("chunks") {
                        if let Some(chunk_list) = chunks.as_array() {
                            println!("\nChunk Structure:");
                            for chunk in chunk_list {
                                let chunk_type = chunk["type"].as_str().unwrap_or("Unknown");
                                let type_id = chunk["type_id"].as_u64().unwrap_or(0);
                                let count = chunk["count"].as_u64().unwrap_or(0);
                                let description = chunk["description"].as_str().unwrap_or("");
                                println!("  {} ({}) - {} items - {}", chunk_type, type_id, count, description);
                            }
                        }
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
    async fn test_dff_handler_creation() {
        let handler = DFFHandler::new(false, false);
        assert!(!handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_dff_handler_with_verbose() {
        let handler = DFFHandler::new(true, false);
        assert!(handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_dff_handler_quiet() {
        let handler = DFFHandler::new(false, true);
        assert!(!handler.verbose);
        assert!(handler.quiet);
    }
}