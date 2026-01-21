# Rengine CLI

A comprehensive command-line interface for GTA file operations.

## Features

### IMG Archive Operations
- **Info**: Display archive information (version, file count, size)
- **List**: List all files in an IMG archive with details
- **Extract**: Extract files from IMG archives with filtering options

### DFF Model Operations
- **Info**: Show basic model information (size, chunks, RenderWare version)
- **Analyze**: Analyze DFF structure with chunk breakdown

### TXD Texture Operations
- **Info**: Display texture dictionary information
- **Extract**: Extract textures (planned feature)

### COL Collision Operations
- **Info**: Show collision file information and statistics
- **Analyze**: Analyze collision structure

### IPL Map Operations
- **Info**: Display item placement file information
- **Analyze**: Analyze IPL structure and sections

### RenderWare Analysis
- **Analyze**: Low-level chunk analysis with tree structure display

### Batch Processing
- **Analyze**: Process multiple files with filtering and statistics
- **Extract IMGs**: Extract all IMG archives in a directory

## Installation

The CLI is part of the Rengine project. Ensure you have the required dependencies:

```bash
# From the project root
cd adapters/python/app-qt
pip install -r requirements.txt
```

## Usage

### Basic Commands

```bash
# IMG operations
python cli.py img info gta3.img
python cli.py img list gta3.img --filter .dff
python cli.py img extract gta3.img --output extracted/

# DFF operations
python cli.py dff info model.dff
python cli.py dff analyze model.dff --format json

# TXD operations
python cli.py txd info textures.txd

# COL operations
python cli.py col info collision.col --detailed

# IPL operations
python cli.py ipl info map.ipl

# RenderWare analysis
python cli.py rw analyze model.dff --depth 5

# Batch processing
python cli.py batch analyze /path/to/gta/files --format dff --recursive --max-files 100
python cli.py batch extract-imgs /path/to/img/archives --output extracted/
```

### Command Reference

#### IMG Commands

```bash
img info <file> [--detailed]
```
Show IMG archive information. Use `--detailed` for file type breakdown and RenderWare version analysis.

```bash
img list <file> [--filter EXT] [--format table|json]
```
List files in IMG archive. Filter by extension (e.g., `.dff`) and choose output format.

```bash
img extract <file> [files...] --output DIR [--overwrite]
```
Extract files from IMG archive. Specify individual files or extract all.

#### DFF Commands

```bash
dff info <file> [--detailed]
dff analyze <file> [--format text|json]
```

#### TXD Commands

```bash
txd info <file> [--detailed]
txd extract <file> --output DIR [--format png|dds|tga]
```

#### COL Commands

```bash
col info <file> [--detailed]
col analyze <file> [--format text|json]
```

#### IPL Commands

```bash
ipl info <file> [--detailed]
ipl analyze <file> [--format text|json]
```

#### RenderWare Commands

```bash
rw analyze <file> [--depth N] [--format tree|json]
```
Analyze RenderWare chunk structure with configurable depth.

#### Batch Commands

```bash
batch analyze <directory> [--format all|dff|txd|col|ipl|img] [--recursive] [--output FILE] [--max-files N]
```
Analyze multiple files with comprehensive statistics.

```bash
batch extract-imgs <directory> --output DIR [--recursive] [--overwrite]
```
Extract all IMG archives found in directory.

## Testing

Test with GTA San Andreas resources:

```bash
# Test IMG operations
python cli.py img info /path/to/gta-sa/custom.img
python cli.py img list /path/to/gta-sa/SAMP.img | head -20

# Test DFF operations
find /path/to/gta-sa -name "*.dff" -size -100k | head -1 | xargs python cli.py dff info

# Test batch analysis
python cli.py batch analyze /path/to/gta-sa --format dff --max-files 10 --output results.json
```

## Output Formats

### Text Format (Default)
Human-readable output with formatted tables and statistics.

### JSON Format
Machine-readable output for scripting and further processing:

```json
{
  "file": "model.dff",
  "size": 6144,
  "chunks": 94,
  "rw_version": "3.6.0.3 (San Andreas) All"
}
```

## Error Handling

The CLI provides detailed error messages and continues processing when possible. Use `--verbose` for additional debugging information.

## Limitations

- Complex 3D operations are not available in CLI mode
- Batch operations may be memory-intensive for large file sets

## Contributing

The CLI follows the same architecture as the main Rengine application. New commands can be added by:

1. Implementing command methods in the `RengineCLI` class
2. Adding subparsers in `create_parser()`
3. Following the existing error handling and output patterns

## License

Part of the Rengine project, licensed under GPL-3.0.