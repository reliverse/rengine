"""
Tests for base classes and utilities
"""

import unittest
import tempfile
import os
from pathlib import Path
from unittest.mock import Mock, patch

from rengine_cli.base import (
    BaseCommand, BaseFileHandler, OutputFormatter,
    RWChunkAnalyzer, DependencyManager, CLIError, FileOperationError
)


class ConcreteCommand(BaseCommand):
    """Concrete implementation for testing"""

    def get_name(self):
        return "test-command"

    def get_description(self):
        return "Test command"

    def execute(self, args):
        return 0


class TestBaseCommand(unittest.TestCase):
    """Test BaseCommand class"""

    def setUp(self):
        self.logger = Mock()
        self.command = ConcreteCommand(self.logger)

    def test_concrete_methods(self):
        """Test that concrete methods work"""
        self.assertEqual(self.command.get_name(), "test-command")
        self.assertEqual(self.command.get_description(), "Test command")
        self.assertEqual(self.command.execute(None), 0)

    def test_validate_args_default(self):
        """Test default validate_args implementation"""
        self.assertTrue(self.command.validate_args(None))

    def test_handle_error(self):
        """Test error handling"""
        args = Mock()
        args.verbose = False

        exit_code = self.command.handle_error(ValueError("test error"), args)
        self.assertEqual(exit_code, 1)
        self.logger.error.assert_called_once()

        # Test with verbose
        self.logger.reset_mock()
        args.verbose = True

        with patch('traceback.print_exc') as mock_traceback:
            exit_code = self.command.handle_error(ValueError("test error"), args)
            self.assertEqual(exit_code, 1)
            mock_traceback.assert_called_once()


class ConcreteFileHandler(BaseFileHandler):
    """Concrete implementation for testing"""

    def get_supported_extensions(self):
        return ['.test']

    def can_handle(self, file_path: str):
        return file_path.endswith('.test')


class TestBaseFileHandler(unittest.TestCase):
    """Test BaseFileHandler class"""

    def setUp(self):
        self.logger = Mock()
        self.handler = ConcreteFileHandler(self.logger)

    def test_concrete_methods(self):
        """Test that concrete methods work"""
        self.assertEqual(self.handler.get_supported_extensions(), ['.test'])
        self.assertTrue(self.handler.can_handle("file.test"))
        self.assertFalse(self.handler.can_handle("file.txt"))

    def test_file_operations(self):
        """Test file operation methods"""
        # Test with temporary file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.test', delete=False) as f:
            f.write("test content")
            temp_path = f.name

        try:
            # Test get_file_info
            info = self.handler.get_file_info(temp_path)
            self.assertEqual(info['name'], os.path.basename(temp_path))
            self.assertEqual(info['extension'], '.test')
            self.assertTrue('size' in info)
            self.assertTrue('modified' in info)

            # Test read_file_binary
            data = self.handler.read_file_binary(temp_path)
            self.assertEqual(data, b"test content")

            # Test read_file_text
            text_data = self.handler.read_file_text(temp_path)
            self.assertEqual(text_data, "test content")

        finally:
            os.unlink(temp_path)

    def test_file_operation_errors(self):
        """Test file operation error handling"""
        with self.assertRaises(FileOperationError):
            self.handler.read_file_binary("nonexistent_file.txt")

        with self.assertRaises(FileOperationError):
            self.handler.get_file_info("nonexistent_file.txt")


class TestOutputFormatter(unittest.TestCase):
    """Test OutputFormatter class"""

    def test_format_text(self):
        """Test text formatting"""
        data = {'name': 'test', 'value': 42}
        result = OutputFormatter.format_text(data)
        self.assertIn('name: test', result)
        self.assertIn('value: 42', result)

    def test_format_json(self):
        """Test JSON formatting"""
        import json
        data = {'name': 'test', 'value': 42}
        result = OutputFormatter.format_json(data)
        parsed = json.loads(result)
        self.assertEqual(parsed, data)

    def test_output_data(self):
        """Test output_data method"""
        data = {'test': 'data'}

        # Test text output
        with patch('builtins.print') as mock_print:
            OutputFormatter.output_data(data, Mock(), 'text')
            mock_print.assert_called_once()
            self.assertIn('test: data', mock_print.call_args[0][0])

        # Test JSON output
        with patch('builtins.print') as mock_print:
            args = Mock()
            args.format = 'json'
            OutputFormatter.output_data(data, args, 'text')
            mock_print.assert_called_once()


class TestRWChunkAnalyzer(unittest.TestCase):
    """Test RWChunkAnalyzer class"""

    def test_chunk_names(self):
        """Test chunk name lookup"""
        self.assertEqual(RWChunkAnalyzer.get_chunk_name(0x00000010), "Clump")
        self.assertEqual(RWChunkAnalyzer.get_chunk_name(0x00000016), "TextureDictionary")
        self.assertEqual(RWChunkAnalyzer.get_chunk_name(0x99999999), "Unknown (0x99999999)")

    def test_version_names(self):
        """Test version name lookup"""
        self.assertEqual(RWChunkAnalyzer.get_version_name(0x36003), "3.6.0.3 (SA Advanced)")
        self.assertEqual(RWChunkAnalyzer.get_version_name(0x34003), "3.4.0.3 (SA)")
        self.assertEqual(RWChunkAnalyzer.get_version_name(0x99999999), "0x99999999")

    def test_count_chunks(self):
        """Test chunk counting"""
        # Create mock RW data (simplified)
        import struct
        data = struct.pack('<III', 0x00000010, 20, 0x36003)  # Clump chunk
        data += b'x' * 20  # Chunk data

        count = RWChunkAnalyzer.count_chunks(data)
        self.assertEqual(count, 1)

    def test_analyze_chunks(self):
        """Test chunk analysis"""
        import struct
        data = struct.pack('<III', 0x00000010, 20, 0x36003)  # Clump chunk
        data += b'x' * 20  # Chunk data

        chunks = RWChunkAnalyzer.analyze_chunks(data)
        self.assertEqual(len(chunks), 1)
        self.assertEqual(chunks[0]['type_name'], 'Clump')
        self.assertEqual(chunks[0]['version_name'], '3.6.0.3 (SA Advanced)')


class TestDependencyManager(unittest.TestCase):
    """Test DependencyManager class"""

    def setUp(self):
        self.deps = DependencyManager()

    def test_dependency_checks(self):
        """Test dependency availability checks"""
        # These will depend on actual environment setup
        qt_available = self.deps.qt_app_available
        blender_img_available = self.deps.blender_img_available
        blender_col_available = self.deps.blender_col_available
        blender_map_available = self.deps.blender_map_available
        blender_available = self.deps.blender_available

        # At least some should be boolean values
        self.assertIsInstance(qt_available, bool)
        self.assertIsInstance(blender_img_available, bool)
        self.assertIsInstance(blender_col_available, bool)
        self.assertIsInstance(blender_map_available, bool)
        self.assertIsInstance(blender_available, bool)

    def test_dependency_requirements(self):
        """Test dependency requirement methods"""
        # These should not raise exceptions in normal operation
        # but we can't easily test the raise cases without mocking
        try:
            self.deps.require_blender()  # May or may not raise depending on setup
        except Exception:
            pass  # Expected if dependencies not available


if __name__ == '__main__':
    unittest.main()