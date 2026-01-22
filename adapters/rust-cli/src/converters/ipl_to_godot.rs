use rengine_core::renderware::ipl::IPLFile;
use std::{fs, path::Path};
use anyhow::{Result, anyhow};
use tracing::{info, error};

use walkdir;
use crate::converters::DffToObjConverter;

pub struct IplToGodotConverter {
    verbose: bool,
    quiet: bool,
}

impl IplToGodotConverter {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub async fn convert(
        &self,
        input: &str,
        output: &str,
        template: Option<&str>,
        project_name: &str,
        with_rust: bool,
        ide_files: Option<&[String]>,
        convert_assets: bool,
        dff_path: Option<&str>,
        txd_path: Option<&str>,
    ) -> Result<i32> {
        let input_path = Path::new(input);

        if input_path.is_dir() {
            // Convert directory to full Godot project
            self.convert_directory_to_godot_project(
                input, output, project_name, template, ide_files, with_rust, convert_assets, dff_path, txd_path
            ).await
        } else {
            // Convert single IPL file to Godot scene
            self.convert_single_ipl_to_godot_scene(
                input, output, template, ide_files, convert_assets, dff_path, txd_path
            ).await
        }
    }

    async fn convert_directory_to_godot_project(
        &self,
        input_dir: &str,
        output_dir: &str,
        project_name: &str,
        _template: Option<&str>,
        ide_files: Option<&[String]>,
        with_rust: bool,
        convert_assets: bool,
        dff_path: Option<&str>,
        txd_path: Option<&str>,
    ) -> Result<i32> {
        if self.verbose {
            info!("Converting IPL directory to Godot project: {} -> {}", input_dir, output_dir);
        }

        // Create project directory structure
        fs::create_dir_all(output_dir)?;

        // Generate project.godot file
        self.generate_project_godot(output_dir, project_name)?;

        // Generate main scene
        let main_scene_path = Path::new(output_dir).join("main.tscn");
        self.generate_main_scene(main_scene_path.to_str().unwrap(), project_name)?;

        // Process IPL files
        let ipl_files = self.find_ipl_files(input_dir)?;
        let mut total_instances = 0;

        // Convert assets if requested
        let mut all_converted_assets = std::collections::HashMap::new();
        if convert_assets {
            if self.verbose {
                info!("Converting assets for all IPL files");
            }
            for ipl_file in &ipl_files {
                if let Ok(ipl) = IPLFile::load_from_path(ipl_file) {
                    let assets_dir = Path::new(output_dir).join("assets");
                    let converted_assets = self.convert_ipl_assets(&ipl, dff_path, txd_path, &assets_dir).await?;
                    all_converted_assets.extend(converted_assets);
                }
            }
        }

        for ipl_file in &ipl_files {
            if self.verbose {
                info!("Processing IPL file: {}", ipl_file);
            }

            match IPLFile::load_from_path(ipl_file) {
                Ok(ipl) => {
                    let scene_name = Path::new(ipl_file)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("scene");

                    let scene_path = Path::new(output_dir).join(format!("{}.tscn", scene_name));

                    // Use converted assets if available
                    let scene_assets = if convert_assets {
                        // Filter assets for this IPL file's models
                        let mut scene_assets = std::collections::HashMap::new();
                        for instance in &ipl.instances {
                            let model_name = self.resolve_model_name(&instance.model_name, ide_files);
                            if let Some(asset_path) = all_converted_assets.get(&model_name) {
                                scene_assets.insert(model_name, asset_path.clone());
                            }
                        }
                        scene_assets
                    } else {
                        std::collections::HashMap::new()
                    };

                    self.generate_godot_scene_from_ipl(&ipl, scene_path.to_str().unwrap(), ide_files, &scene_assets)?;

                    total_instances += ipl.instances.len();

                    if self.verbose {
                        info!("Generated scene with {} instances", ipl.instances.len());
                    }
                }
                Err(e) => {
                    error!("Failed to load IPL file {}: {}", ipl_file, e);
                }
            }
        }

        // Generate Rust GDExtension if requested
        if with_rust {
            self.generate_rust_gdextension(output_dir, project_name)?;
        }

        if !self.quiet {
            println!("Converted IPL directory {} to Godot project: {}", input_dir, output_dir);
            println!("Processed {} IPL files", ipl_files.len());
            println!("Created {} object instances", total_instances);
            if with_rust {
                println!("Included Rust GDExtension integration");
            }
        }

        Ok(0)
    }

    async fn convert_single_ipl_to_godot_scene(
        &self,
        input_file: &str,
        output_file: &str,
        template: Option<&str>,
        ide_files: Option<&[String]>,
        convert_assets: bool,
        dff_path: Option<&str>,
        txd_path: Option<&str>,
    ) -> Result<i32> {
        if self.verbose {
            info!("Converting IPL file to Godot scene: {} -> {}", input_file, output_file);
        }

        let ipl = IPLFile::load_from_path(input_file)?;

        // Convert assets if requested
        let converted_assets = if convert_assets {
            self.convert_ipl_assets(&ipl, dff_path, txd_path, Path::new(output_file).parent().unwrap_or(Path::new("."))).await?
        } else {
            std::collections::HashMap::new()
        };

        // Generate Godot scene
        if let Some(template_path) = template {
            self.generate_godot_scene_from_ipl_with_template(&ipl, output_file, template_path, ide_files, &converted_assets)?;
        } else {
            self.generate_godot_scene_from_ipl(&ipl, output_file, ide_files, &converted_assets)?;
        }

        if !self.quiet {
            if template.is_some() {
                println!("Extended Godot scene template {} with IPL data: {}", template.unwrap(), output_file);
            } else {
                println!("Converted {} to Godot scene: {}", input_file, output_file);
            }
            println!("Created {} object instances", ipl.instances.len());
        }

        Ok(0)
    }

    fn generate_project_godot(&self, project_dir: &str, project_name: &str) -> Result<()> {
        let project_godot_path = Path::new(project_dir).join("project.godot");

        let content = format!("[application]

config/name=\"{}\"
config/description=\"\"
run/main_scene=\"res://main.tscn\"
config/features=PackedStringArray(\"4.1\")

[display]

window/size/viewport_width=1920
window/size/viewport_height=1080

[rendering]

renderer/rendering_method=\"gl_compatibility\"
", project_name);

        fs::write(project_godot_path, content)?;
        Ok(())
    }

    fn generate_main_scene(&self, scene_path: &str, project_name: &str) -> Result<()> {
        let content = format!("[gd_scene load_steps=2 format=3 uid=\"uid://b1q8q8q8q8q8q8q8q\"]

[ext_resource type=\"Script\" path=\"res://main.gd\" id=\"1\"]

[node name=\"Main\" type=\"Node\"]
script = ExtResource(\"1\")

[node name=\"WorldEnvironment\" type=\"WorldEnvironment\" parent=\".\"]
environment = SubResource(\"1\")

[sub_resource type=\"Environment\" id=\"1\"]
background_mode = 1
background_color = Color(0.2, 0.4, 0.8, 1)

[script]
code = \"\"\"
extends Node

func _ready():
    print(\"{project_name} - IPL Map Project\")
    # Add your IPL scenes as children here
\"\"\"
", project_name = project_name);

        fs::write(scene_path, content)?;

        // Generate main.gd script
        let main_gd_path = Path::new(scene_path).with_extension("gd");
        let gd_content = format!("extends Node

func _ready():
    print(\"{project_name} - IPL Map Project\")
    # Load and add IPL scenes
    _load_ipl_scenes()

func _load_ipl_scenes():
    # This function would load IPL scenes dynamically
    # For now, scenes are added manually in the editor
    pass
", project_name = project_name);
        fs::write(main_gd_path, gd_content)?;

        Ok(())
    }

    fn generate_godot_scene_from_ipl(&self, ipl: &IPLFile, scene_path: &str, ide_files: Option<&[String]>, converted_assets: &std::collections::HashMap<String, String>) -> Result<()> {
        let mut content = format!("[gd_scene load_steps={{load_steps}} format=3 uid=\"uid://ipl_scene\"]

[node name=\"IPLScene\" type=\"Node3D\"]

");

        let mut load_step_count = 1; // Start with 1 for the root node

        // Add instances
        for (i, instance) in ipl.instances.iter().enumerate() {
            let model_name = self.resolve_model_name(&instance.model_name, ide_files);

            // Check if we have a converted OBJ asset for this model
            if let Some(obj_path) = converted_assets.get(&model_name) {
                // Use OBJ mesh
                content.push_str(&format!("[node name=\"Instance{i}\" type=\"MeshInstance3D\" parent=\".\"]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {x:.6}, {y:.6}, {z:.6})

[sub_resource type=\"StandardMaterial3D\" id=\"{mat_id}\"]
resource_name = \"{model}\"

[sub_resource type=\"OBJMesh\" id=\"{mesh_id}\"]
material = SubResource(\"{mat_ref}\")
obj_file_path = \"{obj_path}\"

",
                    i = i,
                    x = instance.position.x,
                    y = instance.position.y,
                    z = instance.position.z,
                    mat_id = i + 1,
                    model = model_name,
                    mesh_id = i + 2,
                    mat_ref = i + 1,
                    obj_path = obj_path
                ));
            } else {
                // Use placeholder mesh
                content.push_str(&format!("[node name=\"Instance{i}\" type=\"MeshInstance3D\" parent=\".\"]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {x:.6}, {y:.6}, {z:.6})

[sub_resource type=\"PlaceholderMaterial3D\" id=\"{mat_id}\"]
resource_name = \"{model}\"

[sub_resource type=\"BoxMesh\" id=\"{mesh_id}\"]
material = SubResource(\"{mat_ref}\")

",
                    i = i,
                    x = instance.position.x,
                    y = instance.position.y,
                    z = instance.position.z,
                    mat_id = i + 1,
                    model = model_name,
                    mesh_id = i + 2,
                    mat_ref = i + 1
                ));
            }

            load_step_count += 1;
        }

        // Update load_steps
        content = content.replace("{load_steps}", &format!("{}", load_step_count));

        fs::write(scene_path, content)?;
        Ok(())
    }

    fn generate_godot_scene_from_ipl_with_template(
        &self,
        ipl: &IPLFile,
        scene_path: &str,
        template_path: &str,
        ide_files: Option<&[String]>,
        converted_assets: &std::collections::HashMap<String, String>
    ) -> Result<()> {
        // Read the template scene file
        let template_content = fs::read_to_string(template_path)
            .map_err(|e| anyhow!("Failed to read template file {}: {}", template_path, e))?;

        // Parse the template to find existing load_steps and node structure
        let (load_steps, updated_content) = self.parse_and_extend_godot_scene(&template_content, ipl, ide_files, converted_assets)?;

        // Write the extended scene
        fs::write(scene_path, updated_content)?;

        if self.verbose {
            info!("Extended template scene with {} additional IPL instances, total load_steps: {}", ipl.instances.len(), load_steps);
        }

        Ok(())
    }

    fn parse_and_extend_godot_scene(
        &self,
        template_content: &str,
        ipl: &IPLFile,
        ide_files: Option<&[String]>,
        converted_assets: &std::collections::HashMap<String, String>
    ) -> Result<(usize, String)> {
        let lines: Vec<&str> = template_content.lines().collect();
        let mut load_steps = 1; // Default
        let mut content = template_content.to_string();

        // Try to find and update load_steps
        for (_i, line) in lines.iter().enumerate() {
            if line.contains("load_steps=") {
                // Extract current load_steps value
                if let Some(start) = line.find("load_steps=") {
                    let remaining = &line[start + 11..]; // Skip "load_steps="
                    if let Some(end) = remaining.find(' ') {
                        if let Ok(current_steps) = remaining[..end].parse::<usize>() {
                            load_steps = current_steps + ipl.instances.len() * 2; // Each instance needs 2 sub_resources
                            content = content.replace(
                                &format!("load_steps={}", current_steps),
                                &format!("load_steps={}", load_steps)
                            );
                        }
                    }
                }
                break;
            }
        }

        // Find the root node to add instances as children
        let mut insert_position = content.len();

        // Look for the last node definition (usually the root node)
        for (i, line) in lines.iter().enumerate().rev() {
            if line.starts_with("[node ") && line.contains("type=") {
                // Find the end of this node definition (next [node or end of file)
                let mut end_pos = content.len();
                for j in i+1..lines.len() {
                    if lines[j].starts_with("[node ") || lines[j].starts_with("[sub_resource ") {
                        end_pos = lines[..j].iter().map(|l| l.len() + 1).sum(); // +1 for newline
                        break;
                    }
                }
                insert_position = end_pos;
                break;
            }
        }

        // Add IPL instances after the root node
        let mut instances_content = String::new();
        for (i, instance) in ipl.instances.iter().enumerate() {
            let model_name = self.resolve_model_name(&instance.model_name, ide_files);
            let base_id = 1000 + i * 2; // Use high IDs to avoid conflicts

            // Check if we have a converted OBJ asset for this model
            if let Some(obj_path) = converted_assets.get(&model_name) {
                // Use OBJ mesh
                instances_content.push_str(&format!("
[node name=\"IPL_Instance{i}\" type=\"MeshInstance3D\" parent=\".\"]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {x:.6}, {y:.6}, {z:.6})

[sub_resource type=\"StandardMaterial3D\" id=\"{mat_id}\"]
resource_name = \"{model}\"

[sub_resource type=\"OBJMesh\" id=\"{mesh_id}\"]
material = SubResource(\"{mat_ref}\")
obj_file_path = \"{obj_path}\"
",
                    i = i,
                    x = instance.position.x,
                    y = instance.position.y,
                    z = instance.position.z,
                    mat_id = base_id,
                    model = model_name,
                    mesh_id = base_id + 1,
                    mat_ref = base_id,
                    obj_path = obj_path
                ));
            } else {
                // Use placeholder mesh
                instances_content.push_str(&format!("
[node name=\"IPL_Instance{i}\" type=\"MeshInstance3D\" parent=\".\"]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {x:.6}, {y:.6}, {z:.6})

[sub_resource type=\"PlaceholderMaterial3D\" id=\"{mat_id}\"]
resource_name = \"{model}\"

[sub_resource type=\"BoxMesh\" id=\"{mesh_id}\"]
material = SubResource(\"{mat_ref}\")
",
                    i = i,
                    x = instance.position.x,
                    y = instance.position.y,
                    z = instance.position.z,
                    mat_id = base_id,
                    model = model_name,
                    mesh_id = base_id + 1,
                    mat_ref = base_id
                ));
            }
        }

        // Insert the instances content at the determined position
        content.insert_str(insert_position, &instances_content);

        Ok((load_steps, content))
    }

    fn generate_rust_gdextension(&self, project_dir: &str, project_name: &str) -> Result<()> {
        // Create basic Rust GDExtension structure
        let rust_dir = Path::new(project_dir).join("rust");
        fs::create_dir_all(&rust_dir)?;

        // Generate Cargo.toml for GDExtension
        let cargo_toml = format!("[package]
name = \"{project_name}-gdextension\"
version = \"0.1.0\"
edition = \"2021\"

[lib]
crate-type = [\"cdylib\"]

[dependencies]
godot = \"0.1\"
", project_name = project_name);
        fs::write(rust_dir.join("Cargo.toml"), cargo_toml)?;

        // Generate lib.rs
        let lib_rs = format!("use godot::prelude::*;

struct {project_name}Extension;

#[gdextension]
unsafe impl ExtensionLibrary for {project_name}Extension {{}}
", project_name = project_name);
        fs::write(rust_dir.join("src").join("lib.rs"), lib_rs)?;

        // Create src directory
        fs::create_dir_all(rust_dir.join("src"))?;

        // Update project.godot to include GDExtension
        let project_godot_path = Path::new(project_dir).join("project.godot");
        if let Ok(mut content) = fs::read_to_string(&project_godot_path) {
            content.push_str(&format!("\n[gdextension]\n\nrust=\"res://rust/bin/{project_name}-gdextension.dll\"\n", project_name = project_name));
            fs::write(project_godot_path, content)?;
        }

        Ok(())
    }

    async fn convert_ipl_assets(
        &self,
        ipl: &IPLFile,
        dff_path: Option<&str>,
        txd_path: Option<&str>,
        output_dir: &Path,
    ) -> Result<std::collections::HashMap<String, String>> {
        let mut converted_assets = std::collections::HashMap::new();

        if dff_path.is_none() {
            if self.verbose {
                info!("No DFF path specified, skipping asset conversion");
            }
            return Ok(converted_assets);
        }

        let dff_base_path = Path::new(dff_path.unwrap());
        let assets_dir = output_dir.join("assets");
        fs::create_dir_all(&assets_dir)?;

        // Collect unique model names from IPL instances
        let mut unique_models = std::collections::HashSet::new();
        for instance in &ipl.instances {
            unique_models.insert(instance.model_name.clone());
        }

        if self.verbose {
            info!("Found {} unique models in IPL, converting to OBJ", unique_models.len());
        }

        // Convert each unique model
        for model_name in unique_models {
            let dff_file_path = dff_base_path.join(format!("{}.dff", model_name.to_lowercase()));

            if !dff_file_path.exists() {
                if self.verbose {
                    info!("DFF file not found: {}, skipping", dff_file_path.display());
                }
                continue;
            }

            let obj_file_path = assets_dir.join(format!("{}.obj", model_name));

            // Convert DFF to OBJ
            let converter = DffToObjConverter::new(self.verbose, self.quiet);
            match converter.convert(
                &dff_file_path.to_string_lossy(),
                &obj_file_path.to_string_lossy(),
                txd_path,
                false // Don't use CSV for now
            ).await {
                Ok(_) => {
                    // Store mapping from model name to OBJ path (relative to scene)
                    let relative_obj_path = format!("res://assets/{}.obj", model_name);
                    converted_assets.insert(model_name.clone(), relative_obj_path);

                    if self.verbose {
                        info!("Converted {} to {}", dff_file_path.display(), obj_file_path.display());
                    }
                }
                Err(e) => {
                    error!("Failed to convert {}: {}", dff_file_path.display(), e);
                }
            }
        }

        if self.verbose {
            info!("Successfully converted {} assets", converted_assets.len());
        }

        Ok(converted_assets)
    }

    fn find_ipl_files(&self, directory: &str) -> Result<Vec<String>> {
        let mut ipl_files = Vec::new();

        for entry in walkdir::WalkDir::new(directory) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if let Some(path_str) = entry.path().to_str() {
                    if path_str.to_lowercase().ends_with(".ipl") {
                        ipl_files.push(path_str.to_string());
                    }
                }
            }
        }

        Ok(ipl_files)
    }

    fn resolve_model_name(&self, model_name: &str, _ide_files: Option<&[String]>) -> String {
        // For now, just return the model name
        // In a full implementation, this would look up names from IDE files
        model_name.to_string()
    }
}