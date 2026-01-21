# Rengine Python Adapters

A comprehensive suite of Python tools for Grand Theft Auto 3D era game modding, providing multiple interfaces for working with RenderWare-based assets.

## 🏗️ Architecture Overview

Rengine provides three complementary Python adapters, each optimized for different use cases:

| Adapter | Interface | Purpose | Technologies |
|---------|-----------|---------|-------------|
| **app-qt** | GUI Desktop App | Full-featured modding suite | PyQt6, OpenGL |
| **blender** | 3D Editor Integration | Import/export pipeline | Blender API, DragonFF |
| **app-cli** | Command Line | Batch processing & automation | argparse, Standard Library |

## 📦 Quick Start

### Prerequisites
- **Python 3.8+** (3.9+ recommended)
- **Git** for cloning
- **Qt6 libraries** (for app-qt on Linux)

### Installation
```bash
# Clone the repository
git clone https://github.com/reliverse/rengine.git
cd rengine

# Install all Python adapters
bun install:all

# Or install individually
bun app-qt:install    # GUI app
bun blender:install  # Blender addon
bun app-cli:install      # CLI tools
```

## 🖥️ Qt Desktop Application (`app-qt/`)

A modern, responsive GUI application for comprehensive GTA file modding.

### Features
- **IMG Editor**: Manage GTA archive files with v1/v2 support
- **TXD Viewer**: Inspect texture dictionaries
- **DFF Viewer**: 3D model visualization with Qt3D
- **COL Editor**: Collision file editing
- **Map Editor**: IPL file management
- **RW Analyzer**: Low-level RenderWare chunk exploration
- **IDE Editor**: Item definition file editing

### Usage
```bash
# Launch GUI application
bun app-qt:run

# Build standalone executable
bun app-qt:build

# Run tests
bun app-qt:test
```

### System Requirements
- **Windows**: PyQt6 installs automatically
- **macOS**: `brew install qt6`
- **Linux**: See [`app-qt/LINUX_SETUP.md`](app-qt/LINUX_SETUP.md)

## 🎨 Blender Addon (`blender/`)

DragonFF - A comprehensive Blender addon for importing and exporting GTA files.

### Features
- **DFF Import/Export**: 3D models with full feature support
- **TXD Import/Export**: Texture dictionaries
- **COL Import/Export**: Collision meshes
- **IPL Import**: Map placement data
- **Material Effects**: Environment maps, bump mapping, 2DFX effects
- **Animation Support**: IFP animation files
- **Skinned Meshes**: Bone-based deformations

### Installation
```bash
# Automatic installation (builds addon package)
bun blender:build

# Manual installation:
# 1. Open Blender
# 2. Edit > Preferences > Add-ons > Install from File
# 3. Select `blender/dragonff.zip`
# 4. Enable "GTA DragonFF" addon
```

### Usage
```bash
# Import DFF model
# File > Import > GTA DFF (.dff)

# Export collision file
# Select objects > Object > Export > GTA COL

# Import map data
# Scene Settings panel > Import IPL/IDE
```

## 🖌️ Command Line Interface (`app-cli/`)

A powerful CLI tool combining features from both app-qt and blender for batch processing and automation.

### Features
- **IMG Operations**: Archive management (info, list, extract)
- **DFF Operations**: Model analysis and statistics
- **TXD Operations**: Texture dictionary inspection
- **COL Operations**: Collision file analysis
- **IPL Operations**: Map data processing
- **RW Analysis**: RenderWare chunk exploration
- **Batch Processing**: Multi-file operations with progress tracking

### Usage Examples
```bash
# Get help
bun app-cli:help

# IMG archive operations
bun app-cli:run img info gta3.img
bun app-cli:run img list gta3.img --filter .dff
bun app-cli:run img extract gta3.img model.dff --output extracted/

# Analyze DFF models
bun app-cli:run dff info model.dff
bun app-cli:run dff analyze model.dff --format json

# Batch processing
bun app-cli:run batch analyze /path/to/gta/files --format dff --recursive
bun app-cli:run batch extract-imgs /path/to/archives --output extracted/

# Test with GTA San Andreas resources
bun gta:test:img
bun gta:test:dff
bun gta:test:batch
```

## 🎯 Supported File Formats

| Format | Description | app-qt | blender | app-cli |
|--------|-------------|--------|---------|-----|
| **IMG** | Game archives | ✅ Full editor | ❌ | ✅ Info/List/Extract |
| **DFF** | 3D Models | ✅ Viewer | ✅ Import/Export | ✅ Info/Analyze |
| **TXD** | Texture dictionaries | ✅ Viewer | ✅ Import/Export | ✅ Info |
| **COL** | Collision meshes | ✅ Editor | ✅ Import/Export | ✅ Info/Analyze |
| **IPL** | Map placement | ✅ Editor | ✅ Import | ✅ Info/Analyze |
| **IDE** | Item definitions | ✅ Editor | ✅ Import | ✅ Info |
| **IFP** | Animation files | ❌ | ✅ Import | ❌ |

## 🛠️ Development

### Project Structure
```
adapters/python/
├── app-qt/           # PyQt6 GUI application
│   ├── application/  # Main application code
│   ├── tools/        # Tool modules
│   ├── tests/        # Unit tests
│   └── requirements.txt
├── blender/          # Blender addon (DragonFF)
│   ├── gtaLib/       # Core libraries
│   ├── gui/          # Blender UI
│   ├── ops/          # Import/export operators
│   └── tests/        # Addon tests
├── app-cli/              # Command-line interface
│   ├── cli.py        # Main CLI application
│   └── README.md     # CLI documentation
└── package.json      # Workspace scripts
```

### Development Setup
```bash
# Full development environment
bun dev:setup

# Run all tests
bun dev:test

# Code quality checks
bun dev:check

# Clean build artifacts
bun clean:all
```

### Contributing
- **app-qt**: See [`app-qt/CODING_GUIDELINES.md`](app-qt/CODING_GUIDELINES.md)
- **blender**: Follow DragonFF contribution guidelines
- **cli**: Use existing patterns in `cli.py`

## 🔧 Dependencies

### Core Dependencies
- **app-qt**: PyQt6, psutil, colorama, nuitka
- **blender**: Blender 3.0+, Python 3.8+
- **cli**: Python 3.8+ (no external deps)

### Optional Dependencies
- **Qt3D** for 3D rendering in app-qt
- **pytest** for testing
- **nuitka** for building executables

## 📋 Testing

### Automated Tests
```bash
# Run all tests
bun test:all

# Individual adapter tests
bun app-qt:test
bun blender:test
bun app-cli:test
```

### GTA Resource Testing
The CLI includes test commands using real GTA San Andreas files:
```bash
bun gta:test:img     # Test IMG operations
bun gta:test:dff     # Test DFF operations
bun gta:test:batch   # Test batch processing
```

## 🐳 Docker Support

```bash
# Build Python environment
bun docker:build

# Run in container
bun docker:run
```

## 📚 Documentation

- [**app-qt/README.md**](app-qt/README.md) - GUI application documentation
- [**app-qt/CODING_GUIDELINES.md**](app-qt/CODING_GUIDELINES.md) - Development guidelines
- [**blender/README.md**](blender/README.md) - Blender addon documentation
- [**app-cli/README.md**](app-cli/README.md) - CLI tool documentation
- [**app-qt/LINUX_SETUP.md**](app-qt/LINUX_SETUP.md) - Linux-specific setup

## 🎮 Supported Games

All RenderWare-based GTA games:
- **GTA III** (2001)
- **Vice City** (2002)
- **San Andreas** (2004)
- **Liberty City Stories** (2005)
- **Vice City Stories** (2006)
- **Manhunt** (2003)
- **Sonic Heroes** (2003)

## 📄 License

This project is licensed under the GPL-3.0 License. See individual adapter directories for specific licensing information.

## 🤝 Contributing

Contributions are welcome! Please see the contributing guidelines in each adapter's documentation.

### Development Workflow
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run the test suite: `bun test:all`
6. Submit a pull request

## 🆘 Troubleshooting

### Common Issues

**app-qt won't start on Linux:**
```bash
# Check Qt6 installation
bun app-qt:diagnose
# Follow setup instructions
bun app-qt:setup:linux
```

**Blender addon not loading:**
- Ensure Blender 3.0+ is installed
- Check that the addon is enabled in preferences
- Verify Python paths are correct

**CLI import errors:**
```bash
# Check Python environment
bun deps:check
# Reinstall dependencies
bun install:all
```

### Getting Help
- Check the [Issues](https://github.com/reliverse/rengine/issues) page
- Review adapter-specific documentation
- Test with the provided GTA resource commands

---

**Rengine** - Modern game modding tools for the RenderWare era.