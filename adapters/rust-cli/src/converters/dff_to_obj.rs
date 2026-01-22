use rengine_core::renderware::dff::DffModel;
use rengine_core::renderware::txd::TxdArchive;
use std::{fs, path::Path, collections::HashMap, sync::OnceLock};
use anyhow::{Result, anyhow};
use tracing::{info, error};
extern crate image;

// Import TxdToMaterialsConverter for shared texture decoding logic

pub struct DffToObjConverter {
    verbose: bool,
    quiet: bool,
}

// CSV cache for model data (similar to Python implementation)
static CSV_CACHE: OnceLock<HashMap<String, serde_json::Value>> = OnceLock::new();

impl DffToObjConverter {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    fn load_csv_cache() -> Result<()> {
        if CSV_CACHE.get().is_some() {
            return Ok(());
        }

        let csv_path = Path::new("/home/blefnk/Documents/reliverse/rengine/preloaded/gta-sa.csv");
        if !csv_path.exists() {
            info!("GTA SA CSV file not found at {}, continuing without CSV data", csv_path.display());
            CSV_CACHE.set(HashMap::new()).ok();
            return Ok(());
        }

        let mut cache = HashMap::new();
        let content = fs::read_to_string(csv_path).map_err(|e| anyhow!("Failed to read CSV file: {}", e))?;

        let mut reader = csv::Reader::from_reader(content.as_bytes());

        for result in reader.records() {
            if let Ok(record) = result {
                // CSV columns: id(0), radius(1), name(2), hasCollision(3), breaksOnHit(4),
                // visibleByTime(5), hasAnimation(6), borderBoxLength(7), borderBoxWidth(8),
                // borderBoxHeight(9), materialsFile(10), definitionFile(11), modelFile(12), tags(13)
                if let (Some(id_str), Some(model_name), Some(materials_file), Some(model_file)) = (
                    record.get(0), // id column
                    record.get(2), // name column
                    record.get(10), // materialsFile column
                    record.get(12), // modelFile column
                ) {
                    if let Ok(id) = id_str.parse::<i32>() {
                        let entry = serde_json::json!({
                            "id": id,
                            "model_name": model_name,
                            "materials_file": materials_file,
                            "model_file": model_file
                        });

                        // Store by model name
                        cache.insert(model_name.to_string(), entry.clone());
                        // Store by model file name (with .dff extension)
                        cache.insert(model_file.to_string(), entry.clone());
                        // Store by model file name without extension
                        if let Some(name_without_ext) = Path::new(model_file).file_stem().and_then(|s| s.to_str()) {
                            cache.insert(name_without_ext.to_string(), entry);
                        }
                    }
                }
            }
        }

        info!("Loaded {} models from GTA SA CSV", cache.len());
        CSV_CACHE.set(cache).map_err(|_| anyhow!("Failed to set CSV cache"))?;
        Ok(())
    }

    fn get_model_info_from_cache(dff_filename: &str) -> Option<serde_json::Value> {
        if let Some(cache) = CSV_CACHE.get() {
            // Try exact filename match first
            if let Some(info) = cache.get(dff_filename) {
                return Some(info.clone());
            }

            // Try model name match (without .dff extension)
            if let Some(model_name) = Path::new(dff_filename).file_stem().and_then(|s| s.to_str()) {
                if let Some(info) = cache.get(model_name) {
                    return Some(info.clone());
                }
            }
        }
        None
    }

    pub async fn convert(&self, file: &str, output: &str, txd: Option<&str>, use_csv: bool) -> Result<i32> {
        if self.verbose {
            info!("Converting DFF to OBJ: {} -> {} (use_csv: {})", file, output, use_csv);
        }

        // Load CSV cache if requested
        let csv_model_info = if use_csv {
            match Self::load_csv_cache() {
                Ok(_) => {
                    let filename = Path::new(file).file_name().and_then(|n| n.to_str()).unwrap_or(file);
                    let info = Self::get_model_info_from_cache(filename);
                    if self.verbose {
                        if let Some(ref i) = info {
                            info!("Found CSV info for {}: {:?}", filename, i);
                        } else {
                            info!("No CSV info found for {}", filename);
                        }
                    }
                    info
                }
                Err(e) => {
                    if self.verbose {
                        info!("Failed to load CSV cache: {}", e);
                    }
                    None
                }
            }
        } else {
            None
        };

        if self.verbose && use_csv {
            if let Some(ref info) = csv_model_info {
                info!("Found CSV data for model: {:?}", info);
            } else {
                info!("No CSV data found for model");
            }
        }

        // Determine TXD path - use provided TXD, or try to find from CSV data
        let mut final_txd_path: Option<String> = None;
        let mut auto_detected = false;

        if let Some(txd_path) = txd {
            final_txd_path = Some(txd_path.to_string());
        } else if use_csv && csv_model_info.is_some() {
            if let Some(ref info) = csv_model_info {
                if let Some(materials_file) = info.get("materials_file").and_then(|v| v.as_str()) {
                    if !materials_file.is_empty() {
                        // Try to find TXD file in the same directory as the DFF file
                        let dff_dir = Path::new(file).parent().unwrap_or(Path::new("."));
                        let txd_path = dff_dir.join(materials_file);
                        if txd_path.exists() {
                            final_txd_path = Some(txd_path.to_string_lossy().to_string());
                            auto_detected = true;
                        } else {
                            if self.verbose {
                                info!("TXD file from CSV not found: {}", txd_path.display());
                            }
                        }
                    }
                }
            }
        }

        if self.verbose {
            if let Some(ref txd_path) = final_txd_path {
                if auto_detected {
                    info!("Auto-detected TXD file: {}", txd_path);
                } else {
                    info!("Using provided TXD file: {}", txd_path);
                }
            } else {
                info!("No TXD file specified or found");
            }
        }

        // Load DFF model
        let model = match DffModel::load_from_path(file) {
            Ok(model) => model,
            Err(e) => {
                error!("Failed to load DFF file {}: {}", file, e);
                // Provide additional context for common errors
                let error_msg = match &e {
                    rengine_core::renderware::dff::DffError::Io(io_err) if io_err.kind() == std::io::ErrorKind::UnexpectedEof => {
                        format!("DFF file appears to be truncated or corrupted (unexpected end of file): {}", e)
                    }
                    rengine_core::renderware::dff::DffError::InvalidFormat(msg) if msg.contains("Expected CLUMP") => {
                        format!("File is not a valid DFF format or is severely corrupted: {}", e)
                    }
                    rengine_core::renderware::dff::DffError::InvalidFormat(msg) if msg.contains("truncated") || msg.contains("corrupted") => {
                        format!("DFF file appears to be corrupted or truncated: {}", e)
                    }
                    rengine_core::renderware::dff::DffError::InvalidFormat(msg) if msg.contains("not found") => {
                        format!("DFF file is missing required sections: {}", e)
                    }
                    _ => format!("Failed to load DFF file: {}", e)
                };
                return Err(anyhow!(error_msg));
            }
        };

        // Generate OBJ content
        let obj_content = self.generate_obj_content(&model)?;

        // Write OBJ file
        if let Err(e) = fs::write(output, &obj_content) {
            error!("Failed to write OBJ file {}: {}", output, e);
            return Err(anyhow!("Failed to write OBJ file: {}", e));
        }

        // Generate MTL file if TXD is provided or if CSV data suggests materials
        let mtl_result = self.generate_mtl_file(&model, output, final_txd_path.as_deref(), use_csv).await;

        if !self.quiet {
            println!("Converted {} to {}", file, output);
            if use_csv && csv_model_info.is_some() {
                println!("Used GTA SA CSV data for enhanced model information");
            }
            if auto_detected && final_txd_path.is_some() {
                if let Some(ref txd_path) = final_txd_path {
                    println!("Auto-detected TXD materials: {}", txd_path);
                }
            } else if !auto_detected && final_txd_path.is_some() {
                if let Some(ref txd_path) = final_txd_path {
                    println!("Used TXD materials from {}", txd_path);
                }
            }
            if let Ok(Some(mtl_file)) = &mtl_result {
                println!("Created material file: {}", mtl_file);
            }
        }

        Ok(0)
    }

    fn generate_obj_content(&self, model: &DffModel) -> Result<String> {
        let mut obj = String::new();

        // OBJ header
        obj.push_str("# Generated from DFF model\n");
        obj.push_str(&format!("# RW Version: 0x{:08X}\n", model.rw_version));
        obj.push_str(&format!("# Frames: {}, Geometries: {}\n", model.frames.len(), model.geometries.len()));

        let mut global_vertex_offset = 1; // OBJ uses 1-based indexing

        for (geom_idx, geometry) in model.geometries.iter().enumerate() {
            obj.push_str(&format!("\n# Geometry {}\n", geom_idx));
            obj.push_str(&format!("g geometry_{}\n", geom_idx));

            // Write vertices
            for vertex in &geometry.vertices {
                obj.push_str(&format!("v {:.6} {:.6} {:.6}\n",
                    vertex.x, vertex.y, vertex.z));
            }

            // Write normals (if available)
            if !geometry.normals.is_empty() && geometry.normals.len() == geometry.vertices.len() {
                for normal in &geometry.normals {
                    obj.push_str(&format!("vn {:.6} {:.6} {:.6}\n",
                        normal.x, normal.y, normal.z));
                }
            }

            // Write UV coordinates (if available)
            if !geometry.uv_layers.is_empty() && !geometry.uv_layers[0].is_empty() {
                let uv_layer = &geometry.uv_layers[0];
                for uv in uv_layer {
                    obj.push_str(&format!("vt {:.6} {:.6}\n", uv.u, 1.0 - uv.v)); // Flip V coordinate
                }
            }

            // Write faces (triangles)
            for (_face_idx, triangle) in geometry.triangles.iter().enumerate() {

                // OBJ face indices are 1-based
                let v1 = triangle.a as i32 + global_vertex_offset;
                let v2 = triangle.b as i32 + global_vertex_offset;
                let v3 = triangle.c as i32 + global_vertex_offset;

                if !geometry.uv_layers.is_empty() && !geometry.uv_layers[0].is_empty() {
                    // Include UV coordinates in faces
                    obj.push_str(&format!("f {}/{}/{} {}/{}/{} {}/{}/{}\n",
                        v1, v1, v1, v2, v2, v2, v3, v3, v3));
                } else {
                    // Simple vertex-only faces
                    obj.push_str(&format!("f {} {} {}\n", v1, v2, v3));
                }
            }

            global_vertex_offset += geometry.vertices.len() as i32;
        }

        Ok(obj)
    }

    async fn generate_mtl_file(&self, model: &DffModel, obj_path: &str, txd_path: Option<&str>, _use_csv: bool) -> Result<Option<String>> {
        let mut has_materials = false;
        let mut total_textures = 0;

        for geometry in &model.geometries {
            if !geometry.materials.is_empty() {
                has_materials = true;
                for material in &geometry.materials {
                    total_textures += material.textures.len();
                }
            }
        }

        if !has_materials {
            if self.verbose {
                info!("No materials found in DFF model, skipping MTL file generation");
            }
            return Ok(None);
        }

        if self.verbose {
            info!("Found {} materials with {} total textures in DFF model", model.geometries.iter().map(|g| g.materials.len()).sum::<usize>(), total_textures);
        }

        // Generate MTL filename from OBJ filename
        let obj_path_obj = Path::new(obj_path);
        let mtl_filename = obj_path_obj.with_extension("mtl");
        let mtl_path = mtl_filename.to_str().ok_or_else(|| anyhow!("Invalid MTL filename: {:?}", mtl_filename))?;

        let mut mtl_content = String::new();
        mtl_content.push_str("# Material definitions for DFF model\n");

        for (geom_idx, geometry) in model.geometries.iter().enumerate() {
            for (mat_idx, material) in geometry.materials.iter().enumerate() {
                let material_name = format!("material_{}_{}", geom_idx, mat_idx);
                mtl_content.push_str(&format!("\nnewmtl {}\n", material_name));

                // Diffuse color
                mtl_content.push_str(&format!("Kd {:.3} {:.3} {:.3}\n",
                    material.color.r as f32 / 255.0,
                    material.color.g as f32 / 255.0,
                    material.color.b as f32 / 255.0));

                // Specular color
                mtl_content.push_str(&format!("Ks {:.3} {:.3} {:.3}\n",
                    material.surface_properties.specular,
                    material.surface_properties.specular,
                    material.surface_properties.specular));

                // Ambient color (same as diffuse for now)
                mtl_content.push_str(&format!("Ka {:.3} {:.3} {:.3}\n",
                    material.color.r as f32 / 255.0,
                    material.color.g as f32 / 255.0,
                    material.color.b as f32 / 255.0));

                // Diffuse texture
                if !material.textures.is_empty() {
                    let texture_name = &material.textures[0].name;
                    mtl_content.push_str(&format!("map_Kd {}.png\n", texture_name));

                    if self.verbose {
                        info!("Material {}_{} references texture: {}", geom_idx, mat_idx, texture_name);
                    }

                    // Extract actual texture from TXD file if path provided
                    if let Some(txd_path) = txd_path {
                        match self.extract_texture_from_txd(txd_path, texture_name, obj_path).await {
                            Ok(_) => {
                                if self.verbose {
                                    info!("Extracted texture: {} from TXD", texture_name);
                                }
                                if !self.quiet {
                                    println!("Extracted texture: {}.png", texture_name);
                                }
                            }
                            Err(e) => {
                                error!("Failed to extract texture {} from TXD {}: {}", texture_name, txd_path, e);
                                if !self.quiet {
                                    println!("Warning: Failed to extract texture {}: {}", texture_name, e);
                                }
                            }
                        }
                    } else {
                        if self.verbose {
                            info!("TXD file not provided, skipping texture extraction for {}", texture_name);
                        }
                        if !self.quiet {
                            println!("Note: TXD file not provided, texture {} referenced but not extracted", texture_name);
                        }
                    }
                }
            }
        }

        // Write MTL file
        fs::write(mtl_path, &mtl_content)?;

        // Add MTL reference to OBJ file
        if let Ok(mut obj_content) = fs::read_to_string(obj_path) {
            let mtl_filename_only = Path::new(mtl_path).file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow!("Invalid MTL filename for OBJ reference"))?;
            let mtl_ref = format!("\nmtllib {}\n", mtl_filename_only);
            obj_content.insert_str(0, &mtl_ref);
            fs::write(obj_path, obj_content)?;
        }

        Ok(Some(mtl_path.to_string()))
    }

    async fn extract_texture_from_txd(&self, txd_path: &str, texture_name: &str, obj_path: &str) -> Result<()> {
        if self.verbose {
            info!("Loading TXD archive: {}", txd_path);
        }

        // Load TXD archive
        let archive = TxdArchive::load_from_path(txd_path)?;

        if self.verbose {
            info!("TXD archive loaded with {} textures", archive.textures.len());
        }

        // Find the texture
        let texture = archive.textures.iter()
            .find(|t| t.name == texture_name)
            .ok_or_else(|| {
                let available_textures: Vec<&str> = archive.textures.iter().map(|t| t.name.as_str()).collect();
                anyhow!("Texture '{}' not found in TXD archive. Available textures: {:?}", texture_name, available_textures)
            })?;

        if self.verbose {
            info!("Found texture '{}' in TXD: {}x{} format {:?}", texture.name, texture.width, texture.height, texture.format);
        }

        // Get texture data
        let texture_data = archive.get_texture_data(texture_name)?;

        if self.verbose {
            info!("Retrieved {} bytes of texture data", texture_data.len());
        }

        // Decode texture data using the texture's to_rgba method
        let rgba_data = texture.to_rgba(&archive, 0)?;

        if self.verbose {
            info!("Decoded texture to {} bytes of RGBA data", rgba_data.len());
        }

        // Create image buffer
        let img_buffer = image::RgbaImage::from_raw(texture.width as u32, texture.height as u32, rgba_data)
            .ok_or_else(|| anyhow!("Failed to create image buffer from {}x{} texture", texture.width, texture.height))?;

        // Save as PNG in the same directory as the OBJ file
        let obj_dir = Path::new(obj_path).parent().unwrap_or(Path::new("."));
        let texture_path = obj_dir.join(format!("{}.png", texture_name));

        if self.verbose {
            info!("Saving texture to: {}", texture_path.display());
        }

        img_buffer.save_with_format(&texture_path, image::ImageFormat::Png)?;

        if self.verbose {
            info!("Successfully saved texture: {}", texture_path.display());
        }

        Ok(())
    }





}