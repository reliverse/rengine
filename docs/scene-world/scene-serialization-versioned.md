# Scene Serialization (Versioned)

## Overview

Rengine implements a robust, versioned scene serialization system that ensures scenes remain compatible across different engine versions while preserving all scene data, entity relationships, and metadata.

## Core Concepts

### Versioned Format

Scene files include version information to enable:

- **Backward Compatibility**: Load scenes from older versions
- **Forward Compatibility**: Handle unknown data gracefully
- **Migration Support**: Automatic data transformation between versions
- **Validation**: Ensure data integrity during load/save operations

### Serialization Architecture

The serialization system consists of:

- **Scene Format**: JSON-based scene representation
- **Version Registry**: Maps version numbers to serialization logic
- **Migration Pipeline**: Transforms data between versions
- **Validation Layer**: Ensures data correctness

## File Format

### Scene File Structure

```json
{
  "version": "1.0.0",
  "metadata": {
    "name": "My Scene",
    "description": "A sample 3D scene",
    "created": "2024-01-01T00:00:00Z",
    "modified": "2024-01-01T01:00:00Z",
    "engineVersion": "0.2.0",
    "generator": "Rengine Editor"
  },
  "scene": {
    "backgroundColor": "#1e293b",
    "fog": {
      "enabled": false,
      "color": "#1e293b",
      "near": 1,
      "far": 100
    },
    "environment": {
      "ambientIntensity": 0.6,
      "shadowsEnabled": true
    }
  },
  "entities": [
    {
      "id": "object_1640995200000_abc123def",
      "name": "Cube",
      "type": "cube",
      "transform": {
        "position": [0, 0, 0],
        "rotation": [0, 0, 0],
        "scale": [1, 1, 1]
      },
      "components": {
        "renderer": {
          "material": {
            "color": "#ff0000",
            "metalness": 0.0,
            "roughness": 0.5
          },
          "castShadow": true,
          "receiveShadow": true
        }
      }
    }
  ],
  "lights": [
    {
      "id": "light_1640995260000_def456ghi",
      "name": "Directional Light",
      "type": "directional",
      "transform": {
        "position": [10, 10, 5],
        "rotation": [0, 0, 0],
        "scale": [1, 1, 1]
      },
      "properties": {
        "color": "#ffffff",
        "intensity": 1.0,
        "castShadow": true,
        "shadowMapSize": 2048,
        "shadowBias": -0.0001,
        "shadowNear": 0.1,
        "shadowFar": 100
      }
    }
  ],
  "cameras": [
    {
      "id": "camera_1640995320000_ghi789jkl",
      "name": "Main Camera",
      "type": "perspective",
      "transform": {
        "position": [5, 5, 5],
        "rotation": [0, 0, 0],
        "scale": [1, 1, 1]
      },
      "properties": {
        "fov": 75,
        "near": 0.1,
        "far": 1000,
        "aspect": 1.777
      }
    }
  ],
  "hierarchy": {
    "root": [],
    "relationships": {}
  }
}
```

## Version Management

### Semantic Versioning

Scene versions follow semantic versioning:

- **Major Version**: Breaking changes in format
- **Minor Version**: New features, backward compatible
- **Patch Version**: Bug fixes, no format changes

### Version Registry

```typescript
interface SceneVersion {
  version: string;
  migrateFrom: (data: any) => any;
  migrateTo: (data: any) => any;
  validate: (data: any) => boolean;
}

const sceneVersions: SceneVersion[] = [
  {
    version: "1.0.0",
    migrateFrom: (data) => migrateFromLegacy(data),
    migrateTo: (data) => migrateToCurrent(data),
    validate: (data) => validateV1(data)
  }
];
```

## Serialization Process

### Saving Scenes

1. **Gather Data**: Collect all scene entities and state
2. **Version Stamp**: Add current version information
3. **Serialize**: Convert to JSON format
4. **Compress**: Optional compression for large scenes
5. **Write File**: Save to disk with proper encoding

### Loading Scenes

1. **Read File**: Load JSON data from disk
2. **Version Check**: Determine scene version
3. **Migrate**: Apply migration if needed
4. **Validate**: Ensure data integrity
5. **Deserialize**: Create scene objects from data

## Migration System

### Automatic Migration

The system automatically migrates scenes between versions:

```typescript
async function loadScene(filePath: string): Promise<Scene> {
  const rawData = await readFile(filePath);
  const sceneData = JSON.parse(rawData);

  // Check version and migrate if necessary
  const migratedData = await migrateSceneData(sceneData);

  // Validate migrated data
  const validationResult = validateSceneData(migratedData);
  if (!validationResult.valid) {
    throw new Error(`Invalid scene data: ${validationResult.errors.join(', ')}`);
  }

  // Create scene from validated data
  return createSceneFromData(migratedData);
}
```

### Migration Strategies

Different migration approaches for different changes:

#### Data Structure Changes

```typescript
function migrateV0ToV1(data: any): any {
  // Rename properties
  if (data.objects) {
    data.entities = data.objects.map(obj => ({
      ...obj,
      type: obj.geometryType // Rename property
    }));
    delete data.objects;
  }

  // Add new required fields
  data.metadata = data.metadata || {
    created: new Date().toISOString(),
    modified: new Date().toISOString()
  };

  return { ...data, version: "1.0.0" };
}
```

#### Component System Changes

```typescript
function migrateComponentSystem(data: any): any {
  // Convert old single material to component system
  data.entities = data.entities.map(entity => ({
    ...entity,
    components: {
      renderer: {
        material: entity.material || { color: "#ffffff" },
        castShadow: entity.castShadow !== false,
        receiveShadow: entity.receiveShadow !== false
      }
    }
  }));

  return data;
}
```

## Data Integrity

### Validation System

Comprehensive validation ensures scene correctness:

```typescript
interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

function validateSceneData(data: any): ValidationResult {
  const errors: string[] = [];
  const warnings: string[] = [];

  // Required fields
  if (!data.version) errors.push("Missing version");
  if (!data.entities) errors.push("Missing entities array");

  // Entity validation
  data.entities?.forEach((entity, index) => {
    if (!entity.id) errors.push(`Entity ${index}: Missing id`);
    if (!entity.type) errors.push(`Entity ${index}: Missing type`);

    // Validate transform
    if (!entity.transform?.position) {
      errors.push(`Entity ${index}: Missing position`);
    }
  });

  return { valid: errors.length === 0, errors, warnings };
}
```

### Error Recovery

Graceful handling of corrupted data:

- **Partial Loading**: Load valid parts of corrupted scenes
- **Backup Creation**: Create backups before migration
- **Error Reporting**: Detailed error messages for debugging

## Performance Optimizations

### Efficient Serialization

- **Streaming**: Process large scenes without loading everything into memory
- **Compression**: Optional gzip compression for file size reduction
- **Binary Format**: Future support for binary scene files
- **Incremental Saves**: Save only changed parts of scenes

### Loading Optimizations

- **Lazy Loading**: Load scene parts on demand
- **Asset Preloading**: Preload referenced assets
- **Parallel Processing**: Load independent scene parts in parallel
- **Caching**: Cache parsed scene data

## Asset References

### External Asset Handling

Scenes can reference external assets:

```json
{
  "assets": {
    "models": {
      "character": {
        "path": "models/character.glb",
        "type": "gltf"
      }
    },
    "textures": {
      "brick": {
        "path": "textures/brick.jpg",
        "type": "image"
      }
    }
  },
  "entities": [
    {
      "id": "object_1640995200000_abc123def",
      "name": "Character",
      "type": "imported",
      "model": "character",
      "materials": {
        "body": "brick"
      }
    }
  ]
}
```

### Asset Resolution

- **Path Resolution**: Handle relative and absolute paths
- **Asset Registry**: Track asset dependencies
- **Missing Asset Handling**: Graceful degradation for missing assets

## Metadata System

### Scene Metadata

Rich metadata for scene management:

```typescript
interface SceneMetadata {
  name: string;
  description?: string;
  author?: string;
  tags?: string[];
  thumbnail?: string;
  created: Date;
  modified: Date;
  engineVersion: string;
  customProperties?: Record<string, any>;
}
```

### Version History

Track scene changes over time:

```json
{
  "versionHistory": [
    {
      "version": "1.0.0",
      "timestamp": "2024-01-01T00:00:00Z",
      "changes": "Initial creation"
    },
    {
      "version": "1.0.1",
      "timestamp": "2024-01-01T01:00:00Z",
      "changes": "Added lighting setup"
    }
  ]
}
```

## Cross-Platform Compatibility

### Endianness Handling

Ensure compatibility across platforms:

- **Little-Endian**: Consistent byte order for binary data
- **Float Precision**: Standardized floating-point representation
- **String Encoding**: UTF-8 encoding for all text data

### Path Normalization

Handle different filesystem conventions:

```typescript
function normalizePath(path: string): string {
  // Convert backslashes to forward slashes
  return path.replace(/\\/g, '/');
}

function resolvePath(basePath: string, relativePath: string): string {
  // Handle both absolute and relative paths
  if (relativePath.startsWith('/')) {
    return relativePath;
  }
  return join(basePath, relativePath);
}
```

## Debugging Features

### Serialization Logs

Detailed logging for troubleshooting:

```typescript
const logger = {
  debug: (message: string, data?: any) => {
    console.debug(`[SceneSerializer] ${message}`, data);
  },
  warn: (message: string, data?: any) => {
    console.warn(`[SceneSerializer] ${message}`, data);
  },
  error: (message: string, error?: any) => {
    console.error(`[SceneSerializer] ${message}`, error);
  }
};
```

### Scene Diffing

Compare scene versions for debugging:

```typescript
function diffScenes(sceneA: SceneData, sceneB: SceneData): SceneDiff {
  // Compare entities, transforms, components
  return {
    addedEntities: [],
    removedEntities: [],
    modifiedEntities: [],
    transformChanges: [],
    componentChanges: []
  };
}
```

## Future Enhancements

### Advanced Features

- **Binary Format**: More efficient binary scene files
- **Streaming Loading**: Load scenes progressively
- **Scene Fragments**: Modular scene composition
- **Version Branches**: Support for scene version branches
- **Collaboration**: Multi-user scene editing with merge support
