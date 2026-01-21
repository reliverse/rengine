# Rengine - Ubuntu OS Setup Guide

## Problem: Qt Platform Plugin Issue

When running the Rengine on Ubuntu OS, you may encounter this error:

```
qt.qpa.plugin: From 6.5.0, xcb-cursor0 or libxcb-cursor0 is needed to load the Qt xcb platform plugin.
qt.qpa.plugin: Could not load the Qt platform plugin "xcb" in "" even though it was found.
This application failed to start because no Qt platform plugin could be initialized.
```

## Solution: Install Missing Dependencies

Run these commands in your terminal to install the required Qt dependencies:

```bash
sudo apt update
sudo apt install libxcb-cursor0 qt6-base-dev libxcb-xinerama0 libxcb-icccm4 libxcb-image0 libxcb-keysyms1 libxcb-randr0 libxcb-render-util0 libxcb-shape0
```

## ✅ Current Status: WORKING!

Great news! The Rengine is **working correctly** on your Ubuntu OS system. The application starts successfully with the XCB platform, initializes properly, and shows the full UI. The "crash" you experienced was actually the auto-launcher script killing the application after 10 seconds to try different platforms.

## Available Launcher Scripts

### 1. **Standard Launcher** (`run_app.sh`) - **RECOMMENDED**
- Uses XCB platform (works on your system)
- Includes proper environment variables
- **Use this for normal operation**

### 2. **Auto Launcher** (`run_app_auto.sh`)
- Automatically detects working platform
- Will find XCB and keep the application running
- Good for troubleshooting or if you switch display servers

### 3. **Wayland Launcher** (`run_app_wayland.sh`)
- For Wayland desktop environments
- May not work on your current X11 setup

### 4. **Test Script** (`test_startup.sh`)
- Verifies the application starts correctly
- Runs for 15 seconds to confirm stability

## 🚀 How to Run (Dependencies Already Installed)

The application is working! Use these commands:

### **Primary Usage - Just Run It:**
```bash
cd /home/blefnk/B/R/reliverse/rengine/temp/app
./run_app.sh
```

### **Test It First (Optional):**
```bash
./test_startup.sh
```

### **Auto-Detection (If you want to be sure):**
```bash
./run_app_auto.sh
```

### **Check System Status:**
```bash
./diagnose_system.sh
```

## 🎯 What You Should See When It Works

When the application starts successfully, you should see:

```
[INFO] Starting Rengine
[DEBUG] Qt environment variables configured
[DEBUG] QApplication created
[INFO] Dark theme applied successfully
[INFO] Window configured (size: 1536x769, breakpoint: large)
[INFO] Application initialization completed
⚡ PERF: Application Initialization 93.63ms
```

Then the Rengine window will appear with:
- **File Explorer** (left panel)
- **Tools Panel** (right panel)
- **Content Area** (center, tabbed interface)
- **Status Bar** (bottom, with memory monitoring)

Available tools include:
- IMG Editor (GTA archive management)
- TXD Editor (texture viewing)
- DFF Viewer (3D model viewer)
- RW Analyze (file analysis)
- IDE Editor (item definitions)

## Troubleshooting

If you still have issues:

1. **Check your display server:**
   ```bash
   echo $XDG_SESSION_TYPE
   echo $DISPLAY
   ```

2. **Verify Qt installation:**
   ```bash
   apt list --installed | grep qt6
   ```

3. **Check for missing libraries:**
   ```bash
   ldconfig -p | grep xcb
   ```

4. **Try running with verbose output:**
   ```bash
   QT_DEBUG_PLUGINS=1 ./run_app.sh
   ```

## Desktop Environment Notes

- **Ubuntu OS (GNOME):** Usually uses Wayland by default, but can fall back to X11
- **XFCE/KDE:** May use X11 by default
- **Check your session type:** Look at the login screen or run `echo $XDG_SESSION_TYPE`

## Alternative: Virtual Display (if needed)

If you're running in an environment without a display, you can try:

```bash
# Install virtual framebuffer
sudo apt install xvfb

# Run with virtual display
xvfb-run -a ./run_app.sh
```

But this is not recommended for GUI applications as you'll need additional setup for interaction.