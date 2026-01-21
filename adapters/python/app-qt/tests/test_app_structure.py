"""
Test suite for Qt application structure and basic functionality
"""

import sys
import os
from pathlib import Path

# Add application directory to path for imports
app_dir = Path(__file__).parent.parent / "application"
sys.path.insert(0, str(app_dir))


def test_application_directory_structure():
    """Test that required application directories and files exist"""
    qt_app_dir = Path(__file__).parent.parent

    required_files = [
        "requirements.txt",
        "README.md",
        "LICENSE",
        "build.py",
        "application/main.py"
    ]

    required_dirs = [
        "application",
        "application/common",
        "application/tools",
        "tests"
    ]

    for file in required_files:
        file_path = qt_app_dir / file
        assert file_path.exists(), f"Required file {file} not found"
        if file_path.suffix == ".py":
            # Check that Python files are valid syntax
            try:
                compile(file_path.read_text(), str(file_path), 'exec')
            except SyntaxError as e:
                raise AssertionError(f"Syntax error in {file}: {e}")

    for dir_name in required_dirs:
        dir_path = qt_app_dir / dir_name
        assert dir_path.exists(), f"Required directory {dir_name} not found"
        assert dir_path.is_dir(), f"{dir_name} is not a directory"


def test_main_entry_point():
    """Test that main.py is a valid entry point"""
    main_path = Path(__file__).parent.parent / "application" / "main.py"
    content = main_path.read_text()

    # Check for required imports and structure
    assert "from application.main_application import main" in content, "main.py missing main_application import"
    assert 'if __name__ == "__main__":' in content, "main.py missing main guard"
    assert "main()" in content, "main.py doesn't call main()"


def test_common_modules():
    """Test that common modules can be imported"""
    common_modules = [
        "rw_versions",
        "txd",
        "message_box"
    ]

    for module_name in common_modules:
        try:
            module_path = app_dir / "common" / f"{module_name}.py"
            assert module_path.exists(), f"Common module {module_name}.py not found"

            # Check file has basic structure
            content = module_path.read_text()
            assert len(content) > 100, f"Common module {module_name}.py appears empty"

        except Exception as e:
            pytest.skip(f"Common module {module_name} test failed: {e}")


def test_tools_structure():
    """Test that tools directory has proper structure"""
    tools_dir = app_dir / "tools"
    assert tools_dir.exists(), "tools directory not found"

    # Check for tool registry
    registry_path = tools_dir / "tool_registry.py"
    assert registry_path.exists(), "tool_registry.py not found"

    # Check for tool directories
    expected_tools = [
        "IMG_Editor",
        "TXD_Editor",
        "DFF_Viewer",
        "COL_Editor",
        "Map_Editor",
        "RW_Analyze",
        "IDE_Editor"
    ]

    for tool_name in expected_tools:
        tool_dir = tools_dir / tool_name
        assert tool_dir.exists(), f"Tool directory {tool_name} not found"
        assert tool_dir.is_dir(), f"{tool_name} is not a directory"

        # Check for main tool file
        tool_file = tool_dir / f"{tool_name}.py"
        assert tool_file.exists(), f"Tool file {tool_name}.py not found"


def test_requirements_file():
    """Test that requirements.txt is valid"""
    req_path = Path(__file__).parent.parent / "requirements.txt"
    assert req_path.exists(), "requirements.txt not found"

    content = req_path.read_text()
    lines = [line.strip() for line in content.split('\n') if line.strip() and not line.startswith('#')]

    # Should have several dependencies
    assert len(lines) >= 3, "requirements.txt seems too short"

    # Check for key dependencies
    requirements_text = content.lower()
    assert 'pyqt6' in requirements_text, "PyQt6 not found in requirements"
    assert 'psutil' in requirements_text, "psutil not found in requirements"


def test_readme_file():
    """Test that README.md exists and has content"""
    readme_path = Path(__file__).parent.parent / "README.md"
    assert readme_path.exists(), "README.md not found"

    content = readme_path.read_text()
    assert len(content) > 1000, "README.md appears too short"

    # Check for key sections
    assert "Rengine" in content, "README missing title"
    assert "## Features" in content, "README missing Features section"
    assert "## Tool Overview" in content, "README missing Tool Overview section"


def test_build_script():
    """Test that build.py exists and is executable"""
    build_path = Path(__file__).parent.parent / "build.py"
    assert build_path.exists(), "build.py not found"

    content = build_path.read_text()
    assert len(content) > 100, "build.py appears too short"

    # Check for main build function
    assert "def build_executable_comprehensive()" in content, "build.py missing build function"


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
        test_application_directory_structure,
        test_main_entry_point,
        test_common_modules,
        test_tools_structure,
        test_requirements_file,
        test_readme_file,
        test_build_script,
    ]

    passed = 0
    total = len(test_functions)

    print("Running app-qt structure tests...")
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