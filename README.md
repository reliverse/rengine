# REngine Map Editor

A modern 3D map editor built with Rust, TypeScript, React, and Vite using Tauri for desktop applications.

## Features

### ✅ Implemented
- **3D Canvas**: Interactive 3D viewport with React Three Fiber
- **Object Management**: Add, select, move, rotate, and scale 3D objects
- **Property Panel**: Edit object properties (position, rotation, scale, color, visibility)
- **Grid System**: Visual grid with snap-to-grid functionality
- **File Operations**: Save and load map files in JSON format
- **Camera Controls**: Orbit controls for navigation (pan, zoom, rotate)
- **Dark Mode**: Modern dark theme with shadcn/ui components
- **Toolbar**: Intuitive tool selection and object creation

### 🎯 Object Types
- **Cube**: Basic box geometry
- **Sphere**: Spherical geometry
- **Plane**: Flat plane geometry

### 🛠️ Tools
- **Select**: Select and manipulate objects
- **Move**: Move objects in 3D space
- **Rotate**: Rotate objects around axes
- **Scale**: Scale objects uniformly or per-axis
- **Add**: Place new objects in the scene

### 📁 File Format
Maps are saved as JSON files with the following structure:
```json
{
  "id": "unique-map-id",
  "name": "Map Name",
  "version": "1.0.0",
  "objects": [
    {
      "id": "object-id",
      "type": "cube|sphere|plane|custom",
      "position": { "x": 0, "y": 0, "z": 0 },
      "rotation": { "x": 0, "y": 0, "z": 0 },
      "scale": { "x": 1, "y": 1, "z": 1 },
      "color": "#ffffff",
      "name": "Object Name",
      "visible": true
    }
  ],
  "settings": {
    "gridSize": 1.0,
    "snapToGrid": true,
    "backgroundColor": "#1a1a1a"
  }
}
```

## Development

### Prerequisites
- Bun (package manager)
- Rust (for Tauri backend)
- Node.js (for development tools)

### Setup
```bash
# Install dependencies
bun install

# Start development server
bun run tauri dev
```

### Build
```bash
# Build for production
bun run tauri build
```

## Architecture

### Frontend (React + TypeScript)
- **Components**: Modular React components for UI
- **Store**: Zustand for state management
- **3D Rendering**: React Three Fiber for 3D graphics
- **Styling**: Tailwind CSS v4 with shadcn/ui components

### Backend (Rust + Tauri)
- **Commands**: Tauri commands for file operations and object management
- **Serialization**: Serde for JSON serialization
- **File I/O**: Tauri file system APIs
- **UUID**: Unique identifier generation

### Key Technologies
- **Tauri**: Desktop app framework
- **React Three Fiber**: React renderer for Three.js
- **Three.js**: 3D graphics library
- **Zustand**: State management
- **Tailwind CSS**: Utility-first CSS framework
- **shadcn/ui**: Component library
- **Rust**: Systems programming language

## Usage

1. **Creating Objects**: Click "Add Cube/Sphere/Plane" buttons or use keyboard shortcuts
2. **Selecting Objects**: Click on objects in the 3D viewport
3. **Editing Properties**: Use the property panel on the right
4. **Saving Maps**: Click "Save" and choose a location
5. **Loading Maps**: Click "Open" and select a map file

## Keyboard Shortcuts

- `V`: Select tool
- `G`: Move tool
- `R`: Rotate tool
- `S`: Scale tool
- `C`: Add cube
- `Shift+C`: Add sphere
- `P`: Add plane
- `M`: Mode toggle

## Future Enhancements

- [ ] Undo/Redo functionality
- [ ] Multiple selection
- [ ] Copy/Paste objects
- [ ] Custom geometry import
- [ ] Texture support
- [ ] Lighting controls
- [ ] Animation timeline
- [ ] Export to various formats
- [ ] Plugin system
- [ ] Collaborative editing
