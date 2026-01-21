#!/usr/bin/env python3
"""
Build verification script for Rengine
Verifies build prerequisites and setup without performing the actual build
"""

import sys
import os
from pathlib import Path
import subprocess


def check_python_version():
    """Check Python version compatibility"""
    version = sys.version_info
    if version.major == 3 and version.minor >= 8:
        print("✓ Python version:", f"{version.major}.{version.minor}.{version.micro}")
        return True
    else:
        print(f"✗ Python version {version.major}.{version.minor}.{version.micro} is too old. Need Python 3.8+")
        return False


def check_dependencies():
    """Check if required dependencies are installed"""
    required_modules = [
        'PyQt6',
        'PyQt6.Qt3DCore',
        'psutil',
        'darkdetect',
        'nuitka'
    ]

    missing = []
    for module in required_modules:
        try:
            __import__(module)
            print(f"✓ {module} is installed")
        except ImportError:
            missing.append(module)
            print(f"✗ {module} is missing")

    return len(missing) == 0


def check_nuitka():
    """Check if Nuitka is properly installed and accessible"""
    try:
        result = subprocess.run([sys.executable, "-m", "nuitka", "--version"],
                              capture_output=True, text=True, check=True)
        version = result.stdout.strip().split()[-1]
        print(f"✓ Nuitka version: {version}")
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("✗ Nuitka is not installed or not accessible")
        return False


def check_required_files():
    """Check if all required files for building exist"""
    project_root = Path(__file__).parent
    required_files = [
        "application/main.py",
        "requirements.txt",
        "icon.ico",
        "application/common/versionsets.json",
        "application/tools/IDE_Editor/schema.json"
    ]

    all_exist = True
    for file_path in required_files:
        full_path = project_root / file_path
        if full_path.exists():
            print(f"✓ {file_path} exists")
        else:
            print(f"✗ {file_path} missing")
            all_exist = False

    return all_exist


def check_build_script():
    """Check if build.py is valid and executable"""
    build_script = Path(__file__).parent / "build.py"

    if not build_script.exists():
        print("✗ build.py not found")
        return False

    try:
        # Try to compile the build script to check for syntax errors
        with open(build_script, 'r') as f:
            code = f.read()
        compile(code, str(build_script), 'exec')
        print("✓ build.py syntax is valid")
        return True
    except SyntaxError as e:
        print(f"✗ build.py has syntax error: {e}")
        return False


def check_qt_platform():
    """Check Qt platform plugins"""
    try:
        from PyQt6.QtGui import QGuiApplication
        app = QGuiApplication.instance()
        if app is None:
            app = QGuiApplication([])

        # Try to get platform name
        platform = app.platformName()
        print(f"✓ Qt platform: {platform}")

        app.quit()
        return True
    except Exception as e:
        print(f"✗ Qt platform check failed: {e}")
        return False


def main():
    """Main verification function"""
    print("Rengine Build Verification")
    print("=" * 50)

    checks = [
        ("Python Version", check_python_version),
        ("Required Files", check_required_files),
        ("Build Script", check_build_script),
        ("Dependencies", check_dependencies),
        ("Nuitka", check_nuitka),
        ("Qt Platform", check_qt_platform),
    ]

    passed = 0
    total = len(checks)

    for check_name, check_func in checks:
        print(f"\n{check_name}:")
        if check_func():
            passed += 1

    print(f"\n{'=' * 50}")
    print(f"Build Verification Results: {passed}/{total} checks passed")

    if passed == total:
        print("✓ All build prerequisites are satisfied!")
        print("\nTo build the application, run:")
        print("  python3 build.py")
        print("\nThis will create a standalone executable using Nuitka.")
        return True
    else:
        print("✗ Some build prerequisites are not satisfied.")
        print("Please install missing dependencies and ensure all required files exist.")
        return False


if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)