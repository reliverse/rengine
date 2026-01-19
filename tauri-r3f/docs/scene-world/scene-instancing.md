# Scene Instancing

## Overview

Rengine implements a powerful scene instancing system that allows efficient reuse of scene templates, prefabs, and complex object hierarchies. This system enables memory-efficient duplication of scene elements while maintaining individual instance customization.

## Core Concepts

### Instancing vs Duplication

#### Traditional Duplication
- **Memory Usage**: Each copy stores complete data
- **Storage**: Full scene data for each instance
- **Updates**: Changes require updating each copy individually
- **Performance**: High memory overhead for complex scenes

#### Scene Instancing
- **Memory Usage**: Shared template data with instance-specific overrides
- **Storage**: Template + instance data (transforms, property overrides)
- **Updates**: Template changes affect all instances automatically
- **Performance**: Minimal memory overhead, efficient rendering

### Instance Types

Rengine supports multiple instancing patterns:

#### Template Instancing
- **Definition**: Instances of predefined scene templates
- **Use Case**: Reusable scene elements (furniture, buildings, characters)
- **Data Structure**: Template ID + instance overrides

#### Prefab Instancing
- **Definition**: Instances of prefab objects with hierarchies
- **Use Case**: Complex objects with multiple components
- **Data Structure**: Prefab reference + hierarchical overrides

#### Scene Instancing
- **Definition**: Instances of entire scenes within scenes
- **Use Case**: Level composition, modular environments
- **Data Structure**: Scene reference + placement transforms

## Implementation Details

### Instance Data Structure

```typescript
interface SceneInstance {
  id: string;
  templateId: string; // Reference to template/prefab/scene
  templateType: "template" | "prefab" | "scene";

  // Instance-specific data
  transform: {
    position: [number, number, number];
    rotation: [number, number, number];
    scale: [number, number, number];
  };

  // Property overrides
  overrides: {
    [entityId: string]: {
      [propertyPath: string]: any;
    };
  };

  // Instance metadata
  name: string;
  visible: boolean;
  locked: boolean;
}
```

### Template Registry

Central registry manages all instancing templates:

```typescript
class InstanceRegistry {
  private templates = new Map<string, SceneTemplate>();
  private prefabs = new Map<string, PrefabDefinition>();
  private scenes = new Map<string, SceneDefinition>();

  registerTemplate(id: string, template: SceneTemplate): void {
    this.templates.set(id, template);
  }

  createInstance(templateId: string, transform: Transform): SceneInstance {
    const template = this.templates.get(templateId);
    if (!template) {
      throw new Error(`Template ${templateId} not found`);
    }

    return {
      id: generateEntityId("instance"),
      templateId,
      templateType: "template",
      transform,
      overrides: {},
      name: `${template.name} Instance`,
      visible: true,
      locked: false
    };
  }
}
```

## Features

### Property Overrides

Instances can override template properties:

```typescript
// Create instance with color override
const chairInstance = instanceRegistry.createInstance("chair_template", transform);
chairInstance.overrides = {
  "chair_mesh": {
    "material.color": "#ff0000", // Red chair
    "material.roughness": 0.3
  }
};
```

### Hierarchical Overrides

Complex hierarchies support nested overrides:

```typescript
const carInstance = instanceRegistry.createInstance("car_prefab", transform);
carInstance.overrides = {
  "car_body": {
    "material.color": "#0000ff" // Blue car body
  },
  "wheel_front_left": {
    "transform.rotation": [0, 0, Math.PI/4] // Turned wheel
  },
  "wheel_front_right": {
    "transform.rotation": [0, 0, Math.PI/4]
  }
};
```

## Performance Optimizations

### Memory Management

#### Shared Geometry
- **Instancing**: Single geometry shared across instances
- **Batching**: Group similar instances for efficient rendering
- **LOD**: Level-of-detail based on distance/camera settings

#### Transform Instancing
```typescript
// GPU instancing for transform data
const instanceTransforms = new InstancedBufferAttribute(
  new Float32Array(instanceCount * 16), // 4x4 matrices
  16
);

// Update instance transforms efficiently
function updateInstanceTransforms(instances: SceneInstance[]): void {
  instances.forEach((instance, index) => {
    const matrix = computeTransformMatrix(instance.transform);
    instanceTransforms.set(matrix.elements, index * 16);
  });
  instanceTransforms.needsUpdate = true;
}
```

### Rendering Optimization

#### Frustum Culling
- **Instance Bounds**: Calculate bounding volumes for culling
- **Hierarchical Culling**: Cull entire instance groups
- **Occlusion Culling**: Hide instances behind other geometry

#### Draw Call Batching
```typescript
// Batch instances with same material
const materialBatches = new Map<Material, SceneInstance[]>();

function batchInstances(instances: SceneInstance[]): RenderBatch[] {
  const batches: RenderBatch[] = [];

  // Group by material and geometry
  for (const instance of instances) {
    const template = getTemplate(instance.templateId);
    const key = `${template.geometry.id}_${template.material.id}`;

    if (!materialBatches.has(key)) {
      materialBatches.set(key, []);
    }
    materialBatches.get(key)!.push(instance);
  }

  // Create render batches
  for (const [key, batchInstances] of materialBatches) {
    batches.push({
      geometry: getGeometry(key),
      material: getMaterial(key),
      instances: batchInstances,
      transformArray: createTransformArray(batchInstances)
    });
  }

  return batches;
}
```

## Scene Composition

### Modular Level Design

Build complex scenes from instance libraries:

```typescript
class SceneComposer {
  private instances: SceneInstance[] = [];

  addInstance(templateId: string, position: Vector3): SceneInstance {
    const instance = instanceRegistry.createInstance(templateId, {
      position: position.toArray(),
      rotation: [0, 0, 0],
      scale: [1, 1, 1]
    });

    this.instances.push(instance);
    return instance;
  }

  createBuilding(buildingType: string, position: Vector3): void {
    // Create building instance
    const building = this.addInstance(`building_${buildingType}`, position);

    // Add interior instances based on building type
    if (buildingType === "house") {
      this.addInstance("furniture_chair", position.add(new Vector3(2, 0, 1)));
      this.addInstance("furniture_table", position.add(new Vector3(2, 0, 0)));
    }
  }
}
```

### Procedural Generation

Generate scenes procedurally using instances:

```typescript
class ProceduralGenerator {
  generateForest(size: number): SceneInstance[] {
    const instances: SceneInstance[] = [];
    const treeTemplates = ["tree_oak", "tree_pine", "tree_birch"];

    for (let x = 0; x < size; x += 10) {
      for (let z = 0; z < size; z += 10) {
        const treeType = treeTemplates[Math.floor(Math.random() * treeTemplates.length)];
        const position = [x + Math.random() * 8, 0, z + Math.random() * 8];
        const rotation = [0, Math.random() * Math.PI * 2, 0];
        const scale = [0.8 + Math.random() * 0.4, 0.8 + Math.random() * 0.4, 0.8 + Math.random() * 0.4];

        const instance = instanceRegistry.createInstance(treeType, {
          position,
          rotation,
          scale
        });

        instances.push(instance);
      }
    }

    return instances;
  }
}
```

## Template Management

### Template Creation

Create templates from existing scenes:

```typescript
function createTemplateFromSelection(entities: SceneEntity[], name: string): SceneTemplate {
  // Extract common properties
  const bounds = calculateBoundingBox(entities);
  const center = bounds.getCenter(new Vector3());

  // Create template definition
  const template: SceneTemplate = {
    id: generateTemplateId(),
    name,
    version: "1.0.0",
    entities: entities.map(entity => ({
      ...entity,
      // Make transforms relative to center
      transform: {
        position: entity.transform.position.clone().sub(center).toArray(),
        rotation: entity.transform.rotation.toArray(),
        scale: entity.transform.scale.toArray()
      }
    })),
    bounds: {
      min: bounds.min.toArray(),
      max: bounds.max.toArray()
    },
    thumbnail: generateThumbnail(entities),
    tags: ["user-created"],
    metadata: {
      created: new Date().toISOString(),
      author: getCurrentUser()
    }
  };

  return template;
}
```

### Template Library

Organized template management:

```typescript
class TemplateLibrary {
  private templates = new Map<string, SceneTemplate>();
  private categories = new Map<string, string[]>();

  addTemplate(template: SceneTemplate, category: string = "uncategorized"): void {
    this.templates.set(template.id, template);

    if (!this.categories.has(category)) {
      this.categories.set(category, []);
    }
    this.categories.get(category)!.push(template.id);
  }

  getTemplatesByCategory(category: string): SceneTemplate[] {
    const templateIds = this.categories.get(category) || [];
    return templateIds.map(id => this.templates.get(id)!).filter(Boolean);
  }

  searchTemplates(query: string): SceneTemplate[] {
    const results: SceneTemplate[] = [];
    const searchTerm = query.toLowerCase();

    for (const template of this.templates.values()) {
      if (template.name.toLowerCase().includes(searchTerm) ||
          template.tags.some(tag => tag.toLowerCase().includes(searchTerm))) {
        results.push(template);
      }
    }

    return results;
  }
}
```

## Instance Editing

### Override System

Runtime instance customization:

```typescript
class InstanceEditor {
  setInstanceOverride(instance: SceneInstance, entityId: string, propertyPath: string, value: any): void {
    if (!instance.overrides[entityId]) {
      instance.overrides[entityId] = {};
    }
    instance.overrides[entityId][propertyPath] = value;

    // Mark instance as modified
    instance.modified = true;

    // Update rendering if necessary
    updateInstanceRendering(instance);
  }

  resetInstanceOverride(instance: SceneInstance, entityId: string, propertyPath: string): void {
    if (instance.overrides[entityId]) {
      delete instance.overrides[entityId][propertyPath];

      if (Object.keys(instance.overrides[entityId]).length === 0) {
        delete instance.overrides[entityId];
      }
    }

    instance.modified = true;
    updateInstanceRendering(instance);
  }
}
```

### Visual Editing

Editor integration for instance manipulation:

- **Transform Gizmos**: Manipulate instance positions/rotations/scales
- **Property Panels**: Edit instance-specific overrides
- **Instance Outliner**: Hierarchical view of instance contents
- **Preview Rendering**: Real-time preview of changes

## Serialization

### Instance Storage

Efficient storage of instance data:

```json
{
  "instances": [
    {
      "id": "instance_1640995200000_abc123def",
      "templateId": "building_house_001",
      "templateType": "prefab",
      "transform": {
        "position": [10, 0, 5],
        "rotation": [0, 0, 0],
        "scale": [1, 1, 1]
      },
      "overrides": {
        "roof": {
          "material.color": "#8B4513"
        },
        "door": {
          "material.color": "#654321"
        }
      },
      "name": "House Instance 1",
      "visible": true,
      "locked": false
    }
  ]
}
```

## Performance Metrics

### Memory Savings

- **Geometry Sharing**: 80-95% reduction for duplicated objects
- **Material Sharing**: 50-80% reduction for similar materials
- **Hierarchy Optimization**: 60-90% reduction for complex prefabs

### Rendering Performance

- **Draw Call Reduction**: 70-90% fewer draw calls for instanced scenes
- **GPU Efficiency**: Better utilization of GPU instancing features
- **Frustum Culling**: Faster culling for large numbers of instances

## Use Cases

### Game Development

#### Environment Design
- **City Buildings**: Instance different building types
- **Vegetation**: Scatter trees, rocks, and grass
- **Crowd Simulation**: Instance character models

#### Level Design
- **Modular Pieces**: Rooms, corridors, platforms
- **Interactive Objects**: Doors, switches, containers
- **Destructible Objects**: Breakable crates, walls

### Architectural Visualization

#### Building Models
- **Furniture**: Chairs, tables, lighting fixtures
- **Structural Elements**: Walls, floors, ceilings
- **Landscaping**: Trees, benches, vehicles

### Industrial Applications

#### Product Catalogs
- **Product Variants**: Different colors, sizes, configurations
- **Assembly Lines**: Machine components and tools
- **Warehouse Layouts**: Shelving units and storage containers

## Future Enhancements

### Advanced Features

- **Nested Instancing**: Instances containing other instances
- **Procedural Instancing**: Algorithmic instance placement
- **Instance Animation**: Animate instance properties over time
- **LOD Instancing**: Different detail levels for instances
- **Instance Networking**: Multiplayer instance synchronization
