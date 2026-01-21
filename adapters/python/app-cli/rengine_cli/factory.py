"""
Command factory for creating CLI commands
"""

import logging
from typing import Dict, Type, Any, Optional

from .base import BaseCommand
from .commands import (
    IMGInfoCommand, IMGListCommand, IMGExtractCommand,
    DFFInfoCommand, DFFAnalyzeCommand,
    TXDInfoCommand, TXDExtractCommand,
    COLInfoCommand, COLAnalyzeCommand,
    IPLInfoCommand, IPLAnalyzeCommand,
    RWAnalyzeCommand,
    BatchAnalyzeCommand, BatchExtractIMGsCommand,
    DFFToOBJCommand, TXDToMaterialsCommand, IPLToGodotCommand
)


class CommandFactory:
    """Factory for creating CLI commands"""

    def __init__(self, logger: logging.Logger):
        self.logger = logger
        self._command_map = self._build_command_map()

    def _build_command_map(self) -> Dict[str, Type[BaseCommand]]:
        """Build mapping of command names to command classes"""
        return {
            # IMG commands
            'img-info': IMGInfoCommand,
            'img-list': IMGListCommand,
            'img-extract': IMGExtractCommand,

            # DFF commands
            'dff-info': DFFInfoCommand,
            'dff-analyze': DFFAnalyzeCommand,

            # TXD commands
            'txd-info': TXDInfoCommand,
            'txd-extract': TXDExtractCommand,

            # COL commands
            'col-info': COLInfoCommand,
            'col-analyze': COLAnalyzeCommand,

            # IPL commands
            'ipl-info': IPLInfoCommand,
            'ipl-analyze': IPLAnalyzeCommand,

            # RW commands
            'rw-analyze': RWAnalyzeCommand,

            # Batch commands
            'batch-analyze': BatchAnalyzeCommand,
            'batch-extract-imgs': BatchExtractIMGsCommand,

            # Converter commands
            'dff-to-obj': DFFToOBJCommand,
            'txd-to-materials': TXDToMaterialsCommand,
            'ipl-to-godot': IPLToGodotCommand,
        }

    def create_command(self, command_name: str) -> BaseCommand:
        """Create a command instance"""
        command_class = self._command_map.get(command_name)
        if not command_class:
            raise ValueError(f"Unknown command: {command_name}")

        return command_class(self.logger)

    def get_command_name(self, args) -> Optional[str]:
        """Extract command name from parsed arguments"""
        if not args.command:
            return None

        # Handle direct commands (no subcommands)
        direct_commands = {'dff-to-obj', 'txd-to-materials', 'ipl-to-godot'}
        if args.command in direct_commands:
            return args.command

        # Handle nested subcommands
        command_parts = [args.command]

        # Check for subcommands - only valid commands have subcommands
        subcommand_attr = f'{args.command}_command'
        if hasattr(args, subcommand_attr):
            subcommand = getattr(args, subcommand_attr)
            if subcommand:
                command_parts.append(subcommand)
        else:
            # If this command doesn't support subcommands, it's invalid
            return None

        return '-'.join(command_parts)

    def execute_command(self, command_name: str, args) -> int:
        """Create and execute a command"""
        try:
            command = self.create_command(command_name)
            return command.execute(args)
        except Exception as e:
            self.logger.error(f"Failed to execute command {command_name}: {e}")
            return 1