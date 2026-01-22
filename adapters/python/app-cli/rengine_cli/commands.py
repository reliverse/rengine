"""
CLI command implementations
"""

import glob
import os
from typing import Any, Dict, List, cast

from .base import BaseCommand, DependencyManager, OutputFormatter
from .handlers import (
    COLHandler,
    DFFConverterHandler,
    DFFHandler,
    IMGHandler,
    IPLHandler,
    RWHandler,
    TXDConverterHandler,
    TXDHandler,
)


# IMG Commands
class IMGInfoCommand(BaseCommand):
    """IMG info command"""

    def get_name(self) -> str:
        return "img-info"

    def get_description(self) -> str:
        return "Show IMG archive information"

    def execute(self, args) -> int:
        try:
            handler = IMGHandler(self.logger, DependencyManager())
            info = handler.get_info(args.file, args.detailed)

            if not getattr(args, "quiet", False):
                print(f"IMG Archive: {info['file']}")
                print(f"Version: {info['version']}")
                print(f"Files: {info['file_count']}")
                print(f"Size: {info['size']:,} bytes")

                if args.detailed:
                    print("\nFile Types:")
                    for ext, count in info.get("file_types", {}).items():
                        print(f"  {ext}: {count}")

                    print("\nRenderWare Versions:")
                    for version, count in info.get("rw_versions", {}).items():
                        print(f"  {version}: {count}")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


class IMGListCommand(BaseCommand):
    """IMG list command"""

    def get_name(self) -> str:
        return "img-list"

    def get_description(self) -> str:
        return "List files in IMG archive"

    def execute(self, args) -> int:
        try:
            handler = IMGHandler(self.logger, DependencyManager())
            result = handler.list_files(args.file, args.filter)

            if args.format == "json":
                OutputFormatter.output_data(result, args, "json")
            else:
                print(f"Files in {result['file']} ({len(result['entries'])} entries):")
                print("-" * 60)
                for entry in sorted(result["entries"], key=lambda x: x["name"]):
                    print(f"  {entry['name']:<50} {entry['size']:>10,} bytes")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


class IMGExtractCommand(BaseCommand):
    """IMG extract command"""

    def get_name(self) -> str:
        return "img-extract"

    def get_description(self) -> str:
        return "Extract files from IMG archive"

    def execute(self, args) -> int:
        try:
            handler = IMGHandler(self.logger, DependencyManager())
            result = handler.extract_files(
                args.file,
                args.output,
                args.files,
                args.overwrite,
                getattr(args, "quiet", False),
            )

            if not getattr(args, "quiet", False):
                print(f"\nExtracted {result['extracted']} files to {args.output}")
                if result["skipped"] > 0:
                    print(f"Skipped {result['skipped']} existing files")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


# DFF Commands
class DFFInfoCommand(BaseCommand):
    """DFF info command"""

    def get_name(self) -> str:
        return "dff-info"

    def get_description(self) -> str:
        return "Show DFF model information"

    def execute(self, args) -> int:
        try:
            deps = DependencyManager()
            handler = DFFHandler(self.logger)
            info = handler.get_info(args.file, deps)

            if not getattr(args, "quiet", False):
                print(f"DFF File: {info['file']}")
                print(f"Size: {info['size']:,} bytes")
                print(f"Chunks: {info['chunks']}")
                if "rw_version" in info:
                    print(f"RenderWare: {info['rw_version']}")
                else:
                    print("RenderWare: Version detection not available")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


class DFFAnalyzeCommand(BaseCommand):
    """DFF analyze command"""

    def get_name(self) -> str:
        return "dff-analyze"

    def get_description(self) -> str:
        return "Analyze DFF model structure"

    def execute(self, args) -> int:
        try:
            deps = DependencyManager()
            handler = DFFHandler(self.logger)
            analysis = handler.analyze_structure(args.file, deps)

            if args.format == "json":
                OutputFormatter.output_data(analysis, args, "json")
            else:
                print(f"DFF Analysis: {args.file}")
                print(f"Size: {analysis['size']:,} bytes")
                print(f"Chunks: {len(analysis['chunks'])}")

                def print_chunks(chunks, indent=0):
                    prefix = "  " * indent
                    for i, chunk in enumerate(chunks):
                        if i >= 20 and indent == 0:  # Limit top level chunks
                            print(f"{prefix}... and {len(chunks) - 20} more chunks")
                            break
                        print(
                            f"{prefix}{chunk['type_name']} ({chunk['type']}) - {chunk['size']} bytes - {chunk['version_name']}"
                        )
                        if "children" in chunk and chunk["children"]:
                            print_chunks(chunk["children"], indent + 1)

                print("\nChunk Structure:")
                print_chunks(analysis["chunks"])

            return 0
        except Exception as e:
            return self.handle_error(e, args)


# TXD Commands
class TXDInfoCommand(BaseCommand):
    """TXD info command"""

    def get_name(self) -> str:
        return "txd-info"

    def get_description(self) -> str:
        return "Show TXD texture information"

    def execute(self, args) -> int:
        try:
            deps = DependencyManager()
            handler = TXDHandler(self.logger)
            info = handler.get_info(args.file, deps)

            if not getattr(args, "quiet", False):
                print(f"TXD File: {args.file}")
                print(f"Size: {info['size']:,} bytes")
                print(f"Chunks: {info['chunks']}")
                if "rw_version" in info:
                    print(f"RenderWare: {info['rw_version']}")
                else:
                    print("RenderWare: Version detection not available")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


class TXDExtractCommand(BaseCommand):
    """TXD extract command"""

    def get_name(self) -> str:
        return "txd-extract"

    def get_description(self) -> str:
        return "Extract textures from TXD"

    def execute(self, args) -> int:
        try:
            deps = DependencyManager()
            handler = TXDHandler(self.logger)
            result = handler.extract_textures(args.file, args.output, deps, args.format)

            if not getattr(args, "quiet", False):
                print(
                    f"Extracted {result.get('extracted', 0)} textures to {args.output}"
                )
                if "message" in result:
                    print(f"Note: {result['message']}")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


# COL Commands
class COLInfoCommand(BaseCommand):
    """COL info command"""

    def get_name(self) -> str:
        return "col-info"

    def get_description(self) -> str:
        return "Show COL collision information"

    def execute(self, args) -> int:
        try:
            deps = DependencyManager()
            handler = COLHandler(self.logger)
            info = handler.get_info(args.file, deps)

            if not getattr(args, "quiet", False):
                print(f"COL File: {info['file']}")
                print(f"Size: {info['size']:,} bytes")
                print(f"Version: {info.get('version', 'Unknown')}")
                print(f"Collision Objects: {info['collision_objects']}")

                if getattr(args, "detailed", False):
                    print(f"Spheres: {info.get('spheres', 0)}")
                    print(f"Boxes: {info.get('boxes', 0)}")
                    print(f"Meshes: {info.get('meshes', 0)}")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


class COLAnalyzeCommand(BaseCommand):
    """COL analyze command"""

    def get_name(self) -> str:
        return "col-analyze"

    def get_description(self) -> str:
        return "Analyze COL collision structure"

    def execute(self, args) -> int:
        try:
            deps = DependencyManager()
            handler = COLHandler(self.logger)
            analysis = handler.analyze_structure(args.file, deps)

            if args.format == "json":
                OutputFormatter.output_data(analysis, args, "json")
            else:
                print(f"COL Analysis: {args.file}")
                print(f"Size: {analysis['size']:,} bytes")
                print(f"Collision Objects: {len(analysis['collision_objects'])}")

                print("\nCollision Objects:")
                for col in analysis["collision_objects"]:
                    print(f"  {col['index']}: {col['model_name']}")
                    print(
                        f"    Spheres: {col['spheres']}, Boxes: {col['boxes']}, Meshes: {col['meshes']}"
                    )

            return 0
        except Exception as e:
            return self.handle_error(e, args)


# IPL Commands
class IPLInfoCommand(BaseCommand):
    """IPL info command"""

    def get_name(self) -> str:
        return "ipl-info"

    def get_description(self) -> str:
        return "Show IPL map information"

    def execute(self, args) -> int:
        try:
            deps = DependencyManager()
            handler = IPLHandler(self.logger)
            info = handler.get_info(args.file, deps)

            if not getattr(args, "quiet", False):
                print(f"IPL File: {info['file']}")
                print(f"Size: {info['size']:,} bytes")
                print(f"Instances: {info['instances']}")
                print(f"Zones: {info['zones']}")
                print(f"Occlusions: {info['occlusions']}")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


class IPLAnalyzeCommand(BaseCommand):
    """IPL analyze command"""

    def get_name(self) -> str:
        return "ipl-analyze"

    def get_description(self) -> str:
        return "Analyze IPL map structure"

    def execute(self, args) -> int:
        try:
            deps = DependencyManager()
            handler = IPLHandler(self.logger)
            analysis = handler.analyze_structure(args.file, deps)

            if args.format == "json":
                OutputFormatter.output_data(analysis, args, "json")
            else:
                print(f"IPL Analysis: {args.file}")
                print(f"Sections: {len(analysis['sections'])}")

                for section, data in analysis["sections"].items():
                    print(f"\n{section.upper()} Section ({data['count']} entries):")
                    for entry in data["sample_entries"]:
                        print(f"  {','.join(entry)}")

                    if data["count"] > 5:
                        print(f"  ... and {data['count'] - 5} more entries")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


# RW Commands
class RWAnalyzeCommand(BaseCommand):
    """RW analyze command"""

    def get_name(self) -> str:
        return "rw-analyze"

    def get_description(self) -> str:
        return "Analyze RenderWare file chunks"

    def execute(self, args) -> int:
        try:
            handler = RWHandler(self.logger)
            analysis = handler.analyze_chunks(args.file, args.depth)

            if args.format == "json":
                OutputFormatter.output_data(analysis, args, "json")
            else:

                def print_tree(chunks, indent=0):
                    for chunk in chunks:
                        prefix = "  " * indent
                        print(
                            f"{prefix}{chunk['type_name']} ({chunk['type']}) - {chunk['size']} bytes - {chunk['version_name']}"
                        )
                        if "children" in chunk:
                            print_tree(chunk["children"], indent + 1)

                print(f"RenderWare Analysis: {args.file}")
                print(f"Size: {analysis['size']:,} bytes")
                print(f"Max Depth: {args.depth}")
                print("\nChunk Tree:")
                print_tree(analysis["chunks"])

            return 0
        except Exception as e:
            return self.handle_error(e, args)


# Batch Commands
class BatchAnalyzeCommand(BaseCommand):
    """Batch analyze command"""

    def get_name(self) -> str:
        return "batch-analyze"

    def get_description(self) -> str:
        return "Analyze multiple files"

    def execute(self, args) -> int:
        extensions = {
            "dff": ["*.dff"],
            "txd": ["*.txd"],
            "col": ["*.col"],
            "ipl": ["*.ipl"],
            "img": ["*.img"],
            "all": ["*.dff", "*.txd", "*.col", "*.ipl", "*.img"],
        }

        patterns = extensions.get(args.format, extensions["all"])
        files = []

        for pattern in patterns:
            if args.recursive:
                files.extend(
                    glob.glob(
                        os.path.join(args.directory, "**", pattern), recursive=True
                    )
                )
            else:
                files.extend(glob.glob(os.path.join(args.directory, pattern)))

        if args.max_files:
            files = files[: args.max_files]

        results: Dict[str, Any] = {
            "directory": args.directory,
            "format": args.format,
            "recursive": args.recursive,
            "total_files": len(files),
            "analyzed_files": [],
            "errors": [],
        }

        deps = DependencyManager()

        for file_path in files:
            try:
                file_info = {
                    "path": file_path,
                    "size": os.path.getsize(file_path),
                    "type": os.path.splitext(file_path)[1][1:].upper(),
                }

                # Get basic info based on file type
                ext = str(file_info["type"]).lower()
                if ext == "dff":
                    handler = DFFHandler(self.logger)
                    file_info.update(handler.get_info(file_path, deps))
                elif ext == "txd":
                    handler = TXDHandler(self.logger)
                    file_info.update(handler.get_info(file_path, deps))
                elif ext == "col":
                    handler = COLHandler(self.logger)
                    file_info.update(handler.get_info(file_path, deps))
                elif ext == "ipl":
                    handler = IPLHandler(self.logger)
                    file_info.update(handler.get_info(file_path, deps))
                elif ext == "img":
                    handler = IMGHandler(self.logger, deps)
                    file_info.update(handler.get_info(file_path))

                cast(List[Dict[str, Any]], results["analyzed_files"]).append(file_info)

                if not getattr(args, "quiet", False):
                    print(f"Analyzed: {file_path}")

            except Exception as e:
                cast(List[Dict[str, Any]], results["errors"]).append(
                    {"file": file_path, "error": str(e)}
                )
                if getattr(args, "verbose", False):
                    print(f"Error analyzing {file_path}: {e}")

        if args.output:
            import json

            with open(args.output, "w") as f:
                json.dump(results, f, indent=2)
            if not getattr(args, "quiet", False):
                print(f"\nResults saved to {args.output}")
        else:
            OutputFormatter.output_data(results, args, "json")

        return 0


class BatchExtractIMGsCommand(BaseCommand):
    """Batch extract IMGs command"""

    def get_name(self) -> str:
        return "batch-extract-imgs"

    def get_description(self) -> str:
        return "Extract all IMG archives in directory"

    def execute(self, args) -> int:
        img_files = []
        if args.recursive:
            img_files.extend(
                glob.glob(os.path.join(args.directory, "**", "*.img"), recursive=True)
            )
        else:
            img_files.extend(glob.glob(os.path.join(args.directory, "*.img")))

        if not img_files:
            self.logger.warning(f"No IMG files found in {args.directory}")
            return 0

        total_extracted = 0
        deps = DependencyManager()
        handler = IMGHandler(self.logger, deps)

        for img_file in img_files:
            try:
                # Create output subdirectory for this IMG
                img_name = os.path.splitext(os.path.basename(img_file))[0]
                img_output_dir = os.path.join(args.output, img_name)

                if not getattr(args, "quiet", False):
                    print(f"Extracting {img_file} to {img_output_dir}")

                # Extract all files
                result = handler.extract_files(
                    img_file,
                    img_output_dir,
                    None,
                    args.overwrite,
                    getattr(args, "quiet", False),
                )
                total_extracted += result["extracted"]

            except Exception as e:
                self.logger.error(f"Failed to extract {img_file}: {e}")

        if not getattr(args, "quiet", False):
            print(
                f"\nExtracted {total_extracted} files from {len(img_files)} IMG archives"
            )

        return 0


# Converter Commands
class DFFToOBJCommand(BaseCommand):
    """DFF to OBJ converter command"""

    def get_name(self) -> str:
        return "dff-to-obj"

    def get_description(self) -> str:
        return "Convert DFF model files to OBJ format"

    def execute(self, args) -> int:
        try:
            deps = DependencyManager()
            handler = DFFConverterHandler(self.logger)
            use_csv = not getattr(args, "no_csv", False)
            result = handler.convert_to_obj(args.file, args.output, args.txd, use_csv, deps)

            if not getattr(args, "quiet", False):
                print(f"Converted {args.file} to {args.output}")
                if result.get("csv_used"):
                    print("Used GTA SA CSV data for enhanced model information")
                if args.txd:
                    print(f"Used TXD materials from {args.txd}")
                elif result.get("csv_model_info") and result["csv_model_info"].get("materials_file"):
                    print(f"Auto-detected TXD materials: {result['csv_model_info']['materials_file']}")
                if "mtl_file" in result:
                    print(f"Created material file: {result['mtl_file']}")
                if "textures_extracted" in result:
                    print(f"Extracted {result['textures_extracted']} textures")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


class TXDToMaterialsCommand(BaseCommand):
    """TXD to materials converter command"""

    def get_name(self) -> str:
        return "txd-to-materials"

    def get_description(self) -> str:
        return "Extract textures and materials from TXD files"

    def execute(self, args) -> int:
        try:
            deps = DependencyManager()
            handler = TXDConverterHandler(self.logger)
            result = handler.extract_materials(
                args.file, args.output, args.format, deps
            )

            if not getattr(args, "quiet", False):
                print(
                    f"Extracted {result.get('extracted', 0)} textures from {args.file} to {args.output}"
                )
                if "message" in result:
                    print(f"Note: {result['message']}")

            return 0
        except Exception as e:
            return self.handle_error(e, args)


class IPLToGodotCommand(BaseCommand):
    """IPL to Godot scene/project converter command"""

    def get_name(self) -> str:
        return "ipl-to-godot"

    def get_description(self) -> str:
        return "Convert IPL map files to Godot scene files or complete projects"

    def execute(self, args) -> int:
        try:
            from .modules.ipl_to_godot import IPLToGodotConverter

            input_path = args.input
            output_path = args.output
            project_name = getattr(args, "project_name", "IPLMap")
            ide_files = getattr(args, "ide_files", None)
            convert_assets = getattr(args, "convert_assets", False)
            dff_path = getattr(args, "dff_path", None)
            txd_path = getattr(args, "txd_path", None)

            converter = IPLToGodotConverter(
                self.logger, with_rust=getattr(args, "with_rust", False)
            )

            # Enable asset conversion if requested
            if convert_assets and dff_path:
                dff_paths = [dff_path]
                txd_paths = [txd_path] if txd_path else [dff_path]
                converter.enable_asset_conversion(dff_paths, txd_paths)
            elif convert_assets and not dff_path:
                self.logger.warning("--convert-assets specified but no --dff-path provided")
            convert_assets = getattr(args, "convert_assets", False)
            dff_path = getattr(args, "dff_path", None)
            txd_path = getattr(args, "txd_path", None)

            if os.path.isdir(input_path):
                # Convert directory to full Godot project
                result = converter.convert_directory_to_godot_project(
                    input_path, output_path, project_name, args.template, ide_files
                )

                if not getattr(args, "quiet", False):
                    with_rust = getattr(args, "with_rust", False)
                    print(
                        f"Converted IPL directory {input_path} to Godot project: {output_path}"
                    )
                    print(f"Processed {result.get('ipl_files', 0)} IPL files")
                    print(
                        f"Created {result.get('total_instances', 0)} object instances"
                    )
                    print(f"Created {result.get('total_zones', 0)} zones")
                    if with_rust:
                        print("Included Rust GDExtension integration and demo scenes")
                    else:
                        print(
                            "Generated basic IPL scenes (use --with-rust for full RW analysis features)"
                        )

            else:
                # Convert single file to scene
                result = converter.convert_to_godot(
                    input_path, output_path, args.template, ide_files
                )

                if not getattr(args, "quiet", False):
                    print(f"Converted {input_path} to Godot scene: {output_path}")
                    if "instances" in result:
                        print(f"Created {result['instances']} object instances")
                    if "zones" in result:
                        print(f"Created {result['zones']} zones")
                    if "original_instances" in result and isinstance(result.get("instances"), str):
                        print(f"Original IPL had {result['original_instances']} instances")
                    with_rust = getattr(args, "with_rust", False)
                    if with_rust:
                        print(
                            "Note: --with-rust flag is ignored for single file conversion"
                        )

            return 0
        except Exception as e:
            return self.handle_error(e, args)
