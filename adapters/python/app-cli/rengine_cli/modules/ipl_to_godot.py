#!/usr/bin/env python3
"""
IPL to Godot Scene/Project Converter

Converts IPL (Item Placement) map files to Godot scene files or complete projects.

Usage as a module:
    from ipl_to_godot import IPLToGodotConverter
    converter = IPLToGodotConverter()
    result = converter.convert_directory_to_godot_project('/path/to/ipl/files', '/output/dir')
"""

import os
import glob
import logging
import sys
from typing import Dict, Any, Optional, List


class IPLToGodotConverter:
    """Converter for IPL files to Godot scene files and projects"""

    def __init__(self, logger: Optional[logging.Logger] = None):
        self.logger = logger or logging.getLogger(__name__)

    def convert_to_godot(self, ipl_file: str, output_file: str, template_file: Optional[str] = None) -> Dict[str, Any]:
        """Convert IPL file to Godot .tscn scene file

        Args:
            ipl_file: Path to the IPL file
            output_file: Path for the output .tscn file
            template_file: Optional template scene file

        Returns:
            Dictionary with conversion results and statistics
        """
        # Parse IPL file
        ipl_data = self._parse_ipl_file(ipl_file)

        # Generate Godot scene
        scene_content = self._generate_godot_scene(ipl_data, ipl_file, template_file)

        # Write scene file
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(scene_content)

        result = {
            'scene_file': output_file,
            'ipl_file': ipl_file
        }

        # Add statistics
        if 'sections' in ipl_data:
            sections = ipl_data['sections']
            if 'inst' in sections:
                result['instances'] = sections['inst']['count']
            if 'zone' in sections:
                result['zones'] = sections['zone']['count']

        return result

    def convert_directory_to_godot_project(self, input_dir: str, output_dir: str,
                                         project_name: str = 'IPLMap',
                                         template_file: Optional[str] = None) -> Dict[str, Any]:
        """Convert all IPL files in a directory to a complete Godot project

        Args:
            input_dir: Directory containing IPL files
            output_dir: Output directory for Godot project
            project_name: Name for the Godot project
            template_file: Optional template scene file

        Returns:
            Dictionary with project information and statistics
        """
        # Find all IPL files
        ipl_files = []
        for pattern in ['*.ipl', '*.IPL']:
            ipl_files.extend(glob.glob(os.path.join(input_dir, pattern)))

        if not ipl_files:
            raise Exception(f"No IPL files found in directory: {input_dir}")

        # Parse all IPL files
        all_ipl_data = {}
        total_instances = 0
        total_zones = 0

        for ipl_file in ipl_files:
            try:
                ipl_data = self._parse_ipl_file(ipl_file)
                filename = os.path.basename(ipl_file)
                all_ipl_data[filename] = ipl_data

                # Count totals
                if 'sections' in ipl_data:
                    sections = ipl_data['sections']
                    if 'inst' in sections:
                        total_instances += sections['inst']['count']
                    if 'zone' in sections:
                        total_zones += sections['zone']['count']

                self.logger.info(f"Parsed IPL file: {filename}")

            except Exception as e:
                self.logger.warning(f"Failed to parse IPL file {ipl_file}: {e}")

        # Create Godot project structure
        self._create_godot_project_structure(output_dir, project_name)

        # Generate demo scene with IPL data and RW analysis capabilities
        demo_scene_content = self._generate_demo_godot_scene(all_ipl_data, project_name)
        demo_scene_path = os.path.join(output_dir, 'demo.tscn')

        with open(demo_scene_path, 'w', encoding='utf-8') as f:
            f.write(demo_scene_content)

        # Generate combined IPL scene (for reference)
        main_scene_content = self._generate_combined_godot_scene(all_ipl_data, project_name)
        main_scene_path = os.path.join(output_dir, 'ipl_combined.tscn')

        with open(main_scene_path, 'w', encoding='utf-8') as f:
            f.write(main_scene_content)

        # Generate individual scenes for each IPL file (optional)
        scenes_dir = os.path.join(output_dir, 'scenes')
        os.makedirs(scenes_dir, exist_ok=True)

        for filename, ipl_data in all_ipl_data.items():
            scene_name = os.path.splitext(filename)[0] + '.tscn'
            scene_path = os.path.join(scenes_dir, scene_name)
            scene_content = self._generate_godot_scene(ipl_data, filename, template_file)

            with open(scene_path, 'w', encoding='utf-8') as f:
                f.write(scene_content)

        result = {
            'project_dir': output_dir,
            'project_name': project_name,
            'ipl_files': len(all_ipl_data),
            'total_instances': total_instances,
            'total_zones': total_zones,
            'demo_scene': demo_scene_path,
            'main_scene': main_scene_path,
            'individual_scenes': len(all_ipl_data)
        }

        return result

    def _create_godot_project_structure(self, output_dir: str, project_name: str):
        """Create complete Godot project structure with GDExtension integration"""
        # Create output directory
        os.makedirs(output_dir, exist_ok=True)

        # Create project.godot
        project_godot_content = f"""; Engine configuration file.
; It's best edited using the editor UI and not directly,
; since the parameters that go here are not all obvious.
;
; Format:
;   [section] ; section goes between []
;   param=value ; assign values to parameters

config_version=5

[application]

config/name="{project_name}"
run/main_scene="res://demo.tscn"
config/features=PackedStringArray("4.5", "Forward Plus")
config/icon="res://icon.svg"
"""

        project_godot_path = os.path.join(output_dir, 'project.godot')
        with open(project_godot_path, 'w', encoding='utf-8') as f:
            f.write(project_godot_content)

        # Create basic icon.svg (minimal)
        icon_content = '''<svg width="64" height="64" xmlns="http://www.w3.org/2000/svg">
  <rect width="64" height="64" fill="#478cbf"/>
  <text x="32" y="40" font-family="Arial" font-size="40" fill="white" text-anchor="middle">G</text>
</svg>'''

        icon_path = os.path.join(output_dir, 'icon.svg')
        with open(icon_path, 'w', encoding='utf-8') as f:
            f.write(icon_content)

        # Create .gitignore for Godot project
        gitignore_content = """# Godot-specific ignores
.import/
*.import

# Imported binaries
*.so
*.dll
*.dylib

# Build artifacts
target/
Cargo.lock

# Temporary files
*.tmp
*.swp
*.bak
"""

        gitignore_path = os.path.join(output_dir, '.gitignore')
        with open(gitignore_path, 'w', encoding='utf-8') as f:
            f.write(gitignore_content)

        # Create rengine_rw.gdextension
        gdextension_content = """[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.2
reloadable = true

[libraries]
linux.debug.x86_64 = "res://librengine_godot_rw.so"
linux.release.x86_64 = "res://librengine_godot_rw.so"
windows.debug.x86_64 = "res://rengine_godot_rw.dll"
windows.release.x86_64 = "res://rengine_godot_rw.dll"
macos.debug = "res://librengine_godot_rw.dylib"
macos.release = "res://librengine_godot_rw.dylib"
macos.debug.arm64 = "res://librengine_godot_rw.dylib"
macos.release.arm64 = "res://librengine_godot_rw.dylib"
"""

        gdextension_path = os.path.join(output_dir, 'rengine_rw.gdextension')
        with open(gdextension_path, 'w', encoding='utf-8') as f:
            f.write(gdextension_content)

        # Create build script
        build_script_content = """#!/bin/bash

# Build and copy Rengine RW extension for Godot
# Run this from the project root directory

# Ensure the script is executable (self-healing)
chmod +x "$0" 2>/dev/null || true

# Build the godot-rw extension from workspace root
echo "Building Rengine RW extension..."
if command -v cargo >/dev/null 2>&1; then
    cargo build -p rengine-godot-rw --release
    BUILD_STATUS=$?

    if [ $BUILD_STATUS -eq 0 ]; then
        echo "✅ Extension built successfully!"

        # Copy the library to the project directory
        if [ -f "target/release/librengine_godot_rw.so" ]; then
            cp target/release/librengine_godot_rw.so .
            echo "✅ Linux library copied!"
        fi

        if [ -f "target/release/rengine_godot_rw.dll" ]; then
            cp target/release/rengine_godot_rw.dll .
            echo "✅ Windows library copied!"
        fi

        if [ -f "target/release/librengine_godot_rw.dylib" ]; then
            cp target/release/librengine_godot_rw.dylib .
            echo "✅ macOS library copied!"
        fi

        echo ""
        echo "🎮 You can now open the Godot project and run the demo scene!"
        echo "   Main scene: demo.tscn"
        echo "   IPL scenes: scenes/*.tscn"
    else
        echo "❌ Failed to build extension"
        exit 1
    fi
else
    echo "❌ Cargo not found. Please install Rust and Cargo first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi
"""

        build_script_path = os.path.join(output_dir, 'build_extension.sh')
        with open(build_script_path, 'w', encoding='utf-8') as f:
            f.write(build_script_content)

        # Make build script executable
        try:
            os.chmod(build_script_path, 0o755)
        except:
            pass  # Ignore permission errors on some systems

        # Create addons directory structure
        addons_dir = os.path.join(output_dir, 'addons')
        os.makedirs(addons_dir, exist_ok=True)

        rengine_addon_dir = os.path.join(addons_dir, 'rengine_rw')
        os.makedirs(rengine_addon_dir, exist_ok=True)

        # Create plugin configuration file
        plugin_config = """[plugin]

name="Rengine RW"
description="RenderWare file parsing and IPL map integration for Godot"
author="Rengine"
version="1.0"
script="rengine_rw_plugin.gd"
"""

        plugin_config_path = os.path.join(rengine_addon_dir, 'plugin.cfg')
        with open(plugin_config_path, 'w', encoding='utf-8') as f:
            f.write(plugin_config)

        # Create plugin script (minimal for now)
        plugin_script = """@tool
extends EditorPlugin

const RwAnalyzer = preload("res://rengine_rw.gdextension")

func _enter_tree():
    # Plugin initialization
    print("Rengine RW Plugin loaded")

func _exit_tree():
    # Plugin cleanup
    print("Rengine RW Plugin unloaded")
"""

        plugin_script_path = os.path.join(rengine_addon_dir, 'rengine_rw_plugin.gd')
        with open(plugin_script_path, 'w', encoding='utf-8') as f:
            f.write(plugin_script)

        # Create README for the generated project
        readme_content = f"""# {project_name} - IPL Map Godot Project

This Godot project was generated by Rengine CLI from IPL (Item Placement) map files.

## Features

- **IPL Map Data**: Converted from multiple IPL files
- **RenderWare Analysis**: Built-in file analysis tools for IMG, DFF, TXD, COL files
- **Interactive Demo**: Demo scene with file analysis and IPL data visualization
- **GDExtension Integration**: Full RenderWare parsing capabilities

## Getting Started

### 1. Build the Extension

First, build and copy the RenderWare extension:

```bash
./build_extension.sh
```

This requires:
- Rust and Cargo installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- The extension will be built from the Rengine workspace

### 2. Open in Godot

Open the project in Godot 4.2+:

```bash
godot project.godot
```

Or open `project.godot` directly in Godot Editor.

### 3. Run the Demo

The main scene (`demo.tscn`) provides:

- **File Analysis**: Analyze any RenderWare file format
- **IPL Data Viewer**: Interactive view of your converted map data
- **Statistics**: See counts of instances and zones from your IPL files

## Project Structure

```
{project_name}/
├── project.godot          # Godot project configuration
├── demo.tscn             # Main demo scene with analysis tools
├── ipl_combined.tscn     # Combined IPL data scene
├── scenes/               # Individual IPL file scenes
├── rengine_rw.gdextension # GDExtension configuration
├── build_extension.sh    # Build script for the extension
├── addons/rengine_rw/    # Plugin files
└── icon.svg             # Project icon
```

## RenderWare File Support

This project can analyze:

- **IMG**: GTA game archive files (.img)
- **DFF**: 3D model files (.dff)
- **TXD**: Texture dictionary files (.txd)
- **COL**: Collision data files (.col)
- **IPL**: Item placement files (.ipl)
- **IDE**: Item definition files (.ide)

## Development

To extend this project:

1. **Add more IPL files**: Place additional .ipl files in your source directory and regenerate
2. **Customize scenes**: Modify the generated .tscn files in Godot Editor
3. **Add game logic**: Extend the demo scene script with your gameplay features
4. **Import assets**: Add DFF models, TXD textures, and other game assets

## Technical Details

- **IPL Conversion**: Map instances become Node3D objects with metadata
- **Zone System**: IPL zones become Area3D objects with collision shapes
- **GDExtension**: Provides fast Rust-based file parsing
- **Godot 4.2+**: Requires Godot 4.2 or later for GDExtension support

## Troubleshooting

**Extension won't load?**
- Run `./build_extension.sh` to build the latest extension
- Check that you have the correct Godot version (4.2+)
- Ensure the extension library is in the project root

**IPL data not showing?**
- Check the Godot console for parsing errors
- Verify IPL files are in the correct format
- Look at individual scenes in the `scenes/` directory

**Performance issues?**
- The demo loads all IPL data at once - consider streaming for large maps
- Limit instances/zones displayed in the demo scene
"""

        readme_path = os.path.join(output_dir, 'README.md')
        with open(readme_path, 'w', encoding='utf-8') as f:
            f.write(readme_content)

    def _generate_demo_godot_scene(self, all_ipl_data: Dict[str, Dict], project_name: str) -> str:
        """Generate a demo Godot scene with IPL data and RW analysis capabilities"""

        scene_lines = []
        scene_lines.append('[gd_scene load_steps=3 format=3]')
        scene_lines.append('')

        # Add script subresource
        scene_lines.append('[sub_resource type="GDScript" id="1"]')
        scene_lines.append('script/source = """')
        scene_lines.append('extends Node')
        scene_lines.append('')
        scene_lines.append(f'# {project_name} - IPL Map Demo with RenderWare Analysis')
        scene_lines.append(f'# Generated from {len(all_ipl_data)} IPL files')
        scene_lines.append('')
        scene_lines.append('@onready var rw_analyzer = $RwAnalyzer')
        scene_lines.append('@onready var output_label = $UI/VBoxContainer/OutputLabel')
        scene_lines.append('@onready var ipl_stats_label = $UI/VBoxContainer/IPLStatsLabel')
        scene_lines.append('')
        scene_lines.append('func _ready():')
        scene_lines.append('\t# Connect button signals')
        scene_lines.append('\t$UI/VBoxContainer/AnalyzeFileButton.connect("pressed", Callable(self, "_on_analyze_file_pressed"))')
        scene_lines.append('\t$UI/VBoxContainer/ParseImgButton.connect("pressed", Callable(self, "_on_parse_img_pressed"))')
        scene_lines.append('\t$UI/VBoxContainer/ParseDffButton.connect("pressed", Callable(self, "_on_parse_dff_pressed"))')
        scene_lines.append('\t$UI/VBoxContainer/ClearButton.connect("pressed", Callable(self, "_on_clear_pressed"))')
        scene_lines.append('\t$UI/VBoxContainer/ViewIPLButton.connect("pressed", Callable(self, "_on_view_ipl_pressed"))')
        scene_lines.append('\t')
        scene_lines.append('\t# Display IPL statistics')
        scene_lines.append('\t_update_ipl_stats()')
        scene_lines.append('')
        scene_lines.append('func _on_analyze_file_pressed():')
        scene_lines.append('\tvar dialog = FileDialog.new()')
        scene_lines.append('\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE')
        scene_lines.append('\tdialog.access = FileDialog.ACCESS_FILESYSTEM')
        scene_lines.append('\tdialog.connect("file_selected", Callable(self, "_on_file_selected_for_analysis"))')
        scene_lines.append('\tadd_child(dialog)')
        scene_lines.append('\tdialog.popup_centered()')
        scene_lines.append('')
        scene_lines.append('func _on_file_selected_for_analysis(path: String):')
        scene_lines.append('\tvar result = rw_analyzer.analyze_file(path)')
        scene_lines.append('\toutput_label.text = "File Analysis:\\n" + JSON.stringify(result, "  ")')
        scene_lines.append('')
        scene_lines.append('func _on_parse_img_pressed():')
        scene_lines.append('\tvar dialog = FileDialog.new()')
        scene_lines.append('\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE')
        scene_lines.append('\tdialog.access = FileDialog.ACCESS_FILESYSTEM')
        scene_lines.append('\tdialog.filters = PackedStringArray(["*.img ; IMG Archive Files"])')
        scene_lines.append('\tdialog.connect("file_selected", Callable(self, "_on_img_file_selected"))')
        scene_lines.append('\tadd_child(dialog)')
        scene_lines.append('\tdialog.popup_centered()')
        scene_lines.append('')
        scene_lines.append('func _on_img_file_selected(path: String):')
        scene_lines.append('\tvar result = rw_analyzer.parse_img_archive(path)')
        scene_lines.append('\toutput_label.text = "IMG Archive:\\n" + JSON.stringify(result, "  ")')
        scene_lines.append('')
        scene_lines.append('func _on_parse_dff_pressed():')
        scene_lines.append('\tvar dialog = FileDialog.new()')
        scene_lines.append('\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE')
        scene_lines.append('\tdialog.access = FileDialog.ACCESS_FILESYSTEM')
        scene_lines.append('\tdialog.filters = PackedStringArray(["*.dff ; DFF Model Files"])')
        scene_lines.append('\tdialog.connect("file_selected", Callable(self, "_on_dff_file_selected"))')
        scene_lines.append('\tadd_child(dialog)')
        scene_lines.append('\tdialog.popup_centered()')
        scene_lines.append('')
        scene_lines.append('func _on_dff_file_selected(path: String):')
        scene_lines.append('\tvar result = rw_analyzer.parse_dff_model(path)')
        scene_lines.append('\toutput_label.text = "DFF Model:\\n" + JSON.stringify(result, "  ")')
        scene_lines.append('')
        scene_lines.append('func _on_clear_pressed():')
        scene_lines.append('\toutput_label.text = "Select a file to analyze..."')
        scene_lines.append('\t_update_ipl_stats()')
        scene_lines.append('')
        scene_lines.append('func _on_view_ipl_pressed():')
        scene_lines.append('\tget_tree().change_scene_to_file("res://ipl_combined.tscn")')
        scene_lines.append('')
        scene_lines.append('func _update_ipl_stats():')
        scene_lines.append('\tvar total_instances = 0')
        scene_lines.append('\tvar total_zones = 0')
        scene_lines.append('\tvar file_count = 0')
        scene_lines.append('\t')
        scene_lines.append('\tfor ipl_node in $IPLData.get_children():')
        scene_lines.append('\t\tfile_count += 1')
        scene_lines.append('\t\tfor child in ipl_node.get_children():')
        scene_lines.append('\t\t\tif child is Node3D and not child.name.ends_with("_zone"):')
        scene_lines.append('\t\t\t\ttotal_instances += 1')
        scene_lines.append('\t\t\telif child is Area3D:')
        scene_lines.append('\t\t\t\ttotal_zones += 1')
        scene_lines.append('\t')
        scene_lines.append('\tipl_stats_label.text = "IPL Data: %d files, %d instances, %d zones" % [file_count, total_instances, total_zones]')
        scene_lines.append('"""')
        scene_lines.append('')

        # Root node
        scene_lines.append(f'[node name="{project_name}Demo" type="Node"]')
        scene_lines.append('script = SubResource("1")')
        scene_lines.append('')

        # RwAnalyzer node
        scene_lines.append('[node name="RwAnalyzer" type="RwAnalyzer" parent="."]')
        scene_lines.append('')

        # IPL Data container
        scene_lines.append('[node name="IPLData" type="Node" parent="."]')
        scene_lines.append('')

        # Add IPL data as child nodes
        ipl_index = 0
        total_instances = 0

        for ipl_filename, ipl_data in all_ipl_data.items():
            ipl_name = os.path.splitext(ipl_filename)[0]
            scene_lines.append(f'[node name="{ipl_name}" type="Node" parent="IPLData"]')
            scene_lines.append('')

            # Add instances from this IPL (limited for demo)
            if 'sections' in ipl_data and 'inst' in ipl_data['sections']:
                instance_count = 0
                for entry in ipl_data['sections']['inst']['entries'][:50]:  # Limit to 50 instances for demo
                    if 'model_name' in entry:
                        total_instances += 1
                        instance_count += 1
                        node_name = f"{entry['model_name']}_{instance_count}"
                        scene_lines.append(f'[node name="{node_name}" type="Node3D" parent="{ipl_name}"]')
                        scene_lines.append(f'transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {entry["pos_x"]:.6f}, {entry["pos_y"]:.6f}, {entry["pos_z"]:.6f})')

                        # Add metadata
                        scene_lines.append(f'metadata/model_id = {entry["id"]}')
                        scene_lines.append(f'metadata/model_name = "{entry["model_name"]}"')
                        scene_lines.append(f'metadata/interior = {entry["interior"]}')
                        scene_lines.append(f'metadata/lod = {entry["lod"]}')
                        scene_lines.append(f'metadata/ipl_source = "{ipl_filename}"')
                        scene_lines.append('')

            # Add zones from this IPL (limited)
            if 'sections' in ipl_data and 'zone' in ipl_data['sections']:
                zone_count = 0
                for entry in ipl_data['sections']['zone']['entries'][:20]:  # Limit to 20 zones for demo
                    if 'name' in entry:
                        center_x = (entry['min_x'] + entry['max_x']) / 2
                        center_y = (entry['min_y'] + entry['max_y']) / 2
                        center_z = (entry['min_z'] + entry['max_z']) / 2
                        size_x = abs(entry['max_x'] - entry['min_x'])
                        size_y = abs(entry['max_y'] - entry['min_y'])
                        size_z = abs(entry['max_z'] - entry['max_z'])

                        zone_name = f"{entry['name']}_zone"
                        scene_lines.append(f'[node name="{zone_name}" type="Area3D" parent="{ipl_name}"]')
                        scene_lines.append(f'transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {center_x:.6f}, {center_y:.6f}, {center_z:.6f})')

                        # Add metadata
                        scene_lines.append(f'metadata/zone_type = {entry["type"]}')
                        scene_lines.append(f'metadata/level = "{entry["level"]}"')
                        scene_lines.append(f'metadata/text = "{entry["text"]}"')
                        scene_lines.append(f'metadata/ipl_source = "{ipl_filename}"')

                        # Add CollisionShape3D as child
                        scene_lines.append(f'[node name="CollisionShape3D" type="CollisionShape3D" parent="{zone_name}"]')
                        scene_lines.append('[sub_resource type="BoxShape3D" id="2"]')
                        scene_lines.append(f'size = Vector3({size_x:.6f}, {size_y:.6f}, {size_z:.6f})')
                        scene_lines.append('shape = SubResource("2")')
                        scene_lines.append('')

        # UI Layer
        scene_lines.append('[node name="UI" type="CanvasLayer" parent="."]')
        scene_lines.append('')
        scene_lines.append('[node name="VBoxContainer" type="VBoxContainer" parent="UI"]')
        scene_lines.append('anchors_preset = 15')
        scene_lines.append('anchor_right = 1.0')
        scene_lines.append('anchor_bottom = 1.0')
        scene_lines.append('grow_horizontal = 2')
        scene_lines.append('grow_vertical = 2')
        scene_lines.append('')

        # UI Elements
        scene_lines.append('[node name="TitleLabel" type="Label" parent="UI/VBoxContainer"]')
        scene_lines.append('layout_mode = 2')
        scene_lines.append(f'text = "{project_name} - IPL Map Demo"')
        scene_lines.append('horizontal_alignment = 1')
        scene_lines.append('')

        scene_lines.append('[node name="IPLStatsLabel" type="Label" parent="UI/VBoxContainer"]')
        scene_lines.append('layout_mode = 2')
        scene_lines.append('text = "IPL Data: Loading..."')
        scene_lines.append('horizontal_alignment = 1')
        scene_lines.append('')

        scene_lines.append('[node name="HSeparator" type="HSeparator" parent="UI/VBoxContainer"]')
        scene_lines.append('layout_mode = 2')
        scene_lines.append('')

        scene_lines.append('[node name="SectionLabel" type="Label" parent="UI/VBoxContainer"]')
        scene_lines.append('layout_mode = 2')
        scene_lines.append('text = "RenderWare File Analysis"')
        scene_lines.append('horizontal_alignment = 1')
        scene_lines.append('')

        scene_lines.append('[node name="AnalyzeFileButton" type="Button" parent="UI/VBoxContainer"]')
        scene_lines.append('layout_mode = 2')
        scene_lines.append('text = "Analyze Any File"')
        scene_lines.append('')

        scene_lines.append('[node name="ParseImgButton" type="Button" parent="UI/VBoxContainer"]')
        scene_lines.append('layout_mode = 2')
        scene_lines.append('text = "Parse IMG Archive"')
        scene_lines.append('')

        scene_lines.append('[node name="ParseDffButton" type="Button" parent="UI/VBoxContainer"]')
        scene_lines.append('layout_mode = 2')
        scene_lines.append('text = "Parse DFF Model"')
        scene_lines.append('')

        scene_lines.append('[node name="HSeparator2" type="HSeparator" parent="UI/VBoxContainer"]')
        scene_lines.append('layout_mode = 2')
        scene_lines.append('')

        scene_lines.append('[node name="ViewIPLButton" type="Button" parent="UI/VBoxContainer"]')
        scene_lines.append('layout_mode = 2')
        scene_lines.append('text = "View IPL Map Data"')
        scene_lines.append('')

        scene_lines.append('[node name="ClearButton" type="Button" parent="UI/VBoxContainer"]')
        scene_lines.append('layout_mode = 2')
        scene_lines.append('text = "Clear Output"')
        scene_lines.append('')

        scene_lines.append('[node name="OutputLabel" type="Label" parent="UI/VBoxContainer"]')
        scene_lines.append('custom_minimum_size = Vector2(0, 150)')
        scene_lines.append('layout_mode = 2')
        scene_lines.append('size_flags_vertical = 3')
        scene_lines.append('text = "Welcome to Rengine IPL Demo!\\n\\nUse the buttons above to analyze RenderWare files or view your IPL map data."')
        scene_lines.append('autowrap_mode = 2')

        return '\n'.join(scene_lines)

    def _generate_combined_godot_scene(self, all_ipl_data: Dict[str, Dict], project_name: str) -> str:
        """Generate a combined Godot scene with all IPL data"""

        scene_lines = []
        scene_lines.append('[gd_scene load_steps=2 format=3]')
        scene_lines.append('')

        # Add script subresource
        scene_lines.append('[sub_resource type="GDScript" id="1"]')
        scene_lines.append('script/source = """')
        scene_lines.append('extends Node')
        scene_lines.append('')
        scene_lines.append(f'# {project_name} - Combined IPL Map Data')
        scene_lines.append(f'# Generated from {len(all_ipl_data)} IPL files')
        scene_lines.append('')
        scene_lines.append('func _ready():')
        scene_lines.append('\tpass')
        scene_lines.append('"""')
        scene_lines.append('')

        # Root node
        scene_lines.append(f'[node name="{project_name}" type="Node"]')
        scene_lines.append('script = SubResource("1")')
        scene_lines.append('')

        # Add nodes for each IPL file
        ipl_index = 0
        total_instances = 0

        for ipl_filename, ipl_data in all_ipl_data.items():
            ipl_name = os.path.splitext(ipl_filename)[0]
            scene_lines.append(f'[node name="{ipl_name}" type="Node" parent="."]')
            scene_lines.append('')

            # Add instances from this IPL
            if 'sections' in ipl_data and 'inst' in ipl_data['sections']:
                for entry in ipl_data['sections']['inst']['entries']:
                    if 'model_name' in entry:
                        total_instances += 1
                        node_name = f"{entry['model_name']}_{total_instances}"
                        scene_lines.append(f'[node name="{node_name}" type="Node3D" parent="{ipl_name}"]')
                        scene_lines.append(f'transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {entry["pos_x"]:.6f}, {entry["pos_y"]:.6f}, {entry["pos_z"]:.6f})')

                        # Add metadata
                        scene_lines.append(f'metadata/model_id = {entry["id"]}')
                        scene_lines.append(f'metadata/model_name = "{entry["model_name"]}"')
                        scene_lines.append(f'metadata/interior = {entry["interior"]}')
                        scene_lines.append(f'metadata/lod = {entry["lod"]}')
                        scene_lines.append(f'metadata/ipl_source = "{ipl_filename}"')
                        scene_lines.append('')

            # Add zones from this IPL
            if 'sections' in ipl_data and 'zone' in ipl_data['sections']:
                for entry in ipl_data['sections']['zone']['entries']:
                    if 'name' in entry:
                        center_x = (entry['min_x'] + entry['max_x']) / 2
                        center_y = (entry['min_y'] + entry['max_y']) / 2
                        center_z = (entry['min_z'] + entry['max_z']) / 2
                        size_x = abs(entry['max_x'] - entry['min_x'])
                        size_y = abs(entry['max_y'] - entry['min_y'])
                        size_z = abs(entry['max_z'] - entry['max_z'])

                        zone_name = f"{entry['name']}_zone"
                        scene_lines.append(f'[node name="{zone_name}" type="Area3D" parent="{ipl_name}"]')
                        scene_lines.append(f'transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {center_x:.6f}, {center_y:.6f}, {center_z:.6f})')

                        # Add metadata
                        scene_lines.append(f'metadata/zone_type = {entry["type"]}')
                        scene_lines.append(f'metadata/level = "{entry["level"]}"')
                        scene_lines.append(f'metadata/text = "{entry["text"]}"')
                        scene_lines.append(f'metadata/ipl_source = "{ipl_filename}"')

                        # Add CollisionShape3D as child
                        scene_lines.append(f'[node name="CollisionShape3D" type="CollisionShape3D" parent="{zone_name}"]')
                        scene_lines.append('[sub_resource type="BoxShape3D" id="2"]')
                        scene_lines.append(f'size = Vector3({size_x:.6f}, {size_y:.6f}, {size_z:.6f})')
                        scene_lines.append('shape = SubResource("2")')
                        scene_lines.append('')

        return '\n'.join(scene_lines)

    def _parse_ipl_file(self, ipl_file: str) -> Dict[str, Any]:
        """Parse IPL file and extract sections"""
        try:
            # First try to read as binary IPL file
            with open(ipl_file, 'rb') as f:
                header = f.read(4)
                if header == b'bnry':
                    return self._parse_binary_ipl_file(ipl_file)
        except:
            pass

        # Fall back to text IPL parsing
        with open(ipl_file, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()

        lines = content.split('\n')
        sections = {}
        current_section = None

        for line in lines:
            line = line.strip()
            if line.startswith('#') or not line:
                continue
            if line.endswith(','):
                line = line[:-1]

            # Check for section headers
            if line in ['inst', 'zone', 'occl', 'pick', 'path', 'tcyc', 'mult', 'grge', 'enex', 'cars', 'jump']:
                current_section = line
                sections[current_section] = {'entries': [], 'count': 0}
            elif current_section and line:
                # Parse entry based on section type
                entry = self._parse_ipl_entry(current_section, line)
                if entry:
                    sections[current_section]['entries'].append(entry)
                    sections[current_section]['count'] += 1

        return {'file': ipl_file, 'sections': sections}

    def _parse_binary_ipl_file(self, ipl_file: str) -> Dict[str, Any]:
        """Parse binary IPL file (GTA SA format)"""
        import struct

        sections = {'inst': {'entries': [], 'count': 0}}

        with open(ipl_file, 'rb') as f:
            data = f.read()

        if len(data) < 8 or data[:4] != b'bnry':
            return {'file': ipl_file, 'sections': sections}

        # Read number of instances
        num_instances = struct.unpack('<I', data[4:8])[0]

        # Instances start at offset 0x4C in GTA SA IPL files (after 12 bytes of padding)
        offset = 0x4C

        for i in range(min(num_instances, 1000)):  # Limit to prevent infinite loops
            try:
                if offset + 64 > len(data):  # Need at least 64 bytes for instance + name
                    break

                # Read instance data (GTA SA binary IPL format)
                # 7 floats (28 bytes) + 3 ints (12 bytes) = 40 bytes for instance data
                instance_data = data[offset:offset+40]
                pos_x, pos_y, pos_z, rot_x, rot_y, rot_z, rot_w, model_id, interior, lod = struct.unpack('<fffffffIII', instance_data)

                # Read model name (24 bytes after instance data)
                name_offset = offset + 40
                model_name_data = data[name_offset:name_offset+24]
                model_name = model_name_data.split(b'\x00')[0].decode('ascii', errors='ignore')

                entry = {
                    'id': int(model_id),
                    'model_name': model_name,
                    'interior': int(interior),
                    'pos_x': pos_x,
                    'pos_y': pos_y,
                    'pos_z': pos_z,
                    'rot_x': rot_x,
                    'rot_y': rot_y,
                    'rot_z': rot_z,
                    'rot_w': rot_w,
                    'lod': int(lod)
                }
                sections['inst']['entries'].append(entry)
                sections['inst']['count'] += 1

                offset += 64  # Each instance is 64 bytes (40 + 24)

            except (struct.error, UnicodeDecodeError):
                break

        return {'file': ipl_file, 'sections': sections}

    def _parse_ipl_entry(self, section: str, line: str) -> Optional[Dict[str, Any]]:
        """Parse a single IPL entry based on section type"""
        try:
            parts = line.split(',')

            if section == 'inst':  # Instances
                if len(parts) >= 11:
                    return {
                        'id': int(parts[0]),
                        'model_name': parts[1].strip(),
                        'interior': int(parts[2]) if parts[2].strip() else 0,
                        'pos_x': float(parts[3]),
                        'pos_y': float(parts[4]),
                        'pos_z': float(parts[5]),
                        'rot_x': float(parts[6]),
                        'rot_y': float(parts[7]),
                        'rot_z': float(parts[8]),
                        'rot_w': float(parts[9]) if len(parts) > 9 else 1.0,
                        'lod': int(parts[10]) if len(parts) > 10 and parts[10].strip() else -1
                    }

            elif section == 'zone':  # Zones
                if len(parts) >= 10:
                    return {
                        'name': parts[0].strip(),
                        'type': int(parts[1]),
                        'min_x': float(parts[2]),
                        'min_y': float(parts[3]),
                        'min_z': float(parts[4]),
                        'max_x': float(parts[5]),
                        'max_y': float(parts[6]),
                        'max_z': float(parts[7]),
                        'level': parts[8].strip() if len(parts) > 8 else '',
                        'text': parts[9].strip() if len(parts) > 9 else ''
                    }

            # For other sections, just return the raw data
            return {'raw': parts}

        except (ValueError, IndexError):
            self.logger.warning(f"Failed to parse IPL entry: {line}")
            return None

    def _generate_godot_scene(self, ipl_data: Dict[str, Any], ipl_file: str, template_file: Optional[str] = None) -> str:
        """Generate Godot .tscn scene file content"""

        scene_lines = []
        scene_lines.append('[gd_scene load_steps=3 format=3]')
        scene_lines.append('')

        # Add script subresource
        scene_lines.append('[sub_resource type="GDScript" id="1"]')
        scene_lines.append('script/source = """')
        scene_lines.append('extends Node')
        scene_lines.append('')
        scene_lines.append('# IPL Map Data')
        scene_lines.append(f'# Original file: {os.path.basename(ipl_file)}')
        scene_lines.append('')
        scene_lines.append('func _ready():')
        scene_lines.append('\tpass')
        scene_lines.append('"""')
        scene_lines.append('')

        # Root node
        scene_lines.append('[node name="IPLMap" type="Node"]')
        scene_lines.append('script = SubResource("1")')
        scene_lines.append('')

        # Generate nodes for instances
        if 'sections' in ipl_data and 'inst' in ipl_data['sections']:
            instance_count = 0
            for entry in ipl_data['sections']['inst']['entries']:
                if 'model_name' in entry:
                    instance_count += 1
                    node_name = f"{entry['model_name']}_{instance_count}"
                    scene_lines.append(f'[node name="{node_name}" type="Node3D" parent="."]')
                    scene_lines.append(f'transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {entry["pos_x"]:.6f}, {entry["pos_y"]:.6f}, {entry["pos_z"]:.6f})')

                    # Add metadata
                    scene_lines.append(f'metadata/model_id = {entry["id"]}')
                    scene_lines.append(f'metadata/model_name = "{entry["model_name"]}"')
                    scene_lines.append(f'metadata/interior = {entry["interior"]}')
                    scene_lines.append(f'metadata/lod = {entry["lod"]}')
                    scene_lines.append('')

        # Generate nodes for zones (as Area3D)
        if 'sections' in ipl_data and 'zone' in ipl_data['sections']:
            zone_count = 0
            for entry in ipl_data['sections']['zone']['entries']:
                if 'name' in entry:
                    zone_count += 1
                    center_x = (entry['min_x'] + entry['max_x']) / 2
                    center_y = (entry['min_y'] + entry['max_y']) / 2
                    center_z = (entry['min_z'] + entry['max_z']) / 2
                    size_x = abs(entry['max_x'] - entry['min_x'])
                    size_y = abs(entry['max_y'] - entry['min_y'])
                    size_z = abs(entry['max_z'] - entry['max_z'])

                    zone_name = f"{entry['name']}_{zone_count}"
                    scene_lines.append(f'[node name="{zone_name}" type="Area3D" parent="."]')
                    scene_lines.append(f'transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {center_x:.6f}, {center_y:.6f}, {center_z:.6f})')

                    # Add metadata
                    scene_lines.append(f'metadata/zone_type = {entry["type"]}')
                    scene_lines.append(f'metadata/level = "{entry["level"]}"')
                    scene_lines.append(f'metadata/text = "{entry["text"]}"')

                    # Add CollisionShape3D as child
                    scene_lines.append(f'[node name="CollisionShape3D" type="CollisionShape3D" parent="{zone_name}"]')
                    scene_lines.append('[sub_resource type="BoxShape3D" id="2"]')
                    scene_lines.append(f'size = Vector3({size_x:.6f}, {size_y:.6f}, {size_z:.6f})')
                    scene_lines.append('shape = SubResource("2")')
                    scene_lines.append('')

        return '\n'.join(scene_lines)

