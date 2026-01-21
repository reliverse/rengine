# Rengine RW (RenderWare) Demo

This demo showcases Rengine's RenderWare parsing capabilities through a Godot interface.

## ✅ Current Status

**The RW extension is now fully functional!** The demo project includes a working Godot extension that can parse RenderWare files.

## Features

- **File Analysis**: Analyze any file to detect format and version information
- **IMG Archive Parsing**: Parse GTA IMG archives and get file statistics
- **DFF Model Parsing**: Parse 3D model files and extract metadata
- **Real-time Results**: See parsing results displayed in the Godot GUI

## Supported Formats

- **IMG**: GTA game archive files (.img)
- **DFF**: 3D model files (.dff)
- **TXD**: Texture dictionary files (.txd)
- **COL**: Collision data files (.col)
- **IPL**: Item placement files (.ipl)
- **IDE**: Item definition files (.ide)
- **DFX**: 2D effects files (.dfx)

## Setup

1. Build the extension: `./build_and_copy.sh` (builds and copies the library)
2. Open the project in Godot 4.2+
3. Run the main scene - you should see the demo interface

## Usage

1. **Analyze File**: Click "Analyze Any File" to select any file and see format detection results
2. **Parse IMG**: Click "Parse IMG Archive" to select an IMG file and see archive statistics
3. **Parse DFF**: Click "Parse DFF Model" to select a DFF file and see model information
4. **Clear**: Click "Clear Output" to reset the display

## Technical Architecture

The system uses proper separation of concerns:

```
Fiber (Tauri) UI -> adapters/fiber/src-tauri -> rengine-core/renderware -> File Parsing
Godot UI -> RwAnalyzer Node -> crates/godot/rw -> rengine-core/renderware -> File Parsing
     ↑                           ↑                    ↑
     └─ rw_demo.tscn        RwAnalyzer Node     crates/core/src/renderware/
```

- **`rengine-core`**: Shared Rust library with all RenderWare parsing logic
- **`crates/godot/rw`**: Godot-specific bindings with RwAnalyzer node
- **`examples/godot/rw`**: Complete demo project with working GUI

## Development Status

✅ **Completed**:
- RenderWare parsing logic centralized in `crates/core`
- Godot project structure created and functional
- UI design completed with working buttons
- Core crate builds successfully
- RW Godot extension built and integrated
- Workspace configuration updated
- File parsing functionality working

## Building from Source

To rebuild the RW extension:

```bash
# From the examples/godot/rw directory
./build_and_copy.sh
```

This script builds `crates/godot/rw` and copies the library to the project directory.

## Extending the Demo

To add more functionality:
1. Add file format parsers to `crates/core/src/renderware/`
2. Add corresponding Godot methods to `crates/godot/rw/src/lib.rs`
3. Update the UI in `rw_demo.tscn`