"""
File format handlers for different GTA file types
"""

import os
import struct
from collections import Counter
from typing import Dict, Any, List, Optional
import logging

from .base import BaseFileHandler, OutputFormatter, RWChunkAnalyzer, DependencyManager

# Import DFF and TXD parsers with fallback for missing dependencies
try:
    from .common.DFF import dff, Vector, SkinPLG, HAnimPLG, UserData
    from .common.txd import txd, TextureNative
    DFF_TXD_SUPPORT = True
except ImportError:
    # Create dummy classes if imports fail
    class dff:
        def __init__(self):
            self.atomic_list = []
            self.frame_list = []
            self.geometry_list = []
            self.uvanim_dict = []
            self.collisions = []
            self.rw_version = 0
        def load_file(self, path):
            raise Exception("DFF support not available - DFF/TXD modules failed to import")

    class txd:
        def __init__(self):
            self.native_textures = []
            self.rw_version = 0
            self.device_id = 0
        def load_file(self, path):
            raise Exception("TXD support not available - DFF/TXD modules failed to import")

    Vector = SkinPLG = HAnimPLG = UserData = TextureNative = None
    DFF_TXD_SUPPORT = False


class IMGHandler(BaseFileHandler):
    """Handler for IMG archive files"""

    def __init__(self, logger: logging.Logger, deps: DependencyManager):
        super().__init__(logger)
        self.deps = deps

        # Try to import local img class
        try:
            from .common.img import img as IMGArchive
            self._img_archive_class = IMGArchive
        except ImportError:
            self._img_archive_class = None

    def get_supported_extensions(self) -> List[str]:
        return ['.img']

    def can_handle(self, file_path: str) -> bool:
        ext = os.path.splitext(file_path)[1].lower()
        return ext in self.get_supported_extensions()

    def get_info(self, file_path: str, detailed: bool = False) -> Dict[str, Any]:
        """Get IMG archive information"""
        if not self._img_archive_class:
            raise Exception("IMG support not available - img module failed to import")

        archive = self._img_archive_class.open(file_path)
        info = {
            'file': file_path,
            'version': 'VER2' if hasattr(archive, '_ver2') and archive._ver2 else 'VER1',
            'file_count': len(archive.directory_entries),
            'size': os.path.getsize(file_path)
        }

        if detailed:
            # Analyze file types and RW versions
            file_types = Counter()
            rw_versions = Counter()

            for idx, entry in enumerate(archive.directory_entries):
                ext = os.path.splitext(entry.name)[1].lower()
                file_types[ext] += 1

                # Try to detect RW version
                try:
                    name, data = archive.read_entry(idx)
                    if self.deps.detect_rw_file_format:
                        file_format, version_desc, version_num = self.deps.detect_rw_file_format(data, entry.name)
                        if file_format != 'Unknown':
                            rw_versions[f"{file_format} ({version_desc})"] += 1
                except:
                    pass

            info['file_types'] = dict(file_types)
            info['rw_versions'] = dict(rw_versions)

        return info

    def list_files(self, file_path: str, filter_ext: Optional[str] = None) -> Dict[str, Any]:
        """List files in IMG archive"""
        if not self._img_archive_class:
            raise Exception("IMG support not available - img module failed to import")

        archive = self._img_archive_class.open(file_path)
        entries = archive.directory_entries

        if filter_ext:
            filter_ext = filter_ext.lower()
            entries = [e for e in entries if e.name.lower().endswith(filter_ext)]

        return {
            'file': file_path,
            'entries': [{'name': e.name, 'offset': e.offset, 'size': e.size}
                       for e in entries]
        }

    def extract_files(self, file_path: str, output_dir: str, files: Optional[List[str]] = None,
                     overwrite: bool = False, quiet: bool = False) -> Dict[str, Any]:
        """Extract files from IMG archive"""
        if not self._img_archive_class:
            raise Exception("IMG support not available - img module failed to import")

        os.makedirs(output_dir, exist_ok=True)
        archive = self._img_archive_class.open(file_path)

        entries_to_extract = list(enumerate(archive.directory_entries))
        if files:
            file_names = set(files)
            entries_to_extract = [(idx, e) for idx, e in entries_to_extract if e.name in file_names]

        extracted_count = 0
        skipped_count = 0

        for idx, entry in entries_to_extract:
            output_path = os.path.join(output_dir, entry.name)

            # Create subdirectories if needed
            os.makedirs(os.path.dirname(output_path), exist_ok=True)

            if os.path.exists(output_path) and not overwrite:
                if not quiet:
                    self.logger.info(f"Skipping {entry.name} (exists)")
                skipped_count += 1
                continue

            try:
                name, data = archive.read_entry(idx)
                with open(output_path, 'wb') as f:
                    f.write(data)
                extracted_count += 1

                if not quiet:
                    self.logger.info(f"Extracted {entry.name}")

            except Exception as e:
                self.logger.error(f"Failed to extract {entry.name}: {e}")

        return {
            'extracted': extracted_count,
            'skipped': skipped_count,
            'total': len(entries_to_extract)
        }


class DFFHandler(BaseFileHandler):
    """Handler for DFF model files"""

    def get_supported_extensions(self) -> List[str]:
        return ['.dff']

    def can_handle(self, file_path: str) -> bool:
        ext = os.path.splitext(file_path)[1].lower()
        return ext in self.get_supported_extensions()

    def get_info(self, file_path: str, deps: DependencyManager) -> Dict[str, Any]:
        """Get DFF model information"""
        data = self.read_file_binary(file_path)

        info = {
            'file': file_path,
            'size': len(data),
            'chunks': RWChunkAnalyzer.count_chunks(data)
        }

        # Detect RW version
        if deps.detect_rw_file_format:
            try:
                file_format, version_desc, version_num = deps.detect_rw_file_format(data, file_path)
                info['rw_version'] = version_desc
            except:
                info['rw_version'] = 'Unknown'

        return info

    def analyze_structure(self, file_path: str, deps: DependencyManager) -> Dict[str, Any]:
        """Analyze DFF structure"""
        data = self.read_file_binary(file_path)

        analysis = {
            'file': file_path,
            'size': len(data),
            'chunks': RWChunkAnalyzer.analyze_chunks(data, 0, 0, 3)  # Use proper analyzer with depth limit
        }

        return analysis


class TXDHandler(BaseFileHandler):
    """Handler for TXD texture files"""

    def get_supported_extensions(self) -> List[str]:
        return ['.txd']

    def can_handle(self, file_path: str) -> bool:
        ext = os.path.splitext(file_path)[1].lower()
        return ext in self.get_supported_extensions()

    def get_info(self, file_path: str, deps: DependencyManager) -> Dict[str, Any]:
        """Get TXD texture information"""
        data = self.read_file_binary(file_path)

        info = {
            'file': file_path,
            'size': len(data),
            'chunks': RWChunkAnalyzer.count_chunks(data)
        }

        # Detect RW version
        if deps.detect_rw_file_format:
            try:
                file_format, version_desc, version_num = deps.detect_rw_file_format(data, file_path)
                info['rw_version'] = version_desc
            except:
                info['rw_version'] = 'Unknown'

        return info

    def extract_textures(self, file_path: str, output_dir: str, deps: DependencyManager,
                        format: str = 'png') -> Dict[str, Any]:
        """Extract textures from TXD"""
        # Use the TXDConverterHandler for actual extraction
        converter = TXDConverterHandler(self.logger)
        return converter.extract_materials(file_path, output_dir, format, deps)


class COLHandler(BaseFileHandler):
    """Handler for COL collision files"""

    def get_supported_extensions(self) -> List[str]:
        return ['.col']

    def can_handle(self, file_path: str) -> bool:
        ext = os.path.splitext(file_path)[1].lower()
        return ext in self.get_supported_extensions()

    def get_info(self, file_path: str, deps: DependencyManager) -> Dict[str, Any]:
        """Get COL collision information"""
        try:
            from .common.col import coll as LocalCOL
        except ImportError:
            raise Exception("COL support not available - col module failed to import")

        col_loader = LocalCOL()
        col_loader.load_file(file_path)
        col_data = col_loader

        # Get file size
        file_size = os.path.getsize(file_path)

        info = {
            'file': file_path,
            'size': file_size,
            'collision_objects': len(col_data.models) if hasattr(col_data, 'models') else 0
        }

        if hasattr(col_data, 'models'):
            total_spheres = sum(len(model.spheres) if hasattr(model, 'spheres') else 0
                               for model in col_data.models)
            total_boxes = sum(len(model.boxes) if hasattr(model, 'boxes') else 0
                             for model in col_data.models)
            total_meshes = sum(len(model.mesh_faces) if hasattr(model, 'mesh_faces') else 0
                              for model in col_data.models)

            info.update({
                'spheres': total_spheres,
                'boxes': total_boxes,
                'meshes': total_meshes
            })

        # Detect COL version
        with open(file_path, 'rb') as f:
            header = f.read(4)
            if len(header) >= 4:
                col_type = header
                if col_type == b'COLL':
                    version = 'COL1 (GTA III/Vice City)'
                elif col_type == b'COL2':
                    version = 'COL2 (GTA San Andreas)'
                elif col_type == b'COL3':
                    version = 'COL3 (GTA SA Advanced)'
                elif col_type == b'COL4':
                    version = 'COL4 (Extended)'
                else:
                    version = 'Unknown'
            else:
                version = 'Unknown'
            info['version'] = version

        return info

    def analyze_structure(self, file_path: str, deps: DependencyManager) -> Dict[str, Any]:
        """Analyze COL collision structure"""
        try:
            from .common.col import coll as LocalCOL
        except ImportError:
            raise Exception("COL support not available - col module failed to import")

        col_loader = LocalCOL()
        col_loader.load_file(file_path)
        col_data = col_loader

        analysis = {
            'file': file_path,
            'size': os.path.getsize(file_path),
            'collision_objects': []
        }

        if hasattr(col_data, 'collisions'):
            for i, col in enumerate(col_data.collisions):
                col_info = {
                    'index': i,
                    'model_name': getattr(col, 'model_name', 'Unknown'),
                    'spheres': len(col.spheres) if hasattr(col, 'spheres') else 0,
                    'boxes': len(col.boxes) if hasattr(col, 'boxes') else 0,
                    'meshes': len(col.meshes) if hasattr(col, 'meshes') else 0
                }
                analysis['collision_objects'].append(col_info)

        return analysis


class IPLHandler(BaseFileHandler):
    """Handler for IPL map files"""

    def get_supported_extensions(self) -> List[str]:
        return ['.ipl']

    def can_handle(self, file_path: str) -> bool:
        ext = os.path.splitext(file_path)[1].lower()
        return ext in self.get_supported_extensions()

    def get_info(self, file_path: str, deps: DependencyManager) -> Dict[str, Any]:
        """Get IPL map information"""
        from .common.map import MapDataUtility
        from .common.data.map_data import SA_structures, SA_IPL_aliases

        # Use existing binary/text IPL parser with SA data structures
        try:
            sections = MapDataUtility.read_file(file_path, SA_structures, SA_IPL_aliases)
        except Exception as e:
            # Fallback to basic file info if parsing fails
            self.logger.warning(f"Failed to parse IPL file, returning basic info: {e}")
            return {
                'file': file_path,
                'size': os.path.getsize(file_path),
                'instances': 0,
                'zones': 0,
                'occlusions': 0,
                'culls': 0,
                'pickups': 0,
                'paths': 0,
                'error': str(e)
            }

        info = {
            'file': file_path,
            'size': os.path.getsize(file_path),
            'instances': len(sections.get('inst', [])),
            'zones': len(sections.get('zone', [])),
            'occlusions': len(sections.get('occl', [])),
            'culls': len(sections.get('cull', [])),
            'pickups': len(sections.get('pick', [])),
            'paths': len(sections.get('path', []))
        }

        return info

    def analyze_structure(self, file_path: str, deps: DependencyManager) -> Dict[str, Any]:
        """Analyze IPL map structure"""
        from .common.map import MapDataUtility
        from .common.data.map_data import SA_structures, SA_IPL_aliases

        # Use existing binary/text IPL parser with SA data structures
        try:
            sections = MapDataUtility.read_file(file_path, SA_structures, SA_IPL_aliases)
        except Exception as e:
            # Fallback to basic file info if parsing fails
            return {
                'file': file_path,
                'sections': {},
                'error': str(e)
            }

        analysis = {
            'file': file_path,
            'sections': {}
        }

        for section_name, entries in sections.items():
            analysis['sections'][section_name] = {
                'count': len(entries),
                'sample_entries': []  # We'll format entries properly below
            }

            # Show sample entries based on section type
            if entries:
                try:
                    if section_name == 'inst':
                        # For instances, show ID, position, rotation
                        for entry in entries[:5]:
                            analysis['sections'][section_name]['sample_entries'].append([
                                str(getattr(entry, 'id', 'N/A')),
                                str(getattr(entry, 'interior', '0')),
                                f"({getattr(entry, 'x', 0):.2f}, {getattr(entry, 'y', 0):.2f}, {getattr(entry, 'z', 0):.2f})"
                            ])
                    elif section_name == 'zone':
                        # For zones, show name and boundaries
                        for entry in entries[:5]:
                            analysis['sections'][section_name]['sample_entries'].append([
                                getattr(entry, 'name', 'N/A'),
                                getattr(entry, 'type', 'N/A')
                            ])
                    else:
                        # For other sections, show first few fields
                        for entry in entries[:5]:
                            entry_data = []
                            # Convert entry to list of string values
                            if hasattr(entry, '_fields'):
                                for field in entry._fields[:4]:  # Show first 4 fields
                                    value = getattr(entry, field, 'N/A')
                                    entry_data.append(str(value))
                            else:
                                entry_data = [str(entry)]  # Fallback
                            analysis['sections'][section_name]['sample_entries'].append(entry_data)
                except Exception as e:
                    analysis['sections'][section_name]['sample_entries'] = [f"Error formatting entries: {e}"]

        return analysis


class RWHandler(BaseFileHandler):
    """Handler for RenderWare chunk analysis"""

    def get_supported_extensions(self) -> List[str]:
        return ['.dff', '.txd', '.col']  # RW-based formats

    def can_handle(self, file_path: str) -> bool:
        ext = os.path.splitext(file_path)[1].lower()
        return ext in self.get_supported_extensions()

    def analyze_chunks(self, file_path: str, depth: int = 3) -> Dict[str, Any]:
        """Analyze RenderWare file chunks"""
        data = self.read_file_binary(file_path)

        analysis = {
            'file': file_path,
            'size': len(data),
            'chunks': RWChunkAnalyzer.analyze_chunks(data, 0, 0, depth)
        }

        return analysis


class DFFConverterHandler:
    """Handler for converting DFF files to OBJ format"""

    def __init__(self, logger: logging.Logger):
        self.logger = logger

    def convert_to_obj(self, dff_file: str, obj_file: str, txd_file: Optional[str] = None,
                      deps: DependencyManager = None) -> Dict[str, Any]:
        """Convert DFF file to OBJ format with optional TXD materials"""

        from .modules.dff_to_obj import DFFToOBJConverter

        converter = DFFToOBJConverter(self.logger)
        return converter.convert(dff_file, obj_file, txd_file)



class TXDConverterHandler:
    """Handler for extracting materials and textures from TXD files"""

    def __init__(self, logger: logging.Logger):
        self.logger = logger

    def extract_materials(self, txd_file: str, output_dir: str, format: str = 'png',
                         deps: DependencyManager = None) -> Dict[str, Any]:
        """Extract textures and materials from TXD file"""

        from .modules.txd_to_materials import TXDToMaterialsConverter

        converter = TXDToMaterialsConverter(self.logger)
        return converter.convert(txd_file, output_dir, format)


