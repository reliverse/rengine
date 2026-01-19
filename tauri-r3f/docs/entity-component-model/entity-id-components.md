# Entity = ID + Components

## Overview

Rengine implements a pure Entity-Component-System (ECS) architecture where entities are lightweight identifiers composed of reusable components. This design enables flexible, performant, and maintainable game object management.

## Core Concepts

### Entity Definition

An entity in Rengine is fundamentally:

**Entity = Unique ID + Collection of Components**

- **ID**: Stable, unique identifier (string-based)
- **Components**: Pure data structures attached to the entity
- **No Logic**: Entities contain no behavior or methods
- **Dynamic Composition**: Components can be added/removed at runtime

### Component Definition

Components are pure data structures that:

- **Contain Data Only**: No methods or logic
- **Are Serializable**: Can be saved/loaded
- **Are Reusable**: Same component type used across entities
- **Are Optional**: Entities can exist without any components

## Implementation Details

### Entity Structure

```typescript
interface Entity {
  id: string; // Unique identifier
  name?: string; // Optional human-readable name
  components: Map<ComponentType, Component>; // Component collection
  tags: Set<string>; // Optional categorization tags
}
```

### Component Types

Rengine defines several core component categories:

#### Transform Components
```typescript
interface TransformComponent {
  type: 'transform';
  position: [number, number, number];
  rotation: [number, number, number]; // Euler angles in radians
  scale: [number, number, number];
  parent?: string; // Parent entity ID for hierarchy
}
```

#### Rendering Components
```typescript
interface RenderComponent {
  type: 'render';
  geometry: GeometryType;
  material: MaterialData;
  visible: boolean;
  castShadow: boolean;
  receiveShadow: boolean;
  renderOrder: number;
}

interface GeometryType {
  type: 'box' | 'sphere' | 'plane' | 'imported';
  parameters?: Record<string, any>; // Geometry-specific params
}
```

#### Physics Components
```typescript
interface PhysicsComponent {
  type: 'physics';
  bodyType: 'static' | 'dynamic' | 'kinematic';
  shape: CollisionShape;
  mass: number;
  friction: number;
  restitution: number;
  constraints: PhysicsConstraint[];
}
```

#### Script Components
```typescript
interface ScriptComponent {
  type: 'script';
  scriptPath: string;
  properties: Record<string, any>; // Script-specific properties
  enabled: boolean;
}
```

### Entity Registry

Central system for entity management:

```typescript
class EntityRegistry {
  private entities = new Map<string, Entity>();
  private componentIndex = new Map<ComponentType, Set<string>>();

  createEntity(name?: string): Entity {
    const id = generateEntityId('entity');
    const entity: Entity = {
      id,
      name: name || `Entity_${id.split('_')[2]}`,
      components: new Map(),
      tags: new Set()
    };

    this.entities.set(id, entity);
    return entity;
  }

  destroyEntity(entityId: string): void {
    const entity = this.entities.get(entityId);
    if (!entity) return;

    // Remove from component indices
    for (const componentType of entity.components.keys()) {
      this.componentIndex.get(componentType)?.delete(entityId);
    }

    // Remove entity
    this.entities.delete(entityId);
  }

  addComponent(entityId: string, component: Component): void {
    const entity = this.entities.get(entityId);
    if (!entity) return;

    entity.components.set(component.type, component);

    // Update component index
    if (!this.componentIndex.has(component.type)) {
      this.componentIndex.set(component.type, new Set());
    }
    this.componentIndex.get(component.type)!.add(entityId);
  }

  removeComponent(entityId: string, componentType: ComponentType): void {
    const entity = this.entities.get(entityId);
    if (!entity) return;

    entity.components.delete(componentType);
    this.componentIndex.get(componentType)?.delete(entityId);
  }

  getEntitiesWithComponent(componentType: ComponentType): Entity[] {
    const entityIds = this.componentIndex.get(componentType) || new Set();
    return Array.from(entityIds)
      .map(id => this.entities.get(id))
      .filter(Boolean) as Entity[];
  }
}
```

## Component Archetypes

### Common Entity Patterns

#### Static Geometry Entity
```typescript
const wallEntity = entityRegistry.createEntity('Wall');
entityRegistry.addComponent(wallEntity.id, {
  type: 'transform',
  position: [0, 0, 0],
  rotation: [0, 0, 0],
  scale: [10, 5, 1]
});
entityRegistry.addComponent(wallEntity.id, {
  type: 'render',
  geometry: { type: 'box' },
  material: { color: '#808080' },
  visible: true,
  castShadow: true,
  receiveShadow: true
});
entityRegistry.addComponent(wallEntity.id, {
  type: 'physics',
  bodyType: 'static',
  shape: { type: 'box', size: [10, 5, 1] },
  mass: 0
});
```

#### Dynamic Character Entity
```typescript
const characterEntity = entityRegistry.createEntity('Player');
entityRegistry.addComponent(characterEntity.id, {
  type: 'transform',
  position: [0, 0, 0],
  rotation: [0, 0, 0],
  scale: [1, 1, 1]
});
entityRegistry.addComponent(characterEntity.id, {
  type: 'render',
  geometry: { type: 'imported', modelPath: 'models/character.glb' },
  material: {},
  visible: true,
  castShadow: true,
  receiveShadow: true
});
entityRegistry.addComponent(characterEntity.id, {
  type: 'physics',
  bodyType: 'dynamic',
  shape: { type: 'capsule', radius: 0.5, height: 1.8 },
  mass: 70
});
entityRegistry.addComponent(characterEntity.id, {
  type: 'script',
  scriptPath: 'scripts/player-controller.ts',
  properties: { speed: 5, jumpHeight: 2 },
  enabled: true
});
```

#### Light Entity
```typescript
const lightEntity = entityRegistry.createEntity('Main Light');
entityRegistry.addComponent(lightEntity.id, {
  type: 'transform',
  position: [10, 10, 5],
  rotation: [0, 0, 0],
  scale: [1, 1, 1]
});
entityRegistry.addComponent(lightEntity.id, {
  type: 'light',
  lightType: 'directional',
  color: '#ffffff',
  intensity: 1.0,
  castShadow: true,
  shadowMapSize: 2048
});
```

## Runtime Component Management

### Adding Components Dynamically

```typescript
function addHealthComponent(entityId: string, maxHealth: number): void {
  const healthComponent = {
    type: 'health',
    currentHealth: maxHealth,
    maxHealth,
    regenerationRate: 1,
    lastDamageTime: 0
  };

  entityRegistry.addComponent(entityId, healthComponent);
}

// Usage
const enemyId = createEnemyEntity();
addHealthComponent(enemyId, 100);
```

### Component Queries

Efficient entity querying by component composition:

```typescript
// Find all renderable entities
const renderableEntities = entityRegistry.getEntitiesWithComponent('render');

// Find entities with both physics and health
const damageableEntities = entityRegistry
  .getEntitiesWithComponent('physics')
  .filter(entity => entity.components.has('health'));

// Find entities with specific component values
const lowHealthEntities = entityRegistry
  .getEntitiesWithComponent('health')
  .filter(entity => {
    const health = entity.components.get('health') as HealthComponent;
    return health.currentHealth < health.maxHealth * 0.25;
  });
```

## Serialization

### Component Serialization

Components are designed for easy serialization:

```typescript
function serializeEntity(entity: Entity): SerializedEntity {
  return {
    id: entity.id,
    name: entity.name,
    tags: Array.from(entity.tags),
    components: Object.fromEntries(
      Array.from(entity.components.entries()).map(([type, component]) => [
        type,
        serializeComponent(component)
      ])
    )
  };
}

function deserializeEntity(data: SerializedEntity): Entity {
  const entity: Entity = {
    id: data.id,
    name: data.name,
    components: new Map(),
    tags: new Set(data.tags)
  };

  for (const [type, componentData] of Object.entries(data.components)) {
    const component = deserializeComponent(type as ComponentType, componentData);
    entity.components.set(component.type, component);
  }

  return entity;
}
```

## Performance Characteristics

### Memory Efficiency

- **Minimal Entity Overhead**: Only ID and component references
- **Shared Component Types**: Component definitions shared across entities
- **Sparse Storage**: Only store components that exist

### Query Performance

- **Component Indexing**: O(1) lookup for component types
- **Archetype Caching**: Cache common component combinations
- **Parallel Processing**: Components enable SIMD-friendly processing

### Cache Efficiency

```typescript
// Component data is contiguous in memory
class ComponentStorage<T extends Component> {
  private components = new Map<string, T>();
  private denseArray: T[] = []; // For cache-friendly iteration
  private entityToIndex = new Map<string, number>();

  addComponent(entityId: string, component: T): void {
    this.components.set(entityId, component);
    this.entityToIndex.set(entityId, this.denseArray.length);
    this.denseArray.push(component);
  }

  getComponent(entityId: string): T | undefined {
    return this.components.get(entityId);
  }

  // Cache-friendly iteration
  forEach(callback: (component: T, entityId: string) => void): void {
    for (let i = 0; i < this.denseArray.length; i++) {
      const component = this.denseArray[i];
      const entityId = Array.from(this.components.entries())
        .find(([_, comp]) => comp === component)?.[0];

      if (entityId) {
        callback(component, entityId);
      }
    }
  }
}
```

## Component Lifecycle

### Component Events

Components can emit lifecycle events:

```typescript
interface ComponentLifecycle {
  onAttach?(entity: Entity): void;
  onDetach?(entity: Entity): void;
  onUpdate?(entity: Entity, deltaTime: number): void;
  onDestroy?(entity: Entity): void;
}

class HealthComponent implements ComponentLifecycle {
  type = 'health' as const;

  onAttach(entity: Entity): void {
    console.log(`Health component attached to ${entity.name}`);
  }

  onUpdate(entity: Entity, deltaTime: number): void {
    // Regenerate health over time
    if (this.currentHealth < this.maxHealth) {
      this.currentHealth = Math.min(
        this.maxHealth,
        this.currentHealth + this.regenerationRate * deltaTime
      );
    }
  }
}
```

## Type Safety

### Component Type System

TypeScript ensures component correctness:

```typescript
type ComponentType = 'transform' | 'render' | 'physics' | 'script' | 'health' | 'ai';

interface ComponentBase {
  type: ComponentType;
}

// Union type for all components
type Component =
  | TransformComponent
  | RenderComponent
  | PhysicsComponent
  | ScriptComponent
  | HealthComponent
  | AIComponent;

// Type-safe component access
function getHealth(entity: Entity): HealthComponent | undefined {
  return entity.components.get('health') as HealthComponent | undefined;
}
```

## Best Practices

### Component Design

- **Single Responsibility**: Each component handles one aspect
- **Minimal Data**: Only store necessary data
- **Serializable**: Ensure all data can be serialized
- **Versioned**: Include version info for migrations

### Entity Composition

- **Logical Grouping**: Group related components together
- **Sparse Entities**: Don't add unnecessary components
- **Dynamic Composition**: Add/remove components as needed
- **Naming**: Use descriptive names for complex entities

### Performance Guidelines

- **Batch Operations**: Process similar components together
- **Cache Results**: Cache expensive component calculations
- **Avoid Deep Hierarchies**: Limit component nesting depth
- **Profile Regularly**: Monitor component system performance

## Integration with Systems

### System Pattern

Components work with systems for behavior:

```typescript
class PhysicsSystem {
  update(deltaTime: number): void {
    const physicsEntities = entityRegistry.getEntitiesWithComponent('physics');

    for (const entity of physicsEntities) {
      const physics = entity.components.get('physics') as PhysicsComponent;
      const transform = entity.components.get('transform') as TransformComponent;

      // Update physics simulation
      this.simulatePhysics(physics, transform, deltaTime);
    }
  }
}

class RenderSystem {
  render(): void {
    const renderEntities = entityRegistry.getEntitiesWithComponent('render');

    for (const entity of renderEntities) {
      const render = entity.components.get('render') as RenderComponent;
      const transform = entity.components.get('transform') as TransformComponent;

      // Render entity
      this.renderEntity(render, transform);
    }
  }
}
```

## Future Enhancements

### Advanced Features

- **Component Prefabs**: Reusable component combinations
- **Component Inheritance**: Component type hierarchies
- **Runtime Component Compilation**: Dynamic component creation
- **Component Networking**: Multiplayer component synchronization
- **Component Versioning**: Automatic component data migration
