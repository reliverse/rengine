# Scene Graph (Entities + Parent/Child Relationships)

## Overview

Rengine implements a hierarchical scene graph system that organizes entities in parent-child relationships, enabling complex object transformations and scene management.

## Core Concepts

### Entities

In Rengine, entities are the fundamental building blocks of scenes. Each entity represents an object in the 3D world and can contain:

- **Scene Objects**: 3D geometry (cubes, spheres, planes, imported models)
- **Lights**: Various light types (directional, point, spot, ambient, hemisphere)
- **Transform Data**: Position, rotation, and scale in 3D space

### Scene Graph Structure

The scene graph is a tree-like data structure where:

- **Root Node**: The top-level scene container
- **Parent-Child Relationships**: Entities can have parent and child entities
- **Transform Hierarchy**: Child transforms are relative to their parent transforms

## Implementation Details

### Entity Management

```typescript
interface SceneObject {
  id: string;
  name: string;
  type: "cube" | "sphere" | "plane" | "imported";
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
  visible: boolean;
  // ... additional properties
}
```

### Parent-Child Relationships

Child entities inherit transformations from their parent:

- **Local Transform**: Transform relative to parent
- **World Transform**: Absolute transform in world space
- **Hierarchical Updates**: Changes to parent affect all children

## Features

### Scene Hierarchy Panel

The scene hierarchy panel provides:

- **Visual Tree View**: Expandable/collapsible entity tree
- **Drag & Drop**: Reparent entities by dragging
- **Selection Management**: Click to select, multi-select with Ctrl
- **Visibility Toggle**: Show/hide entities and hierarchies

### Transform Operations

- **Local vs World Space**: Toggle between local and world coordinate systems
- **Parent Constraints**: Child transforms respect parent limitations
- **Pivot Points**: Transform around custom pivot points

## Benefits

### Organization
- **Logical Grouping**: Related objects can be grouped under parent entities
- **Scene Management**: Complex scenes remain organized and manageable
- **Asset Reuse**: Parent-child hierarchies enable reusable object templates

### Performance
- **Culling Optimization**: Hierarchical culling for visibility determination
- **Transform Caching**: Efficient transform matrix calculations
- **LOD Management**: Level-of-detail based on hierarchy depth

### Animation & Physics
- **Hierarchical Animation**: Animate entire object hierarchies
- **Physics Simulation**: Rigid body hierarchies with joint constraints
- **Collision Detection**: Compound collision shapes for complex objects

## Usage Examples

### Creating Object Hierarchies

```typescript
// Create a robot arm hierarchy
const robotBase = sceneStore.addObject("cube", [0, 0, 0]);
const robotArm = sceneStore.addObject("cube", [0, 2, 0]);
const robotHand = sceneStore.addObject("cube", [0, 1, 0]);

// Set parent-child relationships
robotArm.parent = robotBase;
robotHand.parent = robotArm;
```

### Transform Inheritance

When a parent object rotates, all children maintain their relative positions:

- Parent rotation: 45° around Y-axis
- Child local position: [0, 1, 0]
- Child world position: Calculated based on parent's transformation matrix

## Future Enhancements

- **Advanced Parenting**: Multiple parent support for complex hierarchies
- **Transform Constraints**: Limit child movement/rotation relative to parents
- **Instancing**: Share geometry across multiple entity instances
- **Scene Templates**: Predefined hierarchical object templates
