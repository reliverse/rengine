"""
Test suite for Blender addon structure and metadata
Tests addon manifest, module structure, and basic imports
"""

from pathlib import Path

# Use addon_dir without modifying sys.path to avoid importing bpy-dependent modules
addon_dir = Path(__file__).parent.parent


def test_addon_directory_structure():
    """Test that required directories and files exist"""
    required_files = [
        "__init__.py",
        "blender_manifest.toml",
        "README.md",
        "LICENSE"
    ]
    
    required_dirs = [
        "gui",
        "gtaLib",
        "ops"
    ]
    
    for file in required_files:
        assert (addon_dir / file).exists(), f"Required file {file} not found"
    
    for dir in required_dirs:
        assert (addon_dir / dir).exists(), f"Required directory {dir} not found"
        assert (addon_dir / dir).is_dir(), f"{dir} is not a directory"


def test_manifest_file():
    """Test that blender_manifest.toml is valid"""
    manifest_path = addon_dir / "blender_manifest.toml"
    assert manifest_path.exists(), "blender_manifest.toml not found"
    
    # Check that file is not empty
    content = manifest_path.read_text()
    assert len(content) > 0, "blender_manifest.toml is empty"
    
    # Basic TOML validation - should contain expected fields
    assert "name" in content or "id" in content, "Manifest missing name/id field"


def test_init_module_structure():
    """Test __init__.py has required Blender addon components"""
    init_path = addon_dir / "__init__.py"
    content = init_path.read_text()
    
    # Check for bl_info
    assert "bl_info" in content, "__init__.py missing bl_info dictionary"
    
    # Check for required functions
    assert "def register()" in content, "__init__.py missing register() function"
    assert "def unregister()" in content, "__init__.py missing unregister() function"


def test_gtalib_package():
    """Test that gtaLib is a proper Python package"""
    gtalib_dir = addon_dir / "gtaLib"
    
    # Check __init__.py exists
    init_file = gtalib_dir / "__init__.py"
    assert init_file.exists(), "gtaLib/__init__.py not found"
    
    # Check core modules exist
    core_modules = ["dff.py", "txd.py", "col.py", "map.py", "img.py"]
    for module in core_modules:
        module_path = gtalib_dir / module
        assert module_path.exists(), f"gtaLib/{module} not found"


def test_gui_package():
    """Test that gui package exists and has gui.py"""
    gui_dir = addon_dir / "gui"
    assert gui_dir.exists(), "gui directory not found"
    
    # Check for gui.py
    gui_file = gui_dir / "gui.py"
    assert gui_file.exists(), "gui/gui.py not found"


def test_ops_package():
    """Test that ops package exists"""
    ops_dir = addon_dir / "ops"
    assert ops_dir.exists(), "ops directory not found"


def test_init_file_structure():
    """Test that __init__.py has required Blender addon components"""
    init_path = addon_dir / "__init__.py"
    content = init_path.read_text()

    # Check for required components without importing
    assert "import bpy" in content, "__init__.py should import bpy"
    assert "bl_info" in content, "__init__.py should define bl_info"


def test_bl_info_structure():
    """Test that bl_info has required fields"""
    # Import the __init__ module to check bl_info
    init_path = addon_dir / "__init__.py"
    content = init_path.read_text()
    
    # Extract bl_info (simple text search)
    assert '"name"' in content or "'name'" in content, "bl_info missing 'name' field"
    assert '"author"' in content or "'author'" in content, "bl_info missing 'author' field"
    assert '"version"' in content or "'version'" in content, "bl_info missing 'version' field"
    assert '"blender"' in content or "'blender'" in content, "bl_info missing 'blender' field"
    assert '"category"' in content or "'category'" in content, "bl_info missing 'category' field"


def test_license_file():
    """Test that LICENSE file exists and is not empty"""
    license_path = addon_dir / "LICENSE"
    assert license_path.exists(), "LICENSE file not found"
    
    content = license_path.read_text()
    assert len(content) > 100, "LICENSE file appears to be empty or too short"
    assert "GNU" in content or "GPL" in content, "LICENSE doesn't appear to be GPL"


def test_readme_file():
    """Test that README.md exists and has content"""
    readme_path = addon_dir / "README.md"
    assert readme_path.exists(), "README.md not found"
    
    content = readme_path.read_text()
    assert len(content) > 100, "README.md appears to be empty or too short"
    assert "DragonFF" in content, "README.md doesn't mention DragonFF"


if __name__ == "__main__":
    # Simple test runner that doesn't require pytest for CI/testing
    import sys

    def run_test(test_func):
        try:
            test_func()
            print(f"✓ {test_func.__name__}")
            return True
        except Exception as e:
            print(f"✗ {test_func.__name__}: {e}")
            return False

    # Run all test functions
    test_functions = [
        test_addon_directory_structure,
        test_manifest_file,
        test_init_module_structure,
        test_gtalib_package,
        test_gui_package,
        test_ops_package,
        test_init_file_structure,
        test_bl_info_structure,
        test_license_file,
        test_readme_file,
    ]

    passed = 0
    total = len(test_functions)

    print("Running Blender addon structure tests...")
    for test_func in test_functions:
        if run_test(test_func):
            passed += 1

    print(f"\nResults: {passed}/{total} tests passed")

    if passed == total:
        print("All tests passed! ✓")
        sys.exit(0)
    else:
        print("Some tests failed! ✗")
        sys.exit(1)
