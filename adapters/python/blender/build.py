#!/usr/bin/env python3
"""
Build script for DragonFF Blender addon
Verifies the addon can be packaged and installed
"""

import zipfile
import os
from pathlib import Path
import json


def build_addon_zip():
    """Build the addon as a zip file for manual installation"""
    addon_dir = Path(__file__).parent
    zip_path = addon_dir / "dragonff.zip"

    # Files to include in the zip
    include_patterns = [
        "*.py",      # Python files
        "*.toml",    # Manifest
        "*.md",      # Documentation
        "LICENSE",   # License
    ]

    # Directories to include
    include_dirs = ["gtaLib", "gui", "ops"]

    print("Building DragonFF addon zip...")

    with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
        # Add manifest
        manifest_path = addon_dir / "blender_manifest.toml"
        if manifest_path.exists():
            zipf.write(manifest_path, "blender_manifest.toml")
            print("✓ Added blender_manifest.toml")

        # Add main __init__.py
        init_path = addon_dir / "__init__.py"
        if init_path.exists():
            zipf.write(init_path, "__init__.py")
            print("✓ Added __init__.py")

        # Add directories
        for dir_name in include_dirs:
            dir_path = addon_dir / dir_name
            if dir_path.exists():
                for file_path in dir_path.rglob("*.py"):
                    rel_path = file_path.relative_to(addon_dir)
                    zipf.write(file_path, str(rel_path))
                print(f"✓ Added {dir_name}/ directory")

        # Add other files
        for pattern in include_patterns:
            if "*" in pattern:
                for file_path in addon_dir.glob(pattern):
                    if file_path.is_file() and file_path.name != "build.py":
                        rel_path = file_path.relative_to(addon_dir)
                        zipf.write(file_path, str(rel_path))
                        print(f"✓ Added {rel_path}")
            else:
                file_path = addon_dir / pattern
                if file_path.exists():
                    zipf.write(file_path, pattern)
                    print(f"✓ Added {pattern}")

    print(f"✓ Build complete: {zip_path}")
    print(f"  Zip size: {zip_path.stat().st_size} bytes")
    return zip_path


def verify_manifest():
    """Verify the blender_manifest.toml is valid"""
    manifest_path = Path(__file__).parent / "blender_manifest.toml"

    if not manifest_path.exists():
        print("✗ blender_manifest.toml not found")
        return False

    try:
        content = manifest_path.read_text()
        # Basic checks
        required_fields = ["id", "version", "name", "type"]
        for field in required_fields:
            if f'{field} = ' not in content:
                print(f"✗ Required field '{field}' missing from manifest")
                return False

        print("✓ blender_manifest.toml is valid")
        return True
    except Exception as e:
        print(f"✗ Error reading manifest: {e}")
        return False


def verify_addon_structure():
    """Verify the addon has required structure"""
    addon_dir = Path(__file__).parent

    required_files = ["__init__.py", "blender_manifest.toml"]
    required_dirs = ["gtaLib", "gui", "ops"]

    all_good = True

    for file in required_files:
        if not (addon_dir / file).exists():
            print(f"✗ Required file {file} missing")
            all_good = False
        else:
            print(f"✓ Found {file}")

    for dir_name in required_dirs:
        if not (addon_dir / dir_name).exists():
            print(f"✗ Required directory {dir_name} missing")
            all_good = False
        else:
            print(f"✓ Found {dir_name}/ directory")

    return all_good


def main():
    """Main build verification function"""
    print("DragonFF Blender Addon Build Verification")
    print("=" * 50)

    # Verify structure
    if not verify_addon_structure():
        print("\n✗ Addon structure verification failed")
        return False

    # Verify manifest
    if not verify_manifest():
        print("\n✗ Manifest verification failed")
        return False

    # Build zip
    try:
        zip_path = build_addon_zip()
        print("\n✓ Addon build successful!")
        print(f"  Package: {zip_path}")
        print("\nInstallation instructions:")
        print("1. Open Blender")
        print("2. Go to Edit > Preferences > Add-ons")
        print("3. Click 'Install from File'")
        print(f"4. Select {zip_path}")
        print("5. Enable 'GTA DragonFF' addon")
        return True
    except Exception as e:
        print(f"\n✗ Build failed: {e}")
        return False


if __name__ == "__main__":
    success = main()
    exit(0 if success else 1)