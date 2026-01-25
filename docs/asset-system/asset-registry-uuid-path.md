# Asset Registry (UUID → Path)

## Overview

Rengine implements a robust asset registry system that maps universally unique identifiers (UUIDs) to filesystem paths, enabling reliable asset referencing across scenes, prefabs, and runtime systems. This system ensures assets remain accessible even when files are moved or renamed.

## Core Concepts

### UUID-Based Identification

Assets are identified by UUIDs rather than paths:

- **Stable References**: UUIDs don't change when files are moved
- **Cross-Platform**: UUIDs work across different operating systems
- **Merge-Friendly**: UUIDs prevent conflicts during collaboration
- **Database-Ready**: UUIDs integrate well with databases and APIs

### Path Mapping

Registry maintains bidirectional mapping:

- **UUID → Path**: Find asset location by ID
- **Path → UUID**: Find asset ID by location
- **Path Tracking**: Monitor filesystem changes
- **Reference Counting**: Track asset usage

## Implementation Details

### Asset Registry Structure

```typescript
interface AssetRegistry {
  // UUID to asset metadata mapping
  assets: Map<string, AssetMetadata>;

  // Path to UUID mapping for reverse lookups
  pathIndex: Map<string, string>;

  // Asset type categorization
  typeIndex: Map<AssetType, Set<string>>;
}

interface AssetMetadata {
  id: string; // UUID
  path: string; // Filesystem path
  type: AssetType;
  name: string; // Display name
  hash: string; // Content hash for change detection
  size: number; // File size in bytes
  modified: Date; // Last modification time
  imported: Date; // Import timestamp
  dependencies: string[]; // Dependent asset IDs
  tags: string[]; // Categorization tags
  metadata: Record<string, any>; // Type-specific metadata
}

enum AssetType {
  TEXTURE = 'texture',
  MODEL = 'model',
  MATERIAL = 'material',
  AUDIO = 'audio',
  SCRIPT = 'script',
  SCENE = 'scene',
  PREFAB = 'prefab',
  ANIMATION = 'animation',
  FONT = 'font',
  SHADER = 'shader'
}
```

### Registry Operations

```typescript
class AssetRegistryImpl implements AssetRegistry {
  private assets = new Map<string, AssetMetadata>();
  private pathIndex = new Map<string, string>();
  private typeIndex = new Map<AssetType, Set<string>>();

  registerAsset(path: string, type: AssetType, metadata: Partial<AssetMetadata> = {}): string {
    // Generate UUID if not provided
    const id = metadata.id || generateUUID();

    // Check for path conflicts
    if (this.pathIndex.has(path)) {
      throw new Error(`Asset already registered at path: ${path}`);
    }

    // Create asset metadata
    const assetMetadata: AssetMetadata = {
      id,
      path,
      type,
      name: metadata.name || getFilenameFromPath(path),
      hash: metadata.hash || '',
      size: metadata.size || 0,
      modified: metadata.modified || new Date(),
      imported: new Date(),
      dependencies: metadata.dependencies || [],
      tags: metadata.tags || [],
      metadata: metadata.metadata || {}
    };

    // Register asset
    this.assets.set(id, assetMetadata);
    this.pathIndex.set(path, id);

    // Update type index
    if (!this.typeIndex.has(type)) {
      this.typeIndex.set(type, new Set());
    }
    this.typeIndex.get(type)!.add(id);

    return id;
  }

  unregisterAsset(id: string): void {
    const asset = this.assets.get(id);
    if (!asset) return;

    // Remove from indices
    this.assets.delete(id);
    this.pathIndex.delete(asset.path);
    this.typeIndex.get(asset.type)?.delete(id);

    // Notify dependents
    this.notifyAssetRemoved(id);
  }

  getAsset(id: string): AssetMetadata | undefined {
    return this.assets.get(id);
  }

  getAssetByPath(path: string): AssetMetadata | undefined {
    const id = this.pathIndex.get(path);
    return id ? this.assets.get(id) : undefined;
  }

  getAssetsByType(type: AssetType): AssetMetadata[] {
    const ids = this.typeIndex.get(type) || new Set();
    return Array.from(ids).map(id => this.assets.get(id)!).filter(Boolean);
  }

  updateAssetPath(id: string, newPath: string): void {
    const asset = this.assets.get(id);
    if (!asset) return;

    // Check for conflicts
    if (this.pathIndex.has(newPath) && this.pathIndex.get(newPath) !== id) {
      throw new Error(`Path already occupied: ${newPath}`);
    }

    // Update path index
    this.pathIndex.delete(asset.path);
    this.pathIndex.set(newPath, id);

    // Update asset metadata
    asset.path = newPath;
    asset.modified = new Date();

    // Notify dependents
    this.notifyAssetPathChanged(id, asset.path, newPath);
  }
}
```

## UUID Generation

### RFC 4122 UUIDs

Registry uses standard UUID format:

```typescript
function generateUUID(): string {
  // RFC 4122 version 4 (random) UUID
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    const r = Math.random() * 16 | 0;
    const v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

// Example UUID: "550e8400-e29b-41d4-a716-446655440000"
```

### Deterministic UUIDs (Optional)

For reproducible builds:

```typescript
function generateDeterministicUUID(seed: string): string {
  // Use hash of path/content for deterministic ID
  const hash = createHash('sha256').update(seed).digest('hex');
  // Convert first 16 bytes to UUID format
  return [
    hash.substr(0, 8),
    hash.substr(8, 4),
    '4' + hash.substr(12, 3), // Version 4
    '8' + hash.substr(15, 3), // Variant 8
    hash.substr(18, 12)
  ].join('-');
}
```

## Path Resolution

### Absolute and Relative Paths

Registry handles different path formats:

```typescript
class PathResolver {
  private projectRoot: string;
  private assetRoot: string;

  constructor(projectRoot: string) {
    this.projectRoot = projectRoot;
    this.assetRoot = join(projectRoot, 'assets');
  }

  resolvePath(path: string): string {
    if (isAbsolute(path)) {
      return path;
    }

    // Handle asset:// protocol
    if (path.startsWith('asset://')) {
      return join(this.assetRoot, path.substr(8));
    }

    // Handle project-relative paths
    if (path.startsWith('./') || path.startsWith('../')) {
      return resolve(this.projectRoot, path);
    }

    // Default to asset root
    return join(this.assetRoot, path);
  }

  normalizePath(path: string): string {
    return relative(this.projectRoot, resolve(path)).replace(/\\/g, '/');
  }

  getAssetPath(assetId: string): string {
    const asset = assetRegistry.getAsset(assetId);
    return asset ? this.resolvePath(asset.path) : '';
  }
}
```

### Platform Independence

Cross-platform path handling:

```typescript
function normalizeAssetPath(path: string): string {
  // Convert backslashes to forward slashes
  let normalized = path.replace(/\\/g, '/');

  // Remove redundant separators
  normalized = normalized.replace(/\/+/g, '/');

  // Handle Windows drive letters
  if (/^[A-Za-z]:/.test(normalized)) {
    normalized = normalized.toLowerCase();
  }

  return normalized;
}

function getAssetKey(path: string): string {
  // Create platform-independent key for indexing
  return normalizeAssetPath(path).toLowerCase();
}
```

## Asset Discovery

### Automatic Registration

Registry can scan directories for assets:

```typescript
class AssetScanner {
  private supportedExtensions = new Map<AssetType, string[]>([
    [AssetType.TEXTURE, ['.png', '.jpg', '.jpeg', '.webp', '.tga']],
    [AssetType.MODEL, ['.gltf', '.glb', '.obj', '.fbx', '.dae']],
    [AssetType.AUDIO, ['.wav', '.mp3', '.ogg', '.m4a']],
    [AssetType.SCRIPT, ['.js', '.ts', '.lua']],
    [AssetType.SCENE, ['.json']],
    [AssetType.ANIMATION, ['.anim', '.fbx']],
  ]);

  async scanDirectory(directory: string): Promise<string[]> {
    const assetIds: string[] = [];

    const entries = await readdir(directory, { withFileTypes: true });

    for (const entry of entries) {
      const fullPath = join(directory, entry.name);

      if (entry.isDirectory()) {
        // Recursively scan subdirectories
        const subIds = await this.scanDirectory(fullPath);
        assetIds.push(...subIds);
      } else if (entry.isFile()) {
        const assetId = await this.registerFileAsset(fullPath);
        if (assetId) {
          assetIds.push(assetId);
        }
      }
    }

    return assetIds;
  }

  private async registerFileAsset(filePath: string): Promise<string | null> {
    const extension = extname(filePath).toLowerCase();
    const type = this.getAssetTypeFromExtension(extension);

    if (!type) return null;

    try {
      const stats = await stat(filePath);
      const hash = await this.calculateFileHash(filePath);

      return assetRegistry.registerAsset(filePath, type, {
        hash,
        size: stats.size,
        modified: stats.mtime
      });
    } catch (error) {
      console.warn(`Failed to register asset ${filePath}:`, error);
      return null;
    }
  }

  private getAssetTypeFromExtension(extension: string): AssetType | null {
    for (const [type, extensions] of this.supportedExtensions) {
      if (extensions.includes(extension)) {
        return type;
      }
    }
    return null;
  }

  private async calculateFileHash(filePath: string): Promise<string> {
    const fileBuffer = await readFile(filePath);
    return createHash('sha256').update(fileBuffer).digest('hex');
  }
}
```

## Change Detection

### Filesystem Monitoring

Registry monitors asset changes:

```typescript
class AssetWatcher {
  private watcher: FSWatcher;
  private changeQueue: AssetChange[] = [];

  constructor(assetRoot: string) {
    this.watcher = watch(assetRoot, {
      recursive: true,
      persistent: false
    });

    this.watcher.on('change', (eventType, filename) => {
      if (filename) {
        this.handleFileChange(eventType, filename);
      }
    });
  }

  private async handleFileChange(eventType: string, filename: string): Promise<void> {
    const fullPath = join(assetRoot, filename);
    const asset = assetRegistry.getAssetByPath(fullPath);

    if (!asset) {
      // New asset discovered
      if (eventType === 'add') {
        await assetScanner.registerFileAsset(fullPath);
      }
      return;
    }

    switch (eventType) {
      case 'change':
        await this.handleAssetModified(asset);
        break;
      case 'remove':
        assetRegistry.unregisterAsset(asset.id);
        break;
      case 'rename':
        // Handle file moves
        await this.handleAssetMoved(asset, fullPath);
        break;
    }
  }

  private async handleAssetModified(asset: AssetMetadata): Promise<void> {
    const newHash = await calculateFileHash(asset.path);
    const newStats = await stat(asset.path);

    if (newHash !== asset.hash) {
      // Asset content changed
      asset.hash = newHash;
      asset.modified = newStats.mtime;
      asset.size = newStats.size;

      // Notify dependents
      this.notifyAssetChanged(asset.id);
    }
  }
}
```

## Reference Tracking

### Dependency Management

Track asset relationships:

```typescript
class AssetDependencyTracker {
  private dependencies = new Map<string, Set<string>>();
  private dependents = new Map<string, Set<string>>();

  addDependency(assetId: string, dependencyId: string): void {
    // Track what assets this asset depends on
    if (!this.dependencies.has(assetId)) {
      this.dependencies.set(assetId, new Set());
    }
    this.dependencies.get(assetId)!.add(dependencyId);

    // Track what assets depend on this asset
    if (!this.dependents.has(dependencyId)) {
      this.dependents.set(dependencyId, new Set());
    }
    this.dependents.get(dependencyId)!.add(assetId);
  }

  getDependencies(assetId: string): string[] {
    return Array.from(this.dependencies.get(assetId) || []);
  }

  getDependents(assetId: string): string[] {
    return Array.from(this.dependents.get(assetId) || []);
  }

  removeAsset(assetId: string): void {
    // Remove from dependency tracking
    this.dependencies.delete(assetId);

    // Remove from dependents
    for (const [depId, deps] of this.dependents) {
      deps.delete(assetId);
      if (deps.size === 0) {
        this.dependents.delete(depId);
      }
    }
  }

  getLoadOrder(assetIds: string[]): string[] {
    // Topological sort for loading order
    const visited = new Set<string>();
    const order: string[] = [];

    function visit(id: string) {
      if (visited.has(id)) return;
      visited.add(id);

      // Visit dependencies first
      for (const depId of this.getDependencies(id)) {
        visit(depId);
      }

      order.push(id);
    }

    for (const id of assetIds) {
      visit(id);
    }

    return order;
  }
}
```

## Serialization

### Registry Persistence

Save/load registry state:

```typescript
interface SerializedAssetRegistry {
  version: string;
  assets: SerializedAssetMetadata[];
  lastScan: string;
}

interface SerializedAssetMetadata {
  id: string;
  path: string;
  type: AssetType;
  name: string;
  hash: string;
  size: number;
  modified: string;
  imported: string;
  dependencies: string[];
  tags: string[];
  metadata: Record<string, any>;
}

function serializeRegistry(): SerializedAssetRegistry {
  return {
    version: '1.0.0',
    lastScan: new Date().toISOString(),
    assets: Array.from(assetRegistry.assets.values()).map(asset => ({
      id: asset.id,
      path: asset.path,
      type: asset.type,
      name: asset.name,
      hash: asset.hash,
      size: asset.size,
      modified: asset.modified.toISOString(),
      imported: asset.imported.toISOString(),
      dependencies: asset.dependencies,
      tags: asset.tags,
      metadata: asset.metadata
    }))
  };
}

async function deserializeRegistry(data: SerializedAssetRegistry): Promise<void> {
  for (const assetData of data.assets) {
    assetRegistry.registerAsset(assetData.path, assetData.type, {
      id: assetData.id,
      name: assetData.name,
      hash: assetData.hash,
      size: assetData.size,
      modified: new Date(assetData.modified),
      imported: new Date(assetData.imported),
      dependencies: assetData.dependencies,
      tags: assetData.tags,
      metadata: assetData.metadata
    });
  }
}
```

## Performance Optimizations

### Indexing Strategies

Efficient asset lookup:

```typescript
class AssetIndex {
  private byId = new Map<string, AssetMetadata>();
  private byPath = new Map<string, AssetMetadata>();
  private byType = new Map<AssetType, AssetMetadata[]>();
  private byTag = new Map<string, AssetMetadata[]>();
  private byName = new Map<string, AssetMetadata[]>();

  addAsset(asset: AssetMetadata): void {
    this.byId.set(asset.id, asset);
    this.byPath.set(asset.path, asset);

    // Type index
    if (!this.byType.has(asset.type)) {
      this.byType.set(asset.type, []);
    }
    this.byType.get(asset.type)!.push(asset);

    // Tag index
    for (const tag of asset.tags) {
      if (!this.byTag.has(tag)) {
        this.byTag.set(tag, []);
      }
      this.byTag.get(tag)!.push(asset);
    }

    // Name index
    const nameKey = asset.name.toLowerCase();
    if (!this.byName.has(nameKey)) {
      this.byName.set(nameKey, []);
    }
    this.byName.get(nameKey)!.push(asset);
  }

  searchAssets(query: AssetSearchQuery): AssetMetadata[] {
    let results: AssetMetadata[] = [];

    if (query.type) {
      results = this.byType.get(query.type) || [];
    } else {
      results = Array.from(this.byId.values());
    }

    if (query.tags && query.tags.length > 0) {
      results = results.filter(asset =>
        query.tags!.every(tag => asset.tags.includes(tag))
      );
    }

    if (query.name) {
      const namePattern = query.name.toLowerCase();
      results = results.filter(asset =>
        asset.name.toLowerCase().includes(namePattern)
      );
    }

    return results;
  }
}
```

## Error Handling

### Robust Operations

Handle filesystem issues gracefully:

```typescript
class AssetRegistryErrorHandler {
  async safeRegisterAsset(path: string, type: AssetType): Promise<string | null> {
    try {
      // Validate path exists and is accessible
      await access(path);

      // Check file size limits
      const stats = await stat(path);
      if (stats.size > MAX_ASSET_SIZE) {
        throw new Error(`Asset too large: ${stats.size} bytes`);
      }

      return await assetRegistry.registerAsset(path, type);
    } catch (error) {
      console.error(`Failed to register asset ${path}:`, error);

      // Attempt recovery strategies
      if (error.code === 'ENOENT') {
        // File not found - could be temporary
        return null;
      }

      if (error.code === 'EACCES') {
        // Permission denied - notify user
        this.notifyPermissionError(path);
        return null;
      }

      throw error; // Re-throw unhandled errors
    }
  }

  private notifyPermissionError(path: string): void {
    // Show user notification about permission issues
    ui.showNotification(
      `Cannot access asset: ${path}. Check file permissions.`,
      'error'
    );
  }
}
```

## Best Practices

### Registry Management

- **Regular Scanning**: Keep registry synchronized with filesystem
- **Conflict Resolution**: Handle duplicate UUIDs gracefully
- **Backup Registry**: Maintain backups of registry state
- **Version Control**: Track registry changes in version control

### Asset Organization

- **Logical Structure**: Organize assets in hierarchical directories
- **Naming Conventions**: Use consistent naming for assets
- **Metadata Enrichment**: Add descriptive metadata to assets
- **Dependency Awareness**: Understand asset relationships

### Performance

- **Lazy Loading**: Load asset metadata on demand
- **Caching**: Cache frequently accessed assets
- **Batch Operations**: Process multiple assets together
- **Memory Limits**: Monitor memory usage of loaded assets

## Future Enhancements

### Advanced Features

- **Distributed Registry**: Registry synchronization across multiple machines
- **Asset Versioning**: Track asset versions and changes over time
- **Asset Templates**: Reusable asset configurations
- **Smart Import**: Automatic asset type detection and import
- **Asset Optimization**: Automatic asset compression and optimization
