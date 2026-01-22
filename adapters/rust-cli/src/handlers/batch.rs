use rengine_core::renderware::{img::ImgArchive, dff::DffModel, col::ColFile, ipl::IPLFile, versions::RenderWareVersionManager};
use walkdir::WalkDir;
use std::{fs, path::Path};
use anyhow::Result;
use tracing::{info, warn, error};

use crate::BatchFormat;

pub struct BatchHandler {
    verbose: bool,
    quiet: bool,
}

impl BatchHandler {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub async fn analyze(&self, directory: &str, format: BatchFormat, recursive: bool, output: Option<&str>, max_files: Option<usize>) -> Result<i32> {
        if self.verbose {
            info!("Starting batch analysis of directory: {}", directory);
        }

        let files = self.collect_files(directory, recursive, max_files)?;

        if self.verbose {
            info!("Found {} files to analyze", files.len());
        }

        let mut results = serde_json::json!({
            "directory": directory,
            "format": format!("{:?}", format).to_lowercase(),
            "recursive": recursive,
            "total_files": files.len(),
            "analyzed_files": Vec::<serde_json::Value>::new(),
            "errors": Vec::<serde_json::Value>::new()
        });

        let version_manager = RenderWareVersionManager::new();

        for (i, file_path) in files.iter().enumerate() {
            if !self.quiet && (i % 10 == 0 || self.verbose) {
                info!("Analyzing file {}/{}: {}", i + 1, files.len(), file_path);
            }

            let file_info = self.analyze_single_file(file_path, &format, &version_manager).await;
            match file_info {
                Ok(info) => {
                    if let Some(analyzed) = results.get_mut("analyzed_files") {
                        if let Some(arr) = analyzed.as_array_mut() {
                            arr.push(info);
                        }
                    }
                }
                Err(e) => {
                    if let Some(errors) = results.get_mut("errors") {
                        if let Some(arr) = errors.as_array_mut() {
                            arr.push(serde_json::json!({
                                "file": file_path,
                                "error": e.to_string()
                            }));
                        }
                    }
                    if self.verbose {
                        warn!("Error analyzing {}: {}", file_path, e);
                    }
                }
            }
        }

        // Save to output file if specified
        if let Some(output_path) = output {
            fs::write(output_path, serde_json::to_string_pretty(&results)?)?;
            if !self.quiet {
                println!("Results saved to {}", output_path);
            }
        } else {
            // Print JSON to stdout
            println!("{}", serde_json::to_string_pretty(&results)?);
        }

        Ok(0)
    }

    pub async fn extract_imgs(&self, directory: &str, output: &str, recursive: bool) -> Result<i32> {
        if self.verbose {
            info!("Starting batch IMG extraction from directory: {}", directory);
        }

        let img_files = self.collect_files_by_extension(directory, recursive, "img")?;

        if img_files.is_empty() {
            if !self.quiet {
                println!("No IMG files found in {}", directory);
            }
            return Ok(0);
        }

        if self.verbose {
            info!("Found {} IMG files to extract", img_files.len());
        }

        fs::create_dir_all(output)?;
        let mut total_extracted = 0;

        for (i, img_path) in img_files.iter().enumerate() {
            if !self.quiet {
                info!("Extracting IMG {}/{}: {}", i + 1, img_files.len(), img_path);
            }

            let img_name = Path::new(img_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            let img_output_dir = Path::new(output).join(img_name);

            match ImgArchive::load_from_path(img_path) {
                Ok(archive) => {
                    let mut extracted = 0;
                    for entry in &archive.entries {
                        let output_path = img_output_dir.join(&entry.name);

                        // Always overwrite existing files

                        match archive.extract_entry(&entry.name, output_path.to_str().unwrap()) {
                            Ok(_) => {
                                extracted += 1;
                                if self.verbose {
                                    info!("Extracted: {}", entry.name);
                                }
                            }
                            Err(e) => {
                                error!("Failed to extract {}: {}", entry.name, e);
                            }
                        }
                    }

                    total_extracted += extracted;
                    if !self.quiet {
                        println!("Extracted {} files from {}", extracted, img_path);
                    }
                }
                Err(e) => {
                    error!("Failed to load IMG {}: {}", img_path, e);
                }
            }
        }

        if !self.quiet {
            println!("Total extracted: {} files from {} IMG archives", total_extracted, img_files.len());
        }

        Ok(0)
    }

    fn collect_files(&self, directory: &str, recursive: bool, max_files: Option<usize>) -> Result<Vec<String>> {
        let mut files = Vec::new();

        let walker = if recursive {
            WalkDir::new(directory).into_iter()
        } else {
            walkdir::WalkDir::new(directory).max_depth(1).into_iter()
        };

        for entry in walker {
            let entry = entry?;
            if entry.file_type().is_file() {
                if let Some(path_str) = entry.path().to_str() {
                    files.push(path_str.to_string());

                    if let Some(max) = max_files {
                        if files.len() >= max {
                            break;
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    fn collect_files_by_extension(&self, directory: &str, recursive: bool, extension: &str) -> Result<Vec<String>> {
        let all_files = self.collect_files(directory, recursive, None)?;
        Ok(all_files.into_iter()
            .filter(|path| path.to_lowercase().ends_with(&format!(".{}", extension)))
            .collect())
    }

    async fn analyze_single_file(&self, file_path: &str, format: &BatchFormat, version_manager: &RenderWareVersionManager) -> Result<serde_json::Value> {
        let metadata = fs::metadata(file_path)?;
        let file_size = metadata.len();

        let mut file_info = serde_json::json!({
            "path": file_path,
            "size": file_size,
            "type": self.get_file_type(file_path)
        });

        match format {
            BatchFormat::All => {
                // Try to analyze based on file extension
                let ext = self.get_file_type(file_path).to_lowercase();
                match ext.as_str() {
                    "dff" => self.analyze_dff_file(file_path, &mut file_info)?,
                    "txd" => self.analyze_txd_file(file_path, &mut file_info)?,
                    "col" => self.analyze_col_file(file_path, &mut file_info)?,
                    "ipl" => self.analyze_ipl_file(file_path, &mut file_info)?,
                    "img" => self.analyze_img_file(file_path, &mut file_info)?,
                    _ => {
                        // Try RenderWare analysis for unknown files
                        if let Ok(data) = fs::read(file_path) {
                            if let Ok((file_type, version_str, _)) = version_manager.detect_file_format_version(&data, file_path) {
                                file_info["rw_format"] = serde_json::Value::String(file_type);
                                file_info["rw_version"] = serde_json::Value::String(version_str);
                            }
                        }
                    }
                }
            }
            BatchFormat::Dff => self.analyze_dff_file(file_path, &mut file_info)?,
            BatchFormat::Txd => self.analyze_txd_file(file_path, &mut file_info)?,
            BatchFormat::Col => self.analyze_col_file(file_path, &mut file_info)?,
            BatchFormat::Ipl => self.analyze_ipl_file(file_path, &mut file_info)?,
            BatchFormat::Img => self.analyze_img_file(file_path, &mut file_info)?,
        }

        Ok(file_info)
    }

    fn get_file_type(&self, file_path: &str) -> String {
        Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_uppercase()
    }

    fn analyze_dff_file(&self, file_path: &str, file_info: &mut serde_json::Value) -> Result<()> {
        let model = DffModel::load_from_path(file_path)?;
        file_info["frames"] = serde_json::Value::Number(model.frames.len().into());
        file_info["geometries"] = serde_json::Value::Number(model.geometries.len().into());
        file_info["atomics"] = serde_json::Value::Number(model.atomics.len().into());
        file_info["rw_version"] = serde_json::Value::String(format!("0x{:08X}", model.rw_version));
        Ok(())
    }

    fn analyze_txd_file(&self, _file_path: &str, file_info: &mut serde_json::Value) -> Result<()> {
        // For now, just mark as TXD - full analysis would require texture parsing
        file_info["analyzed"] = serde_json::Value::Bool(true);
        Ok(())
    }

    fn analyze_col_file(&self, file_path: &str, file_info: &mut serde_json::Value) -> Result<()> {
        let col_file = ColFile::load_from_path(file_path)?;
        file_info["models"] = serde_json::Value::Number(col_file.models.len().into());
        file_info["version"] = serde_json::Value::String(format!("{:?}", col_file.version));
        Ok(())
    }

    fn analyze_ipl_file(&self, file_path: &str, file_info: &mut serde_json::Value) -> Result<()> {
        let ipl_file = IPLFile::load_from_path(file_path)?;
        file_info["instances"] = serde_json::Value::Number(ipl_file.instances.len().into());
        file_info["zones"] = serde_json::Value::Number(ipl_file.zones.len().into());
        file_info["culls"] = serde_json::Value::Number(ipl_file.culls.len().into());
        file_info["picks"] = serde_json::Value::Number(ipl_file.picks.len().into());
        Ok(())
    }

    fn analyze_img_file(&self, file_path: &str, file_info: &mut serde_json::Value) -> Result<()> {
        let archive = ImgArchive::load_from_path(file_path)?;
        file_info["entries"] = serde_json::Value::Number(archive.total_entries.into());
        file_info["version"] = serde_json::Value::String(format!("{:?}", archive.version));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_batch_handler_creation() {
        let handler = BatchHandler::new(false, false);
        assert!(!handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_batch_handler_with_verbose() {
        let handler = BatchHandler::new(true, false);
        assert!(handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_batch_handler_quiet() {
        let handler = BatchHandler::new(false, true);
        assert!(!handler.verbose);
        assert!(handler.quiet);
    }

    #[test]
    fn test_get_file_type() {
        let handler = BatchHandler::new(false, false);
        assert_eq!(handler.get_file_type("test.dff"), "DFF");
        assert_eq!(handler.get_file_type("test.TXD"), "TXD");
        assert_eq!(handler.get_file_type("test.col"), "COL");
        assert_eq!(handler.get_file_type("test.ipl"), "IPL");
        assert_eq!(handler.get_file_type("test.img"), "IMG");
        assert_eq!(handler.get_file_type("test.unknown"), "UNKNOWN");
        assert_eq!(handler.get_file_type("noextension"), "UNKNOWN");
    }
}