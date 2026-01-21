"""
Argument parser for CLI commands
"""

import argparse
from typing import Dict, Any, List


class CommandParser:
    """Command parser"""

    def __init__(self):
        self.parser = None
        self.subparsers = None

    def create_parser(self) -> argparse.ArgumentParser:
        """Create the main argument parser"""
        self.parser = argparse.ArgumentParser(
            description="Rengine CLI - GTA File Processing Tool",
            formatter_class=argparse.RawDescriptionHelpFormatter,
            epilog=self._get_examples()
        )

        # Global options
        self.parser.add_argument('--verbose', '-v', action='store_true',
                               help='Enable verbose output')
        self.parser.add_argument('--quiet', '-q', action='store_true',
                               help='Suppress non-error output')

        # Create subparsers
        self.subparsers = self.parser.add_subparsers(dest='command',
                                                   help='Available commands')

        # Add all command parsers
        self._add_img_parsers()
        self._add_dff_parsers()
        self._add_txd_parsers()
        self._add_col_parsers()
        self._add_ipl_parsers()
        self._add_rw_parsers()
        self._add_batch_parsers()
        self._add_converter_parsers()

        return self.parser

    def _get_examples(self) -> str:
        """Get usage examples"""
        return """
Examples:
  # IMG operations
  python cli.py img info gta3.img
  python cli.py img list gta3.img
  python cli.py img extract gta3.img model.dff --output extracted/

  # DFF operations
  python cli.py dff info model.dff
  python cli.py dff analyze model.dff --detailed

  # TXD operations
  python cli.py txd info textures.txd
  python cli.py txd extract textures.txd --output textures/

  # COL operations
  python cli.py col info collision.col
  python cli.py col analyze collision.col

  # IPL operations
  python cli.py ipl info map.ipl
  python cli.py ipl analyze map.ipl

  # IPL to Godot conversion
  python cli.py ipl-to-godot map.ipl --output map.tscn
  python cli.py ipl-to-godot /path/to/ipl/directory --output /path/to/godot/project --project-name MyMap

  # RenderWare analysis
  python cli.py rw analyze model.dff --depth 5

  # Batch operations
  python cli.py batch analyze /path/to/gta/files --format dff --recursive
  python cli.py batch extract-imgs /path/to/gta/files --output extracted/
        """

    def _add_img_parsers(self):
        """Add IMG-related parsers"""
        # IMG main parser
        img_parser = self.subparsers.add_parser('img', help='IMG archive operations')
        img_subparsers = img_parser.add_subparsers(dest='img_command')

        # IMG info
        img_info = img_subparsers.add_parser('info', help='Show IMG archive information')
        img_info.add_argument('file', help='IMG file path')
        img_info.add_argument('--detailed', action='store_true',
                             help='Show detailed information')

        # IMG list
        img_list = img_subparsers.add_parser('list', help='List files in IMG archive')
        img_list.add_argument('file', help='IMG file path')
        img_list.add_argument('--filter', help='Filter by file extension (e.g., .dff)')
        img_list.add_argument('--format', choices=['table', 'json'],
                             default='table', help='Output format')

        # IMG extract
        img_extract = img_subparsers.add_parser('extract', help='Extract files from IMG archive')
        img_extract.add_argument('file', help='IMG file path')
        img_extract.add_argument('files', nargs='*', help='Specific files to extract (empty for all)')
        img_extract.add_argument('--output', '-o', required=True,
                                help='Output directory')
        img_extract.add_argument('--overwrite', action='store_true',
                                help='Overwrite existing files')

    def _add_dff_parsers(self):
        """Add DFF-related parsers"""
        dff_parser = self.subparsers.add_parser('dff', help='DFF model operations')
        dff_subparsers = dff_parser.add_subparsers(dest='dff_command')

        # DFF info
        dff_info = dff_subparsers.add_parser('info', help='Show DFF model information')
        dff_info.add_argument('file', help='DFF file path')
        dff_info.add_argument('--detailed', action='store_true',
                             help='Show detailed information')

        # DFF analyze
        dff_analyze = dff_subparsers.add_parser('analyze', help='Analyze DFF model structure')
        dff_analyze.add_argument('file', help='DFF file path')
        dff_analyze.add_argument('--format', choices=['text', 'json'],
                                default='text', help='Output format')

    def _add_txd_parsers(self):
        """Add TXD-related parsers"""
        txd_parser = self.subparsers.add_parser('txd', help='TXD texture operations')
        txd_subparsers = txd_parser.add_subparsers(dest='txd_command')

        # TXD info
        txd_info = txd_subparsers.add_parser('info', help='Show TXD texture information')
        txd_info.add_argument('file', help='TXD file path')
        txd_info.add_argument('--detailed', action='store_true',
                             help='Show detailed information')

        # TXD extract
        txd_extract = txd_subparsers.add_parser('extract', help='Extract textures from TXD')
        txd_extract.add_argument('file', help='TXD file path')
        txd_extract.add_argument('--output', '-o', required=True,
                                help='Output directory')
        txd_extract.add_argument('--format', choices=['png', 'dds', 'tga'],
                                default='png', help='Output image format')

    def _add_col_parsers(self):
        """Add COL-related parsers"""
        col_parser = self.subparsers.add_parser('col', help='COL collision operations')
        col_subparsers = col_parser.add_subparsers(dest='col_command')

        # COL info
        col_info = col_subparsers.add_parser('info', help='Show COL collision information')
        col_info.add_argument('file', help='COL file path')
        col_info.add_argument('--detailed', action='store_true',
                             help='Show detailed information')

        # COL analyze
        col_analyze = col_subparsers.add_parser('analyze', help='Analyze COL collision structure')
        col_analyze.add_argument('file', help='COL file path')
        col_analyze.add_argument('--format', choices=['text', 'json'],
                                default='text', help='Output format')

    def _add_ipl_parsers(self):
        """Add IPL-related parsers"""
        ipl_parser = self.subparsers.add_parser('ipl', help='IPL map operations')
        ipl_subparsers = ipl_parser.add_subparsers(dest='ipl_command')

        # IPL info
        ipl_info = ipl_subparsers.add_parser('info', help='Show IPL map information')
        ipl_info.add_argument('file', help='IPL file path')
        ipl_info.add_argument('--detailed', action='store_true',
                             help='Show detailed information')

        # IPL analyze
        ipl_analyze = ipl_subparsers.add_parser('analyze', help='Analyze IPL map structure')
        ipl_analyze.add_argument('file', help='IPL file path')
        ipl_analyze.add_argument('--format', choices=['text', 'json'],
                                default='text', help='Output format')

    def _add_rw_parsers(self):
        """Add RenderWare parsers"""
        rw_parser = self.subparsers.add_parser('rw', help='RenderWare chunk analysis')
        rw_subparsers = rw_parser.add_subparsers(dest='rw_command')

        # RW analyze
        rw_analyze = rw_subparsers.add_parser('analyze', help='Analyze RenderWare file chunks')
        rw_analyze.add_argument('file', help='RenderWare file path (DFF/TXD/COL)')
        rw_analyze.add_argument('--depth', type=int, default=3,
                               help='Maximum chunk depth to analyze')
        rw_analyze.add_argument('--format', choices=['tree', 'json'],
                               default='tree', help='Output format')

    def _add_batch_parsers(self):
        """Add batch operation parsers"""
        batch_parser = self.subparsers.add_parser('batch', help='Batch processing operations')
        batch_subparsers = batch_parser.add_subparsers(dest='batch_command')

        # Batch analyze
        batch_analyze = batch_subparsers.add_parser('analyze', help='Analyze multiple files')
        batch_analyze.add_argument('directory', help='Directory to scan')
        batch_analyze.add_argument('--format', choices=['dff', 'txd', 'col', 'ipl', 'img', 'all'],
                                  default='all', help='File format to analyze')
        batch_analyze.add_argument('--recursive', '-r', action='store_true',
                                  help='Scan directories recursively')
        batch_analyze.add_argument('--output', '-o', help='Output file for results')
        batch_analyze.add_argument('--max-files', type=int,
                                  help='Maximum number of files to process')

        # Batch extract-imgs
        batch_extract = batch_subparsers.add_parser('extract-imgs',
                                                   help='Extract all IMG archives in directory')
        batch_extract.add_argument('directory', help='Directory containing IMG files')
        batch_extract.add_argument('--output', '-o', required=True,
                                  help='Output directory for extracted files')
        batch_extract.add_argument('--recursive', '-r', action='store_true',
                                  help='Scan directories recursively')
        batch_extract.add_argument('--overwrite', action='store_true',
                                help='Overwrite existing files')

    def _add_converter_parsers(self):
        """Add converter parsers"""
        # DFF to OBJ converter
        dff_to_obj = self.subparsers.add_parser('dff-to-obj', help='Convert DFF to OBJ format')
        dff_to_obj.add_argument('file', help='DFF file to convert')
        dff_to_obj.add_argument('--output', '-o', required=True,
                               help='Output OBJ file path')
        dff_to_obj.add_argument('--txd', help='TXD file for materials (optional)')

        # TXD to materials converter
        txd_to_materials = self.subparsers.add_parser('txd-to-materials',
                                                     help='Extract textures and materials from TXD')
        txd_to_materials.add_argument('file', help='TXD file to process')
        txd_to_materials.add_argument('--output', '-o', required=True,
                                     help='Output directory for textures and materials')
        txd_to_materials.add_argument('--format', choices=['png', 'jpg'],
                                     default='png', help='Output image format')

        # IPL to Godot converter
        ipl_to_godot = self.subparsers.add_parser('ipl-to-godot',
                                                 help='Convert IPL map files to Godot project/scene')
        ipl_to_godot.add_argument('input', help='IPL file or directory containing IPL files')
        ipl_to_godot.add_argument('--output', '-o', required=True,
                                 help='Output directory for Godot project (when input is directory) or scene file (.tscn when input is file)')
        ipl_to_godot.add_argument('--template', help='Template scene file to extend')
        ipl_to_godot.add_argument('--project-name', default='IPLMap',
                                 help='Name for the Godot project (default: IPLMap)')