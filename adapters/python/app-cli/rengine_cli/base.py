"""
Base classes for CLI commands and file handlers
"""

import os
import json
import struct
from abc import ABC, abstractmethod
from pathlib import Path
from typing import Dict, Any, Optional, List, BinaryIO, TextIO
import logging


class CLIError(Exception):
    """Base exception for CLI operations"""
    pass


class FileOperationError(CLIError):
    """Exception for file operation errors"""
    pass


class DependencyError(CLIError):
    """Exception for missing dependencies"""
    pass


class BaseCommand(ABC):
    """Base class for all CLI commands"""

    def __init__(self, logger: logging.Logger):
        self.logger = logger

    @abstractmethod
    def execute(self, args) -> int:
        """Execute the command. Return exit code (0 for success)"""
        pass

    @abstractmethod
    def get_name(self) -> str:
        """Return the command name"""
        pass

    @abstractmethod
    def get_description(self) -> str:
        """Return the command description"""
        pass

    def validate_args(self, args) -> bool:
        """Validate command arguments. Return True if valid."""
        return True

    def handle_error(self, error: Exception, args) -> int:
        """Handle command execution errors. Return exit code."""
        self.logger.error(f"Command failed: {error}")
        if hasattr(args, 'verbose') and args.verbose:
            import traceback
            traceback.print_exc()
        return 1


class BaseFileHandler(ABC):
    """Base class for file format handlers"""

    def __init__(self, logger: logging.Logger):
        self.logger = logger

    @abstractmethod
    def get_supported_extensions(self) -> List[str]:
        """Return list of supported file extensions"""
        pass

    @abstractmethod
    def can_handle(self, file_path: str) -> bool:
        """Check if this handler can process the given file"""
        pass

    def read_file_binary(self, file_path: str) -> bytes:
        """Read file as binary data"""
        try:
            with open(file_path, 'rb') as f:
                return f.read()
        except Exception as e:
            raise FileOperationError(f"Failed to read file {file_path}: {e}")

    def read_file_text(self, file_path: str, encoding: str = 'utf-8') -> str:
        """Read file as text"""
        try:
            with open(file_path, 'r', encoding=encoding) as f:
                return f.read()
        except Exception as e:
            raise FileOperationError(f"Failed to read file {file_path}: {e}")

    def get_file_info(self, file_path: str) -> Dict[str, Any]:
        """Get basic file information"""
        try:
            stat = os.stat(file_path)
            return {
                'path': file_path,
                'name': os.path.basename(file_path),
                'size': stat.st_size,
                'modified': stat.st_mtime,
                'extension': os.path.splitext(file_path)[1].lower()
            }
        except Exception as e:
            raise FileOperationError(f"Failed to get file info for {file_path}: {e}")


class OutputFormatter:
    """Handles different output formats"""

    @staticmethod
    def format_text(data: Dict[str, Any], template: Optional[str] = None) -> str:
        """Format data as human-readable text"""
        if template:
            return template.format(**data)

        # Default formatting
        lines = []
        for key, value in data.items():
            if isinstance(value, dict):
                lines.append(f"{key}:")
                for sub_key, sub_value in value.items():
                    lines.append(f"  {sub_key}: {sub_value}")
            elif isinstance(value, list):
                lines.append(f"{key}:")
                for item in value[:10]:  # Limit to first 10 items
                    lines.append(f"  - {item}")
                if len(value) > 10:
                    lines.append(f"  ... and {len(value) - 10} more")
            else:
                lines.append(f"{key}: {value}")
        return "\n".join(lines)

    @staticmethod
    def format_json(data: Dict[str, Any], indent: int = 2) -> str:
        """Format data as JSON"""
        return json.dumps(data, indent=indent)

    @staticmethod
    def output_data(data: Dict[str, Any], args, default_format: str = 'text') -> None:
        """Output data in the requested format"""
        output_format = getattr(args, 'format', default_format)

        if output_format == 'json':
            print(OutputFormatter.format_json(data))
        else:
            print(OutputFormatter.format_text(data))


class RWChunkAnalyzer:
    """Utility class for RenderWare chunk analysis"""

    # RenderWare chunk type names
    CHUNK_NAMES = {
        0x00000001: "Struct",
        0x00000002: "String",
        0x00000003: "Extension",
        0x00000006: "Texture",
        0x00000008: "Material",
        0x0000000E: "FrameList",
        0x0000000F: "Geometry",
        0x00000010: "Clump",
        0x00000014: "Atomic",
        0x00000015: "Raster",
        0x00000016: "TextureDictionary",
        0x00000019: "GeometryList",
        0x00000301: "BinMeshPLG",
        0x00000302: "NativeDataPLG",
        0x00000310: "MaterialEffectsPLG",
        0x00000FEE: "DeltaMorphPLG",
        0x00005101: "RightToRenderPLG",
        0x00005102: "UVAnimationPLG",
        0x116: "TextureDictionary",
        0x0010: "Clump",
    }

    # RenderWare version names
    VERSION_NAMES = {
        0x31001: "3.1.0.1 (GTA III)",
        0x33002: "3.3.0.2 (Vice City)",
        0x34003: "3.4.0.3 (SA)",
        0x35000: "3.5.0.0 (LCS)",
        0x35002: "3.5.0.2 (VCS)",
        0x36003: "3.6.0.3 (SA Advanced)",
    }

    @classmethod
    def get_chunk_name(cls, chunk_type: int) -> str:
        """Get human-readable name for RenderWare chunk type"""
        return cls.CHUNK_NAMES.get(chunk_type, f"Unknown (0x{chunk_type:08X})")

    @classmethod
    def get_version_name(cls, version: int) -> str:
        """Get human-readable name for RenderWare version"""
        return cls.VERSION_NAMES.get(version, f"0x{version:05X}")

    @classmethod
    def analyze_chunks(cls, data: bytes, offset: int = 0, depth: int = 0, max_depth: int = 3) -> List[Dict[str, Any]]:
        """Recursively analyze RenderWare chunks"""
        chunks = []

        if depth >= max_depth or offset >= len(data) - 12:
            return chunks

        try:
            chunk_type, chunk_size, chunk_version = struct.unpack('<III', data[offset:offset+12])

            chunk_info = {
                'type': f"0x{chunk_type:08X}",
                'type_name': cls.get_chunk_name(chunk_type),
                'size': chunk_size,
                'version': f"0x{chunk_version:08X}",
                'version_name': cls.get_version_name(chunk_version),
                'offset': offset,
                'depth': depth
            }

            # Add child chunks if within size bounds
            child_offset = offset + 12
            if depth < max_depth - 1 and child_offset + 12 < offset + 12 + chunk_size:
                chunk_info['children'] = cls.analyze_chunks(data, child_offset, depth + 1, max_depth)

            chunks.append(chunk_info)

            # Continue with next chunk at this level
            next_offset = offset + chunk_size + 12
            if next_offset + 12 < len(data):
                chunks.extend(cls.analyze_chunks(data, next_offset, depth, max_depth))

        except struct.error:
            pass

        return chunks

    @classmethod
    def count_chunks(cls, data: bytes) -> int:
        """Count total number of chunks in data"""
        count = 0
        pos = 0
        while pos < len(data) - 12:
            try:
                chunk_type, chunk_size, chunk_version = struct.unpack('<III', data[pos:pos+12])
                count += 1
                pos += chunk_size + 12
            except:
                break
        return count


class DependencyManager:
    """Manages optional dependencies and their availability"""

    def __init__(self):
        self._qt_app_available: bool = False
        self._blender_img_available: bool = False
        self._blender_col_available: bool = False
        self._blender_map_available: bool = False
        self._blender_available: bool = False
        self._rw_manager = None
        self._detect_rw_file_format = None

        self._setup_dependencies()

    def _setup_dependencies(self):
        """Initialize dependency availability"""
        # Setup paths
        cli_path = Path(__file__).parent.parent
        qt_app_path = cli_path.parent / "app-qt"
        blender_path = cli_path.parent / "blender"

        import sys
        if str(qt_app_path) not in sys.path:
            sys.path.insert(0, str(qt_app_path))
        if str(blender_path) not in sys.path:
            sys.path.insert(0, str(blender_path))

        # Check app-qt dependencies
        try:
            from .common.rw_versions import RWVersionManager
            self._rw_manager = RWVersionManager()
            self._detect_rw_file_format = self._rw_manager.detect_file_format_version
            self._qt_app_available = True
        except ImportError:
            self._qt_app_available = False

        # CLI uses local implementations, no external blender dependencies needed
        self._blender_img_available = True  # We have local img.py
        self._blender_col_available = True  # We have local col.py
        self._blender_map_available = True  # We have local map.py (simplified IPL support)

        self._blender_available = False  # No external blender dependencies

    @property
    def qt_app_available(self) -> bool:
        return self._qt_app_available

    @property
    def blender_img_available(self) -> bool:
        return self._blender_img_available

    @property
    def blender_col_available(self) -> bool:
        return self._blender_col_available

    @property
    def blender_map_available(self) -> bool:
        return self._blender_map_available

    @property
    def blender_available(self) -> bool:
        return self._blender_available

    @property
    def rw_manager(self):
        return self._rw_manager

    @property
    def detect_rw_file_format(self):
        return self._detect_rw_file_format

    def require_qt_app(self):
        """Raise exception if app-qt is not available"""
        if not self.qt_app_available:
            raise DependencyError("app-qt modules not available")

    def require_blender_img(self):
        """Raise exception if blender IMG is not available"""
        if not self.blender_img_available:
            raise DependencyError("Blender IMG modules not available")

    def require_blender_col(self):
        """Raise exception if blender COL is not available"""
        if not self.blender_col_available:
            raise DependencyError("Blender COL modules not available")

    def require_blender_map(self):
        """Raise exception if blender MAP is not available"""
        if not self.blender_map_available:
            raise DependencyError("Blender MAP modules not available")

    def require_blender(self):
        """Raise exception if any blender module is not available"""
        if not self.blender_available:
            raise DependencyError("Blender modules not available")