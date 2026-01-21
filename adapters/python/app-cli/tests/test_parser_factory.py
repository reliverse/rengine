"""
Tests for parser and factory components
"""

import unittest
from unittest.mock import Mock, patch
import argparse

from rengine_cli.parser import CommandParser
from rengine_cli.factory import CommandFactory
from rengine_cli.commands import IMGInfoCommand


class TestCommandParser(unittest.TestCase):
    """Test CommandParser class"""

    def setUp(self):
        self.parser = CommandParser()

    def test_parser_creation(self):
        """Test parser creation"""
        arg_parser = self.parser.create_parser()
        self.assertIsInstance(arg_parser, argparse.ArgumentParser)

        # Test that parser has expected structure (just check it was created)
        self.assertIsNotNone(arg_parser)

    def test_img_parsers(self):
        """Test IMG parser creation"""
        arg_parser = self.parser.create_parser()

        # Test img info command
        args = arg_parser.parse_args(['img', 'info', 'test.img'])
        self.assertEqual(args.command, 'img')
        self.assertEqual(args.img_command, 'info')
        self.assertEqual(args.file, 'test.img')

        # Test img list command
        args = arg_parser.parse_args(['img', 'list', 'test.img', '--filter', '.dff'])
        self.assertEqual(args.img_command, 'list')
        self.assertEqual(args.filter, '.dff')

        # Test img extract command
        args = arg_parser.parse_args(['img', 'extract', 'test.img', '--output', 'out/'])
        self.assertEqual(args.img_command, 'extract')
        self.assertEqual(args.output, 'out/')

    def test_dff_parsers(self):
        """Test DFF parser creation"""
        arg_parser = self.parser.create_parser()

        # Test dff info command
        args = arg_parser.parse_args(['dff', 'info', 'test.dff'])
        self.assertEqual(args.command, 'dff')
        self.assertEqual(args.dff_command, 'info')

        # Test dff analyze command
        args = arg_parser.parse_args(['dff', 'analyze', 'test.dff', '--format', 'json'])
        self.assertEqual(args.dff_command, 'analyze')
        self.assertEqual(args.format, 'json')

    def test_batch_parsers(self):
        """Test batch parser creation"""
        arg_parser = self.parser.create_parser()

        # Test batch analyze command
        args = arg_parser.parse_args([
            'batch', 'analyze', '/tmp', '--format', 'dff', '--recursive', '--max-files', '10'
        ])
        self.assertEqual(args.command, 'batch')
        self.assertEqual(args.batch_command, 'analyze')
        self.assertEqual(args.directory, '/tmp')
        self.assertEqual(args.format, 'dff')
        self.assertTrue(args.recursive)
        self.assertEqual(args.max_files, 10)


class TestCommandFactory(unittest.TestCase):
    """Test CommandFactory class"""

    def setUp(self):
        self.logger = Mock()
        self.factory = CommandFactory(self.logger)

    def test_command_creation(self):
        """Test command creation"""
        # Test creating a known command
        command = self.factory.create_command('img-info')
        self.assertIsInstance(command, IMGInfoCommand)

    def test_unknown_command(self):
        """Test unknown command handling"""
        with self.assertRaises(ValueError):
            self.factory.create_command('unknown-command')

    def test_get_command_name(self):
        """Test command name extraction from args"""

        # Create a simple args object
        class Args:
            pass

        # Test IMG commands
        args = Args()
        args.command = 'img'
        args.img_command = 'info'
        command_name = self.factory.get_command_name(args)
        self.assertEqual(command_name, 'img-info')

        # Test DFF commands
        args.command = 'dff'
        args.dff_command = 'analyze'
        command_name = self.factory.get_command_name(args)
        self.assertEqual(command_name, 'dff-analyze')

    @patch('rengine_cli.commands.IMGInfoCommand.execute')
    def test_execute_command(self, mock_execute):
        """Test command execution"""
        mock_execute.return_value = 0

        args = Mock()
        exit_code = self.factory.execute_command('img-info', args)

        self.assertEqual(exit_code, 0)
        mock_execute.assert_called_once_with(args)

    def test_execute_unknown_command(self):
        """Test execution of unknown command"""
        args = Mock()
        exit_code = self.factory.execute_command('unknown-command', args)

        self.assertEqual(exit_code, 1)
        self.logger.error.assert_called_once()


if __name__ == '__main__':
    unittest.main()