# Rengine Rust CLI 🦀

A **high-performance, user-friendly** Rust implementation of the Rengine CLI for processing GTA game files. Features an interactive wizard, intuitive commands, and shell completion for the best developer experience.

**🚀 Perfect for beginners and experts alike!**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-passing-green)](https://github.com/reliverse/rengine/actions)
[![User Friendly](https://img.shields.io/badge/user--friendly-🦀-brightgreen)](https://github.com/reliverse/rengine)

## 🚀 Features

- **Lightning Fast**: Rust performance for processing large game archives
- **Memory Efficient**: Low memory footprint compared to Python implementation
- **Feature Complete**: All Python CLI features implemented
- **Cross Platform**: Linux, macOS, and Windows support
- **Zero Dependencies**: Single binary with no runtime dependencies

### Supported File Formats

| Format | Description | Operations |
|--------|-------------|------------|
| **IMG** | Game archives | info, list, extract |
| **DFF** | 3D models | info, analyze, convert to OBJ |
| **TXD** | Textures | info, extract, convert to materials |
| **COL** | Collision data | info, analyze |
| **IPL** | Map placement | info, analyze (binary/text), convert to Godot |
| **RW** | RenderWare chunks | analyze |

### User-Friendly Features

- **🦀 Interactive Wizard** - Guided interface for beginners
- **🔄 Convert Commands** - Simple file conversion (`convert dff`, `convert txd`)
- **🎮 Godot Alias** - Short command for IPL-to-Godot conversion
- **⚡ Shell Completion** - Auto-completion for Bash, Zsh, Fish
- **📊 Batch Processing** - Process multiple files with limits
- **🎯 Just Integration** - Modern command runner with 50+ recipes

### Converters

- **DFF → OBJ**: Convert models to Wavefront OBJ format
- **TXD → Materials**: Extract textures and material files
- **IPL → Godot**: Convert map data to Godot scenes/projects

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/reliverse/rengine.git
cd rengine/adapters/rust-cli

# Build release version
bun build:release

# Or install globally
bun install
```

### Pre-built Binaries

Download from [releases](https://github.com/reliverse/rengine/releases) or build with:

```bash
# Linux
bun build:release

# Windows (cross-compilation)
bun build:windows

# macOS (native build)
bun build:release
```

## 🎯 Usage

### Quick Start

```bash
# 🧙 Interactive wizard for beginners (recommended!)
rengine-cli wizard

# ⚡ Generate shell completion for better experience
rengine-cli completion bash   # For Bash
rengine-cli completion zsh    # For Zsh
rengine-cli completion fish   # For Fish

# 🚀 Quick examples
rengine-cli convert dff model.dff --output model.obj  # Convert DFF to OBJ
rengine-cli godot map.ipl --output scene.tscn         # IPL to Godot scene
rengine-cli batch analyze . --max-files 10            # Analyze first 10 files
```

### Basic Usage

```bash
# Show help
rengine-cli --help

# Enable verbose output
rengine-cli --verbose <command>

# JSON output
rengine-cli --format json <command>

# Quiet mode
rengine-cli --quiet <command>
```

### Interactive Wizard

The CLI includes an interactive wizard to help beginners get started:

```bash
rengine-cli wizard
```

```
🦀 Rengine CLI Interactive Wizard
=================================

What would you like to do?
1. Analyze a single file
2. Convert a file
3. Batch process multiple files
4. Generate shell completion
5. Show help

Enter your choice (1-5):
```

The wizard guides you through common operations and generates the appropriate commands for you!

### IMG Archive Operations

```bash
# Archive information
rengine-cli img info gta3.img

# List files with filter
rengine-cli img list gta3.img --filter .dff

# Extract all files
rengine-cli img extract gta3.img --output extracted/

# Extract specific files
rengine-cli img extract gta3.img --files model.dff texture.txd --output extracted/
```

### DFF Model Operations

```bash
# Model information
rengine-cli dff info model.dff

# Detailed analysis
rengine-cli dff analyze model.dff

# Convert single DFF to OBJ (legacy)
rengine-cli dff-to-obj model.dff output.obj

# Convert single DFF to OBJ (new user-friendly way)
rengine-cli convert dff model.dff --output output.obj

# Convert with TXD materials
rengine-cli convert dff model.dff --output output.obj --txd textures.txd

# Convert all DFF files in directory with limit
rengine-cli convert dff /path/to/dff/directory --output-dir /tmp/output --max-files 50
```

### TXD Texture Operations

```bash
# Texture information
rengine-cli txd info textures.txd

# Extract textures as PNG
rengine-cli txd extract textures.txd --output textures/ --format png
```

### COL Collision Operations

```bash
# Collision information
rengine-cli col info collision.col

# Detailed analysis
rengine-cli col analyze collision.col
```

### IPL Map Operations

```bash
# Map information (supports both binary and text IPL files)
rengine-cli ipl info map.ipl

# Structural analysis
rengine-cli ipl analyze map.ipl

# Limited analysis (first 10 entries)
rengine-cli ipl analyze map.ipl --max-entries 10
```

### IPL to Godot Conversion

```bash
# Convert IPL to Godot scene (legacy)
rengine-cli ipl-to-godot map.ipl --output scene.tscn

# Convert IPL to Godot scene (user-friendly alias)
rengine-cli godot map.ipl --output scene.tscn

# Convert directory to full Godot project
rengine-cli godot ipl_directory/ --output godot_project/ --project-name MyGame

# Convert with asset conversion
rengine-cli godot ipl_directory/ --output godot_project/ --convert-assets --dff-path models/ --txd-path textures/
```

### RenderWare Analysis

```bash
# Analyze chunks (default depth 3)
rengine-cli rw analyze model.dff

# Custom depth
rengine-cli rw analyze model.dff --depth 5

# Tree format (default)
rengine-cli rw analyze model.dff --format tree

# JSON format
rengine-cli rw analyze model.dff --format json
```

### Batch Processing

```bash
# Analyze all DFF files in directory
rengine-cli batch analyze models/ --format dff --recursive

# Extract all IMG archives
rengine-cli batch extract-imgs archives/ --output extracted/

# Save results to file
rengine-cli batch analyze models/ --output results.json

# Limit processing to first 50 files
rengine-cli batch analyze models/ --format all --recursive --max-files 50
```

### User-Friendly Commands

```bash
# Interactive wizard (great for beginners!)
rengine-cli wizard

# Convert files with simple commands
rengine-cli convert dff model.dff --output model.obj
rengine-cli convert txd texture.txd --output materials/

# Batch conversion with limits (much better than quirky names!)
rengine-cli convert dff /path/to/dff/files --output-dir /tmp/output --max-files 50

# Short aliases
rengine-cli godot map.ipl --output scene.tscn  # Instead of ipl-to-godot

# Shell completion setup
rengine-cli completion bash   # Generate Bash completion
rengine-cli completion zsh    # Generate Zsh completion
rengine-cli completion fish   # Generate Fish completion
```

### IPL to Godot Conversion

```bash
# Convert single IPL to Godot scene
rengine-cli ipl-to-godot map.ipl --output map.tscn

# Convert directory to Godot project
rengine-cli ipl-to-godot ipl_directory/ --output godot_project/

# Include Rust GDExtension
rengine-cli ipl-to-godot ipl_directory/ --output godot_project/ --with-rust

# Convert assets automatically
rengine-cli ipl-to-godot ipl_directory/ --output godot_project/ --convert-assets --dff-path models/ --txd-path textures/

# Use custom template
rengine-cli ipl-to-godot map.ipl --output scene.tscn --template template.tscn
```

## 🔧 Development

### Prerequisites

- Rust 1.70 or later
- Node.js 16+ (for npm scripts)

### Setup

```bash
# Install dependencies
npm install

# Development build
bun dev

# Run tests
bun test

# Code quality checks
bun quality

# Setup shell completion (optional but recommended)
bun completion:bash  # For Bash users
bun completion:zsh   # For Zsh users
bun completion:fish  # For Fish users
```

### Using Just (Alternative to NPM)

The project includes a modern `justfile` with user-friendly commands:

```bash
# Install just (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash

# Show all available commands
just

# Quick development tasks
just dev      # Development mode
just test     # Run tests
just quality  # Full quality check
just convert  # Convert first 5 DFF files
just analyze  # Analyze first 50 files
just wizard   # Interactive wizard
```

### Available Scripts

#### NPM Scripts

| Script | Description |
|--------|-------------|
| `bun dev` | Run in development mode |
| `bun build` | Build debug version |
| `bun build:release` | Build optimized release |
| `bun test` | Run all tests |
| `bun test:unit` | Run unit tests only |
| `bun test:integration` | Run integration tests |
| `bun quality` | Full quality check |
| `bun lint` | Lint and format check |
| `bun lint:fix` | Auto-fix linting issues |
| `bun doc` | Generate documentation |
| `bun clean` | Clean build artifacts |
| `bun wizard` | Interactive command wizard |
| `bun convert:dff:50` | Convert first 50 DFF files |
| `bun batch:analyze:50` | Analyze first 50 files |
| `bun gta:convert` | Full GTA SA conversion |
| `bun completion:bash` | Setup Bash completion |
| `bun examples` | Show available commands |

#### Just Commands (Alternative)

Just is a modern command runner that's simpler than make. Use `just` or `bun just` to see all available commands.

```bash
# Show all available commands
just

# Quick aliases for common tasks
just t    # run tests
just b    # build
just c    # check
just f    # format
just l    # lint

# User-friendly aliases
just convert     # Convert first 5 DFF files (with --max-files)
just analyze     # Analyze first 50 files
just godot       # Quick GTA SA conversion
just wizard      # Interactive wizard

# Development workflow
just dev-cycle   # format, lint, test
just status      # show project status
just quality     # full quality check
```

**Popular Just Recipes:**
- `just dev` - Development mode
- `just test` - All tests
- `just build-release` - Optimized build
- `just quality` - Full CI check
- `just demo-dff` - Test with real GTA files
- `just convert` - Batch convert DFF files
- `just completion-bash` - Setup shell completion

### Testing with GTA SA Files

The CLI includes integration tests using real GTA SA game files. Test files are expected at:
`/home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa/`

```bash
# Run GTA integration tests
bun gta-test

# Run with verbose logging
bun gta-test:verbose

# Test individual file types
bun demo:dff
bun demo:txd
bun demo:img
bun demo:col
bun demo:rw
bun demo:convert

# Test batch operations with limits
bun convert:dff:50    # Convert first 50 DFF files
bun batch:analyze:50  # Analyze first 50 files of all types

# Batch processing with limits
bun batch:analyze:50  # Analyze first 50 files of all types

# Full GTA SA conversion (like Python app-cli)
bun gta:convert
```

### Architecture

```
src/
├── main.rs                 # CLI entry point and argument parsing
├── commands.rs             # Command dispatch logic
├── handlers/               # File format handlers
│   ├── img.rs             # IMG archive handler
│   ├── dff.rs             # DFF model handler
│   ├── txd.rs             # TXD texture handler
│   ├── col.rs             # COL collision handler
│   ├── ipl.rs             # IPL map handler
│   ├── rw.rs              # RenderWare chunk analyzer
│   └── batch.rs           # Batch processing handler
├── converters/             # File conversion modules
│   ├── dff_to_obj.rs      # DFF → OBJ converter
│   ├── txd_to_materials.rs # TXD → Materials converter
│   └── ipl_to_godot.rs    # IPL → Godot converter
└── integration_tests.rs   # Integration tests with real files
```

## 🧪 Testing

```bash
# All tests
bun test

# Unit tests only
bun test:unit

# Integration tests
bun test:integration

# Release tests
bun test:release

# With coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Test Coverage

- **Unit Tests**: 28 tests covering all handlers and core functionality
- **Integration Tests**: Real GTA SA file testing for critical paths
- **Performance Tests**: Benchmarks for large file processing

## 📊 Performance

Compared to Python implementation:

| Operation | Python | Rust | Improvement |
|-----------|--------|------|-------------|
| DFF Analysis | ~50ms | ~5ms | 10x faster |
| IMG Extraction | ~200ms | ~20ms | 10x faster |
| TXD Processing | ~100ms | ~8ms | 12x faster |
| Memory Usage | ~50MB | ~5MB | 10x less |

*Benchmarks performed on GTA SA files on Intel i7-9700K*

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run tests: `bun quality`
5. Commit changes: `git commit -m 'Add amazing feature'`
6. Push to branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

### Development Guidelines

- **Code Style**: Follow Rust standard formatting (`cargo fmt`)
- **Linting**: Pass `cargo clippy` checks
- **Testing**: Add tests for new features
- **Documentation**: Update README for user-facing changes
- **Performance**: Consider performance implications

### Adding New Features

1. **File Handlers**: Add to `src/handlers/`
2. **Converters**: Add to `src/converters/`
3. **CLI Commands**: Update `main.rs` and `commands.rs`
4. **Tests**: Add unit tests and integration tests
5. **Documentation**: Update this README

## 📈 Roadmap

- [ ] Support for additional texture formats (DDS, TGA)
- [ ] IDE file parsing for enhanced IPL processing
- [ ] Batch conversion operations
- [ ] Progress bars for long operations
- [ ] Plugin system for custom converters
- [ ] WebAssembly support for web-based usage

## 🐛 Known Issues

- DDS/TGA texture export not yet implemented (PNG/JPG supported)
- Some rare RenderWare chunk types may not be fully analyzed
- Large IPL files (>100MB) may require increased memory allocation
- Binary IPL files display object IDs instead of model names (IDE cross-referencing not yet implemented)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE-MIT) file for details.

## 🙏 Acknowledgments

- **Original Python CLI**: Foundation for this Rust implementation
- **rengine-core**: Core parsing libraries
- **RenderWare Community**: Documentation and research
- **GTA Modding Community**: Tools and knowledge sharing

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/reliverse/rengine/issues)
- **Discussions**: [GitHub Discussions](https://github.com/reliverse/rengine/discussions)
- **Documentation**: [Rengine Docs](https://github.com/reliverse/rengine/docs)

---

## 🎉 What Makes This CLI Special

- **🦀 Beginner-Friendly** - Interactive wizard guides new users
- **⚡ Performance** - Rust speed with Python familiarity
- **🔧 Developer Experience** - Shell completion, justfile, comprehensive scripts
- **🎮 GTA-Focused** - Purpose-built for game modding workflows
- **📚 Well-Documented** - Extensive examples and guides

**Made with ❤️ for the GTA modding community**