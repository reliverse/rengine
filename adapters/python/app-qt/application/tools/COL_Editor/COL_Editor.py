#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
PyQt6-based COL Editor tool integrated into the Rengine.
- View and edit collision files (COL1, COL2, COL3, COL4)
- Display collision model information and statistics
- Support for spheres, boxes, meshes, and face groups
- Themed to match the Rengine style
- Integrated with the suite's custom debug_system

Uses:
- application.debug_system for logging
- gtaLib.col for collision file parsing
- application.common.rw_versions for version detection
"""
from __future__ import annotations

import os
from typing import List, Optional, Dict, Any
from application.common.message_box import MessageBox


from PyQt6 import QtCore, QtGui, QtWidgets
from PyQt6.QtCore import Qt, pyqtSignal
from PyQt6.QtWidgets import (
    QWidget,
    QVBoxLayout,
    QHBoxLayout,
    QPushButton,
    QFileDialog,
    QSplitter,
    QTreeWidget,
    QTreeWidgetItem,
    QTextEdit,
    QLabel,
    QMenu,
    QMessageBox,
    QSizePolicy,
    QGroupBox,
    QFormLayout,
    QLineEdit,
    QSpinBox,
    QDoubleSpinBox,
)

# Suite integrations
from application.debug_system import get_debug_logger, LogCategory
from application.responsive_utils import get_responsive_manager
from application.styles import ModernDarkTheme

# COL helpers - using DragonFF
try:
    from application.common.col import ColModel, TSphere, TBox, TFace, TFaceGroup, TVector
    from application.common.rw_versions import RWVersionManager
    COL_SUPPORT = True
except ImportError:
    COL_SUPPORT = False
    print("Warning: DragonFF COL support not available")


class COLEditorTool(QWidget):
    """COL Editor tool for viewing and editing collision files"""

    # Signal emitted when a tool action occurs
    tool_action = pyqtSignal(str, dict)

    def __init__(self, parent=None):
        super().__init__(parent)

        # Initialize debug logger
        self.debug_logger = get_debug_logger()
        self.debug_logger.info(LogCategory.TOOL, "Initializing COL Editor")

        # Initialize responsive manager
        self.responsive_manager = get_responsive_manager()

        # Current COL data
        self.current_col_model = None
        self.current_file_path = None

        # Setup UI
        self.setup_ui()
        self.setup_connections()

        # Update fonts and spacing
        self.update_responsive_elements()

    def setup_ui(self):
        """Setup the user interface"""
        layout = QVBoxLayout(self)
        layout.setContentsMargins(10, 10, 10, 10)

        # Top toolbar
        toolbar_layout = QHBoxLayout()

        self.open_button = QPushButton("Open COL File")
        self.open_button.setIcon(self.style().standardIcon(QtWidgets.QStyle.StandardPixmap.SP_FileIcon))
        toolbar_layout.addWidget(self.open_button)

        self.save_button = QPushButton("Save COL File")
        self.save_button.setIcon(self.style().standardIcon(QtWidgets.QStyle.StandardPixmap.SP_DriveHDIcon))
        self.save_button.setEnabled(False)
        toolbar_layout.addWidget(self.save_button)

        toolbar_layout.addStretch()

        self.info_label = QLabel("No file loaded")
        toolbar_layout.addWidget(self.info_label)

        layout.addLayout(toolbar_layout)

        # Main splitter
        splitter = QSplitter(Qt.Orientation.Horizontal)
        layout.addWidget(splitter)

        # Left panel - Tree view of COL structure
        left_widget = QWidget()
        left_layout = QVBoxLayout(left_widget)

        self.tree_label = QLabel("COL Structure")
        left_layout.addWidget(self.tree_label)

        self.col_tree = QTreeWidget()
        self.col_tree.setHeaderLabel("Collision Elements")
        self.col_tree.setContextMenuPolicy(Qt.ContextMenuPolicy.CustomContextMenu)
        left_layout.addWidget(self.col_tree)

        splitter.addWidget(left_widget)

        # Right panel - Details and properties
        right_widget = QWidget()
        right_layout = QVBoxLayout(right_widget)

        # File info group
        info_group = QGroupBox("File Information")
        info_layout = QFormLayout(info_group)

        self.version_label = QLabel("N/A")
        info_layout.addRow("Version:", self.version_label)

        self.model_name_label = QLabel("N/A")
        info_layout.addRow("Model Name:", self.model_name_label)

        self.model_id_label = QLabel("N/A")
        info_layout.addRow("Model ID:", self.model_id_label)

        self.bounds_label = QLabel("N/A")
        info_layout.addRow("Bounds:", self.bounds_label)

        right_layout.addWidget(info_group)

        # Statistics group
        stats_group = QGroupBox("Statistics")
        stats_layout = QFormLayout(stats_group)

        self.spheres_count_label = QLabel("0")
        stats_layout.addRow("Spheres:", self.spheres_count_label)

        self.boxes_count_label = QLabel("0")
        stats_layout.addRow("Boxes:", self.boxes_count_label)

        self.vertices_count_label = QLabel("0")
        stats_layout.addRow("Vertices:", self.vertices_count_label)

        self.faces_count_label = QLabel("0")
        stats_layout.addRow("Faces:", self.faces_count_label)

        self.face_groups_count_label = QLabel("0")
        stats_layout.addRow("Face Groups:", self.face_groups_count_label)

        right_layout.addWidget(stats_group)

        # Details text area
        details_label = QLabel("Element Details")
        right_layout.addWidget(details_label)

        self.details_text = QTextEdit()
        self.details_text.setReadOnly(True)
        self.details_text.setMaximumHeight(200)
        right_layout.addWidget(self.details_text)

        splitter.addWidget(right_widget)

        # Set splitter proportions
        splitter.setSizes([400, 400])

    def setup_connections(self):
        """Setup signal/slot connections"""
        self.open_button.clicked.connect(self.open_col_file)
        self.save_button.clicked.connect(self.save_col_file)
        self.col_tree.itemSelectionChanged.connect(self.on_tree_selection_changed)
        self.col_tree.customContextMenuRequested.connect(self.show_context_menu)

    def update_responsive_elements(self):
        """Update fonts and spacing for responsive design"""
        fonts = self.responsive_manager.get_font_config()
        spacing = self.responsive_manager.get_spacing_config()

        # Update fonts
        self.setStyleSheet(f"""
            QLabel {{ font-size: {fonts['body']['size']}px; }}
            QTreeWidget {{ font-size: {fonts['body']['size']}px; }}
            QTextEdit {{ font-size: {fonts['body']['size']}px; }}
            QPushButton {{ font-size: {fonts['body']['size']}px; }}
        """)

        # Update margins and spacing
        layout = self.layout()
        if layout:
            layout.setContentsMargins(
                spacing['medium'], spacing['small'],
                spacing['medium'], spacing['small']
            )
            layout.setSpacing(spacing['small'])

    def open_col_file(self):
        """Open a COL file"""
        if not COL_SUPPORT:
            MessageBox.warning(self, "COL Support Unavailable",
                             "DragonFF COL support is not available.")
            return

        file_dialog = QFileDialog(self)
        file_dialog.setNameFilter("COL files (*.col);;All files (*)")
        file_dialog.setFileMode(QFileDialog.FileMode.ExistingFile)

        if file_dialog.exec():
            file_path = file_dialog.selectedFiles()[0]
            self.load_col_file(file_path)

    def load_col_file(self, file_path: str):
        """Load a COL file"""
        try:
            self.debug_logger.info(LogCategory.FILE_IO, f"Loading COL file: {file_path}")

            with open(file_path, 'rb') as f:
                data = f.read()

            # Create COL model
            self.current_col_model = ColModel()
            self.current_col_model.load(data)
            self.current_file_path = file_path

            # Update UI
            self.update_file_info()
            self.update_col_tree()
            self.update_statistics()

            # Enable save button
            self.save_button.setEnabled(True)

            # Update status
            file_name = os.path.basename(file_path)
            self.info_label.setText(f"Loaded: {file_name}")

            self.debug_logger.info(LogCategory.FILE_IO, f"Successfully loaded COL file: {file_name}")

        except Exception as e:
            self.debug_logger.error(LogCategory.FILE_IO, f"Failed to load COL file: {e}")
            MessageBox.critical(self, "Load Error", f"Failed to load COL file:\n{str(e)}")

    def save_col_file(self):
        """Save the current COL file"""
        if not self.current_col_model or not self.current_file_path:
            return

        try:
            # For now, just show that save functionality would go here
            MessageBox.information(self, "Save Feature",
                                 "COL file saving will be implemented using DragonFF's export functionality.")

        except Exception as e:
            self.debug_logger.error(LogCategory.FILE_IO, f"Failed to save COL file: {e}")
            MessageBox.critical(self, "Save Error", f"Failed to save COL file:\n{str(e)}")

    def update_file_info(self):
        """Update file information display"""
        if not self.current_col_model:
            return

        self.version_label.setText(str(self.current_col_model.version))
        self.model_name_label.setText(self.current_col_model.model_name or "N/A")
        self.model_id_label.setText(str(self.current_col_model.model_id))

        if self.current_col_model.bounds:
            bounds = self.current_col_model.bounds
            bounds_text = ".2f"
            self.bounds_label.setText(bounds_text)
        else:
            self.bounds_label.setText("N/A")

    def update_statistics(self):
        """Update statistics display"""
        if not self.current_col_model:
            return

        self.spheres_count_label.setText(str(len(self.current_col_model.spheres)))
        self.boxes_count_label.setText(str(len(self.current_col_model.boxes)))
        self.vertices_count_label.setText(str(len(self.current_col_model.mesh_verts)))
        self.faces_count_label.setText(str(len(self.current_col_model.mesh_faces)))
        self.face_groups_count_label.setText(str(len(self.current_col_model.face_groups)))

    def update_col_tree(self):
        """Update the COL structure tree"""
        self.col_tree.clear()

        if not self.current_col_model:
            return

        # Root item
        root = QTreeWidgetItem(["Collision Model"])
        self.col_tree.addTopLevelItem(root)

        # Spheres
        if self.current_col_model.spheres:
            spheres_item = QTreeWidgetItem(["Spheres"])
            root.addChild(spheres_item)
            for i, sphere in enumerate(self.current_col_model.spheres):
                sphere_item = QTreeWidgetItem([f"Sphere {i}"])
                sphere_item.setData(0, Qt.ItemDataRole.UserRole, ("sphere", i, sphere))
                spheres_item.addChild(sphere_item)

        # Boxes
        if self.current_col_model.boxes:
            boxes_item = QTreeWidgetItem(["Boxes"])
            root.addChild(boxes_item)
            for i, box in enumerate(self.current_col_model.boxes):
                box_item = QTreeWidgetItem([f"Box {i}"])
                box_item.setData(0, Qt.ItemDataRole.UserRole, ("box", i, box))
                boxes_item.addChild(box_item)

        # Mesh
        if self.current_col_model.mesh_verts or self.current_col_model.mesh_faces:
            mesh_item = QTreeWidgetItem(["Mesh"])
            root.addChild(mesh_item)

            if self.current_col_model.mesh_verts:
                verts_item = QTreeWidgetItem([f"Vertices ({len(self.current_col_model.mesh_verts)})"])
                mesh_item.addChild(verts_item)

            if self.current_col_model.mesh_faces:
                faces_item = QTreeWidgetItem([f"Faces ({len(self.current_col_model.mesh_faces)})"])
                mesh_item.addChild(faces_item)

        # Face Groups
        if self.current_col_model.face_groups:
            groups_item = QTreeWidgetItem(["Face Groups"])
            root.addChild(groups_item)
            for i, group in enumerate(self.current_col_model.face_groups):
                group_item = QTreeWidgetItem([f"Group {i}"])
                group_item.setData(0, Qt.ItemDataRole.UserRole, ("face_group", i, group))
                groups_item.addChild(group_item)

        root.setExpanded(True)

    def on_tree_selection_changed(self):
        """Handle tree selection changes"""
        selected_items = self.col_tree.selectedItems()
        if not selected_items:
            self.details_text.clear()
            return

        item = selected_items[0]
        data = item.data(0, Qt.ItemDataRole.UserRole)

        if data and len(data) == 3:
            element_type, index, element = data
            self.show_element_details(element_type, index, element)
        else:
            self.details_text.setPlainText("Select a collision element to view details.")

    def show_element_details(self, element_type: str, index: int, element):
        """Show details for a collision element"""
        details = f"Type: {element_type.title()}\n"
        details += f"Index: {index}\n\n"

        if element_type == "sphere":
            details += f"Center: {element.x:.2f}, {element.y:.2f}, {element.z:.2f}\n"
            details += f"Radius: {element.radius:.2f}"
        elif element_type == "box":
            details += "Min Corner:\n"
            details += f"  X: {element.min_x:.2f}, Y: {element.min_y:.2f}, Z: {element.min_z:.2f}\n"
            details += "Max Corner:\n"
            details += f"  X: {element.max_x:.2f}, Y: {element.max_y:.2f}, Z: {element.max_z:.2f}"
        elif element_type == "face_group":
            details += f"Start Face: {element.start_face}\n"
            details += f"Face Count: {element.face_count}\n"
            details += f"Material: {element.material}\n"
            if hasattr(element, 'flags'):
                details += f"Flags: {element.flags}"

        self.details_text.setPlainText(details)

    def show_context_menu(self, position):
        """Show context menu for tree items"""
        item = self.col_tree.itemAt(position)
        if not item:
            return

        menu = QMenu(self)

        # Add menu actions based on item type
        data = item.data(0, Qt.ItemDataRole.UserRole)
        if data and len(data) == 3:
            element_type, index, element = data
            action = menu.addAction(f"Edit {element_type.title()}")
            action.triggered.connect(lambda: self.edit_element(element_type, index, element))

        if menu.actions():
            menu.exec(self.col_tree.mapToGlobal(position))

    def edit_element(self, element_type: str, index: int, element):
        """Edit a collision element"""
        # For now, just show that editing functionality would go here
        MessageBox.information(self, "Edit Feature",
                             f"Editing {element_type} elements will be implemented using DragonFF's COL modification capabilities.")

    def cleanup(self):
        """Clean up resources"""
        self.debug_logger.info(LogCategory.TOOL, "Cleaning up COL Editor")
        # Add any cleanup logic here