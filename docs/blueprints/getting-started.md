# Getting Started with Blueprints

## Quick Start

### 1. Open the Blueprint Editor

1. Launch Rengine
2. Click the **"Blueprint"** tab in the main editor
3. The Blueprint editor will open

### 2. Create Your First Blueprint

1. Click **"New"** button in the toolbar
2. Select **"Pawn"** as the language
3. An empty Blueprint canvas will appear

### 3. Add Your First Node

1. Open the **Node Palette** on the left
2. Find **"Literal"** in the Variables category
3. Drag it onto the canvas
4. Set the value to `42` in the property panel

### 4. Create a Simple Script

Let's create a simple SAMP callback:

1. Drag **"OnPlayerConnect"** from Events category
2. Drag **"SendClientMessage"** from Natives category
3. Connect the execution pins (yellow)
4. Set the message parameters:
   - `playerid`: Connect from OnPlayerConnect output
   - `color`: Use a Literal node with value `-1`
   - `message`: Use a Literal node with value `"Welcome!"`

### 5. Export to Code

1. Click **"Save"** to save the Blueprint
2. Click **"Export"** → **"Export to .pwn"**
3. Choose a location and save
4. Your Pawn code is ready!

## Basic Concepts

### Nodes

Nodes are the building blocks of Blueprints. Each node represents:
- A value (Literal, Variable)
- An operation (Add, Compare)
- Control flow (If, While, For)
- A function call (Native, Custom Function)

### Pins

Pins are connection points on nodes:

- **Input Pins** (left side): Receive data or execution
- **Output Pins** (right side): Send data or execution
- **Execution Pins** (yellow): Control program flow
- **Data Pins** (blue): Pass data values

### Connections

Connections link nodes together:
- Drag from output pin to input pin
- Execution connections control flow
- Data connections pass values

## Common Patterns

### Pattern 1: Variable Assignment

```
[Literal: 100] → [Variable Setter: health]
```

### Pattern 2: Conditional Logic

```
[Get Variable: health] → [Compare: <] → [Literal: 50]
                          ↓
                    [If Node]
                    ├─ True → [Action]
                    └─ False → [Continue]
```

### Pattern 3: Loop

```
[For Loop: i = 0 to 10]
    ↓
[Action with i]
    ↓
[Continue Loop]
```

### Pattern 4: Function Call

```
[Function: MyFunction]
    Inputs: param1, param2
    ↓
[Use Return Value]
```

## Step-by-Step Tutorial

### Tutorial: Creating a Welcome Message

1. **Add OnPlayerConnect Callback**
   - Drag "OnPlayerConnect" from Events
   - This is your entry point

2. **Add SendClientMessage**
   - Drag "SendClientMessage" from Natives → Chat
   - This will send a message

3. **Connect Execution**
   - Connect the "Exec" output of OnPlayerConnect
   - To the "Exec" input of SendClientMessage

4. **Set Parameters**
   - `playerid`: Connect from OnPlayerConnect's playerid output
   - `color`: Create a Literal node with value `-1`
   - `message`: Create a Literal node with value `"Welcome to the server!"`

5. **Test**
   - Export to `.pwn`
   - Compile and test in SAMP server

### Tutorial: Health System

1. **OnPlayerSpawn**
   - Set player health to 100

2. **OnPlayerDeath**
   - Send death message
   - Respawn after delay

3. **Health Check**
   - Monitor player health
   - Warn when low

## Tips for Beginners

1. **Start Simple**: Begin with basic nodes and connections
2. **Use Comments**: Add comment nodes to document your logic
3. **Organize**: Use groups and spacing to keep things readable
4. **Test Often**: Export and test frequently
5. **Learn by Example**: Look at existing Blueprints
6. **Use Search**: Search the node palette to find what you need
7. **Read Tooltips**: Hover over nodes for descriptions

## Next Steps

- Learn about [Node Types](./node-types.md)
- Explore [Pawn Language Support](./pawn-language-support.md)
- Understand [Bidirectional Sync](./bidirectional-sync.md)
- Master [UI Features](./ui-features.md)
- Dive into [Advanced Topics](./advanced-topics.md)

## Getting Help

- Check the documentation
- Look at example Blueprints
- Review error messages
- Check the console for warnings

## Common Mistakes

1. **Forgetting Execution Flow**: Connect exec pins for code to run
2. **Type Mismatches**: Ensure pin types match
3. **Missing Connections**: All required inputs must be connected
4. **Infinite Loops**: Be careful with loop conditions
5. **Not Saving**: Save your work frequently
