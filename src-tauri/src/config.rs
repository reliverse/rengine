// Rengine configuration management
// Handles reading and writing rengine.json configuration file

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

/// Renderer backend options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RendererBackend {
    /// React Three Fiber (current default)
    R3f,
    /// Bevy renderer
    Bevy,
}

impl Default for RendererBackend {
    fn default() -> Self {
        RendererBackend::R3f
    }
}

fn default_true() -> bool {
    true
}

fn default_fps() -> u32 {
    60
}

fn default_pixel_ratio() -> f64 {
    1.0
}

/// Main Rengine configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RengineConfig {
    /// Selected renderer backend
    #[serde(default)]
    pub renderer: RendererBackend,
    /// Use headless mode (no windowing) - applies to Bevy
    #[serde(default = "default_true")]
    pub headless: bool,
    /// Target FPS for update loop
    #[serde(default = "default_fps")]
    pub target_fps: u32,
    /// Enable antialiasing
    #[serde(default = "default_true")]
    pub antialias: bool,
    /// Enable shadows
    #[serde(default = "default_true")]
    pub shadows: bool,
    /// Pixel ratio
    #[serde(default = "default_pixel_ratio")]
    pub pixel_ratio: f64,
    /// Enable debug mode
    #[serde(default)]
    pub debug: bool,
    /// Log level: "trace", "debug", "info", "warn", "error"
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

impl Default for RengineConfig {
    fn default() -> Self {
        Self {
            renderer: RendererBackend::R3f,
            headless: true,
            target_fps: 60,
            antialias: true,
            shadows: true,
            pixel_ratio: 1.0,
            debug: false,
            log_level: "info".to_string(),
        }
    }
}

fn default_log_level() -> String {
    "info".to_string()
}

impl RengineConfig {
    /// Get the config file path (in the project root or user config directory)
    pub fn get_config_path(project_root: Option<&Path>) -> PathBuf {
        if let Some(root) = project_root {
            root.join("rengine.json")
        } else {
            // Try to find config in current directory, parent directory, or user config
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            
            // Check current directory first
            let config_in_current = current_dir.join("rengine.json");
            if config_in_current.exists() {
                return config_in_current;
            }
            
            // Check parent directory (in case we're running from src-tauri/)
            if let Some(parent) = current_dir.parent() {
                let config_in_parent = parent.join("rengine.json");
                if config_in_parent.exists() {
                    return config_in_parent;
                }
            }
            
            // Default to current directory (will create file there if it doesn't exist)
            config_in_current
        }
    }

    /// Load configuration from file
    pub fn load(path: Option<&Path>) -> Result<Self, String> {
        let config_path = Self::get_config_path(path);
        
        if !config_path.exists() {
            // Return default config if file doesn't exist
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path).map_err(|e| {
            format!("Failed to read config file {}: {}", config_path.display(), e)
        })?;

        let config: RengineConfig = serde_json::from_str(&content).map_err(|e| {
            format!("Failed to parse config file {}: {}", config_path.display(), e)
        })?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: Option<&Path>) -> Result<(), String> {
        let config_path = Self::get_config_path(path);
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                format!("Failed to create config directory: {}", e)
            })?;
        }

        let content = serde_json::to_string_pretty(self).map_err(|e| {
            format!("Failed to serialize config: {}", e)
        })?;

        fs::write(&config_path, content).map_err(|e| {
            format!("Failed to write config file {}: {}", config_path.display(), e)
        })?;

        Ok(())
    }

    /// Get the effective renderer (same as renderer since Auto is removed)
    pub fn effective_renderer(&self) -> RendererBackend {
        self.renderer
    }
    
    /// Check if Bevy should be enabled based on renderer selection
    pub fn is_bevy_enabled(&self) -> bool {
        self.renderer == RendererBackend::Bevy
    }
    
    /// Check if R3F should be enabled based on renderer selection
    pub fn is_r3f_enabled(&self) -> bool {
        self.renderer == RendererBackend::R3f
    }
}
