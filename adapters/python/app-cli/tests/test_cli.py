"""
Tests for main CLI class
"""

import unittest
from unittest.mock import Mock, patch, MagicMock
import sys
from io import StringIO

from rengine_cli.cli import RengineCLI


class TestRengineCLI(unittest.TestCase):
    """Test RengineCLI class"""

    def setUp(self):
        self.cli = RengineCLI()

    @patch('rengine_cli.cli.CommandParser')
    @patch('rengine_cli.cli.CommandFactory')
    @patch('sys.exit')
    def test_run_help(self, mock_exit, mock_factory_class, mock_parser_class):
        """Test CLI run with help command"""
        # Mock parser
        mock_parser = Mock()
        mock_parser_class.return_value.create_parser.return_value.parse_args.return_value = Mock(command=None)
        mock_parser_class.return_value = mock_parser

        # Mock factory
        mock_factory = Mock()
        mock_factory_class.return_value = mock_factory

        # Run CLI
        self.cli.run()

        # Verify parser was created and used
        mock_parser.create_parser.assert_called_once()
        mock_exit.assert_not_called()  # Should not exit for help

    @patch('rengine_cli.cli.CommandParser')
    @patch('rengine_cli.cli.CommandFactory')
    @patch('sys.exit')
    def test_run_command(self, mock_exit, mock_factory_class, mock_parser_class):
        """Test CLI run with actual command"""
        # Mock parser
        mock_parser = Mock()
        args = Mock()
        args.command = 'img'
        args.img_command = 'info'
        args.verbose = False
        mock_parser.create_parser.return_value.parse_args.return_value = args
        mock_parser_class.return_value = mock_parser

        # Mock factory
        mock_factory = Mock()
        mock_factory.get_command_name.return_value = 'img-info'
        mock_factory.execute_command.return_value = 0
        mock_factory_class.return_value = mock_factory

        # Run CLI
        self.cli.run()

        # Verify command execution
        mock_factory.get_command_name.assert_called_once_with(args)
        mock_factory.execute_command.assert_called_once_with('img-info', args)
        mock_exit.assert_called_once_with(0)

    @patch('rengine_cli.cli.CommandParser')
    @patch('rengine_cli.cli.CommandFactory')
    @patch('sys.exit')
    def test_run_command_error(self, mock_exit, mock_factory_class, mock_parser_class):
        """Test CLI run with command error"""
        # Mock parser
        mock_parser = Mock()
        args = Mock()
        args.command = 'img'
        args.img_command = 'info'
        args.verbose = False
        mock_parser.create_parser.return_value.parse_args.return_value = args
        mock_parser_class.return_value = mock_parser

        # Mock factory
        mock_factory = Mock()
        mock_factory.get_command_name.return_value = 'img-info'
        mock_factory.execute_command.side_effect = Exception("Test error")
        mock_factory_class.return_value = mock_factory

        # Run CLI
        self.cli.run()

        # Verify error handling
        mock_exit.assert_called_once_with(1)

    def test_setup_logging(self):
        """Test logging setup"""
        cli = RengineCLI()
        cli.setup_logging()

        # Verify logger was created
        self.assertIsNotNone(cli.logger)
        self.assertIsNotNone(cli.factory)


if __name__ == '__main__':
    unittest.main()