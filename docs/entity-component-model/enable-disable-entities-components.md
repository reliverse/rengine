# Enable/Disable Entities & Components

## Overview

Rengine provides fine-grained control over entity and component activation through enable/disable mechanisms. This allows dynamic runtime behavior modification without destroying or recreating objects, enabling efficient object pooling, conditional logic, and performance optimization.

## Core Concepts

### Entity Enable/Disable

Entity-level activation control:

- **Enabled Entity**: Fully active, all components processed by systems
- **Disabled Entity**: Inactive, components ignored by systems
- **Preserved State**: Disabled entities maintain their state
- **Quick Toggle**: Fast activation/deactivation

### Component Enable/Disable

Component-level activation control:

- **Enabled Component**: Processed by relevant systems
- **Disabled Component**: Ignored by systems but preserved
- **Independent Control**: Components can be enabled/disabled independently
- **Conditional Logic**: Enable components based on game state

## Implementation Details

### Entity State Management

```typescript
interface Entity {
  id: string;
  name?: string;
  enabled: boolean; // Entity-level enable/disable
  components: Map<ComponentType, Component>;
  tags: Set<string>;
}

class EntityRegistry {
  private entities = new Map<string, Entity>();

  enableEntity(entityId: string): void {
    const entity = this.entities.get(entityId);
    if (entity) {
      entity.enabled = true;
      this.notifySystems('entity_enabled', entity);
    }
  }

  disableEntity(entityId: string): void {
    const entity = this.entities.get(entityId);
    if (entity) {
      entity.enabled = false;
      this.notifySystems('entity_disabled', entity);
    }
  }

  isEntityEnabled(entityId: string): boolean {
    const entity = this.entities.get(entityId);
    return entity?.enabled ?? false;
  }

  getEnabledEntities(): Entity[] {
    return Array.from(this.entities.values()).filter(entity => entity.enabled);
  }
}
```

### Component State Management

```typescript
interface Component {
  type: ComponentType;
  enabled: boolean; // Component-level enable/disable
  // ... other component data
}

class ComponentRegistry {
  enableComponent(entityId: string, componentType: ComponentType): void {
    const entity = this.entities.get(entityId);
    if (!entity) return;

    const component = entity.components.get(componentType);
    if (component) {
      component.enabled = true;
      this.notifySystems('component_enabled', { entity, component });
    }
  }

  disableComponent(entityId: string, componentType: ComponentType): void {
    const entity = this.entities.get(entityId);
    if (!entity) return;

    const component = entity.components.get(componentType);
    if (component) {
      component.enabled = false;
      this.notifySystems('component_disabled', { entity, component });
    }
  }

  isComponentEnabled(entityId: string, componentType: ComponentType): boolean {
    const entity = this.entities.get(entityId);
    if (!entity || !entity.enabled) return false;

    const component = entity.components.get(componentType);
    return component?.enabled ?? false;
  }
}
```

## System Integration

### System Query Filtering

Systems automatically filter for enabled entities and components:

```typescript
abstract class BaseSystem {
  protected queryEntities(componentTypes: ComponentType[]): Entity[] {
    return entityRegistry
      .getEnabledEntities()
      .filter(entity => this.hasEnabledComponents(entity, componentTypes));
  }

  private hasEnabledComponents(entity: Entity, componentTypes: ComponentType[]): boolean {
    return componentTypes.every(type =>
      componentRegistry.isComponentEnabled(entity.id, type)
    );
  }
}

// Example: Physics system only processes enabled physics components
class PhysicsSystem extends BaseSystem {
  update(deltaTime: number): void {
    const physicsEntities = this.queryEntities(['transform', 'physics']);

    for (const entity of physicsEntities) {
      // Only processes entities with enabled transform and physics components
      this.simulatePhysics(entity, deltaTime);
    }
  }
}
```

### Selective Processing

Systems can implement different logic based on component states:

```typescript
class RenderSystem extends BaseSystem {
  update(deltaTime: number): void {
    const renderEntities = this.queryEntities(['transform', 'render']);

    for (const entity of renderEntities) {
      const renderComponent = this.getComponent<RenderComponent>(entity, 'render');

      if (renderComponent.visible) {
        // Only render if visible component is enabled
        this.renderEntity(entity);
      }
    }
  }
}
```

## Use Cases

### Object Pooling

Efficient reuse of entities:

```typescript
class EntityPool {
  private pool: Entity[] = [];
  private maxSize = 100;

  createEntity(template: EntityTemplate): Entity {
    // Try to reuse disabled entity
    const pooledEntity = this.pool.find(entity => !entity.enabled);
    if (pooledEntity) {
      this.resetEntity(pooledEntity, template);
      entityRegistry.enableEntity(pooledEntity.id);
      return pooledEntity;
    }

    // Create new entity if pool is not full
    if (this.pool.length < this.maxSize) {
      const newEntity = entityRegistry.createEntity(template);
      this.pool.push(newEntity);
      return newEntity;
    }

    // Pool is full, create temporary entity
    return entityRegistry.createEntity(template);
  }

  releaseEntity(entityId: string): void {
    const entity = entityRegistry.getEntity(entityId);
    if (entity && this.pool.includes(entity)) {
      entityRegistry.disableEntity(entityId);
      // Entity remains in pool for reuse
    } else {
      // Temporary entity, destroy it
      entityRegistry.destroyEntity(entityId);
    }
  }

  private resetEntity(entity: Entity, template: EntityTemplate): void {
    // Reset entity to template state
    entity.components.clear();
    for (const component of template.components) {
      entityRegistry.addComponent(entity.id, component);
    }
    entity.name = template.name;
  }
}

// Usage
const bulletPool = new EntityPool();
const bullet = bulletPool.createEntity(bulletTemplate);

// ... use bullet ...

bulletPool.releaseEntity(bullet.id); // Disable and return to pool
```

### Conditional Behavior

Dynamic component activation:

```typescript
class CharacterSystem extends BaseSystem {
  update(deltaTime: number): void {
    const characters = this.queryEntities(['character', 'health']);

    for (const entity of characters) {
      const character = this.getComponent<CharacterComponent>(entity, 'character');
      const health = this.getComponent<HealthComponent>(entity, 'health');

      // Enable/disable components based on character state
      if (health.currentHealth <= 0) {
        // Character is dead
        this.disableComponent(entity.id, 'input'); // Can't control dead character
        this.disableComponent(entity.id, 'ai');    // No AI behavior when dead
        this.enableComponent(entity.id, 'ragdoll'); // Enable ragdoll physics
      } else if (health.currentHealth < health.maxHealth * 0.25) {
        // Character is badly injured
        this.disableComponent(entity.id, 'sprint'); // Can't sprint when injured
        this.enableComponent(entity.id, 'limp');    // Enable limping animation
      } else {
        // Character is healthy
        this.enableComponent(entity.id, 'input');
        this.enableComponent(entity.id, 'ai');
        this.enableComponent(entity.id, 'sprint');
        this.disableComponent(entity.id, 'limp');
        this.disableComponent(entity.id, 'ragdoll');
      }
    }
  }
}
```

### Performance Optimization

Disable unnecessary processing:

```typescript
class LODSystem extends BaseSystem {
  update(deltaTime: number): void {
    const lodEntities = this.queryEntities(['transform', 'lod']);

    for (const entity of lodEntities) {
      const transform = this.getComponent<TransformComponent>(entity, 'transform');
      const lod = this.getComponent<LODComponent>(entity, 'lod');

      const distance = this.calculateDistanceToCamera(transform.position);
      const currentLOD = this.getLODLevel(distance, lod.levels);

      // Enable/disable components based on LOD level
      for (let i = 0; i < lod.levels.length; i++) {
        const levelEnabled = i === currentLOD;

        // Enable/disable render components for this LOD level
        this.setLODComponentEnabled(entity.id, `render_lod_${i}`, levelEnabled);

        // Enable/disable physics components for performance
        if (i > 0) { // Lower LOD levels can disable physics
          this.setComponentEnabled(entity.id, 'physics', levelEnabled);
        }
      }
    }
  }

  private setLODComponentEnabled(entityId: string, componentType: string, enabled: boolean): void {
    if (enabled) {
      componentRegistry.enableComponent(entityId, componentType);
    } else {
      componentRegistry.disableComponent(entityId, componentType);
    }
  }
}
```

### Game State Management

Enable/disable based on game phases:

```typescript
class GameStateSystem extends BaseSystem {
  update(deltaTime: number): void {
    const currentState = gameStateManager.getCurrentState();

    switch (currentState) {
      case 'menu':
        this.disableGameplayEntities();
        this.enableUIEntities();
        break;

      case 'playing':
        this.enableGameplayEntities();
        this.disableUIEntities();
        break;

      case 'paused':
        this.disableGameplayEntities();
        this.enablePauseMenuEntities();
        break;

      case 'game_over':
        this.disableGameplayEntities();
        this.enableGameOverEntities();
        break;
    }
  }

  private disableGameplayEntities(): void {
    const gameplayEntities = entityRegistry.getEntitiesByTag('gameplay');
    for (const entity of gameplayEntities) {
      entityRegistry.disableEntity(entity.id);
    }
  }

  private enableGameplayEntities(): void {
    const gameplayEntities = entityRegistry.getEntitiesByTag('gameplay');
    for (const entity of gameplayEntities) {
      entityRegistry.enableEntity(entity.id);
    }
  }
}
```

## Serialization

### State Preservation

Enabled/disabled state persists across saves:

```typescript
interface SerializedEntity {
  id: string;
  name?: string;
  enabled: boolean;
  components: SerializedComponent[];
  tags: string[];
}

interface SerializedComponent {
  type: ComponentType;
  enabled: boolean;
  data: any; // Component-specific data
}

function serializeEntity(entity: Entity): SerializedEntity {
  return {
    id: entity.id,
    name: entity.name,
    enabled: entity.enabled,
    tags: Array.from(entity.tags),
    components: Array.from(entity.components.entries()).map(([type, component]) => ({
      type,
      enabled: component.enabled,
      data: serializeComponentData(component)
    }))
  };
}

function deserializeEntity(data: SerializedEntity): Entity {
  const entity = entityRegistry.createEntity({
    name: data.name,
    enabled: data.enabled
  });

  // Add tags
  for (const tag of data.tags) {
    entity.tags.add(tag);
  }

  // Add components with correct enabled state
  for (const componentData of data.components) {
    const component = deserializeComponentData(componentData.type, componentData.data);
    component.enabled = componentData.enabled;
    entityRegistry.addComponent(entity.id, component);
  }

  return entity;
}
```

## Event System

### Enable/Disable Events

Systems can respond to enable/disable events:

```typescript
interface EntityEnabledEvent {
  type: 'entity_enabled';
  entityId: string;
}

interface EntityDisabledEvent {
  type: 'entity_disabled';
  entityId: string;
}

interface ComponentEnabledEvent {
  type: 'component_enabled';
  entityId: string;
  componentType: ComponentType;
}

interface ComponentDisabledEvent {
  type: 'component_disabled';
  entityId: string;
  componentType: ComponentType;
}

// Example: Sound system responds to entity state changes
class SoundSystem extends BaseSystem implements EventListener {
  onEntityEnabled(event: EntityEnabledEvent): void {
    // Play "appear" sound for enabled entities
    const entity = entityRegistry.getEntity(event.entityId);
    if (entity?.components.has('sound')) {
      this.playSound(entity, 'appear');
    }
  }

  onEntityDisabled(event: EntityDisabledEvent): void {
    // Play "disappear" sound and stop ongoing sounds
    const entity = entityRegistry.getEntity(event.entityId);
    if (entity?.components.has('sound')) {
      this.stopEntitySounds(entity);
      this.playSound(entity, 'disappear');
    }
  }
}
```

## Performance Considerations

### Query Optimization

Efficient queries for enabled entities:

```typescript
class OptimizedEntityRegistry {
  private enabledEntities = new Set<string>();
  private componentEnabledIndex = new Map<ComponentType, Set<string>>();

  enableEntity(entityId: string): void {
    this.enabledEntities.add(entityId);
    this.updateComponentIndices(entityId, true);
  }

  disableEntity(entityId: string): void {
    this.enabledEntities.delete(entityId);
    this.updateComponentIndices(entityId, false);
  }

  queryEnabledEntities(componentTypes: ComponentType[]): Entity[] {
    if (componentTypes.length === 0) {
      return Array.from(this.enabledEntities).map(id => this.entities.get(id)!);
    }

    // Find intersection of all required component types
    const entitySets = componentTypes.map(type => this.componentEnabledIndex.get(type) || new Set());

    if (entitySets.length === 0) return [];

    // Start with smallest set for efficiency
    const sortedSets = entitySets.sort((a, b) => a.size - b.size);
    let result = new Set(sortedSets[0]);

    // Intersect with remaining sets
    for (let i = 1; i < sortedSets.length; i++) {
      result = new Set([...result].filter(id => sortedSets[i].has(id)));
    }

    return Array.from(result).map(id => this.entities.get(id)!);
  }

  private updateComponentIndices(entityId: string, enabled: boolean): void {
    const entity = this.entities.get(entityId);
    if (!entity) return;

    for (const componentType of entity.components.keys()) {
      const index = this.componentEnabledIndex.get(componentType) || new Set();

      if (enabled && entity.enabled) {
        index.add(entityId);
      } else {
        index.delete(entityId);
      }

      if (index.size > 0) {
        this.componentEnabledIndex.set(componentType, index);
      } else {
        this.componentEnabledIndex.delete(componentType);
      }
    }
  }
}
```

### Memory Management

Disable entities to reduce processing load:

```typescript
class EntityCullingSystem extends BaseSystem {
  private cullDistance = 100;

  update(deltaTime: number): void {
    const cameraPosition = camera.getPosition();
    const allEntities = entityRegistry.getAllEntities();

    for (const entity of allEntities) {
      const transform = entity.components.get('transform') as TransformComponent;
      if (!transform) continue;

      const distance = vector3.distance(cameraPosition, transform.position);

      if (distance > this.cullDistance) {
        // Disable distant entities
        entityRegistry.disableEntity(entity.id);
      } else if (distance < this.cullDistance * 0.8) {
        // Re-enable nearby entities
        entityRegistry.enableEntity(entity.id);
      }
    }
  }
}
```

## Debugging Features

### State Inspection

Debug tools for enable/disable state:

```typescript
class EnableDisableDebugger {
  getEntityState(entityId: string): EntityStateInfo {
    const entity = entityRegistry.getEntity(entityId);
    if (!entity) return null;

    return {
      id: entity.id,
      name: entity.name,
      enabled: entity.enabled,
      componentStates: Array.from(entity.components.entries()).map(([type, component]) => ({
        type,
        enabled: component.enabled
      }))
    };
  }

  getSystemProcessingStats(): ProcessingStats {
    return {
      totalEntities: entityRegistry.getAllEntities().length,
      enabledEntities: entityRegistry.getEnabledEntities().length,
      disabledEntities: entityRegistry.getAllEntities().length - entityRegistry.getEnabledEntities().length,
      componentBreakdown: this.getComponentBreakdown()
    };
  }

  private getComponentBreakdown(): ComponentStats[] {
    const stats: ComponentStats[] = [];

    for (const [type, index] of componentRegistry.getComponentIndices()) {
      const total = index.size;
      const enabled = Array.from(index).filter(id =>
        componentRegistry.isComponentEnabled(id, type)
      ).length;

      stats.push({
        type,
        total,
        enabled,
        disabled: total - enabled
      });
    }

    return stats;
  }
}
```

## Best Practices

### Enable/Disable Patterns

- **Use Sparingly**: Only disable when necessary for performance
- **Clear Ownership**: Document which systems control enable/disable state
- **Consistent State**: Ensure related components are enabled/disabled together
- **Event Communication**: Use events to notify other systems of state changes

### Performance Guidelines

- **Batch Operations**: Enable/disable multiple entities/components together
- **Cache Queries**: Cache results of complex enable/disable queries
- **Profile Impact**: Monitor performance impact of enable/disable operations
- **Avoid Thrashing**: Minimize rapid enable/disable cycles

### Maintainability

- **State Documentation**: Document expected enable/disable states
- **Validation**: Validate enable/disable operations in development
- **Testing**: Test enable/disable behavior thoroughly
- **Monitoring**: Monitor enable/disable state in production

## Future Enhancements

### Advanced Features

- **Conditional Enable/Disable**: Rules-based automatic enable/disable
- **Component Groups**: Enable/disable related component sets
- **State Machines**: Entity state machines with automatic transitions
- **Performance Profiles**: Different enable/disable profiles for performance tiers
- **Network Synchronization**: Synchronize enable/disable state across network
