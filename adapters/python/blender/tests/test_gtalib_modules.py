"""
Test suite for gtaLib modules (dff, txd, col, map)
Tests basic module file structure and existence without requiring Blender
"""

from pathlib import Path

# Get addon directory path
addon_dir = Path(__file__).parent.parent


def test_dff_module_exists():
    """Test that dff module file exists and has basic structure"""
    dff_file = addon_dir / "gtaLib" / "dff.py"
    assert dff_file.exists(), "dff.py not found"

    content = dff_file.read_text()
    assert "class dff" in content, "dff class not found in dff.py"
    assert "def load_memory" in content, "load_memory method not found in dff.py"


def test_txd_module_exists():
    """Test that txd module file exists and has basic structure"""
    txd_file = addon_dir / "gtaLib" / "txd.py"
    assert txd_file.exists(), "txd.py not found"

    content = txd_file.read_text()
    assert "class txd" in content, "txd class not found in txd.py"
    assert "def load_memory" in content, "load_memory method not found in txd.py"


def test_col_module_exists():
    """Test that col module file exists and has basic structure"""
    col_file = addon_dir / "gtaLib" / "col.py"
    assert col_file.exists(), "col.py not found"

    content = col_file.read_text()
    assert "class coll" in content, "coll class not found in col.py"
    assert "def load_memory" in content, "load_memory method not found in col.py"


def test_map_module_exists():
    """Test that map module file exists"""
    map_file = addon_dir / "gtaLib" / "map.py"
    assert map_file.exists(), "map.py not found"

    content = map_file.read_text()
    assert len(content) > 100, "map.py appears to be empty"


def test_img_module_exists():
    """Test that img module file exists and has basic structure"""
    img_file = addon_dir / "gtaLib" / "img.py"
    assert img_file.exists(), "img.py not found"

    content = img_file.read_text()
    assert "class img" in content, "img class not found in img.py"


def test_dff_structure():
    """Test basic dff class structure in file"""
    dff_file = addon_dir / "gtaLib" / "dff.py"
    content = dff_file.read_text()

    assert "def load_memory" in content, "load_memory method not found in dff.py"
    assert "def to_mem" in content, "to_mem method not found in dff.py"


def test_txd_structure():
    """Test basic txd class structure in file"""
    txd_file = addon_dir / "gtaLib" / "txd.py"
    content = txd_file.read_text()

    assert "def load_memory" in content, "load_memory method not found in txd.py"
    assert "def to_mem" in content, "to_mem method not found in txd.py"


def test_col_file_structure():
    """Test basic coll class structure in file"""
    col_file = addon_dir / "gtaLib" / "col.py"
    content = col_file.read_text()

    assert "def load_memory" in content, "load_memory method not found in col.py"


def test_native_modules_exist():
    """Test that native platform modules exist"""
    native_modules = ['native_gc', 'native_ps2', 'native_psp', 'native_wdgl', 'native_xbox']

    for module_name in native_modules:
        module_path = addon_dir / "gtaLib" / f"{module_name}.py"
        assert module_path.exists(), f"Native module {module_name}.py not found"


def test_pyffi_utilities_exist():
    """Test that pyffi utilities files exist"""
    tristrip_path = addon_dir / "gtaLib" / "pyffi" / "utils" / "tristrip.py"
    assert tristrip_path.exists(), "tristrip.py not found"

    trianglemesh_path = addon_dir / "gtaLib" / "pyffi" / "utils" / "trianglemesh.py"
    assert trianglemesh_path.exists(), "trianglemesh.py not found"


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
        test_dff_module_exists,
        test_txd_module_exists,
        test_col_module_exists,
        test_map_module_exists,
        test_img_module_exists,
        test_dff_structure,
        test_txd_structure,
        test_col_file_structure,
        test_native_modules_exist,
        test_pyffi_utilities_exist,
    ]

    passed = 0
    total = len(test_functions)

    print("Running gtaLib modules tests...")
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
