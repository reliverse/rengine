# Scene Hot-Reload

## Overview

Rengine implements a sophisticated scene hot-reload system that enables real-time scene updates during development and runtime. This system allows developers and users to see changes instantly without restarting the application or reloading the entire scene.

## Core Concepts

### Hot-Reload vs Cold-Reload

#### Cold Reload
- **Full Restart**: Complete application/scene restart
- **State Loss**: All runtime state is lost
- **Slow Iteration**: Long reload times for large scenes
- **Disruptive**: Breaks workflow and concentration

#### Hot Reload
- **Incremental Updates**: Only changed parts are updated
- **State Preservation**: Runtime state maintained where possible
- **Fast Iteration**: Near-instant feedback for changes
- **Non-Disruptive**: Maintains workflow continuity

### Reload Triggers

Hot-reload can be triggered by:

- **File Changes**: Scene files modified on disk
- **Asset Updates**: Textures, models, materials changed
- **Script Changes**: Component scripts modified
- **Property Changes**: Entity properties updated in editor
- **Manual Trigger**: User-initiated reload command

## Implementation Details

### Change Detection

The system monitors various sources for changes:

```typescript
class ChangeDetector {
  private fileWatcher: FileWatcher;
  private assetWatcher: AssetWatcher;
  private sceneWatcher: SceneWatcher;

  constructor() {
    this.setupFileWatching();
    this.setupAssetWatching();
    this.setupSceneWatching();
  }

  private setupFileWatching(): void {
    this.fileWatcher = new FileWatcher({
      paths: ['scenes/', 'assets/', 'scripts/'],
      extensions: ['.rengine', '.gltf', '.js', '.ts'],
      onChange: (filePath, changeType) => {
        this.handleFileChange(filePath, changeType);
      }
    });
  }

  private async handleFileChange(filePath: string, changeType: ChangeType): Promise<void> {
    const change = await this.analyzeChange(filePath, changeType);
    if (change.needsReload) {
      await this.triggerHotReload(change);
    }
  }
}
```

### Change Analysis

Analyze what changed and what needs updating:

```typescript
interface SceneChange {
  type: 'entity' | 'asset' | 'material' | 'script' | 'scene';
  entityId?: string;
  assetId?: string;
  propertyPath?: string;
  oldValue?: any;
  newValue?: any;
  requiresFullReload: boolean;
}

function analyzeSceneChange(oldScene: SceneData, newScene: SceneData): SceneChange[] {
  const changes: SceneChange[] = [];

  // Compare entities
  const entityChanges = compareEntities(oldScene.entities, newScene.entities);
  changes.push(...entityChanges);

  // Compare assets
  const assetChanges = compareAssets(oldScene.assets, newScene.assets);
  changes.push(...assetChanges);

  // Compare scene settings
  const sceneChanges = compareSceneSettings(oldScene.settings, newScene.settings);
  changes.push(...sceneChanges);

  return changes;
}
```

## Hot-Reload Pipeline

### Incremental Update Process

1. **Change Detection**: Identify what changed
2. **Dependency Analysis**: Determine affected entities/components
3. **State Preservation**: Save current runtime state
4. **Incremental Update**: Apply changes to running scene
5. **State Restoration**: Restore preserved state where possible
6. **Validation**: Ensure scene integrity after update

### Entity Updates

Update individual entities without full scene reload:

```typescript
class EntityHotReloader {
  async updateEntity(entityId: string, changes: EntityChange[]): Promise<void> {
    const entity = scene.getEntityById(entityId);
    if (!entity) return;

    // Preserve runtime state
    const runtimeState = this.captureEntityState(entity);

    // Apply changes
    for (const change of changes) {
      await this.applyEntityChange(entity, change);
    }

    // Restore compatible runtime state
    this.restoreEntityState(entity, runtimeState);

    // Update dependent systems
    this.updateDependents(entity);
  }

  private async applyEntityChange(entity: Entity, change: EntityChange): Promise<void> {
    switch (change.type) {
      case 'transform':
        entity.setTransform(change.newValue);
        break;
      case 'material':
        await entity.updateMaterial(change.newValue);
        break;
      case 'geometry':
        await entity.updateGeometry(change.newValue);
        break;
      case 'component':
        entity.updateComponent(change.propertyPath, change.newValue);
        break;
    }
  }
}
```

### Asset Updates

Hot-reload textures, models, and other assets:

```typescript
class AssetHotReloader {
  private assetCache = new Map<string, Asset>();

  async reloadAsset(assetId: string, newData: AssetData): Promise<void> {
    // Unload old asset
    const oldAsset = this.assetCache.get(assetId);
    if (oldAsset) {
      await this.unloadAsset(oldAsset);
    }

    // Load new asset
    const newAsset = await this.loadAsset(newData);

    // Update cache
    this.assetCache.set(assetId, newAsset);

    // Update all entities using this asset
    const affectedEntities = scene.findEntitiesUsingAsset(assetId);
    for (const entity of affectedEntities) {
      entity.updateAssetReference(assetId, newAsset);
    }

    // Trigger re-render
    renderer.update();
  }

  private async unloadAsset(asset: Asset): Promise<void> {
    // Release GPU resources
    if (asset.texture) {
      asset.texture.dispose();
    }
    if (asset.geometry) {
      asset.geometry.dispose();
    }
    if (asset.material) {
      asset.material.dispose();
    }
  }
}
```

## State Preservation

### Runtime State Management

Preserve dynamic state during hot-reload:

```typescript
interface PreservedState {
  entityStates: Map<string, EntityRuntimeState>;
  systemStates: Map<string, SystemState>;
  userStates: Map<string, any>;
}

interface EntityRuntimeState {
  velocity?: Vector3;
  animationTime?: number;
  physicsState?: PhysicsBodyState;
  scriptVariables?: Map<string, any>;
}

class StatePreserver {
  captureState(): PreservedState {
    return {
      entityStates: this.captureEntityStates(),
      systemStates: this.captureSystemStates(),
      userStates: this.captureUserStates()
    };
  }

  restoreState(state: PreservedState): void {
    this.restoreEntityStates(state.entityStates);
    this.restoreSystemStates(state.systemStates);
    this.restoreUserStates(state.userStates);
  }

  private captureEntityStates(): Map<string, EntityRuntimeState> {
    const states = new Map<string, EntityRuntimeState>();

    for (const entity of scene.entities) {
      states.set(entity.id, {
        velocity: entity.physics?.velocity?.clone(),
        animationTime: entity.animation?.currentTime,
        physicsState: entity.physics?.getState(),
        scriptVariables: entity.script?.getVariables()
      });
    }

    return states;
  }
}
```

## Script Hot-Reload

### Component Script Updates

Reload scripts without restarting entities:

```typescript
class ScriptHotReloader {
  private scriptCache = new Map<string, ScriptClass>();

  async reloadScript(scriptPath: string): Promise<void> {
    // Compile new script
    const newScriptClass = await this.compileScript(scriptPath);

    // Update cache
    this.scriptCache.set(scriptPath, newScriptClass);

    // Find all entities using this script
    const affectedEntities = scene.findEntitiesUsingScript(scriptPath);

    for (const entity of affectedEntities) {
      // Preserve script state
      const scriptState = entity.script?.getState();

      // Replace script instance
      entity.script = new newScriptClass();

      // Restore compatible state
      if (scriptState) {
        entity.script.setState(scriptState);
      }

      // Reinitialize if needed
      entity.script.onReload?.();
    }
  }

  private async compileScript(scriptPath: string): Promise<ScriptClass> {
    const sourceCode = await readFile(scriptPath);

    // Use dynamic compilation or eval in development
    // In production, this would use pre-compiled scripts
    const compiledScript = this.compileTypeScript(sourceCode);

    return compiledScript;
  }
}
```

## Performance Optimizations

### Incremental Updates

Only update what changed:

```typescript
class IncrementalUpdater {
  private changeQueue: SceneChange[] = [];
  private updateBatchSize = 10;

  queueChange(change: SceneChange): void {
    this.changeQueue.push(change);
    this.scheduleUpdate();
  }

  private scheduleUpdate(): void {
    if (this.updateScheduled) return;

    this.updateScheduled = true;
    requestAnimationFrame(() => this.processUpdates());
  }

  private async processUpdates(): Promise<void> {
    this.updateScheduled = false;

    // Process changes in batches
    while (this.changeQueue.length > 0) {
      const batch = this.changeQueue.splice(0, this.updateBatchSize);

      await Promise.all(
        batch.map(change => this.applyChange(change))
      );

      // Allow frame to render between batches
      await new Promise(resolve => requestAnimationFrame(resolve));
    }
  }
}
```

### Memory Management

Efficient memory usage during hot-reload:

- **Reference Counting**: Track asset usage to avoid premature disposal
- **Garbage Collection**: Clean up unused assets after reload
- **Memory Pooling**: Reuse objects to reduce allocation overhead

## Error Handling

### Graceful Degradation

Handle reload failures gracefully:

```typescript
class HotReloadErrorHandler {
  async handleReloadError(error: Error, change: SceneChange): Promise<void> {
    console.error('Hot-reload failed:', error);

    // Log error details
    this.logReloadError(error, change);

    // Attempt partial reload
    if (this.canAttemptPartialReload(change)) {
      await this.attemptPartialReload(change);
    } else {
      // Fallback to full reload
      await this.fallbackToFullReload();
    }
  }

  private async attemptPartialReload(change: SceneChange): Promise<void> {
    // Try to reload just the affected component
    try {
      await this.reloadComponent(change);
    } catch (componentError) {
      // If component reload fails, reload entity
      await this.reloadEntity(change.entityId!);
    }
  }

  private async fallbackToFullReload(): Promise<void> {
    // Show user notification
    ui.showNotification('Hot-reload failed, performing full reload...', 'warning');

    // Perform full scene reload
    await scene.reload();

    // Restore camera position if possible
    camera.restorePosition();
  }
}
```

## Development Integration

### Editor Integration

Hot-reload integration with the editor:

- **Live Preview**: See changes instantly in editor viewport
- **Error Highlighting**: Show reload errors in editor UI
- **Reload Controls**: Manual reload buttons and shortcuts
- **Reload History**: Track recent reloads for debugging

### Debugging Features

Tools for debugging hot-reload issues:

```typescript
class HotReloadDebugger {
  private reloadHistory: ReloadEvent[] = [];
  private performanceMetrics: ReloadMetrics[] = [];

  logReloadEvent(event: ReloadEvent): void {
    this.reloadHistory.push(event);

    // Keep only recent history
    if (this.reloadHistory.length > 100) {
      this.reloadHistory.shift();
    }
  }

  getReloadMetrics(): ReloadMetrics {
    return {
      averageReloadTime: this.calculateAverageReloadTime(),
      successRate: this.calculateSuccessRate(),
      mostCommonErrors: this.getMostCommonErrors(),
      performanceTrend: this.analyzePerformanceTrend()
    };
  }

  exportDebugInfo(): DebugInfo {
    return {
      reloadHistory: this.reloadHistory,
      performanceMetrics: this.performanceMetrics,
      systemInfo: this.getSystemInfo(),
      configuration: this.getConfiguration()
    };
  }
}
```

## Configuration

### Hot-Reload Settings

Configurable hot-reload behavior:

```typescript
interface HotReloadConfig {
  enabled: boolean;
  watchPaths: string[];
  ignoredPaths: string[];
  debounceMs: number;
  maxReloadAttempts: number;
  preserveState: boolean;
  showNotifications: boolean;
  performanceMonitoring: boolean;
}

const defaultConfig: HotReloadConfig = {
  enabled: true,
  watchPaths: ['scenes/', 'assets/', 'scripts/'],
  ignoredPaths: ['*.tmp', '*.bak'],
  debounceMs: 300,
  maxReloadAttempts: 3,
  preserveState: true,
  showNotifications: true,
  performanceMonitoring: false
};
```

## Use Cases

### Development Workflow

#### Rapid Prototyping
- **Material Tweaking**: Instantly see material changes
- **Layout Adjustments**: Real-time scene layout updates
- **Animation Preview**: Live animation playback during editing

#### Script Development
- **Behavior Testing**: Test script changes without scene restart
- **Debug Iteration**: Quick fix-and-test cycles
- **State Preservation**: Maintain game state during script updates

### Runtime Applications

#### Dynamic Content
- **Live Events**: Update scenes during live events
- **User-Generated Content**: Hot-reload user-created content
- **Remote Updates**: Update content from remote servers

#### Educational Tools
- **Interactive Learning**: Show code changes instantly
- **Tutorial Updates**: Modify tutorial content on-the-fly
- **Live Demos**: Update demos without interruption

## Performance Metrics

### Reload Performance

Typical performance characteristics:

- **Asset Reload**: 50-200ms for textures/materials
- **Entity Update**: 10-50ms for property changes
- **Script Reload**: 100-500ms for TypeScript compilation
- **Scene Reload**: 1-5 seconds for large scenes

### Memory Overhead

Hot-reload system memory usage:

- **Change Detection**: ~1-2MB for file watchers
- **State Preservation**: ~10-50MB depending on scene complexity
- **Asset Cache**: ~50-200MB for loaded assets
- **Script Cache**: ~5-20MB for compiled scripts

## Future Enhancements

### Advanced Features

- **Selective Reload**: Reload only specific scene regions
- **Collaborative Reload**: Synchronize reloads across multiple users
- **Version Control Integration**: Git-aware hot-reload
- **AI-Assisted Reload**: Intelligent change analysis and application
- **Cloud Reload**: Reload from cloud-based asset management
