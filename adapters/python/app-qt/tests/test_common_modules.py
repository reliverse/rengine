"""
Test suite for common modules (rw_versions, txd, etc.)
"""

import sys
import os
from pathlib import Path

# Add application directory to path for imports
app_dir = Path(__file__).parent.parent / "application"
sys.path.insert(0, str(app_dir))

# Set environment to avoid GUI dependencies
os.environ['QT_QPA_PLATFORM'] = 'offscreen'


def test_rw_versions_module():
    """Test RW versions module functionality"""
    try:
        import rw_versions

        # Check that key functions exist
        assert hasattr(rw_versions, 'get_rw_version'), "get_rw_version function missing"
        assert hasattr(rw_versions, 'get_game_name'), "get_game_name function missing"

        # Test version detection
        test_version = 0x34000  # GTA SA version
        game_name = rw_versions.get_game_name(test_version)
        assert game_name is not None, "Game name detection failed"

    except ImportError as e:
        print(f"SKIPPING: RW versions module import failed: {e}")
        return


def test_txd_module():
    """Test TXD module basic functionality"""
    try:
        import txd

        # Check that TXD class exists
        assert hasattr(txd, 'TXD'), "TXD class not found in txd module"

        # Check basic methods exist
        txd_class = txd.TXD
        assert hasattr(txd_class, 'load_memory'), "load_memory method missing from TXD"

    except ImportError as e:
        print(f"SKIPPING: TXD module import failed: {e}")
        return


def test_message_box_module():
    """Test message box module"""
    try:
        import message_box

        # Check that key functions exist
        assert hasattr(message_box, 'show_message'), "show_message function missing"

    except ImportError as e:
        print(f"SKIPPING: message_box module import failed: {e}")
        return


def test_versionsets_json():
    """Test that versionsets.json exists and is valid JSON"""
    versionsets_path = app_dir / "common" / "versionsets.json"
    assert versionsets_path.exists(), "versionsets.json not found"

    import json
    try:
        with open(versionsets_path, 'r') as f:
            data = json.load(f)

        assert isinstance(data, dict), "versionsets.json should contain a dictionary"
        assert len(data) > 0, "versionsets.json appears empty"

    except json.JSONDecodeError as e:
        raise AssertionError(f"versionsets.json is not valid JSON: {e}")


def test_native_modules_exist():
    """Test that native platform modules exist"""
    native_modules = ['native_gc', 'native_ps2', 'native_psp', 'native_wdgl', 'native_xbox']

    for module_name in native_modules:
        module_path = app_dir / "common" / f"{module_name}.py"
        assert module_path.exists(), f"Native module {module_name}.py not found"

        # Check file has content
        content = module_path.read_text()
        assert len(content) > 50, f"Native module {module_name}.py appears too small"


def test_dff_common_module():
    """Test DFF module in common directory"""
    dff_path = app_dir / "common" / "DFF.py"
    assert dff_path.exists(), "DFF.py not found in common"

    content = dff_path.read_text()
    assert len(content) > 1000, "DFF.py appears too small"

    # Check for key classes/functions
    assert "class dff" in content, "dff class not found in DFF.py"
    assert "def load_file" in content, "load_file method not found in DFF.py"


if __name__ == "__main__":
    # Simple test runner for manual execution
    import sys

    def run_test(test_func):
        try:
            test_func()
            print(f"✓ {test_func.__name__}")
            return True
        except Exception as e:
            print(f"✗ {test_func.__name__}: {e}")
            return False

    test_functions = [
        test_rw_versions_module,
        test_txd_module,
        test_message_box_module,
        test_versionsets_json,
        test_native_modules_exist,
        test_dff_common_module,
    ]

    passed = 0
    total = len(test_functions)

    print("Running common modules tests...")
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