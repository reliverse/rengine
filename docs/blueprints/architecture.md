# Blueprint Architecture

## System Overview

The Blueprint Visual Scripting System is built on a layered architecture that separates concerns and enables language-agnostic code representation.

## Architecture Layers

```
┌─────────────────────────────────────┐
│   User Interface (React/TypeScript) │
│   - Blueprint Canvas                │
│   - Node Palette                    │
│   - Property Panel                  │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Blueprint Graph Layer              │
│   - Node Graph Data Structure       │
│   - Connection System               │
│   - Graph Serialization             │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   AST Foundation Layer               │
│   - Language-Agnostic AST            │
│   - AST Visitors                    │
│   - AST Transformers                │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Converter Layer                    │
│   - AST ↔ Blueprint                 │
│   - Bidirectional Conversion        │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Parser/Generator Layer (Rust)      │
│   - Code Parsing                    │
│   - Code Generation                 │
│   - AST Bridge                      │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Code Files (.pwn, .ts, .rs)        │
└─────────────────────────────────────┘
```

## Core Components

### 1. AST Foundation Layer

**Location**: `src/utils/blueprint/ast/`

The AST (Abstract Syntax Tree) layer provides a language-agnostic representation of code structure.

#### Key Files

- `ast-core.ts`: Base AST node types and interfaces
- `ast-pawn.ts`: Pawn-specific AST extensions
- `ast-visitor.ts`: Visitor pattern for AST traversal

#### AST Node Types

```typescript
interface ASTNode {
  id: string;
  type: NodeType;
  position: SourcePosition;
  metadata?: Record<string, any>;
}

interface FunctionNode extends ASTNode {
  type: "function";
  name: string;
  parameters: ParameterNode[];
  returnType: TypeNode;
  body: StatementNode[];
}

interface StatementNode extends ASTNode {
  type: "statement";
  kind: "if" | "while" | "for" | "return" | "assignment";
  statement: any;
}
```

### 2. Blueprint Graph System

**Location**: `src/utils/blueprint/graph/`

The graph system manages the visual representation of code as connected nodes.

#### Key Files

- `blueprint-graph.ts`: Graph data structure and operations
- `node-types.ts`: Node type definitions
- `connection-system.ts`: Pin connection logic
- `graph-serialization.ts`: Save/load Blueprint graphs

#### Graph Structure

```typescript
interface BlueprintGraph {
  id: string;
  name: string;
  language: "pawn" | "typescript" | "rust" | "cpp";
  nodes: BlueprintNode[];
  connections: BlueprintConnection[];
  metadata: GraphMetadata;
  version: string;
}

interface BlueprintNode {
  id: string;
  type: BlueprintNodeType;
  title: string;
  position: { x: number; y: number };
  inputs: Pin[];
  outputs: Pin[];
  properties: Record<string, any>;
}

interface Pin {
  id: string;
  name: string;
  type: PinType;
  direction: "input" | "output" | "exec";
  defaultValue?: any;
}
```

### 3. Code Parser & Generator

**Location**: `src-tauri/src/blueprint/`

Rust-based parsers and generators handle code file operations.

#### Key Files

- `parser.rs`: Language-agnostic parser trait
- `pawn_parser.rs`: Pawn language parser (using pest)
- `pawn_generator.rs`: Pawn code generator from AST
- `ast_bridge.rs`: Bridge between Rust AST and TypeScript AST
- `grammar/pawn.pest`: Pawn grammar definition

#### Parser Flow

```
Pawn Code (.pwn)
    ↓
Pest Parser (Grammar-based)
    ↓
Pawn AST (Rust)
    ↓
AST Bridge
    ↓
Universal AST (JSON)
    ↓
TypeScript AST
    ↓
Blueprint Graph
```

### 4. Node Editor UI

**Location**: `src/components/blueprint/`

React-based visual editor for creating and editing Blueprints.

#### Key Components

- `blueprint-editor.tsx`: Main editor component
- `blueprint-canvas.tsx`: Canvas rendering with HTML5 Canvas
- `node-palette.tsx`: Node library sidebar
- `property-panel.tsx`: Node property editor
- `blueprint-file-list.tsx`: File management

#### Canvas Rendering

The canvas uses HTML5 Canvas API for performance:

- **Grid Rendering**: Background grid for alignment
- **Node Rendering**: Custom node drawing with pins
- **Connection Rendering**: Bezier curves for connections
- **Selection Rendering**: Visual feedback for selected nodes
- **Minimap**: Overview of entire graph

### 5. Bidirectional Sync Engine

**Location**: `src/utils/blueprint/sync/`

Keeps code files and Blueprint graphs synchronized.

#### Key Files

- `sync-manager.ts`: Main sync orchestration
- `code-to-blueprint.ts`: Convert code changes to Blueprint
- `blueprint-to-code.ts`: Convert Blueprint changes to code
- `conflict-resolution.ts`: Handle merge conflicts

#### Sync Flow

```
Code File Changed
    ↓
Parse Code → AST
    ↓
Convert AST → Blueprint
    ↓
Update Blueprint Graph
    ↓
Render in Canvas

Blueprint Changed
    ↓
Convert Blueprint → AST
    ↓
Generate Code from AST
    ↓
Write to Code File
```

#### Conflict Resolution

When both code and Blueprint are modified:

1. **Detect Conflict**: Compare hashes of both versions
2. **Analyze Changes**: Determine what changed in each
3. **Resolve Strategy**:
   - `code_wins`: Use code version
   - `blueprint_wins`: Use Blueprint version
   - `merge`: Attempt to merge both
   - `ask_user`: Prompt user for decision

### 6. Language Support

**Location**: `src/utils/blueprint/languages/pawn/`

Language-specific implementations for Pawn.

#### Key Files

- `pawn-nodes.ts`: Pawn-specific node factories
- `pawn-functions.ts`: SAMP native function library
- `pawn-types.ts`: Pawn type system
- `pawn-callbacks.ts`: SAMP callback definitions

## Data Flow

### Creating a Blueprint from Code

```
1. User opens .pwn file
   ↓
2. Rust parser reads file
   ↓
3. Parse to Pawn AST
   ↓
4. Convert to Universal AST
   ↓
5. Convert AST to Blueprint Graph
   ↓
6. Render nodes on canvas
```

### Generating Code from Blueprint

```
1. User edits Blueprint graph
   ↓
2. Convert Blueprint to AST
   ↓
3. Convert AST to Universal AST
   ↓
4. Rust generator creates code
   ↓
5. Write to .pwn file
```

## State Management

### Blueprint Store

**Location**: `src/stores/blueprint-store.ts`

Zustand store for Blueprint editor state:

```typescript
interface BlueprintState {
  currentGraph: BlueprintGraph | null;
  currentFilePath: string | null;
  isModified: boolean;
  lastSavedAt: Date | null;
}
```

### Scene Store Integration

Blueprints are integrated into the scene store:

```typescript
interface SceneState {
  // ... other state
  blueprintFiles: Array<{
    id: string;
    name: string;
    filePath: string | null;
    isModified: boolean;
    lastSavedAt?: Date;
  }>;
}
```

## Performance Considerations

### Large Graphs

- **Virtual Scrolling**: Only render visible nodes
- **Lazy Parsing**: Parse code incrementally
- **Debounced Sync**: Debounce sync operations
- **Minimap**: Efficient overview rendering

### Canvas Rendering

- **Request Animation Frame**: Smooth rendering
- **Dirty Regions**: Only redraw changed areas
- **Connection Caching**: Cache connection paths
- **Node Pooling**: Reuse node render objects

## Extension Points

### Adding New Languages

1. Create parser in Rust (`src-tauri/src/blueprint/`)
2. Add grammar file (`.pest` format)
3. Implement AST bridge
4. Create language-specific nodes
5. Add to language enum

### Adding New Node Types

1. Define node type in `node-types.ts`
2. Create node factory function
3. Add to node palette
4. Implement rendering in canvas
5. Add conversion logic (AST ↔ Blueprint)

## Security Considerations

- **Sandboxed Parsing**: Parsers run in isolated context
- **Input Validation**: Validate all user inputs
- **Type Checking**: Strict type checking for connections
- **Code Generation**: Sanitize generated code

## Future Enhancements

- **Multi-language Support**: TypeScript, Rust, C++
- **Node Templates**: Reusable node groups
- **Visual Debugging**: Breakpoints and step-through
- **Code Completion**: IntelliSense in node editor
- **Version Control**: Git integration for Blueprints
