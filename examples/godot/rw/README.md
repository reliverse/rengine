# Rengine RW (RenderWare) Demo

This demo showcases Rengine's RenderWare parsing capabilities through a Godot Editor Plugin interface.

## ✅ Current Status

**The RW extension is now fully functional as an Editor Plugin!** 🎉 The demo project includes a working Godot editor plugin that provides RenderWare tools directly in the Godot editor interface.

## Features

- **File Analysis**: Analyze any file to detect format and version information
- **IMG Archive Parsing**: Parse GTA IMG archives and get file statistics
- **DFF Model Parsing**: Parse 3D model files and extract metadata
- **Professional UI**: Custom control following Godot's official UI guidelines
- **Keyboard Shortcuts**: Ctrl+A (analyze), Ctrl+I (IMG), Ctrl+D (DFF), Ctrl+C (clear)
- **Focus Management**: Proper focus indicators and keyboard navigation
- **Theme Integration**: Respects Godot's theme system with hover/focus effects
- **Tooltips**: Helpful descriptions for all buttons
- **Error Handling**: Graceful handling when GDExtension is unavailable

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
3. The "Rengine RenderWare Tools" panel appears in the editor dock (right side)

## Usage

1. **Analyze File**: Click "Analyze Any File" to select any file and see format detection results
2. **Parse IMG**: Click "Parse IMG Archive" to select an IMG file and see archive statistics
3. **Parse DFF**: Click "Parse DFF Model" to select a DFF file and see model information
4. **Clear**: Click "Clear Output" to reset the display

## Architecture

The plugin uses a hybrid approach:

- **Rust GDExtension**: `RwAnalyzer` class provides the core parsing functionality
- **GDScript Plugin**: Manages the editor integration and UI
- **Scene-based Dock**: The UI is defined as a Godot scene for easy editing

```
Editor Plugin (GDScript)
    ↓ loads
Dock Scene (.tscn)
    ↓ instantiates
RwAnalyzer (Rust)
    ↓ uses
rengine-core/renderware/
```

## Technical Architecture

The system uses proper separation of concerns and follows Godot UI best practices:

```
Fiber (Tauri) UI -> adapters/fiber/src-tauri -> rengine-core/renderware -> File Parsing
Godot Editor Plugin -> RwEditorPlugin -> crates/godot/rw -> rengine-core/renderware -> File Parsing
     ↑                           ↑                    ↑
     └─ Custom Control UI   RwEditorPlugin      crates/core/src/renderware/
```

- **`rengine-core`**: Shared Rust library with all RenderWare parsing logic
- **`crates/godot/rw`**: Godot-specific bindings with RwAnalyzer class
- **`examples/godot/rw`**: Complete demo project with custom control implementation

## UI Implementation

The dock control follows Godot's official UI guidelines:

- **Proper Sizing**: Implements `_get_minimum_size()` for correct layout
- **Focus Management**: Handles focus states with `_notification()` and custom drawing
- **Input Handling**: Uses `_gui_input()` for mouse events and `_input()` for keyboard shortcuts
- **Theme Integration**: Respects Godot's theme system and provides visual feedback
- **Accessibility**: Includes tooltips and keyboard shortcuts (Ctrl+A, Ctrl+I, Ctrl+D, Ctrl+C)
- **Error Handling**: Graceful degradation when GDExtension is not available

## Development Status

✅ **Completed & Working**:
- RenderWare parsing logic centralized in `crates/core`
- Godot Editor Plugin fully functional with dock-based UI
- UI successfully moved from in-game CanvasLayer to editor dock panel
- Core crate builds successfully
- RW Godot extension built and integrated as editor plugin
- Workspace configuration updated
- File parsing functionality working in editor context
- GDExtension loads and classes register properly
- Plugin appears in editor dock and responds to user input

🚀 **Ready for Production Use**

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
2. Add corresponding methods to the `RwAnalyzer` class in `crates/godot/rw/src/lib.rs`
3. Update the dock UI scene in `addons/rengine_rw/rw_dock.tscn`
4. Modify the GDScript logic in the scene script as needed

## Troubleshooting

If the plugin doesn't appear:
1. Check that the GDExtension built successfully: `./build_and_copy.sh`
2. Verify the plugin is enabled in `project.godot` under `[editor_plugins]`
3. Check Godot's output console for any GDExtension loading errors
4. Ensure you're using Godot 4.2+ (the extension targets 4.2 compatibility)