# Stable Public API

## Overview

Rengine provides a stable, versioned public API that enables developers to extend, integrate with, and build upon the 3D scene editor. The API follows semantic versioning and maintains backward compatibility within major versions.

## API Versioning

### Semantic Versioning
Rengine follows [semantic versioning](https://semver.org/) for its public API:

```
MAJOR.MINOR.PATCH

- MAJOR: Breaking changes (incompatible API changes)
- MINOR: New features (backward compatible)
- PATCH: Bug fixes (backward compatible)
```

### API Compatibility
- **Patch versions** (1.0.x): Always backward compatible
- **Minor versions** (1.x.0): Add new features, maintain compatibility
- **Major versions** (x.0.0): May include breaking changes

## Core API Modules

### Scene Management API

#### Scene State Interface
```typescript
interface SceneAPI {
  // Scene data access
  getSceneState(): Promise<SceneState>;
  getSceneMetadata(): Promise<SceneMetadata>;

  // Object management
  createObject(type: ObjectType, position?: Vector3): Promise<string>;
  updateObject(id: string, properties: Partial<ObjectProperties>): Promise<void>;
  deleteObject(id: string): Promise<void>;

  // Selection management
  getSelectedObjects(): Promise<SceneObject[]>;
  setSelection(objectIds: string[]): Promise<void>;

  // Scene operations
  clearScene(): Promise<void>;
  duplicateSelection(): Promise<string[]>;
}
```

#### Usage Example
```typescript
// Get the scene API instance
const sceneAPI = await rengine.getAPI('scene', '1.0');

// Create a new cube
const objectId = await sceneAPI.createObject('cube', { x: 0, y: 0, z: 0 });

// Update object properties
await sceneAPI.updateObject(objectId, {
  position: { x: 1, y: 2, z: 3 },
  color: '#ff0000',
  scale: { x: 2, y: 1, z: 1 }
});
```

### File System API

#### File Operations Interface
```typescript
interface FileSystemAPI {
  // File operations
  readFile(path: string): Promise<string>;
  writeFile(path: string, content: string): Promise<void>;
  deleteFile(path: string): Promise<void>;

  // Directory operations
  readDirectory(path: string): Promise<FileEntry[]>;
  createDirectory(path: string): Promise<void>;

  // Dialog operations
  showOpenDialog(options?: OpenDialogOptions): Promise<string[]>;
  showSaveDialog(options?: SaveDialogOptions): Promise<string>;

  // Path utilities
  joinPath(...parts: string[]): string;
  getDirectoryName(path: string): string;
  getFileName(path: string): string;
}
```

#### File Dialog Example
```typescript
const fsAPI = await rengine.getAPI('filesystem', '1.0');

// Show file open dialog for 3D models
const filePaths = await fsAPI.showOpenDialog({
  title: 'Import 3D Model',
  filters: [
    { name: '3D Models', extensions: ['gltf', 'glb', 'obj', 'fbx'] }
  ],
  multiple: true
});

// Import each selected file
for (const filePath of filePaths) {
  const content = await fsAPI.readFile(filePath);
  await sceneAPI.importModel(content, filePath);
}
```

### UI Extension API

#### Component Registration
```typescript
interface UIExtensionAPI {
  // Component registration
  registerComponent(
    name: string,
    component: React.ComponentType,
    options?: ComponentOptions
  ): string;

  unregisterComponent(id: string): void;

  // Panel management
  addSidebarPanel(panel: SidebarPanel): string;
  removeSidebarPanel(id: string): void;
  updateSidebarPanel(id: string, updates: Partial<SidebarPanel>): void;

  // Toolbar management
  addToolbarButton(button: ToolbarButton): string;
  removeToolbarButton(id: string): void;

  // Menu management
  addMenuItem(item: MenuItem): string;
  removeMenuItem(id: string): void;
}
```

#### Custom Panel Example
```typescript
const uiAPI = await rengine.getAPI('ui', '1.0');

// Register a custom React component
const panelId = uiAPI.registerComponent('CustomPanel', CustomPanelComponent);

// Add it as a sidebar panel
uiAPI.addSidebarPanel({
  id: panelId,
  title: 'Custom Tools',
  component: 'CustomPanel',
  icon: 'settings',
  defaultOpen: false
});
```

### Event System API

#### Event Subscription
```typescript
interface EventAPI {
  // Event subscription
  on(event: string, callback: EventCallback): string;
  off(subscriptionId: string): void;
  once(event: string, callback: EventCallback): string;

  // Event emission
  emit(event: string, data?: any): void;

  // Event utilities
  getAvailableEvents(): string[];
  describeEvent(event: string): EventDescription;
}
```

#### Scene Events Example
```typescript
const eventsAPI = await rengine.getAPI('events', '1.0');

// Listen for scene changes
const subscriptionId = eventsAPI.on('scene.object.added', (event) => {
  console.log('Object added:', event.objectId);
  // Update custom UI or perform actions
});

// Listen for selection changes
eventsAPI.on('scene.selection.changed', (event) => {
  const selectedIds = event.selectedIds;
  // Update selection-dependent UI
});

// Clean up when done
eventsAPI.off(subscriptionId);
```

## API Initialization

### Getting API Instances
```typescript
// Initialize Rengine API
const rengine = await RengineAPI.initialize({
  version: '1.0',
  permissions: ['scene.read', 'scene.write', 'filesystem']
});

// Get specific API modules
const sceneAPI = rengine.getAPI('scene');
const uiAPI = rengine.getAPI('ui');
const fsAPI = rengine.getAPI('filesystem');
```

### Permission System
```typescript
// Request specific permissions
const rengine = await RengineAPI.initialize({
  permissions: [
    'scene.read',      // Read scene data
    'scene.write',     // Modify scene
    'filesystem',      // File operations
    'ui',             // UI modifications
    'network'         // Network requests
  ]
});
```

## Breaking Changes Policy

### Deprecation Process
1. **Introduction**: New API marked as experimental
2. **Deprecation Warning**: API marked as deprecated with warnings
3. **Removal**: Deprecated API removed in next major version

### Migration Guide
Each major version includes:
- Migration guide for breaking changes
- Automated migration tools where possible
- Compatibility layer for common use cases

## Version History

### Version 1.0.0 (Current)
- Initial stable public API release
- Core scene management operations
- File system integration
- UI extension capabilities
- Event system for plugin communication

### Version 0.2.0 (Development)
- Experimental plugin system
- Extended file format support
- Enhanced UI customization options

## API Stability Guarantees

### Stable APIs
- Core scene operations (create, update, delete objects)
- File I/O operations
- UI extension points
- Event system contracts

### Experimental APIs
- Advanced rendering features
- Custom shader support
- Real-time collaboration
- Performance profiling tools

### Deprecated APIs
- Legacy file format support
- Old UI component patterns
- Deprecated event names

## Error Handling

### API Error Types
```typescript
interface APIError {
  code: string;
  message: string;
  details?: any;
}

// Common error codes
const ErrorCodes = {
  PERMISSION_DENIED: 'permission_denied',
  INVALID_PARAMETERS: 'invalid_parameters',
  RESOURCE_NOT_FOUND: 'resource_not_found',
  OPERATION_FAILED: 'operation_failed'
};
```

### Error Handling Example
```typescript
try {
  await sceneAPI.createObject('invalid_type');
} catch (error) {
  if (error.code === ErrorCodes.INVALID_PARAMETERS) {
    console.error('Invalid object type specified');
  } else {
    console.error('Failed to create object:', error.message);
  }
}
```

## Testing and Validation

### API Test Suite
```typescript
// Run API compatibility tests
const testResults = await rengine.runAPITests({
  version: '1.0',
  includeExperimental: false
});

console.log('API Tests passed:', testResults.passed);
```

### Validation Tools
- **API Linter**: Validates API usage patterns
- **Compatibility Checker**: Ensures plugin compatibility
- **Performance Profiler**: Monitors API performance

## Future API Extensions

### Planned APIs
- **Asset Management API**: Cloud asset integration
- **Collaboration API**: Real-time multi-user editing
- **Rendering API**: Advanced graphics customization
- **Scripting API**: Runtime JavaScript execution
- **Physics API**: Physics simulation control

### API Evolution Process
1. **RFC Process**: Community discussion for new APIs
2. **Experimental Phase**: APIs marked as experimental
3. **Stabilization**: APIs promoted to stable after testing
4. **Documentation**: Comprehensive API documentation
