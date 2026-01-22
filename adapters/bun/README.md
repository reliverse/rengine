# Rengine Bun Adapter

[![npm version](https://badge.fury.io/js/rengine-converter.svg)](https://badge.fury.io/js/rengine-converter)
[![Node.js CI](https://github.com/reliverse/rengine/actions/workflows/npm-publish.yml/badge.svg)](https://github.com/reliverse/rengine/actions/workflows/npm-publish.yml)

A modern TypeScript toolkit for 3D asset processing and game development workflows. Currently features a powerful DFF-to-glTF converter for retro game assets, with extensibility for future tools.

## Overview

Rengine is a unified game engine with 5 editions (Bevy, Fiber, Godot, Qt, Blender). The Bun adapter provides TypeScript/JavaScript tools for asset processing, conversion, and development workflows.

### Current Features
- **3D Model Converter**: Convert RenderWare (.dff, .txd) files from GTA games to modern glTF/GLB format
- **Texture Processing**: Automatic PNG conversion and optimization
- **Batch Processing**: Handle multiple files with concurrency control
- **CLI & Library APIs**: Use as a command-line tool or embed in your applications

### Future Capabilities
- Asset optimization and compression tools
- Material and shader processing utilities
- Game data extraction and conversion tools
- Development workflow automation
- Integration with other Rengine editions

## Installation

### Global CLI Installation
```bash
npm install -g rengine-converter
# or
yarn global add rengine-converter
# or
pnpm add -g rengine-converter
```

### Local Development
```bash
git clone https://github.com/reliverse/rengine.git
cd rengine/adapters/bun
bun install
bun run build
```

## Usage

### Library API

Import and use the converter programmatically in your TypeScript/JavaScript applications:

```typescript
import fs from 'fs';
import { DffConverter, ModelType } from 'rengine-converter';

// Read DFF and TXD files (TXD is optional)
const dffBuffer = fs.readFileSync('model.dff');
const txdBuffer = fs.readFileSync('model.txd'); // Optional for textures

// Initialize converter with model type
const dffConverter = new DffConverter(dffBuffer, txdBuffer, ModelType.OBJECT);

// Convert to glTF
const result = await dffConverter.convertDffToGltf();

// Export as GLB file
result.exportAs('./output/model.glb');

// Or get buffer for custom processing
const gltfBuffer = await result.getBuffer();
fs.writeFileSync('./output/model.glb', gltfBuffer);
```

#### Model Types

Choose the appropriate model type for best results:

- `ModelType.OBJECT` - Static objects, buildings, props (default)
- `ModelType.SKIN` - Character models with animations
- `ModelType.CAR` - Vehicle models

> **⚠️ Important Notes:**
> - Model type affects geometry processing and material setup
> - Car and skin models from GTA III/VC are currently experimental
> - Using the wrong type may produce unexpected results

### Command Line Interface

Convert models directly from the command line:

```bash
# Basic single file conversion
rengine --dff model.dff --txd model.txd

# Specify output format and location
rengine -d model.dff -t model.txd -o output/model.glb -f glb

# Convert without textures (TXD optional)
rengine -d model.dff -o model.glb
```

#### Batch Processing

Process entire directories of models:

```bash
# Convert all DFF files in a directory (limited to 10 by default)
rengine -i ./models --output-dir ./converted

# Process all files without limit
rengine -i ./models --output-dir ./output --limit 0

# Limit files and control concurrency
rengine -i ./models --output-dir ./output --limit 50 --concurrency 8
```

#### CLI Options

| Option | Description | Default |
|--------|-------------|---------|
| `-d, --dff <file>` | Path to DFF file (single file mode) | - |
| `-t, --txd <file>` | Path to TXD file (optional) | - |
| `-i, --input-dir <dir>` | Input directory (batch mode) | - |
| `--output-dir <dir>` | Output directory (batch mode) | - |
| `-o, --output <file>` | Output file path | input name + .glb |
| `-T, --type <type>` | Model type: object, skin, car | object |
| `-f, --format <format>` | Output format: gltf, glb | glb |
| `--limit <number>` | Max files to process (0 = unlimited) | 10 |
| `--concurrency <number>` | Concurrent conversions | 4 |
| `-h, --help` | Show help message | - |

#### Advanced Examples

```bash
# Convert GTA vehicle with specific settings
rengine -d vehicle.dff -t vehicle.txd -T car -f gltf -o vehicles/

# Batch process with high concurrency
rengine -i /path/to/gta/models --output-dir ./glb --concurrency 12

# Limited testing conversion
rengine -i ./models --limit 5 --output-dir ./test-output

# Process all files (unlimited)
rengine -i ./models --limit 0 --output-dir ./output
```

## Rengine Ecosystem

This Bun adapter is part of the larger Rengine game engine toolkit:

- **Bevy Edition** (Rust) - Core engine with experimental editor
- **Fiber Edition** (Tauri + React) - Scene editor with stable UI
- **Godot Edition** (Rust + Godot) - Godot integration
- **Qt Edition** (Python) - Desktop application framework
- **Blender Edition** (Python) - Blender extension tools

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/reliverse/rengine.git
cd rengine/adapters/bun

# Install dependencies
bun install

# Build the project
bun run build

# Run tests
bun test

# Development with watch mode
bun test --watch
```

### Project Structure

```
adapters/bun/
├── src/
│   ├── cli.ts              # Command-line interface
│   ├── index.ts            # Main exports
│   ├── converter/          # Core conversion logic
│   │   ├── dff-converter.ts
│   │   └── dff-conversion-result.ts
│   └── utils/              # Utility functions
├── tests/                  # Test files and assets
└── assets/                 # Test assets
```

### Contributing

Contributions are welcome! Areas for improvement:

- **New Converters**: Add support for additional game formats
- **Performance**: Optimize conversion speed and memory usage
- **Features**: Batch processing, GUI tools, validation
- **Testing**: Expand test coverage and add more asset formats

## Compatibility

### Supported Input Formats
- **DFF (DirectX File Format)** - RenderWare 3D models from GTA games
- **TXD (Texture Dictionary)** - RenderWare texture archives

### Supported Output Formats
- **GLB (glTF Binary)** - Binary glTF for web and modern engines
- **GLTF (glTF JSON)** - Text-based glTF with separate buffers

### Tested Games
- Grand Theft Auto: San Andreas
- Grand Theft Auto: Vice City
- Grand Theft Auto III

## Performance

- **Memory Efficient**: Processes large models without excessive memory usage
- **Concurrent Processing**: Configurable concurrency for batch operations
- **Texture Optimization**: Automatic PNG compression and size optimization

## Troubleshooting

### Common Issues

**"TXD file not found" warnings**
- TXD files are optional - conversion continues without textures
- Use `--txd` flag if you have texture files

**"Invalid DFF file" errors**
- Ensure the DFF file is from a supported GTA game
- Check file corruption or incomplete downloads

**Memory issues with large batches**
- Reduce `--concurrency` value (default: 4)
- Reduce `--limit` value (default: 10) to process fewer files at once
- Use `--limit 0` for unlimited processing only if memory allows

### Getting Help

- [GitHub Issues](https://github.com/reliverse/rengine/issues)
- [Documentation](https://docs.reliverse.org)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
