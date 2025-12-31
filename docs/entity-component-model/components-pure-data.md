# Components = Pure Data

## Overview

In Rengine's Entity-Component-System architecture, components are strictly pure data structures containing no logic or behavior. This design ensures components are serializable, testable, and can be processed efficiently by systems.

## Core Principles

### Pure Data Definition

Components must be:

- **Data Only**: No methods, functions, or executable code
- **Serializable**: Can be converted to/from JSON without loss
- **Immutable During Processing**: Data doesn't change itself
- **Stateless**: No internal state that affects behavior
- **Deterministic**: Same data always produces same results

### Separation of Concerns

```
Component (Data) ← System (Logic) → Entity (Identity)
```

- **Components**: What an entity is/has
- **Systems**: What an entity does
- **Entities**: Which entity it is

## Component Structure

### Basic Component Format

```typescript
interface Component {
  readonly type: ComponentType; // Discriminant for type safety
  readonly version?: number;    // For migration support
  // ... pure data fields
}
```

### Transform Component Example

```typescript
interface TransformComponent {
  readonly type: 'transform';
  readonly version: 1;

  // Spatial data
  readonly position: readonly [number, number, number];
  readonly rotation: readonly [number, number, number]; // Euler angles
  readonly scale: readonly [number, number, number];

  // Hierarchy data
  readonly parent?: string; // Parent entity ID
  readonly children?: readonly string[]; // Child entity IDs
}
```

### Material Component Example

```typescript
interface MaterialComponent {
  readonly type: 'material';
  readonly version: 2;

  // Base properties
  readonly color: string; // Hex color
  readonly opacity: number; // 0.0 to 1.0

  // PBR properties
  readonly metalness: number; // 0.0 to 1.0
  readonly roughness: number; // 0.0 to 1.0
  readonly emissive: string; // Hex color

  // Texture references
  readonly diffuseMap?: string;   // Asset ID
  readonly normalMap?: string;    // Asset ID
  readonly roughnessMap?: string; // Asset ID
  readonly metalnessMap?: string; // Asset ID
}
```

## Data Types

### Primitive Types

Components use standard serializable types:

```typescript
// Numbers
readonly health: number;
readonly speed: number;
readonly damage: number;

// Strings
readonly name: string;
readonly description: string;
readonly assetId: string;

// Booleans
readonly enabled: boolean;
readonly visible: boolean;
readonly collidable: boolean;

// Arrays (readonly for immutability)
readonly tags: readonly string[];
readonly waypoints: readonly [number, number, number][];
readonly keyFrames: readonly KeyFrame[];
```

### Complex Data Structures

Nested readonly structures:

```typescript
interface AnimationComponent {
  readonly type: 'animation';
  readonly currentClip: string;
  readonly playbackState: {
    readonly time: number;
    readonly speed: number;
    readonly loop: boolean;
    readonly playing: boolean;
  };
  readonly blendTree?: {
    readonly nodes: readonly BlendNode[];
    readonly parameters: Readonly<Record<string, number>>;
  };
}
```

### Asset References

Components reference assets by ID:

```typescript
interface ModelComponent {
  readonly type: 'model';
  readonly assetId: string; // References asset registry
  readonly lodLevels: readonly {
    readonly distance: number;
    readonly assetId: string;
  }[];
  readonly animations: readonly string[]; // Animation clip names
}
```

## Immutability

### Readonly Modifier

All component properties are readonly:

```typescript
interface PhysicsComponent {
  readonly type: 'physics';

  // Body properties
  readonly bodyType: 'static' | 'dynamic' | 'kinematic';
  readonly mass: number;
  readonly friction: number;
  readonly restitution: number;

  // Collision shape
  readonly shape: {
    readonly type: 'box' | 'sphere' | 'capsule' | 'mesh';
    readonly parameters: Readonly<Record<string, number>>;
  };

  // Constraints
  readonly constraints: readonly PhysicsConstraint[];
}
```

### Update Pattern

Components are never mutated directly:

```typescript
// ❌ Wrong: Direct mutation
entity.components.get('transform')!.position[0] = 10;

// ✅ Correct: Create new component
const currentTransform = entity.components.get('transform') as TransformComponent;
const newTransform: TransformComponent = {
  ...currentTransform,
  position: [10, currentTransform.position[1], currentTransform.position[2]]
};

entityRegistry.updateComponent(entity.id, newTransform);
```

## Serialization

### JSON Compatibility

Components are designed for JSON serialization:

```typescript
function serializeComponent(component: Component): string {
  return JSON.stringify(component, null, 2);
}

function deserializeComponent(json: string): Component {
  const data = JSON.parse(json);

  // Type validation
  if (!isValidComponentType(data.type)) {
    throw new Error(`Unknown component type: ${data.type}`);
  }

  // Version checking
  if (data.version && data.version > currentVersion[data.type]) {
    return migrateComponent(data);
  }

  return data as Component;
}
```

### Binary Serialization

For performance-critical scenarios:

```typescript
class BinaryComponentSerializer {
  private buffer = new ArrayBuffer(1024);
  private view = new DataView(this.buffer);

  serialize(component: Component): ArrayBuffer {
    // Write type discriminant
    this.writeString(component.type, 0);

    // Write version
    this.writeUint32(component.version || 1, 4);

    // Write component-specific data
    switch (component.type) {
      case 'transform':
        return this.serializeTransform(component as TransformComponent);
      case 'physics':
        return this.serializePhysics(component as PhysicsComponent);
      // ... other component types
    }
  }

  private serializeTransform(component: TransformComponent): ArrayBuffer {
    const buffer = new ArrayBuffer(4 + 3 * 8 * 3); // type + 9 floats
    const view = new DataView(buffer);

    // Write position
    view.setFloat64(0, component.position[0], true);
    view.setFloat64(8, component.position[1], true);
    view.setFloat64(16, component.position[2], true);

    // Write rotation
    view.setFloat64(24, component.rotation[0], true);
    view.setFloat64(32, component.rotation[1], true);
    view.setFloat64(40, component.rotation[2], true);

    // Write scale
    view.setFloat64(48, component.scale[0], true);
    view.setFloat64(56, component.scale[1], true);
    view.setFloat64(64, component.scale[2], true);

    return buffer;
  }
}
```

## Component Factories

### Pure Creation Functions

Components are created through pure functions:

```typescript
function createTransformComponent(
  position: [number, number, number] = [0, 0, 0],
  rotation: [number, number, number] = [0, 0, 0],
  scale: [number, number, number] = [1, 1, 1]
): TransformComponent {
  return {
    type: 'transform',
    version: 1,
    position: position as readonly [number, number, number],
    rotation: rotation as readonly [number, number, number],
    scale: scale as readonly [number, number, number]
  } as const;
}

function createMaterialComponent(
  color: string = '#ffffff',
  metalness: number = 0.0,
  roughness: number = 0.5
): MaterialComponent {
  return {
    type: 'material',
    version: 2,
    color,
    opacity: 1.0,
    metalness: Math.max(0, Math.min(1, metalness)),
    roughness: Math.max(0, Math.min(1, roughness)),
    emissive: '#000000'
  } as const;
}
```

### Builder Pattern

For complex components:

```typescript
class PhysicsComponentBuilder {
  private bodyType: 'static' | 'dynamic' | 'kinematic' = 'dynamic';
  private mass = 1.0;
  private friction = 0.5;
  private restitution = 0.3;
  private shape: CollisionShape = { type: 'box', parameters: { width: 1, height: 1, depth: 1 } };

  setBodyType(type: 'static' | 'dynamic' | 'kinematic'): this {
    this.bodyType = type;
    return this;
  }

  setMass(mass: number): this {
    this.mass = Math.max(0, mass);
    return this;
  }

  setShape(shape: CollisionShape): this {
    this.shape = shape;
    return this;
  }

  build(): PhysicsComponent {
    return {
      type: 'physics',
      version: 1,
      bodyType: this.bodyType,
      mass: this.bodyType === 'static' ? 0 : this.mass,
      friction: this.friction,
      restitution: this.restitution,
      shape: this.shape,
      constraints: []
    } as const;
  }
}

// Usage
const physicsComponent = new PhysicsComponentBuilder()
  .setBodyType('dynamic')
  .setMass(10)
  .setShape({ type: 'sphere', parameters: { radius: 0.5 } })
  .build();
```

## Validation

### Component Schema Validation

Runtime validation ensures data integrity:

```typescript
interface ComponentSchema {
  type: string;
  version: number;
  properties: Record<string, PropertySchema>;
}

interface PropertySchema {
  type: 'number' | 'string' | 'boolean' | 'array' | 'object';
  required?: boolean;
  min?: number;
  max?: number;
  pattern?: RegExp;
  enum?: readonly any[];
}

const componentSchemas: Record<string, ComponentSchema> = {
  transform: {
    type: 'transform',
    version: 1,
    properties: {
      position: { type: 'array', required: true },
      rotation: { type: 'array', required: true },
      scale: { type: 'array', required: true }
    }
  }
};

function validateComponent(component: Component): ValidationResult {
  const schema = componentSchemas[component.type];
  if (!schema) {
    return { valid: false, errors: [`Unknown component type: ${component.type}`] };
  }

  const errors: string[] = [];

  // Check required properties
  for (const [propName, propSchema] of Object.entries(schema.properties)) {
    if (propSchema.required && !(propName in component)) {
      errors.push(`Missing required property: ${propName}`);
    }
  }

  // Validate property types and values
  for (const [propName, value] of Object.entries(component)) {
    if (propName === 'type' || propName === 'version') continue;

    const propSchema = schema.properties[propName];
    if (!propSchema) {
      errors.push(`Unknown property: ${propName}`);
      continue;
    }

    if (!validateProperty(value, propSchema)) {
      errors.push(`Invalid property ${propName}: ${value}`);
    }
  }

  return { valid: errors.length === 0, errors };
}
```

## Performance Benefits

### Memory Layout

Pure data components enable efficient memory usage:

- **Contiguous Storage**: Components stored in contiguous arrays
- **Cache-Friendly**: Predictable memory access patterns
- **SIMD Processing**: Vectorizable operations on component arrays
- **Reduced GC Pressure**: No object allocation during processing

### Processing Efficiency

Systems can process components efficiently:

```typescript
class TransformSystem {
  // Process all transform components
  updateTransforms(entities: Entity[]): void {
    // Components are stored contiguously for SIMD processing
    const transforms = entities
      .map(e => e.components.get('transform'))
      .filter(Boolean) as TransformComponent[];

    // Vectorized position updates (conceptual SIMD)
    for (let i = 0; i < transforms.length; i += 4) {
      // Process 4 transforms simultaneously
      this.updateTransformBatch(transforms.slice(i, i + 4));
    }
  }

  private updateTransformBatch(batch: TransformComponent[]): void {
    // SIMD-friendly operations
    for (const transform of batch) {
      // Pure mathematical operations on pure data
      transform.position[0] += transform.velocity?.[0] || 0;
      transform.position[1] += transform.velocity?.[1] || 0;
      transform.position[2] += transform.velocity?.[2] || 0;
    }
  }
}
```

## Testing

### Pure Function Testing

Components enable easy unit testing:

```typescript
describe('TransformComponent', () => {
  it('should create valid transform component', () => {
    const transform = createTransformComponent([1, 2, 3], [0, 0, 0], [1, 1, 1]);

    expect(transform.type).toBe('transform');
    expect(transform.position).toEqual([1, 2, 3]);
    expect(transform.rotation).toEqual([0, 0, 0]);
    expect(transform.scale).toEqual([1, 1, 1]);
  });

  it('should serialize and deserialize correctly', () => {
    const original = createTransformComponent([5, 10, 15]);
    const serialized = serializeComponent(original);
    const deserialized = deserializeComponent(serialized);

    expect(deserialized).toEqual(original);
  });
});
```

### Deterministic Behavior

Pure components ensure deterministic system behavior:

```typescript
describe('PhysicsSystem', () => {
  it('should produce consistent results', () => {
    const component1 = createPhysicsComponent({ mass: 10, velocity: [1, 0, 0] });
    const component2 = createPhysicsComponent({ mass: 10, velocity: [1, 0, 0] });

    const result1 = physicsSystem.simulate(component1, 1.0);
    const result2 = physicsSystem.simulate(component2, 1.0);

    expect(result1).toEqual(result2); // Deterministic results
  });
});
```

## Best Practices

### Design Guidelines

- **Keep Components Small**: Focus on single responsibilities
- **Use Descriptive Names**: Clear property names
- **Provide Sensible Defaults**: Reasonable default values
- **Version Components**: Include version for migrations
- **Document Properties**: Clear documentation for each property

### Performance Tips

- **Minimize Property Count**: Fewer properties = better cache performance
- **Use Fixed-Size Arrays**: Prefer tuples over variable-length arrays
- **Avoid Deep Nesting**: Shallow component structures
- **Preallocate Arrays**: Known sizes improve memory layout

### Maintenance Practices

- **Immutable Updates**: Always create new components for changes
- **Schema Validation**: Validate components in development
- **Migration Paths**: Plan for component evolution
- **Documentation**: Keep component documentation current

## Future Enhancements

### Advanced Features

- **Component DSL**: Domain-specific language for component definitions
- **Runtime Component Generation**: Dynamic component creation from schemas
- **Component Compression**: Efficient storage of component data
- **Component Streaming**: Progressive loading of component data
- **Component Queries**: Advanced query language for component selection
