#!/usr/bin/env python3
"""
TXD to Materials Converter

Extracts textures and materials from TXD (RenderWare) texture dictionary files.

Usage as a module:
    from txd_to_materials import TXDToMaterialsConverter
    converter = TXDToMaterialsConverter()
    result = converter.convert('/path/to/textures.txd', '/output/directory')
"""

import os
import logging
from typing import Dict, Any, Optional

# Import TXD parser with fallback for missing dependencies
try:
    from ..common.txd import txd, TextureNative
    TXD_SUPPORT = True
except ImportError:
    # Create dummy classes if imports fail
    class txd:
        def __init__(self):
            self.native_textures = []
            self.rw_version = 0
            self.device_id = 0
        def load_file(self, path):
            raise Exception("TXD support not available - TXD module failed to import")

    TextureNative = None
    TXD_SUPPORT = False


class TXDToMaterialsConverter:
    """Converter for TXD files to extracted textures and materials"""

    def __init__(self, logger: Optional[logging.Logger] = None):
        self.logger = logger or logging.getLogger(__name__)

    def convert(self, txd_file: str, output_dir: str, format: str = 'png') -> Dict[str, Any]:
        """Extract textures and materials from TXD file

        Args:
            txd_file: Path to the TXD file
            output_dir: Output directory for extracted files
            format: Image format for textures ('png', 'jpg', etc.)

        Returns:
            Dictionary with extraction results and statistics
        """
        if not TXD_SUPPORT:
            raise Exception("TXD support not available - required libraries not found")

        # Load TXD file
        txd_data = txd()
        txd_data.load_file(txd_file)

        # Create output directory
        os.makedirs(output_dir, exist_ok=True)

        # Extract textures
        extracted_count = self._extract_textures(txd_data, output_dir, format)

        # Create material file
        material_file = self._create_material_file(txd_data, output_dir)

        result = {
            'txd_file': txd_file,
            'output_dir': output_dir,
            'extracted': extracted_count,
            'format': format
        }

        if material_file:
            result['material_file'] = material_file

        return result

    def _extract_textures(self, txd_data, output_dir: str, format: str = 'png') -> int:
        """Extract textures from TXD to files"""
        try:
            from PIL import Image
        except ImportError:
            return 0

        extracted_count = 0

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

        return extracted_count

    def _create_material_file(self, txd_data, output_dir: str) -> Optional[str]:
        """Create a material definition file"""
        if not txd_data.native_textures:
            return None

        material_filename = "materials.txt"
        material_path = os.path.join(output_dir, material_filename)

        with open(material_path, 'w') as f:
            f.write("# Material definitions extracted from TXD\n")
            f.write(f"# TXD Version: 0x{txd_data.rw_version:08x}\n")
            f.write(f"# Device ID: {txd_data.device_id}\n")
            f.write(f"# Texture Count: {len(txd_data.native_textures)}\n\n")

            for idx, texture in enumerate(txd_data.native_textures):
                f.write(f"Material_{idx}:\n")
                f.write(f"  Name: {texture.name}\n")
                f.write(f"  Width: {texture.get_width(0)}\n")
                f.write(f"  Height: {texture.get_height(0)}\n")
                f.write(f"  Mipmaps: {texture.num_levels}\n")

                # Add texture format info if available
                if hasattr(texture, 'format'):
                    f.write(f"  Format: {texture.format}\n")

                f.write(f"  File: {texture.name}.png\n\n")

        return material_path