use rengine_core::renderware::ipl::IPLFile;
use std::fs;
use anyhow::{Result, anyhow};
use tracing::{info, error};

use crate::OutputFormat;

pub struct IPLHandler {
    verbose: bool,
    quiet: bool,
}

impl IPLHandler {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub async fn info(&self, file_path: &str, detailed: bool, format: OutputFormat) -> Result<i32> {
        if self.verbose {
            info!("Loading IPL file: {}", file_path);
        }

        let ipl_file = match IPLFile::load_from_path(file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to load IPL file {}: {}", file_path, e);
                return Err(anyhow!("Failed to load IPL file: {}", e));
            }
        };

        let file_size = fs::metadata(file_path)?.len();

        let mut info = serde_json::json!({
            "file": file_path,
            "size": file_size,
            "instances": ipl_file.instances.len(),
            "zones": ipl_file.zones.len(),
            "culls": ipl_file.culls.len(),
            "picks": ipl_file.picks.len()
        });

        if detailed {
            // Count unique model names
            let mut model_names = std::collections::HashSet::new();
            for instance in &ipl_file.instances {
                model_names.insert(&instance.model_name);
            }

            info["unique_models"] = serde_json::Value::Number(model_names.len().into());

            // Add section details
            let mut section_counts = serde_json::json!({});
            section_counts["instances"] = serde_json::Value::Number(ipl_file.instances.len().into());
            section_counts["zones"] = serde_json::Value::Number(ipl_file.zones.len().into());
            section_counts["culls"] = serde_json::Value::Number(ipl_file.culls.len().into());
            section_counts["picks"] = serde_json::Value::Number(ipl_file.picks.len().into());

            info["section_counts"] = section_counts;
        }

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&info)?);
            }
            OutputFormat::Text => {
                if !self.quiet {
                    println!("IPL File: {}", info["file"]);
                    println!("Size: {} bytes", info["size"]);
                    println!("Instances: {}", info["instances"]);
                    println!("Zones: {}", info["zones"]);
                    println!("Culls: {}", info["culls"]);
                    println!("Picks: {}", info["picks"]);

                    if detailed {
                        if let Some(unique_models) = info["unique_models"].as_u64() {
                            println!("Unique Models: {}", unique_models);
                        }

                        if let Some(sections) = info.get("section_counts") {
                            println!("\nSection Details:");
                            if let Some(obj) = sections.as_object() {
                                for (section, count) in obj {
                                    println!("  {}: {}", section, count);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(0)
    }

    pub async fn analyze(&self, file_path: &str, format: OutputFormat, max_entries: Option<usize>) -> Result<i32> {
        if self.verbose {
            info!("Analyzing IPL file: {}", file_path);
        }

        let ipl_file = match IPLFile::load_from_path(file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to load IPL file {}: {}", file_path, e);
                return Err(anyhow!("Failed to load IPL file: {}", e));
            }
        };

        let file_size = fs::metadata(file_path)?.len();

        let mut analysis = serde_json::json!({
            "file": file_path,
            "size": file_size,
            "sections": {}
        });

        // Analyze each section
        let mut sections = serde_json::json!({});

        // Instances section
        if !ipl_file.instances.is_empty() {
            let mut instance_analysis = serde_json::json!({
                "count": ipl_file.instances.len(),
                "sample_entries": Vec::<serde_json::Value>::new()
            });

            // Show first 5 instances as samples
            for (_i, instance) in ipl_file.instances.iter().enumerate().take(max_entries.unwrap_or(5)) {
                instance_analysis["sample_entries"].as_array_mut().unwrap().push(serde_json::json!({
                    "id": instance.id,
                    "model": instance.model_name,
                    "position": format!("{:.2},{:.2},{:.2}",
                        instance.position.x, instance.position.y, instance.position.z)
                }));
            }

            let max_instances = max_entries.unwrap_or(5);
            if ipl_file.instances.len() > max_instances {
                instance_analysis["note"] = serde_json::Value::String(
                    format!("... and {} more instances", ipl_file.instances.len() - max_instances)
                );
            }

            sections["inst"] = instance_analysis;
        }

        // Zones section
        if !ipl_file.zones.is_empty() {
            sections["zone"] = serde_json::json!({
                "count": ipl_file.zones.len(),
                "sample_entries": ipl_file.zones.iter().take(3).map(|zone| {
                    serde_json::json!({
                        "name": zone.name,
                        "island": zone.island,
                        "type": zone.zone_type
                    })
                }).collect::<Vec<_>>()
            });
        }

        // Culls section
        if !ipl_file.culls.is_empty() {
            sections["cull"] = serde_json::json!({
                "count": ipl_file.culls.len(),
                "sample_entries": ipl_file.culls.iter().take(3).map(|cull| {
                    serde_json::json!({
                        "center": format!("{:.2},{:.2},{:.2}",
                            cull.center.x, cull.center.y, cull.center.z),
                        "name": cull.name
                    })
                }).collect::<Vec<_>>()
            });
        }

        analysis["sections"] = sections;

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&analysis)?);
            }
            OutputFormat::Text => {
                if !self.quiet {
                    println!("IPL Analysis: {}", analysis["file"]);
                    println!("Size: {} bytes", analysis["size"]);

                    if let Some(sections) = analysis.get("sections") {
                        if let Some(sections_obj) = sections.as_object() {
                            for (section_name, section_data) in sections_obj {
                                let section_name_upper = section_name.to_uppercase();
                                if let Some(count) = section_data.get("count") {
                                    println!("\n{} Section ({} entries):", section_name_upper, count);

                                    if let Some(samples) = section_data.get("sample_entries") {
                                        if let Some(sample_array) = samples.as_array() {
                                            for sample in sample_array {
                                                if section_name == "inst" {
                                                    println!("  {}: {} at {}",
                                                        sample["id"], sample["model"], sample["position"]);
                                                } else if section_name == "zone" {
                                                    println!("  {} (island: {}, type: {})",
                                                        sample["name"], sample["island"], sample["type"]);
                                                } else if section_name == "cull" {
                                                    println!("  {} at {}", sample["name"], sample["center"]);
                                                }
                                            }
                                        }
                                    }

                                    if let Some(count_val) = count.as_u64() {
                                        if count_val > 3 && section_name != "inst" {
                                            println!("  ... and {} more entries", count_val - 3);
                                        } else if count_val > 5 && section_name == "inst" {
                                            println!("  ... and {} more entries", count_val - 5);
                                        }
                                    }
                                }
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
    async fn test_ipl_handler_creation() {
        let handler = IPLHandler::new(false, false);
        assert!(!handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_ipl_handler_with_verbose() {
        let handler = IPLHandler::new(true, false);
        assert!(handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_ipl_handler_quiet() {
        let handler = IPLHandler::new(false, true);
        assert!(!handler.verbose);
        assert!(handler.quiet);
    }
}