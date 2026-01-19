# Versioned Engine Core

## Overview

Rengine's engine core follows a strict versioning system that ensures stability, backward compatibility, and clear upgrade paths. The versioned architecture allows for incremental improvements while maintaining compatibility with existing projects and plugins.

## Versioning Strategy

### Core Version Components

#### Engine Version
```
MAJOR.MINOR.PATCH-BUILD

Examples:
- 1.0.0-stable    (Production release)
- 1.1.0-beta      (Beta release)
- 2.0.0-alpha     (Major version in development)
- 1.0.5-hotfix    (Patch release)
```

#### API Version
```
API_MAJOR.API_MINOR

Examples:
- 1.0    (Stable API)
- 1.1    (Backward compatible additions)
- 2.0    (Breaking changes)
```

#### File Format Version
```
FORMAT_MAJOR.FORMAT_MINOR

Examples:
- scene-1.0    (Scene file format)
- asset-1.1    (Asset format)
- plugin-2.0   (Plugin manifest format)
```

## Core Architecture

### Modular Core Design

#### Core Modules
```typescript
interface EngineCore {
  // Core subsystems
  readonly scene: SceneSystem;
  readonly render: RenderSystem;
  readonly input: InputSystem;
  readonly asset: AssetSystem;

  // Version information
  readonly version: EngineVersion;
  readonly apiVersion: APIVersion;

  // Lifecycle management
  initialize(): Promise<void>;
  shutdown(): Promise<void>;
}
```

#### System Interfaces
```typescript
interface CoreSystem {
  readonly name: string;
  readonly version: SystemVersion;

  initialize(): Promise<void>;
  update(deltaTime: number): void;
  shutdown(): Promise<void>;

  // Version compatibility
  getSupportedVersions(): VersionRange[];
  isCompatible(version: Version): boolean;
}
```

### Version Management System

#### Version Registry
```typescript
class VersionManager {
  private versionMap = new Map<string, VersionInfo>();

  registerVersion(systemName: string, version: VersionInfo): void {
    this.versionMap.set(systemName, version);
  }

  getVersion(systemName: string): VersionInfo | undefined {
    return this.versionMap.get(systemName);
  }

  checkCompatibility(systemName: string, requiredVersion: Version): boolean {
    const currentVersion = this.getVersion(systemName);
    if (!currentVersion) return false;

    return this.isVersionCompatible(currentVersion, requiredVersion);
  }
}
```

#### Compatibility Checking
```typescript
function isVersionCompatible(current: Version, required: Version): boolean {
  // Major version must match for compatibility
  if (current.major !== required.major) {
    return false;
  }

  // Minor version can be higher (backward compatible)
  if (current.minor < required.minor) {
    return false;
  }

  // Patch version can be higher or equal
  return current.patch >= required.patch;
}
```

## File Format Versioning

### Scene File Format

#### Version Header
```json
{
  "rengine": {
    "version": "1.0.0",
    "format": "scene-1.0",
    "created": "2024-01-01T00:00:00Z",
    "generator": "Rengine 1.0.0"
  },
  "scene": {
    "name": "My Scene",
    "metadata": { ... }
  }
}
```

#### Backward Compatibility
```typescript
class SceneLoader {
  async loadScene(filePath: string): Promise<SceneState> {
    const rawData = await this.readFile(filePath);
    const version = this.extractVersion(rawData);

    // Route to appropriate loader based on version
    switch (version.format) {
      case 'scene-1.0':
        return this.loadSceneV1(rawData);
      case 'scene-0.5':
        return this.migrateSceneV05(rawData);
      default:
        throw new Error(`Unsupported scene format: ${version.format}`);
    }
  }

  private migrateSceneV05(oldData: any): SceneState {
    // Migration logic for old format
    return {
      ...oldData,
      // Apply migrations for breaking changes
      objects: oldData.objects.map(this.migrateObjectV05),
      lights: oldData.lights.map(this.migrateLightV05)
    };
  }
}
```

### Asset Versioning

#### Asset Metadata
```typescript
interface AssetMetadata {
  id: string;
  name: string;
  type: AssetType;
  version: string;
  formatVersion: string;
  created: Date;
  modified: Date;
  dependencies: AssetDependency[];
  checksum: string;
}
```

#### Version-Aware Asset Loading
```typescript
class AssetLoader {
  async loadAsset(assetPath: string): Promise<Asset> {
    const metadata = await this.loadAssetMetadata(assetPath);
    const loader = this.getLoaderForVersion(metadata.formatVersion);

    const asset = await loader.load(assetPath);

    // Apply version-specific processing
    return this.processAsset(asset, metadata);
  }

  private getLoaderForVersion(formatVersion: string): AssetLoader {
    const versionMap = {
      'model-1.0': new ModelLoaderV1(),
      'texture-1.0': new TextureLoaderV1(),
      'material-1.1': new MaterialLoaderV1_1()
    };

    const loader = versionMap[formatVersion];
    if (!loader) {
      throw new Error(`No loader available for format version: ${formatVersion}`);
    }

    return loader;
  }
}
```

## Plugin Versioning

### Plugin Manifest Versioning
```json
{
  "name": "my-plugin",
  "version": "1.0.0",
  "rengine": {
    "apiVersion": "1.0",
    "engineVersion": ">=1.0.0 <2.0.0",
    "formatVersions": {
      "scene": "1.0",
      "asset": "1.0"
    }
  },
  "dependencies": {
    "other-plugin": "^1.0.0"
  }
}
```

### Version Resolution
```typescript
class PluginManager {
  async loadPlugin(pluginPath: string): Promise<Plugin> {
    const manifest = await this.loadManifest(pluginPath);

    // Check engine compatibility
    if (!this.isEngineCompatible(manifest.rengine.engineVersion)) {
      throw new Error(`Plugin requires engine version ${manifest.rengine.engineVersion}`);
    }

    // Check API compatibility
    if (!this.isAPICompatible(manifest.rengine.apiVersion)) {
      throw new Error(`Plugin requires API version ${manifest.rengine.apiVersion}`);
    }

    // Load and initialize plugin
    return this.initializePlugin(pluginPath, manifest);
  }
}
```

## Migration System

### Automatic Migration
```typescript
interface Migration {
  fromVersion: Version;
  toVersion: Version;
  migrate: (data: any) => any;
}

class MigrationManager {
  private migrations = new Map<string, Migration[]>();

  registerMigration(system: string, migration: Migration): void {
    if (!this.migrations.has(system)) {
      this.migrations.set(system, []);
    }
    this.migrations.get(system)!.push(migration);
  }

  migrate(system: string, data: any, targetVersion: Version): any {
    const migrations = this.migrations.get(system) || [];
    let currentData = data;
    let currentVersion = this.extractVersion(data);

    // Apply migrations in order
    for (const migration of migrations) {
      if (this.shouldApplyMigration(currentVersion, migration, targetVersion)) {
        currentData = migration.migrate(currentData);
        currentVersion = migration.toVersion;
      }
    }

    return currentData;
  }
}
```

### Migration Examples

#### Scene Format Migration (0.5 → 1.0)
```typescript
// Migrate old scene format to new format
const sceneMigration: Migration = {
  fromVersion: { major: 0, minor: 5, patch: 0 },
  toVersion: { major: 1, minor: 0, patch: 0 },
  migrate: (oldScene: any) => {
    return {
      rengine: {
        version: "1.0.0",
        format: "scene-1.0"
      },
      scene: {
        name: oldScene.name || "Untitled Scene",
        objects: oldScene.objects.map(migrateObject),
        lights: oldScene.lights.map(migrateLight),
        metadata: {
          ...oldScene.metadata,
          migrated: true,
          migrationDate: new Date().toISOString()
        }
      }
    };
  }
};
```

#### Component Property Migration
```typescript
// Migrate object properties
function migrateObject(oldObject: any) {
  return {
    id: oldObject.id || generateId(),
    name: oldObject.name || `Object ${oldObject.id}`,
    type: oldObject.type,
    position: oldObject.position || [0, 0, 0],
    rotation: oldObject.rotation || [0, 0, 0],
    scale: oldObject.scale || [1, 1, 1],
    color: oldObject.color || "#ffffff",
    visible: oldObject.visible !== undefined ? oldObject.visible : true,
    // New properties with defaults
    castShadow: oldObject.castShadow !== undefined ? oldObject.castShadow : true,
    receiveShadow: oldObject.receiveShadow !== undefined ? oldObject.receiveShadow : true
  };
}
```

## Version Testing

### Compatibility Test Suite
```typescript
class VersionTestSuite {
  async runCompatibilityTests(): Promise<TestResults> {
    const results: TestResults = {
      passed: 0,
      failed: 0,
      errors: []
    };

    // Test scene file loading across versions
    for (const testFile of this.getTestFiles()) {
      try {
        await this.testSceneLoading(testFile);
        results.passed++;
      } catch (error) {
        results.failed++;
        results.errors.push({
          file: testFile,
          error: error.message
        });
      }
    }

    // Test plugin compatibility
    for (const plugin of this.getTestPlugins()) {
      try {
        await this.testPluginLoading(plugin);
        results.passed++;
      } catch (error) {
        results.failed++;
        results.errors.push({
          plugin: plugin.name,
          error: error.message
        });
      }
    }

    return results;
  }
}
```

### Version Validation
```typescript
function validateVersion(version: Version): boolean {
  // Check version format
  if (!version.major || !version.minor || !version.patch) {
    return false;
  }

  // Check version ranges
  if (version.major < 0 || version.minor < 0 || version.patch < 0) {
    return false;
  }

  // Check for pre-release/build metadata
  if (version.preRelease && !isValidPreRelease(version.preRelease)) {
    return false;
  }

  return true;
}
```

## Release Process

### Version Planning
1. **Feature Development**: New features developed in feature branches
2. **Version Assignment**: Assign appropriate version numbers
3. **Compatibility Testing**: Test against previous versions
4. **Migration Development**: Create migration paths for breaking changes

### Release Checklist
- [ ] Update version numbers in all relevant files
- [ ] Update CHANGELOG.md with release notes
- [ ] Run full test suite including compatibility tests
- [ ] Generate migration documentation
- [ ] Create backup/restore procedures for version upgrades
- [ ] Update plugin compatibility matrix
- [ ] Publish release notes and migration guides

### Post-Release Monitoring
- Monitor error reports for version-related issues
- Track plugin compatibility issues
- Gather user feedback on new features
- Plan next version based on feedback and roadmap

## Future Versioning Improvements

### Planned Enhancements
- **Automatic Migration Tools**: GUI tools for project migration
- **Version Branching**: Support for multiple active versions
- **Plugin Version Marketplace**: Version-aware plugin ecosystem
- **Cloud Version Management**: Server-side version tracking and migration
- **Advanced Compatibility Checking**: Machine learning-based compatibility prediction
