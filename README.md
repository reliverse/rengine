# Rengine

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/reliverse/rengine/releases)
[![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/reliverse/rengine/releases)
[![Rust](https://img.shields.io/badge/rust-1.92+-000000.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/react-19-61dafb.svg)](https://reactjs.org/)
[![Tauri](https://img.shields.io/badge/tauri-2-24c8db.svg)](https://tauri.app/)
[![Three.js](https://img.shields.io/badge/three.js-r165+-049ef4.svg)](https://threejs.org/)

A Tauri-based game engine inspired by Unreal Engine 5, Godot, Blender, Spline, and similar. The engine can be also used as 3D editor.

## Features

- **3D Canvas**: Interactive 3D viewport with React Three Fiber
- **Object Management**: Add, select, move, rotate, and scale 3D objects
- **Property Panel**: Edit object properties (position, rotation, scale, color, visibility)
- **Grid System**: Visual grid for precise positioning
- **File Operations**: Save and load scene files in JSON format
- **Camera Controls**: Orbit controls for navigation (pan, zoom, rotate)
- **Modern UI**: Shadcn/ui components with dark theme
- **Toolbar**: Intuitive tool selection and object creation

## 🎯 Object Types

- **Cube**: Basic box geometry
- **Sphere**: Spherical geometry
- **Plane**: Flat plane geometry

## 🛠️ Tools

- **Select**: Select and manipulate objects
- **Move**: Move objects in 3D space
- **Rotate**: Rotate objects around axes
- **Scale**: Scale objects uniformly or per-axis
- **Add**: Place new objects in the scene

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
2. **Create Scene**: Start with an empty 3D scene
3. **Add Objects**: Use the toolbar to add geometric primitives
4. **Manipulate**: Select objects and use transform tools
5. **Save/Load**: Export scenes as JSON files

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

- **Move (G)**: Translate objects along axes
- **Rotate (R)**: Rotate objects around pivot points
- **Scale (S)**: Resize objects with uniform or axis-specific scaling

#### Object Properties

- Use the property panel to adjust:
  - Position (X, Y, Z coordinates)
  - Rotation (Euler angles)
  - Scale (Width, Height, Depth)
  - Color and materials
  - Visibility and rendering options

### Keyboard Shortcuts

| Shortcut  | Action                  |
| --------- | ----------------------- |
| `V`       | Select tool             |
| `G`       | Move tool               |
| `R`       | Rotate tool             |
| `S`       | Scale tool              |
| `C`       | Add cube                |
| `Shift+C` | Add sphere              |
| `P`       | Add plane               |
| `M`       | Mode toggle             |
| `Ctrl+S`  | Save scene              |
| `Ctrl+O`  | Open scene              |
| `Ctrl+N`  | New scene               |
| `Delete`  | Delete selected objects |

## Development

### Technology Stack

#### Frontend

- React 19 with TypeScript
- TanStack Router for routing
- React Three Fiber for 3D rendering
- Zustand for state management
- Shadcn/ui components
- Tailwind CSS for styling
- Vite for build tooling

#### Backend

- Tauri 2 for desktop app framework
- Rust for systems programming

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
| `bun start`     | Start development server with hot reload |
| `bun build`     | Vite-only build (no tauri)               |
| `bun app:build` | Build distributables                     |
| `bun app:pub`   | Build distributables AND bump files      |
| `bun test`      | Run test suite                           |
| `bun check`     | Lint and format check                    |
| `bun format`    | Auto-fix formatting issues               |
| `bun typecheck` | TypeScript type checking                 |

## Architecture

### Frontend Architecture (React + TypeScript)

- **Routing**: TanStack Router with file-based routing
- **3D Rendering**: React Three Fiber with Three.js
- **State Management**: Zustand for scene and UI state
- **UI Components**: Shadcn/ui built on Radix UI primitives
- **Styling**: Tailwind CSS with custom design system

### Backend Architecture (Rust + Tauri)

- **Framework**: Tauri 2 for cross-platform desktop applications
- **File Operations**: Native filesystem access for scene save/load
- **Security**: Sandboxed execution with permission-based access

### Key Design Patterns

- Component-based architecture
- State-driven UI updates
- Event-driven 3D interactions
- File-based scene serialization

## Future Enhancements

- [ ] Undo/Redo functionality
- [ ] Multiple selection
- [ ] Copy/Paste objects
- [ ] Custom geometry import
- [ ] Texture support
- [ ] Lighting controls
- [ ] Animation timeline
- [ ] Export to various formats (GLTF, OBJ, etc.)
- [ ] Plugin system
- [ ] Collaborative editing
- [ ] Material editor
- [ ] Physics simulation
- [ ] Particle systems

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

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
