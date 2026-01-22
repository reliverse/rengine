#!/usr/bin/env python3
"""
IDE Parser for GTA Game Files

Parses IDE (Item Definition) files to map model IDs to names and textures.
Focused on extracting object definitions for IPL processing.

Usage as a module:
    from ide_parser import IDEParser
    parser = IDEParser()
    model_map = parser.parse_ide_files(['/path/to/ide1.ide', '/path/to/ide2.ide'])
"""

import csv
import logging
import os
from pathlib import Path
from typing import Any, Dict, List, Optional


class IDEParser:
    """Parser for IDE files to extract model definitions"""

    def __init__(self, logger: Optional[logging.Logger] = None):
        self.logger = logger or logging.getLogger(__name__)

    def parse_ide_files(self, ide_files: List[str]) -> Dict[int, Dict[str, Any]]:
        """
        Parse multiple IDE files and return a mapping of model IDs to model data.
        First loads from CSV if available, then IDE files.

        Args:
            ide_files: List of paths to IDE files

        Returns:
            Dictionary mapping model_id -> model data
        """
        model_map = {}

        # First, try to load from CSV if it exists
        csv_models = self._load_from_csv()
        if csv_models:
            model_map.update(csv_models)
            self.logger.info(f"Loaded {len(csv_models)} models from GTA SA CSV")

        # Then load from IDE files (can override CSV data)
        for ide_file in ide_files:
            try:
                file_models = self._parse_single_ide_file(ide_file)
                # Merge with existing models (later files can override)
                model_map.update(file_models)
                self.logger.info(
                    f"Parsed {len(file_models)} models from {os.path.basename(ide_file)}"
                )
            except Exception as e:
                self.logger.warning(f"Failed to parse IDE file {ide_file}: {e}")

        return model_map

    def _parse_ide_files_only(self, ide_files: List[str]) -> Dict[int, Dict[str, Any]]:
        """
        Parse only IDE files (without CSV) - used when CSV is not available
        """
        model_map = {}

        for ide_file in ide_files:
            try:
                file_models = self._parse_single_ide_file(ide_file)
                # Merge with existing models (later files can override)
                model_map.update(file_models)
                self.logger.info(
                    f"Parsed {len(file_models)} models from {os.path.basename(ide_file)}"
                )
            except Exception as e:
                self.logger.warning(f"Failed to parse IDE file {ide_file}: {e}")

        return model_map

    def _load_from_csv(self) -> Dict[int, Dict[str, Any]]:
        """Load model data from GTA SA CSV file if it exists"""
        csv_path = Path("/home/blefnk/Documents/reliverse/rengine/preloaded/gta-sa.csv")
        if not csv_path.exists():
            return {}

        model_map = {}
        try:
            with open(csv_path, encoding="utf-8") as f:
                reader = csv.DictReader(f)
                for row in reader:
                    try:
                        model_id = int(row["id"])
                        model_map[model_id] = {
                            "id": model_id,
                            "model_name": row.get("name", f"model_{model_id}"),
                            "texture_name": row.get("materialsFile", ""),
                            "definition_file": row.get("definitionFile", ""),
                            "model_file": row.get("modelFile", ""),
                            "has_collision": row.get("hasCollision", "No") == "Yes",
                            "breaks_on_hit": row.get("breaksOnHit", "No") == "Yes",
                            "has_animation": row.get("hasAnimation", "No") == "Yes",
                            "visible_by_time": row.get(
                                "visibleByTime", "No, visible always"
                            ),
                            "radius": float(row.get("radius", 0))
                            if row.get("radius")
                            else 0,
                            "border_box_length": float(row.get("borderBoxLength", 0))
                            if row.get("borderBoxLength")
                            else 0,
                            "border_box_width": float(row.get("borderBoxWidth", 0))
                            if row.get("borderBoxWidth")
                            else 0,
                            "border_box_height": float(row.get("borderBoxHeight", 0))
                            if row.get("borderBoxHeight")
                            else 0,
                            "tags": row.get("tags", ""),
                            "section": "csv",  # Mark as coming from CSV
                        }
                    except (ValueError, KeyError) as e:
                        self.logger.debug(f"Failed to parse CSV row: {row}, error: {e}")
                        continue
        except Exception as e:
            self.logger.warning(f"Failed to load CSV file {csv_path}: {e}")
            return {}

        return model_map

    def _parse_single_ide_file(self, ide_file: str) -> Dict[int, Dict[str, Any]]:
        """Parse a single IDE file and extract model definitions"""
        model_map = {}

        with open(ide_file, encoding="utf-8", errors="ignore") as f:
            content = f.read()

        lines = content.splitlines()
        current_section = None

        for line in lines:
            line = line.strip()
            if not line or line.startswith("#"):
                continue

            # Check for section headers
            if line.lower() in ["objs", "tobj", "anim", "peds", "weap", "cars", "hier"]:
                current_section = line.lower()
                continue

            # Check for section end
            if line.lower() == "end":
                current_section = None
                continue

            # Parse object definitions
            if current_section in [
                "objs",
                "tobj",
                "anim",
                "peds",
                "weap",
                "cars",
                "hier",
            ]:
                model_data = self._parse_object_line(line, current_section)
                if model_data and "id" in model_data:
                    model_map[model_data["id"]] = model_data

        return model_map

    def _parse_object_line(self, line: str, section: str) -> Optional[Dict[str, Any]]:
        """Parse a single object definition line based on section type"""
        try:
            parts = line.split(",")
            if not parts:
                return None

            parts = [p.strip() for p in parts]

            # All sections start with an ID
            if not parts or not parts[0].isdigit():
                return None

            model_id = int(parts[0])

            if section in ["objs", "tobj", "anim"]:
                # Standard object format: id, modelName, textureName, [drawDists...], flags
                if len(parts) < 4:
                    return None

                model_name = parts[1]
                texture_name = parts[2]

                # Parse draw distances and flags
                draw_dists = []
                flags = 0
                current_index = 3

                # Parse remaining parts as draw distances and flags
                for i in range(3, len(parts)):
                    part = parts[i]
                    try:
                        if "." in part or "e" in part.lower():
                            draw_dists.append(float(part))
                        else:
                            # Last number is usually flags
                            if i == len(parts) - 1:
                                flags = int(part)
                            else:
                                draw_dists.append(float(part))
                    except ValueError:
                        break

                model_data = {
                    "id": model_id,
                    "model_name": model_name,
                    "texture_name": texture_name,
                    "draw_distances": draw_dists,
                    "flags": flags,
                    "section": section,
                }

                # Handle timed objects (tobj)
                if section == "tobj" and len(parts) >= 6:
                    try:
                        model_data["time_on"] = int(parts[-2])
                        model_data["time_off"] = int(parts[-1])
                    except (ValueError, IndexError):
                        pass

                # Handle animated objects (anim)
                elif section == "anim" and len(parts) >= 6:
                    try:
                        model_data["anim_name"] = parts[4]
                    except IndexError:
                        pass

                return model_data

            elif section == "peds":
                # Ped format: id, modelName, textureName, defaultType, behavior, animGroup, etc.
                if len(parts) < 7:
                    return None

                model_data = {
                    "id": model_id,
                    "model_name": parts[1],
                    "texture_name": parts[2],
                    "threat": parts[3] if len(parts) > 3 else "",
                    "behavior": parts[4] if len(parts) > 4 else "",
                    "anim_group": parts[5] if len(parts) > 5 else "",
                    "section": section,
                }
                return model_data

            elif section == "cars":
                # Vehicle format: id, modelName, textureName, type, handlingId, etc.
                if len(parts) < 6:
                    return None

                model_data = {
                    "id": model_id,
                    "model_name": parts[1],
                    "texture_name": parts[2],
                    "type": parts[3] if len(parts) > 3 else "",
                    "handling_id": parts[4] if len(parts) > 4 else "",
                    "section": section,
                }
                return model_data

            # For other sections, just extract basic info
            elif len(parts) >= 2:
                model_data = {
                    "id": model_id,
                    "model_name": parts[1],
                    "section": section,
                }
                if len(parts) >= 3:
                    model_data["texture_name"] = parts[2]
                return model_data

        except (ValueError, IndexError) as e:
            self.logger.debug(f"Failed to parse {section} line '{line}': {e}")
            return None

    def find_ide_files(self, directory: str) -> List[str]:
        """Find all IDE files in a directory"""
        ide_files = []
        for root, dirs, files in os.walk(directory):
            for file in files:
                if file.lower().endswith(".ide"):
                    ide_files.append(os.path.join(root, file))
        return sorted(ide_files)

    def get_model_name_by_id(
        self, model_map: Dict[int, Dict[str, Any]], model_id: int
    ) -> Optional[str]:
        """Get model name for a given model ID"""
        model_data = model_map.get(model_id)
        return model_data["model_name"] if model_data else None

    def get_texture_name_by_id(
        self, model_map: Dict[int, Dict[str, Any]], model_id: int
    ) -> Optional[str]:
        """Get texture name for a given model ID"""
        model_data = model_map.get(model_id)
        return model_data["texture_name"] if model_data else None
