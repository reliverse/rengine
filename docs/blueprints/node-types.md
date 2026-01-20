# Blueprint Node Types

## Overview

Blueprint nodes are the building blocks of visual scripts. Each node represents a specific operation, value, or control flow construct. Nodes have input and output pins that can be connected to create the program flow.

## Node Structure

Every node consists of:

- **Title**: Display name of the node
- **Type**: Category of the node (function, variable, control flow, etc.)
- **Input Pins**: Data or execution inputs
- **Output Pins**: Data or execution outputs
- **Properties**: Configurable parameters
- **Position**: Visual position on the canvas

## Node Categories

### Flow Control Nodes

Control the execution flow of your script.

#### If Node
- **Inputs**: Condition (bool), Exec (execution)
- **Outputs**: True (execution), False (execution)
- **Description**: Executes different code paths based on a condition

#### While Loop Node
- **Inputs**: Condition (bool), Exec (execution)
- **Outputs**: Loop Body (execution), Completed (execution)
- **Description**: Repeats code while condition is true

#### For Loop Node
- **Inputs**: Initial Value (int), Condition (bool), Increment (int), Exec (execution)
- **Outputs**: Loop Body (execution), Index (int), Completed (execution)
- **Description**: Iterates a specific number of times

### Variable Nodes

Work with variables and data.

#### Variable Getter
- **Inputs**: None
- **Outputs**: Value (any type)
- **Description**: Gets the value of a variable

#### Variable Setter
- **Inputs**: Exec (execution), Value (any type)
- **Outputs**: Exec (execution)
- **Description**: Sets the value of a variable

#### Literal Node
- **Inputs**: None
- **Outputs**: Value (int, Float, String, bool)
- **Description**: Represents a constant value

### Operator Nodes

Perform mathematical and logical operations.

#### Binary Operators
- **Types**: Add, Subtract, Multiply, Divide, Modulo
- **Inputs**: A (number), B (number)
- **Outputs**: Result (number)

#### Comparison Operators
- **Types**: Equal, Not Equal, Less Than, Greater Than, Less Equal, Greater Equal
- **Inputs**: A (any), B (any)
- **Outputs**: Result (bool)

#### Logical Operators
- **Types**: AND, OR
- **Inputs**: A (bool), B (bool)
- **Outputs**: Result (bool)

### Function Nodes

#### Function Call Node
- **Inputs**: Exec (execution), Parameters (various types)
- **Outputs**: Exec (execution), Return Value (any type)
- **Description**: Calls a user-defined function

#### Return Node
- **Inputs**: Exec (execution), Value (any type)
- **Outputs**: None
- **Description**: Returns a value from a function

### Pawn-Specific Nodes

#### SAMP Callback Nodes
- **Types**: OnPlayerConnect, OnGameModeInit, OnPlayerSpawn, etc.
- **Inputs**: Exec (execution), Callback Parameters
- **Outputs**: Exec (execution), Return Value (int)
- **Description**: Represents SAMP server callbacks

#### SAMP Native Nodes
- **Types**: CreateDynamicObject, SetPlayerPos, SendClientMessage, etc.
- **Inputs**: Exec (execution), Function Parameters
- **Outputs**: Exec (execution), Return Value (varies)
- **Description**: Calls SAMP native functions

### Special Nodes

#### Comment Node
- **Inputs**: None
- **Outputs**: None
- **Description**: Adds documentation to your Blueprint

#### Group Node
- **Inputs**: None
- **Outputs**: None
- **Description**: Organizes nodes visually

## Pin Types

### Execution Pins
- **Type**: `exec`
- **Color**: Yellow/Orange
- **Purpose**: Control program flow
- **Rules**: Must connect exec-to-exec

### Data Pins
- **Types**: `int`, `Float`, `String`, `bool`, `any`
- **Color**: Blue
- **Purpose**: Pass data between nodes
- **Rules**: Types must be compatible

## Node Properties

Each node can have custom properties:

- **Variable Name**: For variable nodes
- **Function Name**: For function nodes
- **Operator**: For operator nodes
- **Value**: For literal nodes
- **Description**: Documentation text

## Creating Custom Nodes

While the system comes with many built-in nodes, you can extend it by:

1. Creating node definitions in `src/utils/blueprint/languages/pawn/pawn-nodes.ts`
2. Adding node renderers in `src/components/blueprint/nodes/`
3. Registering nodes in the node palette

## Best Practices

1. **Use Descriptive Names**: Name variables and functions clearly
2. **Organize with Groups**: Use comment nodes to document sections
3. **Keep Graphs Readable**: Avoid crossing connections when possible
4. **Use Appropriate Types**: Match pin types correctly
5. **Test Incrementally**: Build and test small sections at a time

## Examples

### Simple Variable Assignment

```
[Literal: 100] → [Variable Setter: health]
```

### Conditional Logic

```
[Variable Getter: playerHealth] → [Comparison: <] → [Literal: 50]
                                    ↓
                            [If Node]
                            ├─ True → [SendClientMessage: "Low Health"]
                            └─ False → [Continue]
```

### Loop Example

```
[For Loop: i = 0 to 10]
    ↓
[CreateDynamicObject: modelid, x, y, z]
    ↓
[Continue Loop]
```
