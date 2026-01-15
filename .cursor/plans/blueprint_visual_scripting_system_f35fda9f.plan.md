---
name: Blueprint Visual Scripting System
overview: Implement a UE5-like Blueprint Visual Scripting system that is programming language agnostic, with full Pawn support as the first implementation. The system will maintain bidirectional sync between visual Blueprint nodes and actual code files, allowing users to seamlessly switch between visual and code editing.
todos: []
---

# Blueprint Visual Scripting System Implementation Plan

## Architecture Overview

The Blueprint system will be built on an AST (Abstract Syntax Tree) foundation where:

- **Visual Blueprints** are stored as node graphs representing code structure
- **Code files** (`.pwn`, `.ts`, `.rs`, etc.) are the source of truth
- **Bidirectional sync** keeps both representations in sync
- **Language-agnostic core** with language-specific parsers/generators

## Core Components

### 1. AST Foundation Layer (`src/utils/blueprint/ast/`)

**Purpose**: Language-agnostic AST representation that can represent any programming language construct.

**Key Files**:

- `ast-core.ts` - Base AST node types and interfaces
- `ast-pawn.ts` - Pawn-specific AST nodes and transformations
- `ast-visitor.ts` - Visitor pattern for AST traversal

**AST Node Types**:

```typescript
interface ASTNode {
  id: string;
  type: NodeType;
  position: { line: number; column: number };
}

interface FunctionNode extends ASTNode {
  name: string;
  parameters: ParameterNode[];
  returnType: TypeNode;
  body: StatementNode[];
}

interface StatementNode extends ASTNode {
  kind: 'expression' | 'assignment' | 'if' | 'while' | 'return' | 'call';
}
```

### 2. Blueprint Node Graph System (`src/utils/blueprint/graph/`)

**Purpose**: Visual representation of code as connected nodes.

**Key Files**:

- `blueprint-graph.ts` - Graph data structure and operations
- `node-types.ts` - Node type definitions (function, variable, control flow, etc.)
- `connection-system.ts` - Node connection/pin system
- `graph-serialization.ts` - Save/load Blueprint graphs

**Node Graph Structure**:

```typescript
interface BlueprintGraph {
  id: string;
  name: string;
  language: 'pawn' | 'typescript' | 'rust' | 'cpp';
  nodes: BlueprintNode[];
  connections: BlueprintConnection[];
  metadata: GraphMetadata;
}

interface BlueprintNode {
  id: string;
  type: NodeType;
  position: { x: number; y: number };
  inputs: Pin[];
  outputs: Pin[];
  properties: Record<string, any>;
}

interface Pin {
  id: string;
  name: string;
  type: DataType;
  defaultValue?: any;
}
```

### 3. Code Parser & Generator (`src-tauri/src/blueprint/`)

**Purpose**: Parse code files into AST and generate code from AST.

**Key Files**:

- `parser.rs` - Language-agnostic parser interface
- `pawn_parser.rs` - Pawn language parser (using pest or similar)
- `pawn_generator.rs` - Pawn code generator from AST
- `ast_bridge.rs` - Bridge between Rust AST and TypeScript AST

**Parser Strategy**:

- Use `pest` crate for Pawn parsing (grammar-based)
- Parse to language-specific AST first
- Convert to language-agnostic AST for Blueprint system
- Handle includes, macros, and Pawn-specific features

### 4. Custom Node Editor UI (`src/components/blueprint/`)

**Purpose**: Visual node editor built with React and Canvas API.

**Key Files**:

- `blueprint-editor.tsx` - Main editor component
- `blueprint-canvas.tsx` - Canvas rendering with React Three Fiber or Canvas 2D
- `node-renderer.tsx` - Individual node rendering
- `connection-renderer.tsx` - Connection line rendering
- `node-palette.tsx` - Node library/sidebar
- `property-panel.tsx` - Node property editor

**UI Features**:

- Drag-and-drop nodes from palette
- Connect nodes via pins
- Pan/zoom canvas
- Multi-select and group operations
- Context menus for nodes
- Minimap for large graphs
- Search/filter nodes

### 5. Bidirectional Sync Engine (`src/utils/blueprint/sync/`)

**Purpose**: Keep code and Blueprint in sync.

**Key Files**:

- `sync-manager.ts` - Main sync orchestration
- `code-to-blueprint.ts` - Convert code changes to Blueprint updates
- `blueprint-to-code.ts` - Convert Blueprint changes to code updates
- `conflict-resolution.ts` - Handle merge conflicts

**Sync Strategy**:

- Watch code files for changes (via Tauri file watcher)
- Parse changed code → update Blueprint graph
- Watch Blueprint changes → regenerate code
- Use AST diffing to minimize code regeneration
- Preserve user formatting where possible
- Handle conflicts when both are edited simultaneously

### 6. Pawn Language Support (`src/utils/blueprint/languages/pawn/`)

**Purpose**: Pawn-specific features and nodes.

**Key Files**:

- `pawn-nodes.ts` - Pawn-specific node types (CreateDynamicObject, callbacks, etc.)
- `pawn-functions.ts` - SAMP function library definitions
- `pawn-types.ts` - Pawn type system (int, Float, String, etc.)
- `pawn-callbacks.ts` - SAMP callback definitions

**Pawn-Specific Features**:

- SAMP callbacks (OnPlayerConnect, OnGameModeInit, etc.)
- Native function calls (CreateDynamicObject, SetPlayerPos, etc.)
- Pawn-specific types (Float, String arrays, etc.)
- Tag system support
- Array handling

## Implementation Phases

### Phase 1: Foundation (Weeks 1-2)

1. Create AST core types and interfaces
2. Implement basic Blueprint graph data structure
3. Build custom node editor canvas (basic rendering)
4. Create node palette UI
5. Implement basic node types (variable, function, expression)

**Files to Create**:

- `src/utils/blueprint/ast/ast-core.ts`
- `src/utils/blueprint/graph/blueprint-graph.ts`
- `src/components/blueprint/blueprint-editor.tsx`
- `src/components/blueprint/blueprint-canvas.tsx`
- `src/components/blueprint/node-palette.tsx`

### Phase 2: Pawn Parser (Weeks 3-4)

1. Implement Pawn grammar using pest
2. Build Pawn AST parser in Rust
3. Create AST bridge (Rust ↔ TypeScript)
4. Implement basic code-to-AST conversion
5. Test with existing `.pwn` files

**Files to Create**:

- `src-tauri/src/blueprint/parser.rs`
- `src-tauri/src/blueprint/pawn_parser.rs` (with `pawn.pest` grammar)
- `src-tauri/src/blueprint/ast_bridge.rs`
- `src/utils/blueprint/parsers/pawn-parser.ts` (TypeScript wrapper)

### Phase 3: Code Generator (Weeks 5-6)

1. Implement AST-to-Pawn code generator
2. Handle formatting and style preservation
3. Generate proper Pawn syntax (indentation, brackets, etc.)
4. Test round-trip: code → AST → code

**Files to Create**:

- `src-tauri/src/blueprint/pawn_generator.rs`
- `src/utils/blueprint/generators/pawn-generator.ts`

### Phase 4: Visual Node System (Weeks 7-8)

1. Implement all Pawn node types
2. Build connection system (pins, wires, validation)
3. Add node property editors
4. Implement control flow nodes (if/else, loops, etc.)
5. Add SAMP-specific nodes (callbacks, natives)

**Files to Create**:

- `src/components/blueprint/nodes/pawn-nodes.tsx`
- `src/components/blueprint/nodes/control-flow-nodes.tsx`
- `src/components/blueprint/nodes/samp-nodes.tsx`
- `src/utils/blueprint/languages/pawn/pawn-nodes.ts`

### Phase 5: AST ↔ Blueprint Conversion (Weeks 9-10)

1. Implement AST-to-Blueprint converter
2. Implement Blueprint-to-AST converter
3. Handle complex structures (nested functions, closures)
4. Preserve node positions and layout

**Files to Create**:

- `src/utils/blueprint/converters/ast-to-blueprint.ts`
- `src/utils/blueprint/converters/blueprint-to-ast.ts`

### Phase 6: Bidirectional Sync (Weeks 11-12)

1. Implement file watcher for code changes
2. Implement Blueprint change detection
3. Build sync manager with conflict resolution
4. Add user preferences for sync behavior
5. Handle edge cases (deleted nodes, renamed functions)

**Files to Create**:

- `src/utils/blueprint/sync/sync-manager.ts`
- `src/utils/blueprint/sync/code-to-blueprint.ts`
- `src/utils/blueprint/sync/blueprint-to-code.ts`
- `src/utils/blueprint/sync/conflict-resolution.ts`

### Phase 7: Integration & Polish (Weeks 13-14)

1. Integrate Blueprint editor into main UI
2. Add Blueprint file management (create, open, save)
3. Add export to `.pwn` functionality
4. Performance optimization
5. Documentation and examples

**Files to Modify**:

- `src/components/rengine-editor.tsx` - Add Blueprint tab/mode
- `src/components/left-sidebar.tsx` - Add Blueprint files to hierarchy
- `src/stores/scene-store.ts` - Add Blueprint state management
- `src/utils/scene-persistence.ts` - Include Blueprints in scene files

## Technical Decisions

### Node Editor Rendering

- **Custom Canvas**: Use HTML5 Canvas with React hooks for rendering
- **Why**: Full control over rendering, better performance than DOM-based solutions
- **Alternative considered**: React Flow (rejected per user preference)

### AST Storage Format

- **JSON-based**: Store AST as JSON for serialization
- **Why**: Easy to serialize, debug, and version control
- **Format**: Use JSON Schema for validation

### Code Formatting

- **Prettier-like**: Use AST-based formatter to preserve user style
- **Why**: Maintains readability while allowing code generation

### Conflict Resolution

- **Three-way merge**: Compare code, Blueprint, and last sync point
- **User choice**: Prompt user when conflicts detected
- **Fallback**: Prefer code when in doubt (code is source of truth)

## File Structure

```
src/
├── components/
│   └── blueprint/
│       ├── blueprint-editor.tsx          # Main editor component
│       ├── blueprint-canvas.tsx          # Canvas rendering
│       ├── node-palette.tsx              # Node library sidebar
│       ├── property-panel.tsx             # Node properties
│       ├── nodes/
│       │   ├── pawn-nodes.tsx            # Pawn-specific nodes
│       │   ├── control-flow-nodes.tsx    # If/else, loops, etc.
│       │   └── samp-nodes.tsx             # SAMP callbacks/natives
│       └── ...
├── utils/
│   └── blueprint/
│       ├── ast/
│       │   ├── ast-core.ts               # Core AST types
│       │   ├── ast-pawn.ts               # Pawn AST
│       │   └── ast-visitor.ts            # Visitor pattern
│       ├── graph/
│       │   ├── blueprint-graph.ts        # Graph data structure
│       │   ├── node-types.ts             # Node definitions
│       │   └── connection-system.ts       # Pin/connection logic
│       ├── languages/
│       │   └── pawn/
│       │       ├── pawn-nodes.ts         # Pawn node definitions
│       │       ├── pawn-functions.ts     # SAMP function library
│       │       └── pawn-types.ts         # Pawn type system
│       ├── converters/
│       │   ├── ast-to-blueprint.ts       # AST → Blueprint
│       │   └── blueprint-to-ast.ts       # Blueprint → AST
│       ├── sync/
│       │   ├── sync-manager.ts           # Sync orchestration
│       │   ├── code-to-blueprint.ts      # Code → Blueprint sync
│       │   └── blueprint-to-code.ts      # Blueprint → Code sync
│       └── parsers/
│           └── pawn-parser.ts            # TypeScript parser wrapper

src-tauri/src/
└── blueprint/
    ├── parser.rs                         # Parser trait
    ├── pawn_parser.rs                   # Pawn parser implementation
    ├── pawn_generator.rs                # Pawn code generator
    ├── ast_bridge.rs                    # Rust ↔ TS bridge
    └── grammar/
        └── pawn.pest                    # Pawn grammar definition
```

## Key Challenges & Solutions

### Challenge 1: Preserving Code Comments

**Solution**: Store comments as metadata in AST, regenerate in same positions

### Challenge 2: Handling Pawn Macros

**Solution**: Expand macros during parsing, store original macro usage in metadata

### Challenge 3: Performance with Large Files

**Solution**: Lazy parsing, incremental updates, virtual scrolling in node editor

### Challenge 4: Round-trip Fidelity

**Solution**: Comprehensive test suite, AST diffing to detect changes

### Challenge 5: User Experience

**Solution**: Clear visual indicators for sync status, undo/redo, conflict resolution UI

## Success Metrics

1. **Functionality**: Can create/edit Pawn scripts visually
2. **Sync**: Bidirectional sync works reliably
3. **Performance**: Handles files with 1000+ lines smoothly
4. **Fidelity**: Round-trip code → Blueprint → code preserves functionality
5. **Usability**: Users can create basic scripts without code knowledge

## Future Extensions

- TypeScript/TSX support
- Rust support
- C++ support
- Node templates and snippets
- Blueprint debugging
- Visual breakpoints
- Code completion in node editor
- Multi-file Blueprint projects