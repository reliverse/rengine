# Pawn Language Support

## Overview

The Blueprint system has full support for the Pawn programming language, which is commonly used for SAMP (San Andreas Multiplayer) server scripting. This includes support for SAMP callbacks, native functions, and Pawn-specific types.

## Pawn Type System

### Supported Types

#### Basic Types

- **int**: Integer values (e.g., `42`, `-10`)
- **Float**: Floating-point numbers (e.g., `3.14`, `-0.5`)
- **bool**: Boolean values (`true`, `false`)
- **String**: String arrays (e.g., `"Hello World"`)

#### Type Tags

Pawn supports type tags for additional type safety:

- **Float**: Tagged floating-point type
- **String**: Tagged string type
- **Custom Tags**: User-defined tags

### Type Conversion

The system handles implicit type conversions:

- **int → Float**: Automatic promotion
- **Float → int**: Truncation (with warning)
- **String → int**: Parsing (when possible)

## SAMP Callbacks

### Available Callbacks

#### Game Mode Callbacks

**OnGameModeInit**
- Called when the game mode initializes
- No parameters
- Returns: `int` (0 to continue, 1 to stop)

**OnGameModeExit**
- Called when the game mode exits
- No parameters
- Returns: `int`

#### Player Callbacks

**OnPlayerConnect**
- Called when a player connects to the server
- Parameters:
  - `playerid` (int): The ID of the player who connected
- Returns: `int`

**OnPlayerDisconnect**
- Called when a player disconnects
- Parameters:
  - `playerid` (int): The ID of the player
  - `reason` (int): Disconnection reason
- Returns: `int`

**OnPlayerSpawn**
- Called when a player spawns
- Parameters:
  - `playerid` (int): The ID of the player
- Returns: `int`

**OnPlayerDeath**
- Called when a player dies
- Parameters:
  - `playerid` (int): The ID of the player who died
  - `killerid` (int): The ID of the killer
  - `reason` (int): Death reason
- Returns: `int`

**OnPlayerCommandText**
- Called when a player types a command
- Parameters:
  - `playerid` (int): The ID of the player
  - `cmdtext` (String): The command text
- Returns: `int` (1 if command handled, 0 otherwise)

**OnPlayerText**
- Called when a player sends a chat message
- Parameters:
  - `playerid` (int): The ID of the player
  - `text` (String): The message text
- Returns: `int` (1 to show message, 0 to hide)

**OnPlayerEnterVehicle**
- Called when a player enters a vehicle
- Parameters:
  - `playerid` (int): The ID of the player
  - `vehicleid` (int): The ID of the vehicle
  - `ispassenger` (bool): Whether player is passenger
- Returns: `int`

**OnPlayerExitVehicle**
- Called when a player exits a vehicle
- Parameters:
  - `playerid` (int): The ID of the player
  - `vehicleid` (int): The ID of the vehicle
- Returns: `int`

### Using Callbacks in Blueprints

1. **Add Callback Node**: Search for callback name in node palette
2. **Connect Logic**: Connect nodes to the callback's Exec output
3. **Use Parameters**: Connect callback parameters to other nodes
4. **Return Values**: Set return value if callback requires it

**Example: Welcome Message**

```
OnPlayerConnect
  → Exec → SendClientMessage
    → playerid: playerid (from callback)
    → color: 0xFFFFFF (literal)
    → message: "Welcome!" (literal)
```

## SAMP Native Functions

### Function Categories

#### Object Functions

**CreateDynamicObject**
- Creates a dynamic object in the world
- Parameters:
  - `modelid` (int): Model ID
  - `x, y, z` (Float): Position
  - `rx, ry, rz` (Float, optional): Rotation
  - `worldid` (int, optional): Virtual world
  - `interiorid` (int, optional): Interior ID
  - `playerid` (int, optional): Player ID
  - `streamdistance` (Float, optional): Stream distance
  - `drawdistance` (Float, optional): Draw distance
- Returns: `int` (Object ID)

**DestroyDynamicObject**
- Destroys a dynamic object
- Parameters:
  - `objectid` (int): Object ID to destroy
- Returns: `int`

#### Player Functions

**SetPlayerPos**
- Sets a player's position
- Parameters:
  - `playerid` (int): Player ID
  - `x, y, z` (Float): Position coordinates
- Returns: `int`

**GetPlayerPos**
- Gets a player's position
- Parameters:
  - `playerid` (int): Player ID
  - `x, y, z` (Float, by reference): Position output
- Returns: `int`

**SetPlayerHealth**
- Sets a player's health
- Parameters:
  - `playerid` (int): Player ID
  - `health` (Float): Health value (0-100)
- Returns: `int`

**GetPlayerHealth**
- Gets a player's health
- Parameters:
  - `playerid` (int): Player ID
  - `health` (Float, by reference): Health output
- Returns: `int`

**GivePlayerMoney**
- Gives money to a player
- Parameters:
  - `playerid` (int): Player ID
  - `money` (int): Amount to give
- Returns: `int`

**GetPlayerMoney**
- Gets a player's money
- Parameters:
  - `playerid` (int): Player ID
- Returns: `int` (Money amount)

#### Vehicle Functions

**CreateVehicle**
- Creates a vehicle
- Parameters:
  - `vehicletype` (int): Vehicle model ID
  - `x, y, z` (Float): Position
  - `rotation` (Float, optional): Rotation angle
  - `color1, color2` (int, optional): Colors
- Returns: `int` (Vehicle ID)

**DestroyVehicle**
- Destroys a vehicle
- Parameters:
  - `vehicleid` (int): Vehicle ID
- Returns: `int`

#### Chat Functions

**SendClientMessage**
- Sends a message to a player
- Parameters:
  - `playerid` (int): Player ID
  - `color` (int): Message color
  - `message` (String): Message text
- Returns: `int`

**SendClientMessageToAll**
- Sends a message to all players
- Parameters:
  - `color` (int): Message color
  - `message` (String): Message text
- Returns: `int`

#### Utility Functions

**printf**
- Prints formatted text to server console
- Parameters:
  - `format` (String): Format string
- Returns: `int`

**format**
- Formats a string
- Parameters:
  - `output` (String, by reference): Output string
  - `len` (int): Maximum length
  - `format` (String): Format string
- Returns: `int`

### Using Native Functions

1. **Add Native Node**: Search for function name in node palette
2. **Set Parameters**: Configure required and optional parameters
3. **Connect Values**: Connect other nodes to parameter inputs
4. **Use Return Value**: Connect return value output to other nodes

**Example: Spawn Object**

```
OnGameModeInit
  → Exec → CreateDynamicObject
    → modelid: 1337 (literal)
    → x: 0.0 (literal)
    → y: 0.0 (literal)
    → z: 3.0 (literal)
    → Result → Store in variable
```

## Pawn-Specific Features

### Arrays

Pawn supports arrays with fixed sizes:

```pawn
new playerNames[MAX_PLAYERS][24];
new playerScores[MAX_PLAYERS];
```

In Blueprints:
- Use array type nodes
- Specify array size in properties
- Access elements with index nodes

### String Handling

Pawn uses character arrays for strings:

- Fixed-size arrays
- Null-terminated strings
- String functions (format, strcmp, etc.)

### Tags

Pawn supports type tags:

```pawn
new Float:playerX;
new String:playerName[24];
```

In Blueprints:
- Tags are preserved in type system
- Tagged types show in node properties
- Type checking respects tags

### Macros

Pawn macros are expanded during parsing:

- `#define` macros
- Conditional compilation
- Include files

The parser handles macro expansion and preserves original usage in metadata.

## Code Generation

### Generated Code Style

The code generator produces clean, readable Pawn code:

```pawn
public OnPlayerConnect(playerid)
{
    SendClientMessage(playerid, 0xFFFFFF, "Welcome to the server!");
    return 1;
}
```

### Formatting Options

- **Indentation**: Configurable (spaces or tabs)
- **Bracket Style**: K&R style (opening brace on same line)
- **Line Breaks**: Automatic line breaks for readability
- **Comments**: Preserved from original code

### Best Practices

1. **Use Descriptive Names**: Clear variable and function names
2. **Add Comments**: Document complex logic
3. **Organize Code**: Group related functions
4. **Handle Errors**: Check return values
5. **Optimize**: Avoid unnecessary operations

## Examples

### Example 1: Basic Welcome System

```
OnPlayerConnect
  → Exec → SendClientMessage
    → playerid: playerid
    → color: 0x00FF00
    → message: "Welcome to the server!"
```

Generated Code:
```pawn
public OnPlayerConnect(playerid)
{
    SendClientMessage(playerid, 0x00FF00, "Welcome to the server!");
    return 1;
}
```

### Example 2: Conditional Spawn

```
OnPlayerSpawn
  → Exec → If
    → Condition: GetPlayerLevel >= 10
    → True: SetPlayerPos (spawn location)
    → False: SendClientMessage ("Level too low")
```

### Example 3: Loop Through Players

```
OnGameModeInit
  → Exec → For Loop
    → Start: 0
    → End: MAX_PLAYERS
    → Body: ProcessPlayer(Index)
```

## Integration with SAMP

### Include Files

The system supports SAMP include files:

```pawn
#include <a_samp>
#include <streamer>
```

Include statements are preserved in generated code.

### Plugin Support

Works with SAMP plugins:
- Streamer (dynamic objects)
- MySQL (database)
- Other plugins via includes

### Server Configuration

Generated code is compatible with:
- Standard SAMP server
- OpenMP (Open Multi-Player)
- Other Pawn-based servers

## Limitations

### Current Limitations

- **Macro Expansion**: Some complex macros may not expand correctly
- **Preprocessor Directives**: Limited support for all directives
- **Inline Assembly**: Not supported
- **Function Pointers**: Limited support

### Workarounds

- Use code comments for complex macros
- Manually edit generated code for edge cases
- Report issues for future improvements

## Future Enhancements

- **More Native Functions**: Complete SAMP native library
- **Plugin Support**: Direct plugin function access
- **Advanced Types**: Better array and string handling
- **Code Optimization**: Automatic code optimization
