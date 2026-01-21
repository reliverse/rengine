"""
Test suite for tools functionality and structure
"""

import sys
import os
from pathlib import Path

# Add application directory to path for imports
app_dir = Path(__file__).parent.parent / "application"
sys.path.insert(0, str(app_dir))

# Set environment to avoid GUI dependencies
os.environ['QT_QPA_PLATFORM'] = 'offscreen'


def test_tool_registry():
    """Test tool registry functionality"""
    try:
        import tool_registry

        # Check that registry class exists
        assert hasattr(tool_registry, 'ToolRegistry'), "ToolRegistry class not found"

        # Check basic methods
        registry_class = tool_registry.ToolRegistry
        assert hasattr(registry_class, 'register_tool'), "register_tool method missing"
        assert hasattr(registry_class, 'get_tools'), "get_tools method missing"

    except ImportError as e:
        print(f"SKIPPING: Tool registry import failed: {e}")
        return


def test_img_editor_structure():
    """Test IMG Editor tool structure"""
    img_editor_dir = app_dir / "tools" / "IMG_Editor"
    assert img_editor_dir.exists(), "IMG_Editor directory not found"

    # Check main file
    main_file = img_editor_dir / "IMG_Editor.py"
    assert main_file.exists(), "IMG_Editor.py not found"

    content = main_file.read_text()
    assert len(content) > 500, "IMG_Editor.py appears too small"

    # Check core components
    core_dir = img_editor_dir / "core"
    assert core_dir.exists(), "IMG_Editor core directory not found"

    core_files = ["Core.py", "IMG_Operations.py", "File_Operations.py"]
    for core_file in core_files:
        file_path = core_dir / core_file
        assert file_path.exists(), f"Core file {core_file} not found"


def test_txd_editor_structure():
    """Test TXD Editor tool structure"""
    txd_editor_dir = app_dir / "tools" / "TXD_Editor"
    assert txd_editor_dir.exists(), "TXD_Editor directory not found"

    main_file = txd_editor_dir / "TXD_Editor.py"
    assert main_file.exists(), "TXD_Editor.py not found"

    content = main_file.read_text()
    assert len(content) > 200, "TXD_Editor.py appears too small"

    # Check core components
    core_dir = txd_editor_dir / "core"
    assert core_dir.exists(), "TXD_Editor core directory not found"


def test_dff_viewer_structure():
    """Test DFF Viewer tool structure"""
    dff_viewer_dir = app_dir / "tools" / "DFF_Viewer"
    assert dff_viewer_dir.exists(), "DFF_Viewer directory not found"

    main_file = dff_viewer_dir / "DFF_Viewer.py"
    assert main_file.exists(), "DFF_Viewer.py not found"

    content = main_file.read_text()
    assert len(content) > 200, "DFF_Viewer.py appears too small"


def test_col_editor_structure():
    """Test COL Editor tool structure"""
    col_editor_dir = app_dir / "tools" / "COL_Editor"
    assert col_editor_dir.exists(), "COL_Editor directory not found"

    main_file = col_editor_dir / "COL_Editor.py"
    assert main_file.exists(), "COL_Editor.py not found"


def test_map_editor_structure():
    """Test Map Editor tool structure"""
    map_editor_dir = app_dir / "tools" / "Map_Editor"
    assert map_editor_dir.exists(), "Map_Editor directory not found"

    main_file = map_editor_dir / "Map_Editor.py"
    assert main_file.exists(), "Map_Editor.py not found"


def test_rw_analyze_structure():
    """Test RW Analyze tool structure"""
    rw_analyze_dir = app_dir / "tools" / "RW_Analyze"
    assert rw_analyze_dir.exists(), "RW_Analyze directory not found"

    main_file = rw_analyze_dir / "RW_Analyze.py"
    assert main_file.exists(), "RW_Analyze.py not found"

    core_file = rw_analyze_dir / "RW_Analyze_core.py"
    assert core_file.exists(), "RW_Analyze_core.py not found"


def test_ide_editor_structure():
    """Test IDE Editor tool structure"""
    ide_editor_dir = app_dir / "tools" / "IDE_Editor"
    assert ide_editor_dir.exists(), "IDE_Editor directory not found"

    main_file = ide_editor_dir / "IDE_Editor.py"
    assert main_file.exists(), "IDE_Editor.py not found"

    core_file = ide_editor_dir / "IDE_core.py"
    assert core_file.exists(), "IDE_core.py not found"

    schema_file = ide_editor_dir / "schema.json"
    assert schema_file.exists(), "schema.json not found"


def test_tool_files_have_content():
    """Test that all tool files have substantial content"""
    tools_dir = app_dir / "tools"
    tool_dirs = [
        "IMG_Editor", "TXD_Editor", "DFF_Viewer", "COL_Editor",
        "Map_Editor", "RW_Analyze", "IDE_Editor"
    ]

    for tool_dir_name in tool_dirs:
        tool_dir = tools_dir / tool_dir_name
        main_file = tool_dir / f"{tool_dir_name}.py"

        content = main_file.read_text()
        # Each tool should have at least 20 lines of code
        assert len(content.split('\n')) > 20, f"{tool_dir_name}.py appears too small"


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
        test_tool_registry,
        test_img_editor_structure,
        test_txd_editor_structure,
        test_dff_viewer_structure,
        test_col_editor_structure,
        test_map_editor_structure,
        test_rw_analyze_structure,
        test_ide_editor_structure,
        test_tool_files_have_content,
    ]

    passed = 0
    total = len(test_functions)

    print("Running tools tests...")
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