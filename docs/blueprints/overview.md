# Blueprint Visual Scripting System

## Overview

The Blueprint Visual Scripting System is a UE5-like node-based visual programming interface that allows users to create code visually by connecting nodes together. The system is programming language agnostic, with full Pawn support as the first implementation.

## Key Features

- **Visual Node Editor**: Drag-and-drop interface for creating code visually
- **Language Agnostic**: Core system supports multiple programming languages
- **Bidirectional Sync**: Seamless synchronization between visual Blueprints and code files
- **Pawn Language Support**: Full support for Pawn scripting language with SAMP callbacks and natives
- **Real-time Preview**: See your code structure as you build visually
- **Export/Import**: Convert between `.pwn` files and Blueprint graphs

## Architecture

The Blueprint system is built on an AST (Abstract Syntax Tree) foundation:

```
Code Files (.pwn, .ts, .rs)
    ↕ (Parser/Generator)
AST (Abstract Syntax Tree)
    ↕ (Converters)
Blueprint Graph (Visual Nodes)
    ↕ (UI)
User Interface
```

### Core Components

1. **AST Foundation Layer**: Language-agnostic code representation
2. **Blueprint Node Graph**: Visual representation of code structure
3. **Code Parser & Generator**: Convert between code and AST (Rust-based)
4. **Custom Node Editor UI**: React-based visual editor
5. **Bidirectional Sync Engine**: Keep code and Blueprints in sync
6. **Language Support**: Pawn-specific features and nodes

## Use Cases

- **Visual Scripting**: Create scripts without writing code
- **Code Visualization**: Understand existing code structure visually
- **Rapid Prototyping**: Quickly prototype game logic
- **Learning Tool**: Learn programming concepts visually
- **Code Refactoring**: Refactor code by manipulating visual nodes

## Getting Started

1. Open the Blueprint editor by clicking the "Blueprint" tab in the main editor
2. Create a new Blueprint or open an existing `.pwn` file
3. Drag nodes from the palette onto the canvas
4. Connect nodes by dragging from output pins to input pins
5. Export your Blueprint to `.pwn` code when ready

## File Formats

- **`.blueprint`**: Blueprint graph file (JSON format)
- **`.pwn`**: Pawn script file (text format)
- **`.json`**: Alternative Blueprint format

## Next Steps

- [Node Types and Usage](./node-types.md)
- [Pawn Language Support](./pawn-language-support.md)
- [Bidirectional Sync](./bidirectional-sync.md)
- [UI Features and Controls](./ui-features.md)
- [Advanced Topics](./advanced-topics.md)
