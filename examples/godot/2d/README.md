# Rengine Godot Edition - 2D Example

This is a basic 2D example demonstrating how to use Rengine as a Godot extension.

## Setup

1. Make sure you have Godot 4.2+ installed (godot-rust v0.4 requires this)
2. The extension is automatically configured via `rengine.gdextension`
3. Build the Rust extension and copy library: `cd ../../../adapters/godot && ./build_and_copy.sh`

   Or manually:
   - `cd ../../../adapters/godot && cargo build`
   - `cp ../../../adapters/godot/target/debug/librengine_godot.so .`

## Testing the Extension

The project is now configured to run `player_test.tscn` by default, which demonstrates the Player class.

1. Open this project in Godot Editor
2. Run the project (F5) - you'll see a rotating/moving player sprite with instructions
3. Or manually: Change any Sprite2D node type to "Player" in the scene tree
4. The Player rotates and moves in a circle automatically

## Features Demonstrated

- **Player Class**: Inherits from Sprite2D with custom behavior
- **Automatic Movement**: Rotates and moves in a circle using physics_process
- **Custom API**: `increase_speed()` method to modify movement speed
- **Signals**: `speed_increased` signal emitted when speed changes
- **godot-rust v0.4**: Uses latest API with improved type safety and ergonomics

## Usage in GDScript

```gdscript
# Get reference to Player node
var player = $Player

# Call custom method
player.increase_speed(100.0)

# Connect to signal
player.speed_increased.connect(func(): print("Speed increased!"))
```

## Development

To modify the extension:

1. Edit `/adapters/godot/src/lib.rs`
2. Run `cargo build` in the rust directory
3. Restart Godot Editor (or it should hot-reload in Godot 4.2+)

## Troubleshooting

- **Extension not loading**: Check that `librengine_godot.so` exists in the project root directory
- **Player class not appearing**: Verify the .gdextension file paths are correct and run `./build_and_copy.sh`
- **Build errors**: Ensure you're using compatible godot-rust and Godot versions
- **Library path issues**: Use the `build_and_copy.sh` script instead of manual copying