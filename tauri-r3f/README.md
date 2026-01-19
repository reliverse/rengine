# Rengine

Rengine is a modern 3D and 2D game engine.

There is two editions:

1. `./tauri-r3f` - Tauri 2.0 + React + Three.js + etc. More stable.
2. `./` - egui + Bevy 0.18 + etc. More experimental, missing features.

---

**Note**: Everything below describes the Tauri edition of Rengine. For the Bevy edition, see [../README.md](../README.md).

[![Version](https://img.shields.io/badge/version-0.2.0-blue.svg)](https://github.com/reliverse/rengine/releases)
[![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/reliverse/rengine/releases)
[![Rust](https://img.shields.io/badge/rust-1.92+-000000.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/react-19-61dafb.svg)](https://reactjs.org/)
[![Tauri](https://img.shields.io/badge/tauri-2-24c8db.svg)](https://tauri.app/)
[![Three.js](https://img.shields.io/badge/three.js-r182+-049ef4.svg)](https://threejs.org/)

A modern 3D scene editor and prototyping tool built with React, Three.js, and Tauri. Create interactive 3D scenes with professional lighting systems, import 3D models (GLTF, OBJ, FBX), and export complete scenes. Perfect for game prototyping, architectural visualization, product design, and creative 3D projects.

> **Note**: Rengine is currently focused on providing a powerful 3D scene editing experience. While the documentation references future game engine features, the current implementation is a dedicated 3D scene editor with plans for expansion into a full game engine.

## Current Features

### Core 3D Editing
- **3D Canvas**: Interactive 3D viewport with React Three Fiber and real-time rendering
- **Object Management**: Add, select, move, rotate, and scale 3D objects with transform gizmos
- **Property Panel**: Edit object properties (position, rotation, scale, color, visibility)
- **Scene Hierarchy**: Visual scene tree with object organization and selection

### Lighting & Environment
- **Lighting System**: Professional lighting controls with multiple light types:
  - Ambient, Directional, Point, Spot, and Hemisphere lights
  - Real-time shadow casting with customizable parameters
  - Color temperature and intensity controls
- **Lighting Presets**: Pre-configured lighting setups (Default, Studio, Outdoor, Sunset, etc.)
- **Environment Settings**: Background color and fog effects

### 3D Model Support
- **Model Import**: Support for GLTF, OBJ, and FBX 3D model formats
- **Automatic Processing**: Scaling, positioning, and optimization during import
- **Performance Warnings**: Alerts for complex models that may impact performance

### Scene Management
- **Grid System**: Visual grid for precise positioning with snap-to-grid functionality
- **File Operations**: Save and load scene files in JSON format with auto-save
- **Camera Controls**: Orbit controls for navigation (pan, zoom, rotate)
- **Scene Templates**: Pre-configured starting scenes for different use cases

### User Interface
- **Modern UI**: Shadcn/ui components with dark theme and responsive design
- **Toolbar**: Intuitive tool selection and object creation with keyboard shortcuts
- **Unified Sidebar**: Context-aware panels for scene management and properties
- **Transform Tools**: Select, Move (G), Rotate (R), Scale (S) with visual gizmos

## 🎯 Object Types

- **Cube**: Basic box geometry with customizable dimensions
- **Sphere**: Spherical geometry with adjustable radius and segments
- **Plane**: Flat plane geometry for floors, walls, and surfaces
- **Imported Models**: GLTF, OBJ, and FBX 3D models with automatic scaling and positioning

## 🛠️ Tools

- **Select (V)**: Select and manipulate objects with multi-selection support
- **Move (G)**: Translate objects in 3D space with gizmo handles
- **Rotate (R)**: Rotate objects around pivot points with rotation gizmos
- **Scale (S)**: Scale objects uniformly or per-axis with scale gizmos
- **Add**: Place new objects and imported models in the scene
- **Lighting**: Comprehensive lighting controls with real-time preview

## Installation

### Quick Start

Download the latest release for your platform from the [Releases](https://github.com/reliverse/rengine/releases) page.

### Recommended System Requirements

**OS**: Windows 11+ / macOS 12+ / Ubuntu 20.04+ |
**RAM**: 8GB |
**Storage**: 2GB SSD |
**Display**: 1920x1080 |
**GPU**: OpenGL 3.3+ compatible

### Platform Downloads

- [Go to GitHub Releases Page](https://github.com/reliverse/rengine/releases/latest)

> **Note**: You may see warnings from tools such as SmartScreen, as we do not yet have the financial means to sign the code.

## Usage Guide

### Getting Started

1. **Launch Rengine**: Open the application
2. **Choose Template**: Select from scene templates (Basic, Lighting, Character, Outdoor, etc.)
3. **Create Scene**: Start with a pre-configured scene or create empty
4. **Add Objects**: Use the toolbar to add geometric primitives or import 3D models
5. **Manipulate**: Select objects and use transform tools with gizmos
6. **Lighting**: Apply lighting presets or customize individual lights
7. **Save/Load**: Export scenes as JSON files with auto-save

### Scene Templates

Rengine includes several pre-configured scene templates to get you started:

- **Basic Scene**: Simple cube with default lighting
- **Lighting Showcase**: Multiple objects demonstrating different lighting techniques
- **Character Scene**: Setup optimized for character modeling and posing
- **Outdoor Scene**: Large terrain with realistic outdoor lighting
- **Neon Scene**: Colorful urban environment with neon lighting
- **Dramatic Scene**: High-contrast lighting for cinematic shots

### 3D Navigation

- **Orbit**: Left-click and drag to rotate camera
- **Pan**: Middle-click and drag to pan
- **Zoom**: Mouse wheel to zoom in/out
- **Focus**: Double-click object to focus camera

### Object Manipulation

#### Selection

- Click objects in the viewport to select them
- Hold `Ctrl` for multi-selection
- Press `Escape` to deselect all

#### Transform Tools

- **Move (G)**: Translate objects along axes with gizmo handles
- **Rotate (R)**: Rotate objects around pivot points with rotation gizmos
- **Scale (S)**: Resize objects with uniform or axis-specific scaling

#### Scene Hierarchy

- **Object Tree**: Visual representation of all scene objects
- **Parent/Child Relationships**: Organize objects in hierarchies
- **Selection Management**: Click to select, drag to reorder
- **Visibility Toggle**: Show/hide objects and entire hierarchies

#### Object Properties

- Use the property panel to adjust:
  - Position (X, Y, Z coordinates)
  - Rotation (Euler angles)
  - Scale (Width, Height, Depth)
  - Color and materials
  - Visibility and rendering options

### Lighting System

Rengine features a comprehensive lighting system with professional-grade controls:

#### Light Types

- **Ambient Light**: Overall scene illumination
- **Directional Light**: Sun-like lighting with shadows
- **Point Light**: Omnidirectional light sources with distance falloff
- **Spot Light**: Cone-shaped lighting with adjustable angles
- **Hemisphere Light**: Sky/ground ambient lighting

#### Lighting Presets

- **Default**: Basic ambient and directional lighting
- **Studio**: Soft, even lighting for product photography
- **Outdoor**: Bright sunlight with sky lighting
- **Indoor**: Warm indoor lighting with multiple sources
- **Sunset**: Golden hour lighting with warm tones
- **Dramatic**: High contrast lighting with strong shadows
- **Neon**: Colorful neon-style lighting
- **Moonlight**: Cool blue moonlight

#### Advanced Features

- Real-time shadow casting with customizable parameters
- Color temperature and intensity controls
- Position and target positioning for directional/spot lights
- Distance and decay settings for realistic falloff
- Shadow bias, map size, and radius adjustments

### Model Import

Import 3D models in industry-standard formats:

- **GLTF/GLB**: Modern 3D format with materials and animations
- **OBJ**: Wavefront OBJ format with MTL material support
- **FBX**: Autodesk FBX format for complex scenes

Features include automatic scaling, positioning, and progress tracking during import.

### Keyboard Shortcuts

| Shortcut       | Action                  |
| -------------- | ----------------------- |
| `V`            | Select tool             |
| `G`            | Move tool               |
| `R`            | Rotate tool             |
| `S`            | Scale tool              |
| `C`            | Add cube                |
| `Shift+C`      | Add sphere              |
| `P`            | Add plane               |
| `Ctrl+A`       | Select all objects      |
| `Escape`       | Clear selection         |
| `Delete`       | Delete selected objects |
| `Ctrl+S`       | Save scene              |
| `Ctrl+O`       | Open scene              |
| `Ctrl+N`       | New scene               |
| `Ctrl+Shift+I` | Import 3D model         |

## Development

### Technology Stack

#### Frontend

- React 19 with TypeScript and TanStack Router
- React Three Fiber & Drei for 3D rendering and controls
- Zustand for state management with middleware
- Shadcn/ui components built on Base UI primitives
- Tailwind CSS 4 with custom design system
- Vite for build tooling and hot reload
- Biome for code quality and formatting

#### Backend

- Tauri 2 for cross-platform desktop applications
- Rust for native performance and security
- Native file system access for scene persistence

#### Key Technologies

- **@react-three/fiber**: React renderer for Three.js
- **@react-three/drei**: Useful helpers for React Three Fiber
- **three**: 3D graphics library
- **zustand**: Lightweight state management
- **@tauri-apps/api**: Tauri API bindings

### Prerequisites

**Required Software:**

- Node.js 22.0+
- Bun 1.3.5+ (recommended package manager)
- Rust 1.92.0+
- Git 2.30+

**Platform-Specific Requirements:**

- **Windows**: Windows 10 version 1903+ (build 18362+)
- **macOS**: macOS 10.15+ (Catalina or later)
- **Linux**: Ubuntu 18.04+, CentOS 7+, or equivalent

### Development Setup

```bash
# 1. Clone the repository
git clone https://github.com/reliverse/rengine.git
cd rengine

# 2. Install dependencies (Bun is ~3x faster than npm/yarn)
bun install

# 3. Start development server (opens Tauri app automatically)
bun start

# 4. Build for production
bun run build

# 5. Create distributable packages for all platforms
bun run tauri build
```

### Quality Assurance

```bash
# Run the complete test suite
bun test

# Run tests in watch mode during development
bun test --watch

# Check code quality and formatting (Biome + Ultracite)
bun check

# Auto-fix formatting and linting issues
bun format

# Type checking
bun typecheck
```

### Development Scripts

| Command         | Description                              |
| --------------- | ---------------------------------------- |
| `bun start`     | Start development server with Tauri      |
| `bun dev`       | Start Vite dev server only               |
| `bun build`     | Build for production (frontend only)     |
| `bun app:build` | Build distributable packages             |
| `bun app:pub`   | Build and publish with version bump      |
| `bun tests`     | Run Vitest test suite                    |
| `bun check`     | Full quality check (lint+format+type)    |
| `bun format`    | Auto-fix formatting with Biome           |
| `bun lint`      | Run Biome linter with fixes              |
| `bun typecheck` | TypeScript type checking                 |
| `bun uc`        | Run Ultracite quality checks             |

## Architecture Overview

### Current Implementation Status

Rengine is currently implemented as a **3D scene editor** with a foundation for future expansion into a full game engine. The codebase is architected with modularity and extensibility in mind.

#### Core Systems (Implemented)
- **Scene Management**: Zustand-based state management for 3D scenes
- **Rendering Engine**: React Three Fiber + Three.js for 3D graphics
- **File System**: Tauri-based native file operations
- **UI Framework**: Modern React components with professional design
- **3D Model Pipeline**: Import/export system for GLTF, OBJ, FBX formats

#### Future Systems (Planned)
- **Entity Component System**: For game logic and behaviors
- **Physics Engine**: Collision detection and simulation
- **Scripting System**: Runtime JavaScript/TypeScript execution
- **Asset Pipeline**: Advanced import processing and caching
- **Plugin System**: Third-party extension support

### Technical Architecture

#### Frontend Architecture (React + TypeScript)

- **Framework**: React 19 with modern concurrent features
- **Routing**: TanStack Router with file-based routing
- **3D Rendering**: React Three Fiber with Three.js WebGL backend
- **State Management**: Zustand stores with selector optimization
- **UI Components**: Shadcn/ui built on Radix UI primitives
- **Styling**: Tailwind CSS 4 with custom design tokens
- **Build Tool**: Vite with TypeScript and hot reload

#### Backend Architecture (Rust + Tauri)

- **Framework**: Tauri 2 for cross-platform desktop applications
- **Language**: Rust for native performance and memory safety
- **File Operations**: Native filesystem access with permission controls
- **Security**: Sandboxed execution with capability-based security
- **Cross-platform**: Windows, macOS, and Linux support

#### 3D Graphics Pipeline

- **Rendering Engine**: Three.js with WebGL 2.0
- **Scene Graph**: Hierarchical object management
- **Lighting**: Physically-based lighting with multiple light types
- **Materials**: Standard and custom shader materials
- **Post-processing**: Future support for advanced visual effects

### Design Patterns

#### State Management Pattern
```typescript
// Centralized scene state with actions
export const useSceneStore = create<SceneState & SceneActions>()(
  subscribeWithSelector((set, get) => ({
    // State and actions co-located
    objects: [],
    lights: [],

    addObject: (object) => {
      set((state) => ({ objects: [...state.objects, object] }));
      get().markSceneModified();
    }
  }))
);
```

#### Component Composition Pattern
```typescript
// Declarative 3D rendering
function SceneCanvas() {
  return (
    <Canvas>
      <SceneObjects />
      <SceneLights />
      <TransformControls />
    </Canvas>
  );
}
```

#### File-based Routing Pattern
```
src/routes/
├── index.tsx      # Welcome/Home page
├── auth.tsx       # Authentication
├── account.tsx    # User account
├── settings.tsx   # Application settings
└── __root.tsx     # Layout and global context
```

#### Native Integration Pattern
```rust
// Safe Rust backend with Tauri commands
#[tauri::command]
async fn read_directory(path: String) -> Result<DirectoryContents, String> {
    // Cross-platform file operations
}
```

### Data Flow Architecture

```
User Input → React Event Handlers → Zustand Actions → Three.js Scene Updates → Visual Feedback
    ↓
File Save → Tauri Command → Native File System → JSON Serialization
    ↓
File Load ← JSON Deserialization ← Native File System ← Tauri Command
```

### Performance Optimizations

#### Rendering Optimizations
- **React Three Fiber**: Efficient virtual DOM for 3D scenes
- **Object Instancing**: Shared geometry for similar objects
- **Frustum Culling**: Automatic visibility culling
- **LOD System**: Level-of-detail for complex models

#### Memory Management
- **Asset Deduplication**: Prevent duplicate loading
- **Lazy Loading**: Components loaded on demand
- **Garbage Collection**: Proper cleanup of 3D resources

#### UI Performance
- **Virtual Scrolling**: For large scene hierarchies
- **Debounced Updates**: Prevent excessive re-renders
- **Memoization**: React.memo and useMemo for expensive operations

## Development Roadmap

### Recently Completed ✅
- [x] Core 3D scene editing with transform tools
- [x] Professional lighting system with multiple light types
- [x] 3D model import (GLTF, OBJ, FBX) with automatic processing
- [x] Scene save/load with JSON format and auto-save
- [x] Modern UI with dark theme and responsive design
- [x] Grid system with snap-to-grid functionality
- [x] Camera controls with orbit navigation
- [x] Property panels for object and light editing
- [x] Scene hierarchy with object organization
- [x] Cross-platform desktop application (Windows/macOS/Linux)

### Short Term (Next 1-2 Releases) 🎯
- [ ] Undo/Redo system with command history
- [ ] Copy/Paste objects and hierarchies
- [ ] Export to GLTF format
- [ ] Advanced material editor
- [ ] Texture support and UV mapping
- [ ] Scene templates and presets
- [ ] Keyboard shortcuts customization

### Medium Term (3-6 Months) 🚀
- [ ] Animation timeline and keyframe system
- [ ] Plugin system foundation
- [ ] Physics simulation integration
- [ ] Multi-scene support and scene references
- [ ] Asset library with search and organization
- [ ] Performance optimizations for large scenes

### Long Term (6+ Months) 🌟
- [ ] Full game engine features (ECS, scripting)
- [ ] Real-time collaboration and multi-user editing
- [ ] Advanced rendering pipeline (PBR, post-processing)
- [ ] Audio system integration
- [ ] Terrain editor with procedural generation
- [ ] Particle systems and visual effects
- [ ] Network multiplayer support

### Future Engine Features (Post-Scene Editor) 🔮
These features will be implemented as Rengine evolves from a 3D scene editor into a full game engine:

#### Entity Component System
- [ ] Component-based architecture
- [ ] Runtime-safe add/remove components
- [ ] System scheduling and dependencies

#### Scripting & Logic
- [ ] JavaScript/TypeScript scripting
- [ ] Visual scripting with node graphs
- [ ] Lifecycle hooks and event system

#### Advanced Rendering
- [ ] Physically-based rendering (PBR)
- [ ] Global illumination
- [ ] Advanced post-processing stack
- [ ] Multiple render targets and viewports

#### Asset Pipeline
- [ ] Asset importing and processing
- [ ] Dependency tracking and caching
- [ ] Asset variants and LODs

#### Tooling & Workflow
- [ ] Scene play-in-editor
- [ ] Live editing during play mode
- [ ] Performance profiling and debugging
- [ ] Automated testing and CI/CD

## Roadmap

### Scene & World

- [ ] [Scene graph (entities + parent/child)](docs/scene-world/scene-graph-entities-parent-child.md)
- [ ] [Deterministic transforms (local / world)](docs/scene-world/deterministic-transforms-local-world.md)
- [ ] [Stable IDs for all entities](docs/scene-world/stable-ids-entity.md)
- [ ] [Scene serialization (versioned)](docs/scene-world/scene-serialization-versioned.md)
- [ ] [Scene instancing](docs/scene-world/scene-instancing.md)
- [ ] [Scene hot-reload](docs/scene-world/scene-hot-reload.md)

### Entity / Component Model

- [ ] [Entity = ID + components](docs/entity-component-model/entity-id-components.md)
- [ ] [Components = pure data](docs/entity-component-model/components-pure-data.md)
- [ ] [Systems = logic](docs/entity-component-model/systems-logic.md)
- [ ] [Enable/disable entities & components](docs/entity-component-model/enable-disable-entities-components.md)
- [ ] [Runtime-safe add/remove components](docs/entity-component-model/runtime-safe-add-remove-components.md)

### Asset System

- [ ] [Asset registry (UUID → path)](docs/asset-system/asset-registry-uuid-path.md)
- [ ] [Reference counting / lifetime](docs/asset-system/reference-counting-lifetime.md)
- [ ] [Import pipeline (source → runtime)](docs/asset-system/import-pipeline-source-runtime.md)
- [ ] [Asset cache](docs/asset-system/asset-cache.md)
- [ ] [Asset dependency tracking](docs/asset-system/asset-dependency-tracking.md)

### Renderer Abstraction

- [ ] [Renderer API (not Three.js-bound)](docs/renderer-abstraction/renderer-api-not-threejs-bound.md)
- [ ] [Camera system](docs/renderer-abstraction/camera-system.md)
- [ ] [Multiple viewports](docs/renderer-abstraction/multiple-viewports.md)
- [ ] [Render layers / masks](docs/renderer-abstraction/render-layers-masks.md)
- [ ] [Debug rendering layer](docs/renderer-abstraction/debug-rendering-layer.md)

### 3D Rendering

- [ ] [Mesh system](docs/3d-rendering/mesh-system.md)
- [ ] [Material system (PBR)](docs/3d-rendering/material-system-pbr.md)
- [ ] [Lighting (forward minimum)](docs/3d-rendering/lighting-forward-minimum.md)
- [ ] [Shadows](docs/3d-rendering/shadows.md)
- [ ] [Skybox / environment map](docs/3d-rendering/skybox-environment-map.md)
- [ ] [Post-processing stack](docs/3d-rendering/post-processing-stack.md)

### 2D Rendering

- [ ] [Sprite renderer](docs/2d-rendering/sprite-renderer.md)
- [ ] [Texture atlas](docs/2d-rendering/texture-atlas.md)
- [ ] [2D camera](docs/2d-rendering/2d-camera.md)
- [ ] [Z-order / layers](docs/2d-rendering/z-order-layers.md)
- [ ] [Pixel-perfect mode](docs/2d-rendering/pixel-perfect-mode.md)

### Input System

- [ ] [Keyboard / mouse / gamepad](docs/input-system/keyboard-mouse-gamepad.md)
- [ ] [Action mapping (not key-based)](docs/input-system/action-mapping-not-key-based.md)
- [ ] [Input contexts / layers](docs/input-system/input-contexts-layers.md)
- [ ] [Rebindable controls](docs/input-system/rebindable-controls.md)

### Scripting

- [ ] [Scriptable components](docs/scripting/scriptable-components.md)
- [ ] [Lifecycle hooks](docs/scripting/lifecycle-hooks.md)
- [ ] [Hot reload scripts](docs/scripting/hot-reload-scripts.md)
- [ ] [Script sandboxing](docs/scripting/script-sandboxing.md)
- [ ] [Editor → runtime parity](docs/scripting/editor-runtime-parity.md)

### Physics

- [ ] [Collision shapes (2D/3D)](docs/physics/collision-shapes-2d-3d.md)
- [ ] [Broadphase / narrowphase](docs/physics/broadphase-narrowphase.md)
- [ ] [Rigid bodies](docs/physics/rigid-bodies.md)
- [ ] [Triggers](docs/physics/triggers.md)
- [ ] [Raycasts](docs/physics/raycasts.md)
- [ ] [Physics layers & masks](docs/physics/physics-layers-masks.md)

### Editor Integration

- [ ] [Inspector auto-generated from components](docs/editor-integration/inspector-auto-generated-from-components.md)
- [ ] [Gizmos per component](docs/editor-integration/gizmos-per-component.md)
- [ ] [Scene play-in-editor](docs/editor-integration/scene-play-in-editor.md)
- [ ] [Live editing during play](docs/editor-integration/live-editing-during-play.md)
- [ ] [Undo/Redo everywhere](docs/editor-integration/undo-redo-everywhere.md)

### Prefabs / Scenes

- [ ] [Prefabs with overrides](docs/prefabs-scenes/prefabs-with-overrides.md)
- [ ] [Nested prefabs](docs/prefabs-scenes/nested-prefabs.md)
- [ ] [Variant system](docs/prefabs-scenes/variant-system.md)
- [ ] [Scene inheritance](docs/prefabs-scenes/scene-inheritance.md)

### Debugging

- [ ] [Entity inspector](docs/debugging/entity-inspector.md)
- [ ] [Runtime console](docs/debugging/runtime-console.md)
- [ ] [Visual debug (colliders, rays)](docs/debugging/visual-debug-colliders-rays.md)
- [ ] [Performance overlays](docs/debugging/performance-overlays.md)
- [ ] [Error stack traces with entity context](docs/debugging/error-stack-traces-entity-context.md)

### Animation

- [ ] [Animation player](docs/animation/animation-player.md)
- [ ] [Keyframe tracks](docs/animation/keyframe-tracks.md)
- [ ] [Blend trees](docs/animation/blend-trees.md)
- [ ] [Skeletal animation](docs/animation/skeletal-animation.md)
- [ ] [Animation events](docs/animation/animation-events.md)

### Audio

- [ ] [Audio sources](docs/audio/audio-sources.md)
- [ ] [2D/3D audio](docs/audio/2d-3d-audio.md)
- [ ] [Mixer & buses](docs/audio/mixer-buses.md)
- [ ] [Effects (reverb, filter)](docs/audio/effects-reverb-filter.md)
- [ ] [Spatialization](docs/audio/spatialization.md)

### UI

- [ ] [Retained-mode UI](docs/ui/retained-mode-ui.md)
- [ ] [Anchors / layout](docs/ui/anchors-layout.md)
- [ ] [Input routing](docs/ui/input-routing.md)
- [ ] [World-space UI](docs/ui/world-space-ui.md)
- [ ] [Theme system](docs/ui/theme-system.md)

### Runtime

- [ ] [Deterministic update loop](docs/runtime/deterministic-update-loop.md)
- [ ] [Fixed timestep](docs/runtime/fixed-timestep.md)
- [ ] [Pause / step frame](docs/runtime/pause-step-frame.md)
- [ ] [Scene streaming](docs/runtime/scene-streaming.md)
- [ ] [Background loading](docs/runtime/background-loading.md)

### Export & Platforms

- [ ] [Headless build](docs/export-platforms/headless-build.md)
- [ ] [Web export](docs/export-platforms/web-export.md)
- [ ] [Desktop export](docs/export-platforms/desktop-export.md)
- [ ] [Mobile (optional)](docs/export-platforms/mobile-optional.md)
- [ ] [Platform abstraction layer](docs/export-platforms/platform-abstraction-layer.md)

### Tooling

- [ ] [CLI tools](docs/tooling/cli-tools.md)
- [ ] [Asset import CLI](docs/tooling/asset-import-cli.md)
- [ ] [Build pipeline](docs/tooling/build-pipeline.md)
- [ ] [Scene compiler](docs/tooling/scene-compiler.md)
- [ ] [Crash reporting](docs/tooling/crash-reporting.md)

### Architecture

- [ ] [Modular subsystems](docs/architecture/modular-subsystems.md)
- [ ] [Plugin system](docs/architecture/plugin-system.md)
- [ ] [Stable public API](docs/architecture/stable-public-api.md)
- [ ] [Versioned engine core](docs/architecture/versioned-engine-core.md)

### Advanced

- [ ] [Multiplayer hooks](docs/advanced/multiplayer-hooks.md)
- [ ] [Deterministic rollback (optional)](docs/advanced/deterministic-rollback-optional.md)
- [ ] [Save/load system](docs/advanced/save-load-system.md)
- [ ] [Replay system](docs/advanced/replay-system.md)
- [ ] [Modding support](docs/advanced/modding-support.md)

## Contributing

We welcome contributions! Rengine is an open-source project that thrives on community involvement.

### Ways to Contribute

- **Bug Reports**: Found a bug? [Open an issue](https://github.com/reliverse/rengine/issues/new)
- **Feature Requests**: Have an idea? [Start a discussion](https://github.com/reliverse/rengine/discussions)
- **Code**: Fix bugs, add features, or improve performance
- **Documentation**: Help improve docs or tutorials

### Development Workflow

1. **Fork** the repository on GitHub
2. **Clone** your fork locally: `git clone https://github.com/your-username/rengine.git`
3. **Create** a feature branch: `git checkout -b feature/your-feature-name`
4. **Set up** development environment: `bun install && bun start`
5. **Make** your changes following our guidelines
6. **Test** thoroughly: `bun test && bun check`
7. **Commit** with clear messages: `git commit -m "feat: add amazing new feature"`
8. **Push** to your branch and open a Pull Request

## License

Rengine is free and open source software, created by [Nazar Kornienko (blefnk)](https://github.com/blefnk) and whole [Reliverse](https://github.com/reliverse) community, licensed under [MIT](../LICENSE-MIT) or [Apache License 2.0](../LICENSE-APACHE), at your choice. Rengine follows Rust's dual-license recommendation and Bevy's [best practices](https://bevy.org/learn/quick-start/plugin-development).
