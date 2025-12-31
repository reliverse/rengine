# Systems = Logic

## Overview

In Rengine's Entity-Component-System architecture, systems contain all the logic and behavior that operate on pure data components. Systems process entities with specific component combinations, implementing the game's rules and mechanics.

## Core Concepts

### System Definition

A system is a pure function that:

- **Processes Components**: Operates on entity component data
- **Contains Logic**: Implements game mechanics and behavior
- **Has No State**: Stateless processing (or managed state)
- **Is Composable**: Can be combined with other systems
- **Runs in Parallel**: Independent systems can run simultaneously

### System Categories

#### Update Systems
Process continuous game logic each frame:

```typescript
interface UpdateSystem {
  name: string;
  priority: number; // Execution order
  update(deltaTime: number): void;
}
```

#### Event Systems
Respond to discrete events:

```typescript
interface EventSystem<T extends Event> {
  name: string;
  eventType: string;
  handle(event: T): void;
}
```

#### Render Systems
Handle visual presentation:

```typescript
interface RenderSystem {
  name: string;
  render(camera: Camera, scene: Scene): void;
}
```

## System Architecture

### System Registry

Central system management:

```typescript
class SystemRegistry {
  private systems = new Map<string, System>();
  private updateSystems: UpdateSystem[] = [];
  private eventSystems = new Map<string, EventSystem<any>[]>();

  register(system: System): void {
    this.systems.set(system.name, system);

    if (this.isUpdateSystem(system)) {
      this.updateSystems.push(system);
      this.updateSystems.sort((a, b) => a.priority - b.priority);
    }

    if (this.isEventSystem(system)) {
      const eventType = system.eventType;
      if (!this.eventSystems.has(eventType)) {
        this.eventSystems.set(eventType, []);
      }
      this.eventSystems.get(eventType)!.push(system);
    }
  }

  update(deltaTime: number): void {
    for (const system of this.updateSystems) {
      system.update(deltaTime);
    }
  }

  dispatchEvent(event: Event): void {
    const systems = this.eventSystems.get(event.type) || [];
    for (const system of systems) {
      system.handle(event);
    }
  }
}
```

### System Base Classes

```typescript
abstract class BaseSystem implements UpdateSystem {
  abstract readonly name: string;
  abstract readonly priority: number;

  protected queryEntities(componentTypes: ComponentType[]): Entity[] {
    // Query entities with specific component combinations
    return entityRegistry.query(componentTypes);
  }

  protected getComponent<T extends Component>(
    entity: Entity,
    type: ComponentType
  ): T | undefined {
    return entity.components.get(type) as T | undefined;
  }

  abstract update(deltaTime: number): void;
}
```

## Core Systems

### Transform System

Handles entity positioning and hierarchy:

```typescript
class TransformSystem extends BaseSystem {
  readonly name = 'TransformSystem';
  readonly priority = 10; // Run early

  update(deltaTime: number): void {
    const entities = this.queryEntities(['transform']);

    for (const entity of entities) {
      const transform = this.getComponent<TransformComponent>(entity, 'transform');
      if (!transform) continue;

      // Update world matrix from local transform
      this.updateWorldMatrix(transform);

      // Handle parent-child relationships
      if (transform.parent) {
        this.applyParentTransform(transform, transform.parent);
      }
    }
  }

  private updateWorldMatrix(transform: TransformComponent): void {
    // Calculate transformation matrix from position, rotation, scale
    const matrix = new Matrix4();
    matrix.compose(
      new Vector3(...transform.position),
      new Euler(...transform.rotation),
      new Vector3(...transform.scale)
    );

    transform.worldMatrix = matrix;
  }

  private applyParentTransform(child: TransformComponent, parentId: string): void {
    const parentEntity = entityRegistry.getEntity(parentId);
    const parentTransform = parentEntity?.components.get('transform') as TransformComponent;

    if (parentTransform?.worldMatrix) {
      child.worldMatrix.multiplyMatrices(parentTransform.worldMatrix, child.localMatrix);
    }
  }
}
```

### Physics System

Manages physical simulation:

```typescript
class PhysicsSystem extends BaseSystem {
  readonly name = 'PhysicsSystem';
  readonly priority = 20;

  private world: PhysicsWorld;

  constructor() {
    super();
    this.world = new PhysicsWorld();
  }

  update(deltaTime: number): void {
    // Sync transforms to physics bodies
    this.syncTransformsToPhysics();

    // Step physics simulation
    this.world.step(deltaTime);

    // Sync physics results back to transforms
    this.syncPhysicsToTransforms();

    // Handle collisions
    this.processCollisions();
  }

  private syncTransformsToPhysics(): void {
    const entities = this.queryEntities(['transform', 'physics']);

    for (const entity of entities) {
      const transform = this.getComponent<TransformComponent>(entity, 'transform');
      const physics = this.getComponent<PhysicsComponent>(entity, 'physics');

      if (!transform || !physics) continue;

      const body = this.world.getBody(entity.id);
      if (body) {
        body.position.copy(transform.position);
        body.quaternion.setFromEuler(new Euler(...transform.rotation));
      }
    }
  }

  private syncPhysicsToTransforms(): void {
    const entities = this.queryEntities(['transform', 'physics']);

    for (const entity of entities) {
      const transform = this.getComponent<TransformComponent>(entity, 'transform');
      const physics = this.getComponent<PhysicsComponent>(entity, 'physics');

      if (!transform || !physics || physics.bodyType === 'static') continue;

      const body = this.world.getBody(entity.id);
      if (body) {
        // Update transform with physics results
        entityRegistry.updateComponent(entity.id, {
          ...transform,
          position: body.position.toArray(),
          rotation: body.quaternion.toEuler().toArray()
        });
      }
    }
  }

  private processCollisions(): void {
    const collisions = this.world.getCollisions();

    for (const collision of collisions) {
      const event: CollisionEvent = {
        type: 'collision',
        entityA: collision.bodyA.entityId,
        entityB: collision.bodyB.entityId,
        contactPoint: collision.contactPoint,
        normal: collision.normal,
        impulse: collision.impulse
      };

      eventSystem.dispatch(event);
    }
  }
}
```

### Render System

Handles visual presentation:

```typescript
class RenderSystem extends BaseSystem {
  readonly name = 'RenderSystem';
  readonly priority = 100; // Run last

  private renderer: WebGLRenderer;
  private camera: Camera;

  update(deltaTime: number): void {
    // Frustum culling
    const visibleEntities = this.cullEntities();

    // Sort by render order
    visibleEntities.sort((a, b) => {
      const renderA = this.getComponent<RenderComponent>(a, 'render');
      const renderB = this.getComponent<RenderComponent>(b, 'render');
      return (renderA?.renderOrder || 0) - (renderB?.renderOrder || 0);
    });

    // Render entities
    for (const entity of visibleEntities) {
      this.renderEntity(entity);
    }
  }

  private cullEntities(): Entity[] {
    const renderEntities = this.queryEntities(['transform', 'render']);
    const frustum = new Frustum().setFromProjectionMatrix(
      new Matrix4().multiplyMatrices(
        this.camera.projectionMatrix,
        this.camera.matrixWorldInverse
      )
    );

    return renderEntities.filter(entity => {
      const transform = this.getComponent<TransformComponent>(entity, 'transform');
      const render = this.getComponent<RenderComponent>(entity, 'render');

      if (!transform || !render || !render.visible) return false;

      // Simple bounding sphere test
      const sphere = new Sphere(transform.position, 1.0);
      return frustum.intersectsSphere(sphere);
    });
  }

  private renderEntity(entity: Entity): void {
    const transform = this.getComponent<TransformComponent>(entity, 'transform');
    const render = this.getComponent<RenderComponent>(entity, 'render');

    if (!transform || !render) return;

    // Set up material
    const material = this.createMaterial(render.material);

    // Set up geometry
    const geometry = this.createGeometry(render.geometry);

    // Apply transform
    const matrix = transform.worldMatrix;
    material.modelViewMatrix.multiplyMatrices(this.camera.matrixWorldInverse, matrix);
    material.normalMatrix.getNormalMatrix(material.modelViewMatrix);

    // Render
    this.renderer.renderBuffer(geometry, material);
  }
}
```

### Input System

Processes user input:

```typescript
class InputSystem extends BaseSystem {
  readonly name = 'InputSystem';
  readonly priority = 5; // Run very early

  private keyboard = new KeyboardState();
  private mouse = new MouseState();

  update(deltaTime: number): void {
    // Update input state
    this.keyboard.update();
    this.mouse.update();

    // Process input for controllable entities
    const controllableEntities = this.queryEntities(['input', 'transform']);

    for (const entity of controllableEntities) {
      this.processEntityInput(entity);
    }

    // Dispatch input events
    this.dispatchInputEvents();
  }

  private processEntityInput(entity: Entity): void {
    const input = this.getComponent<InputComponent>(entity, 'input');
    const transform = this.getComponent<TransformComponent>(entity, 'transform');

    if (!input || !transform) return;

    // Movement input
    const moveVector = new Vector3(0, 0, 0);

    if (this.keyboard.pressed('KeyW')) moveVector.z -= 1;
    if (this.keyboard.pressed('KeyS')) moveVector.z += 1;
    if (this.keyboard.pressed('KeyA')) moveVector.x -= 1;
    if (this.keyboard.pressed('KeyD')) moveVector.x += 1;

    if (moveVector.length() > 0) {
      moveVector.normalize().multiplyScalar(input.moveSpeed * deltaTime);

      // Apply movement
      entityRegistry.updateComponent(entity.id, {
        ...transform,
        position: [
          transform.position[0] + moveVector.x,
          transform.position[1] + moveVector.y,
          transform.position[2] + moveVector.z
        ]
      });
    }

    // Rotation input
    if (this.mouse.deltaX !== 0 || this.mouse.deltaY !== 0) {
      const rotationDelta = new Euler(
        -this.mouse.deltaY * input.lookSensitivity,
        -this.mouse.deltaX * input.lookSensitivity,
        0
      );

      entityRegistry.updateComponent(entity.id, {
        ...transform,
        rotation: [
          transform.rotation[0] + rotationDelta.x,
          transform.rotation[1] + rotationDelta.y,
          transform.rotation[2] + rotationDelta.z
        ]
      });
    }
  }

  private dispatchInputEvents(): void {
    // Keyboard events
    for (const key of this.keyboard.justPressed) {
      eventSystem.dispatch({
        type: 'key_pressed',
        key,
        modifiers: this.keyboard.modifiers
      });
    }

    // Mouse events
    if (this.mouse.justPressed) {
      eventSystem.dispatch({
        type: 'mouse_pressed',
        button: this.mouse.button,
        position: this.mouse.position,
        modifiers: this.keyboard.modifiers
      });
    }
  }
}
```

### AI System

Manages artificial intelligence:

```typescript
class AISystem extends BaseSystem {
  readonly name = 'AISystem';
  readonly priority = 30;

  update(deltaTime: number): void {
    const aiEntities = this.queryEntities(['ai', 'transform']);

    for (const entity of aiEntities) {
      this.updateAI(entity, deltaTime);
    }
  }

  private updateAI(entity: Entity, deltaTime: number): void {
    const ai = this.getComponent<AIComponent>(entity, 'ai');
    const transform = this.getComponent<TransformComponent>(entity, 'transform');

    if (!ai || !transform) return;

    // Update AI state machine
    const currentState = ai.states[ai.currentState];
    if (currentState) {
      const action = currentState.update(entity, deltaTime);

      // Execute AI action
      this.executeAction(entity, action);
    }

    // Update behavior tree
    if (ai.behaviorTree) {
      const result = ai.behaviorTree.tick(entity, deltaTime);
      this.handleBehaviorResult(entity, result);
    }
  }

  private executeAction(entity: Entity, action: AIAction): void {
    switch (action.type) {
      case 'move_to':
        this.moveTo(entity, action.target);
        break;
      case 'attack':
        this.attack(entity, action.target);
        break;
      case 'flee':
        this.flee(entity, action.from);
        break;
      case 'patrol':
        this.patrol(entity, action.waypoints);
        break;
    }
  }
}
```

## System Communication

### Event-Driven Communication

Systems communicate through events:

```typescript
interface Event {
  type: string;
  source?: string; // Source system/entity
  target?: string; // Target system/entity
  data?: any;
}

// Example events
interface DamageEvent extends Event {
  type: 'damage';
  source: string; // Attacking entity
  target: string; // Damaged entity
  amount: number;
  type: 'physical' | 'magical' | 'environmental';
}

interface SoundEvent extends Event {
  type: 'play_sound';
  soundId: string;
  position: [number, number, number];
  volume: number;
  loop: boolean;
}

// Event system
class EventSystem {
  private listeners = new Map<string, EventHandler[]>();

  on(eventType: string, handler: EventHandler): void {
    if (!this.listeners.has(eventType)) {
      this.listeners.set(eventType, []);
    }
    this.listeners.get(eventType)!.push(handler);
  }

  emit(event: Event): void {
    const handlers = this.listeners.get(event.type) || [];
    for (const handler of handlers) {
      handler(event);
    }
  }
}
```

### System Dependencies

Systems can declare dependencies:

```typescript
interface SystemDependencies {
  required: string[];  // Must run before this system
  optional: string[];  // Should run before this system if available
  conflicts: string[]; // Cannot run with these systems
}

class PhysicsSystem extends BaseSystem {
  readonly dependencies: SystemDependencies = {
    required: ['TransformSystem'], // Physics needs transforms
    optional: ['CollisionSystem'], // Better with collision detection
    conflicts: [] // No conflicts
  };
}
```

## Performance Optimization

### System Profiling

Performance monitoring for systems:

```typescript
class SystemProfiler {
  private metrics = new Map<string, SystemMetrics>();

  profileSystem(systemName: string, executionTime: number): void {
    const metrics = this.metrics.get(systemName) || {
      name: systemName,
      totalTime: 0,
      callCount: 0,
      averageTime: 0,
      maxTime: 0,
      minTime: Number.MAX_VALUE
    };

    metrics.totalTime += executionTime;
    metrics.callCount++;
    metrics.averageTime = metrics.totalTime / metrics.callCount;
    metrics.maxTime = Math.max(metrics.maxTime, executionTime);
    metrics.minTime = Math.min(metrics.minTime, executionTime);

    this.metrics.set(systemName, metrics);
  }

  getReport(): SystemReport {
    return {
      systems: Array.from(this.metrics.values()),
      totalTime: Array.from(this.metrics.values())
        .reduce((sum, m) => sum + m.totalTime, 0),
      bottleneck: this.findBottleneck()
    };
  }
}
```

### Parallel Execution

Systems can run in parallel when independent:

```typescript
class ParallelSystemExecutor {
  async executeSystems(systems: System[], deltaTime: number): Promise<void> {
    // Group systems by dependencies
    const { independent, dependent } = this.groupSystems(systems);

    // Execute independent systems in parallel
    const parallelPromises = independent.map(system =>
      this.executeSystem(system, deltaTime)
    );

    // Execute dependent systems serially
    for (const system of dependent) {
      await this.executeSystem(system, deltaTime);
    }

    // Wait for parallel systems
    await Promise.all(parallelPromises);
  }

  private groupSystems(systems: System[]): { independent: System[], dependent: System[] } {
    const independent: System[] = [];
    const dependent: System[] = [];

    for (const system of systems) {
      const deps = (system as any).dependencies as SystemDependencies;
      if (deps && (deps.required.length > 0 || deps.optional.length > 0)) {
        dependent.push(system);
      } else {
        independent.push(system);
      }
    }

    return { independent, dependent };
  }
}
```

## System Lifecycle

### Initialization

Systems need proper setup:

```typescript
interface SystemLifecycle {
  initialize?(): Promise<void>;
  shutdown?(): Promise<void>;
  pause?(): void;
  resume?(): void;
}

class AssetLoadingSystem extends BaseSystem implements SystemLifecycle {
  async initialize(): Promise<void> {
    // Load required assets
    await this.loadTextures();
    await this.loadModels();
    await this.loadSounds();
  }

  async shutdown(): Promise<void> {
    // Clean up resources
    this.unloadTextures();
    this.unloadModels();
    this.unloadSounds();
  }
}
```

### Hot Reloading

Systems support hot reloading for development:

```typescript
class HotReloadableSystem extends BaseSystem {
  private scriptPath: string;
  private lastModified: number;

  update(deltaTime: number): void {
    // Check for script changes
    if (this.hasScriptChanged()) {
      this.reloadScript();
    }

    // Execute system logic
    this.executeSystemLogic(deltaTime);
  }

  private hasScriptChanged(): boolean {
    const stats = fs.statSync(this.scriptPath);
    return stats.mtime.getTime() > this.lastModified;
  }

  private reloadScript(): void {
    // Reload system implementation
    delete require.cache[require.resolve(this.scriptPath)];
    const newSystemClass = require(this.scriptPath);

    // Replace system methods
    Object.assign(this, newSystemClass.prototype);

    this.lastModified = Date.now();
  }
}
```

## Testing

### System Testing

Unit testing for systems:

```typescript
describe('PhysicsSystem', () => {
  let physicsSystem: PhysicsSystem;
  let testEntity: Entity;

  beforeEach(() => {
    physicsSystem = new PhysicsSystem();
    testEntity = createTestEntity();
  });

  it('should apply gravity to dynamic bodies', () => {
    // Set up entity with physics component
    const physicsComponent = createPhysicsComponent({
      bodyType: 'dynamic',
      mass: 1.0
    });

    entityRegistry.addComponent(testEntity.id, physicsComponent);

    // Run physics for 1 second
    physicsSystem.update(1.0);

    // Check that entity fell due to gravity
    const transform = entityRegistry.getComponent(testEntity.id, 'transform');
    expect(transform.position[1]).toBeLessThan(0);
  });

  it('should not move static bodies', () => {
    // Set up static entity
    const physicsComponent = createPhysicsComponent({
      bodyType: 'static'
    });

    entityRegistry.addComponent(testEntity.id, physicsComponent);
    const initialPosition = [...testEntity.transform.position];

    // Run physics
    physicsSystem.update(1.0);

    // Check that position didn't change
    const finalPosition = testEntity.transform.position;
    expect(finalPosition).toEqual(initialPosition);
  });
});
```

## Best Practices

### System Design

- **Single Responsibility**: Each system handles one concern
- **Clear Dependencies**: Explicit system dependencies
- **Efficient Queries**: Minimize entity queries
- **Cache Results**: Cache expensive computations
- **Event-Driven**: Use events for system communication

### Performance Guidelines

- **Batch Operations**: Process entities in batches
- **Minimize Allocations**: Reuse objects to reduce GC pressure
- **Profile Regularly**: Monitor system performance
- **Optimize Hot Path**: Focus optimization on frequently called systems

### Maintainability

- **Clear Naming**: Descriptive system and method names
- **Documentation**: Document system responsibilities
- **Error Handling**: Robust error handling in systems
- **Logging**: Appropriate logging for debugging

## Future Enhancements

### Advanced Features

- **System Graphs**: Visual system dependency graphs
- **Dynamic Systems**: Runtime system creation and modification
- **System Pools**: Object pooling for system instances
- **System Templates**: Reusable system configurations
- **Distributed Systems**: Systems that span multiple machines
