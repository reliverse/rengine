# Stable IDs for All Entities

## Overview

Rengine implements a robust entity identification system using stable, unique IDs that persist across scene saves, loads, and runtime sessions. This ensures reliable entity referencing for scripting, networking, and scene management.

## Core Concepts

### Entity Identity

Every entity in Rengine has a unique identifier that:

- **Persists**: Remains constant across application sessions
- **Unique**: Guaranteed to be unique within the scene
- **Stable**: Doesn't change during normal operations
- **Serializable**: Can be saved and loaded from disk

### ID Structure

Entity IDs follow a structured format:

```
[entity_type]_[timestamp]_[random_suffix]
```

Examples:
- `object_1640995200000_abc123def` - Scene object
- `light_1640995260000_def456ghi` - Light entity
- `camera_1640995320000_ghi789jkl` - Camera entity

## Implementation Details

### ID Generation

```typescript
function generateEntityId(type: string): string {
  const timestamp = Date.now();
  const randomSuffix = Math.random().toString(36).substr(2, 9);
  return `${type}_${timestamp}_${randomSuffix}`;
}
```

### ID Management

The entity ID system provides:

- **Generation**: Create new unique IDs
- **Validation**: Verify ID format and uniqueness
- **Lookup**: Fast entity retrieval by ID
- **Persistence**: Save/load IDs with scene data

## Features

### Scene Serialization

Entity IDs are preserved during scene save/load operations:

```json
{
  "entities": [
    {
      "id": "object_1640995200000_abc123def",
      "name": "Player Character",
      "type": "imported",
      "position": [0, 0, 0]
    }
  ]
}
```

### Runtime Stability

Entity IDs remain stable during:

- **Scene Editing**: Adding/removing other entities
- **Hierarchy Changes**: Reparenting entities
- **Property Updates**: Modifying entity properties
- **Component Changes**: Adding/removing components

## ID Types

### Object IDs

Scene objects (geometry, models) use object IDs:

```typescript
interface SceneObject {
  id: string; // "object_[timestamp]_[suffix]"
  name: string;
  type: "cube" | "sphere" | "plane" | "imported";
  // ... other properties
}
```

### Light IDs

Lighting entities use light IDs:

```typescript
interface SceneLight {
  id: string; // "light_[timestamp]_[suffix]"
  name: string;
  type: "directional" | "point" | "spot" | "ambient" | "hemisphere";
  // ... other properties
}
```

### Camera IDs

Camera entities use camera IDs:

```typescript
interface Camera {
  id: string; // "camera_[timestamp]_[suffix]"
  name: string;
  type: "perspective" | "orthographic";
  // ... other properties
}
```

## Benefits

### Scripting Support

Stable IDs enable reliable scripting:

```typescript
// Script can reference entities by stable ID
const player = scene.getEntityById("object_1640995200000_abc123def");
player.translate([0, 0, 1]); // Move player forward
```

### Networking

Entity IDs work for multiplayer synchronization:

```typescript
// Network messages can reference entities by ID
network.send({
  type: "entity_update",
  entityId: "object_1640995200000_abc123def",
  position: [10, 5, 0]
});
```

### Scene References

IDs enable cross-scene references:

```typescript
// Prefab can reference external entities
const prefab = {
  rootEntityId: "object_1640995200000_abc123def",
  references: ["light_1640995260000_def456ghi"]
};
```

## ID Registry

### Global Registry

Rengine maintains a global entity registry:

```typescript
class EntityRegistry {
  private entities = new Map<string, Entity>();
  private idCounter = new Map<string, number>();

  register(entity: Entity): void {
    this.entities.set(entity.id, entity);
  }

  getById(id: string): Entity | undefined {
    return this.entities.get(id);
  }

  generateId(type: string): string {
    const count = this.idCounter.get(type) || 0;
    const timestamp = Date.now();
    const suffix = count.toString(36).padStart(4, '0');
    this.idCounter.set(type, count + 1);
    return `${type}_${timestamp}_${suffix}`;
  }
}
```

### Collision Detection

The registry prevents ID collisions:

- **Uniqueness Check**: Verify new IDs don't conflict
- **Namespace Separation**: Different entity types have separate ID spaces
- **Timestamp Uniqueness**: Millisecond precision prevents collisions

## Persistence

### Scene Files

Entity IDs are stored in scene files:

```json
{
  "version": "1.0",
  "metadata": {
    "name": "My Scene",
    "created": "2024-01-01T00:00:00Z"
  },
  "entities": [
    {
      "id": "object_1640995200000_abc123def",
      "type": "cube",
      "position": [0, 0, 0],
      "components": ["transform", "renderer"]
    }
  ]
}
```

### Migration Support

ID system supports scene format migrations:

- **Backward Compatibility**: Load old scenes with legacy IDs
- **ID Translation**: Map old IDs to new format
- **Reference Updates**: Update all references during migration

## Performance Considerations

### Lookup Optimization

Entity registry uses efficient data structures:

- **Hash Map**: O(1) lookup by ID
- **Type Indexes**: Separate maps for different entity types
- **Spatial Indexes**: Optional spatial partitioning for proximity queries

### Memory Management

ID system minimizes memory overhead:

- **String Interning**: Reuse common ID prefixes
- **Compact Storage**: Efficient string storage
- **Reference Counting**: Automatic cleanup of unused entities

## Debugging Features

### ID Validation

Debug tools validate entity IDs:

```typescript
function validateEntityId(id: string): boolean {
  const parts = id.split('_');
  if (parts.length !== 3) return false;

  const [type, timestamp, suffix] = parts;

  // Validate type
  if (!['object', 'light', 'camera'].includes(type)) return false;

  // Validate timestamp
  const ts = parseInt(timestamp);
  if (isNaN(ts) || ts < 1640995200000) return false; // After 2022

  // Validate suffix format
  if (!/^[a-z0-9]{9}$/.test(suffix)) return false;

  return true;
}
```

### Entity Inspector

Debug panel shows entity information:

- **ID Details**: Breakdown of ID components
- **References**: What references this entity
- **Hierarchy**: Parent/child relationships
- **Components**: Attached components and their data

## Best Practices

### ID Generation

- **Use Timestamps**: Include creation time for debugging
- **Random Suffixes**: Ensure uniqueness across sessions
- **Type Prefixes**: Clearly identify entity types

### Reference Management

- **Weak References**: Use weak references for optional links
- **Cascade Deletes**: Clean up references when entities are removed
- **Validation**: Check references during scene loading

### Naming Conventions

- **Descriptive Names**: Use meaningful entity names alongside IDs
- **Consistent Types**: Follow established type naming conventions
- **Versioning**: Include version info in ID format if needed

## Future Enhancements

### Advanced ID Features

- **UUID Support**: Option to use RFC 4122 UUIDs
- **Custom ID Schemes**: User-defined ID generation strategies
- **ID Namespaces**: Hierarchical ID spaces for large projects
- **ID Aliases**: Human-readable aliases for complex IDs

### Distributed Systems

- **Cluster IDs**: IDs that work across multiple engine instances
- **Replication**: ID synchronization for multiplayer scenarios
- **Conflict Resolution**: Handle ID collisions in distributed environments
