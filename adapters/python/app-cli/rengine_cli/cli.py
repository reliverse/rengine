"""
Main CLI class
"""

import sys
import logging

from .parser import CommandParser
from .factory import CommandFactory
from .base import DependencyManager


class RengineCLI:
    """Main CLI class"""

    def __init__(self):
        self.logger = None
        self.parser = CommandParser()
        self.factory = None
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