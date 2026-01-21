#!/bin/bash

# Rengine - Startup Test
# Tests if the application can start successfully without auto-killing it

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

# Set Qt environment variables for XCB
export QT_QPA_PLATFORM=xcb
export QT_DEBUG_PLUGINS=0
export DISPLAY=:0
export XDG_SESSION_TYPE=x11

echo "Testing Rengine startup..."
echo "This will run for 15 seconds to verify it starts properly"
echo "Press Ctrl+C to stop early if it's working"
echo ""

# Run with timeout but longer to see if it stabilizes
timeout 15s python3 -m application.main 2>&1 | head -50

exit_code=$?
if [ $exit_code -eq 124 ]; then
    echo ""
    echo "✓ SUCCESS: Application started and ran for 15 seconds!"
    echo "✓ It was killed by timeout, not by a crash"
    echo ""
    echo "The application is working! You can now run it normally with:"
    echo "  ./run_app.sh"
    echo "  ./run_app_auto.sh (will detect and use the working platform)"
elif [ $exit_code -eq 0 ]; then
    echo ""
    echo "✓ Application started and exited cleanly"
    echo "This usually means it was closed by the user or ran successfully"
else
    echo ""
    echo "✗ Application failed with exit code $exit_code"
    echo "Check the output above for error messages"
fi