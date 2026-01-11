// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use base64::{engine::general_purpose, Engine as _};
use photon_rs::native::{open_image, save_image};
use serde::{Deserialize, Serialize};
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
            get_supported_effects
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
