"""
DFF to OBJ Converter Tool module for the GTA Rengine
Provides DFF to OBJ conversion with optional TXD to materials conversion.
"""

from pathlib import Path
import os
import traceback

from PyQt6.QtCore import Qt, pyqtSignal, QThread, QObject
from PyQt6.QtWidgets import (
    QWidget,
    QVBoxLayout,
    QHBoxLayout,
    QLabel,
    QSplitter,
    QSizePolicy,
    QFrame,
    QScrollArea,
    QGroupBox,
    QGridLayout,
    QPushButton,
    QMessageBox,
    QTextEdit,
    QCheckBox,
    QProgressBar,
    QFileDialog,
    QLineEdit,
    QComboBox,
)

from application.common.message_box import message_box
from application.responsive_utils import get_responsive_manager
from application.styles import ModernDarkTheme
from application.debug_system import get_debug_logger, LogCategory

# Import DFF and TXD parsers from local common directory
try:
    from application.common.DFF import dff, Vector, SkinPLG, HAnimPLG, UserData
    from application.common.txd import txd, TextureNative
    debug_logger.info(LogCategory.TOOL, "[DFF to OBJ Converter] DFF/TXD support enabled")
    DFF_TXD_SUPPORT = True
except ImportError as e:
    debug_logger.warning(LogCategory.TOOL, f"[DFF to OBJ Converter] DFF/TXD import failed", {"error": str(e)})
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

# Module-level debug logger
debug_logger = get_debug_logger()


class OBJExportWorker(QObject):
    """Worker thread for OBJ export operations"""

    progress = pyqtSignal(int, str)  # progress percentage, status message
    finished = pyqtSignal(bool, str)  # success, message
    error = pyqtSignal(str)  # error message

    def __init__(self, dff_file, obj_file, convert_txd_materials=False, txd_file=None):
        super().__init__()
        self.dff_file = dff_file
        self.obj_file = obj_file
        self.convert_txd_materials = convert_txd_materials
        self.txd_file = txd_file

    def run(self):
        """Run the OBJ export in a separate thread"""
        try:
            self.progress.emit(0, "Loading DFF file...")

            if not DFF_TXD_SUPPORT:
                raise Exception("DFF/TXD support not available - required libraries not found")

            # Load DFF file
            dff_data = dff()
            dff_data.load_file(self.dff_file)
            self.progress.emit(20, "DFF file loaded successfully")

            # Load TXD file if needed
            txd_data = None
            if self.convert_txd_materials and self.txd_file:
                self.progress.emit(30, "Loading TXD file for materials...")
                txd_data = txd()
                txd_data.load_file(self.txd_file)
                self.progress.emit(40, "TXD file loaded successfully")

            # Export to OBJ
            self.progress.emit(50, "Converting to OBJ format...")
            obj_content = self.convert_dff_to_obj(dff_data, txd_data)

            # Write OBJ file
            self.progress.emit(80, "Writing OBJ file...")
            with open(self.obj_file, 'w') as f:
                f.write(obj_content)

            # Create MTL file if materials are enabled
            if self.convert_txd_materials and txd_data:
                self.progress.emit(90, "Creating material file...")
                mtl_content = self.create_mtl_file(txd_data, os.path.splitext(self.obj_file)[0])
                mtl_file = os.path.splitext(self.obj_file)[0] + '.mtl'
                with open(mtl_file, 'w') as f:
                    f.write(mtl_content)

                # Extract textures if available
                self.extract_textures(txd_data, os.path.dirname(self.obj_file))

            self.progress.emit(100, "Export completed successfully")
            self.finished.emit(True, f"Successfully exported {len(dff_data.atomic_list)} meshes to {self.obj_file}")

        except Exception as e:
            error_msg = f"Export failed: {str(e)}"
            debug_logger.error(LogCategory.TOOL, error_msg, {"traceback": traceback.format_exc()})
            self.error.emit(error_msg)

    def convert_dff_to_obj(self, dff_data, txd_data=None):
        """Convert DFF data to OBJ format based on Blender exporter"""
        obj_lines = []
        obj_lines.append("# OBJ file exported from Rengine DFF Converter")
        obj_lines.append(f"# Original DFF: {os.path.basename(self.dff_file)}")
        obj_lines.append(f"# RW Version: 0x{dff_data.rw_version:08x}")
        obj_lines.append("")

        global_vertex_offset = 1  # OBJ vertices are 1-indexed
        global_normal_offset = 1
        global_uv_offset = 1

        # Material library reference
        if txd_data and self.convert_txd_materials:
            mtl_filename = os.path.splitext(os.path.basename(self.obj_file))[0] + '.mtl'
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

            # Write vertices with transformation applied
            for vertex in geometry.vertices:
                # Apply frame transformation if needed
                transformed_vertex = vertex
                # Note: In full implementation, would apply frame matrix transformation
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
                if layer_idx >= 2:  # Limit to 2 UV layers as per Blender
                    break

                obj_lines.append(f"# UV Layer {layer_idx}")
                for uv in uv_layer:
                    obj_lines.append(f"vt {uv.u:.6f} {1.0 - uv.v:.6f}")  # Flip V coordinate for OBJ

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
                    if self.convert_txd_materials and geometry.materials and mat_idx < len(geometry.materials):
                        material = geometry.materials[mat_idx]
                        if material.is_textured and material.textures:
                            texture_name = material.textures[0].name
                            obj_lines.append(f"usemtl material_{mat_idx}")
                        else:
                            obj_lines.append(f"usemtl material_{mat_idx}")

                    for triangle in triangles:
                        # OBJ face format: vertex/uv/normal
                        v1 = triangle.a + local_vertex_offset
                        v2 = triangle.b + local_vertex_offset
                        v3 = triangle.c + local_vertex_offset

                        if geometry.has_normals and geometry.uv_layers:
                            # vertex/uv/normal format
                            n1 = triangle.a + local_normal_offset
                            n2 = triangle.b + local_normal_offset
                            n3 = triangle.c + local_normal_offset
                            obj_lines.append(f"f {v1}/{v1}/{n1} {v2}/{v2}/{n2} {v3}/{v3}/{n3}")
                        elif geometry.uv_layers:
                            # vertex/uv format
                            obj_lines.append(f"f {v1}/{v1} {v2}/{v2} {v3}/{v3}")
                        elif geometry.has_normals:
                            # vertex//normal format
                            n1 = triangle.a + local_normal_offset
                            n2 = triangle.b + local_normal_offset
                            n3 = triangle.c + local_normal_offset
                            obj_lines.append(f"f {v1}//{n1} {v2}//{n2} {v3}//{n3}")
                        else:
                            # vertex only format
                            obj_lines.append(f"f {v1} {v2} {v3}")

                obj_lines.append("")

        return "\n".join(obj_lines)

    def create_mtl_file(self, txd_data, base_name):
        """Create MTL material file from TXD data"""
        mtl_lines = []
        mtl_lines.append("# MTL file exported from Rengine DFF Converter")
        mtl_lines.append(f"# Original TXD: {os.path.basename(self.txd_file) if self.txd_file else 'Unknown'}")
        mtl_lines.append("")

        for tex_idx, texture in enumerate(txd_data.native_textures):
            material_name = f"material_{tex_idx}"

            mtl_lines.append(f"newmtl {material_name}")

            # Basic material properties (Phong shading model)
            mtl_lines.append("Ns 96.078431")  # Specular exponent
            mtl_lines.append("Ka 1.000000 1.000000 1.000000")  # Ambient color
            mtl_lines.append("Kd 0.640000 0.640000 0.640000")  # Diffuse color
            mtl_lines.append("Ks 0.500000 0.500000 0.500000")  # Specular color
            mtl_lines.append("Ke 0.000000 0.000000 0.000000")  # Emissive color
            mtl_lines.append("Ni 1.000000")  # Optical density (refraction)
            mtl_lines.append("d 1.000000")   # Dissolve (transparency)
            mtl_lines.append("illum 2")      # Illumination model

            # Texture maps
            if hasattr(texture, 'name') and texture.name:
                texture_filename = f"{texture.name}.png"
                mtl_lines.append(f"map_Kd {texture_filename}")  # Diffuse map
                # Could also add normal maps, specular maps, etc. if available

            mtl_lines.append("")

        return "\n".join(mtl_lines)

    def extract_textures(self, txd_data, output_dir):
        """Extract textures from TXD to PNG files"""
        from PyQt6.QtGui import QImage

        extracted_count = 0
        for texture in txd_data.native_textures:
            try:
                # Get first mipmap level (highest quality)
                rgba_data = texture.to_rgba(0)
                if not rgba_data:
                    debug_logger.warning(LogCategory.TOOL, f"No RGBA data for texture {texture.name}")
                    continue

                width = texture.get_width(0)
                height = texture.get_height(0)

                if width == 0 or height == 0:
                    debug_logger.warning(LogCategory.TOOL, f"Invalid dimensions for texture {texture.name}: {width}x{height}")
                    continue

                # Create QImage from RGBA data
                # QImage expects BGRA format for RGBA8888
                image = QImage(width, height, QImage.Format.Format_RGBA8888)

                # Fill image pixel by pixel (this is the correct way for PyQt6)
                for y in range(height):
                    for x in range(width):
                        idx = (y * width + x) * 4
                        if idx + 3 < len(rgba_data):
                            r, g, b, a = rgba_data[idx:idx+4]
                            # Convert to ARGB format for QImage.setPixel
                            pixel_value = (a << 24) | (r << 16) | (g << 8) | b
                            image.setPixel(x, y, pixel_value)

                # Save as PNG
                texture_path = os.path.join(output_dir, f"{texture.name}.png")
                if image.save(texture_path, "PNG"):
                    extracted_count += 1
                    debug_logger.info(LogCategory.TOOL, f"Extracted texture: {texture.name}.png ({width}x{height})")
                else:
                    debug_logger.warning(LogCategory.TOOL, f"Failed to save texture {texture.name}.png")

            except Exception as e:
                debug_logger.warning(LogCategory.TOOL, f"Failed to extract texture {texture.name}: {str(e)}")
                import traceback
                debug_logger.debug(LogCategory.TOOL, f"Texture extraction traceback: {traceback.format_exc()}")

        debug_logger.info(LogCategory.TOOL, f"Texture extraction completed: {extracted_count} textures extracted")


class DFFtoOBJConverterTool(QWidget):
    """DFF to OBJ Converter tool interface"""

    # Signals for tool actions
    tool_action = pyqtSignal(str, str)  # action_name, parameters

    def __init__(self, parent=None):
        super().__init__(parent)
        self.current_dff_file = None
        self.current_txd_file = None
        self.export_worker = None
        self.export_thread = None

        self.setup_ui()

    def setup_ui(self):
        """Setup the main UI layout"""
        rm = get_responsive_manager()
        spacing = rm.get_spacing_config()

        # Main layout
        main_layout = QVBoxLayout(self)
        main_layout.setSpacing(spacing['medium'])

        # Create toolbar
        self.create_toolbar()
        main_layout.addWidget(self.toolbar)

        # Create main content area
        self.create_content_area()
        main_layout.addWidget(self.content_area)

        # Apply styling
        self.apply_styling()

    def create_toolbar(self):
        """Create the toolbar with conversion controls"""
        self.toolbar = QFrame()
        self.toolbar.setFixedHeight(50)
        self.toolbar.setFrameStyle(QFrame.Shape.StyledPanel)

        toolbar_layout = QHBoxLayout(self.toolbar)
        toolbar_layout.setContentsMargins(8, 4, 8, 4)
        toolbar_layout.setSpacing(8)

        # DFF file selection
        dff_layout = QVBoxLayout()
        dff_layout.setSpacing(2)
        dff_label = QLabel("DFF File:")
        dff_label.setStyleSheet("font-weight: bold;")
        self.dff_path_edit = QLineEdit()
        self.dff_path_edit.setPlaceholderText("Select DFF file...")
        self.dff_path_edit.setReadOnly(True)
        self.dff_browse_btn = QPushButton("📁 Browse")
        self.dff_browse_btn.setFixedWidth(80)
        self.dff_browse_btn.clicked.connect(self.browse_dff_file)

        dff_file_layout = QHBoxLayout()
        dff_file_layout.addWidget(self.dff_path_edit)
        dff_file_layout.addWidget(self.dff_browse_btn)

        dff_layout.addWidget(dff_label)
        dff_layout.addLayout(dff_file_layout)

        # TXD file selection
        txd_layout = QVBoxLayout()
        txd_layout.setSpacing(2)
        txd_label = QLabel("TXD File (Optional):")
        txd_label.setStyleSheet("font-weight: bold;")
        self.txd_path_edit = QLineEdit()
        self.txd_path_edit.setPlaceholderText("Select TXD file for materials...")
        self.txd_path_edit.setReadOnly(True)
        self.txd_browse_btn = QPushButton("📁 Browse")
        self.txd_browse_btn.setFixedWidth(80)
        self.txd_browse_btn.clicked.connect(self.browse_txd_file)

        txd_file_layout = QHBoxLayout()
        txd_file_layout.addWidget(self.txd_path_edit)
        txd_file_layout.addWidget(self.txd_browse_btn)

        txd_layout.addWidget(txd_label)
        txd_layout.addLayout(txd_file_layout)

        # Options
        options_layout = QVBoxLayout()
        options_layout.setSpacing(2)
        options_label = QLabel("Options:")
        options_label.setStyleSheet("font-weight: bold;")

        self.convert_materials_checkbox = QCheckBox("Convert TXD to Materials")
        self.convert_materials_checkbox.setChecked(True)
        self.convert_materials_checkbox.setToolTip("Extract textures and create material files from TXD")

        options_layout.addWidget(options_label)
        options_layout.addWidget(self.convert_materials_checkbox)
        options_layout.addStretch()

        # Convert button
        convert_layout = QVBoxLayout()
        convert_layout.setSpacing(2)
        convert_label = QLabel("Output:")
        convert_label.setStyleSheet("font-weight: bold;")

        self.convert_btn = QPushButton("🔄 Convert to OBJ")
        self.convert_btn.setFixedHeight(35)
        self.convert_btn.setStyleSheet("""
            QPushButton {
                background-color: #4CAF50;
                color: white;
                font-weight: bold;
                border-radius: 4px;
            }
            QPushButton:hover {
                background-color: #45a049;
            }
            QPushButton:disabled {
                background-color: #cccccc;
                color: #666666;
            }
        """)
        self.convert_btn.clicked.connect(self.start_conversion)
        self.convert_btn.setEnabled(False)

        convert_layout.addWidget(convert_label)
        convert_layout.addWidget(self.convert_btn)

        # Add to toolbar
        toolbar_layout.addLayout(dff_layout)
        toolbar_layout.addLayout(txd_layout)
        toolbar_layout.addLayout(options_layout)
        toolbar_layout.addLayout(convert_layout)

    def create_content_area(self):
        """Create the main content area"""
        self.content_area = QWidget()
        content_layout = QVBoxLayout(self.content_area)

        # Progress bar
        self.progress_bar = QProgressBar()
        self.progress_bar.setVisible(False)
        self.progress_bar.setRange(0, 100)
        content_layout.addWidget(self.progress_bar)

        # Status label
        self.status_label = QLabel("Ready - Select a DFF file to begin conversion")
        self.status_label.setStyleSheet("color: #888888; padding: 8px;")
        content_layout.addWidget(self.status_label)

        # Info display area
        info_group = QGroupBox("File Information")
        info_layout = QVBoxLayout(info_group)

        self.info_text = QTextEdit()
        self.info_text.setReadOnly(True)
        self.info_text.setPlainText("No file loaded yet.")
        info_layout.addWidget(self.info_text)

        content_layout.addWidget(info_group)

    def apply_styling(self):
        """Apply modern dark theme styling"""
        self.setStyleSheet(f"""
            QWidget {{
                background-color: {ModernDarkTheme.BACKGROUND_TERTIARY};
                color: white;
            }}
            QGroupBox {{
                font-weight: bold;
                border: 1px solid {ModernDarkTheme.BORDER_PRIMARY};
                border-radius: 4px;
                margin-top: 8px;
                padding-top: 8px;
            }}
            QGroupBox::title {{
                subcontrol-origin: margin;
                left: 8px;
                padding: 0 4px 0 4px;
            }}
            QLineEdit {{
                background-color: {ModernDarkTheme.BACKGROUND_TERTIARY};
                border: 1px solid {ModernDarkTheme.BORDER_PRIMARY};
                border-radius: 4px;
                padding: 4px;
                color: white;
            }}
            QTextEdit {{
                background-color: {ModernDarkTheme.BACKGROUND_TERTIARY};
                border: 1px solid {ModernDarkTheme.BORDER_PRIMARY};
                border-radius: 4px;
                color: white;
            }}
        """)

    def browse_dff_file(self):
        """Browse for DFF file"""
        file_path, _ = QFileDialog.getOpenFileName(
            self, "Select DFF File", "", "DFF Files (*.dff);;All Files (*)"
        )
        if file_path:
            self.current_dff_file = file_path
            self.dff_path_edit.setText(os.path.basename(file_path))
            self.update_convert_button()
            self.load_dff_info()

    def browse_txd_file(self):
        """Browse for TXD file"""
        file_path, _ = QFileDialog.getOpenFileName(
            self, "Select TXD File", "", "TXD Files (*.txd);;All Files (*)"
        )
        if file_path:
            self.current_txd_file = file_path
            self.txd_path_edit.setText(os.path.basename(file_path))
            self.update_convert_button()

    def update_convert_button(self):
        """Update convert button enabled state"""
        has_dff = self.current_dff_file is not None
        has_txd = self.current_txd_file is not None
        materials_enabled = self.convert_materials_checkbox.isChecked()

        # Enable if we have DFF, and either materials disabled or TXD provided
        self.convert_btn.setEnabled(has_dff and (not materials_enabled or has_txd))

    def load_dff_info(self):
        """Load and display DFF file information"""
        if not self.current_dff_file or not DFF_TXD_SUPPORT:
            self.info_text.setPlainText("DFF support not available or no file selected.")
            return

        try:
            dff_data = dff()
            dff_data.load_file(self.current_dff_file)

            info_lines = []
            info_lines.append(f"File: {os.path.basename(self.current_dff_file)}")
            info_lines.append(f"RW Version: 0x{dff_data.rw_version:08x}")
            info_lines.append(f"Frames: {len(dff_data.frame_list)}")
            info_lines.append(f"Atomics: {len(dff_data.atomic_list)}")
            info_lines.append(f"Geometries: {len(dff_data.geometry_list)}")
            info_lines.append("")

            # Show frame information
            if dff_data.frame_list:
                info_lines.append("Frames:")
                for i, frame in enumerate(dff_data.frame_list[:10]):  # Show first 10
                    info_lines.append(f"  {i}: {frame.name}")
                if len(dff_data.frame_list) > 10:
                    info_lines.append(f"  ... and {len(dff_data.frame_list) - 10} more")

            self.info_text.setPlainText("\n".join(info_lines))
            self.status_label.setText(f"Loaded DFF file with {len(dff_data.atomic_list)} meshes")

        except Exception as e:
            error_msg = f"Failed to load DFF file: {str(e)}"
            self.info_text.setPlainText(error_msg)
            self.status_label.setText("Error loading DFF file")
            debug_logger.error(LogCategory.TOOL, error_msg, {"traceback": traceback.format_exc()})

    def start_conversion(self):
        """Start the OBJ conversion process"""
        if not self.current_dff_file:
            message_box("Error", "No DFF file selected", QMessageBox.Icon.Critical)
            return

        # Choose output file
        obj_file, _ = QFileDialog.getSaveFileName(
            self, "Save OBJ File", "", "OBJ Files (*.obj);;All Files (*)"
        )
        if not obj_file:
            return

        # Ensure .obj extension
        if not obj_file.lower().endswith('.obj'):
            obj_file += '.obj'

        # Check TXD requirement
        convert_materials = self.convert_materials_checkbox.isChecked()
        if convert_materials and not self.current_txd_file:
            message_box("Error", "TXD file required for material conversion", QMessageBox.Icon.Critical)
            return

        # Start conversion in background thread
        self.progress_bar.setVisible(True)
        self.progress_bar.setValue(0)
        self.status_label.setText("Starting conversion...")
        self.convert_btn.setEnabled(False)

        self.export_thread = QThread()
        self.export_worker = OBJExportWorker(
            self.current_dff_file,
            obj_file,
            convert_materials,
            self.current_txd_file
        )

        self.export_worker.moveToThread(self.export_thread)

        # Connect signals
        self.export_thread.started.connect(self.export_worker.run)
        self.export_worker.progress.connect(self.on_export_progress)
        self.export_worker.finished.connect(self.on_export_finished)
        self.export_worker.error.connect(self.on_export_error)
        self.export_worker.finished.connect(self.export_thread.quit)
        self.export_worker.error.connect(self.export_thread.quit)

        self.export_thread.start()

    def on_export_progress(self, percentage, message):
        """Handle export progress updates"""
        self.progress_bar.setValue(percentage)
        self.status_label.setText(message)

    def on_export_finished(self, success, message):
        """Handle successful export completion"""
        self.progress_bar.setVisible(False)
        self.convert_btn.setEnabled(True)
        self.status_label.setText(message)

        if success:
            message_box("Success", message, QMessageBox.Icon.Information)
        else:
            message_box("Error", message, QMessageBox.Icon.Critical)

        # Clean up
        if self.export_thread:
            self.export_thread.wait()
            self.export_thread = None
        self.export_worker = None

    def on_export_error(self, error_message):
        """Handle export errors"""
        self.progress_bar.setVisible(False)
        self.convert_btn.setEnabled(True)
        self.status_label.setText("Conversion failed")
        message_box("Export Error", error_message, QMessageBox.Icon.Critical)

        # Clean up
        if self.export_thread:
            self.export_thread.wait()
            self.export_thread = None
        self.export_worker = None