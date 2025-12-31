# Reference Counting / Lifetime

## Overview

Rengine implements a sophisticated reference counting system for asset lifetime management, ensuring assets are loaded when needed and unloaded when no longer referenced. This prevents memory leaks, optimizes memory usage, and provides automatic resource management.

## Core Concepts

### Reference Counting

Assets track how many times they're referenced:

- **Acquire Reference**: Increment count when asset is used
- **Release Reference**: Decrement count when asset is no longer needed
- **Automatic Unloading**: Unload asset when reference count reaches zero
- **Thread Safety**: Reference operations are thread-safe

### Asset Lifetime States

Assets transition through different lifetime states:

```
Unused → Loading → Loaded → Unloading → Unloaded
    ↑                                       ↓
    └─────────────── Reference ─────────────┘
```

## Implementation Details

### Reference Counter

```typescript
class AssetReferenceCounter {
  private references = new Map<string, number>();
  private loadingAssets = new Set<string>();
  private unloadingAssets = new Set<string>();

  acquireReference(assetId: string): void {
    const currentCount = this.references.get(assetId) || 0;
    this.references.set(assetId, currentCount + 1);

    // If this is the first reference, start loading
    if (currentCount === 0) {
      this.startLoadingAsset(assetId);
    }
  }

  releaseReference(assetId: string): void {
    const currentCount = this.references.get(assetId) || 0;

    if (currentCount <= 0) {
      console.warn(`Attempting to release reference for unreferenced asset: ${assetId}`);
      return;
    }

    const newCount = currentCount - 1;
    this.references.set(assetId, newCount);

    // If no more references, start unloading
    if (newCount === 0) {
      this.startUnloadingAsset(assetId);
    }
  }

  getReferenceCount(assetId: string): number {
    return this.references.get(assetId) || 0;
  }

  isAssetLoaded(assetId: string): boolean {
    return this.references.has(assetId) && this.references.get(assetId)! > 0;
  }

  private async startLoadingAsset(assetId: string): Promise<void> {
    if (this.loadingAssets.has(assetId)) return;

    this.loadingAssets.add(assetId);

    try {
      await assetLoader.loadAsset(assetId);
      this.loadingAssets.delete(assetId);
    } catch (error) {
      this.loadingAssets.delete(assetId);
      console.error(`Failed to load asset ${assetId}:`, error);
      // Reset reference count on load failure
      this.references.delete(assetId);
    }
  }

  private async startUnloadingAsset(assetId: string): Promise<void> {
    if (this.unloadingAssets.has(assetId)) return;

    this.unloadingAssets.add(assetId);

    try {
      await assetLoader.unloadAsset(assetId);
      this.unloadingAssets.delete(assetId);
      this.references.delete(assetId);
    } catch (error) {
      this.unloadingAssets.delete(assetId);
      console.error(`Failed to unload asset ${assetId}:`, error);
    }
  }
}
```

### Asset Handle System

Provides safe asset access with automatic reference management:

```typescript
class AssetHandle<T = any> {
  private assetId: string;
  private asset: T | null = null;
  private disposed = false;

  constructor(assetId: string) {
    this.assetId = assetId;
    assetReferenceCounter.acquireReference(assetId);
  }

  async getAsset(): Promise<T> {
    if (this.disposed) {
      throw new Error('Asset handle has been disposed');
    }

    if (this.asset) {
      return this.asset;
    }

    // Wait for asset to load
    this.asset = await assetLoader.getLoadedAsset(this.assetId);
    return this.asset;
  }

  dispose(): void {
    if (this.disposed) return;

    this.disposed = true;
    this.asset = null;
    assetReferenceCounter.releaseReference(this.assetId);
  }

  isValid(): boolean {
    return !this.disposed && assetReferenceCounter.isAssetLoaded(this.assetId);
  }

  getAssetId(): string {
    return this.assetId;
  }
}

// Usage with automatic cleanup
function useAsset<T>(assetId: string): AssetHandle<T> {
  const handle = new AssetHandle<T>(assetId);

  // Automatic cleanup on scope exit (in async context)
  return handle;
}
```

## Automatic Reference Management

### Component Integration

Components automatically manage asset references:

```typescript
class AssetReferencingComponent {
  private assetHandles: Map<string, AssetHandle> = new Map();

  setAssetReference(propertyName: string, assetId: string): void {
    // Release previous reference
    const previousHandle = this.assetHandles.get(propertyName);
    if (previousHandle) {
      previousHandle.dispose();
    }

    // Acquire new reference
    if (assetId) {
      const newHandle = new AssetHandle(assetId);
      this.assetHandles.set(propertyName, newHandle);
    } else {
      this.assetHandles.delete(propertyName);
    }
  }

  async getAsset(propertyName: string): Promise<any> {
    const handle = this.assetHandles.get(propertyName);
    return handle ? await handle.getAsset() : null;
  }

  dispose(): void {
    // Release all asset references
    for (const handle of this.assetHandles.values()) {
      handle.dispose();
    }
    this.assetHandles.clear();
  }
}

// Example: Material component with texture references
class MaterialComponent extends AssetReferencingComponent {
  constructor() {
    super();
  }

  setDiffuseTexture(textureId: string): void {
    this.setAssetReference('diffuseTexture', textureId);
  }

  setNormalTexture(textureId: string): void {
    this.setAssetReference('normalTexture', textureId);
  }

  async getDiffuseTexture(): Promise<Texture> {
    return await this.getAsset('diffuseTexture');
  }

  async getNormalTexture(): Promise<Texture> {
    return await this.getAsset('normalTexture');
  }
}
```

### Scene Graph Integration

Scene entities manage asset references through their components:

```typescript
class SceneEntity {
  private components: Map<string, Component> = new Map();

  addComponent(component: Component): void {
    this.components.set(component.type, component);
  }

  removeComponent(componentType: string): void {
    const component = this.components.get(componentType);
    if (component && typeof component.dispose === 'function') {
      component.dispose();
    }
    this.components.delete(componentType);
  }

  dispose(): void {
    // Dispose all components, releasing their asset references
    for (const component of this.components.values()) {
      if (typeof component.dispose === 'function') {
        component.dispose();
      }
    }
    this.components.clear();
  }
}

class Scene {
  private entities: Map<string, SceneEntity> = new Map();

  addEntity(entity: SceneEntity): void {
    this.entities.set(entity.id, entity);
  }

  removeEntity(entityId: string): void {
    const entity = this.entities.get(entityId);
    if (entity) {
      entity.dispose(); // Releases all asset references
      this.entities.delete(entityId);
    }
  }

  dispose(): void {
    // Dispose all entities, cascading asset reference releases
    for (const entity of this.entities.values()) {
      entity.dispose();
    }
    this.entities.clear();
  }
}
```

## Memory Management

### Garbage Collection Strategy

Intelligent unloading based on memory pressure:

```typescript
class AssetGarbageCollector {
  private memoryThreshold = 512 * 1024 * 1024; // 512MB
  private lastCollection = Date.now();

  async collectGarbage(): Promise<void> {
    const currentMemory = this.getCurrentMemoryUsage();

    if (currentMemory < this.memoryThreshold) {
      return; // No need to collect
    }

    console.log(`Memory usage high (${currentMemory} bytes), collecting garbage...`);

    // Find assets that can be unloaded (reference count = 0)
    const unloadableAssets = this.findUnloadableAssets();

    // Sort by unload priority (LRU, size, etc.)
    unloadableAssets.sort((a, b) => this.getUnloadPriority(a) - this.getUnloadPriority(b));

    // Unload assets until memory usage is acceptable
    for (const assetId of unloadableAssets) {
      if (this.getCurrentMemoryUsage() < this.memoryThreshold * 0.8) {
        break; // Memory usage is now acceptable
      }

      await assetLoader.unloadAsset(assetId);
    }

    this.lastCollection = Date.now();
  }

  private findUnloadableAssets(): string[] {
    const unloadable: string[] = [];

    for (const [assetId, refCount] of assetReferenceCounter.getAllReferenceCounts()) {
      if (refCount === 0 && !assetLoader.isAssetLoading(assetId)) {
        unloadable.push(assetId);
      }
    }

    return unloadable;
  }

  private getUnloadPriority(assetId: string): number {
    const metadata = assetRegistry.getAsset(assetId);
    if (!metadata) return 0;

    // Priority based on:
    // - Time since last access (lower = higher priority to unload)
    // - Asset size (larger = higher priority to unload)
    // - Asset type priority

    const timeSinceAccess = Date.now() - (metadata.lastAccess || 0);
    const sizePriority = metadata.size / 1024 / 1024; // MB
    const typePriority = this.getAssetTypePriority(metadata.type);

    return timeSinceAccess / 1000 + sizePriority * 10 + typePriority;
  }

  private getAssetTypePriority(type: AssetType): number {
    const priorities: Record<AssetType, number> = {
      [AssetType.TEXTURE]: 1,    // Easy to reload
      [AssetType.MODEL]: 2,      // Moderate reload cost
      [AssetType.AUDIO]: 1,      // Easy to reload
      [AssetType.SCRIPT]: 3,     // Expensive to reload
      [AssetType.SCENE]: 3,      // Expensive to reload
      [AssetType.ANIMATION]: 2,  // Moderate reload cost
      [AssetType.MATERIAL]: 1,   // Easy to reload
      [AssetType.SHADER]: 3,     // Very expensive to reload
      [AssetType.FONT]: 2,       // Moderate reload cost
      [AssetType.PREFAB]: 2      // Moderate reload cost
    };

    return priorities[type] || 1;
  }

  private getCurrentMemoryUsage(): number {
    // Platform-specific memory monitoring
    if (typeof performance !== 'undefined' && performance.memory) {
      return performance.memory.usedJSHeapSize;
    }

    // Fallback estimation
    return this.estimateMemoryUsage();
  }
}
```

### Weak References

Handle cyclic references safely:

```typescript
class WeakAssetReference {
  private assetId: string;
  private weakRef: WeakRef<AssetHandle>;

  constructor(handle: AssetHandle) {
    this.assetId = handle.getAssetId();
    this.weakRef = new WeakRef(handle);
  }

  getAssetId(): string {
    return this.assetId;
  }

  isValid(): boolean {
    const handle = this.weakRef.deref();
    return handle !== undefined && handle.isValid();
  }

  async getAsset(): Promise<any> {
    const handle = this.weakRef.deref();
    if (handle && handle.isValid()) {
      return await handle.getAsset();
    }
    throw new Error('Asset reference is no longer valid');
  }
}

// Usage for cache-like structures
class AssetCache {
  private cache = new Map<string, WeakAssetReference>();

  get(assetId: string): WeakAssetReference | undefined {
    return this.cache.get(assetId);
  }

  put(assetId: string, handle: AssetHandle): void {
    this.cache.set(assetId, new WeakAssetReference(handle));
  }

  cleanup(): void {
    // Remove invalid references
    for (const [assetId, ref] of this.cache) {
      if (!ref.isValid()) {
        this.cache.delete(assetId);
      }
    }
  }
}
```

## Loading Strategies

### Preloading

Load assets before they're needed:

```typescript
class AssetPreloader {
  private preloadQueue: string[] = [];
  private maxConcurrentLoads = 4;

  preloadAssets(assetIds: string[]): Promise<void> {
    this.preloadQueue.push(...assetIds);
    return this.processPreloadQueue();
  }

  private async processPreloadQueue(): Promise<void> {
    const loadingPromises: Promise<void>[] = [];

    while (this.preloadQueue.length > 0 && loadingPromises.length < this.maxConcurrentLoads) {
      const assetId = this.preloadQueue.shift()!;
      const loadPromise = this.preloadAsset(assetId);
      loadingPromises.push(loadPromise);

      // Remove completed promises
      loadingPromises.filter(p => {
        const isCompleted = p === loadPromise && this.isAssetLoaded(assetId);
        if (isCompleted) {
          // Keep reference to prevent unloading
          assetReferenceCounter.acquireReference(assetId);
        }
        return !isCompleted;
      });
    }

    await Promise.all(loadingPromises);
  }

  private async preloadAsset(assetId: string): Promise<void> {
    const handle = new AssetHandle(assetId);
    try {
      await handle.getAsset();
      // Keep handle alive until explicitly released
      this.keepAliveHandles.set(assetId, handle);
    } catch (error) {
      console.error(`Failed to preload asset ${assetId}:`, error);
      handle.dispose();
    }
  }
}
```

### Streaming Loading

Load assets progressively:

```typescript
class StreamingAssetLoader {
  async loadAssetStreaming(assetId: string, onProgress?: (progress: number) => void): Promise<AssetHandle> {
    const handle = new AssetHandle(assetId);
    const asset = await assetLoader.getAssetMetadata(assetId);

    if (asset.type === AssetType.MODEL) {
      await this.loadModelStreaming(asset, onProgress);
    } else if (asset.type === AssetType.TEXTURE) {
      await this.loadTextureStreaming(asset, onProgress);
    } else {
      // Regular loading for other types
      await handle.getAsset();
    }

    return handle;
  }

  private async loadModelStreaming(asset: AssetMetadata, onProgress?: (progress: number) => void): Promise<void> {
    const response = await fetch(asset.path);
    const contentLength = parseInt(response.headers.get('content-length') || '0');

    const reader = response.body?.getReader();
    if (!reader) throw new Error('Failed to get stream reader');

    const chunks: Uint8Array[] = [];
    let receivedLength = 0;

    while (true) {
      const { done, value } = await reader.read();

      if (done) break;

      chunks.push(value);
      receivedLength += value.length;

      const progress = contentLength > 0 ? receivedLength / contentLength : 0;
      onProgress?.(progress);
    }

    // Assemble and parse the model
    const fullBuffer = new Uint8Array(receivedLength);
    let offset = 0;
    for (const chunk of chunks) {
      fullBuffer.set(chunk, offset);
      offset += chunk.length;
    }

    // Parse and create the model
    const model = await parseModelBuffer(fullBuffer);
    assetLoader.storeLoadedAsset(asset.id, model);
  }
}
```

## Error Handling

### Reference Counting Errors

Handle reference counting edge cases:

```typescript
class ReferenceCountingErrorHandler {
  private errorCounts = new Map<string, number>();

  handleReferenceError(assetId: string, operation: 'acquire' | 'release', error: Error): void {
    const errorKey = `${assetId}_${operation}`;
    const count = this.errorCounts.get(errorKey) || 0;

    if (count < 3) { // Allow some errors before escalating
      console.warn(`Reference counting error for ${assetId} (${operation}):`, error);
      this.errorCounts.set(errorKey, count + 1);
    } else {
      console.error(`Persistent reference counting error for ${assetId} (${operation}):`, error);
      this.attemptRecovery(assetId, operation);
    }
  }

  private attemptRecovery(assetId: string, operation: 'acquire' | 'release'): void {
    if (operation === 'acquire') {
      // Force acquire reference
      assetReferenceCounter.forceAcquireReference(assetId);
    } else {
      // Force release reference
      assetReferenceCounter.forceReleaseReference(assetId);
    }

    // Reset error count
    this.errorCounts.delete(`${assetId}_${operation}`);
  }
}
```

## Performance Monitoring

### Reference Counting Metrics

Monitor reference counting performance:

```typescript
class ReferenceCountingMetrics {
  private acquireCount = 0;
  private releaseCount = 0;
  private loadCount = 0;
  private unloadCount = 0;
  private errorCount = 0;

  recordAcquire(): void {
    this.acquireCount++;
  }

  recordRelease(): void {
    this.releaseCount++;
  }

  recordLoad(): void {
    this.loadCount++;
  }

  recordUnload(): void {
    this.unloadCount++;
  }

  recordError(): void {
    this.errorCount++;
  }

  getReport(): ReferenceCountingReport {
    return {
      totalReferences: this.acquireCount - this.releaseCount,
      loadUnloadRatio: this.loadCount / Math.max(this.unloadCount, 1),
      errorRate: this.errorCount / Math.max(this.acquireCount + this.releaseCount, 1),
      averageReferenceLifetime: this.calculateAverageLifetime()
    };
  }

  private calculateAverageLifetime(): number {
    // Simplified calculation - would need more detailed tracking
    return (this.acquireCount + this.releaseCount) / Math.max(this.loadCount, 1);
  }
}
```

## Best Practices

### Reference Management

- **RAII Pattern**: Use AssetHandle in try-finally blocks
- **Scope Awareness**: Release references when objects go out of scope
- **Exception Safety**: Ensure references are released even on errors
- **Testing**: Test reference counting edge cases

### Memory Management

- **Proactive Collection**: Run garbage collection during loading screens
- **Memory Budgets**: Set memory limits for different asset types
- **Streaming Priority**: Stream critical assets first
- **Cache Warming**: Preload frequently used assets

### Debugging

- **Reference Leak Detection**: Monitor for assets that never get unloaded
- **Circular Reference Prevention**: Avoid reference cycles
- **Memory Profiling**: Track memory usage by asset type
- **Load Time Monitoring**: Measure asset loading performance

## Future Enhancements

### Advanced Features

- **Distributed References**: Reference counting across network
- **Reference Prediction**: Predictive loading based on usage patterns
- **Asset Pooling**: Object pooling for frequently used assets
- **Memory Pressure Response**: Adaptive loading based on system memory
- **Reference Debugging**: Visual tools for debugging reference issues
