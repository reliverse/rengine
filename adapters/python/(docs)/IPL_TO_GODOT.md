# IPL to Godot Converter

This document explains how to use the IPL (Item Placement) to Godot converter in Rengine CLI, which converts GTA map files into Godot game engine projects.

## Overview

The IPL-to-Godot converter transforms IPL files (which contain 3D object placement data from GTA games) into complete Godot 4.2+ projects with:

- **Interactive scenes** with all map objects positioned correctly
- **RenderWare file analysis tools** for IMG, DFF, TXD, COL, and IPL files
- **GDExtension integration** for fast file parsing
- **Demo interface** for exploring map data

## Prerequisites

- Python 3.8+
- Rengine CLI dependencies installed
- Godot 4.2+ (for running generated projects)
- Rust toolchain (automatically installed via `bun ipl-to-godot:setup:rust`)

## Installation

The converter is part of the Rengine CLI. Ensure dependencies are installed:

```bash
cd adapters/python
bun app-cli:install  # Install CLI dependencies using uv
```

## Usage

### Command Syntax

```bash
python run_cli.py ipl-to-godot [input] --output [output_path] [options]
```

### Parameters

- `input`: Path to IPL file or directory containing IPL files
- `--output, -o`: Output path (directory for projects, .tscn file for scenes)
- `--template`: Optional template scene file to extend
- `--project-name`: Project name (default: "IPLMap")

### Examples

#### Convert Single IPL File to Scene

```bash
python run_cli.py ipl-to-godot data/countryw.ipl --output scenes/countryw.tscn
```

#### Convert GTA SA Resources Directory to Full Project

```bash
python run_cli.py ipl-to-godot /path/to/gta-sa/resources \
    --output ~/GodotProjects/GTASA_Map \
    --project-name "GTA San Andreas Map"
```

## Quick Start Scripts

The Rengine Python adapters package includes convenient npm/bun scripts for IPL-to-Godot operations. These scripts simplify common workflows and provide shortcuts for conversion tasks.

### Main Workflow Scripts

- **`bun ipl:workflow`** - Complete conversion workflow: converts GTA SA IPL files to Godot project
- **`bun ipl:quickstart`** - Shows information and setup instructions for getting started

### Core Conversion Scripts

- **`bun ipl-to-godot:help`** - Shows the IPL-to-Godot command help and usage information
- **`bun ipl-to-godot:convert:gta-sa`** - Converts the entire GTA SA resources directory to a Godot project in `/home/blefnk/B/R/reliverse/rengine/examples/godot/gta-sa-map`
- **`bun ipl-to-godot:convert:file`** - Shows usage instructions for converting individual IPL files (requires additional parameters)
- **`bun ipl-to-godot:test`** - Tests IPL conversion by finding and converting one IPL file from GTA SA resources
- **`bun ipl-to-godot:list`** - Lists all IPL files found in the GTA SA resources directory
- **`bun ipl-to-godot:analyze`** - Analyzes the `countrye.ipl` file structure and shows detailed information

### Project Management Scripts

- **`bun ipl-to-godot:clean:projects`** - Removes the generated GTA SA map project in the `examples/godot` directory
- **`bun ipl-to-godot:build:extension`** - Shows instructions for building the RenderWare GDExtension (must be run from generated project directory)

### Setup and Information Scripts

- **`bun ipl-to-godot:setup:godot`** - Shows download link and installation instructions for Godot 4.2+
- **`bun ipl-to-godot:setup:rust`** - Automatically installs Rust toolchain if not present (required for GDExtension)
- **`bun ipl-to-godot:setup:rust:manual`** - Shows manual Rust installation instructions
- **`bun ipl-to-godot:info`** - Displays general information about the IPL converter and links to documentation

### Usage Examples

```bash
# Get started with IPL conversion
bun ipl:quickstart

# Complete workflow: convert IPL files
bun ipl:workflow

# Convert GTA SA IPL files to Godot project
bun ipl-to-godot:convert:gta-sa
# Creates project at: /home/blefnk/B/R/reliverse/rengine/examples/godot/gta-sa-map

# List available IPL files
bun ipl-to-godot:list

# Analyze IPL file structure
bun ipl-to-godot:analyze

# Test conversion with one file
bun ipl-to-godot:test

# Clean up generated project
bun ipl-to-godot:clean:projects

# Get help and usage information
bun ipl-to-godot:help
```

### Script Dependencies

All IPL-to-Godot scripts require:
- The `app-cli` dependencies to be installed (`bun app-cli:install`)
- Access to GTA SA resources at `/home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa`
- For full project conversion: Godot 4.2+ and Rust toolchain

### Development Setup

To set up the full development environment including automatic Rust installation:

```bash
cd adapters/python
bun dev:setup          # Install all Python adapters
bun ipl-to-godot:setup:rust  # Install Rust toolchain (if needed)
```

### Alternative Rust Installation Methods

If you prefer manual control or need specific Rust versions:

#### Quick Install (Recommended)
```bash
bun ipl-to-godot:setup:rust  # Automated installation
```

#### Manual Install
```bash
bun ipl-to-godot:setup:rust:manual  # Shows installation commands
# Then run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Using System Package Managers
```bash
# Ubuntu/Debian
sudo apt install cargo rustc

# Fedora/RHEL
sudo dnf install rust cargo

# macOS with Homebrew
brew install rust

# Arch Linux
sudo pacman -S rust
```

### Customizing Scripts

To modify the default paths or behavior, you can:
1. Edit the script commands in `package.json` (scripts have been cleaned up to remove redundancy)
2. Create custom scripts for different IPL sources
3. Add new scripts for specific conversion workflows

The `package.json` now contains only essential scripts without duplicates, making customization easier.

## Generated Project Structure

When converting a directory of IPL files, the following structure is created:

```
GTASA_Map/
├── project.godot                 # Godot project configuration
├── demo.tscn                     # Interactive demo scene with analysis UI
├── ipl_combined.tscn             # Combined scene with all IPL data
├── scenes/                       # Individual scenes per IPL file
│   ├── countrye.tscn
│   ├── countrys.tscn
│   └── ...
├── rengine_rw.gdextension        # GDExtension configuration
├── build_extension.sh            # Build script for RenderWare extension
├── addons/rengine_rw/            # Plugin directory
│   ├── plugin.cfg
│   └── rengine_rw_plugin.gd
├── icon.svg                      # Project icon
├── .gitignore                    # Git ignore file
└── README.md                     # Project documentation
```

## IPL Data Conversion

### Instances (3D Objects)

Each IPL instance is converted to a `Node3D` with:

- **Position**: World coordinates from IPL data
- **Rotation**: Converted from quaternion to Godot transform
- **Metadata**: All original IPL properties preserved
  - `model_id`: Numeric model identifier
  - `model_name`: Model name string
  - `interior`: Interior number
  - `lod`: Level of detail value
  - `ipl_source`: Source IPL filename

### Zones (Areas)

IPL zones become `Area3D` nodes with:

- **Position**: Center point of zone bounds
- **Collision Shape**: `BoxShape3D` matching zone dimensions
- **Metadata**:
  - `zone_type`: Zone type number
  - `level`: Zone level string
  - `text`: Zone description text
  - `ipl_source`: Source IPL filename

## Setting Up Generated Projects

### 1. Build the Extension

Navigate to your project directory and run:

```bash
cd GTASA_Map
./build_extension.sh
```

This builds the RenderWare parsing extension. The build script will automatically detect if Rust is installed. If not, run:

```bash
# From the adapters/python directory
bun ipl-to-godot:setup:rust
```

Requirements:
- Rust toolchain (automatically installed if missing)
- Cargo package manager (included with Rust)

### 2. Open in Godot

```bash
godot project.godot
```

Or open `project.godot` directly in Godot Editor.

### 3. Run the Demo

The `demo.tscn` scene provides:

- **File Analysis**: Browse and analyze RenderWare files
- **IPL Data Viewer**: Interactive exploration of map objects
- **Statistics**: Object and zone counts
- **Navigation**: Switch between different IPL scenes

## Advanced Usage

### Custom Project Templates

Use a template scene as a base:

```bash
python run_cli.py ipl-to-godot map.ipl \
    --output custom_map.tscn \
    --template templates/base_scene.tscn
```

### Batch Processing Multiple Directories

Process multiple IPL collections:

```bash
# Convert multiple directories
for dir in sa_resources vc_resources; do
    python run_cli.py ipl-to-godot "$dir" \
        --output "GodotProjects/${dir}_Map" \
        --project-name "${dir^^} Map"
done
```

## IPL File Format Support

The converter handles multiple IPL formats:

### Text IPL Files (.ipl)
Standard GTA IPL format with comma-separated values:

```
# IPL format
# Comment lines
inst
123, model_name, 0, 100.0, 200.0, 300.0, 0.0, 0.0, 0.0, 1.0, -1
zone
zone_name, 0, -100.0, -100.0, -100.0, 100.0, 100.0, 100.0, level_name, description
end
```

### Binary IPL Files (.ipl)
GTA SA binary format with compiled instance data.

## RenderWare Integration

Generated projects include full RenderWare file analysis capabilities:

### Supported File Types
- **IMG**: Game archive files containing models and textures
- **DFF**: 3D model files with geometry and materials
- **TXD**: Texture dictionary files
- **COL**: Collision mesh files
- **IPL**: Item placement files (already converted)

### Analysis Features
- **File Information**: Size, version, chunk counts
- **Structure Analysis**: Detailed chunk breakdown
- **Content Extraction**: Export textures, models, and data
- **Batch Processing**: Analyze entire directories

## Performance Considerations

### Large Maps
For maps with thousands of objects:

- **Demo Scene Limits**: Limited to ~50 instances per IPL for demo performance
- **Combined Scene**: Contains ALL data for complete maps
- **Individual Scenes**: Split by IPL file for manageable chunks

### Memory Usage
- Binary IPL parsing is memory-efficient
- Text IPL parsing loads entire files into memory
- Large texture extractions may require significant RAM

## Troubleshooting

### Common Issues

#### "No IPL files found"
- Verify the input directory contains `.ipl` files
- Check file permissions
- Ensure IPL files are not corrupted

#### "Extension failed to load"
- Run `./build_extension.sh` manually
- If Rust is missing, run: `bun ipl-to-godot:setup:rust`
- Verify Rust toolchain installation: `rustc --version`
- Check Godot version (requires 4.2+)

#### "Godot project won't open"
- Ensure `project.godot` is in the project root
- Check Godot version compatibility
- Verify all required files were generated

#### "IPL data not displaying"
- Check Godot console for parsing errors
- Verify IPL file format compatibility
- Look at individual scenes in `scenes/` directory

### Debug Mode

Enable verbose output for troubleshooting:

```bash
python run_cli.py ipl-to-godot input.ipl --output output.tscn --verbose
```

## Technical Details

### Coordinate System
- IPL coordinates are preserved as-is
- GTA uses left-handed coordinate system
- Godot uses right-handed (Y-up) system
- No automatic coordinate conversion (maintains original positioning)

### Data Preservation
- All IPL metadata is preserved in Godot node metadata
- Original instance rotations converted from quaternion to Euler
- Zone boundaries converted to collision shapes
- File relationships maintained through source tracking

### GDExtension Architecture
- **Rust Core**: High-performance RenderWare parsing
- **Godot Integration**: Native Godot node types
- **Cross-platform**: Linux, Windows, macOS support
- **Thread-safe**: Async file operations where possible

## Integration with Game Development

### Extending Generated Projects

1. **Add Gameplay Logic**: Attach scripts to generated nodes
2. **Import Assets**: Add DFF models and TXD textures
3. **Create Navigation**: Generate navigation meshes from collision data
4. **Add Lighting**: Place lights based on IPL zone data
5. **Implement LOD**: Use IPL LOD values for level-of-detail systems

### Asset Pipeline Integration

```gdscript
# Example: Load DFF model for IPL instance
func load_model_for_instance(node: Node3D):
    var model_name = node.get_meta("model_name")
    var dff_path = "res://models/" + model_name + ".dff"

    # Use GDExtension to load and convert DFF
    var model_data = RwAnalyzer.parse_dff_file(dff_path)
    # Convert to Godot MeshInstance3D
    var mesh_instance = create_mesh_from_dff(model_data)
    node.add_child(mesh_instance)
```

## Examples and Use Cases

### GTA Map Recreation
Convert entire GTA game maps for:
- **Remakes**: Faithful recreation with modern graphics
- **Mods**: Base for custom map modifications
- **Analysis**: Study original game level design
- **Education**: Learn game development with real examples

### Custom Game Worlds
Use IPL data as a base for:
- **Level Prototyping**: Quick map layout testing
- **Asset Placement**: Automated object positioning
- **Zone Systems**: Area-based gameplay mechanics
- **Navigation**: Pathfinding setup

## Contributing

### Adding IPL Format Support
1. Extend the `IPLToGodotConverter` class
2. Add new parsing methods for format variants
3. Update the command-line interface
4. Add comprehensive tests

### Improving Performance
- Implement streaming for large IPL files
- Add multithreaded conversion
- Optimize Godot scene generation
- Cache frequently used assets

## Related Documentation

- [Rengine CLI Overview](../README.md)
- [RenderWare File Formats](../../docs/renderware-formats.md)
- [Godot GDExtension Guide](https://docs.godotengine.org/en/stable/development/cpp/custom_gdextension.html)
- [GTA IPL Format Specification](../../docs/gta-file-formats/ipl-format.md)

## License

This converter is part of the Rengine project and is licensed under GPL-3.0.