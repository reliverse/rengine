// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod blueprint;
pub mod models;
pub mod renderware;
pub mod config;

use crate::renderware::img::ImgArchive;
use base64::{engine::general_purpose, Engine as _};
#[allow(unused_imports)]
use lazy_static::lazy_static;
use photon_rs::native::{open_image, save_image};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::fs::read_dir;
use std::io::Read;
use thiserror::Error;
// Cross-platform permission handling
fn get_permission_number(permissions: &fs::Permissions) -> u32 {
    #[cfg(windows)]
    {
        // Windows doesn't have Unix-style permissions, so we provide basic read/write info
        // Return a reasonable default that indicates the file is accessible
        0o644 // rw-r--r--
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        permissions.mode()
    }
    #[cfg(not(any(windows, unix)))]
    {
        // Fallback for other platforms
        0o644
    }
}

use std::path::Path;

// Structured error types for better error handling
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum RengineError {
    #[error("Failed to read file {path}: {details}")]
    FileReadFailed { path: String, details: String },

    #[error("Failed to write file {path}: {details}")]
    FileWriteFailed { path: String, details: String },

    #[error("Failed to parse {path}: {details}")]
    ParseError { path: String, details: String },

    #[error("Directory does not exist: {path}")]
    DirectoryNotFound { path: String },

    #[error("Path is not a directory: {path}")]
    PathNotDirectory { path: String },

    #[error("Failed to read directory {path}: {details}")]
    DirectoryReadFailed { path: String, details: String },

    #[error("Clipboard operation failed")]
    ClipboardFailed,

    #[error("Invalid image file: {path}")]
    InvalidImageFile { path: String },

    #[error("Unsupported image format: {format}")]
    UnsupportedImageFormat { format: String },
}

// File system entry models
#[derive(Serialize, Deserialize, Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub size: Option<u64>,
    pub modified_at: Option<String>,
    pub created_at: Option<String>,
    pub permissions: Option<String>,
    pub extension: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryContents {
    pub directories: Vec<FileEntry>,
    pub files: Vec<FileEntry>,
}

impl From<std::io::Error> for RengineError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => RengineError::FileReadFailed {
                path: "unknown".to_string(),
                details: err.to_string(),
            },
            std::io::ErrorKind::PermissionDenied => RengineError::FileReadFailed {
                path: "unknown".to_string(),
                details: format!("Permission denied: {}", err.to_string()),
            },
            _ => RengineError::FileReadFailed {
                path: "unknown".to_string(),
                details: err.to_string(),
            },
        }
    }
}

// Logging helper
#[macro_export]
macro_rules! log_error {
    ($err:expr) => {
        eprintln!("[ERROR] {}", $err);
    };
    ($err:expr, $($arg:tt)*) => {
        eprintln!("[ERROR] {}", format!($err, $($arg)*));
    };
}

#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {
        println!("[INFO] {}", $msg);
    };
    ($msg:expr, $($arg:tt)*) => {
        println!("[INFO] {}", format!($msg, $($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr) => {
        println!("[WARN] {}", $msg);
    };
    ($msg:expr, $($arg:tt)*) => {
        println!("[WARN] {}", format!($msg, $($arg)*));
    };
}

// Global SA:MP model database - loaded once at startup
lazy_static::lazy_static! {
    static ref SAMP_MODEL_DATABASE: crate::models::SampModelDatabase = {
        match crate::models::SampModelDatabase::load_from_embedded_csv() {
            Ok(db) => {
                // println!("Loaded {} SA:MP models from embedded CSV", db.len());
                db
            }
            Err(e) => {
                eprintln!("Failed to load SA:MP model database: {}", e);
                crate::models::SampModelDatabase::new()
            }
        }
    };
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn copy_to_clipboard(text: String) -> Result<(), String> {
    log_info!(
        "Copy to clipboard requested for text of length: {}",
        text.len()
    );

    // Note: This requires tauri-plugin-clipboard-manager
    // For now, we'll use a simple approach - the frontend can use the Clipboard API
    // This command is a placeholder - implement with clipboard plugin if needed
    log_warn!("Clipboard functionality not implemented - frontend should handle this");
    Err(RengineError::ClipboardFailed.to_string())
}

#[tauri::command]
async fn read_directory(path: String, show_hidden: bool) -> Result<DirectoryContents, String> {
    let path_obj = Path::new(&path);

    // Check if path exists
    if !path_obj.exists() {
        log_error!("Directory does not exist: {}", path);
        return Err(RengineError::DirectoryNotFound { path }.to_string());
    }

    // Check if path is a directory
    if !path_obj.is_dir() {
        log_error!("Path is not a directory: {}", path);
        return Err(RengineError::PathNotDirectory { path }.to_string());
    }

    let mut directories = Vec::new();
    let mut files = Vec::new();

    let entries = read_dir(path_obj).map_err(|err| {
        log_error!("Failed to read directory: {}", err);
        RengineError::DirectoryReadFailed {
            path: path.clone(),
            details: err.to_string(),
        }
        .to_string()
    })?;

    for entry in entries {
        let entry = entry.map_err(|err| {
            log_error!("Failed to read entry: {}", err);
            RengineError::DirectoryReadFailed {
                path: path.clone(),
                details: err.to_string(),
            }
            .to_string()
        })?;

        let file_name = entry.file_name().to_string_lossy().to_string();
        let entry_path = entry.path();
        let file_type = entry.file_type().map_err(|err| {
            log_error!("Failed to get file type: {}", err);
            RengineError::DirectoryReadFailed {
                path: path.clone(),
                details: err.to_string(),
            }
            .to_string()
        })?;

        // Skip hidden files if not showing them
        if !show_hidden && file_name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata().map_err(|err| {
            log_error!("Failed to get metadata: {}", err);
            RengineError::DirectoryReadFailed {
                path: path.clone(),
                details: err.to_string(),
            }
            .to_string()
        })?;

        let extension = entry_path
            .extension()
            .map(|ext| ext.to_string_lossy().to_string());

        let file_entry = FileEntry {
            name: file_name,
            path: entry_path.to_string_lossy().to_string(),
            is_directory: file_type.is_dir(),
            is_file: file_type.is_file(),
            is_symlink: file_type.is_symlink(),
            size: Some(metadata.len()),
            modified_at: metadata.modified().ok().map(|t| {
                let duration = t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                // Return as milliseconds since epoch for JavaScript compatibility
                format!("{}", duration.as_millis())
            }),
            created_at: metadata.created().ok().map(|t| {
                let duration = t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                // Return as milliseconds since epoch for JavaScript compatibility
                format!("{}", duration.as_millis())
            }),
            permissions: Some(format!(
                "{:o}",
                get_permission_number(&metadata.permissions())
            )),
            extension,
        };

        if file_type.is_dir() {
            directories.push(file_entry);
        } else if file_type.is_file() {
            files.push(file_entry);
        }
    }

    Ok(DirectoryContents { directories, files })
}

#[tauri::command]
async fn create_directory(path: String) -> Result<(), String> {
    let path_obj = Path::new(&path);

    fs::create_dir_all(path_obj).map_err(|err| {
        log_error!("Failed to create directory {}: {}", path, err);
        RengineError::FileWriteFailed {
            path,
            details: err.to_string(),
        }
        .to_string()
    })?;

    Ok(())
}

#[tauri::command]
async fn create_file(path: String) -> Result<(), String> {
    let path_obj = Path::new(&path);

    // Create parent directories if they don't exist
    if let Some(parent) = path_obj.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            log_error!("Failed to create parent directories for {}: {}", path, err);
            RengineError::FileWriteFailed {
                path: path.clone(),
                details: err.to_string(),
            }
            .to_string()
        })?;
    }

    fs::File::create(path_obj).map_err(|err| {
        log_error!("Failed to create file {}: {}", path, err);
        RengineError::FileWriteFailed {
            path,
            details: err.to_string(),
        }
        .to_string()
    })?;

    Ok(())
}

#[tauri::command]
async fn delete_file(path: String) -> Result<(), String> {
    let path_obj = Path::new(&path);

    if path_obj.is_dir() {
        fs::remove_dir_all(path_obj).map_err(|err| {
            log_error!("Failed to delete directory {}: {}", path, err);
            RengineError::FileWriteFailed {
                path,
                details: err.to_string(),
            }
            .to_string()
        })?;
    } else {
        fs::remove_file(path_obj).map_err(|err| {
            log_error!("Failed to delete file {}: {}", path, err);
            RengineError::FileWriteFailed {
                path,
                details: err.to_string(),
            }
            .to_string()
        })?;
    }

    Ok(())
}

#[tauri::command]
async fn rename_file(old_path: String, new_path: String) -> Result<(), String> {
    fs::rename(&old_path, &new_path).map_err(|err| {
        log_error!("Failed to rename {} to {}: {}", old_path, new_path, err);
        RengineError::FileWriteFailed {
            path: format!("{} -> {}", old_path, new_path),
            details: err.to_string(),
        }
        .to_string()
    })?;

    Ok(())
}

#[tauri::command]
async fn copy_file(source_path: String, destination_path: String) -> Result<(), String> {
    fs::copy(&source_path, &destination_path).map_err(|err| {
        log_error!(
            "Failed to copy {} to {}: {}",
            source_path,
            destination_path,
            err
        );
        RengineError::FileWriteFailed {
            path: format!("{} -> {}", source_path, destination_path),
            details: err.to_string(),
        }
        .to_string()
    })?;

    Ok(())
}

#[tauri::command]
async fn write_file(file_path: String, content: String) -> Result<(), String> {
    fs::write(&file_path, &content).map_err(|err| {
        log_error!("Failed to write to file {}: {}", file_path, err);
        RengineError::FileWriteFailed {
            path: file_path,
            details: err.to_string(),
        }
        .to_string()
    })?;

    Ok(())
}

#[tauri::command]
async fn read_file(file_path: String) -> Result<String, String> {
    fs::read_to_string(&file_path).map_err(|err| {
        log_error!("Failed to read file {}: {}", file_path, err);
        RengineError::FileReadFailed {
            path: file_path,
            details: err.to_string(),
        }
        .to_string()
    })
}

#[tauri::command]
async fn write_log_file(content: String, path: String) -> Result<(), String> {
    use std::io::Write;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|err| {
            log_error!("Failed to open log file {}: {}", path, err);
            format!("Failed to open log file: {}", err)
        })?;

    file.write_all(content.as_bytes()).map_err(|err| {
        log_error!("Failed to write to log file {}: {}", path, err);
        format!("Failed to write to log file: {}", err)
    })?;

    file.flush().map_err(|err| {
        log_error!("Failed to flush log file {}: {}", path, err);
        format!("Failed to flush log file: {}", err)
    })?;

    Ok(())
}

// Texture-specific commands
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub size_bytes: u64,
}

#[tauri::command]
async fn read_image_as_base64(file_path: String) -> Result<String, String> {
    // Read the file as bytes
    let mut file = fs::File::open(&file_path).map_err(|err| {
        log_error!("Failed to open image file {}: {}", file_path, err);
        RengineError::FileReadFailed {
            path: file_path.clone(),
            details: err.to_string(),
        }
        .to_string()
    })?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|err| {
        log_error!("Failed to read image file {}: {}", file_path, err);
        RengineError::FileReadFailed {
            path: file_path.clone(),
            details: err.to_string(),
        }
        .to_string()
    })?;

    // Convert to base64
    let base64_string = general_purpose::STANDARD.encode(&buffer);

    // Detect MIME type based on file extension
    let mime_type = match Path::new(&file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "tiff" | "tif" => "image/tiff",
        "tga" => "image/x-tga",
        "dds" => "image/vnd-ms.dds",
        "hdr" => "image/vnd.radiance",
        "exr" => "image/x-exr",
        _ => "application/octet-stream",
    };

    Ok(format!("data:{};base64,{}", mime_type, base64_string))
}

#[tauri::command]
async fn get_image_metadata(file_path: String) -> Result<ImageMetadata, String> {
    // For basic metadata, we can try to read the file and get basic info
    // For full image metadata (width, height), we'd need an image processing library
    // For now, return basic file info

    let metadata = fs::metadata(&file_path).map_err(|err| {
        log_error!("Failed to get metadata for {}: {}", file_path, err);
        RengineError::FileReadFailed {
            path: file_path.clone(),
            details: err.to_string(),
        }
        .to_string()
    })?;

    // Detect format from extension
    let format = Path::new(&file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_uppercase();

    // Try to open the image with photon-rs to get actual dimensions
    let (width, height) = match open_image(&file_path) {
        Ok(photon_img) => {
            let w = photon_img.get_width() as u32;
            let h = photon_img.get_height() as u32;
            (w, h)
        }
        Err(err) => {
            log_warn!(
                "Failed to open image with photon-rs, using fallback: {}",
                err
            );
            // Fallback to basic metadata if photon-rs can't read the file
            (0, 0)
        }
    };

    Ok(ImageMetadata {
        width,
        height,
        format,
        size_bytes: metadata.len(),
    })
}

#[tauri::command]
async fn save_texture_to_file(
    file_path: String,
    base64_data: String,
    _format: String,
) -> Result<(), String> {
    // Remove data URL prefix if present
    let base64_clean = if base64_data.starts_with("data:") {
        base64_data.split(',').nth(1).unwrap_or(&base64_data)
    } else {
        &base64_data
    };

    // Decode base64
    let image_data = general_purpose::STANDARD
        .decode(base64_clean)
        .map_err(|err| {
            log_error!("Failed to decode base64 data: {}", err);
            format!("Failed to decode base64 data: {}", err)
        })?;

    // Ensure parent directory exists
    if let Some(parent) = Path::new(&file_path).parent() {
        fs::create_dir_all(parent).map_err(|err| {
            log_error!(
                "Failed to create parent directories for {}: {}",
                file_path,
                err
            );
            RengineError::FileWriteFailed {
                path: file_path.clone(),
                details: err.to_string(),
            }
            .to_string()
        })?;
    }

    // Write file
    fs::write(&file_path, &image_data).map_err(|err| {
        log_error!("Failed to write texture to {}: {}", file_path, err);
        RengineError::FileWriteFailed {
            path: file_path.clone(),
            details: err.to_string(),
        }
        .to_string()
    })?;

    log_info!("Successfully saved texture to {}", file_path.as_str());
    Ok(())
}

#[tauri::command]
async fn batch_process_textures(
    operations: Vec<TextureOperation>,
) -> Result<Vec<TextureOperationResult>, String> {
    let mut results = Vec::new();

    for operation in operations {
        let result = match operation.operation_type.as_str() {
            "copy" => {
                if let (Some(source), Some(destination)) =
                    (&operation.source_path, &operation.destination_path)
                {
                    match fs::copy(source, destination) {
                        Ok(_) => TextureOperationResult {
                            operation_id: operation.id,
                            success: true,
                            error: None,
                        },
                        Err(err) => TextureOperationResult {
                            operation_id: operation.id,
                            success: false,
                            error: Some(err.to_string()),
                        },
                    }
                } else {
                    TextureOperationResult {
                        operation_id: operation.id,
                        success: false,
                        error: Some("Missing source or destination path".to_string()),
                    }
                }
            }
            "delete" => {
                if let Some(path) = &operation.source_path {
                    match fs::remove_file(path) {
                        Ok(_) => TextureOperationResult {
                            operation_id: operation.id,
                            success: true,
                            error: None,
                        },
                        Err(err) => TextureOperationResult {
                            operation_id: operation.id,
                            success: false,
                            error: Some(err.to_string()),
                        },
                    }
                } else {
                    TextureOperationResult {
                        operation_id: operation.id,
                        success: false,
                        error: Some("Missing source path".to_string()),
                    }
                }
            }
            "rename" => {
                if let (Some(source), Some(destination)) =
                    (&operation.source_path, &operation.destination_path)
                {
                    match fs::rename(source, destination) {
                        Ok(_) => TextureOperationResult {
                            operation_id: operation.id,
                            success: true,
                            error: None,
                        },
                        Err(err) => TextureOperationResult {
                            operation_id: operation.id,
                            success: false,
                            error: Some(err.to_string()),
                        },
                    }
                } else {
                    TextureOperationResult {
                        operation_id: operation.id,
                        success: false,
                        error: Some("Missing source or destination path".to_string()),
                    }
                }
            }
            _ => TextureOperationResult {
                operation_id: operation.id,
                success: false,
                error: Some(format!(
                    "Unknown operation type: {}",
                    operation.operation_type
                )),
            },
        };

        results.push(result);
    }

    Ok(results)
}

#[tauri::command]
async fn process_image(
    input_path: String,
    output_path: String,
    operations: Vec<ImageOperation>,
) -> Result<ProcessedImageMetadata, String> {
    // Load the image
    let mut img = open_image(&input_path).map_err(|err| {
        log_error!("Failed to open image {}: {}", input_path, err);
        format!("Failed to open image: {}", err)
    })?;

    // Apply operations
    for operation in operations {
        match operation.operation_type.as_str() {
            "resize" => {
                if let (Some(width), Some(height)) = (operation.width, operation.height) {
                    photon_rs::transform::resize(
                        &mut img,
                        width,
                        height,
                        photon_rs::transform::SamplingFilter::Lanczos3,
                    );
                    log_info!("Resized image to {}x{}", width, height);
                }
            }
            "filter" => {
                if let Some(filter_name) = &operation.filter_name {
                    match filter_name.as_str() {
                        "oceanic" => photon_rs::filters::filter(&mut img, "oceanic"),
                        "islands" => photon_rs::filters::filter(&mut img, "islands"),
                        "marine" => photon_rs::filters::filter(&mut img, "marine"),
                        "seagreen" => photon_rs::filters::filter(&mut img, "seagreen"),
                        "flagblue" => photon_rs::filters::filter(&mut img, "flagblue"),
                        "liquid" => photon_rs::filters::filter(&mut img, "liquid"),
                        "diamante" => photon_rs::filters::filter(&mut img, "diamante"),
                        "radio" => photon_rs::filters::filter(&mut img, "radio"),
                        "twenties" => photon_rs::filters::filter(&mut img, "twenties"),
                        "rosetint" => photon_rs::filters::filter(&mut img, "rosetint"),
                        "mauve" => photon_rs::filters::filter(&mut img, "mauve"),
                        "bluechrome" => photon_rs::filters::filter(&mut img, "bluechrome"),
                        "vintage" => photon_rs::filters::filter(&mut img, "vintage"),
                        "perfume" => photon_rs::filters::filter(&mut img, "perfume"),
                        "serenity" => photon_rs::filters::filter(&mut img, "serenity"),
                        _ => {
                            log_warn!("Unknown filter: {}", filter_name);
                        }
                    }
                }
            }
            "effect" => {
                if let Some(effect_name) = &operation.effect_name {
                    match effect_name.as_str() {
                        "solarize" => photon_rs::effects::solarize(&mut img),
                        "invert" => photon_rs::channels::invert(&mut img),
                        "grayscale" => photon_rs::monochrome::grayscale(&mut img),
                        "sepia" => photon_rs::monochrome::sepia(&mut img),
                        _ => {
                            log_warn!("Unknown effect: {}", effect_name);
                        }
                    }
                }
            }
            "adjust" => {
                if let Some(brightness) = operation.brightness {
                    photon_rs::colour_spaces::lighten_hsl(&mut img, brightness as f32);
                }
                if let Some(saturation) = operation.saturation {
                    photon_rs::colour_spaces::saturate_hsl(&mut img, saturation as f32);
                }
            }
            _ => {
                log_warn!("Unknown operation type: {}", operation.operation_type);
            }
        }
    }

    // Save the processed image
    save_image(img, &output_path).map_err(|err| {
        log_error!("Failed to save processed image to {}: {}", output_path, err);
        format!("Failed to save processed image: {}", err)
    })?;

    // Get final metadata
    let final_img = open_image(&output_path).map_err(|err| {
        log_error!("Failed to reopen processed image {}: {}", output_path, err);
        format!("Failed to reopen processed image: {}", err)
    })?;

    let final_metadata = fs::metadata(&output_path).map_err(|err| {
        log_error!(
            "Failed to get metadata for processed image {}: {}",
            output_path,
            err
        );
        format!("Failed to get metadata: {}", err)
    })?;

    // Convert to base64 for frontend
    let mut file = fs::File::open(&output_path).map_err(|err| {
        log_error!(
            "Failed to open processed image for base64 conversion {}: {}",
            output_path,
            err
        );
        format!("Failed to open processed image: {}", err)
    })?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|err| {
        log_error!("Failed to read processed image {}: {}", output_path, err);
        format!("Failed to read processed image: {}", err)
    })?;

    let base64_data = general_purpose::STANDARD.encode(&buffer);
    let mime_type = match Path::new(&output_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    };

    Ok(ProcessedImageMetadata {
        width: final_img.get_width() as u32,
        height: final_img.get_height() as u32,
        format: mime_type.to_string(),
        size_bytes: final_metadata.len(),
        base64_data: format!("data:{};base64,{}", mime_type, base64_data),
    })
}

#[tauri::command]
async fn get_supported_filters() -> Result<Vec<String>, String> {
    Ok(vec![
        "oceanic".to_string(),
        "islands".to_string(),
        "marine".to_string(),
        "seagreen".to_string(),
        "flagblue".to_string(),
        "liquid".to_string(),
        "diamante".to_string(),
        "radio".to_string(),
        "twenties".to_string(),
        "rosetint".to_string(),
        "mauve".to_string(),
        "bluechrome".to_string(),
        "vintage".to_string(),
        "perfume".to_string(),
        "serenity".to_string(),
    ])
}

#[tauri::command]
async fn get_supported_effects() -> Result<Vec<String>, String> {
    Ok(vec![
        "solarize".to_string(),
        "invert".to_string(),
        "grayscale".to_string(),
        "sepia".to_string(),
    ])
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageOperation {
    pub operation_type: String, // "resize", "filter", "effect", "adjust"
    // Resize options
    pub width: Option<u32>,
    pub height: Option<u32>,
    // Filter options
    pub filter_name: Option<String>,
    // Effect options
    pub effect_name: Option<String>,
    // Adjustment options
    pub brightness: Option<i32>, // -100 to 100
    pub contrast: Option<i32>,   // -100 to 100
    pub saturation: Option<i32>, // -100 to 100
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextureOperation {
    pub id: String,
    pub operation_type: String, // "copy", "delete", "rename"
    pub source_path: Option<String>,
    pub destination_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextureOperationResult {
    pub operation_id: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessedImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub size_bytes: u64,
    pub base64_data: String,
}

// IMG Archive Commands
#[tauri::command]
async fn load_img_archive(path: String) -> Result<crate::renderware::img::ImgArchive, String> {
    use crate::renderware::img::ImgArchive;
    use crate::renderware::versions::RenderWareVersionManager;

    match ImgArchive::load_from_path(&path) {
        Ok(mut archive) => {
            // Analyze RenderWare versions for all entries
            let version_manager = RenderWareVersionManager::new();
            archive.analyze_renderware_versions(&version_manager);
            Ok(archive)
        }
        Err(e) => Err(format!("Failed to load IMG archive: {}", e)),
    }
}

#[tauri::command]
async fn extract_img_entry(
    archive_path: String,
    entry_name: String,
    output_path: String,
) -> Result<(), String> {
    use crate::renderware::img::ImgArchive;

    match ImgArchive::load_from_path(&archive_path) {
        Ok(archive) => match archive.extract_entry(&entry_name, &output_path) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Failed to extract entry '{}': {}", entry_name, e)),
        },
        Err(e) => Err(format!("Failed to load IMG archive: {}", e)),
    }
}

#[derive(serde::Deserialize)]
struct BatchExtractRequest {
    archive_path: String,
    operations: Vec<ImgOperation>,
}

#[derive(serde::Deserialize)]
struct ImgOperation {
    entry_name: String,
    output_path: String,
}

#[tauri::command]
async fn batch_extract_img_entries(
    request: BatchExtractRequest,
) -> Result<Vec<crate::renderware::img::OperationResult>, String> {
    use crate::renderware::img::{ImgArchive, OperationResult};

    let mut results = Vec::new();

    match ImgArchive::load_from_path(&request.archive_path) {
        Ok(archive) => {
            for operation in &request.operations {
                let result =
                    match archive.extract_entry(&operation.entry_name, &operation.output_path) {
                        Ok(()) => OperationResult {
                            entry_name: operation.entry_name.clone(),
                            success: true,
                            error: None,
                        },
                        Err(e) => OperationResult {
                            entry_name: operation.entry_name.clone(),
                            success: false,
                            error: Some(format!("Failed to extract entry: {}", e)),
                        },
                    };
                results.push(result);
            }
        }
        Err(e) => {
            // If archive fails to load, mark all operations as failed
            for operation in &request.operations {
                results.push(OperationResult {
                    entry_name: operation.entry_name.clone(),
                    success: false,
                    error: Some(format!("Failed to load archive: {}", e)),
                });
            }
        }
    }

    Ok(results)
}

#[derive(serde::Deserialize)]
struct ImportViaIdeRequest {
    img_archive_path: String,
    ide_file_path: String,
    models_directory: Option<String>,
}

#[derive(serde::Serialize)]
struct ImportViaIdeResult {
    imported_entries: Vec<String>,
    failed_files: Vec<String>,
    parsed_info: serde_json::Value,
}

#[derive(serde::Serialize)]
struct MemoryStats {
    process_memory_mb: f64,
    system_memory_used_mb: u64,
    system_memory_total_mb: u64,
    system_memory_percentage: f32,
    timestamp: String,
}

// IDE-based importing functionality
#[tauri::command]
async fn import_via_ide(request: ImportViaIdeRequest) -> Result<ImportViaIdeResult, String> {
    use crate::renderware::img::ImgArchive;
    use std::path::Path;

    // Load the IMG archive
    let mut archive = match ImgArchive::load_from_path(&request.img_archive_path) {
        Ok(archive) => archive,
        Err(e) => return Err(format!("Failed to load IMG archive: {}", e)),
    };

    // Parse IDE file to extract model and texture references
    let (models, textures): (HashSet<String>, HashSet<String>) =
        match parse_ide_file(&request.ide_file_path) {
            Ok(result) => result,
            Err(e) => return Err(format!("Failed to parse IDE file: {}", e)),
        };

    if models.is_empty() && textures.is_empty() {
        return Ok(ImportViaIdeResult {
            imported_entries: vec![],
            failed_files: vec![],
            parsed_info: serde_json::json!({
                "objs_count": 0,
                "tobj_count": 0,
                "unique_models": 0,
                "unique_textures": 0,
                "found_models": [],
                "found_textures": [],
                "missing_models": [],
                "missing_textures": []
            }),
        });
    }

    // Determine models directory
    let models_directory = request.models_directory.unwrap_or_else(|| {
        Path::new(&request.ide_file_path)
            .parent()
            .unwrap_or(Path::new("."))
            .to_string_lossy()
            .to_string()
    });

    let mut imported_entries = Vec::new();
    let mut failed_files = Vec::new();
    let mut found_models = Vec::new();
    let mut found_textures = Vec::new();
    let mut missing_models = Vec::new();
    let mut missing_textures = Vec::new();

    // Import DFF files
    for model_name in &models {
        let dff_path = find_file_in_directory(&models_directory, &format!("{}.dff", model_name));
        if let Some(path) = dff_path {
            found_models.push(model_name.to_string());
            match import_file_to_archive(&mut archive, &path, &format!("{}.dff", model_name)) {
                Ok(_) => {
                    imported_entries.push(format!("{}.dff", model_name));
                }
                Err(e) => {
                    failed_files.push(path);
                    log_warn!("Failed to import model {}: {}", model_name, e);
                }
            }
        } else {
            missing_models.push(model_name.clone());
        }
    }

    // Import TXD files
    for texture_name in &textures {
        let txd_path = find_file_in_directory(&models_directory, &format!("{}.txd", texture_name));
        if let Some(path) = txd_path {
            found_textures.push(texture_name.to_string());
            match import_file_to_archive(&mut archive, &path, &format!("{}.txd", texture_name)) {
                Ok(_) => {
                    imported_entries.push(format!("{}.txd", texture_name));
                }
                Err(e) => {
                    failed_files.push(path);
                    log_warn!("Failed to import texture {}: {}", texture_name, e);
                }
            }
        } else {
            missing_textures.push(texture_name.clone());
        }
    }

    let parsed_info = serde_json::json!({
        "objs_count": models.len(),
        "tobj_count": textures.len(),
        "unique_models": models.len(),
        "unique_textures": textures.len(),
        "found_models": found_models,
        "found_textures": found_textures,
        "missing_models": missing_models,
        "missing_textures": missing_textures
    });

    Ok(ImportViaIdeResult {
        imported_entries,
        failed_files,
        parsed_info,
    })
}

#[tauri::command]
async fn delete_img_entry(archive_path: String, entry_name: String) -> Result<(), String> {
    use crate::renderware::img::ImgArchive;

    // For now, we'll need to load, modify, and save the archive
    // In a full implementation, this would work with in-memory archives
    match ImgArchive::load_from_path(&archive_path) {
        Ok(mut archive) => {
            match archive.delete_entry(&entry_name) {
                Ok(()) => {
                    // In a real implementation, we'd save the archive back
                    // For now, just return success
                    Ok(())
                }
                Err(e) => Err(format!("Failed to delete entry '{}': {}", entry_name, e)),
            }
        }
        Err(e) => Err(format!("Failed to load IMG archive: {}", e)),
    }
}

#[tauri::command]
async fn restore_img_entry(archive_path: String, entry_name: String) -> Result<(), String> {
    use crate::renderware::img::ImgArchive;

    match ImgArchive::load_from_path(&archive_path) {
        Ok(mut archive) => match archive.restore_entry(&entry_name) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Failed to restore entry '{}': {}", entry_name, e)),
        },
        Err(e) => Err(format!("Failed to load IMG archive: {}", e)),
    }
}

#[tauri::command]
async fn get_img_modification_info(archive_path: String) -> Result<serde_json::Value, String> {
    use crate::renderware::img::ImgArchive;

    match ImgArchive::load_from_path(&archive_path) {
        Ok(archive) => Ok(archive.get_modification_info()),
        Err(e) => Err(format!("Failed to load IMG archive: {}", e)),
    }
}

#[tauri::command]
async fn get_memory_stats() -> Result<MemoryStats, String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Get current timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // For now, return basic stats
    // In a full implementation, you'd use system APIs to get actual memory stats
    let process_memory_mb = 0.0; // Placeholder
    let system_memory_used_mb = 0; // Placeholder
    let system_memory_total_mb = 0; // Placeholder
    let system_memory_percentage = 0.0; // Placeholder

    Ok(MemoryStats {
        process_memory_mb,
        system_memory_used_mb,
        system_memory_total_mb,
        system_memory_percentage,
        timestamp: timestamp.to_string(),
    })
}

#[tauri::command]
async fn get_home_directory() -> Result<String, String> {
    match std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
        Ok(home) => Ok(home),
        Err(_) => Ok("/".to_string()), // Fallback to root
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PwnObjectData {
    pub modelid: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rx: f32,
    pub ry: f32,
    pub rz: f32,
    pub worldid: Option<i32>,
    pub interiorid: Option<i32>,
    pub playerid: Option<i32>,
    pub streamdistance: Option<f32>,
    pub drawdistance: Option<f32>,
    pub areaid: Option<i32>,
    pub priority: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PwnImportResult {
    pub objects: Vec<PwnObjectData>,
    pub line_count: usize,
    pub parsed_count: usize,
    pub errors: Vec<String>,
}

#[tauri::command]
async fn import_dff_file(file_path: String) -> Result<crate::renderware::dff::DffModel, String> {
    use crate::renderware::dff::DffModel;

    match DffModel::load_from_path(&file_path) {
        Ok(model) => Ok(model),
        Err(e) => Err(format!("Failed to import DFF file: {}", e)),
    }
}

#[tauri::command]
async fn import_txd_file(file_path: String) -> Result<crate::renderware::txd::TxdArchive, String> {
    use crate::renderware::txd::TxdArchive;

    match TxdArchive::load_from_path(&file_path) {
        Ok(archive) => Ok(archive),
        Err(e) => Err(format!("Failed to import TXD file: {}", e)),
    }
}

#[tauri::command]
async fn import_col_file(file_path: String) -> Result<crate::renderware::col::ColFile, String> {
    use crate::renderware::col::ColFile;

    match ColFile::load_from_path(&file_path) {
        Ok(col_file) => Ok(col_file),
        Err(e) => Err(format!("Failed to import COL file: {}", e)),
    }
}

#[tauri::command]
async fn import_ipl_file(file_path: String) -> Result<crate::renderware::ipl::IPLFile, String> {
    use crate::renderware::ipl::IPLFile;

    match IPLFile::load_from_path(&file_path) {
        Ok(ipl_file) => Ok(ipl_file),
        Err(e) => Err(format!("Failed to import IPL file: {}", e)),
    }
}

#[tauri::command]
async fn parse_pwn_file(file_path: String) -> Result<PwnImportResult, String> {
    use regex::Regex;
    use std::fs::File;
    use std::io::Read;

    // Read the PWN file
    let mut file = File::open(&file_path)
        .map_err(|e| format!("Failed to open PWN file {}: {}", file_path, e))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Failed to read PWN file {}: {}", file_path, e))?;

    // Remove multi-line comments first
    let multiline_comment_re = Regex::new(r"/\*[\s\S]*?\*/")
        .map_err(|e| format!("Failed to create multiline comment regex: {}", e))?;
    let content = multiline_comment_re.replace_all(&content, "");

    // Regex patterns to match CreateDynamicObject calls in both formats
    // Positional format: CreateDynamicObject(modelid, x, y, z, rx, ry, rz[, worldid, interiorid, playerid, streamdistance, drawdistance, areaid, priority]);
    // Named format: CreateDynamicObject(modelid, x, y, z, rx, ry, rz[, worldid = value, interiorid = value, playerid = value, streamdistance = value, drawdistance = value, areaid = value, priority = value]);
    // Basic parameters (modelid, x, y, z, rx, ry, rz) are required, optional parameters in [] can be omitted
    let positional_re = Regex::new(r"CreateDynamicObject\s*\(\s*(\d+)\s*,\s*([+-]?\d*\.?\d+)\s*,\s*([+-]?\d*\.?\d+)\s*,\s*([+-]?\d*\.?\d+)\s*,\s*([+-]?\d*\.?\d+)\s*,\s*([+-]?\d*\.?\d+)\s*,\s*([+-]?\d*\.?\d+)\s*(?:,\s*([+-]?\d+))?\s*(?:,\s*([+-]?\d+))?\s*(?:,\s*([+-]?\d+))?\s*(?:,\s*([+-]?\d*\.?\d+|[A-Z_]+))?\s*(?:,\s*([+-]?\d*\.?\d+|[A-Z_]+))?\s*(?:,\s*([+-]?\d+))?\s*(?:,\s*(\d+))?\s*\)\s*;").map_err(|e| {
        format!("Failed to create positional regex pattern: {}", e)
    })?;

    let named_re = Regex::new(r"CreateDynamicObject\s*\(\s*(\d+)\s*,\s*([+-]?\d*\.?\d+)\s*,\s*([+-]?\d*\.?\d+)\s*,\s*([+-]?\d*\.?\d+)\s*,\s*([+-]?\d*\.?\d+)\s*,\s*([+-]?\d*\.?\d+)\s*,\s*([+-]?\d*\.?\d+)\s*(?:,\s*worldid\s*=\s*([+-]?\d+))?\s*(?:,\s*interiorid\s*=\s*([+-]?\d+))?\s*(?:,\s*playerid\s*=\s*([+-]?\d+))?\s*(?:,\s*streamdistance\s*=\s*([+-]?\d*\.?\d+|[A-Z_]+))?\s*(?:,\s*drawdistance\s*=\s*([+-]?\d*\.?\d+|[A-Z_]+))?\s*(?:,\s*areaid\s*=\s*([+-]?\d+))?\s*(?:,\s*priority\s*=\s*(\d+))?\s*\)\s*;").map_err(|e| {
        format!("Failed to create named regex pattern: {}", e)
    })?;

    let mut objects = Vec::new();
    let mut errors = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut parsed_count = 0;

    for (line_num, line) in lines.iter().enumerate() {
        let line = line.trim();

        // Skip empty lines, single-line comments, includes, and other non-relevant content
        if line.is_empty()
            || line.starts_with("//")
            || line.starts_with("#")
            || line.starts_with("/*")
            || line.starts_with("*")
            || line.ends_with("*/")
            || line.contains("hook ")
            || line.contains("function ")
            || line.contains("stock ")
            || line.contains("public ")
            || line.contains("return ")
            || line.starts_with("{")
            || line.starts_with("}")
            || line.starts_with(")")
            || (!line.contains("CreateDynamicObject") && !line.contains("createDynamicObject"))
        {
            continue;
        }

        // Try positional format first, then named format
        let (captures, is_positional) = if let Some(cap) = positional_re.captures(line) {
            (cap, true)
        } else if let Some(cap) = named_re.captures(line) {
            (cap, false)
        } else {
            // Don't report errors for lines that don't contain CreateDynamicObject calls
            // This allows the parser to be more robust and ignore irrelevant content
            continue;
        };

        match parse_create_dynamic_object(&captures, is_positional) {
            Ok(obj) => {
                objects.push(obj);
                parsed_count += 1;
            }
            Err(e) => {
                errors.push(format!("Line {}: {}", line_num + 1, e));
            }
        }
    }

    Ok(PwnImportResult {
        objects,
        line_count: lines.len(),
        parsed_count,
        errors,
    })
}

fn parse_create_dynamic_object(
    captures: &regex::Captures,
    is_positional: bool,
) -> Result<PwnObjectData, String> {
    // Helper function to parse float values, handling constants
    fn parse_float_or_constant(value: &str) -> Result<f32, String> {
        match value.to_uppercase().as_str() {
            "STREAMER_OBJECT_SD" => Ok(200.0), // Default stream distance
            "STREAMER_OBJECT_DD" => Ok(0.0),   // Default draw distance
            _ => value
                .parse::<f32>()
                .map_err(|_| format!("Invalid float value: {}", value)),
        }
    }

    // Helper function to parse optional integer values
    fn parse_optional_i32(value: Option<&regex::Match>) -> Result<Option<i32>, String> {
        match value {
            Some(m) => {
                let val = m
                    .as_str()
                    .parse::<i32>()
                    .map_err(|_| format!("Invalid integer value: {}", m.as_str()))?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    // Helper function to parse optional float values
    fn parse_optional_f32(value: Option<&regex::Match>) -> Result<Option<f32>, String> {
        match value {
            Some(m) => {
                let val = parse_float_or_constant(m.as_str())?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    // Helper function to parse optional u32 values
    fn parse_optional_u32(value: Option<&regex::Match>) -> Result<Option<u32>, String> {
        match value {
            Some(m) => {
                let val = m
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| format!("Invalid unsigned integer value: {}", m.as_str()))?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    let modelid = captures
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .map_err(|_| "Invalid model ID")?;

    let x = parse_float_or_constant(captures.get(2).unwrap().as_str())?;
    let y = parse_float_or_constant(captures.get(3).unwrap().as_str())?;
    let z = parse_float_or_constant(captures.get(4).unwrap().as_str())?;
    let rx = parse_float_or_constant(captures.get(5).unwrap().as_str())?;
    let ry = parse_float_or_constant(captures.get(6).unwrap().as_str())?;
    let rz = parse_float_or_constant(captures.get(7).unwrap().as_str())?;

    // Parse parameters based on format
    let (worldid, interiorid, playerid, streamdistance, drawdistance, areaid, priority) =
        if is_positional {
            // Positional format: modelid, x, y, z, rx, ry, rz, worldid, interiorid, playerid, streamdistance, drawdistance, areaid, priority
            (
                parse_optional_i32(captures.get(8).as_ref())?,
                parse_optional_i32(captures.get(9).as_ref())?,
                parse_optional_i32(captures.get(10).as_ref())?,
                parse_optional_f32(captures.get(11).as_ref())?,
                parse_optional_f32(captures.get(12).as_ref())?,
                parse_optional_i32(captures.get(13).as_ref())?,
                parse_optional_u32(captures.get(14).as_ref())?,
            )
        } else {
            // Named format: modelid, x, y, z, rx, ry, rz, worldid = value, interiorid = value, playerid = value, streamdistance = value, drawdistance = value, areaid = value, priority = value
            (
                parse_optional_i32(captures.get(8).as_ref())?,
                parse_optional_i32(captures.get(9).as_ref())?,
                parse_optional_i32(captures.get(10).as_ref())?,
                parse_optional_f32(captures.get(11).as_ref())?,
                parse_optional_f32(captures.get(12).as_ref())?,
                parse_optional_i32(captures.get(13).as_ref())?,
                parse_optional_u32(captures.get(14).as_ref())?,
            )
        };

    Ok(PwnObjectData {
        modelid,
        x,
        y,
        z,
        rx,
        ry,
        rz,
        worldid,
        interiorid,
        playerid,
        streamdistance,
        drawdistance,
        areaid,
        priority,
    })
}

// Helper function to parse IDE file and extract model/texture names
fn parse_ide_file(ide_path: &str) -> Result<(HashSet<String>, HashSet<String>), String> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(ide_path).map_err(|e| format!("Failed to open IDE file: {}", e))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Failed to read IDE file: {}", e))?;

    let mut models = HashSet::new();
    let mut textures = HashSet::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse objs and tobj sections
        // Format: ID, ModelName, TextureName, ObjectCount, DrawDist, [DrawDist2, ...], Flags
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() >= 3 {
            let model_name = parts[1];
            let texture_name = parts[2];

            if !model_name.is_empty() && model_name != "-" {
                models.insert(model_name.to_string());
            }
            if !texture_name.is_empty() && texture_name != "-" {
                textures.insert(texture_name.to_string());
            }
        }
    }

    Ok((models, textures))
}

// Helper function to find file in directory (case-insensitive)
fn find_file_in_directory(directory: &str, filename: &str) -> Option<String> {
    use std::fs;

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                if name.to_lowercase() == filename.to_lowercase() {
                    return Some(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    None
}

// Helper function to import file to archive
fn import_file_to_archive(
    archive: &mut ImgArchive,
    file_path: &str,
    entry_name: &str,
) -> Result<(), String> {
    use std::fs;

    let data =
        fs::read(file_path).map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

    // Add entry to archive (this is in-memory operation)
    archive
        .add_entry(entry_name, &data)
        .map_err(|e| format!("Failed to add entry to archive: {}", e))?;

    Ok(())
}


// Rengine configuration commands
#[tauri::command]
fn load_rengine_config(config_path: Option<String>) -> Result<crate::config::RengineConfig, String> {
    let path = config_path.as_ref().map(|p| std::path::Path::new(p));
    crate::config::RengineConfig::load(path)
}

#[tauri::command]
fn save_rengine_config(
    config: crate::config::RengineConfig,
    config_path: Option<String>,
) -> Result<(), String> {
    let path = config_path.as_ref().map(|p| std::path::Path::new(p));
    config.save(path)
}

#[tauri::command]
fn get_rengine_config_path(config_path: Option<String>) -> String {
    let path = config_path.as_ref().map(|p| std::path::Path::new(p));
    crate::config::RengineConfig::get_config_path(path)
        .to_string_lossy()
        .to_string()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use crate::config::RengineConfig;

    // Load configuration
    let config_path = RengineConfig::get_config_path(None);
    // log_info!("Loading config from: {}", config_path.display());
    let _config = RengineConfig::load(None).unwrap_or_else(|e| {
        log_warn!("Failed to load rengine.json config: {}. Using defaults.", e);
        RengineConfig::default()
    });

    // Determine effective renderer
    // let effective_renderer = config.effective_renderer();
    // log_info!("Rengine config loaded. Renderer: {:?} (raw: {:?})", effective_renderer, config.renderer);
    
    log_info!("Rengine started. Using config: {}", config_path.display());

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            copy_to_clipboard,
            read_directory,
            create_directory,
            create_file,
            delete_file,
            rename_file,
            copy_file,
            write_file,
            read_file,
            write_log_file,
            read_image_as_base64,
            get_image_metadata,
            save_texture_to_file,
            batch_process_textures,
            process_image,
            get_supported_filters,
            get_supported_effects,
            load_img_archive,
            extract_img_entry,
            batch_extract_img_entries,
            import_via_ide,
            delete_img_entry,
            restore_img_entry,
            get_img_modification_info,
            get_memory_stats,
            get_home_directory,
            import_dff_file,
            import_txd_file,
            import_col_file,
            import_ipl_file,
            parse_pwn_file,
            get_samp_model_by_id,
            get_samp_model_by_name,
            search_samp_models_by_name,
            get_all_samp_models_count,
            parse_blueprint_code,
            generate_blueprint_code,
            load_rengine_config,
            save_rengine_config,
            get_rengine_config_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // SA:MP Model Database Commands
    #[tauri::command]
    fn get_samp_model_by_id(id: u32) -> Result<Option<crate::models::SampModel>, String> {
        Ok(SAMP_MODEL_DATABASE.get_model_by_id(id).cloned())
    }

    #[tauri::command]
    fn get_samp_model_by_name(name: String) -> Result<Option<crate::models::SampModel>, String> {
        Ok(SAMP_MODEL_DATABASE.get_model_by_name(&name).cloned())
    }

    #[tauri::command]
    fn search_samp_models_by_name(
        query: String,
        limit: Option<usize>,
    ) -> Result<Vec<crate::models::SampModel>, String> {
        let limit = limit.unwrap_or(10);
        let models = if query.len() <= 2 {
            // For short queries, use prefix search
            SAMP_MODEL_DATABASE.find_models_by_name_prefix(&query, limit)
        } else {
            // For longer queries, use contains search
            SAMP_MODEL_DATABASE.find_models_by_name_contains(&query, limit)
        };

        Ok(models.into_iter().cloned().collect())
    }

    #[tauri::command]
    fn get_all_samp_models_count() -> Result<usize, String> {
        Ok(SAMP_MODEL_DATABASE.len())
    }

    #[tauri::command]
    async fn parse_blueprint_code(
        source: String,
        language: String,
    ) -> Result<serde_json::Value, String> {
        use crate::blueprint::parser::Parser as ParserTrait;
        use crate::blueprint::PawnParser;

        match language.as_str() {
            "pawn" => {
                let parser = PawnParser::new();
                let result = parser.parse(&source)?;
                Ok(serde_json::json!({
                    "ast": result.ast,
                    "errors": result.errors,
                    "warnings": result.warnings,
                    "language": parser.language()
                }))
            }
            _ => Err(format!("Unsupported language: {}", language)),
        }
    }

    #[tauri::command]
    async fn generate_blueprint_code(
        ast: serde_json::Value,
        language: String,
        options: Option<serde_json::Value>,
    ) -> Result<String, String> {
        use crate::blueprint::{GenerationOptions, PawnGenerator};

        match language.as_str() {
            "pawn" => {
                let gen_options = if let Some(opts) = options {
                    serde_json::from_value(opts).unwrap_or_default()
                } else {
                    GenerationOptions::default()
                };

                let generator = PawnGenerator::with_options(gen_options);
                generator.generate(&ast)
            }
            _ => Err(format!("Unsupported language: {}", language)),
        }
    }
}
