#!/bin/bash

# Rengine Launcher for Linux
# This script runs the application with the required Python environment

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if Python 3 is available
if ! command -v python3 &> /dev/null; then
    echo "Error: Python 3 is not installed or not in PATH"
    exit 1
fi

# Check if required Python packages are installed
python3 -c "import PyQt6, psutil, colorama, darkdetect" 2>/dev/null
if [ $? -ne 0 ]; then
    echo "Error: Required Python packages are not installed."
    echo "Please run: pip install -r requirements.txt"
    exit 1
fi

# Change to the script directory
cd "$SCRIPT_DIR"


# Set Qt platform environment variables to help with display issues
export QT_QPA_PLATFORM=xcb
export QT_DEBUG_PLUGINS=1
export DISPLAY=:0

# Run the application
echo "Starting Rengine..."
python3 -m application.main

echo "Rengine closed."