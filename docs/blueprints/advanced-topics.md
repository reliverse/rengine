# Advanced Topics

## Custom Node Types

### Creating Custom Nodes

You can extend the Blueprint system with custom node types:

#### 1. Define Node Type

```typescript
// src/utils/blueprint/languages/pawn/pawn-nodes.ts
export function createCustomNode(
  position: { x: number; y: number }
): BlueprintNode {
  return createBlueprintNode(
    "custom",
    "Custom Node",
    position,
    [createDataPin("input", "Input", "int", true)],
    [createDataPin("output", "Output", "int")],
    {
      customProperty: "value",
    }
  );
}
```

#### 2. Add to Node Palette

```typescript
// src/components/blueprint/node-palette.tsx
const CUSTOM_NODES = [
  {
    type: "custom",
    title: "Custom Node",
    description: "A custom node type",
    icon: CustomIcon,
    color: "text-blue-500",
  },
];
```

#### 3. Handle in Converters

Update AST converters to handle your custom node type:

```typescript
// src/utils/blueprint/converters/blueprint-to-ast.ts
case "custom":
  return convertCustomNode(node, graph);
```

## AST Extensions

### Custom AST Nodes

Extend the AST system for language-specific features:

```typescript
// src/utils/blueprint/ast/ast-pawn.ts
export interface CustomPawnNode extends ASTNode {
  type: "custom_pawn";
  customData: any;
}
```

### AST Transformations

Apply transformations during conversion:

```typescript
// Transform AST before generating code
const transformedAST = transformAST(ast, {
  optimize: true,
  inlineConstants: true,
});
```

## Code Generation Customization

### Custom Formatters

Control code formatting:

```typescript
const options: GenerationOptions = {
  indentSize: 4,
  useTabs: false,
  preserveComments: true,
  style: "compact", // or "verbose"
};
```

### Code Templates

Use templates for code generation:

```typescript
const template = `
public OnPlayerConnect(playerid)
{
    ${generateBody(ast)}
    return 1;
}
`;
```

## Sync Extensions

### Custom Sync Hooks

Add hooks for sync events:

```typescript
syncManager.onCodeChange((ast) => {
  // Custom processing
  validateAST(ast);
  optimizeAST(ast);
});

syncManager.onConflict((conflict) => {
  // Custom conflict handling
  logConflict(conflict);
  notifyUser(conflict);
});
```

### Sync Filters

Filter what gets synced:

```typescript
const filter = {
  includeNodes: (node: BlueprintNode) => {
    return node.type !== "comment";
  },
  includeConnections: (conn: BlueprintConnection) => {
    return conn.metadata?.important !== false;
  },
};
```

## Performance Optimization

### Large Graph Handling

For graphs with 1000+ nodes:

1. **Virtual Rendering**: Only render visible nodes
2. **Lazy Loading**: Load nodes on demand
3. **Incremental Updates**: Update only changed nodes
4. **Debouncing**: Debounce expensive operations

### Memory Management

- **Node Pooling**: Reuse node objects
- **Connection Caching**: Cache connection lookups
- **Garbage Collection**: Clean up unused objects

## Debugging

### Debug Mode

Enable debug mode for development:

```typescript
const DEBUG = true;

if (DEBUG) {
  console.log("Node created:", node);
  console.log("Connection made:", connection);
  console.log("AST generated:", ast);
}
```

### Validation

Validate graphs and ASTs:

```typescript
function validateGraph(graph: BlueprintGraph): ValidationResult {
  const errors: string[] = [];
  
  // Check for orphaned nodes
  // Check for invalid connections
  // Check for type mismatches
  
  return { valid: errors.length === 0, errors };
}
```

## Testing

### Unit Tests

Test individual components:

```typescript
describe("Blueprint Graph", () => {
  it("should create a graph", () => {
    const graph = createBlueprintGraph("Test", "pawn");
    expect(graph.nodes).toEqual([]);
  });
  
  it("should add a node", () => {
    const graph = createBlueprintGraph("Test", "pawn");
    const node = createLiteralNode(42, "int", { x: 0, y: 0 });
    const updated = addNode(graph, node);
    expect(updated.nodes).toHaveLength(1);
  });
});
```

### Integration Tests

Test full workflows:

```typescript
describe("Code to Blueprint", () => {
  it("should convert code to Blueprint", async () => {
    const code = "public OnPlayerConnect(playerid) { return 1; }";
    const ast = await parsePawnCode(code);
    const graph = convertASTToBlueprint(ast);
    expect(graph.nodes.length).toBeGreaterThan(0);
  });
});
```

## Extending Language Support

### Adding New Languages

1. **Create Parser**: Implement parser in Rust
2. **Create Generator**: Implement code generator
3. **Define AST Mapping**: Map language AST to universal AST
4. **Add Node Types**: Define language-specific nodes
5. **Update UI**: Add language to UI

### Example: TypeScript Support

```rust
// src-tauri/src/blueprint/typescript_parser.rs
pub struct TypeScriptParser;

impl Parser for TypeScriptParser {
    fn parse(&self, source: &str) -> Result<ParseResult, String> {
        // Parse TypeScript
    }
    
    fn language(&self) -> &str {
        "typescript"
    }
}
```

## Plugin System

### Creating Plugins

Extend functionality with plugins:

```typescript
interface BlueprintPlugin {
  name: string;
  version: string;
  nodes: NodeDefinition[];
  hooks: PluginHooks;
}

const myPlugin: BlueprintPlugin = {
  name: "My Plugin",
  version: "1.0.0",
  nodes: [/* custom nodes */],
  hooks: {
    onNodeCreate: (node) => { /* ... */ },
    onConnectionCreate: (conn) => { /* ... */ },
  },
};
```

## Best Practices

### Code Organization

1. **Modular Design**: Keep components separate
2. **Type Safety**: Use TypeScript strictly
3. **Error Handling**: Handle errors gracefully
4. **Documentation**: Document complex logic

### Performance

1. **Optimize Rendering**: Only render what's visible
2. **Cache Results**: Cache expensive computations
3. **Debounce Updates**: Prevent excessive updates
4. **Lazy Load**: Load data on demand

### Maintainability

1. **Clear Naming**: Use descriptive names
2. **Consistent Style**: Follow code style guide
3. **Test Coverage**: Write comprehensive tests
4. **Version Control**: Use Git effectively

## Troubleshooting

### Common Issues

#### Nodes Not Rendering
- Check node positions are valid
- Verify viewport is correct
- Check for rendering errors in console

#### Connections Not Working
- Verify pin types match
- Check connection validation
- Ensure nodes exist

#### Sync Not Working
- Check file watcher is enabled
- Verify file permissions
- Check for parse errors

#### Performance Issues
- Reduce node count
- Enable virtual rendering
- Optimize graph structure

## Future Enhancements

Potential future features:

- **Node Templates**: Save and reuse node groups
- **Visual Debugging**: Step through execution visually
- **Code Completion**: Autocomplete in node editor
- **Multi-File Support**: Work with multiple files
- **Version Control**: Git integration
- **Collaboration**: Real-time collaboration
- **Export Formats**: Export to other formats
- **Node Marketplace**: Share custom nodes
