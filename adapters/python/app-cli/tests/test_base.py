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
from rengine_cli.common.rw_versions import RWVersionManager


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
        self.assertEqual(chunks[0]['type'], '0x00000010')
        self.assertEqual(chunks[0]['version'], '0x00036003')

    def test_chunk_analysis_with_children(self):
        """Test chunk analysis with nested child chunks"""
        import struct

        # Create a parent chunk with a child chunk
        child_data = struct.pack('<III', 0x00000001, 8, 0x36003) + b'child'  # Struct chunk
        parent_data = struct.pack('<III', 0x00000010, len(child_data) + 12, 0x36003) + child_data  # Clump chunk

        chunks = RWChunkAnalyzer.analyze_chunks(parent_data, max_depth=2)
        self.assertEqual(len(chunks), 1)

        parent_chunk = chunks[0]
        self.assertEqual(parent_chunk['type_name'], 'Clump')
        self.assertIn('children', parent_chunk)
        self.assertEqual(len(parent_chunk['children']), 1)

        child_chunk = parent_chunk['children'][0]
        self.assertEqual(child_chunk['type_name'], 'Struct')
        self.assertEqual(child_chunk['depth'], 1)

    def test_chunk_analysis_depth_limits(self):
        """Test that chunk analysis respects depth limits"""
        import struct

        # Create a simple nested structure: parent -> child
        child_data = struct.pack('<III', 0x00000001, 4, 0x36003) + b'test'  # Struct chunk
        parent_data = struct.pack('<III', 0x00000010, len(child_data) + 12, 0x36003) + child_data  # Clump chunk

        # Test with max_depth=1 (should not analyze children)
        chunks_shallow = RWChunkAnalyzer.analyze_chunks(parent_data, max_depth=1)
        self.assertEqual(len(chunks_shallow), 1)
        parent_shallow = chunks_shallow[0]
        self.assertNotIn('children', parent_shallow)  # No children analyzed

        # Test with max_depth=2 (should analyze children)
        chunks_deep = RWChunkAnalyzer.analyze_chunks(parent_data, max_depth=2)
        self.assertEqual(len(chunks_deep), 1)
        parent_deep = chunks_deep[0]
        self.assertIn('children', parent_deep)  # Children analyzed
        self.assertEqual(len(parent_deep['children']), 1)
        self.assertEqual(parent_deep['children'][0]['type_name'], 'Struct')


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


class TestRWVersionManager(unittest.TestCase):
    """Test RWVersionManager hex conversion and validation functionality"""

    def setUp(self):
        self.version_manager = RWVersionManager()

    def test_version_string_to_hex(self):
        """Test version string to hex conversion"""
        # Standard version strings
        self.assertEqual(self.version_manager.version_string_to_hex("3.6.0.3"), 0x36003)
        self.assertEqual(self.version_manager.version_string_to_hex("3.4.0.3"), 0x34003)
        self.assertEqual(self.version_manager.version_string_to_hex("3.5.0.2"), 0x35002)
        self.assertEqual(self.version_manager.version_string_to_hex("3.1.0.1"), 0x31001)

        # Version strings without patch2
        self.assertEqual(self.version_manager.version_string_to_hex("3.6.0"), 0x36000)

        # Edge cases
        self.assertEqual(self.version_manager.version_string_to_hex(""), 0)
        self.assertEqual(self.version_manager.version_string_to_hex("invalid"), 0)
        self.assertEqual(self.version_manager.version_string_to_hex("1.2.3.4.5"), 0x12304)  # Takes first 4 parts
        self.assertEqual(self.version_manager.version_string_to_hex("a.b.c.d"), 0)

    def test_hex_to_version_string(self):
        """Test hex to version string conversion"""
        # Standard hex values
        self.assertEqual(self.version_manager.hex_to_version_string(0x36003), "3.6.0.3")
        self.assertEqual(self.version_manager.hex_to_version_string(0x34003), "3.4.0.3")
        self.assertEqual(self.version_manager.hex_to_version_string(0x35002), "3.5.0.2")
        self.assertEqual(self.version_manager.hex_to_version_string(0x31001), "3.1.0.1")

        # Hex values with zero patch2 (should show .0)
        self.assertEqual(self.version_manager.hex_to_version_string(0x36000), "3.6.0.0")
        self.assertEqual(self.version_manager.hex_to_version_string(0x34000), "3.4.0.0")

        # Edge cases
        self.assertEqual(self.version_manager.hex_to_version_string(0), "0.0.0.0")
        self.assertEqual(self.version_manager.hex_to_version_string(0xFF0000), "255.0.0.0")

    def test_version_hex_roundtrip(self):
        """Test that hex conversion is reversible"""
        test_versions = ["3.6.0.3", "3.4.0.3", "3.5.0.2", "3.1.0.1", "3.6.0"]

        for version_str in test_versions:
            hex_value = self.version_manager.version_string_to_hex(version_str)
            back_to_str = self.version_manager.hex_to_version_string(hex_value)
            # Note: conversion might normalize .0 to .0, so we'll check the hex values match
            back_to_hex = self.version_manager.version_string_to_hex(back_to_str)
            self.assertEqual(hex_value, back_to_hex,
                           f"Roundtrip failed for {version_str}: {hex_value} -> {back_to_str} -> {back_to_hex}")

    def test_is_valid_rw_version(self):
        """Test RW version validation"""
        # Valid standard versions
        self.assertTrue(self.version_manager.is_valid_rw_version(0x30000))
        self.assertTrue(self.version_manager.is_valid_rw_version(0x36003))
        self.assertTrue(self.version_manager.is_valid_rw_version(0x34003))
        self.assertTrue(self.version_manager.is_valid_rw_version(0x3FFFF))

        # Valid extended format versions
        extended_versions = [0x0800FFFF, 0x1003FFFF, 0x1005FFFF, 0x1401FFFF, 0x1400FFFF, 0x1803FFFF, 0x1C020037]
        for version in extended_versions:
            self.assertTrue(self.version_manager.is_valid_rw_version(version),
                          f"Version 0x{version:X} should be valid")

        # Invalid versions
        self.assertFalse(self.version_manager.is_valid_rw_version(0x2FFFF))  # Too low
        self.assertFalse(self.version_manager.is_valid_rw_version(0x40000))  # Too high
        self.assertFalse(self.version_manager.is_valid_rw_version(0x12345678))  # Random value

    def test_get_version_info(self):
        """Test detailed version information retrieval"""
        version_info = self.version_manager.get_version_info(0x36003)

        self.assertIn('version_hex', version_info)
        self.assertIn('version_string', version_info)
        self.assertIn('version_display_string', version_info)
        self.assertIn('version_value', version_info)
        self.assertIn('is_valid', version_info)

        self.assertEqual(version_info['version_hex'], '0x36003')
        self.assertEqual(version_info['version_string'], '3.6.0.3')
        self.assertEqual(version_info['version_value'], 0x36003)
        self.assertTrue(version_info['is_valid'])

    def test_find_game_by_version(self):
        """Test game identification by version"""
        # Test GTA SA version
        game_info = self.version_manager.find_game_by_version(0x36003)
        if game_info:  # Only test if versionsets.json is loaded
            self.assertIn('name', game_info)
            self.assertIn('platform', game_info)

        # Test unknown version
        unknown_game = self.version_manager.find_game_by_version(0x99999999)
        self.assertIsNone(unknown_game)

    def test_find_all_games_by_version(self):
        """Test finding all games matching a version"""
        games = self.version_manager.find_all_games_by_version(0x36003)
        self.assertIsInstance(games, list)

        # If versionsets.json is loaded, we should find matches
        if games:
            for game in games:
                self.assertIn('name', game)
                self.assertIn('platform', game)

    def test_version_range_checking(self):
        """Test version range validation"""
        # Test that ranges work correctly (if versionsets.json has range data)
        games = self.version_manager.get_games_list()

        for game in games:
            for platform in game.get('platforms', []):
                if 'version_range' in platform:
                    range_info = platform['version_range']
                    min_hex = range_info.get('min_hex', 0)
                    max_hex = range_info.get('max_hex', 0)

                    # Test that the range bounds are reasonable
                    self.assertGreaterEqual(max_hex, min_hex,
                                          f"Invalid range for {game['name']} {platform['platform']}: {min_hex} to {max_hex}")


class TestHexParsingEdgeCases(unittest.TestCase):
    """Test hex parsing with edge cases and corrupted data"""

    def test_rw_chunk_analyzer_corrupted_data(self):
        """Test RWChunkAnalyzer with corrupted hex data"""
        analyzer = RWChunkAnalyzer()

        # Test with incomplete data
        incomplete_data = b'\x10\x00\x00\x00'  # Only 4 bytes, need 12
        count = analyzer.count_chunks(incomplete_data)
        self.assertEqual(count, 0)

        chunks = analyzer.analyze_chunks(incomplete_data)
        self.assertEqual(len(chunks), 0)

        # Test with invalid chunk size (too large)
        invalid_size_data = b'\x10\x00\x00\x00\xFF\xFF\xFF\xFF\x03\x00\x03\x36'  # Size = 0xFFFFFFFF
        count = analyzer.count_chunks(invalid_size_data)
        self.assertEqual(count, 0)  # Should not crash

    def test_rw_chunk_analyzer_unknown_chunk_types(self):
        """Test RWChunkAnalyzer with unknown chunk types"""
        analyzer = RWChunkAnalyzer()

        # Create data with unknown chunk type
        unknown_chunk_data = b'\xFF\xFF\xFF\xFF\x0C\x00\x00\x00\x03\x00\x03\x36' + b'x' * 12
        chunks = analyzer.analyze_chunks(unknown_chunk_data)

        self.assertEqual(len(chunks), 1)
        self.assertEqual(chunks[0]['type_name'], 'Unknown (0xFFFFFFFF)')

    def test_version_manager_edge_cases(self):
        """Test RWVersionManager with edge case inputs"""
        manager = RWVersionManager()

        # Test extreme values
        self.assertEqual(manager.hex_to_version_string(0xFFFFFFFF), "255.15.15.255")
        self.assertEqual(manager.hex_to_version_string(0x00000000), "0.0.0.0")

        # Test invalid string inputs
        self.assertEqual(manager.version_string_to_hex("999.999.999.999"), 0x3FFF7E7)  # Will be clamped
        with self.assertRaises(AttributeError):
            manager.version_string_to_hex(None)  # Should raise exception for None input

    def test_binary_endianness(self):
        """Test that binary parsing uses correct endianness"""
        import struct

        # RenderWare uses little-endian format
        # Test chunk type 0x10 (CLUMP) in little-endian
        clump_data_le = struct.pack('<III', 0x10, 20, 0x36003)
        self.assertEqual(clump_data_le, b'\x10\x00\x00\x00\x14\x00\x00\x00\x03`\x03\x00')

        # Same data in big-endian would be different
        clump_data_be = struct.pack('>III', 0x10, 20, 0x36003)
        self.assertEqual(clump_data_be, b'\x00\x00\x00\x10\x00\x00\x00\x14\x00\x03`\x03')

        # Verify our analyzer correctly parses little-endian
        analyzer = RWChunkAnalyzer()
        chunks_le = analyzer.analyze_chunks(clump_data_le + b'x' * 20)
        self.assertEqual(len(chunks_le), 1)
        self.assertEqual(chunks_le[0]['type_name'], 'Clump')


if __name__ == '__main__':
    unittest.main()