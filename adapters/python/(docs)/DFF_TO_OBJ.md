# DFF to OBJ Converter

This document explains how to use the DFF (RenderWare) to OBJ converter in Rengine CLI, which converts GTA 3D model files to OBJ format for use in modern 3D applications.

## Overview

The DFF-to-OBJ converter transforms DFF files (which contain 3D model data from GTA games) into OBJ format with:

- **Mesh geometry** with vertices, normals, and UV coordinates
- **Material definitions** in MTL format
- **Texture extraction** from associated TXD files
- **Batch processing** for entire model collections
- **Cross-platform compatibility** (Linux, Windows, macOS)
- **GTA SA CSV integration** for accurate model names and automatic TXD detection

## Prerequisites

- Python 3.8+
- Rengine CLI dependencies installed
- PIL/Pillow (for texture extraction)
- Access to DFF and TXD files

## Installation

The converter is part of the Rengine CLI. Ensure dependencies are installed:

```bash
cd adapters/python
bun app-cli:install  # Install CLI dependencies using uv
```

## Usage

### Command Syntax

```bash
python run_cli.py dff-to-obj [input_file] --output [output_file] [options]
```

### Parameters

- `input_file`: Path to DFF file to convert
- `--output, -o`: Output OBJ file path (required)
- `--txd`: Optional TXD file for materials and textures
- `--no-csv`: Disable loading of GTA SA CSV data (enabled by default)

### Examples

#### Convert Single DFF File

```bash
python run_cli.py dff-to-obj model.dff --output model.obj
```

#### Convert DFF with TXD Materials

```bash
python run_cli.py dff-to-obj model.dff --output model.obj --txd model.txd
```

#### Convert DFF with Automatic TXD Detection (using CSV)

```bash
python run_cli.py dff-to-obj model.dff --output model.obj
# Automatically finds TXD file using GTA SA CSV data
```

#### Convert DFF without CSV Data

```bash
python run_cli.py dff-to-obj model.dff --output model.obj --no-csv
# Uses only filename-based detection
```

#### Convert GTA SA Model

```bash
python run_cli.py dff-to-obj /home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa/models/gta3.img/landstal.dff \
    --output landstal.obj \
    --txd /home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa/models/gta3.img/landstal.txd
```

## Quick Start Scripts

The Rengine Python adapters package includes convenient npm/bun scripts for DFF-to-OBJ operations. These scripts simplify common workflows and provide shortcuts for conversion tasks.

### Main Workflow Scripts

- **`bun dff:workflow`** - Complete conversion workflow: converts all GTA SA DFF files to OBJ format (skips existing files)
- **`bun dff:quickstart`** - Shows information and PIL/Pillow setup instructions

### Core Conversion Scripts

- **`bun dff-to-obj:help`** - Shows the DFF-to-OBJ command help and usage information
- **`bun dff-to-obj:convert:gta-sa`** - Converts first 50 DFF files from GTA SA resources to OBJ format in `/home/blefnk/B/R/reliverse/rengine/examples/obj/gta-sa-models` (skips existing files)
- **`bun dff-to-obj:convert:file`** - Shows usage instructions for converting individual DFF files (requires additional parameters)
- **`bun dff-to-obj:test`** - Tests DFF conversion by finding and converting one small DFF file from GTA SA resources
- **`bun dff-to-obj:list`** - Lists first 20 DFF files found in the GTA SA resources directory
- **`bun dff-to-obj:analyze`** - Analyzes a sample DFF file structure using the CLI analyzer

### Project Management Scripts

- **`bun dff-to-obj:clean:models`** - Removes the generated OBJ models in the `examples/obj` directory

### Setup and Information Scripts

- **`bun dff-to-obj:setup:pillow`** - Shows PIL/Pillow installation instructions
- **`bun dff-to-obj:info`** - Displays general information about the DFF converter and links to documentation

### Usage Examples

```bash
# Get started with DFF conversion
bun dff:quickstart

# Complete workflow: convert all DFF files (skips existing)
bun dff:workflow

# Convert first 50 GTA SA DFF files to OBJ format (skips existing)
bun dff-to-obj:convert:gta-sa
# Creates models at: /home/blefnk/B/R/reliverse/rengine/examples/obj/gta-sa-models

# List first 20 available DFF files
bun dff-to-obj:list

# Analyze DFF file structure with CLI analyzer
bun dff-to-obj:analyze

# Test conversion with one small file
bun dff-to-obj:test

# Clean up generated models
bun dff-to-obj:clean:models

# Get help and usage information
bun dff-to-obj:help
```

### Script Dependencies

All DFF-to-OBJ scripts require:
- The `app-cli` dependencies to be installed (`bun app-cli:install`)
- Access to GTA SA resources at `/home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa`
- PIL/Pillow for texture extraction (optional but recommended)

### Development Setup

To set up the full development environment:

```bash
cd adapters/python
bun dev:setup          # Install all Python adapters
bun dff-to-obj:setup:pillow  # Install PIL/Pillow if needed
```

### PIL/Pillow Installation Methods

#### Quick Install (Recommended)
```bash
# Using uv (recommended)
cd adapters/python/app-cli
uv add pillow

# Or using pip
pip install pillow
```

#### System Package Managers
```bash
# Ubuntu/Debian
sudo apt install python3-pil

# Fedora/RHEL
sudo dnf install python3-pillow

# macOS with Homebrew
brew install pillow

# Arch Linux
sudo pacman -S python-pillow
```

## Generated Output Structure

When converting a DFF file, the following files are created:

```
model.obj                 # Main OBJ geometry file
model.mtl                 # Material definitions (if TXD provided)
texture1.png              # Extracted textures (if TXD provided)
texture2.png              # Additional textures...
```

## GTA SA CSV Integration

The converter automatically loads model information from `gta-sa.csv` (located at `/home/blefnk/Documents/reliverse/rengine/preloaded/gta-sa.csv`) to provide more accurate conversions:

### CSV-Enhanced Features

- **Accurate Model Names**: Uses official GTA SA model names instead of DFF filenames
- **Automatic TXD Detection**: Finds corresponding TXD files without manual specification
- **Model Metadata**: Includes collision, animation, and bounding box information
- **Batch Processing**: Enables intelligent processing of large model collections

### CSV Data Sources

The CSV file contains comprehensive model information including:
- **Model IDs**: Unique identifiers for each model
- **Names**: Official model names used in the game
- **TXD Files**: Associated texture dictionary files
- **Collision Data**: Whether models have collision geometry
- **Animation Flags**: Whether models contain animations
- **Bounding Boxes**: Model dimensions and radius information

### Automatic TXD Detection

When CSV data is available and no TXD file is specified, the converter automatically:
1. Looks up the model in the CSV database
2. Finds the associated TXD filename
3. Searches for the TXD file in the same directory as the DFF file
4. Loads textures and materials automatically

### Example CSV Integration

```bash
# CSV data for landstal.dff:
# id: 400, name: landstal, materialsFile: vehicle.txd

# Conversion with automatic TXD detection:
python run_cli.py dff-to-obj landstal.dff --output landstal.obj
# Result: Uses "landstal" as object name and auto-loads vehicle.txd
```

### Disabling CSV Support

To disable CSV integration and use only filename-based detection:

```bash
python run_cli.py dff-to-obj model.dff --output model.obj --no-csv
```

## DFF Data Conversion

### Geometry Conversion

Each DFF atomic (mesh) is converted to OBJ format with:

- **Vertices**: XYZ coordinates transformed to right-handed coordinate system
- **Normals**: Surface normals for lighting calculations
- **UV Coordinates**: Texture mapping coordinates (flipped V for OBJ compatibility)
- **Faces**: Triangle definitions with vertex/normal/UV indices

### Material Conversion

When TXD files are provided:

- **MTL File**: Material definitions with diffuse/specular properties
- **Texture Maps**: Diffuse textures extracted as PNG files
- **Material Names**: Consistent naming for easy identification

### Texture Extraction

TXD textures are extracted with:

- **RGBA Format**: Full color and alpha channel support
- **Mipmap Support**: Highest quality level extracted
- **PNG Format**: Lossless compression for quality preservation

## Batch Processing

### Converting Multiple DFF Files

Process entire collections of DFF files:

```bash
# Convert all DFF files in a directory
find /path/to/models -name "*.dff" -exec python run_cli.py dff-to-obj {} --output {}.obj \;

# Convert with TXD files (assuming same basename)
find /path/to/models -name "*.dff" -exec bash -c '
  dff="$1"
  base="${dff%.dff}"
  python run_cli.py dff-to-obj "$dff" --output "${base}.obj" --txd "${base}.txd"
' _ {} \;
```

### GTA SA Model Conversion

Convert the first 50 DFF files from the GTA SA model collection:

```bash
bun dff-to-obj:convert:gta-sa
```

This creates the following structure (for successfully converted models):

```
examples/obj/gta-sa-models/
├── vehicles/           # Vehicle models
│   ├── landstal.obj
│   ├── landstal.mtl
│   └── landstal.png
├── ped/               # Character models
├── weapons/           # Weapon models
└── misc/              # Miscellaneous models
```

### Skip Existing Files

The batch conversion automatically skips files that have already been converted. This allows you to:

- **Resume interrupted conversions** without re-processing completed files
- **Update partial conversions** by running the same command multiple times
- **Incremental processing** for large datasets
- **Safe re-runs** without risk of data loss

When a conversion runs, you'll see output like:
```
Conversion complete: 25 / 30 files (5 skipped, 0 failed)
```

This means 25 new files were converted, 5 already existed and were skipped, and 0 failed.

## Technical Details

### Coordinate System
- DFF uses left-handed coordinate system
- OBJ conversion transforms to right-handed system
- Y-axis remains vertical
- UV coordinates flipped for proper texture mapping

### Material Properties
- Basic Phong shading model
- Specular exponent (Ns): 96.078431
- Ambient color (Ka): 1.0, 1.0, 1.0
- Diffuse color (Kd): 0.64, 0.64, 0.64
- Specular color (Ks): 0.5, 0.5, 0.5

### File Format Support

The converter handles multiple RenderWare versions:

### DFF Format Versions
- GTA III (0x1803FFFF)
- GTA Vice City (0x1003FFFF)
- GTA San Andreas (0x0800FFFF)
- Multiple platform variants (PC, PS2, Xbox)

### Texture Formats
- Compressed textures (DXT1, DXT3, DXT5)
- Uncompressed RGBA
- Paletted textures
- Mipmapped textures

## Performance Considerations

### Memory Usage
- Large DFF files loaded entirely into memory
- Texture extraction requires additional RAM for image processing
- Batch processing benefits from sufficient system memory

### Processing Time
- Geometry conversion is fast (milliseconds per file)
- Texture extraction depends on image size and count
- Large TXD files with many textures may take longer

## Troubleshooting

### Common Issues

#### "DFF support not available"
- Ensure all dependencies are installed: `bun app-cli:install`
- Check that DFF/TXD modules can be imported
- Verify Python path includes the rengine_cli package

#### "PIL not available, skipping texture extraction"
- Install PIL/Pillow: `pip install pillow`
- Or use: `bun dff-to-obj:setup:pillow`

#### "No such file or directory"
- Verify input DFF file path exists
- Check file permissions
- Ensure output directory is writable

#### "Failed to extract texture"
- Some texture formats may not be supported
- Corrupted TXD data can cause extraction failures
- Check console output for specific error messages

### Debug Mode

Enable verbose output for troubleshooting:

```bash
python run_cli.py dff-to-obj input.dff --output output.obj --verbose
```

### Compatibility Issues

#### RenderWare Versions
- Some older DFF versions may have limited support
- Platform-specific variants (PS2, Xbox) may have differences

#### Texture Formats
- Unsupported compression formats will be skipped
- Some textures may require manual conversion

## Integration with 3D Applications

### Blender Import
```python
# Python script for batch importing OBJ files
import bpy
import os

obj_dir = "/path/to/obj/files"
for file in os.listdir(obj_dir):
    if file.endswith(".obj"):
        bpy.ops.import_scene.obj(filepath=os.path.join(obj_dir, file))
```

### 3D Software Compatibility
- **Blender**: Full support with materials and textures
- **Maya**: Standard OBJ import
- **3ds Max**: Requires OBJ import plugin
- **Houdini**: Native OBJ support
- **Unreal Engine**: Via OBJ import or custom pipeline
- **Unity**: Via OBJ import or asset processing

### Material Setup
After importing OBJ files with MTL materials:
1. Review material assignments
2. Adjust specular/diffuse values as needed
3. Verify texture paths are correct
4. Consider converting to PBR materials for modern workflows

## Advanced Usage

### Custom Material Templates

Modify the generated MTL files for specific rendering engines:

```python
# Example: Convert to Blender Principled BSDF
import bpy

def convert_to_principled_bsdf(obj):
    for material in obj.data.materials:
        # Convert basic MTL properties to Principled BSDF
        principled = material.node_tree.nodes.new('ShaderNodeBsdfPrincipled')
        # ... material conversion logic
```

### Pipeline Integration

Create automated conversion pipelines:

```bash
#!/bin/bash
# Convert DFF to OBJ and import into Blender
python run_cli.py dff-to-obj "$1" --output "${1%.dff}.obj" --txd "${1%.dff}.txd"
blender --background --python-script import_obj.py "${1%.dff}.obj"
```

## Examples and Use Cases

### GTA Model Extraction
Convert entire GTA model libraries for:
- **Remakes**: High-quality source assets
- **Mods**: Base models for modifications
- **Analysis**: Study original game art
- **Education**: Learn 3D modeling with real examples

### Asset Pipeline Integration
Use DFF models as part of modern pipelines:
- **LOD Generation**: Create multiple detail levels
- **Texture Baking**: Generate normal/specular maps
- **Optimization**: Reduce polygon counts
- **Format Conversion**: Export to game engine formats

### Research and Analysis
Extract models for academic purposes:
- **Game Archaeology**: Preserve legacy game assets
- **Comparative Analysis**: Study model evolution across GTA series
- **Technical Documentation**: Document RenderWare formats

## Contributing

### Adding DFF Format Support
1. Extend the `DFFToOBJConverter` class
2. Add new parsing methods for format variants
3. Update the command-line interface
4. Add comprehensive tests

### Improving Performance
- Implement streaming for large DFF files
- Add multithreaded texture extraction
- Optimize memory usage for batch processing
- Cache frequently used materials

### Texture Format Support
- Add support for additional compression formats
- Implement texture format conversion
- Improve mipmap handling
- Add texture optimization options

## Related Documentation

- [Rengine CLI Overview](../README.md)
- [RenderWare File Formats](../../docs/renderware-formats.md)
- [IPL to Godot Converter](./IPL_TO_GODOT.md)
- [GTA DFF Format Specification](../../docs/gta-file-formats/dff-format.md)
- [OBJ File Format Specification](https://en.wikipedia.org/wiki/Wavefront_.obj_file)

## License

This converter is part of the Rengine project and is licensed under GPL-3.0.