#!/bin/bash

# Build the godot-rust extension
cargo build

# Copy the library to the Godot project directory
cp target/debug/librengine_godot.so ../../../examples/godot/2d/

echo "✅ Extension built and library copied to Godot project!"
echo "You can now open the Godot project at ../../../examples/godot/2d/"