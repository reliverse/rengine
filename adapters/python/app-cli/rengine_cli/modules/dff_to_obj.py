#!/usr/bin/env python3
"""
DFF to OBJ Converter

Converts DFF (RenderWare) model files to OBJ format.

Usage as a module:
    from dff_to_obj import DFFToOBJConverter
    converter = DFFToOBJConverter()
    result = converter.convert('/path/to/model.dff', '/output/model.obj', '/path/to/materials.txd')
"""

import os
import logging
from typing import Dict, Any, Optional

# Import DFF and TXD parsers with fallback for missing dependencies
try:
    from ..common.DFF import dff, Vector, SkinPLG, HAnimPLG, UserData
    from ..common.txd import txd, TextureNative
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


class DFFToOBJConverter:
    """Converter for DFF files to OBJ format"""

    def __init__(self, logger: Optional[logging.Logger] = None):
        self.logger = logger or logging.getLogger(__name__)

    def convert(self, dff_file: str, obj_file: str, txd_file: Optional[str] = None) -> Dict[str, Any]:
        """Convert DFF file to OBJ format with optional TXD materials

        Args:
            dff_file: Path to the DFF file
            obj_file: Path for the output OBJ file
            txd_file: Optional path to TXD texture file

        Returns:
            Dictionary with conversion results and statistics
        """
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