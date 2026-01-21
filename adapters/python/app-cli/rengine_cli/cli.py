"""
Main CLI class
"""

import sys
import logging
from typing import Optional

from .parser import CommandParser
from .factory import CommandFactory
from .base import DependencyManager


class RengineCLI:
    """Main CLI class"""

    def __init__(self):
        self.logger: Optional[logging.Logger] = None
        self.parser = CommandParser()
        self.factory: Optional[CommandFactory] = None
        self.deps = DependencyManager()

    def setup_logging(self):
        """Setup logging configuration"""
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(levelname)s - %(message)s',
            handlers=[
                logging.StreamHandler(sys.stdout)
            ]
        )
        self.logger = logging.getLogger(__name__)
        self.factory = CommandFactory(self.logger)

    def run(self):
        """Main entry point"""
        self.setup_logging()

        # Ensure logging and factory are initialized
        assert self.logger is not None
        assert self.factory is not None

        arg_parser = self.parser.create_parser()
        args = arg_parser.parse_args()

        if not args.command:
            arg_parser.print_help()
            return

        # Get the command name from args
        command_name = self.factory.get_command_name(args)

        if not command_name:
            self.logger.error(f"No subcommand specified for {args.command}")
            arg_parser.print_help()
            sys.exit(1)

        # Execute the command
        try:
            assert command_name is not None  # Type checker hint
            exit_code = self.factory.execute_command(command_name, args)
            sys.exit(exit_code)
        except Exception as e:
            self.logger.error(f"Command failed: {e}")
            if args.verbose:
                import traceback
                traceback.print_exc()
            sys.exit(1)


def main():
    """Main entry point"""
    cli = RengineCLI()
    cli.run()


if __name__ == "__main__":
    main()