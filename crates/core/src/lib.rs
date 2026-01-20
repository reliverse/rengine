//! Core shared functionality for Rengine
//!
//! This crate contains shared logic that can be used across different
//! Rengine backends (Fiber/Tauri, Godot, etc.)

use serde::{Deserialize, Serialize};
use thiserror::Error;

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


pub mod renderware;

// Re-export renderware functionality for easy access
pub use renderware::*;