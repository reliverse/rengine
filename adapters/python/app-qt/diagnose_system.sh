#!/bin/bash

# Rengine - System Diagnostics
# This script checks if your system is ready to run the application

echo "=== Rengine - System Diagnostics ==="
echo ""

# Check Python
echo "1. Checking Python..."
if command -v python3 &> /dev/null; then
    PYTHON_VERSION=$(python3 --version)
    echo "   ✓ Python found: $PYTHON_VERSION"
else
    echo "   ✗ Python 3 not found!"
    exit 1
fi
echo ""

# Check required Python packages
echo "2. Checking Python packages..."
PACKAGES=("PyQt6" "psutil" "colorama" "darkdetect")
for package in "${PACKAGES[@]}"; do
    if python3 -c "import $package" 2>/dev/null; then
        echo "   ✓ $package installed"
    else
        echo "   ✗ $package missing - run: pip install -r requirements.txt"
    fi
done
echo ""

# Check display environment
echo "3. Checking display environment..."
if [ -n "$DISPLAY" ]; then
    echo "   ✓ DISPLAY variable set: $DISPLAY"
else
    echo "   ⚠ DISPLAY variable not set"
fi

if [ -n "$XDG_SESSION_TYPE" ]; then
    echo "   ✓ Session type: $XDG_SESSION_TYPE"
else
    echo "   ⚠ XDG_SESSION_TYPE not set"
fi

# Check if running in graphical environment
if [ -n "$DISPLAY" ] || [ "$XDG_SESSION_TYPE" = "wayland" ]; then
    echo "   ✓ Graphical environment detected"
else
    echo "   ⚠ No graphical environment detected"
fi
echo ""

# Check Qt platform plugins
echo "4. Checking Qt platform plugins..."
QT_PLUGINS_DIR="$HOME/.local/lib/python3.12/site-packages/PyQt6/Qt6/plugins/platforms"
if [ -d "$QT_PLUGINS_DIR" ]; then
    echo "   ✓ Qt plugins directory found"
    echo "   Available platforms:"
    ls -1 "$QT_PLUGINS_DIR"/*.so 2>/dev/null | sed 's/.*libq//' | sed 's/\.so//' | while read plugin; do
        echo "     - $plugin"
    done
else
    echo "   ✗ Qt plugins directory not found"
fi
echo ""

# Check system libraries
echo "5. Checking system libraries..."
LIBRARIES=("libxcb-cursor" "libxcb-xinerama" "libxcb-icccm" "libxcb-image" "libxcb-keysyms")
for lib in "${LIBRARIES[@]}"; do
    if ldconfig -p | grep -q "$lib"; then
        echo "   ✓ $lib found"
    else
        echo "   ✗ $lib missing - run: sudo apt install $lib"0
    fi
done
echo ""

# Summary and recommendations
echo "6. Summary and Recommendations:"
echo ""

MISSING_DEPS=0
if ! ldconfig -p | grep -q "libxcb-cursor"; then
    echo "   ⚠ Install missing dependencies:"
    echo "     sudo apt install libxcb-cursor0 qt6-base-dev libxcb-xinerama0 libxcb-icccm4 libxcb-image0 libxcb-keysyms1"
    MISSING_DEPS=1
fi

if [ -z "$DISPLAY" ] && [ "$XDG_SESSION_TYPE" != "wayland" ]; then
    echo "   ⚠ No display environment detected"
    echo "     Make sure you're running in a graphical desktop environment"
    MISSING_DEPS=1
fi

if [ $MISSING_DEPS -eq 0 ]; then
    echo "   ✓ System appears ready!"
    echo "   Try running: ./run_app_auto.sh"
else
    echo "   ✗ System needs setup - see README_ZORIN_SETUP.md"
fi

echo ""
echo "=== Diagnostics Complete ==="