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
            'chunks': []
        }

        # Basic chunk analysis
        pos = 0
        while pos < len(data) - 12:
            try:
                chunk_type, chunk_size, chunk_version = struct.unpack('<III', data[pos:pos+12])
                analysis['chunks'].append({
                    'type': f"0x{chunk_type:08X}",
                    'size': chunk_size,
                    'version': f"0x{chunk_version:08X}",
                    'offset': pos
                })
                pos += chunk_size + 12
            except:
                break

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
            'size': len(data),
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
        content = self.read_file_text(file_path)

        # This is a simplified IPL loader - actual implementation would use Rengine CLI
        lines = [line.strip() for line in content.split('\n') if line.strip() and not line.startswith('#')]

        # Basic parsing to count sections
        sections = {}
        current_section = None

        for line in lines:
            if line.endswith(','):
                line = line[:-1]

            if line in ['inst', 'zone', 'occl', 'pick', 'path', 'tcyc', 'mult', 'grge', 'enex', 'cars', 'jump']:
                current_section = line
                sections[current_section] = []
            elif current_section and line:
                sections[current_section].append(line.split(','))

        info = {
            'file': file_path,
            'size': len(content),
            'instances': len(sections.get('inst', [])),
            'zones': len(sections.get('zone', [])),
            'occlusions': len(sections.get('occl', []))
        }

        return info

    def analyze_structure(self, file_path: str, deps: DependencyManager) -> Dict[str, Any]:
        """Analyze IPL map structure"""
        content = self.read_file_text(file_path)
        lines = content.split('\n')
        sections = {}
        current_section = None

        for line in lines:
            line = line.strip()
            if line.startswith('#') or not line:
                continue
            if line.endswith(','):
                line = line[:-1]

            if line in ['inst', 'zone', 'occl', 'pick', 'path', 'tcyc', 'mult', 'grge', 'enex', 'cars', 'jump']:
                current_section = line
                sections[current_section] = []
            elif current_section and line:
                sections[current_section].append(line.split(','))

        analysis = {
            'file': file_path,
            'sections': {}
        }

        for section, entries in sections.items():
            analysis['sections'][section] = {
                'count': len(entries),
                'sample_entries': entries[:5] if entries else []
            }

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

        if not DFF_TXD_SUPPORT:
            raise Exception("DFF support not available - required libraries not found")

        # Load DFF file
        dff_data = dff()
        dff_data.load_file(dff_file)

        # Load TXD file if provided
        txd_data = None
        if txd_file:
            txd_data = txd()
            txd_data.load_file(txd_file)

        # Convert to OBJ
        obj_content = self._convert_dff_to_obj(dff_data, txd_data, dff_file)

        # Write OBJ file
        with open(obj_file, 'w') as f:
            f.write(obj_content)

        result = {
            'obj_file': obj_file,
            'meshes': len(dff_data.atomic_list)
        }

        # Create MTL file if materials are enabled
        if txd_data:
            mtl_content = self._create_mtl_file(txd_data, os.path.splitext(obj_file)[0])
            mtl_file = os.path.splitext(obj_file)[0] + '.mtl'
            with open(mtl_file, 'w') as f:
                f.write(mtl_content)

            result['mtl_file'] = mtl_file

            # Extract textures
            textures_extracted = self._extract_textures(txd_data, os.path.dirname(obj_file), 'png')
            result['textures_extracted'] = textures_extracted

        return result

    def _convert_dff_to_obj(self, dff_data, txd_data, dff_file):
        """Convert DFF data to OBJ format"""
        obj_lines = []
        obj_lines.append("# OBJ file exported from Rengine DFF Converter")
        obj_lines.append(f"# Original DFF: {os.path.basename(dff_file)}")
        obj_lines.append(f"# RW Version: 0x{dff_data.rw_version:08x}")
        obj_lines.append("")

        global_vertex_offset = 1  # OBJ vertices are 1-indexed
        global_normal_offset = 1
        global_uv_offset = 1

        # Material library reference
        if txd_data:
            mtl_filename = os.path.splitext(os.path.basename(dff_file))[0] + '.mtl'
            obj_lines.append(f"mtllib {mtl_filename}")
            obj_lines.append("")

        # Process each atomic (mesh)
        for atomic_index, atomic in enumerate(dff_data.atomic_list):
            if atomic.frame >= len(dff_data.frame_list):
                continue

            frame = dff_data.frame_list[atomic.frame]
            if atomic.geometry >= len(dff_data.geometry_list):
                continue

            geometry = dff_data.geometry_list[atomic.geometry]

            obj_lines.append(f"# Atomic {atomic_index}: {frame.name}")
            obj_lines.append(f"o {frame.name}")
            obj_lines.append("")

            local_vertex_offset = global_vertex_offset
            local_normal_offset = global_normal_offset
            local_uv_offset = global_uv_offset

            # Write vertices
            for vertex in geometry.vertices:
                transformed_vertex = vertex
                obj_lines.append(f"v {transformed_vertex.x:.6f} {transformed_vertex.y:.6f} {transformed_vertex.z:.6f}")

            global_vertex_offset += len(geometry.vertices)
            obj_lines.append("")

            # Write normals
            if geometry.has_normals:
                for normal in geometry.normals:
                    obj_lines.append(f"vn {normal.x:.6f} {normal.y:.6f} {normal.z:.6f}")

                global_normal_offset += len(geometry.normals)
                obj_lines.append("")

            # Write UV coordinates for all layers
            for layer_idx, uv_layer in enumerate(geometry.uv_layers):
                if layer_idx >= 2:  # Limit to 2 UV layers
                    break

                obj_lines.append(f"# UV Layer {layer_idx}")
                for uv in uv_layer:
                    obj_lines.append(f"vt {uv.u:.6f} {1.0 - uv.v:.6f}")  # Flip V coordinate

                global_uv_offset += len(uv_layer)
                obj_lines.append("")

            # Write faces with materials
            if geometry.triangles:
                obj_lines.append("s 1")  # Enable smooth shading

                # Group faces by material
                material_groups = {}
                for triangle in geometry.triangles:
                    mat_idx = triangle.material
                    if mat_idx not in material_groups:
                        material_groups[mat_idx] = []
                    material_groups[mat_idx].append(triangle)

                # Write faces grouped by material
                for mat_idx, triangles in material_groups.items():
                    if txd_data and geometry.materials and mat_idx < len(geometry.materials):
                        material = geometry.materials[mat_idx]
                        if material.is_textured and material.textures:
                            texture_name = material.textures[0].name
                            obj_lines.append(f"usemtl material_{mat_idx}")
                        else:
                            obj_lines.append(f"usemtl material_{mat_idx}")

                    for triangle in triangles:
                        v1 = triangle.a + local_vertex_offset
                        v2 = triangle.b + local_vertex_offset
                        v3 = triangle.c + local_vertex_offset

                        if geometry.has_normals and geometry.uv_layers:
                            n1 = triangle.a + local_normal_offset
                            n2 = triangle.b + local_normal_offset
                            n3 = triangle.c + local_normal_offset
                            obj_lines.append(f"f {v1}/{v1}/{n1} {v2}/{v2}/{n2} {v3}/{v3}/{n3}")
                        elif geometry.uv_layers:
                            obj_lines.append(f"f {v1}/{v1} {v2}/{v2} {v3}/{v3}")
                        elif geometry.has_normals:
                            n1 = triangle.a + local_normal_offset
                            n2 = triangle.b + local_normal_offset
                            n3 = triangle.c + local_normal_offset
                            obj_lines.append(f"f {v1}//{n1} {v2}//{n2} {v3}//{n3}")
                        else:
                            obj_lines.append(f"f {v1} {v2} {v3}")

                obj_lines.append("")

        return "\n".join(obj_lines)

    def _create_mtl_file(self, txd_data, base_name):
        """Create MTL material file from TXD data"""
        mtl_lines = []
        mtl_lines.append("# MTL file exported from Rengine DFF Converter")
        mtl_lines.append("")

        for tex_idx, texture in enumerate(txd_data.native_textures):
            material_name = f"material_{tex_idx}"

            mtl_lines.append(f"newmtl {material_name}")

            # Basic material properties
            mtl_lines.append("Ns 96.078431")  # Specular exponent
            mtl_lines.append("Ka 1.000000 1.000000 1.000000")  # Ambient color
            mtl_lines.append("Kd 0.640000 0.640000 0.640000")  # Diffuse color
            mtl_lines.append("Ks 0.500000 0.500000 0.500000")  # Specular color
            mtl_lines.append("Ke 0.000000 0.000000 0.000000")  # Emissive color
            mtl_lines.append("Ni 1.000000")  # Optical density
            mtl_lines.append("d 1.000000")   # Dissolve (transparency)
            mtl_lines.append("illum 2")      # Illumination model

            # Texture maps
            if hasattr(texture, 'name') and texture.name:
                texture_filename = f"{texture.name}.png"
                mtl_lines.append(f"map_Kd {texture_filename}")

            mtl_lines.append("")

        return "\n".join(mtl_lines)

    def _extract_textures(self, txd_data, output_dir, format='png'):
        """Extract textures from TXD to files"""
        try:
            from PIL import Image
        except ImportError:
            self.logger.warning("PIL not available, skipping texture extraction")
            return 0

        extracted_count = 0
        for texture in txd_data.native_textures:
            try:
                # Get first mipmap level (highest quality)
                rgba_data = texture.to_rgba(0)
                if not rgba_data:
                    continue

                width = texture.get_width(0)
                height = texture.get_height(0)

                if width == 0 or height == 0:
                    continue

                # Create PIL Image from RGBA data
                image = Image.frombytes('RGBA', (width, height), rgba_data)

                # Save as PNG
                texture_path = os.path.join(output_dir, f"{texture.name}.png")
                image.save(texture_path, "PNG")
                extracted_count += 1

            except Exception as e:
                self.logger.warning(f"Failed to extract texture {texture.name}: {str(e)}")

        return extracted_count


class TXDConverterHandler:
    """Handler for extracting materials and textures from TXD files"""

    def __init__(self, logger: logging.Logger):
        self.logger = logger

    def extract_materials(self, txd_file: str, output_dir: str, format: str = 'png',
                         deps: DependencyManager = None) -> Dict[str, Any]:
        """Extract textures and materials from TXD file"""

        if not DFF_TXD_SUPPORT:
            raise Exception("TXD support not available - required libraries not found")

        # Load TXD file
        txd_data = txd()
        txd_data.load_file(txd_file)

        # Create output directory
        os.makedirs(output_dir, exist_ok=True)

        extracted_count = 0

        try:
            from PIL import Image
        except ImportError:
            return {'error': 'PIL library not available for image processing'}

        # Extract each texture
        for texture in txd_data.native_textures:
            try:
                # Get first mipmap level (highest quality)
                rgba_data = texture.to_rgba(0)
                if not rgba_data:
                    continue

                width = texture.get_width(0)
                height = texture.get_height(0)

                if width == 0 or height == 0:
                    continue

                # Create PIL Image from RGBA data
                image = Image.frombytes('RGBA', (width, height), rgba_data)

                # Save in requested format
                ext = format.lower()
                if ext == 'png':
                    texture_path = os.path.join(output_dir, f"{texture.name}.png")
                    image.save(texture_path, "PNG")
                elif ext == 'jpg' or ext == 'jpeg':
                    texture_path = os.path.join(output_dir, f"{texture.name}.jpg")
                    # Convert RGBA to RGB for JPEG
                    rgb_image = Image.new('RGB', image.size, (255, 255, 255))
                    rgb_image.paste(image, mask=image.split()[-1])  # Use alpha as mask
                    rgb_image.save(texture_path, "JPEG", quality=95)
                else:
                    # Default to PNG
                    texture_path = os.path.join(output_dir, f"{texture.name}.png")
                    image.save(texture_path, "PNG")

                extracted_count += 1

            except Exception as e:
                self.logger.warning(f"Failed to extract texture {texture.name}: {str(e)}")

        # Create material file
        mtl_content = self._create_mtl_file(txd_data, os.path.basename(txd_file))
        mtl_file = os.path.join(output_dir, f"{os.path.splitext(os.path.basename(txd_file))[0]}.mtl")
        with open(mtl_file, 'w') as f:
            f.write(mtl_content)

        result = {
            'extracted': extracted_count,
            'mtl_file': mtl_file,
            'output_dir': output_dir
        }

        if extracted_count == 0:
            result['message'] = 'No textures were extracted'

        return result

    def _create_mtl_file(self, txd_data, txd_filename):
        """Create MTL material file from TXD data"""
        mtl_lines = []
        mtl_lines.append("# MTL file exported from Rengine TXD Converter")
        mtl_lines.append(f"# Original TXD: {txd_filename}")
        mtl_lines.append("")

        for tex_idx, texture in enumerate(txd_data.native_textures):
            material_name = f"material_{tex_idx}"

            mtl_lines.append(f"newmtl {material_name}")

            # Basic material properties
            mtl_lines.append("Ns 96.078431")  # Specular exponent
            mtl_lines.append("Ka 1.000000 1.000000 1.000000")  # Ambient color
            mtl_lines.append("Kd 0.640000 0.640000 0.640000")  # Diffuse color
            mtl_lines.append("Ks 0.500000 0.500000 0.500000")  # Specular color
            mtl_lines.append("Ke 0.000000 0.000000 0.000000")  # Emissive color
            mtl_lines.append("Ni 1.000000")  # Optical density
            mtl_lines.append("d 1.000000")   # Dissolve (transparency)
            mtl_lines.append("illum 2")      # Illumination model

            # Texture maps
            if hasattr(texture, 'name') and texture.name:
                texture_filename = f"{texture.name}.png"
                mtl_lines.append(f"map_Kd {texture_filename}")

            mtl_lines.append("")

        return "\n".join(mtl_lines)

class IPLToGodotHandler:
    """Handler for converting IPL files to Godot scene files"""

    def __init__(self, logger: logging.Logger):
        self.logger = logger

    def convert_to_godot(self, ipl_file: str, output_file: str, template_file: Optional[str] = None) -> Dict[str, Any]:
        """Convert IPL file to Godot .tscn scene file"""

        # Parse IPL file
        ipl_data = self._parse_ipl_file(ipl_file)

        # Generate Godot scene
        scene_content = self._generate_godot_scene(ipl_data, ipl_file, template_file)

        # Write scene file
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(scene_content)

        result = {
            'scene_file': output_file,
            'ipl_file': ipl_file
        }

        # Add statistics
        if 'sections' in ipl_data:
            sections = ipl_data['sections']
            if 'inst' in sections:
                result['instances'] = sections['inst']['count']
            if 'zone' in sections:
                result['zones'] = sections['zone']['count']

        return result

    def convert_directory_to_godot_project(self, input_dir: str, output_dir: str,
                                         project_name: str = 'IPLMap',
                                         template_file: Optional[str] = None) -> Dict[str, Any]:
        """Convert all IPL files in a directory to a complete Godot project"""

        import os
        import glob

        # Find all IPL files
        ipl_files = []
        for pattern in ['*.ipl', '*.IPL']:
            ipl_files.extend(glob.glob(os.path.join(input_dir, pattern)))

        if not ipl_files:
            raise Exception(f"No IPL files found in directory: {input_dir}")

        # Parse all IPL files
        all_ipl_data = {}
        total_instances = 0
        total_zones = 0

        for ipl_file in ipl_files:
            try:
                ipl_data = self._parse_ipl_file(ipl_file)
                filename = os.path.basename(ipl_file)
                all_ipl_data[filename] = ipl_data

                # Count totals
                if 'sections' in ipl_data:
                    sections = ipl_data['sections']
                    if 'inst' in sections:
                        total_instances += sections['inst']['count']
                    if 'zone' in sections:
                        total_zones += sections['zone']['count']

                self.logger.info(f"Parsed IPL file: {filename}")

            except Exception as e:
                self.logger.warning(f"Failed to parse IPL file {ipl_file}: {e}")

        # Create Godot project structure
        self._create_godot_project_structure(output_dir, project_name)

        # Generate main scene combining all IPL data
        main_scene_content = self._generate_combined_godot_scene(all_ipl_data, project_name)
        main_scene_path = os.path.join(output_dir, 'main.tscn')

        with open(main_scene_path, 'w', encoding='utf-8') as f:
            f.write(main_scene_content)

        # Generate individual scenes for each IPL file (optional)
        scenes_dir = os.path.join(output_dir, 'scenes')
        os.makedirs(scenes_dir, exist_ok=True)

        for filename, ipl_data in all_ipl_data.items():
            scene_name = os.path.splitext(filename)[0] + '.tscn'
            scene_path = os.path.join(scenes_dir, scene_name)
            scene_content = self._generate_godot_scene(ipl_data, filename, template_file)

            with open(scene_path, 'w', encoding='utf-8') as f:
                f.write(scene_content)

        result = {
            'project_dir': output_dir,
            'project_name': project_name,
            'ipl_files': len(all_ipl_data),
            'total_instances': total_instances,
            'total_zones': total_zones,
            'main_scene': main_scene_path,
            'individual_scenes': len(all_ipl_data)
        }

        return result

    def _create_godot_project_structure(self, output_dir: str, project_name: str):
        """Create basic Godot project structure"""
        import os

        # Create output directory
        os.makedirs(output_dir, exist_ok=True)

        # Create project.godot
        project_godot_content = f"""; Engine configuration file.
; It's best edited using the editor UI and not directly,
; since the parameters that go here are not all obvious.
;
; Format:
;   [section] ; section goes between []
;   param=value ; assign values to parameters

config_version=5

[application]

config/name="{project_name}"
run/main_scene="res://main.tscn"
config/features=PackedStringArray("4.5", "Forward Plus")
config/icon="res://icon.svg"
"""

        project_godot_path = os.path.join(output_dir, 'project.godot')
        with open(project_godot_path, 'w', encoding='utf-8') as f:
            f.write(project_godot_content)

        # Create basic icon.svg (minimal)
        icon_content = '''<svg width="64" height="64" xmlns="http://www.w3.org/2000/svg">
  <rect width="64" height="64" fill="#478cbf"/>
  <text x="32" y="40" font-family="Arial" font-size="40" fill="white" text-anchor="middle">G</text>
</svg>'''

        icon_path = os.path.join(output_dir, 'icon.svg')
        with open(icon_path, 'w', encoding='utf-8') as f:
            f.write(icon_content)

    def _generate_combined_godot_scene(self, all_ipl_data: Dict[str, Dict], project_name: str) -> str:
        """Generate a combined Godot scene with all IPL data"""

        scene_lines = []
        scene_lines.append('[gd_scene load_steps=2 format=3]')
        scene_lines.append('')

        # Add script subresource
        scene_lines.append('[sub_resource type="GDScript" id="1"]')
        scene_lines.append('script/source = """')
        scene_lines.append('extends Node')
        scene_lines.append('')
        scene_lines.append(f'# {project_name} - Combined IPL Map Data')
        scene_lines.append(f'# Generated from {len(all_ipl_data)} IPL files')
        scene_lines.append('')
        scene_lines.append('func _ready():')
        scene_lines.append('\tpass')
        scene_lines.append('"""')
        scene_lines.append('')

        # Root node
        scene_lines.append(f'[node name="{project_name}" type="Node"]')
        scene_lines.append('script = SubResource("1")')
        scene_lines.append('')

        # Add nodes for each IPL file
        ipl_index = 0
        total_instances = 0

        for ipl_filename, ipl_data in all_ipl_data.items():
            ipl_name = os.path.splitext(ipl_filename)[0]
            scene_lines.append(f'[node name="{ipl_name}" type="Node" parent="."]')
            scene_lines.append('')

            # Add instances from this IPL
            if 'sections' in ipl_data and 'inst' in ipl_data['sections']:
                for entry in ipl_data['sections']['inst']['entries']:
                    if 'model_name' in entry:
                        total_instances += 1
                        node_name = f"{entry['model_name']}_{total_instances}"
                        scene_lines.append(f'[node name="{node_name}" type="Node3D" parent="{ipl_name}"]')
                        scene_lines.append(f'transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {entry["pos_x"]:.6f}, {entry["pos_y"]:.6f}, {entry["pos_z"]:.6f})')

                        # Add metadata
                        scene_lines.append(f'metadata/model_id = {entry["id"]}')
                        scene_lines.append(f'metadata/model_name = "{entry["model_name"]}"')
                        scene_lines.append(f'metadata/interior = {entry["interior"]}')
                        scene_lines.append(f'metadata/lod = {entry["lod"]}')
                        scene_lines.append(f'metadata/ipl_source = "{ipl_filename}"')
                        scene_lines.append('')

            # Add zones from this IPL
            if 'sections' in ipl_data and 'zone' in ipl_data['sections']:
                for entry in ipl_data['sections']['zone']['entries']:
                    if 'name' in entry:
                        center_x = (entry['min_x'] + entry['max_x']) / 2
                        center_y = (entry['min_y'] + entry['max_y']) / 2
                        center_z = (entry['min_z'] + entry['max_z']) / 2
                        size_x = abs(entry['max_x'] - entry['min_x'])
                        size_y = abs(entry['max_y'] - entry['min_y'])
                        size_z = abs(entry['max_z'] - entry['max_z'])

                        zone_name = f"{entry['name']}_zone"
                        scene_lines.append(f'[node name="{zone_name}" type="Area3D" parent="{ipl_name}"]')
                        scene_lines.append(f'transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {center_x:.6f}, {center_y:.6f}, {center_z:.6f})')

                        # Add metadata
                        scene_lines.append(f'metadata/zone_type = {entry["type"]}')
                        scene_lines.append(f'metadata/level = "{entry["level"]}"')
                        scene_lines.append(f'metadata/text = "{entry["text"]}"')
                        scene_lines.append(f'metadata/ipl_source = "{ipl_filename}"')

                        # Add CollisionShape3D as child
                        scene_lines.append(f'[node name="CollisionShape3D" type="CollisionShape3D" parent="{zone_name}"]')
                        scene_lines.append('[sub_resource type="BoxShape3D" id="2"]')
                        scene_lines.append(f'size = Vector3({size_x:.6f}, {size_y:.6f}, {size_z:.6f})')
                        scene_lines.append('shape = SubResource("2")')
                        scene_lines.append('')

        return '\n'.join(scene_lines)

    def _parse_ipl_file(self, ipl_file: str) -> Dict[str, Any]:
        """Parse IPL file and extract sections"""
        try:
            # First try to read as binary IPL file
            with open(ipl_file, 'rb') as f:
                header = f.read(4)
                if header == b'bnry':
                    return self._parse_binary_ipl_file(ipl_file)
        except:
            pass

        # Fall back to text IPL parsing
        with open(ipl_file, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()

        lines = content.split('\n')
        sections = {}
        current_section = None

        for line in lines:
            line = line.strip()
            if line.startswith('#') or not line:
                continue
            if line.endswith(','):
                line = line[:-1]

            # Check for section headers
            if line in ['inst', 'zone', 'occl', 'pick', 'path', 'tcyc', 'mult', 'grge', 'enex', 'cars', 'jump']:
                current_section = line
                sections[current_section] = {'entries': [], 'count': 0}
            elif current_section and line:
                # Parse entry based on section type
                entry = self._parse_ipl_entry(current_section, line)
                if entry:
                    sections[current_section]['entries'].append(entry)
                    sections[current_section]['count'] += 1

        return {'file': ipl_file, 'sections': sections}

    def _parse_binary_ipl_file(self, ipl_file: str) -> Dict[str, Any]:
        """Parse binary IPL file (GTA SA format)"""
        import struct

        sections = {'inst': {'entries': [], 'count': 0}}

        with open(ipl_file, 'rb') as f:
            data = f.read()

        if len(data) < 8 or data[:4] != b'bnry':
            return {'file': ipl_file, 'sections': sections}

        # Read number of instances
        num_instances = struct.unpack('<I', data[4:8])[0]

        # Instances start at offset 0x4C in GTA SA IPL files (after 12 bytes of padding)
        offset = 0x4C

        for i in range(min(num_instances, 1000)):  # Limit to prevent infinite loops
            try:
                if offset + 64 > len(data):  # Need at least 64 bytes for instance + name
                    break

                # Read instance data (GTA SA binary IPL format)
                # 7 floats (28 bytes) + 3 ints (12 bytes) = 40 bytes for instance data
                instance_data = data[offset:offset+40]
                pos_x, pos_y, pos_z, rot_x, rot_y, rot_z, rot_w, model_id, interior, lod = struct.unpack('<fffffffIII', instance_data)

                # Read model name (24 bytes after instance data)
                name_offset = offset + 40
                model_name_data = data[name_offset:name_offset+24]
                model_name = model_name_data.split(b'\x00')[0].decode('ascii', errors='ignore')

                entry = {
                    'id': int(model_id),
                    'model_name': model_name,
                    'interior': int(interior),
                    'pos_x': pos_x,
                    'pos_y': pos_y,
                    'pos_z': pos_z,
                    'rot_x': rot_x,
                    'rot_y': rot_y,
                    'rot_z': rot_z,
                    'rot_w': rot_w,
                    'lod': int(lod)
                }
                sections['inst']['entries'].append(entry)
                sections['inst']['count'] += 1

                offset += 64  # Each instance is 64 bytes (40 + 24)

            except (struct.error, UnicodeDecodeError):
                break

        return {'file': ipl_file, 'sections': sections}

    def _parse_ipl_entry(self, section: str, line: str) -> Optional[Dict[str, Any]]:
        """Parse a single IPL entry based on section type"""
        try:
            parts = line.split(',')

            if section == 'inst':  # Instances
                if len(parts) >= 12:
                    return {
                        'id': int(parts[0]),
                        'model_name': parts[1].strip(),
                        'interior': int(parts[2]) if parts[2].strip() else 0,
                        'pos_x': float(parts[3]),
                        'pos_y': float(parts[4]),
                        'pos_z': float(parts[5]),
                        'rot_x': float(parts[6]),
                        'rot_y': float(parts[7]),
                        'rot_z': float(parts[8]),
                        'rot_w': float(parts[9]) if len(parts) > 9 else 1.0,
                        'lod': int(parts[10]) if len(parts) > 10 and parts[10].strip() else -1
                    }

            elif section == 'zone':  # Zones
                if len(parts) >= 10:
                    return {
                        'name': parts[0].strip(),
                        'type': int(parts[1]),
                        'min_x': float(parts[2]),
                        'min_y': float(parts[3]),
                        'min_z': float(parts[4]),
                        'max_x': float(parts[5]),
                        'max_y': float(parts[6]),
                        'max_z': float(parts[7]),
                        'level': parts[8].strip() if len(parts) > 8 else '',
                        'text': parts[9].strip() if len(parts) > 9 else ''
                    }

            # For other sections, just return the raw data
            return {'raw': parts}

        except (ValueError, IndexError):
            self.logger.warning(f"Failed to parse IPL entry: {line}")
            return None

    def _generate_godot_scene(self, ipl_data: Dict[str, Any], ipl_file: str, template_file: Optional[str] = None) -> str:
        """Generate Godot .tscn scene file content"""

        scene_lines = []
        scene_lines.append('[gd_scene load_steps=3 format=3]')
        scene_lines.append('')

        # Add script subresource
        scene_lines.append('[sub_resource type="GDScript" id="1"]')
        scene_lines.append('script/source = """')
        scene_lines.append('extends Node')
        scene_lines.append('')
        scene_lines.append('# IPL Map Data')
        scene_lines.append(f'# Original file: {os.path.basename(ipl_file)}')
        scene_lines.append('')
        scene_lines.append('func _ready():')
        scene_lines.append('\tpass')
        scene_lines.append('"""')
        scene_lines.append('')

        # Root node
        scene_lines.append('[node name="IPLMap" type="Node"]')
        scene_lines.append('script = SubResource("1")')
        scene_lines.append('')

        # Generate nodes for instances
        if 'sections' in ipl_data and 'inst' in ipl_data['sections']:
            instance_count = 0
            for entry in ipl_data['sections']['inst']['entries']:
                if 'model_name' in entry:
                    instance_count += 1
                    node_name = f"{entry['model_name']}_{instance_count}"
                    scene_lines.append(f'[node name="{node_name}" type="Node3D" parent="."]')
                    scene_lines.append(f'transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {entry["pos_x"]:.6f}, {entry["pos_y"]:.6f}, {entry["pos_z"]:.6f})')

                    # Add metadata
                    scene_lines.append(f'metadata/model_id = {entry["id"]}')
                    scene_lines.append(f'metadata/model_name = "{entry["model_name"]}"')
                    scene_lines.append(f'metadata/interior = {entry["interior"]}')
                    scene_lines.append(f'metadata/lod = {entry["lod"]}')
                    scene_lines.append('')

        # Generate nodes for zones (as Area3D)
        if 'sections' in ipl_data and 'zone' in ipl_data['sections']:
            zone_count = 0
            for entry in ipl_data['sections']['zone']['entries']:
                if 'name' in entry:
                    zone_count += 1
                    center_x = (entry['min_x'] + entry['max_x']) / 2
                    center_y = (entry['min_y'] + entry['max_y']) / 2
                    center_z = (entry['min_z'] + entry['max_z']) / 2
                    size_x = abs(entry['max_x'] - entry['min_x'])
                    size_y = abs(entry['max_y'] - entry['min_y'])
                    size_z = abs(entry['max_z'] - entry['max_z'])

                    zone_name = f"{entry['name']}_{zone_count}"
                    scene_lines.append(f'[node name="{zone_name}" type="Area3D" parent="."]')
                    scene_lines.append(f'transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {center_x:.6f}, {center_y:.6f}, {center_z:.6f})')

                    # Add metadata
                    scene_lines.append(f'metadata/zone_type = {entry["type"]}')
                    scene_lines.append(f'metadata/level = "{entry["level"]}"')
                    scene_lines.append(f'metadata/text = "{entry["text"]}"')

                    # Add CollisionShape3D as child
                    scene_lines.append(f'[node name="CollisionShape3D" type="CollisionShape3D" parent="{zone_name}"]')
                    scene_lines.append('[sub_resource type="BoxShape3D" id="2"]')
                    scene_lines.append(f'size = Vector3({size_x:.6f}, {size_y:.6f}, {size_z:.6f})')
                    scene_lines.append('shape = SubResource("2")')
                    scene_lines.append('')

        return '\n'.join(scene_lines)
