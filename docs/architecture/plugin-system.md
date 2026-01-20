# Plugin System

## Overview

Rengine's plugin system enables extensibility through a modular architecture that allows third-party developers to add new features, tools, and integrations without modifying the core codebase.

## Current Architecture

### Plugin Types

#### UI Plugins
Extend the editor interface with new panels, tools, and UI components:

```typescript
interface UIPlugin {
  name: string;
  version: string;
  description: string;

  // Lifecycle methods
  activate(): void;
  deactivate(): void;

  // UI contributions
  getToolbarButtons?(): ToolbarButton[];
  getSidebarPanels?(): SidebarPanel[];
  getMenuItems?(): MenuItem[];
}
```

#### Tool Plugins
Add new transformation and editing tools:

```typescript
interface ToolPlugin {
  name: string;
  tools: TransformTool[];

  createToolOverlay(tool: TransformTool): React.Component;
  handleToolAction(action: ToolAction, context: ToolContext): void;
}
```

#### Import/Export Plugins
Support additional 3D file formats and export options:

```typescript
interface ImportExportPlugin {
  supportedFormats: FileFormat[];

  importFile(file: File, options?: ImportOptions): Promise<SceneObject[]>;
  exportScene(scene: SceneState, format: string): Promise<Blob>;
}
```

### Plugin Registration

#### Plugin Manifest
```json
{
  "name": "custom-tools",
  "version": "1.0.0",
  "description": "Custom editing tools for Rengine",
  "author": "Developer Name",
  "main": "dist/index.js",
  "rengine": {
    "apiVersion": "1.0",
    "permissions": ["scene-read", "scene-write", "file-system"],
    "entryPoints": {
      "ui": "./ui-plugin.js",
      "tools": "./tool-plugin.js"
    }
  }
}
```

#### Runtime Registration
```typescript
// Plugin loading and registration
const pluginLoader = {
  async loadPlugin(manifestPath: string): Promise<Plugin> {
    const manifest = await loadManifest(manifestPath);
    const pluginModule = await import(manifest.main);

    const plugin = new pluginModule.default();
    await plugin.activate();

    return plugin;
  },

  registerPlugin(plugin: Plugin): void {
    pluginRegistry.set(plugin.name, plugin);
    this.notifyPluginActivated(plugin);
  }
};
```

## Plugin API

### Core Interfaces

#### Scene Access API
```typescript
interface SceneAPI {
  // Read operations
  getSceneState(): SceneState;
  getSelectedObjects(): SceneObject[];
  getSceneMetadata(): SceneMetadata;

  // Write operations
  addObject(object: SceneObject): string;
  updateObject(id: string, updates: Partial<SceneObject>): void;
  removeObject(id: string): void;

  // Events
  onSceneChange(callback: (scene: SceneState) => void): () => void;
}
```

#### UI Extension API
```typescript
interface UIExtensionAPI {
  // Component registration
  registerComponent(name: string, component: React.Component): void;
  unregisterComponent(name: string): void;

  // Panel management
  addSidebarPanel(panel: SidebarPanel): string;
  removeSidebarPanel(id: string): void;

  // Toolbar extensions
  addToolbarButton(button: ToolbarButton): string;
  removeToolbarButton(id: string): void;
}
```

#### File System API
```typescript
interface FileSystemAPI {
  // File operations
  readFile(path: string): Promise<string>;
  writeFile(path: string, content: string): Promise<void>;
  readDirectory(path: string): Promise<FileEntry[]>;

  // Dialogs
  showOpenDialog(options: OpenDialogOptions): Promise<string[]>;
  showSaveDialog(options: SaveDialogOptions): Promise<string>;
}
```

### Event System

#### Plugin Events
```typescript
// Plugin lifecycle events
pluginEvents.on('plugin-activated', (plugin: Plugin) => {
  console.log(`Plugin ${plugin.name} activated`);
});

pluginEvents.on('plugin-deactivated', (plugin: Plugin) => {
  console.log(`Plugin ${plugin.name} deactivated`);
});

// Scene events that plugins can listen to
sceneEvents.on('object-added', (object: SceneObject) => {
  // Plugin can react to scene changes
});

sceneEvents.on('selection-changed', (selectedIds: string[]) => {
  // Update plugin UI based on selection
});
```

## Security and Sandboxing

### Permission System
Plugins run with restricted permissions based on their manifest:

```typescript
const permissionLevels = {
  'scene-read': 'Read access to scene data',
  'scene-write': 'Modify scene objects and properties',
  'file-system': 'Access to file system operations',
  'network': 'Network requests and external APIs',
  'system-info': 'Access to system information'
};
```

### Code Isolation
```typescript
// Plugin execution in isolated context
const pluginSandbox = new PluginSandbox({
  permissions: manifest.permissions,
  allowedGlobals: ['console', 'setTimeout', 'clearTimeout'],
  api: pluginAPI
});

await pluginSandbox.execute(pluginCode);
```

## Development Workflow

### Creating a Plugin

1. **Setup Project Structure**
```bash
mkdir my-rengine-plugin
cd my-rengine-plugin
npm init -y
npm install @rengine/plugin-api typescript
```

2. **Implement Plugin Class**
```typescript
import { Plugin, PluginContext } from '@rengine/plugin-api';

export class MyCustomPlugin implements Plugin {
  name = 'my-custom-plugin';
  version = '1.0.0';

  async activate(context: PluginContext): Promise<void> {
    // Register custom tools, UI components, etc.
    context.ui.registerComponent('MyTool', MyCustomTool);
    context.tools.registerTool('custom-transform', CustomTransformTool);
  }

  async deactivate(): Promise<void> {
    // Cleanup resources
  }
}
```

3. **Create Manifest**
```json
{
  "name": "my-custom-plugin",
  "version": "1.0.0",
  "main": "dist/index.js",
  "rengine": {
    "apiVersion": "1.0",
    "permissions": ["scene-read", "scene-write"]
  }
}
```

### Plugin Distribution

#### NPM Registry
```bash
npm publish
# Users can install with: npm install my-custom-plugin
```

#### Local Installation
```bash
# Copy plugin files to Rengine plugins directory
cp -r my-plugin ~/.rengine/plugins/
```

## Built-in Plugins

### Official Plugins
- **Material Editor**: Advanced material creation and editing
- **Animation Timeline**: Keyframe animation system
- **Physics Simulator**: Real-time physics simulation
- **Asset Library**: Cloud-based asset management

### Third-Party Ecosystem
- **Modeling Tools**: Advanced mesh manipulation
- **Lighting Presets**: Professional lighting setups
- **Export Formats**: Additional export formats (USD, FBX)
- **Collaboration**: Real-time multi-user editing

## Future Enhancements

- **WebAssembly Plugins**: High-performance plugins using Rust/WebAssembly
- **Plugin Marketplace**: Official plugin store with ratings and reviews
- **Plugin Dependencies**: Support for plugin-to-plugin dependencies
- **Hot Reloading**: Live plugin development and reloading
- **Plugin Analytics**: Usage statistics and performance monitoring
