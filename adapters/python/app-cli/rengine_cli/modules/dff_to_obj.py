#!/usr/bin/env python3
"""
DFF to OBJ Converter

Converts DFF (RenderWare) model files to OBJ format.

Usage as a module:
    from dff_to_obj import DFFToOBJConverter
    converter = DFFToOBJConverter()
    result = converter.convert('/path/to/model.dff', '/output/model.obj', '/path/to/materials.txd')
"""

import csv
import logging
import os
from pathlib import Path
from typing import Any, Dict, List, Optional

# Import DFF and TXD parsers with fallback for missing dependencies
try:
    from ..common.DFF import HAnimPLG, SkinPLG, UserData, Vector, dff
    from ..common.txd import TextureNative, txd

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
            raise Exception(
                "DFF support not available - DFF/TXD modules failed to import"
            )

    class txd:
        def __init__(self):
            self.native_textures = []
            self.rw_version = 0
            self.device_id = 0

        def load_file(self, path):
            raise Exception(
                "TXD support not available - DFF/TXD modules failed to import"
            )

    Vector = SkinPLG = HAnimPLG = UserData = TextureNative = None
    DFF_TXD_SUPPORT = False


class DFFToOBJConverter:
    """Converter for DFF files to OBJ format"""

    # Class-level cache for CSV data (shared across all instances)
    _csv_cache = None
    _csv_loaded = False

    def __init__(self, logger: Optional[logging.Logger] = None):
        self.logger = logger or logging.getLogger(__name__)
        # Load CSV data once at class level for all instances
        if not DFFToOBJConverter._csv_loaded:
            DFFToOBJConverter._load_csv_cache(self.logger)
            DFFToOBJConverter._csv_loaded = True

    @classmethod
    def _load_csv_cache(cls, logger: logging.Logger) -> None:
        """Load model data from GTA SA CSV file into class-level cache"""
        csv_path = Path("/home/blefnk/Documents/reliverse/rengine/preloaded/gta-sa.csv")
        if not csv_path.exists():
            logger.debug("GTA SA CSV file not found, continuing without CSV data")
            cls._csv_cache = {}
            return

        try:
            csv_cache = {}
            with open(csv_path, encoding="utf-8") as f:
                reader = csv.DictReader(f)
                for row in reader:
                    try:
                        model_id = int(row["id"])
                        model_name = row.get("name", "").strip()
                        model_file = row.get("modelFile", "").strip()

                        # Create lookup by both model name and filename for faster access
                        if model_name:
                            csv_cache[model_name] = {
                                "id": model_id,
                                "model_name": model_name,
                                "materials_file": row.get("materialsFile", "").strip(),
                                "definition_file": row.get(
                                    "definitionFile", ""
                                ).strip(),
                                "model_file": model_file,
                                "has_collision": row.get("hasCollision", "No") == "Yes",
                                "breaks_on_hit": row.get("breaksOnHit", "No") == "Yes",
                                "has_animation": row.get("hasAnimation", "No") == "Yes",
                                "radius": float(row.get("radius", 0))
                                if row.get("radius")
                                else 0,
                            }

                        # Also index by filename if different from model name
                        if model_file and model_file != model_name + ".dff":
                            csv_cache[model_file] = csv_cache.get(model_name, {})

                    except (ValueError, KeyError) as e:
                        logger.debug(f"Failed to parse CSV row: {row}, error: {e}")
                        continue

            cls._csv_cache = csv_cache
            logger.info(f"Loaded {len(csv_cache)} models into CSV cache")
            cls._csv_loaded = True

        except Exception as e:
            logger.warning(f"Failed to load CSV file {csv_path}: {e}")
            cls._csv_cache = {}

    @classmethod
    def _get_model_info_from_cache(cls, dff_filename: str) -> Optional[Dict[str, Any]]:
        """Get model information from cached CSV data based on DFF filename"""
        if cls._csv_cache is None:
            return None

        # Try exact filename match first
        if dff_filename in cls._csv_cache:
            return cls._csv_cache[dff_filename]

        # Try model name match (without .dff extension)
        model_name = os.path.splitext(dff_filename)[0]
        if model_name in cls._csv_cache:
            return cls._csv_cache[model_name]

        return None

    def convert(
        self,
        dff_file: str,
        obj_file: str,
        txd_file: Optional[str] = None,
        use_csv: bool = True,
    ) -> Dict[str, Any]:
        """Convert DFF file to OBJ format with optional TXD materials

        Args:
            dff_file: Path to the DFF file
            obj_file: Path for the output OBJ file
            txd_file: Optional path to TXD texture file
            use_csv: Whether to use GTA SA CSV data for better model names and TXD paths

        Returns:
            Dictionary with conversion results and statistics
        """
        # Check for required dependencies at runtime
        if not DFF_TXD_SUPPORT:
            raise Exception("DFF support not available - required libraries not found")

        # Get model info from cached CSV data for better naming and TXD detection
        dff_filename = os.path.basename(dff_file)
        csv_model_info: Optional[Dict[str, Any]] = None
        if use_csv and DFFToOBJConverter._csv_cache is not None:
            csv_model_info = DFFToOBJConverter._get_model_info_from_cache(dff_filename)

        # Load DFF file - this requires DFF_TXD_SUPPORT to be True
        dff_data = dff()
        dff_data.load_file(dff_file)

        # Load TXD file if provided, or try to find it from CSV data
        txd_data = None
        if txd_file:
            # User explicitly provided TXD file
            txd_data = txd()
            txd_data.load_file(txd_file)
        else:
            # Try to auto-detect TXD file from CSV data
            if csv_model_info and csv_model_info.get("materials_file"):
                materials_file = csv_model_info["materials_file"]
                if materials_file:  # Additional check to ensure it's not empty
                    dff_dir = os.path.dirname(dff_file)
                    txd_path = os.path.join(dff_dir, materials_file)
                    if os.path.exists(txd_path):
                        try:
                            txd_data = txd()
                            txd_data.load_file(txd_path)
                            self.logger.info(f"Auto-loaded TXD file: {materials_file}")
                        except Exception as e:
                            self.logger.debug(
                                f"Failed to load auto-detected TXD file {txd_path}: {e}"
                            )
                    else:
                        self.logger.debug(f"TXD file from CSV not found: {txd_path}")

        # Convert to OBJ
        obj_content = self._convert_dff_to_obj(
            dff_data, txd_data, dff_file, csv_model_info
        )

        # Write OBJ file
        with open(obj_file, "w") as f:
            f.write(obj_content)

        result = {
            "obj_file": obj_file,
            "meshes": len(dff_data.atomic_list),
            "csv_used": DFFToOBJConverter._csv_loaded,
            "csv_model_info": csv_model_info,
        }

        # Create MTL file if materials are enabled
        if txd_data:
            mtl_content = self._create_mtl_file(txd_data, os.path.splitext(obj_file)[0])
            mtl_file = os.path.splitext(obj_file)[0] + ".mtl"
            with open(mtl_file, "w") as f:
                f.write(mtl_content)

            result["mtl_file"] = mtl_file

            # Extract textures
            textures_extracted = self._extract_textures(
                txd_data, os.path.dirname(obj_file), "png"
            )
            result["textures_extracted"] = textures_extracted

        return result

    def _convert_dff_to_obj(self, dff_data, txd_data, dff_file, csv_model_info=None):
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
            mtl_filename = os.path.splitext(os.path.basename(dff_file))[0] + ".mtl"
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

            # Use CSV model name if available, otherwise use frame name
            object_name = frame.name
            if csv_model_info and csv_model_info.get("model_name"):
                object_name = csv_model_info["model_name"]

            obj_lines.append(f"# Atomic {atomic_index}: {object_name}")
            obj_lines.append(f"o {object_name}")
            obj_lines.append("")

            local_vertex_offset = global_vertex_offset
            local_normal_offset = global_normal_offset

            # Write vertices
            for vertex in geometry.vertices:
                transformed_vertex = vertex
                obj_lines.append(
                    f"v {transformed_vertex.x:.6f} {transformed_vertex.y:.6f} {transformed_vertex.z:.6f}"
                )

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
                    obj_lines.append(
                        f"vt {uv.u:.6f} {1.0 - uv.v:.6f}"
                    )  # Flip V coordinate

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
                    if (
                        txd_data
                        and geometry.materials
                        and mat_idx < len(geometry.materials)
                    ):
                        material = geometry.materials[mat_idx]
                        if material.is_textured and material.textures:
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
                            obj_lines.append(
                                f"f {v1}/{v1}/{n1} {v2}/{v2}/{n2} {v3}/{v3}/{n3}"
                            )
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
            mtl_lines.append("d 1.000000")  # Dissolve (transparency)
            mtl_lines.append("illum 2")  # Illumination model

            # Texture maps
            if hasattr(texture, "name") and texture.name:
                texture_filename = f"{texture.name}.png"
                mtl_lines.append(f"map_Kd {texture_filename}")

            mtl_lines.append("")

        return "\n".join(mtl_lines)

    def _extract_textures(self, txd_data, output_dir, format="png"):
        """Extract textures from TXD to files - optimized for batch processing"""
        try:
            from PIL import Image
        except ImportError:
            self.logger.debug("PIL not available, skipping texture extraction")
            return 0

        extracted_count = 0

        # Pre-check all textures to avoid partial failures
        valid_textures = []
        for texture in txd_data.native_textures:
            try:
                # Quick validation before processing
                rgba_data = texture.to_rgba(0)
                if rgba_data and texture.get_width(0) > 0 and texture.get_height(0) > 0:
                    valid_textures.append((texture, rgba_data))
            except Exception:
                continue

        # Batch process valid textures
        for texture, rgba_data in valid_textures:
            try:
                width = texture.get_width(0)
                height = texture.get_height(0)

                # Create PIL Image from RGBA data
                image = Image.frombytes("RGBA", (width, height), rgba_data)

                # Save as PNG with optimized settings
                texture_path = os.path.join(output_dir, f"{texture.name}.png")
                image.save(texture_path, "PNG", optimize=True)
                extracted_count += 1

            except Exception as e:
                self.logger.debug(f"Failed to extract texture {texture.name}: {str(e)}")

        return extracted_count

    @classmethod
    def batch_convert(
        cls,
        dff_files: List[str],
        output_dir: str,
        use_csv: bool = True,
        max_workers: int = 4,
        logger: Optional[logging.Logger] = None,
        skip_existing: bool = True,
    ) -> Dict[str, Any]:
        """Batch convert multiple DFF files with optimized parallel processing"""
        import concurrent.futures

        logger = logger or logging.getLogger(__name__)

        # Ensure output directory exists
        os.makedirs(output_dir, exist_ok=True)

        # Initialize CSV cache once for all workers
        if use_csv and not cls._csv_loaded:
            cls._load_csv_cache(logger)

        # Filter out files that already exist if skip_existing is True
        files_to_convert = []
        skipped_files = []

        for dff_file in dff_files:
            base_name = os.path.splitext(os.path.basename(dff_file))[0]
            obj_file = os.path.join(output_dir, f"{base_name}.obj")

            if skip_existing and os.path.exists(obj_file):
                skipped_files.append(dff_file)
                logger.debug(
                    f"⏭️  Skipped (already exists): {os.path.basename(dff_file)}"
                )
            else:
                files_to_convert.append(dff_file)

        results = {
            "total_files": len(dff_files),
            "files_to_convert": len(files_to_convert),
            "skipped_files": len(skipped_files),
            "successful": 0,
            "failed": 0,
            "errors": [],
        }

        if not files_to_convert:
            logger.info("All files already exist, nothing to convert")
            return results

        def convert_single_file(dff_file: str) -> tuple[bool, str, Optional[str]]:
            """Convert a single file and return result"""
            try:
                converter = cls(logger=logger)
                base_name = os.path.splitext(os.path.basename(dff_file))[0]
                obj_file = os.path.join(output_dir, f"{base_name}.obj")

                result = converter.convert(dff_file, obj_file, use_csv=use_csv)
                return True, dff_file, None
            except Exception as e:
                return False, dff_file, str(e)

        # Use ThreadPoolExecutor for I/O bound operations
        with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
            # Submit all tasks
            future_to_file = {
                executor.submit(convert_single_file, dff_file): dff_file
                for dff_file in files_to_convert
            }

            # Process results as they complete
            for future in concurrent.futures.as_completed(future_to_file):
                success, filename, error = future.result()
                if success:
                    results["successful"] += 1
                    logger.info(f"✓ Converted: {os.path.basename(filename)}")
                else:
                    results["failed"] += 1
                    results["errors"].append({"file": filename, "error": error})
                    logger.warning(f"✗ Failed: {os.path.basename(filename)} - {error}")

        logger.info(
            f"Batch conversion complete: {results['successful']} successful, {results['failed']} failed, {results['skipped_files']} skipped"
        )
        return results
