# Current Implementation: 3D Scene Editor

## Overview

Rengine is currently implemented as a modern 3D scene editor and prototyping tool, built with React, Three.js, and Tauri. While the documentation structure suggests a full game engine, the current implementation focuses on providing an intuitive 3D editing experience with professional lighting, model import, and scene management capabilities.

## Architecture Overview

### Technology Stack

#### Frontend (React + TypeScript)
- **React 19**: Modern React with concurrent features
- **TypeScript**: Full type safety throughout the application
- **TanStack Router**: File-based routing for the application
- **Zustand**: Lightweight state management for scene data
- **React Three Fiber**: Declarative 3D rendering with Three.js
- **Shadcn/ui**: Modern UI components built on Radix UI
- **Tailwind CSS**: Utility-first styling system

#### Backend (Rust + Tauri)
- **Tauri 2**: Cross-platform desktop application framework
- **Rust**: Native performance and security
- **File System APIs**: Native file operations for scene persistence

#### 3D Graphics
- **Three.js**: WebGL-based 3D graphics engine
- **React Three Drei**: Useful helpers and abstractions
- **GLTF/GLB Support**: Modern 3D model format
- **OBJ/FBX Support**: Legacy 3D model formats

## Core Systems

### Scene Management System

#### State Architecture
```typescript
interface SceneState {
  // 3D Objects
  objects: SceneObject[];

  // Lighting
  lights: SceneLight[];

  // User Interaction
  selectedObjectIds: string[];
  selectedLightIds: string[];
  activeTool: TransformTool;

  // Camera & View
  cameraPosition: [number, number, number];
  cameraTarget: [number, number, number];

  // Scene Settings
  gridVisible: boolean;
  snapToGrid: boolean;
  gridSize: number;

  // Environment
  backgroundColor: string;
  fogEnabled: boolean;
  fogColor: string;

  // File Management
  currentFilePath: string | null;
  sceneMetadata: SceneMetadata;
}
```

#### Scene Object Structure
```typescript
interface SceneObject {
  id: string;
  name: string;
  type: "cube" | "sphere" | "plane" | "imported";
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
  color: string;
  visible: boolean;

  // 3D Model support
  geometry?: THREE.BufferGeometry;
  material?: THREE.Material;
  importedModel?: THREE.Object3D;
  initialScale?: number;
  modelid?: number; // SAMP model ID for export
}
```

#### Lighting System
```typescript
interface SceneLight {
  id: string;
  name: string;
  type: LightType; // "directional" | "point" | "spot" | "ambient" | "hemisphere"
  color: string;
  intensity: number;
  position: [number, number, number];
  visible: boolean;
  castShadow: boolean;

  // Type-specific properties
  target?: [number, number, number]; // For directional/spot lights
  distance?: number; // For point lights
  decay?: number; // For point/spot lights
  angle?: number; // For spot lights
  penumbra?: number; // For spot lights
  groundColor?: string; // For hemisphere lights

  // Shadow properties
  shadowBias?: number;
  shadowMapSize?: number;
  shadowNear?: number;
  shadowFar?: number;
  shadowRadius?: number;
}
```

### Rendering Pipeline

#### React Three Fiber Integration
```typescript
function SceneCanvas() {
  return (
    <Canvas
      camera={{ position: [5, 5, 5], fov: 50 }}
      shadows
      gl={{ antialias: true }}
    >
      {/* Lighting setup */}
      <SceneLights />

      {/* 3D Objects */}
      <SceneObjects />

      {/* Helpers */}
      <GridHelper visible={gridVisible} size={gridSize} />
      <AxesHelper visible={axesVisible} />

      {/* Camera controls */}
      <OrbitControls
        target={cameraTarget}
        position={cameraPosition}
      />

      {/* Transform controls */}
      <TransformControls
        mode={activeTool}
        object={selectedObject}
      />
    </Canvas>
  );
}
```

#### Object Rendering
```typescript
function SceneObjectMesh({ object }: { object: SceneObject }) {
  const meshRef = useRef<THREE.Mesh>(null);

  // Handle object selection
  const handleClick = useCallback(() => {
    selectObject(object.id);
  }, [object.id]);

  if (object.type === "imported" && object.importedModel) {
    // Render imported 3D model
    return (
      <primitive
        ref={meshRef}
        object={object.importedModel}
        position={object.position}
        rotation={object.rotation}
        scale={object.scale}
        onClick={handleClick}
        castShadow={object.castShadow}
        receiveShadow={object.receiveShadow}
      />
    );
  }

  // Render primitive shapes
  return (
    <mesh
      ref={meshRef}
      position={object.position}
      rotation={object.rotation}
      scale={object.scale}
      onClick={handleClick}
      castShadow
      receiveShadow
    >
      {getGeometryForType(object.type)}
      <meshStandardMaterial color={object.color} />
    </mesh>
  );
}
```

### File Management System

#### Scene Persistence
```typescript
interface ScenePersistence {
  // Save scene to file
  saveScene(sceneState: SceneState, filePath: string): Promise<void>;

  // Load scene from file
  loadScene(filePath: string): Promise<SceneState>;

  // Export scene in different formats
  exportScene(sceneState: SceneState, format: ExportFormat): Promise<Blob>;
}
```

#### Auto-save Functionality
```typescript
useEffect(() => {
  const autoSaveInterval = setInterval(() => {
    if (sceneState.isModified && sceneState.currentFilePath) {
      saveScene(sceneState, sceneState.currentFilePath);
      sceneState.markSceneSaved();
    }
  }, 30000); // Auto-save every 30 seconds

  return () => clearInterval(autoSaveInterval);
}, [sceneState]);
```

### 3D Model Import System

#### Supported Formats
- **GLTF/GLB**: Modern format with materials, animations, and PBR
- **OBJ**: Wavefront OBJ with MTL material support
- **FBX**: Autodesk FBX for complex scenes

#### Import Process
```typescript
interface ImportResult {
  success: boolean;
  object?: SceneObject;
  error?: string;
  warnings?: string[];
}

async function importModel(file: File): Promise<ImportResult> {
  // 1. Validate file format and size
  const validation = validateFile(file);
  if (!validation.valid) {
    return { success: false, error: validation.error };
  }

  // 2. Load and parse 3D model
  const model = await loadModelFile(file);

  // 3. Process geometry and materials
  const processedModel = await processModel(model);

  // 4. Create scene object
  const sceneObject = createSceneObjectFromModel(processedModel, file.name);

  // 5. Add to scene
  sceneStore.addObject(sceneObject);

  return {
    success: true,
    object: sceneObject,
    warnings: validation.warnings
  };
}
```

### User Interface Architecture

#### Component Hierarchy
```
App
├── Toolbar (File, Edit, View, Tools)
├── UnifiedSidebar
│   ├── SceneHierarchy (Object tree)
│   └── PropertyPanel (Object/light properties)
└── SceneCanvas (3D viewport)
    ├── SceneObjects
    ├── SceneLights
    ├── TransformControls
    └── CameraControls
```

#### State Management
```typescript
// Zustand store with middleware
export const useSceneStore = create<SceneState & SceneActions>()(
  subscribeWithSelector((set, get) => ({
    // Initial state
    objects: [createDefaultObject("cube", [0, 0, 0])],
    lights: [createDefaultLight("ambient"), createDefaultLight("directional")],

    // Actions
    addObject: (object) => {
      set((state) => ({
        objects: [...state.objects, object],
        selectedObjectIds: [object.id]
      }));
      get().markSceneModified();
    },

    updateObject: (id, updates) => {
      set((state) => ({
        objects: state.objects.map(obj =>
          obj.id === id ? { ...obj, ...updates } : obj
        )
      }));
      get().markSceneModified();
    }
  }))
);
```

### Tool System

#### Transform Tools
```typescript
type TransformTool = "select" | "move" | "rotate" | "scale";

interface TransformControls {
  mode: TransformTool;
  object: SceneObject | null;
  onObjectChange: (updates: Partial<SceneObject>) => void;
  snapToGrid: boolean;
  gridSize: number;
}
```

#### Tool Gizmos
- **Select Tool**: Object selection and highlighting
- **Move Tool**: Position translation with XYZ handles
- **Rotate Tool**: Rotation with axis rings
- **Scale Tool**: Uniform and per-axis scaling

### Lighting System

#### Light Types Implementation
```typescript
function SceneLight({ light }: { light: SceneLight }) {
  const lightRef = useRef<THREE.Light>(null);

  switch (light.type) {
    case "directional":
      return (
        <directionalLight
          ref={lightRef}
          color={light.color}
          intensity={light.intensity}
          position={light.position}
          target-position={light.target}
          castShadow={light.castShadow}
          shadow-bias={light.shadowBias}
        />
      );

    case "point":
      return (
        <pointLight
          ref={lightRef}
          color={light.color}
          intensity={light.intensity}
          position={light.position}
          distance={light.distance}
          decay={light.decay}
          castShadow={light.castShadow}
        />
      );

    // ... other light types
  }
}
```

#### Lighting Presets
```typescript
const lightingPresets = {
  default: () => [
    createDefaultLight("ambient", [0, 0, 0]),
    createDefaultLight("directional", [10, 10, 5])
  ],

  studio: () => [
    createLight("ambient", { intensity: 0.4 }),
    createLight("directional", { position: [5, 5, 5], intensity: 0.8 }),
    createLight("point", { position: [-5, 5, 5], intensity: 0.6, color: "#ffaa88" })
  ],

  // ... more presets
};
```

## Development Workflow

### Build System
```json
{
  "scripts": {
    "start": "bun --bun run tauri dev",
    "build": "tsc && vite build",
    "tauri": "tauri",
    "test": "vitest run",
    "lint": "biome check --write --unsafe"
  }
}
```

### Development Setup
1. **Install Dependencies**: `bun install`
2. **Start Development**: `bun start`
3. **Run Tests**: `bun test`
4. **Build Production**: `bun run build`

### Code Quality
- **Biome**: Linting and formatting
- **TypeScript**: Type checking
- **Vitest**: Unit testing
- **Ultracite**: Additional quality checks

## Performance Optimizations

### Rendering Optimizations
- **Object Instancing**: Efficient rendering of multiple similar objects
- **LOD System**: Level-of-detail for complex models
- **Frustum Culling**: Only render visible objects
- **Shadow Mapping**: Optimized shadow rendering

### Memory Management
- **Asset Caching**: Prevent duplicate asset loading
- **Geometry Pooling**: Reuse geometry instances
- **Texture Atlasing**: Combine small textures

### UI Performance
- **Virtual Scrolling**: For large scene hierarchies
- **Debounced Updates**: Prevent excessive re-renders
- **Lazy Loading**: Load components on demand

## Future Roadmap

### Short Term (Next Release)
- [ ] Undo/Redo system
- [ ] Copy/Paste objects
- [ ] Advanced material editor
- [ ] Export to GLTF format

### Medium Term (3-6 months)
- [ ] Animation timeline
- [ ] Physics simulation
- [ ] Plugin system
- [ ] Multi-scene support

### Long Term (6+ months)
- [ ] Real-time collaboration
- [ ] Advanced rendering pipeline
- [ ] Scripting system
- [ ] Full game engine features

## Migration Path

### From Current to Full Engine
The current 3D scene editor serves as the foundation for a full game engine:

1. **Scene Editor** (Current): Core 3D editing capabilities
2. **Game Engine Core**: Add ECS, physics, scripting
3. **Full Engine**: Advanced features like networking, audio, etc.

### Backward Compatibility
- Scene files remain compatible
- Plugin API evolves gradually
- Migration tools provided for breaking changes

This architecture ensures that Rengine grows from a solid 3D editing foundation into a comprehensive game development platform while maintaining stability and usability.
