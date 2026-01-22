use rengine_core::renderware::{img::{ImgArchive, ImgVersion}, versions::RenderWareVersionManager};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use tracing::{info, error};
use serde_json;

use crate::OutputFormat;

pub struct IMGHandler {
    verbose: bool,
    quiet: bool,
}

impl IMGHandler {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub async fn info(&self, file_path: &str, detailed: bool, format: OutputFormat) -> Result<i32> {
        let archive = ImgArchive::load_from_path(file_path)?;

        let mut info = HashMap::new();
        info.insert("file".to_string(), file_path.to_string());
        info.insert("version".to_string(), match archive.version {
            ImgVersion::V1 => "VER1".to_string(),
            ImgVersion::V2 => "VER2".to_string(),
        });
        info.insert("file_count".to_string(), archive.total_entries.to_string());
        info.insert("size".to_string(), archive.file_size.to_string());

        if detailed {
            // Analyze file types and RW versions
            let mut file_types = HashMap::new();
            let mut rw_versions = HashMap::new();

            // Create version manager for RW version detection
            let version_manager = RenderWareVersionManager::new();

            for entry in &archive.entries {
                // Count file types
                if let Some(ref ext) = entry.file_type {
                    *file_types.entry(ext.clone()).or_insert(0) += 1;
                }

                // Try to detect RW version
                if let Ok(data) = archive.get_entry_data(&entry.name) {
                    if let Ok((file_type, version_str, _version_num)) =
                        version_manager.detect_file_format_version(&data, &entry.name)
                    {
                        if file_type != "Unknown" {
                            *rw_versions.entry(format!("{} ({})", file_type, version_str)).or_insert(0) += 1;
                        }
                    }
                }
            }

            info.insert("file_types".to_string(), format!("{:?}", file_types));
            info.insert("rw_versions".to_string(), format!("{:?}", rw_versions));
        }

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&info)?);
            }
            OutputFormat::Text => {
                if !self.quiet {
                    println!("IMG Archive: {}", info["file"]);
                    println!("Version: {}", info["version"]);
                    println!("Files: {}", info["file_count"]);
                    println!("Size: {} bytes", info["size"]);

                    if detailed {
                        if let Some(file_types_str) = info.get("file_types") {
                            println!("\nFile Types:");
                            println!("  {}", file_types_str);
                        }

                        if let Some(rw_versions_str) = info.get("rw_versions") {
                            println!("\nRenderWare Versions:");
                            println!("  {}", rw_versions_str);
                        }
                    }
                }
            }
        }

        Ok(0)
    }

    pub async fn list(&self, file_path: &str, filter: Option<&str>, format: OutputFormat) -> Result<i32> {
        let archive = ImgArchive::load_from_path(file_path)?;
        let mut filtered_entries = Vec::new();

        for entry in &archive.entries {
            let include = if let Some(filter_str) = filter {
                entry.name.to_lowercase().contains(&filter_str.to_lowercase())
            } else {
                true
            };

            if include {
                filtered_entries.push(serde_json::json!({
                    "name": entry.name,
                    "size": (entry.size as u64) * 2048, // Convert sectors to bytes
                    "offset": (entry.offset as u64) * 2048, // Convert sectors to bytes
                    "type": entry.file_type,
                    "rw_version": entry.renderware_version
                }));
            }
        }

        let result = serde_json::json!({
            "file": file_path,
            "total_entries": archive.total_entries,
            "filtered_entries": filtered_entries.len(),
            "entries": filtered_entries
        });

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
            OutputFormat::Text => {
                if !self.quiet {
                    println!("Files in {} ({} entries):", result["file"], result["filtered_entries"]);
                    println!("{:-<60}", "");

                    if let Some(entries) = result["entries"].as_array() {
                        for entry in entries {
                            let name = entry["name"].as_str().unwrap_or("unknown");
                            let size = entry["size"].as_u64().unwrap_or(0);
                            println!("  {:<50} {:>10} bytes", name, size);
                        }
                    }
                }
            }
        }

        Ok(0)
    }

    pub async fn extract(&self, file_path: &str, files: Option<&Vec<String>>, output_dir: &str) -> Result<i32> {
        let archive = ImgArchive::load_from_path(file_path)?;

        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)?;

        let files_to_extract: Vec<&String> = if let Some(specific_files) = files {
            specific_files.iter().collect()
        } else {
            // Extract all files
            archive.entries.iter().map(|e| &e.name).collect()
        };

        let mut extracted = 0;

        for file_name in files_to_extract {
            let output_path = Path::new(output_dir).join(file_name);

            // Always overwrite existing files

            match archive.extract_entry(file_name, output_path.to_str().unwrap()) {
                Ok(_) => {
                    if self.verbose {
                        info!("Extracted: {}", file_name);
                    }
                    extracted += 1;
                }
                Err(e) => {
                    error!("Failed to extract {}: {}", file_name, e);
                    continue;
                }
            }
        }

        if !self.quiet {
            println!("\nExtracted {} files to {}", extracted, output_dir);
        }

        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_img_handler_creation() {
        let handler = IMGHandler::new(false, false);
        assert!(!handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_img_handler_with_verbose() {
        let handler = IMGHandler::new(true, false);
        assert!(handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_img_handler_quiet() {
        let handler = IMGHandler::new(false, true);
        assert!(!handler.verbose);
        assert!(handler.quiet);
    }

    // Integration test - requires actual IMG file
    // This would be run separately with real test data
    // #[tokio::test]
    // async fn test_img_info_with_real_file() {
    //     let handler = IMGHandler::new(true, false);
    //     // This would need a real IMG file path
    //     // let result = handler.info("path/to/test.img", false, OutputFormat::Text).await;
    //     // assert!(result.is_ok());
    // }
}