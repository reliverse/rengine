# Modular subsystems

## Overview

Rengine implements a modular architecture where different subsystems are organized as independent, composable modules. This design enables better maintainability, testability, and extensibility of the 3D scene editor.

## Core Architecture

### Frontend Subsystems

#### Scene Management System
The scene management system handles all 3D scene operations using Zustand for state management:

```typescript
// Scene state structure
interface SceneState {
  objects: SceneObject[];        // 3D objects in the scene
  lights: SceneLight[];          // Lighting setup
  selectedObjectIds: string[];   // Current selection
  activeTool: TransformTool;     // Current transform tool
  // ... additional state properties
}
```

**Key Features:**
- Real-time 3D object manipulation (move, rotate, scale)
- Multi-object selection with transform gizmos
- Grid-based positioning with snap-to-grid
- Scene serialization and persistence

#### Rendering System
Built on React Three Fiber for declarative 3D rendering:

```typescript
// React Three Fiber integration
<Canvas camera={{ position: [5, 5, 5] }}>
  <SceneObjects />
  <SceneLights />
  <GridHelper />
  <TransformControls />
</Canvas>
```

**Components:**
- **SceneCanvas**: Main 3D viewport with orbit controls
- **SceneObjectMesh**: Individual 3D object rendering
- **TransformControls**: Interactive transform gizmos
- **PlacementPreview**: Real-time object placement preview

#### UI System
Modern React-based interface using Shadcn/ui components:

```typescript
// Component hierarchy
<UnifiedSidebar context="scene">     {/* Scene hierarchy */}
<UnifiedSidebar context="tools">     {/* Property panels */}
<Toolbar />                         {/* Main toolbar */}
<SceneCanvas />                     {/* 3D viewport */}
```

### Backend Subsystems

#### File System Operations
Native file operations via Tauri Rust backend:

```rust
#[tauri::command]
async fn read_directory(path: String) -> Result<DirectoryContents, String> {
    // Cross-platform directory reading
}

#[tauri::command]
async fn write_file(file_path: String, content: String) -> Result<(), String> {
    // Secure file writing with permission checks
}
```

**Features:**
- Scene file save/load operations
- 3D model import (GLTF, OBJ, FBX)
- Cross-platform file dialogs
- Auto-save functionality

### Subsystem Communication

#### State Flow
```
User Input → React Event Handlers → Zustand Actions → Three.js Updates → UI Re-render
```

#### Data Flow
```
File Load → Tauri Command → Frontend State Update → 3D Scene Update → Visual Feedback
```

## Module Organization

### Directory Structure
```
src/
├── components/     # UI components and 3D rendering
├── stores/        # State management (Zustand)
├── utils/         # Utility functions and helpers
├── hooks/         # React hooks for cross-cutting concerns
├── types/         # TypeScript type definitions
└── integrations/  # External service integrations
```

### Separation of Concerns

#### Presentation Layer (React Components)
- Pure UI components
- Event handling and user interactions
- Visual feedback and animations

#### Business Logic Layer (Zustand Stores)
- Scene state management
- Object manipulation logic
- Lighting and material calculations

#### Infrastructure Layer (Tauri + Utilities)
- File system operations
- 3D model parsing and validation
- Cross-platform compatibility

## Benefits

### Maintainability
- Clear separation between UI, business logic, and infrastructure
- Independent testing of subsystems
- Easier debugging with isolated concerns

### Extensibility
- Plugin architecture foundation
- Easy addition of new tools and features
- Modular lighting and material systems

### Performance
- Efficient state updates with Zustand selectors
- Optimized 3D rendering with React Three Fiber
- Lazy loading of heavy components

## Future Extensions

The modular architecture provides a foundation for expanding into a full game engine:

- **Physics System**: Integration with physics engines (Rapier, Cannon.js)
- **Animation System**: Keyframe animation and skeletal animation
- **Scripting System**: JavaScript/TypeScript runtime scripting
- **Asset Pipeline**: Advanced import/export and processing
- **Multiplayer**: Real-time collaboration features
