# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

Rengine is a modern 3D and 2D game engine with three distinct editions:

1. **Bevy Edition** (`./`) - egui + Bevy 0.18 + Rust. Level editor for 3D worlds using the Bevy game engine. More experimental, some features are missing.
2. **Fiber Edition** (`./adapters/fiber`) - Tauri 2.0 + React + Three.js. Modern 3D scene editor and prototyping tool. More stable.
3. **Godot Edition** (`./adapters/godot`) - Godot 4.5.1 + Rust. More experimental.

Each edition has its own build process and development workflow - always check which edition you're working in.

## Common Development Commands

### Root Project (Monorepo Management)

```bash
# Install dependencies (uses Bun)
bun install

# Format and lint all TypeScript/JavaScript code
bun check                # Runs tsc, format, and lint
bun format              # Auto-fix formatting with Biome
bun lint                # Run Biome linter with fixes

# Type checking
bun typecheck           # Run TypeScript type checking

# Clean workspace
bun clean               # Clean all build artifacts
bun fresh               # Clean and reinstall dependencies
```

### Bevy Edition (Root Directory)

```bash
# Build and run the editor (development mode)
cargo run

# Build for release
cargo build --release

# Build for specific targets
cargo build --release --target x86_64-pc-windows-gnu

# Run utility scripts
cargo run --bin upgrade_zone_files
cargo run --bin translate_zone_entities

# Test (if tests are present)
cargo test
```

**Note**: The Bevy edition does NOT use standard Rust test frameworks extensively. Check codebase for specific test patterns.

### Fiber Edition (Tauri + React)

```bash
cd adapters/fiber

# Start development server with Tauri
bun start

# Frontend-only development
bun dev

# Build for production
bun build                # Frontend only
bun app:build           # Full distributable packages

# Quality assurance
bun tests               # Run Vitest tests
bun check               # Full quality check (lint+format+type+cargo check)
bun typecheck           # TypeScript type checking
bun format              # Auto-fix formatting
bun lint                # Biome linter with fixes

# Publishing
bun app:pub             # Build and publish with version bump
```

## Architecture

### Bevy Edition Architecture

The Bevy edition is structured as a level editor with several core subsystems:

#### Core Plugin Architecture
- **Plugin-based system**: Each major feature is a Bevy plugin (terrain, doodads, foliage, regions, etc.)
- **Asset loading pipeline**: Uses Bevy's asset system with custom `.ron` manifest files
- **ECS-based**: Entity Component System pattern throughout
- **External asset support**: Assets can be loaded from external repos via `external_game_assets_folder` config

#### Key Modules (`src/`)
- **`main.rs`**: Application entry point, plugin registration, and setup
- **`tools.rs`**: Brush tools and editing tool state management (terrain, regions, tiles, foliage)
- **`ui.rs`**: Editor UI using egui, tool mode selection, and brush settings
- **`terrain/`**: Terrain editing (heightmap, splatmap, colliders, generation)
- **`doodads/`**: 3D object placement system (trees, rocks, structures)
- **`foliage.rs`**: Grass/foliage painting system (Warbler grass)
- **`regions.rs`**: Region/zone editing
- **`clay_tiles.rs`**: Tile-based building system
- **`camera.rs`**: Editor camera controls
- **`asset_loading.rs`**: Asset loading states and external asset copying
- **`editor_config.rs`**: Editor configuration from `.ron` files
- **`liquid.rs`**: Water/liquid rendering
- **`post_processing/`**: Custom depth and rendering effects

#### Crates Structure (`crates/bevy/`)
Contains local Bevy plugins and libraries:
- **`spirit_edit_core`**: Core editor utilities (zones, prefabs, placement)
- **`bevy_editor_pls*`**: In-app editor tools
- **`degen_toon_terrain`**: Terrain mesh and editing system
- **`bevy_material_wizard`**: Material system with manifest-based definitions
- **`bevy_foliage_tool`**: Foliage painting and rendering
- **`bevy_regions`**: Region/area editing
- **`bevy_clay_tiles`**: Tile building system
- **`bevy_magic_fx`**: Effects system
- **`bevy_mesh_outline`**: Mesh outlining for selection
- Other vendored dependencies

#### Configuration Files (`.ron` format)
- `assets/editor_config.editorconfig.ron`: Main editor configuration
  - `initial_level_to_load`: Specifies which level to load on startup
  - `external_game_assets_folder`: Path to external game assets repo (set to None for standalone use)
- `assets/terrain_manifest.terrainmanifest.ron`: Terrain configuration
- `assets/region_manifest.regionmanifest.ron`: Region definitions
- `assets/material_manifest.materialmanifest.ron`: Material definitions
- `assets/liquid_manifest.liquidmanifest.ron`: Liquid/water configuration
- Doodad manifests: `assets/doodad_manifests/*.ron` or external game assets

#### Editor Workflow
1. **First Run**: Configure `initial_level_to_load` to point to `default_level.level` in `assets/editor_config.editorconfig.ron`
2. **Asset Loading**: External assets copied to `./artifacts/` (gitignored) on startup
3. **Tool Modes**: Terrain Height, Terrain Splat, Foliage, Regions, Doodad Placement (ESC to toggle)
4. **Saving**: Right-click zone entities in hierarchy to save zone files (only way to persist doodads)

### Fiber Edition Architecture

The Fiber edition is a Tauri 2.0 desktop app with React frontend:

#### Frontend (`adapters/fiber/src/`)
- **React 19** with TypeScript
- **TanStack Router** for file-based routing
- **React Three Fiber** + **drei** for 3D rendering (Three.js wrapper)
- **Zustand** for state management
- **Shadcn/ui** components (built on Radix UI/Base UI)
- **Tailwind CSS 4** for styling
- **Vite** build tool

#### Backend (`adapters/fiber/src-tauri/`)
- **Tauri 2** framework (Rust backend)
- Native file system access for scene persistence
- Cross-platform desktop (Windows/macOS/Linux)

#### State Management Pattern
Uses Zustand stores with selector optimization:
```typescript
// Scene state is centralized with actions co-located
useSceneStore // Objects, lights, selections
// Actions like addObject, deleteObject trigger scene updates
```

#### 3D Rendering Pipeline
- Three.js with WebGL 2.0
- Hierarchical scene graph
- Multiple light types (ambient, directional, point, spot, hemisphere)
- GLTF/OBJ/FBX model import
- Transform gizmos for manipulation

## Development Patterns

### Bevy Edition Patterns

#### Plugin Registration
Plugins are registered in `main.rs` in a specific order:
1. Core Bevy plugins (DefaultPlugins)
2. Asset and rendering plugins (MaterializePlugin, etc.)
3. Editor core (SpiritEditCorePlugin)
4. Feature plugins (terrain, foliage, regions, doodads)
5. UI and camera plugins

#### Asset Loading Pattern
```rust
// Assets loaded via Bevy's asset server
// Custom .ron files for manifests
// Registered with bevy_common_assets
```

#### Tool System Pattern
Editor tools follow a state-driven pattern:
- `EditorToolsState` resource tracks current tool mode and settings
- Tool modes: Terrain, TerrainGen, Foliage, Regions, Tiles
- Each tool has subtool variants (e.g., Terrain → Height, Splat)
- Brush events translated to appropriate subsystem events

#### Zone/Scene Saving
Zones are sub-scenes containing doodad entities. Always save via hierarchy right-click, as this is the only persistence mechanism for placed objects.

### Fiber Edition Patterns

#### Component Composition
Declarative 3D scene composition:
```typescript
<Canvas>
  <SceneObjects />
  <SceneLights />
  <TransformControls />
</Canvas>
```

#### File-based Routing
Routes in `src/routes/`:
- `index.tsx` - Home/welcome
- `__root.tsx` - Layout and global context
- Other routes as needed

### Code Quality

#### TypeScript/JavaScript (Fiber Edition)
- **Biome** for linting and formatting (configured in `biome.json`)
- Extends `ultracite` presets (core, react, remix)
- Double quotes for strings
- 2-space indentation
- Specific rules disabled (see `biome.json`)

#### Rust (Bevy Edition)
- Standard Rust formatting with `rustfmt`
- Bevy's ECS patterns and conventions
- Edition 2024 (Cargo.toml)

## Project-Specific Notes

### External Dependencies Source Code

The `opensrc/` directory contains source code for dependencies. Use this for deeper understanding of package internals:
- Check `opensrc/sources.json` for available packages
- Fetch additional source: `npx opensrc <package>` (npm), `npx opensrc pypi:<package>` (Python), `npx opensrc crates:<package>` (Rust), or `npx opensrc <owner>/<repo>` (GitHub)

### Asset Licensing
**WARNING**: Any texture or GLTF assets in the `assets/` folder are NOT for reuse due to licensing restrictions. You are responsible for providing your own assets.

### Gitignored Paths
- `./artifacts/`: External game assets copied on startup (Bevy edition)
- `./target/`: Rust build artifacts
- Node modules and standard build outputs
- Crates exclude `rerecast` and `transform-gizmo` from workspace

### Multi-Edition Considerations
- Root `package.json` manages monorepo with Bun
- Workspace packages in `adapters/*`
- Each edition can be developed independently
- Shared tooling: Biome, Ultracite, Turbo

## Hotkeys (Bevy Edition)

- **F1**: Toggle physics collider rendering
- **ESC**: Toggle doodad placement mode

## Important Workflows

### Bevy Edition: First-Time Setup
1. Configure `assets/editor_config.editorconfig.ron` to load `default_level.level`
2. Set `external_game_assets_folder` to `None` for standalone use
3. Run `cargo run`
4. Sample level with single cube should load

### Bevy Edition: Doodad System
1. Create doodad manifest at `assets/doodad_manifests/doodad_manifest.manifest.ron`
2. Define doodads with name, model path, and optional custom props
3. Doodads appear in editor for placement
4. Save zones via right-click on zone entity in hierarchy

### Fiber Edition: Scene Workflow
1. Launch app (`bun start`)
2. Select scene template or create empty
3. Add objects (primitives or imported models)
4. Apply lighting presets or customize lights
5. Save scene as JSON file (auto-save enabled)

## Build Outputs

### Bevy Edition
- Development: `./target/debug/`
- Release: `./target/release/`
- Cross-platform Windows: `./target/x86_64-pc-windows-gnu/release/`

### Fiber Edition
- Development: Vite dev server (hot reload)
- Production: `./adapters/fiber/dist/` (frontend), Tauri bundles in `./adapters/fiber/src-tauri/target/`

## Troubleshooting

### Bevy Edition
- If levels fail to load: Check `initial_level_to_load` in `editor_config.editorconfig.ron`
- Missing doodads: Verify doodad manifest files exist and are properly formatted
- Asset errors: Ensure `external_game_assets_folder` is None or points to valid repo with `doodad_manifests/` subfolder

### Fiber Edition
- Build issues: Run `bun check` to verify all quality checks pass
- Tauri errors: Ensure Rust toolchain is up to date and Tauri CLI is installed
- Type errors: Run `bun typecheck` to identify TypeScript issues
