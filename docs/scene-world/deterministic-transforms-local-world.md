# Deterministic Transforms (Local / World Space)

## Overview

Rengine implements a deterministic transform system that provides consistent and predictable object positioning, rotation, and scaling in both local and world coordinate spaces. This ensures reliable behavior across different platforms and runtime conditions.

## Core Concepts

### Coordinate Spaces

#### Local Space
- **Definition**: Coordinate system relative to an object's parent
- **Origin**: Object's pivot point (usually center or base)
- **Scale**: [1, 1, 1] = same size as parent coordinate system
- **Use Case**: Object-specific transformations and animations

#### World Space
- **Definition**: Absolute coordinate system of the scene
- **Origin**: Scene origin (0, 0, 0)
- **Scale**: Absolute units in world coordinates
- **Use Case**: Global positioning and physics calculations

### Transform Components

Each entity has three fundamental transform properties:

#### Position (Translation)
```typescript
position: [number, number, number] // [x, y, z] coordinates
```

#### Rotation (Orientation)
```typescript
rotation: [number, number, number] // Euler angles in radians [rx, ry, rz]
```

#### Scale (Size)
```typescript
scale: [number, number, number] // [sx, sy, sz] scale factors
```

## Transform Mathematics

### Transformation Matrix

Each object maintains a 4x4 transformation matrix that combines translation, rotation, and scale:

```
[ sx*cos(ry)*cos(rz)    sx*(-cos(rx)*sin(rz) + sin(rx)*sin(ry)*cos(rz))    sx*(sin(rx)*sin(rz) + cos(rx)*sin(ry)*cos(rz))    tx ]
[ sy*sin(rx)*cos(ry)    sy*(cos(rx)*cos(rz) + sin(rx)*sin(ry)*sin(rz))     sy*(-sin(rx)*cos(rz) + cos(rx)*sin(ry)*sin(rz))   ty ]
[ sz*(-sin(ry))         sz*sin(rx)*cos(ry)                                 sz*cos(rx)*cos(ry)                               tz ]
[ 0                      0                                                  0                                                  1  ]
```

### Hierarchical Transforms

For objects in a parent-child hierarchy:

1. **Local Transform**: Object's transform relative to parent
2. **Parent Transform**: Parent's world transform
3. **World Transform**: Local × Parent (matrix multiplication)

### Deterministic Calculations

All transform calculations use:

- **Fixed Precision**: Consistent floating-point arithmetic
- **Matrix Operations**: Standard linear algebra for composition
- **Quaternion Internals**: Rotation represented as quaternions internally
- **Euler Angle Conversion**: Deterministic conversion between representations

## Implementation Details

### Transform System Architecture

```typescript
class Transform {
  private localPosition: Vector3;
  private localRotation: Quaternion;
  private localScale: Vector3;

  private worldPosition: Vector3;
  private worldRotation: Quaternion;
  private worldScale: Vector3;

  private localMatrix: Matrix4;
  private worldMatrix: Matrix4;

  private parent: Transform | null;
  private children: Transform[];
}
```

### Update Propagation

Transform updates propagate through the hierarchy:

1. **Bottom-up**: Child transforms update first
2. **Top-down**: World matrices calculated from root
3. **Dirty Flags**: Only recalculate when necessary
4. **Caching**: Store computed matrices to avoid redundant calculations

## Transform Operations

### Translation (Movement)

```typescript
// Local space movement
object.translateLocal([1, 0, 0]); // Move 1 unit right in local space

// World space movement
object.translateWorld([0, 1, 0]); // Move 1 unit up in world space

// Set absolute position
object.setPosition([10, 5, 0]); // World space coordinates
```

### Rotation

```typescript
// Rotate around local axis
object.rotateLocal([0, Math.PI/2, 0]); // 90° around local Y-axis

// Rotate around world axis
object.rotateWorld([0, 0, Math.PI]); // 180° around world Z-axis

// Look at target
object.lookAt(targetPosition); // Face towards point in world space
```

### Scaling

```typescript
// Uniform scaling
object.setScale([2, 2, 2]); // Double size in all directions

// Non-uniform scaling
object.setScale([1, 2, 0.5]); // Stretch Y, squash Z

// Reset to original size
object.setScale([1, 1, 1]);
```

## Space Conversion

### Local to World

```typescript
const worldPosition = localToWorld(localPosition, parentWorldMatrix);
const worldDirection = localToWorldDirection(localDirection, parentWorldRotation);
```

### World to Local

```typescript
const localPosition = worldToLocal(worldPosition, parentWorldMatrix);
const localDirection = worldToLocalDirection(worldDirection, parentWorldRotation);
```

## Gizmo Integration

The transform gizmos respect coordinate space settings:

- **Local Space Gizmos**: Oriented to object's local coordinate system
- **World Space Gizmos**: Aligned to world axes regardless of object rotation
- **Parent Space Gizmos**: Aligned to parent's coordinate system

## Physics Integration

### Rigid Body Transforms

Physics bodies maintain synchronization with transform system:

- **Kinematic Bodies**: Transforms drive physics simulation
- **Dynamic Bodies**: Physics simulation updates transforms
- **Collision Shapes**: Transform matrices used for collision detection

### Joint Constraints

Physics joints use transform spaces for constraint calculations:

- **Hinge Joints**: Rotation limits in local space
- **Ball Joints**: Angular constraints in parent space
- **Slider Joints**: Linear movement along specified axes

## Animation System

### Keyframe Animation

Animation curves work in specified coordinate spaces:

- **Local Space Animation**: Animate relative to parent
- **World Space Animation**: Animate in absolute coordinates
- **Blend Spaces**: Interpolate between different coordinate systems

### Skeletal Animation

Bone transforms use hierarchical coordinate spaces:

- **Bone Local Space**: Relative to parent bone
- **Model Space**: Relative to model root
- **World Space**: Absolute positioning for attachment points

## Deterministic Guarantees

### Cross-Platform Consistency

- **Floating Point Precision**: Consistent results across platforms
- **Matrix Decomposition**: Reliable extraction of TRS components
- **Quaternio n Normalization**: Prevents drift in rotations
- **Scale Constraints**: Prevent negative scales and zero divisions

### Serialization

Transform state serializes deterministically:

```json
{
  "position": [1.234567, -0.987654, 3.141592],
  "rotation": [0.523598, 1.047197, -0.785398],
  "scale": [1.0, 2.5, 0.8]
}
```

## Performance Optimizations

### Transform Caching

- **Matrix Caching**: Store computed matrices until invalidated
- **Dirty Flag System**: Only recalculate when transforms change
- **Hierarchical Updates**: Batch updates for efficiency

### SIMD Operations

- **Vector Math**: SIMD-accelerated vector operations
- **Matrix Multiplication**: Optimized matrix operations
- **Quaternion Math**: Fast quaternion calculations

## Debugging Features

### Transform Visualization

- **Local Axes**: Display object's local coordinate system
- **World Axes**: Show world coordinate system
- **Transform Hierarchy**: Visualize parent-child relationships
- **Bounding Boxes**: Display object bounds in different spaces

### Transform Inspector

Debug panel showing:

- **Local Properties**: Position, rotation, scale in local space
- **World Properties**: Absolute values in world space
- **Matrix Values**: Raw transformation matrix
- **Parent Chain**: Full hierarchy path

## Best Practices

### Coordinate Space Selection

- **Use Local Space**: For object-relative operations (animations, attachments)
- **Use World Space**: For global positioning (navigation, physics)
- **Consistent Spaces**: Choose one space for related operations

### Transform Order

- **Scale First**: Apply scaling before rotation
- **Rotate Second**: Apply rotation in desired space
- **Translate Last**: Apply translation after other transforms

### Hierarchy Design

- **Stable Parents**: Use stable parent transforms for children
- **Avoid Deep Hierarchies**: Limit hierarchy depth for performance
- **Logical Grouping**: Group related objects under common parents
