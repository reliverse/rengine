//! Godot-specific bindings for Rengine RenderWare functionality

use godot::prelude::*;
use rengine_core::renderware::RwAnalyzer as CoreRwAnalyzer;

use std::fs::File;

/// Godot wrapper for RenderWare analyzer
#[derive(GodotClass)]
#[class(base=Node)]
pub struct RwAnalyzer {
    core_analyzer: CoreRwAnalyzer,
    base: Base<Node>,
}

#[godot_api]
impl RwAnalyzer {
    #[func]
    fn analyze_file(&mut self, file_path: GString) -> VarDictionary {
        let path = file_path.to_string();

        match File::open(&path) {
            Ok(_file) => {
                match std::fs::read(&path) {
                    Ok(data) => {
                        match self.core_analyzer.analyze_file(&path, &data) {
                            Ok(result) => {
                                let mut dict = VarDictionary::new();
                                dict.set("success", true);
                                dict.set("format", result.format);
                                dict.set("size", result.file_size as i64);
                                dict.set("format_description", result.format_description);
                                dict.set("rw_version", result.rw_version as i64);
                                dict.set("total_chunks", result.total_chunks as i64);
                                dict.set("max_depth", result.max_depth as i64);
                                dict.set("analysis_time_ms", result.analysis_time_ms as i64);

                                dict.set("corruption_warnings_count", result.corruption_warnings.len() as i64);

                                dict
                            }
                            Err(e) => {
                                let mut dict = VarDictionary::new();
                                dict.set("success", false);
                                dict.set("error", format!("Failed to analyze file: {}", e));
                                dict
                            }
                        }
                    }
                    Err(e) => {
                        let mut dict = VarDictionary::new();
                        dict.set("success", false);
                        dict.set("error", format!("Failed to read file: {}", e));
                        dict
                    }
                }
            }
            Err(e) => {
                let mut dict = VarDictionary::new();
                dict.set("success", false);
                dict.set("error", format!("Failed to open file: {}", e));
                dict
            }
        }
    }

    #[func]
    fn parse_img_archive(&mut self, file_path: GString) -> VarDictionary {
        let path = file_path.to_string();

        match File::open(&path) {
            Ok(mut file) => {
                match rengine_core::renderware::img::ImgArchive::parse_archive(&mut file, &path) {
                    Ok(archive) => {
                        let mut dict = VarDictionary::new();
                        dict.set("success", true);
                        dict.set("version", format!("{:?}", archive.version));
                        dict.set("entries_count", archive.entries.len() as i64);
                        dict.set("total_size", archive.entries.iter().map(|e| e.size).sum::<u32>() as i64);
                        dict
                    }
                    Err(e) => {
                        let mut dict = VarDictionary::new();
                        dict.set("success", false);
                        dict.set("error", format!("Failed to parse IMG archive: {:?}", e));
                        dict
                    }
                }
            }
            Err(e) => {
                let mut dict = VarDictionary::new();
                dict.set("success", false);
                dict.set("error", format!("Failed to open file: {}", e));
                dict
            }
        }
    }

    #[func]
    fn parse_dff_model(&mut self, file_path: GString) -> VarDictionary {
        let path = file_path.to_string();

        match File::open(&path) {
            Ok(mut file) => {
                match rengine_core::renderware::dff::DffModel::load_from_reader(&mut file, &path) {
                    Ok(model) => {
                        let mut dict = VarDictionary::new();
                        dict.set("success", true);
                        dict.set("rw_version", model.rw_version as i64);
                        dict.set("geometry_count", model.geometries.len() as i64);
                        dict.set("material_count", model.geometries.iter().map(|g| g.materials.len()).sum::<usize>() as i64);
                        dict.set("has_animations", !model.uv_animations.is_empty());
                        dict.set("atomic_count", model.atomics.len() as i64);
                        dict.set("frame_count", model.frames.len() as i64);
                        dict
                    }
                    Err(e) => {
                        let mut dict = VarDictionary::new();
                        dict.set("success", false);
                        dict.set("error", format!("Failed to parse DFF model: {:?}", e));
                        dict
                    }
                }
            }
            Err(e) => {
                let mut dict = VarDictionary::new();
                dict.set("success", false);
                dict.set("error", format!("Failed to open file: {}", e));
                dict
            }
        }
    }
}

#[godot_api]
impl INode for RwAnalyzer {
    fn init(base: Base<Node>) -> Self {
        godot_print!("RwAnalyzer initialized");

        Self {
            core_analyzer: CoreRwAnalyzer::new(),
            base,
        }
    }
}


use godot::init::{ExtensionLibrary};

struct RwExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RwExtension {
    fn on_level_init(level: godot::init::InitLevel) {
        if level == godot::init::InitLevel::Scene {
            godot_print!("Rengine RW GDExtension initialized");
        }
    }

    fn on_level_deinit(level: godot::init::InitLevel) {
        if level == godot::init::InitLevel::Scene {
            godot_print!("Rengine RW GDExtension deinitialized");
        }
    }
}