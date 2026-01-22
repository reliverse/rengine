#!/usr/bin/env python3
"""
IPL to Godot Scene/Project Converter

Converts IPL (Item Placement) map files to Godot scene files or complete projects.

Usage as a module:
    from ipl_to_godot import IPLToGodotConverter
    converter = IPLToGodotConverter()
    result = converter.convert_directory_to_godot_project('/path/to/ipl/files', '/output/dir')
"""

import glob
import logging
import os
from typing import Any, Dict, List, Optional


class IPLToGodotConverter:
    """Converter for IPL files to Godot scene files and projects

    Args:
        logger: Optional logger instance
        godot_version: Target Godot version (default: "4.5")
        godot_features: List of Godot features (default: [godot_version, "Forward Plus"])
        godot_compatibility_min: Minimum Godot compatibility version (default: "4.2")
        project_icon_color: Hex color for project icon background (default: "#478cbf")
        project_icon_text: Text to display on project icon (default: "G")
        generate_uids: Whether to generate dynamic UIDs for Godot files (default: True)
        with_rust: Whether to include Rust GDExtension integration and demo scenes (default: False)
    """

    def __init__(
        self,
        logger: Optional[logging.Logger] = None,
        godot_version: str = "4.5",
        godot_features: Optional[List[str]] = None,
        godot_compatibility_min: str = "4.2",
        project_icon_color: str = "#478cbf",
        project_icon_text: str = "G",
        generate_uids: bool = True,
        with_rust: bool = False,
    ):
        self.logger = logger or logging.getLogger(__name__)

        # Godot configuration
        self.godot_version = godot_version
        self.godot_features = godot_features or [godot_version, "Forward Plus"]
        self.godot_compatibility_min = godot_compatibility_min

        # Project appearance
        self.project_icon_color = project_icon_color
        self.project_icon_text = project_icon_text

        # UID generation
        self.generate_uids = generate_uids
        self._uid_counter = 0

        # Rust integration
        self.with_rust = with_rust

        # Asset conversion
        self.convert_assets = False
        self.dff_search_paths = []
        self.txd_search_paths = []

    def enable_asset_conversion(
        self, dff_search_paths: List[str], txd_search_paths: Optional[List[str]] = None
    ):
        """Enable automatic conversion of DFF models to OBJ format"""
        self.convert_assets = True
        self.dff_search_paths = dff_search_paths
        self.txd_search_paths = txd_search_paths or dff_search_paths

    def _generate_uid(self) -> str:
        """Generate a unique UID for Godot files"""
        if not self.generate_uids:
            return ""

        import uuid

        # Generate a consistent UID based on a namespace for reproducibility
        namespace = uuid.UUID("12345678-1234-5678-9012-123456789012")
        name = f"rengine-godot-{self._uid_counter}"
        self._uid_counter += 1

        uid = uuid.uuid5(namespace, name)
        return f"uid://{uid}"

    def _generate_unique_id(self) -> str:
        """Generate a simple unique ID for Godot scene nodes"""
        if not self.generate_uids:
            return ""

        # Use a simple counter-based ID for scene nodes
        # Godot expects simple numeric identifiers for unique_id
        node_id = 160704174 + self._uid_counter  # Start from a reasonable base
        self._uid_counter += 1
        return str(node_id)

    def _sanitize_node_name(self, name: str) -> str:
        """Sanitize a string to be a valid Godot node name

        Godot node names can only contain alphanumeric characters, underscores, and hyphens.
        """
        if not name:
            return "node"

        # Replace invalid characters with underscores
        import re

        sanitized = re.sub(r"[^a-zA-Z0-9_-]", "_", name)

        # Ensure it doesn't start with a number (Godot convention)
        if sanitized and sanitized[0].isdigit():
            sanitized = f"node_{sanitized}"

        # Ensure it's not empty after sanitization
        if not sanitized:
            sanitized = "node"

        return sanitized

    def _escape_string_for_godot(self, text: str) -> str:
        """Escape a string for safe inclusion in Godot scene files"""
        if not text:
            return ""

        # Escape backslashes first
        text = text.replace("\\", "\\\\")
        # Escape quotes
        text = text.replace('"', '\\"')
        # Escape newlines and tabs
        text = text.replace("\n", "\\n")
        text = text.replace("\t", "\\t")
        text = text.replace("\r", "\\r")

        # For any remaining control characters, replace with safe representations
        safe_chars = []
        for char in text:
            if ord(char) < 32 or ord(char) > 126:  # Control chars or non-ASCII
                safe_chars.append(f"\\x{ord(char):02x}")
            else:
                safe_chars.append(char)

        return "".join(safe_chars)

    def convert_to_godot(
        self,
        ipl_file: str,
        output_file: str,
        template_file: Optional[str] = None,
        ide_files: Optional[List[str]] = None,
    ) -> Dict[str, Any]:
        """Convert IPL file to Godot .tscn scene file

        Args:
            ipl_file: Path to the IPL file
            output_file: Path for the output .tscn file
            template_file: Optional template scene file
            ide_files: Optional list of IDE files for model name lookup (auto-detected if None)

        Returns:
            Dictionary with conversion results and statistics
        """
        # Parse IPL file
        ipl_data = self._parse_ipl_file(ipl_file)

        # Parse model data (CSV first, then IDE files as fallback)
        model_map = {}
        csv_available = False

        # First try to load from CSV
        from .ide_parser import IDEParser

        ide_parser = IDEParser(self.logger)

        csv_models = ide_parser._load_from_csv()
        if csv_models:
            model_map.update(csv_models)
            csv_available = True
            self.logger.info(
                f"Loaded {len(csv_models)} models from GTA SA CSV (single source of truth)"
            )
        else:
            self.logger.info("GTA SA CSV not found, falling back to IDE files")

        # Load IDE files only if CSV is not available
        if not csv_available:
            if ide_files is None:
                ide_files = self._find_ide_files_for_ipl(ipl_file)

            if ide_files:
                ide_models = ide_parser._parse_ide_files_only(ide_files)
                model_map.update(ide_models)
                self.logger.info(
                    f"Loaded {len(ide_models)} model definitions from {len(ide_files)} IDE files"
                )

        # Generate Godot scene
        scene_content = self._generate_godot_scene(
            ipl_data, ipl_file, template_file, model_map, csv_available
        )

        # Write scene file
        with open(output_file, "w", encoding="utf-8") as f:
            f.write(scene_content)

        result = {"scene_file": output_file, "ipl_file": ipl_file}

        # Add statistics
        if "sections" in ipl_data:
            sections = ipl_data["sections"]
            if "inst" in sections:
                original_count = sections["inst"]["count"]
                result["original_instances"] = original_count
                if csv_available:
                    result["instances"] = f"{original_count} (filtered by CSV)"
                else:
                    result["instances"] = original_count
            if "zone" in sections:
                result["zones"] = sections["zone"]["count"]

        return result

    def _find_ide_files_for_ipl(self, ipl_file: str) -> List[str]:
        """Automatically find IDE files related to an IPL file

        Searches for .ide files in:
        1. Same directory as the IPL file
        2. Parent directories (up to 3 levels up)
        3. Common GTA SA directories

        Args:
            ipl_file: Path to the IPL file

        Returns:
            List of IDE file paths
        """
        from pathlib import Path

        ide_files = []
        ipl_path = Path(ipl_file)

        # Search directories (current dir and up to 3 levels up)
        search_dirs = []
        current_dir = ipl_path.parent
        for _ in range(4):  # Current dir + 3 levels up
            if current_dir.exists():
                search_dirs.append(current_dir)
            current_dir = current_dir.parent

        # Add common GTA SA directories if they exist
        common_dirs = [
            Path("/home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa"),
            Path(
                "/home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa/models"
            ),
            Path(
                "/home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa/data"
            ),
        ]

        for common_dir in common_dirs:
            if common_dir.exists():
                search_dirs.append(common_dir)

        # Find all .ide files in search directories
        found_files = set()
        for search_dir in search_dirs:
            try:
                for ide_file in search_dir.rglob("*.ide"):
                    if ide_file.is_file():
                        found_files.add(str(ide_file))
            except (OSError, PermissionError):
                continue

        ide_files = sorted(list(found_files))
        self.logger.debug(f"Found {len(ide_files)} IDE files for IPL {ipl_file}")
        return ide_files

    def _find_ide_files_for_directory(self, input_dir: str) -> List[str]:
        """Automatically find IDE files for a directory of IPL files

        Searches for .ide files in:
        1. The input directory
        2. Parent directories (up to 3 levels up)
        3. Common GTA SA directories

        Args:
            input_dir: Directory containing IPL files

        Returns:
            List of IDE file paths
        """
        from pathlib import Path

        ide_files = []
        input_path = Path(input_dir)

        # Search directories (input dir and up to 3 levels up)
        search_dirs = []
        current_dir = input_path
        for _ in range(4):  # Input dir + 3 levels up
            if current_dir.exists():
                search_dirs.append(current_dir)
            current_dir = current_dir.parent

        # Add common GTA SA directories if they exist
        common_dirs = [
            Path("/home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa"),
            Path(
                "/home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa/models"
            ),
            Path(
                "/home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa/data"
            ),
        ]

        for common_dir in common_dirs:
            if common_dir.exists():
                search_dirs.append(common_dir)

        # Find all .ide files in search directories
        found_files = set()
        for search_dir in search_dirs:
            try:
                for ide_file in search_dir.rglob("*.ide"):
                    if ide_file.is_file():
                        found_files.add(str(ide_file))
            except (OSError, PermissionError):
                continue

        ide_files = sorted(list(found_files))
        self.logger.debug(f"Found {len(ide_files)} IDE files for directory {input_dir}")
        return ide_files

    def convert_directory_to_godot_project(
        self,
        input_dir: str,
        output_dir: str,
        project_name: str = "IPLMap",
        template_file: Optional[str] = None,
        ide_files: Optional[List[str]] = None,
    ) -> Dict[str, Any]:
        """Convert all IPL files in a directory to a complete Godot project

        Args:
            input_dir: Directory containing IPL files
            output_dir: Output directory for Godot project
            project_name: Name for the Godot project
            template_file: Optional template scene file
            ide_files: Optional list of IDE files for model name lookup (auto-detected if None)
        """
        """Convert all IPL files in a directory to a complete Godot project

        Args:
            input_dir: Directory containing IPL files
            output_dir: Output directory for Godot project
            project_name: Name for the Godot project
            template_file: Optional template scene file

        Returns:
            Dictionary with project information and statistics
        """
        # Find all IPL files
        ipl_files = []
        for pattern in ["*.ipl", "*.IPL"]:
            ipl_files.extend(glob.glob(os.path.join(input_dir, pattern)))

        if not ipl_files:
            raise Exception(f"No IPL files found in directory: {input_dir}")

        # Parse model data (CSV first, then IDE files as fallback)
        model_map = {}
        csv_available = False

        # First try to load from CSV
        from .ide_parser import IDEParser

        ide_parser = IDEParser(self.logger)

        csv_models = ide_parser._load_from_csv()
        if csv_models:
            model_map.update(csv_models)
            csv_available = True
            self.logger.info(
                f"Loaded {len(csv_models)} models from GTA SA CSV (single source of truth)"
            )
        else:
            self.logger.info("GTA SA CSV not found, falling back to IDE files")

        # Load IDE files only if CSV is not available
        if not csv_available:
            if ide_files is None:
                ide_files = self._find_ide_files_for_directory(input_dir)

            if ide_files:
                ide_models = ide_parser._parse_ide_files_only(ide_files)
                model_map.update(ide_models)
                self.logger.info(
                    f"Parsed {len(ide_models)} model definitions from {len(ide_files)} IDE files"
                )

        # Store CSV availability for scene generation
        self._csv_available = csv_available

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
                if "sections" in ipl_data:
                    sections = ipl_data["sections"]
                    if "inst" in sections:
                        total_instances += sections["inst"]["count"]
                    if "zone" in sections:
                        total_zones += sections["zone"]["count"]

                self.logger.info(f"Parsed IPL file: {filename}")

            except Exception as e:
                self.logger.warning(f"Failed to parse IPL file {ipl_file}: {e}")

        # Create Godot project structure
        self._create_godot_project_structure(output_dir, project_name)

        # Generate demo scenes (only when with_rust is True)
        if self.with_rust:
            # Generate simple demo scene with RW analysis capabilities
            demo_scene_content = self._generate_simple_demo_godot_scene(project_name)
            demo_scene_path = os.path.join(output_dir, "demo.tscn")

            with open(demo_scene_path, "w", encoding="utf-8") as f:
                f.write(demo_scene_content)

            # Generate RW parser demo scene (alternative demo)
            rw_demo_scene_content = self._generate_rw_demo_godot_scene(project_name)
            rw_demo_scene_path = os.path.join(output_dir, "rw_demo.tscn")

            with open(rw_demo_scene_path, "w", encoding="utf-8") as f:
                f.write(rw_demo_scene_content)

            # Create UID files for scene files (if enabled)
            if self.generate_uids:
                # UID for demo.tscn
                demo_uid_path = os.path.join(output_dir, "demo.tscn.uid")
                with open(demo_uid_path, "w", encoding="utf-8") as f:
                    f.write(self._generate_uid())

                # UID for rw_demo.tscn
                rw_demo_uid_path = os.path.join(output_dir, "rw_demo.tscn.uid")
                with open(rw_demo_uid_path, "w", encoding="utf-8") as f:
                    f.write(self._generate_uid())

        # Generate combined IPL scene (for reference)
        main_scene_content = self._generate_combined_godot_scene(
            all_ipl_data, project_name, model_map, csv_available
        )
        main_scene_path = os.path.join(output_dir, "ipl_combined.tscn")

        with open(main_scene_path, "w", encoding="utf-8") as f:
            f.write(main_scene_content)

        # Convert assets if enabled
        asset_conversion_result = None
        if self.convert_assets and model_map:
            self.logger.info("Converting DFF models to OBJ format...")
            from .ipl_to_godot_assets import IPLToGodotAssetsConverter

            asset_converter = IPLToGodotAssetsConverter(self.logger)

            # Combine all IPL data for asset conversion
            combined_ipl_data = {"sections": {"inst": {"entries": []}}}
            for ipl_data in all_ipl_data.values():
                if "sections" in ipl_data and "inst" in ipl_data["sections"]:
                    combined_ipl_data["sections"]["inst"]["entries"].extend(
                        ipl_data["sections"]["inst"]["entries"]
                    )

            assets_dir = os.path.join(output_dir, "assets")
            asset_conversion_result = asset_converter.convert_ipl_models_to_obj(
                combined_ipl_data,
                model_map,
                self.dff_search_paths,
                assets_dir,
                self.txd_search_paths,
            )

            if asset_conversion_result:
                self.logger.info(
                    f"Converted {asset_conversion_result['successful_conversions']} models to OBJ format"
                )

        # Generate individual scenes for each IPL file (optional)
        scenes_dir = os.path.join(output_dir, "scenes")
        os.makedirs(scenes_dir, exist_ok=True)

        for filename, ipl_data in all_ipl_data.items():
            scene_name = os.path.splitext(filename)[0] + ".tscn"
            scene_path = os.path.join(scenes_dir, scene_name)
            scene_content = self._generate_godot_scene(
                ipl_data,
                filename,
                template_file,
                model_map,
                csv_available,
                asset_conversion_result,
            )

            with open(scene_path, "w", encoding="utf-8") as f:
                f.write(scene_content)

        result = {
            "project_dir": output_dir,
            "project_name": project_name,
            "ipl_files": len(all_ipl_data),
            "total_instances": total_instances,
            "total_zones": total_zones,
            "main_scene": main_scene_path,
            "individual_scenes": len(all_ipl_data),
        }

        # Add demo scenes only if with_rust is True
        if self.with_rust:
            result["demo_scene"] = demo_scene_path
            result["rw_demo_scene"] = rw_demo_scene_path

        return result

    def _create_godot_project_structure(self, output_dir: str, project_name: str):
        """Create complete Godot project structure with GDExtension integration"""
        # Create output directory
        os.makedirs(output_dir, exist_ok=True)

        # Create project.godot
        features_str = ", ".join(f'"{f}"' for f in self.godot_features)

        # Set main scene based on whether Rust integration is enabled
        main_scene = "res://demo.tscn" if self.with_rust else "res://ipl_combined.tscn"

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
run/main_scene="{main_scene}"
config/features=PackedStringArray({features_str})
config/icon="res://icon.svg"
"""

        project_godot_path = os.path.join(output_dir, "project.godot")
        with open(project_godot_path, "w", encoding="utf-8") as f:
            f.write(project_godot_content)

        # Create basic icon.svg (minimal)
        icon_content = f'''<svg width="64" height="64" xmlns="http://www.w3.org/2000/svg">
  <rect width="64" height="64" fill="{self.project_icon_color}"/>
  <text x="32" y="40" font-family="Arial" font-size="40" fill="white" text-anchor="middle">{self.project_icon_text}</text>
</svg>'''

        icon_path = os.path.join(output_dir, "icon.svg")
        with open(icon_path, "w", encoding="utf-8") as f:
            f.write(icon_content)

        # Create icon.svg.import file for Godot
        icon_uid = self._generate_uid() if self.generate_uids else "uid://briwoxt8dobum"
        import hashlib

        # Generate a consistent hash for the import path
        hash_input = f"icon.svg-{icon_uid}".encode()
        import_hash = hashlib.md5(hash_input).hexdigest()[:32]

        icon_import_content = f"""[remap]

importer="texture"
type="CompressedTexture2D"
uid="{icon_uid}"
path="res://.godot/imported/icon.svg-{import_hash}.ctex"
metadata={{
"vram_texture": false
}}

[deps]

source_file="res://icon.svg"
dest_files=["res://.godot/imported/icon.svg-{import_hash}.ctex"]

[params]

compress/mode=0
compress/high_quality=false
compress/lossy_quality=0.7
compress/uastc_level=0
compress/rdo_quality_loss=0.0
compress/hdr_compression=1
compress/normal_map=0
compress/channel_pack=0
mipmaps/generate=false
mipmaps/limit=-1
roughness/mode=0
roughness/src_normal=""
process/channel_remap/red=0
process/channel_remap/green=1
process/channel_remap/blue=2
process/channel_remap/alpha=3
process/fix_alpha_border=true
process/premult_alpha=false
process/normal_map_invert_y=false
process/hdr_as_srgb=false
process/hdr_clamp_exposure=false
process/size_limit=0
detect_3d/compress_to=1
svg/scale=1.0
editor/scale_with_editor_scale=false
editor/convert_colors_with_editor_theme=false
"""

        icon_import_path = os.path.join(output_dir, "icon.svg.import")
        with open(icon_import_path, "w", encoding="utf-8") as f:
            f.write(icon_import_content)

        # Create .gitignore for Godot project
        gitignore_content = """# Godot-specific ignores
.import/
*.import

# Imported binaries
*.so
*.dll
*.dylib

# Build artifacts
target/
Cargo.lock

# Temporary files
*.tmp
*.swp
*.bak
"""

        gitignore_path = os.path.join(output_dir, ".gitignore")
        with open(gitignore_path, "w", encoding="utf-8") as f:
            f.write(gitignore_content)

        # Create .editorconfig for consistent editor settings
        editorconfig_content = """root = true

[*]
charset = utf-8
"""

        editorconfig_path = os.path.join(output_dir, ".editorconfig")
        with open(editorconfig_path, "w", encoding="utf-8") as f:
            f.write(editorconfig_content)

        # Create .gitattributes for line ending normalization
        gitattributes_content = """# Normalize EOL for all files that Git considers text files.
* text=auto eol=lf
"""

        gitattributes_path = os.path.join(output_dir, ".gitattributes")
        with open(gitattributes_path, "w", encoding="utf-8") as f:
            f.write(gitattributes_content)

        # Create rengine_rw.gdextension (only when with_rust is True)
        if self.with_rust:
            gdextension_content = f"""[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = {self.godot_compatibility_min}
reloadable = true

[libraries]
linux.debug.x86_64 = "res://librengine_godot_rw.so"
linux.release.x86_64 = "res://librengine_godot_rw.so"
windows.debug.x86_64 = "res://rengine_godot_rw.dll"
windows.release.x86_64 = "res://rengine_godot_rw.dll"
macos.debug = "res://librengine_godot_rw.dylib"
macos.release = "res://librengine_godot_rw.dylib"
macos.debug.arm64 = "res://librengine_godot_rw.dylib"
macos.release.arm64 = "res://librengine_godot_rw.dylib"
"""

            gdextension_path = os.path.join(output_dir, "rengine_rw.gdextension")
            with open(gdextension_path, "w", encoding="utf-8") as f:
                f.write(gdextension_content)

            # Create UID file for the gdextension (if enabled)
            if self.generate_uids:
                gdextension_uid_path = os.path.join(
                    output_dir, "rengine_rw.gdextension.uid"
                )
                with open(gdextension_uid_path, "w", encoding="utf-8") as f:
                    f.write(self._generate_uid())

        # Create build script (only when with_rust is True)
        if self.with_rust:
            build_script_content = """#!/bin/bash

# Build and copy Rengine RW extension for Godot
# Run this from the project root directory

# Ensure the script is executable (self-healing)
chmod +x "$0" 2>/dev/null || true

# Build the godot-rw extension from workspace root
echo "Building Rengine RW extension..."
if command -v cargo >/dev/null 2>&1; then
    cargo build -p rengine-godot-rw --release
    BUILD_STATUS=$?

    if [ $BUILD_STATUS -eq 0 ]; then
        echo "✅ Extension built successfully!"

        # Copy the library to the project directory
        if [ -f "target/release/librengine_godot_rw.so" ]; then
            cp target/release/librengine_godot_rw.so .
            echo "✅ Linux library copied!"
        fi

        if [ -f "target/release/rengine_godot_rw.dll" ]; then
            cp target/release/rengine_godot_rw.dll .
            echo "✅ Windows library copied!"
        fi

        if [ -f "target/release/librengine_godot_rw.dylib" ]; then
            cp target/release/librengine_godot_rw.dylib .
            echo "✅ macOS library copied!"
        fi

        echo ""
        echo "🎮 You can now open the Godot project and run the demo scenes!"
        echo "   Main demo: demo.tscn (recommended)"
        echo "   RW parser demo: rw_demo.tscn"
        echo "   IPL scenes: scenes/*.tscn"
    else
        echo "❌ Failed to build extension"
        exit 1
    fi
else
    echo "❌ Cargo not found. Please install Rust and Cargo first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi
"""

            build_script_path = os.path.join(output_dir, "build_extension.sh")
            with open(build_script_path, "w", encoding="utf-8") as f:
                f.write(build_script_content)

            # Make build script executable
            try:
                os.chmod(build_script_path, 0o755)
            except OSError:
                pass  # Ignore permission errors on some systems

        # Create alternative build script (build_and_copy.sh) (only when with_rust is True)
        if self.with_rust:
            build_and_copy_script_content = """#!/bin/bash

# Build and copy Rengine RW extension for Godot
# Run this from the project root directory

# Ensure the script is executable (self-healing)
chmod +x "$0" 2>/dev/null || true

# Build the godot-rw extension from workspace root
echo "Building Rengine RW extension..."
if command -v cargo >/dev/null 2>&1; then
    cargo build -p rengine-godot-rw --release
    BUILD_STATUS=$?

    if [ $BUILD_STATUS -eq 0 ]; then
        echo "✅ Extension built successfully!"

        # Find the target directory by going up directories from current location
        TARGET_DIR=""
        CURRENT_DIR="$(pwd)"
        for i in {1..5}; do
            if [ -d "$CURRENT_DIR/target/release" ]; then
                TARGET_DIR="$CURRENT_DIR/target/release"
                break
            fi
            CURRENT_DIR="$(dirname "$CURRENT_DIR")"
        done

        # Copy the library to the project directory
        if [ -n "$TARGET_DIR" ]; then
            if [ -f "$TARGET_DIR/librengine_godot_rw.so" ]; then
                cp "$TARGET_DIR/librengine_godot_rw.so" .
                echo "✅ Linux library copied!"
            fi

            if [ -f "$TARGET_DIR/rengine_godot_rw.dll" ]; then
                cp "$TARGET_DIR/rengine_godot_rw.dll" .
                echo "✅ Windows library copied!"
            fi

            if [ -f "$TARGET_DIR/librengine_godot_rw.dylib" ]; then
                cp "$TARGET_DIR/librengine_godot_rw.dylib" .
                echo "✅ macOS library copied!"
            fi
        else
            echo "⚠️  Could not find target/release directory, library not copied"
        fi

        echo ""
        echo "🎮 You can now open the Godot project and run the demo scenes!"
        echo "   Main demo: demo.tscn (recommended)"
        echo "   RW parser demo: rw_demo.tscn"
        echo "   IPL scenes: scenes/*.tscn"
    else
        echo "❌ Failed to build extension"
        exit 1
    fi
else
    echo "❌ Cargo not found. Please install Rust and Cargo first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi
"""

            build_and_copy_script_path = os.path.join(output_dir, "build_and_copy.sh")
            with open(build_and_copy_script_path, "w", encoding="utf-8") as f:
                f.write(build_and_copy_script_content)

            # Make build_and_copy script executable
            try:
                os.chmod(build_and_copy_script_path, 0o755)
            except OSError:
                pass  # Ignore permission errors on some systems

        # Create addons directory structure (only when with_rust is True)
        if self.with_rust:
            addons_dir = os.path.join(output_dir, "addons")
            os.makedirs(addons_dir, exist_ok=True)

            rengine_addon_dir = os.path.join(addons_dir, "rengine_rw")
            os.makedirs(rengine_addon_dir, exist_ok=True)

            # Create plugin configuration file
            plugin_config = """[plugin]

name="Rengine RW"
description="RenderWare file parsing and IPL map integration for Godot"
author="Rengine"
version="1.0"
script="rengine_rw_plugin.gd"
"""

            plugin_config_path = os.path.join(rengine_addon_dir, "plugin.cfg")
            with open(plugin_config_path, "w", encoding="utf-8") as f:
                f.write(plugin_config)

            # Create plugin script (minimal for now)
            plugin_script = """@tool
extends EditorPlugin

func _enter_tree():
    # Plugin initialization
    print("Rengine RW Plugin loaded")

func _exit_tree():
    # Plugin cleanup
    print("Rengine RW Plugin unloaded")
"""

            plugin_script_path = os.path.join(rengine_addon_dir, "rengine_rw_plugin.gd")
            with open(plugin_script_path, "w", encoding="utf-8") as f:
                f.write(plugin_script)

            # Create UID files for Godot (if enabled)
            if self.generate_uids:
                plugin_uid_path = os.path.join(
                    rengine_addon_dir, "rengine_rw_plugin.gd.uid"
                )
                with open(plugin_uid_path, "w", encoding="utf-8") as f:
                    f.write(self._generate_uid())

                dock_uid_path = os.path.join(rengine_addon_dir, "rw_dock.gd.uid")
                with open(dock_uid_path, "w", encoding="utf-8") as f:
                    f.write(self._generate_uid())

        # Create README for the generated project
        if self.with_rust:
            readme_content = f"""# {project_name} - IPL Map Godot Project

This Godot project was generated by Rengine CLI from IPL (Item Placement) map files.

## Features

- **IPL Map Data**: Converted from multiple IPL files
- **RenderWare Analysis**: Built-in file analysis tools for IMG, DFF, TXD, COL files
- **Interactive Demo**: Demo scene with file analysis and IPL data visualization
- **GDExtension Integration**: Full RenderWare parsing capabilities

## Getting Started

### 1. Build the Extension

First, build and copy the RenderWare extension:

```bash
./build_extension.sh
```

This requires:
- Rust and Cargo installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- The extension will be built from the Rengine workspace

### 2. Open in Godot

Open the project in Godot 4.5+:

```bash
godot project.godot
```

Or open `project.godot` directly in Godot Editor.

### 3. Run the Demo

You can run either demo scene:

**Main Demo** (`demo.tscn` - Recommended):
- **File Analysis**: Analyze any RenderWare file format
- **IMG Archive Parsing**: Parse GTA IMG archives and get file statistics
- **DFF Model Parsing**: Parse 3D model files and extract metadata
- **Clean Interface**: Lightweight demo focused on RW analysis

**RW Parser Demo** (`rw_demo.tscn`):
- **File Analysis**: Analyze any file to detect format and version information
- **IMG Archive Parsing**: Parse GTA IMG archives and get file statistics
- **DFF Model Parsing**: Parse 3D model files and extract metadata
- **Real-time Results**: See parsing results displayed in the Godot GUI

## Project Structure

```
{project_name}/
├── project.godot          # Godot project configuration
├── demo.tscn             # Main demo scene (recommended)
├── rw_demo.tscn          # RW parser demo scene
├── ipl_combined.tscn     # Combined IPL data scene
├── scenes/               # Individual IPL file scenes
├── rengine_rw.gdextension # GDExtension configuration
├── build_extension.sh    # Build script for the extension
├── build_and_copy.sh     # Alternative build script
├── addons/rengine_rw/    # Plugin files
├── .editorconfig         # Editor configuration
├── .gitattributes        # Git line ending configuration
├── icon.svg             # Project icon
└── icon.svg.import      # Godot icon import configuration
```

## RenderWare File Support

This project can analyze:

- **IMG**: GTA game archive files (.img)
- **DFF**: 3D model files (.dff)
- **TXD**: Texture dictionary files (.txd)
- **COL**: Collision data files (.col)
- **IPL**: Item placement files (.ipl)
- **IDE**: Item definition files (.ide)

## Development

To extend this project:

1. **Add more IPL files**: Place additional .ipl files in your source directory and regenerate
2. **Customize scenes**: Modify the generated .tscn files in Godot Editor
3. **Add game logic**: Extend the demo scene script with your gameplay features
4. **Import assets**: Add DFF models, TXD textures, and other game assets

## Technical Details

- **IPL Conversion**: Map instances become Node3D objects with metadata
- **Zone System**: IPL zones become Area3D objects with collision shapes
- **GDExtension**: Provides fast Rust-based file parsing
- **Godot 4.2+**: Requires Godot 4.2 or later for GDExtension support

## Troubleshooting

**Extension won't load?**
- Run `./build_extension.sh` to build the latest extension
- Check that you have the correct Godot version (4.2+)
- Ensure the extension library is in the project root

**IPL data not showing?**
- Check the Godot console for parsing errors
- Verify IPL files are in the correct format
- Look at individual scenes in the `scenes/` directory

**Performance issues?**
- The demo loads all IPL data at once - consider streaming for large maps
- Limit instances/zones displayed in the demo scene
"""
        else:
            readme_content = f"""# {project_name} - IPL Map Godot Project

This Godot project was generated by Rengine CLI from IPL (Item Placement) map files.

## Features

- **IPL Map Data**: Converted from multiple IPL files
- **Godot Scene Files**: Ready-to-use .tscn files for Godot Engine

## Getting Started

### 1. Open in Godot

Open the project in Godot 4.2+:

```bash
godot project.godot
```

Or open `project.godot` directly in Godot Editor.

### 2. Explore the Scenes

- **ipl_combined.tscn**: Combined IPL data from all input files
- **scenes/*.tscn**: Individual scenes for each IPL file

## Project Structure

```
{project_name}/
├── project.godot          # Godot project configuration
├── ipl_combined.tscn     # Combined IPL data scene
├── scenes/               # Individual IPL file scenes
├── .editorconfig         # Editor configuration
├── .gitattributes        # Git line ending configuration
├── icon.svg             # Project icon
└── icon.svg.import      # Godot icon import configuration
```

## Development

To extend this project:

1. **Add more IPL files**: Place additional .ipl files in your source directory and regenerate
2. **Customize scenes**: Modify the generated .tscn files in Godot Editor
3. **Add game logic**: Extend the scenes with your gameplay features
4. **Import assets**: Add models, textures, and other game assets

## Technical Details

- **IPL Conversion**: Map instances become Node3D objects with metadata
- **Zone System**: IPL zones become Area3D objects with collision shapes
- **Godot 4.2+**: Requires Godot 4.2 or later

## Troubleshooting

**IPL data not showing?**
- Check the Godot console for parsing errors
- Verify IPL files are in the correct format
- Look at individual scenes in the `scenes/` directory

**Performance issues?**
- Large IPL files may cause performance issues - consider splitting into smaller scenes
"""

        readme_path = os.path.join(output_dir, "README.md")
        with open(readme_path, "w", encoding="utf-8") as f:
            f.write(readme_content)

    def _generate_demo_godot_scene(
        self, all_ipl_data: Dict[str, Dict], project_name: str
    ) -> str:
        """Generate a demo Godot scene with IPL data and RW analysis capabilities"""

        scene_lines = []
        scene_lines.append("[gd_scene load_steps=3 format=3]")
        scene_lines.append("")

        # Add script subresource
        scene_lines.append('[sub_resource type="GDScript" id="1"]')
        scene_lines.append('script/source = """')
        scene_lines.append("extends Node")
        scene_lines.append("")
        scene_lines.append(f"# {project_name} - IPL Map Demo with RenderWare Analysis")
        scene_lines.append(f"# Generated from {len(all_ipl_data)} IPL files")
        scene_lines.append("")
        scene_lines.append("@onready var rw_analyzer = $RwAnalyzer")
        scene_lines.append("@onready var output_label = $UI/VBoxContainer/OutputLabel")
        scene_lines.append(
            "@onready var ipl_stats_label = $UI/VBoxContainer/IPLStatsLabel"
        )
        scene_lines.append("")
        scene_lines.append("func _ready():")
        scene_lines.append("\t# Connect button signals")
        scene_lines.append(
            '\t$UI/VBoxContainer/AnalyzeFileButton.connect("pressed", Callable(self, "_on_analyze_file_pressed"))'
        )
        scene_lines.append(
            '\t$UI/VBoxContainer/ParseImgButton.connect("pressed", Callable(self, "_on_parse_img_pressed"))'
        )
        scene_lines.append(
            '\t$UI/VBoxContainer/ParseDffButton.connect("pressed", Callable(self, "_on_parse_dff_pressed"))'
        )
        scene_lines.append(
            '\t$UI/VBoxContainer/ClearButton.connect("pressed", Callable(self, "_on_clear_pressed"))'
        )
        scene_lines.append(
            '\t$UI/VBoxContainer/ViewIPLButton.connect("pressed", Callable(self, "_on_view_ipl_pressed"))'
        )
        scene_lines.append("\t")
        scene_lines.append("\t# Display IPL statistics")
        scene_lines.append("\t_update_ipl_stats()")
        scene_lines.append("")
        scene_lines.append("func _on_analyze_file_pressed():")
        scene_lines.append("\tvar dialog = FileDialog.new()")
        scene_lines.append("\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE")
        scene_lines.append("\tdialog.access = FileDialog.ACCESS_FILESYSTEM")
        scene_lines.append(
            '\tdialog.connect("file_selected", Callable(self, "_on_file_selected_for_analysis"))'
        )
        scene_lines.append("\tadd_child(dialog)")
        scene_lines.append("\tdialog.popup_centered()")
        scene_lines.append("")
        scene_lines.append("func _on_file_selected_for_analysis(path: String):")
        scene_lines.append("\tvar result = rw_analyzer.analyze_file(path)")
        scene_lines.append(
            '\toutput_label.text = "File Analysis:\\n" + JSON.stringify(result, "  ")'
        )
        scene_lines.append("")
        scene_lines.append("func _on_parse_img_pressed():")
        scene_lines.append("\tvar dialog = FileDialog.new()")
        scene_lines.append("\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE")
        scene_lines.append("\tdialog.access = FileDialog.ACCESS_FILESYSTEM")
        scene_lines.append(
            '\tdialog.filters = PackedStringArray(["*.img ; IMG Archive Files"])'
        )
        scene_lines.append(
            '\tdialog.connect("file_selected", Callable(self, "_on_img_file_selected"))'
        )
        scene_lines.append("\tadd_child(dialog)")
        scene_lines.append("\tdialog.popup_centered()")
        scene_lines.append("")
        scene_lines.append("func _on_img_file_selected(path: String):")
        scene_lines.append("\tvar result = rw_analyzer.parse_img_archive(path)")
        scene_lines.append(
            '\toutput_label.text = "IMG Archive:\\n" + JSON.stringify(result, "  ")'
        )
        scene_lines.append("")
        scene_lines.append("func _on_parse_dff_pressed():")
        scene_lines.append("\tvar dialog = FileDialog.new()")
        scene_lines.append("\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE")
        scene_lines.append("\tdialog.access = FileDialog.ACCESS_FILESYSTEM")
        scene_lines.append(
            '\tdialog.filters = PackedStringArray(["*.dff ; DFF Model Files"])'
        )
        scene_lines.append(
            '\tdialog.connect("file_selected", Callable(self, "_on_dff_file_selected"))'
        )
        scene_lines.append("\tadd_child(dialog)")
        scene_lines.append("\tdialog.popup_centered()")
        scene_lines.append("")
        scene_lines.append("func _on_dff_file_selected(path: String):")
        scene_lines.append("\tvar result = rw_analyzer.parse_dff_model(path)")
        scene_lines.append(
            '\toutput_label.text = "DFF Model:\\n" + JSON.stringify(result, "  ")'
        )
        scene_lines.append("")
        scene_lines.append("func _on_clear_pressed():")
        scene_lines.append('\toutput_label.text = "Select a file to analyze..."')
        scene_lines.append("\t_update_ipl_stats()")
        scene_lines.append("")
        scene_lines.append("func _on_view_ipl_pressed():")
        scene_lines.append(
            '\tget_tree().change_scene_to_file("res://ipl_combined.tscn")'
        )
        scene_lines.append("")
        scene_lines.append("func _update_ipl_stats():")
        scene_lines.append("\tvar total_instances = 0")
        scene_lines.append("\tvar total_zones = 0")
        scene_lines.append("\tvar file_count = 0")
        scene_lines.append("\t")
        scene_lines.append("\tfor ipl_node in $IPLData.get_children():")
        scene_lines.append("\t\tfile_count += 1")
        scene_lines.append("\t\tfor child in ipl_node.get_children():")
        scene_lines.append(
            '\t\t\tif child is Node3D and not child.name.ends_with("_zone"):'
        )
        scene_lines.append("\t\t\t\ttotal_instances += 1")
        scene_lines.append("\t\t\telif child is Area3D:")
        scene_lines.append("\t\t\t\ttotal_zones += 1")
        scene_lines.append("\t")
        scene_lines.append(
            '\tipl_stats_label.text = "IPL Data: %d files, %d instances, %d zones" % [file_count, total_instances, total_zones]'
        )
        scene_lines.append('"""')
        scene_lines.append("")

        # Root node
        scene_lines.append(f'[node name="{project_name}Demo" type="Node"]')
        scene_lines.append('script = SubResource("1")')
        scene_lines.append("")

        # RwAnalyzer node
        scene_lines.append('[node name="RwAnalyzer" type="RwAnalyzer" parent="."]')
        scene_lines.append("")

        # IPL Data container
        scene_lines.append('[node name="IPLData" type="Node" parent="."]')
        scene_lines.append("")

        # Add IPL data as child nodes
        total_instances = 0

        for ipl_filename, ipl_data in all_ipl_data.items():
            ipl_name = os.path.splitext(ipl_filename)[0]
            scene_lines.append(f'[node name="{ipl_name}" type="Node" parent="IPLData"]')
            scene_lines.append("")

            # Add instances from this IPL (limited for demo)
            if "sections" in ipl_data and "inst" in ipl_data["sections"]:
                instance_count = 0
                for entry in ipl_data["sections"]["inst"]["entries"][
                    :50
                ]:  # Limit to 50 instances for demo
                    if "model_name" in entry:
                        total_instances += 1
                        instance_count += 1
                        sanitized_model_name = self._sanitize_node_name(
                            entry["model_name"]
                        )
                        node_name = f"{sanitized_model_name}_{instance_count}"
                        scene_lines.append(
                            f'[node name="{node_name}" type="Node3D" parent="{ipl_name}"]'
                        )
                        scene_lines.append(
                            f"transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {entry['pos_x']:.6f}, {entry['pos_y']:.6f}, {entry['pos_z']:.6f})"
                        )

                        # Add metadata
                        scene_lines.append(f"metadata/model_id = {entry['id']}")
                        scene_lines.append(f"metadata/interior = {entry['interior']}")
                        scene_lines.append(f"metadata/lod = {entry['lod']}")
                        scene_lines.append(
                            f'metadata/ipl_source = "{self._escape_string_for_godot(ipl_filename)}"'
                        )
                        scene_lines.append("")

            # Add zones from this IPL (limited)
            if "sections" in ipl_data and "zone" in ipl_data["sections"]:
                for entry in ipl_data["sections"]["zone"]["entries"][
                    :20
                ]:  # Limit to 20 zones for demo
                    if "name" in entry:
                        center_x = (entry["min_x"] + entry["max_x"]) / 2
                        center_y = (entry["min_y"] + entry["max_y"]) / 2
                        center_z = (entry["min_z"] + entry["max_z"]) / 2
                        size_x = abs(entry["max_x"] - entry["min_x"])
                        size_y = abs(entry["max_y"] - entry["min_y"])
                        size_z = abs(entry["max_z"] - entry["max_z"])

                        zone_name = f"{entry['name']}_zone"
                        scene_lines.append(
                            f'[node name="{zone_name}" type="Area3D" parent="{ipl_name}"]'
                        )
                        scene_lines.append(
                            f"transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {center_x:.6f}, {center_y:.6f}, {center_z:.6f})"
                        )

                        # Add metadata
                        scene_lines.append(f"metadata/zone_type = {entry['type']}")
                        scene_lines.append(
                            f'metadata/level = "{self._escape_string_for_godot(entry["level"])}"'
                        )
                        scene_lines.append(
                            f'metadata/text = "{self._escape_string_for_godot(entry["text"])}"'
                        )
                        scene_lines.append(
                            f'metadata/ipl_source = "{self._escape_string_for_godot(ipl_filename)}"'
                        )

                        # Add CollisionShape3D as child
                        scene_lines.append(
                            f'[node name="CollisionShape3D" type="CollisionShape3D" parent="{zone_name}"]'
                        )
                        scene_lines.append('[sub_resource type="BoxShape3D" id="2"]')
                        scene_lines.append(
                            f"size = Vector3({size_x:.6f}, {size_y:.6f}, {size_z:.6f})"
                        )
                        scene_lines.append('shape = SubResource("2")')
                        scene_lines.append("")

        # UI Layer
        scene_lines.append('[node name="UI" type="CanvasLayer" parent="."]')
        scene_lines.append("")
        scene_lines.append(
            '[node name="VBoxContainer" type="VBoxContainer" parent="UI"]'
        )
        scene_lines.append("anchors_preset = 15")
        scene_lines.append("anchor_right = 1.0")
        scene_lines.append("anchor_bottom = 1.0")
        scene_lines.append("grow_horizontal = 2")
        scene_lines.append("grow_vertical = 2")
        scene_lines.append("")

        # UI Elements
        scene_lines.append(
            '[node name="TitleLabel" type="Label" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append(f'text = "{project_name} - IPL Map Demo"')
        scene_lines.append("horizontal_alignment = 1")
        scene_lines.append("")

        scene_lines.append(
            '[node name="IPLStatsLabel" type="Label" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "IPL Data: Loading..."')
        scene_lines.append("horizontal_alignment = 1")
        scene_lines.append("")

        scene_lines.append(
            '[node name="HSeparator" type="HSeparator" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append("")

        scene_lines.append(
            '[node name="SectionLabel" type="Label" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "RenderWare File Analysis"')
        scene_lines.append("horizontal_alignment = 1")
        scene_lines.append("")

        scene_lines.append(
            '[node name="AnalyzeFileButton" type="Button" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Analyze Any File"')
        scene_lines.append("")

        scene_lines.append(
            '[node name="ParseImgButton" type="Button" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Parse IMG Archive"')
        scene_lines.append("")

        scene_lines.append(
            '[node name="ParseDffButton" type="Button" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Parse DFF Model"')
        scene_lines.append("")

        scene_lines.append(
            '[node name="HSeparator2" type="HSeparator" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append("")

        scene_lines.append(
            '[node name="ViewIPLButton" type="Button" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "View IPL Map Data"')
        scene_lines.append("")

        scene_lines.append(
            '[node name="ClearButton" type="Button" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Clear Output"')
        scene_lines.append("")

        scene_lines.append(
            '[node name="OutputLabel" type="Label" parent="UI/VBoxContainer"]'
        )
        scene_lines.append("custom_minimum_size = Vector2(0, 150)")
        scene_lines.append("layout_mode = 2")
        scene_lines.append("size_flags_vertical = 3")
        scene_lines.append(
            'text = "Welcome to Rengine IPL Demo!\\n\\nUse the buttons above to analyze RenderWare files or view your IPL map data."'
        )
        scene_lines.append("autowrap_mode = 2")

        return "\n".join(scene_lines)

    def _generate_simple_demo_godot_scene(self, project_name: str) -> str:
        """Generate a simple demo Godot scene with RW analysis capabilities"""

        scene_lines = []
        scene_uid = (
            self._generate_uid() if self.generate_uids else "uid://b8x4nav2p1h7j"
        )
        scene_lines.append(f'[gd_scene format=3 uid="{scene_uid}"]')
        scene_lines.append("")

        scene_lines.append('[sub_resource type="GDScript" id="1"]')
        scene_lines.append('script/source = "')
        scene_lines.append("extends Node")
        scene_lines.append("")
        scene_lines.append("var rw_analyzer")
        scene_lines.append("var output_label")
        scene_lines.append("")
        scene_lines.append("func _ready():")
        scene_lines.append("\trw_analyzer = $RwAnalyzer")
        scene_lines.append("\toutput_label = $UI/VBoxContainer/OutputLabel")
        scene_lines.append("\t")
        scene_lines.append(
            '\t$UI/VBoxContainer/AnalyzeFileButton.connect(\\"pressed\\", Callable(self, \\"_on_analyze_file_pressed\\"))'
        )
        scene_lines.append(
            '\t$UI/VBoxContainer/ParseImgButton.connect(\\"pressed\\", Callable(self, \\"_on_parse_img_pressed\\"))'
        )
        scene_lines.append(
            '\t$UI/VBoxContainer/ParseDffButton.connect(\\"pressed\\", Callable(self, \\"_on_parse_dff_pressed\\"))'
        )
        scene_lines.append(
            '\t$UI/VBoxContainer/ClearButton.connect(\\"pressed\\", Callable(self, \\"_on_clear_pressed\\"))'
        )
        scene_lines.append("\t")
        scene_lines.append(
            '\toutput_label.text = \\"Rengine IPL Demo\\\\n\\\\nUse the buttons to analyze RenderWare files.\\"'
        )
        scene_lines.append("")
        scene_lines.append("func _on_analyze_file_pressed():")
        scene_lines.append("\tvar dialog = FileDialog.new()")
        scene_lines.append("\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE")
        scene_lines.append("\tdialog.access = FileDialog.ACCESS_FILESYSTEM")
        scene_lines.append(
            '\tdialog.connect(\\"file_selected\\", Callable(self, \\"_on_file_selected_for_analysis\\"))'
        )
        scene_lines.append("\tadd_child(dialog)")
        scene_lines.append("\tdialog.popup_centered()")
        scene_lines.append("")
        scene_lines.append("func _on_file_selected_for_analysis(path: String):")
        scene_lines.append("\tvar result = rw_analyzer.analyze_file(path)")
        scene_lines.append(
            '\toutput_label.text = \\"File Analysis:\\\\n\\" + JSON.stringify(result, \\"  \\")'
        )
        scene_lines.append("")
        scene_lines.append("func _on_parse_img_pressed():")
        scene_lines.append("\tvar dialog = FileDialog.new()")
        scene_lines.append("\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE")
        scene_lines.append("\tdialog.access = FileDialog.ACCESS_FILESYSTEM")
        scene_lines.append(
            '\tdialog.filters = PackedStringArray([\\"*.img ; IMG Archive Files\\"])'
        )
        scene_lines.append(
            '\tdialog.connect(\\"file_selected\\", Callable(self, \\"_on_img_file_selected\\"))'
        )
        scene_lines.append("\tadd_child(dialog)")
        scene_lines.append("\tdialog.popup_centered()")
        scene_lines.append("")
        scene_lines.append("func _on_img_file_selected(path: String):")
        scene_lines.append("\tvar result = rw_analyzer.parse_img_archive(path)")
        scene_lines.append(
            '\toutput_label.text = \\"IMG Archive:\\\\n\\" + JSON.stringify(result, \\"  \\")'
        )
        scene_lines.append("")
        scene_lines.append("func _on_parse_dff_pressed():")
        scene_lines.append("\tvar dialog = FileDialog.new()")
        scene_lines.append("\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE")
        scene_lines.append("\tdialog.access = FileDialog.ACCESS_FILESYSTEM")
        scene_lines.append(
            '\tdialog.filters = PackedStringArray([\\"*.dff ; DFF Model Files\\"])'
        )
        scene_lines.append(
            '\tdialog.connect(\\"file_selected\\", Callable(self, \\"_on_dff_file_selected\\"))'
        )
        scene_lines.append("\tadd_child(dialog)")
        scene_lines.append("\tdialog.popup_centered()")
        scene_lines.append("")
        scene_lines.append("func _on_dff_file_selected(path: String):")
        scene_lines.append("\tvar result = rw_analyzer.parse_dff_model(path)")
        scene_lines.append(
            '\toutput_label.text = \\"DFF Model:\\\\n\\" + JSON.stringify(result, \\"  \\")'
        )
        scene_lines.append("")
        scene_lines.append("func _on_clear_pressed():")
        scene_lines.append(
            '\toutput_label.text = \\"Rengine IPL Demo\\\\n\\\\nUse the buttons to analyze RenderWare files.\\"'
        )
        scene_lines.append('"')
        scene_lines.append("")

        demo_node_uid = self._generate_uid() if self.generate_uids else "1177046869"
        analyzer_uid = self._generate_uid() if self.generate_uids else "389288978"
        ui_uid = self._generate_uid() if self.generate_uids else "909622078"
        container_uid = self._generate_uid() if self.generate_uids else "1063079614"
        title_label_uid = self._generate_uid() if self.generate_uids else "2067397487"
        analyze_btn_uid = self._generate_uid() if self.generate_uids else "1265005479"
        parse_img_btn_uid = self._generate_uid() if self.generate_uids else "331067572"
        parse_dff_btn_uid = self._generate_uid() if self.generate_uids else "1328955608"
        clear_btn_uid = self._generate_uid() if self.generate_uids else "845460302"
        output_label_uid = self._generate_uid() if self.generate_uids else "497941531"

        scene_lines.append(
            f'[node name="{project_name}Demo" type="Node" unique_id={demo_node_uid}]'
        )
        scene_lines.append('script = SubResource("1")')
        scene_lines.append("")

        scene_lines.append(
            f'[node name="RwAnalyzer" type="RwAnalyzer" parent="." unique_id={analyzer_uid}]'
        )
        scene_lines.append("")

        scene_lines.append(
            f'[node name="UI" type="CanvasLayer" parent="." unique_id={ui_uid}]'
        )
        scene_lines.append("")

        scene_lines.append(
            f'[node name="VBoxContainer" type="VBoxContainer" parent="UI" unique_id={container_uid}]'
        )
        scene_lines.append("anchors_preset = 15")
        scene_lines.append("anchor_right = 1.0")
        scene_lines.append("anchor_bottom = 1.0")
        scene_lines.append("grow_horizontal = 2")
        scene_lines.append("grow_vertical = 2")
        scene_lines.append("")

        scene_lines.append(
            f'[node name="TitleLabel" type="Label" parent="UI/VBoxContainer" unique_id={title_label_uid}]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Rengine IPL Demo - RenderWare Analysis"')
        scene_lines.append("horizontal_alignment = 1")
        scene_lines.append("")

        scene_lines.append(
            f'[node name="AnalyzeFileButton" type="Button" parent="UI/VBoxContainer" unique_id={analyze_btn_uid}]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Analyze Any File"')
        scene_lines.append("")

        scene_lines.append(
            f'[node name="ParseImgButton" type="Button" parent="UI/VBoxContainer" unique_id={parse_img_btn_uid}]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Parse IMG Archive"')
        scene_lines.append("")

        scene_lines.append(
            f'[node name="ParseDffButton" type="Button" parent="UI/VBoxContainer" unique_id={parse_dff_btn_uid}]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Parse DFF Model"')
        scene_lines.append("")

        scene_lines.append(
            f'[node name="ClearButton" type="Button" parent="UI/VBoxContainer" unique_id={clear_btn_uid}]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Clear Output"')
        scene_lines.append("")

        scene_lines.append(
            f'[node name="OutputLabel" type="Label" parent="UI/VBoxContainer" unique_id={output_label_uid}]'
        )
        scene_lines.append("custom_minimum_size = Vector2(0, 150)")
        scene_lines.append("layout_mode = 2")
        scene_lines.append("size_flags_vertical = 3")
        scene_lines.append(
            'text = "Rengine IPL Demo\\n\\nUse the buttons to analyze RenderWare files."'
        )
        scene_lines.append("autowrap_mode = 2")

        return "\n".join(scene_lines)

    def _generate_rw_demo_godot_scene(self, project_name: str) -> str:
        """Generate a RW parser demo Godot scene with file analysis capabilities"""

        scene_lines = []
        scene_uid = (
            self._generate_uid() if self.generate_uids else "uid://cr7q5mh6aj7ie"
        )
        scene_lines.append(f'[gd_scene format=3 uid="{scene_uid}"]')
        scene_lines.append("")
        scene_lines.append('[sub_resource type="GDScript" id="1"]')
        scene_lines.append('script/source = "')
        scene_lines.append("extends Node")
        scene_lines.append("")
        scene_lines.append("@onready var rw_analyzer = $RwAnalyzer")
        scene_lines.append("@onready var output_label = $UI/VBoxContainer/OutputLabel")
        scene_lines.append("")
        scene_lines.append("func _ready():")
        scene_lines.append("\t# Connect button signals")
        scene_lines.append(
            '\t$UI/VBoxContainer/AnalyzeFileButton.connect(\\"pressed\\", Callable(self, \\"_on_analyze_file_pressed\\"))'
        )
        scene_lines.append(
            '\t$UI/VBoxContainer/ParseImgButton.connect(\\"pressed\\", Callable(self, \\"_on_parse_img_pressed\\"))'
        )
        scene_lines.append(
            '\t$UI/VBoxContainer/ParseDffButton.connect(\\"pressed\\", Callable(self, \\"_on_parse_dff_pressed\\"))'
        )
        scene_lines.append(
            '\t$UI/VBoxContainer/ClearButton.connect(\\"pressed\\", Callable(self, \\"_on_clear_pressed\\"))'
        )
        scene_lines.append("")
        scene_lines.append("func _on_analyze_file_pressed():")
        scene_lines.append("\tvar dialog = FileDialog.new()")
        scene_lines.append("\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE")
        scene_lines.append("\tdialog.access = FileDialog.ACCESS_FILESYSTEM")
        scene_lines.append(
            '\tdialog.connect(\\"file_selected\\", Callable(self, \\"_on_file_selected_for_analysis\\"))'
        )
        scene_lines.append("\tadd_child(dialog)")
        scene_lines.append("\tdialog.popup_centered()")
        scene_lines.append("")
        scene_lines.append("func _on_file_selected_for_analysis(path: String):")
        scene_lines.append("\tvar result = rw_analyzer.analyze_file(path)")
        scene_lines.append(
            '\toutput_label.text = \\"File Analysis:\\\\n\\" + JSON.stringify(result, \\"  \\")'
        )
        scene_lines.append("")
        scene_lines.append("func _on_parse_img_pressed():")
        scene_lines.append("\tvar dialog = FileDialog.new()")
        scene_lines.append("\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE")
        scene_lines.append("\tdialog.access = FileDialog.ACCESS_FILESYSTEM")
        scene_lines.append(
            '\tdialog.filters = PackedStringArray([\\"*.img ; IMG Archive Files\\"])'
        )
        scene_lines.append(
            '\tdialog.connect(\\"file_selected\\", Callable(self, \\"_on_img_file_selected\\"))'
        )
        scene_lines.append("\tadd_child(dialog)")
        scene_lines.append("\tdialog.popup_centered()")
        scene_lines.append("")
        scene_lines.append("func _on_img_file_selected(path: String):")
        scene_lines.append("\tvar result = rw_analyzer.parse_img_archive(path)")
        scene_lines.append(
            '\toutput_label.text = \\"IMG Archive:\\\\n\\" + JSON.stringify(result, \\"  \\")'
        )
        scene_lines.append("")
        scene_lines.append("func _on_parse_dff_pressed():")
        scene_lines.append("\tvar dialog = FileDialog.new()")
        scene_lines.append("\tdialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE")
        scene_lines.append("\tdialog.access = FileDialog.ACCESS_FILESYSTEM")
        scene_lines.append(
            '\tdialog.filters = PackedStringArray([\\"*.dff ; DFF Model Files\\"])'
        )
        scene_lines.append(
            '\tdialog.connect(\\"file_selected\\", Callable(self, \\"_on_dff_file_selected\\"))'
        )
        scene_lines.append("\tadd_child(dialog)")
        scene_lines.append("\tdialog.popup_centered()")
        scene_lines.append("")
        scene_lines.append("func _on_dff_file_selected(path: String):")
        scene_lines.append("\tvar result = rw_analyzer.parse_dff_model(path)")
        scene_lines.append(
            '\toutput_label.text = \\"DFF Model:\\\\n\\" + JSON.stringify(result, \\"  \\")'
        )
        scene_lines.append("")
        scene_lines.append("func _on_clear_pressed():")
        scene_lines.append('\toutput_label.text = \\"Select a file to analyze...\\"')
        scene_lines.append('"')
        scene_lines.append("")

        demo_node_uid = (
            self._generate_unique_id() if self.generate_uids else "160704174"
        )
        analyzer_uid = (
            self._generate_unique_id() if self.generate_uids else "1675776169"
        )
        ui_uid = self._generate_unique_id() if self.generate_uids else "1795349199"
        container_uid = (
            self._generate_unique_id() if self.generate_uids else "376198662"
        )
        title_label_uid = (
            self._generate_unique_id() if self.generate_uids else "40936362"
        )
        analyze_btn_uid = (
            self._generate_unique_id() if self.generate_uids else "1798682199"
        )
        parse_img_btn_uid = (
            self._generate_unique_id() if self.generate_uids else "1423067493"
        )
        parse_dff_btn_uid = (
            self._generate_unique_id() if self.generate_uids else "811492032"
        )
        clear_btn_uid = (
            self._generate_unique_id() if self.generate_uids else "2114485243"
        )
        output_label_uid = (
            self._generate_unique_id() if self.generate_uids else "882303745"
        )

        scene_lines.append(
            f'[node name="{project_name}RWDemo" type="Node" unique_id={demo_node_uid}]'
        )
        scene_lines.append('script = SubResource("1")')
        scene_lines.append("")

        scene_lines.append(
            f'[node name="RwAnalyzer" type="RwAnalyzer" parent="." unique_id={analyzer_uid}]'
        )
        scene_lines.append("")

        scene_lines.append(
            f'[node name="UI" type="CanvasLayer" parent="." unique_id={ui_uid}]'
        )
        scene_lines.append("")

        scene_lines.append(
            f'[node name="VBoxContainer" type="VBoxContainer" parent="UI" unique_id={container_uid}]'
        )
        scene_lines.append("anchors_preset = 15")
        scene_lines.append("anchor_right = 1.0")
        scene_lines.append("anchor_bottom = 1.0")
        scene_lines.append("grow_horizontal = 2")
        scene_lines.append("grow_vertical = 2")
        scene_lines.append("")

        scene_lines.append(
            f'[node name="TitleLabel" type="Label" parent="UI/VBoxContainer" unique_id={title_label_uid}]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Rengine RenderWare Parser Demo"')
        scene_lines.append("horizontal_alignment = 1")
        scene_lines.append("")

        scene_lines.append(
            f'[node name="AnalyzeFileButton" type="Button" parent="UI/VBoxContainer" unique_id={analyze_btn_uid}]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Analyze Any File"')
        scene_lines.append("")

        scene_lines.append(
            f'[node name="ParseImgButton" type="Button" parent="UI/VBoxContainer" unique_id={parse_img_btn_uid}]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Parse IMG Archive"')
        scene_lines.append("")

        scene_lines.append(
            f'[node name="ParseDffButton" type="Button" parent="UI/VBoxContainer" unique_id={parse_dff_btn_uid}]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Parse DFF Model"')
        scene_lines.append("")

        scene_lines.append(
            f'[node name="ClearButton" type="Button" parent="UI/VBoxContainer" unique_id={clear_btn_uid}]'
        )
        scene_lines.append("layout_mode = 2")
        scene_lines.append('text = "Clear Output"')
        scene_lines.append("")

        scene_lines.append(
            f'[node name="OutputLabel" type="Label" parent="UI/VBoxContainer" unique_id={output_label_uid}]'
        )
        scene_lines.append("custom_minimum_size = Vector2(0, 100)")
        scene_lines.append("layout_mode = 2")
        scene_lines.append("size_flags_vertical = 3")
        scene_lines.append('text = "Select a file to analyze..."')
        scene_lines.append("autowrap_mode = 2")

        return "\n".join(scene_lines)

    def _generate_combined_godot_scene(
        self,
        all_ipl_data: Dict[str, Dict],
        project_name: str,
        model_map: Optional[Dict[int, Dict[str, Any]]] = None,
        csv_available: bool = False,
        asset_conversion_result: Optional[Dict[str, Any]] = None,
    ) -> str:
        """Generate a combined Godot scene with all IPL data"""

        scene_lines = []

        # Add script subresource
        scene_lines.append('[sub_resource type="GDScript" id="1"]')
        scene_lines.append('script/source = """')
        scene_lines.append("extends Node")
        scene_lines.append("")
        scene_lines.append(f"# {project_name} - Combined IPL Map Data")
        scene_lines.append(f"# Generated from {len(all_ipl_data)} IPL files")
        scene_lines.append("")
        scene_lines.append("func _ready():")
        scene_lines.append("\tpass")
        scene_lines.append('"""')
        scene_lines.append("")

        # Count load steps (sub_resources + ext_resources)
        load_steps = 1  # 1 sub_resource (the GDScript)

        scene_lines.insert(0, f"[gd_scene load_steps={load_steps} format=3]")
        scene_lines.insert(1, "")

        # Root node
        scene_lines.append(f'[node name="{project_name}" type="Node"]')
        scene_lines.append('script = SubResource("1")')
        scene_lines.append("")

        # Add nodes for each IPL file
        total_instances = 0

        for ipl_filename, ipl_data in all_ipl_data.items():
            ipl_name = os.path.splitext(ipl_filename)[0]
            scene_lines.append(f'[node name="{ipl_name}" type="Node" parent="."]')
            scene_lines.append("")

            # Add instances from this IPL
            if "sections" in ipl_data and "inst" in ipl_data["sections"]:
                for entry in ipl_data["sections"]["inst"]["entries"]:
                    if "model_name" in entry or "id" in entry:
                        # Skip objects not in model map when CSV is the single source of truth
                        if csv_available and model_map and entry["id"] not in model_map:
                            continue

                        total_instances += 1

                        # Try to get proper model name from model data
                        model_name = entry.get("model_name", f"model_{entry['id']}")
                        if model_map and entry["id"] in model_map:
                            model_data = model_map[entry["id"]]
                            model_name = model_data.get("model_name", model_name)

                        sanitized_model_name = self._sanitize_node_name(model_name)
                        node_name = f"{sanitized_model_name}_{total_instances}"

                        # Check if we have converted assets for this model
                        has_mesh = False
                        mesh_resource_path = ""
                        if (
                            asset_conversion_result
                            and "converted_models" in asset_conversion_result
                        ):
                            converted_model = asset_conversion_result[
                                "converted_models"
                            ].get(entry["id"])
                            if converted_model and "obj_file" in converted_model:
                                # Use MeshInstance3D instead of Node3D
                                mesh_resource_path = f"res://assets/{model_name}.obj"
                                has_mesh = True

                        if has_mesh:
                            scene_lines.append(
                                f'[node name="{node_name}" type="MeshInstance3D" parent="{ipl_name}"]'
                            )
                            scene_lines.append(
                                f'mesh = ExtResource("{mesh_resource_path}")'
                            )
                        else:
                            scene_lines.append(
                                f'[node name="{node_name}" type="Node3D" parent="{ipl_name}"]'
                            )

                        scene_lines.append(
                            f"transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {entry['pos_x']:.6f}, {entry['pos_y']:.6f}, {entry['pos_z']:.6f})"
                        )

                        # Add metadata
                        scene_lines.append(f"metadata/model_id = {entry['id']}")
                        scene_lines.append(
                            f'metadata/model_name = "{self._escape_string_for_godot(model_name)}"'
                        )
                        scene_lines.append(f"metadata/interior = {entry['interior']}")

                        # Set LOD bias based on object size for Godot's automatic mesh LOD
                        lod_bias = 1.0  # Default: normal LOD behavior
                        if model_data and model_data.get("radius"):
                            # Larger objects maintain detail longer, smaller objects transition sooner
                            radius = model_data["radius"]
                            if radius < 5:
                                lod_bias = (
                                    0.5  # Small objects: transition to LOD sooner
                                )
                            elif radius < 15:
                                lod_bias = (
                                    0.8  # Medium objects: slight early transition
                                )
                            elif radius < 50:
                                lod_bias = 1.2  # Large objects: maintain detail longer
                            else:
                                lod_bias = (
                                    1.5  # Very large objects: keep high detail longest
                                )

                        # Set LOD bias on MeshInstance3D nodes
                        if has_mesh:
                            scene_lines.append(f"lod_bias = {lod_bias}")

                        # Keep metadata for informational purposes
                        scene_lines.append(f"metadata/lod_bias = {lod_bias}")
                        scene_lines.append(
                            'metadata/lod_info = "Godot automatic mesh LOD with custom bias"'
                        )
                        scene_lines.append(
                            f'metadata/ipl_source = "{self._escape_string_for_godot(ipl_filename)}"'
                        )

                        # Add additional metadata from CSV/IDE data
                        if model_data:
                            if model_data.get("texture_name"):
                                scene_lines.append(
                                    f'metadata/texture_name = "{self._escape_string_for_godot(model_data["texture_name"])}"'
                                )
                            if model_data.get("definition_file"):
                                scene_lines.append(
                                    f'metadata/definition_file = "{self._escape_string_for_godot(model_data["definition_file"])}"'
                                )
                            if model_data.get("model_file"):
                                scene_lines.append(
                                    f'metadata/model_file = "{self._escape_string_for_godot(model_data["model_file"])}"'
                                )
                        if "has_collision" in model_data:
                            scene_lines.append(
                                f'metadata/has_collision = "{model_data["has_collision"]}"'
                            )
                        if "breaks_on_hit" in model_data:
                            scene_lines.append(
                                f'metadata/breaks_on_hit = "{model_data["breaks_on_hit"]}"'
                            )
                        if "has_animation" in model_data:
                            scene_lines.append(
                                f'metadata/has_animation = "{model_data["has_animation"]}"'
                            )
                            if model_data.get("radius"):
                                scene_lines.append(
                                    f"metadata/radius = {model_data['radius']}"
                                )
                            if model_data.get("border_box_length"):
                                scene_lines.append(
                                    f"metadata/border_box_length = {model_data['border_box_length']}"
                                )
                            if model_data.get("border_box_width"):
                                scene_lines.append(
                                    f"metadata/border_box_width = {model_data['border_box_width']}"
                                )
                            if model_data.get("border_box_height"):
                                scene_lines.append(
                                    f"metadata/border_box_height = {model_data['border_box_height']}"
                                )
                            if model_data.get("tags"):
                                scene_lines.append(
                                    f'metadata/tags = "{self._escape_string_for_godot(model_data["tags"])}"'
                                )

                        scene_lines.append("")

            # Add zones from this IPL
            if "sections" in ipl_data and "zone" in ipl_data["sections"]:
                for entry in ipl_data["sections"]["zone"]["entries"]:
                    if "name" in entry:
                        center_x = (entry["min_x"] + entry["max_x"]) / 2
                        center_y = (entry["min_y"] + entry["max_y"]) / 2
                        center_z = (entry["min_z"] + entry["max_z"]) / 2
                        size_x = abs(entry["max_x"] - entry["min_x"])
                        size_y = abs(entry["max_y"] - entry["min_y"])
                        size_z = abs(entry["max_z"] - entry["max_z"])

                        zone_name = f"{entry['name']}_zone"
                        scene_lines.append(
                            f'[node name="{zone_name}" type="Area3D" parent="{ipl_name}"]'
                        )
                        scene_lines.append(
                            f"transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {center_x:.6f}, {center_y:.6f}, {center_z:.6f})"
                        )

                        # Add metadata
                        scene_lines.append(f"metadata/zone_type = {entry['type']}")
                        scene_lines.append(
                            f'metadata/level = "{self._escape_string_for_godot(entry["level"])}"'
                        )
                        scene_lines.append(
                            f'metadata/text = "{self._escape_string_for_godot(entry["text"])}"'
                        )
                        scene_lines.append(
                            f'metadata/ipl_source = "{self._escape_string_for_godot(ipl_filename)}"'
                        )

                        # Add CollisionShape3D as child
                        scene_lines.append(
                            f'[node name="CollisionShape3D" type="CollisionShape3D" parent="{zone_name}"]'
                        )
                        scene_lines.append('[sub_resource type="BoxShape3D" id="2"]')
                        scene_lines.append(
                            f"size = Vector3({size_x:.6f}, {size_y:.6f}, {size_z:.6f})"
                        )
                        scene_lines.append('shape = SubResource("2")')
                        scene_lines.append("")

        return "\n".join(scene_lines)

    def _parse_ipl_file(self, ipl_file: str) -> Dict[str, Any]:
        """Parse IPL file and extract sections"""
        try:
            # First try to read as binary IPL file
            with open(ipl_file, "rb") as f:
                header = f.read(4)
                if header == b"bnry":
                    return self._parse_binary_ipl_file(ipl_file)
        except OSError:
            pass

        # Fall back to text IPL parsing
        with open(ipl_file, encoding="utf-8", errors="ignore") as f:
            content = f.read()

        lines = content.split("\n")
        sections = {}
        current_section = None

        for line in lines:
            line = line.strip()
            if line.startswith("#") or not line:
                continue
            if line.endswith(","):
                line = line[:-1]

            # Check for section headers
            if line in [
                "inst",
                "zone",
                "occl",
                "pick",
                "path",
                "tcyc",
                "mult",
                "grge",
                "enex",
                "cars",
                "jump",
            ]:
                current_section = line
                sections[current_section] = {"entries": [], "count": 0}
            elif current_section and line:
                # Parse entry based on section type
                entry = self._parse_ipl_entry(current_section, line)
                if entry:
                    sections[current_section]["entries"].append(entry)
                    sections[current_section]["count"] += 1

        return {"file": ipl_file, "sections": sections}

    def _parse_binary_ipl_file(self, ipl_file: str) -> Dict[str, Any]:
        """Parse binary IPL file (GTA SA format)"""
        import struct

        sections = {"inst": {"entries": [], "count": 0}}

        with open(ipl_file, "rb") as f:
            data = f.read()

        if len(data) < 8 or data[:4] != b"bnry":
            return {"file": ipl_file, "sections": sections}

        # Read number of instances
        num_instances = struct.unpack("<I", data[4:8])[0]

        # Instances start at offset 0x4C in GTA SA IPL files (after 12 bytes of padding)
        offset = 0x4C

        for _ in range(min(num_instances, 1000)):  # Limit to prevent infinite loops
            try:
                if offset + 64 > len(
                    data
                ):  # Need at least 64 bytes for instance + name
                    break

                # Read instance data (GTA SA binary IPL format)
                # 7 floats (28 bytes) + 3 ints (12 bytes) = 40 bytes for instance data
                instance_data = data[offset : offset + 40]
                (
                    pos_x,
                    pos_y,
                    pos_z,
                    rot_x,
                    rot_y,
                    rot_z,
                    rot_w,
                    model_id,
                    interior,
                    lod,
                ) = struct.unpack("<fffffffIII", instance_data)

                # Try to extract model name from binary IPL data
                # Some IPL files contain model names, others don't
                model_name = None

                # Read model name (24 bytes after instance data)
                name_offset = offset + 40
                if name_offset + 24 <= len(data):
                    model_name_data = data[name_offset : name_offset + 24]

                    # Try to decode as ASCII first
                    try:
                        decoded_name = (
                            model_name_data.split(b"\x00")[0]
                            .decode("ascii", errors="ignore")
                            .strip()
                        )

                        # Validate the extracted name
                        if (
                            decoded_name
                            and len(decoded_name) >= 2
                            and len(decoded_name) <= 23  # reasonable name length
                            and all(
                                ord(c) >= 32 and ord(c) <= 126 for c in decoded_name
                            )
                        ):  # printable ASCII
                            model_name = decoded_name
                    except (UnicodeDecodeError, IndexError):
                        pass

                entry = {
                    "id": int(model_id),
                    "interior": int(interior),
                    "pos_x": pos_x,
                    "pos_y": pos_y,
                    "pos_z": pos_z,
                    "rot_x": rot_x,
                    "rot_y": rot_y,
                    "rot_z": rot_z,
                    "rot_w": rot_w,
                    "lod": int(lod),
                }

                # Only set model_name if we successfully extracted a valid one
                if model_name:
                    entry["model_name"] = model_name
                sections["inst"]["entries"].append(entry)
                sections["inst"]["count"] += 1

                offset += 64  # Each instance is 64 bytes (40 + 24)

            except (struct.error, UnicodeDecodeError):
                break

        return {"file": ipl_file, "sections": sections}

    def _parse_ipl_entry(self, section: str, line: str) -> Optional[Dict[str, Any]]:
        """Parse a single IPL entry based on section type"""
        try:
            parts = line.split(",")

            if section == "inst":  # Instances
                if len(parts) >= 11:
                    return {
                        "id": int(parts[0]),
                        "model_name": parts[1].strip(),
                        "interior": int(parts[2]) if parts[2].strip() else 0,
                        "pos_x": float(parts[3]),
                        "pos_y": float(parts[4]),
                        "pos_z": float(parts[5]),
                        "rot_x": float(parts[6]),
                        "rot_y": float(parts[7]),
                        "rot_z": float(parts[8]),
                        "rot_w": float(parts[9]) if len(parts) > 9 else 1.0,
                        "lod": int(parts[10])
                        if len(parts) > 10 and parts[10].strip()
                        else -1,
                    }

            elif section == "zone":  # Zones
                if len(parts) >= 10:
                    return {
                        "name": parts[0].strip(),
                        "type": int(parts[1]),
                        "min_x": float(parts[2]),
                        "min_y": float(parts[3]),
                        "min_z": float(parts[4]),
                        "max_x": float(parts[5]),
                        "max_y": float(parts[6]),
                        "max_z": float(parts[7]),
                        "level": parts[8].strip() if len(parts) > 8 else "",
                        "text": parts[9].strip() if len(parts) > 9 else "",
                    }

            # For other sections, just return the raw data
            return {"raw": parts}

        except (ValueError, IndexError):
            self.logger.warning(f"Failed to parse IPL entry: {line}")
            return None

    def _generate_godot_scene(
        self,
        ipl_data: Dict[str, Any],
        ipl_file: str,
        template_file: Optional[str] = None,
        model_map: Optional[Dict[int, Dict[str, Any]]] = None,
        csv_available: bool = False,
        asset_conversion_result: Optional[Dict[str, Any]] = None,
    ) -> str:
        """Generate Godot .tscn scene file content"""

        scene_lines = []
        scene_lines.append("[gd_scene load_steps=3 format=3]")
        scene_lines.append("")

        # Add script subresource
        scene_lines.append('[sub_resource type="GDScript" id="1"]')
        scene_lines.append('script/source = """')
        scene_lines.append("extends Node")
        scene_lines.append("")
        scene_lines.append("# IPL Map Data")
        scene_lines.append(f"# Original file: {os.path.basename(ipl_file)}")
        scene_lines.append("")
        scene_lines.append("func _ready():")
        scene_lines.append("\tpass")
        scene_lines.append('"""')
        scene_lines.append("")

        # Root node
        scene_lines.append('[node name="IPLMap" type="Node"]')
        scene_lines.append('script = SubResource("1")')
        scene_lines.append("")

        # Generate nodes for instances
        if "sections" in ipl_data and "inst" in ipl_data["sections"]:
            instance_count = 0
            for entry in ipl_data["sections"]["inst"]["entries"]:
                if "model_name" in entry or "id" in entry:
                    # Skip objects not in model map when CSV is the single source of truth
                    if csv_available and model_map and entry["id"] not in model_map:
                        continue

                    instance_count += 1

                    # Try to get proper model name from model data
                    model_name = entry.get("model_name", f"model_{entry['id']}")
                    if model_map and entry["id"] in model_map:
                        model_data = model_map[entry["id"]]
                        model_name = model_data.get("model_name", model_name)

                    sanitized_model_name = self._sanitize_node_name(model_name)
                    node_name = f"{sanitized_model_name}_{instance_count}"

                    # Check if we have converted assets for this model
                    has_mesh = False
                    mesh_resource_path = ""
                    if (
                        asset_conversion_result
                        and "converted_models" in asset_conversion_result
                    ):
                        converted_model = asset_conversion_result[
                            "converted_models"
                        ].get(entry["id"])
                        if converted_model and "obj_file" in converted_model:
                            # Use MeshInstance3D instead of Node3D
                            mesh_resource_path = f"res://assets/{model_name}.obj"
                            has_mesh = True

                    if has_mesh:
                        scene_lines.append(
                            f'[node name="{node_name}" type="MeshInstance3D" parent="."]'
                        )
                        scene_lines.append(
                            f'mesh = ExtResource("{mesh_resource_path}")'
                        )
                    else:
                        scene_lines.append(
                            f'[node name="{node_name}" type="Node3D" parent="."]'
                        )

                    scene_lines.append(
                        f"transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {entry['pos_x']:.6f}, {entry['pos_y']:.6f}, {entry['pos_z']:.6f})"
                    )

                    # Add metadata
                    scene_lines.append(f"metadata/model_id = {entry['id']}")
                    scene_lines.append(
                        f'metadata/model_name = "{self._escape_string_for_godot(model_name)}"'
                    )
                    scene_lines.append(f"metadata/interior = {entry['interior']}")

                    # Set LOD bias based on object size for Godot's automatic mesh LOD
                    lod_bias = 1.0  # Default: normal LOD behavior
                    if model_data and model_data.get("radius"):
                        # Larger objects maintain detail longer, smaller objects transition sooner
                        radius = model_data["radius"]
                        if radius < 5:
                            lod_bias = 0.5  # Small objects: transition to LOD sooner
                        elif radius < 15:
                            lod_bias = 0.8  # Medium objects: slight early transition
                        elif radius < 50:
                            lod_bias = 1.2  # Large objects: maintain detail longer
                        else:
                            lod_bias = (
                                1.5  # Very large objects: keep high detail longest
                            )

                    # Set LOD bias on MeshInstance3D nodes
                    if has_mesh:
                        scene_lines.append(f"lod_bias = {lod_bias}")

                    # Keep metadata for informational purposes
                    scene_lines.append(f"metadata/lod_bias = {lod_bias}")
                    scene_lines.append(
                        'metadata/lod_info = "Godot automatic mesh LOD with custom bias"'
                    )

                    # Add additional metadata from CSV/IDE data
                    if model_data:
                        if model_data.get("texture_name"):
                            scene_lines.append(
                                f'metadata/texture_name = "{self._escape_string_for_godot(model_data["texture_name"])}"'
                            )
                        if model_data.get("definition_file"):
                            scene_lines.append(
                                f'metadata/definition_file = "{self._escape_string_for_godot(model_data["definition_file"])}"'
                            )
                        if model_data.get("model_file"):
                            scene_lines.append(
                                f'metadata/model_file = "{self._escape_string_for_godot(model_data["model_file"])}"'
                            )
                        if "has_collision" in model_data:
                            scene_lines.append(
                                f'metadata/has_collision = "{model_data["has_collision"]}"'
                            )
                        if "breaks_on_hit" in model_data:
                            scene_lines.append(
                                f'metadata/breaks_on_hit = "{model_data["breaks_on_hit"]}"'
                            )
                        if "has_animation" in model_data:
                            scene_lines.append(
                                f'metadata/has_animation = "{model_data["has_animation"]}"'
                            )
                        if model_data.get("radius"):
                            scene_lines.append(
                                f"metadata/radius = {model_data['radius']}"
                            )
                        if model_data.get("border_box_length"):
                            scene_lines.append(
                                f"metadata/border_box_length = {model_data['border_box_length']}"
                            )
                        if model_data.get("border_box_width"):
                            scene_lines.append(
                                f"metadata/border_box_width = {model_data['border_box_width']}"
                            )
                        if model_data.get("border_box_height"):
                            scene_lines.append(
                                f"metadata/border_box_height = {model_data['border_box_height']}"
                            )
                        if model_data.get("tags"):
                            scene_lines.append(
                                f'metadata/tags = "{self._escape_string_for_godot(model_data["tags"])}"'
                            )

                    scene_lines.append("")

        # Generate nodes for zones (as Area3D)
        if "sections" in ipl_data and "zone" in ipl_data["sections"]:
            zone_count = 0
            for entry in ipl_data["sections"]["zone"]["entries"]:
                if "name" in entry:
                    zone_count += 1
                    center_x = (entry["min_x"] + entry["max_x"]) / 2
                    center_y = (entry["min_y"] + entry["max_y"]) / 2
                    center_z = (entry["min_z"] + entry["max_z"]) / 2
                    size_x = abs(entry["max_x"] - entry["min_x"])
                    size_y = abs(entry["max_y"] - entry["min_y"])
                    size_z = abs(entry["max_z"] - entry["max_z"])

                    zone_name = f"{entry['name']}_{zone_count}"
                    scene_lines.append(
                        f'[node name="{zone_name}" type="Area3D" parent="."]'
                    )
                    scene_lines.append(
                        f"transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, {center_x:.6f}, {center_y:.6f}, {center_z:.6f})"
                    )

                    # Add metadata
                    scene_lines.append(f"metadata/zone_type = {entry['type']}")
                    scene_lines.append(f'metadata/level = "{entry["level"]}"')
                    scene_lines.append(f'metadata/text = "{entry["text"]}"')

                    # Add CollisionShape3D as child
                    scene_lines.append(
                        f'[node name="CollisionShape3D" type="CollisionShape3D" parent="{zone_name}"]'
                    )
                    scene_lines.append('[sub_resource type="BoxShape3D" id="2"]')
                    scene_lines.append(
                        f"size = Vector3({size_x:.6f}, {size_y:.6f}, {size_z:.6f})"
                    )
                    scene_lines.append('shape = SubResource("2")')
                    scene_lines.append("")

        return "\n".join(scene_lines)
