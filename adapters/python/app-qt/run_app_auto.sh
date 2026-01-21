#!/bin/bash

# Rengine Launcher for Linux - Auto platform detection
# This script tries different Qt platform plugins automatically

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

# Function to try running with a specific platform
try_platform() {
    local platform=$1
    echo "Trying Qt platform: $platform"

    export QT_QPA_PLATFORM=$platform
    export QT_DEBUG_PLUGINS=0  # Disable verbose debug output
    export DISPLAY=:0

    # Add platform-specific environment variables
    case $platform in
        "xcb")
            export XDG_SESSION_TYPE=x11
            ;;
        "wayland")
            export XDG_SESSION_TYPE=wayland
            ;;
        "eglfs")
            export QT_QPA_EGLFS_INTEGRATION=eglfs_kms
            ;;
    esac

    # Try to run the application with a short timeout to check if it starts
    # Capture output and check for success indicators
    local output
    local start_time=$(date +%s)

    # Run with timeout but capture output to check for success
    output=$(timeout 10s python3 -m application.main 2>&1)
    local exit_code=$?

    # Check if the application started successfully by looking for success indicators
    if echo "$output" | grep -q "Application initialization completed"; then
        echo "✓ Application started successfully with $platform platform!"
        echo "✓ Initialization completed - application should be running"
        # If it started successfully, run it without timeout
        echo "Launching full application..."
        exec python3 -m application.main
        return 0
    else
        echo "✗ Application failed to initialize with $platform platform"
        echo "Output was:"
        echo "$output" | head -20
        return 1
    fi
}

# Try different platforms in order of preference
platforms=("xcb" "wayland" "eglfs" "offscreen")

for platform in "${platforms[@]}"; do
    echo "Attempting to start with $platform platform..."
    if try_platform $platform; then
        echo "Success with $platform platform!"
        exit 0
    else
        echo "Failed with $platform platform, trying next..."
    fi
done

echo "Error: Could not start Rengine with any platform plugin."
echo "Available platform plugins were: ${platforms[*]}"
echo ""
echo "Troubleshooting:"
echo "1. Make sure you have a display server running (X11 or Wayland)"
echo "2. Try installing missing Qt dependencies:"
echo "   sudo apt install libxcb-cursor0 qt6-base-dev libxcb-xinerama0"
echo "3. For Wayland, make sure you're running in a Wayland session"
echo "4. For X11, make sure DISPLAY environment variable is set correctly"
exit 1