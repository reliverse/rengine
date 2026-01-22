#!/usr/bin/env python3
"""
IPL to Godot Assets Converter

Converts DFF models referenced in IPL files to OBJ format for Godot import.
Handles batch conversion of models found in IPL data.

Usage as a module:
    from ipl_to_godot_assets import IPLToGodotAssetsConverter
    converter = IPLToGodotAssetsConverter()
    result = converter.convert_ipl_models_to_obj(ipl_data, ide_files, dff_dir, output_dir)
"""

import os
import logging
from typing import Dict, List, Any, Optional, Set
from pathlib import Path


class IPLToGodotAssetsConverter:
    """Converter for IPL-referenced models to Godot-compatible OBJ format"""

    def __init__(self, logger: Optional[logging.Logger] = None):
        self.logger = logger or logging.getLogger(__name__)

    def convert_ipl_models_to_obj(
        self,
        ipl_data: Dict[str, Any],
        model_map: Dict[int, Dict[str, Any]],
        dff_search_paths: List[str],
        output_dir: str,
        txd_search_paths: Optional[List[str]] = None
    ) -> Dict[str, Any]:
        """
        Convert all DFF models referenced in IPL data to OBJ format

        Args:
            ipl_data: Parsed IPL data
            model_map: Model ID to model data mapping from IDE files
            dff_search_paths: Directories to search for DFF files
            output_dir: Output directory for OBJ files
            txd_search_paths: Optional directories to search for TXD files

        Returns:
            Dictionary with conversion results and statistics
        """
        # Extract unique model IDs from IPL data
        model_ids = self._extract_model_ids_from_ipl(ipl_data)

        self.logger.info(f"Found {len(model_ids)} unique model IDs in IPL data")

        # Convert each model
        converted_models = {}
        successful_conversions = 0
        failed_conversions = 0

        os.makedirs(output_dir, exist_ok=True)

        for model_id in model_ids:
            try:
                result = self._convert_single_model(
                    model_id, model_map, dff_search_paths, output_dir, txd_search_paths
                )
                if result:
                    converted_models[model_id] = result
                    successful_conversions += 1
                else:
                    failed_conversions += 1
            except Exception as e:
                self.logger.warning(f"Failed to convert model {model_id}: {e}")
                failed_conversions += 1

        return {
            'total_models': len(model_ids),
            'successful_conversions': successful_conversions,
            'failed_conversions': failed_conversions,
            'converted_models': converted_models,
            'output_dir': output_dir
        }

    def _extract_model_ids_from_ipl(self, ipl_data: Dict[str, Any]) -> Set[int]:
        """Extract all unique model IDs from IPL data"""
        model_ids = set()

        if "sections" in ipl_data and "inst" in ipl_data["sections"]:
            for entry in ipl_data["sections"]["inst"]["entries"]:
                if "id" in entry:
                    model_ids.add(entry["id"])

        return model_ids

    def _convert_single_model(
        self,
        model_id: int,
        model_map: Dict[int, Dict[str, Any]],
        dff_search_paths: List[str],
        output_dir: str,
        txd_search_paths: Optional[List[str]] = None
    ) -> Optional[Dict[str, Any]]:
        """Convert a single model ID to OBJ format"""

        # Get model data from IDE
        if model_id not in model_map:
            self.logger.warning(f"Model ID {model_id} not found in IDE data")
            return None

        model_data = model_map[model_id]
        model_name = model_data.get('model_name', f'model_{model_id}')
        texture_name = model_data.get('texture_name', '')

        # Find DFF file
        dff_file = self._find_dff_file(model_name, dff_search_paths)
        if not dff_file:
            self.logger.warning(f"DFF file not found for model {model_name} (ID: {model_id})")
            return None

        # Find TXD file (optional)
        txd_file = None
        if txd_search_paths and texture_name:
            txd_file = self._find_txd_file(texture_name, txd_search_paths)

        # Convert to OBJ
        obj_file = os.path.join(output_dir, f"{model_name}.obj")

        try:
            from .dff_to_obj import DFFToOBJConverter
            converter = DFFToOBJConverter(self.logger)
            result = converter.convert(dff_file, obj_file, txd_file)

            return {
                'model_id': model_id,
                'model_name': model_name,
                'dff_file': dff_file,
                'obj_file': obj_file,
                'mtl_file': result.get('mtl_file'),
                'textures_extracted': result.get('textures_extracted', 0),
                'txd_file': txd_file
            }

        except Exception as e:
            self.logger.error(f"Failed to convert {dff_file} to OBJ: {e}")
            return None

    def _find_dff_file(self, model_name: str, search_paths: List[str]) -> Optional[str]:
        """Find DFF file for a given model name"""
        dff_filename = f"{model_name}.dff"

        for search_path in search_paths:
            dff_path = os.path.join(search_path, dff_filename)
            if os.path.exists(dff_path):
                return dff_path

            # Also try case-insensitive search
            for root, dirs, files in os.walk(search_path):
                for file in files:
                    if file.lower() == dff_filename.lower():
                        return os.path.join(root, file)

        return None

    def _find_txd_file(self, texture_name: str, search_paths: List[str]) -> Optional[str]:
        """Find TXD file for a given texture name"""
        txd_filename = f"{texture_name}.txd"

        for search_path in search_paths:
            txd_path = os.path.join(search_path, txd_filename)
            if os.path.exists(txd_path):
                return txd_path

            # Also try case-insensitive search
            for root, dirs, files in os.walk(search_path):
                for file in files:
                    if file.lower() == txd_filename.lower():
                        return os.path.join(root, file)

        return None

    def generate_godot_resource_files(
        self,
        converted_models: Dict[int, Dict[str, Any]],
        output_dir: str
    ) -> Dict[str, str]:
        """
        Generate Godot resource files (.tres) for the converted models

        Args:
            converted_models: Results from convert_ipl_models_to_obj
            output_dir: Directory to save resource files

        Returns:
            Mapping of model_id to resource file path
        """
        resource_files = {}
        resources_dir = os.path.join(output_dir, "resources")
        os.makedirs(resources_dir, exist_ok=True)

        for model_id, model_data in converted_models.items():
            if not model_data or 'obj_file' not in model_data:
                continue

            obj_file = model_data['obj_file']
            model_name = model_data['model_name']

            # Create a Mesh resource file
            resource_content = f"""[gd_resource type="Mesh" load_steps=2 format=3]

[ext_resource path="res://{os.path.basename(obj_file)}" type="ArrayMesh" id="1"]

[resource]
mesh = ExtResource("1")
"""

            resource_file = os.path.join(resources_dir, f"{model_name}.tres")
            with open(resource_file, 'w') as f:
                f.write(resource_content)

            resource_files[model_id] = resource_file

        return resource_files