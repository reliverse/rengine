#!/bin/bash

# Ensure the script is executable (self-healing)
chmod +x "$0" 2>/dev/null || true

# Build the godot-rw extension from workspace root
cd ../../..
cargo build -p rengine-godot-rw

# Copy the library to the Godot project directory
cp target/debug/librengine_godot_rw.so examples/godot/rw/

echo "✅ RW Extension built and library copied to Godot RW project!"
echo "You can now open the Godot RW project at examples/godot/rw/"