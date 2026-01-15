# Pawn Language Support

## Overview

The Blueprint system has full support for the Pawn scripting language, including SAMP (San Andreas Multiplayer) callbacks and native functions. This allows you to create SAMP gamemodes and filterscripts visually.

## Pawn Type System

### Supported Types

- **int**: Integer numbers
- **Float**: Floating-point numbers (tagged type)
- **bool**: Boolean values (true/false)
- **String**: String arrays
- **void**: No return value

### Type Tags

Pawn supports type tags for better type safety:

```pawn
new Float:health = 100.0;
new playerid = 0;
```

The Blueprint system preserves these tags when generating code.

## SAMP Callbacks

SAMP callbacks are special functions that the server calls at specific events. The Blueprint system includes nodes for all common callbacks.

### Available Callback Nodes

#### Game Mode Callbacks
- **OnGameModeInit**: Called when the gamemode starts
- **OnGameModeExit**: Called when the gamemode ends

#### Player Callbacks
- **OnPlayerConnect**: Player connects to server
- **OnPlayerDisconnect**: Player disconnects
- **OnPlayerSpawn**: Player spawns
- **OnPlayerDeath**: Player dies
- **OnPlayerText**: Player sends chat message
- **OnPlayerCommandText**: Player types a command

#### Vehicle Callbacks
- **OnPlayerEnterVehicle**: Player enters a vehicle
- **OnPlayerExitVehicle**: Player exits a vehicle

### Using Callback Nodes

1. Drag a callback node from the palette
2. Connect execution pins to define the callback body
3. Use parameter inputs to access callback data
4. Return values are handled automatically

### Example: OnPlayerConnect

```
[OnPlayerConnect]
    ↓
[SendClientMessage: "Welcome to the server"]
    ↓
[SetPlayerPos: spawn location]
```

## SAMP Native Functions

Native functions are built-in SAMP functions that perform specific operations.

### Categories

#### Object Functions
- **CreateDynamicObject**: Creates a dynamic object
- **DestroyDynamicObject**: Destroys an object
- **SetDynamicObjectPos**: Sets object position

#### Player Functions
- **SetPlayerPos**: Sets player position
- **GetPlayerPos**: Gets player position
- **SetPlayerHealth**: Sets player health
- **GetPlayerHealth**: Gets player health
- **GivePlayerMoney**: Gives money to player
- **GetPlayerMoney**: Gets player's money

#### Vehicle Functions
- **CreateVehicle**: Creates a vehicle
- **DestroyVehicle**: Destroys a vehicle

#### Chat Functions
- **SendClientMessage**: Sends message to player
- **SendClientMessageToAll**: Sends message to all players

#### Utility Functions
- **printf**: Prints to server console
- **format**: Formats a string

### Using Native Nodes

1. Search for the native in the node palette
2. Drag it onto the canvas
3. Connect input pins for parameters
4. Use output pins for return values

### Example: Creating an Object

```
[OnGameModeInit]
    ↓
[CreateDynamicObject]
    Inputs:
    - modelid: 1337
    - x: 0.0
    - y: 0.0
    - z: 3.0
    Output:
    - objectid → [Store in variable]
```

## Pawn-Specific Features

### Arrays

Pawn arrays are supported through the type system:

```pawn
new playerNames[MAX_PLAYERS][MAX_PLAYER_NAME];
```

In Blueprint, arrays are represented with array size in the type definition.

### Tags

Type tags are preserved:

- `Float:` tag for floating-point numbers
- `String:` tag for strings
- Custom tags are supported

### Macros

Macros are expanded during parsing and stored in metadata. The original macro usage is preserved when possible.

### Includes

Include statements are preserved:

```pawn
#include <a_samp>
#include <streamer>
```

## Code Generation

When exporting a Blueprint to Pawn code:

1. Includes are generated first
2. Global variables are declared
3. Callbacks are generated with proper signatures
4. Functions are generated with modifiers (public, stock, etc.)
5. Code formatting is preserved where possible

### Generated Code Example

```pawn
#include <a_samp>

new g_PlayerCount = 0;

public OnPlayerConnect(playerid)
{
    g_PlayerCount++;
    SendClientMessage(playerid, -1, "Welcome!");
    return 1;
}

public OnPlayerDisconnect(playerid, reason)
{
    g_PlayerCount--;
    return 1;
}
```

## Importing Existing Code

You can import existing `.pwn` files:

1. Click "Open" in the Blueprint editor
2. Select a `.pwn` file
3. The system parses the code and creates a Blueprint graph
4. Node positions are auto-laid out

### Import Limitations

- Complex macros may not parse correctly
- Some advanced Pawn features may require manual adjustment
- Comments are preserved in metadata

## Best Practices

1. **Use Appropriate Types**: Use `Float:` for coordinates, `int` for IDs
2. **Handle Return Values**: Check return values from natives
3. **Use Callbacks Properly**: Return 1 or 0 as appropriate
4. **Organize Code**: Use groups and comments to organize large scripts
5. **Test Incrementally**: Test callbacks and functions as you build

## Troubleshooting

### Type Mismatch Errors
- Ensure pin types match (int to int, Float to Float)
- Use type conversion nodes if needed

### Missing Natives
- Check if the native is in the SAMP_NATIVES list
- Add custom natives to `pawn-functions.ts` if needed

### Parse Errors
- Check for syntax errors in imported code
- Verify includes are correct
- Ensure proper callback signatures
