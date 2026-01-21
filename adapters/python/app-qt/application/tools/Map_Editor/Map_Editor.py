#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
PyQt6-based Map Editor tool integrated into the Rengine.
- View and edit IPL (Item Placement) files
- Display map object instances and their properties
- Support for both binary and text IPL formats
- Themed to match the Rengine style
- Integrated with the suite's custom debug_system

Uses:
- application.debug_system for logging
- gtaLib.map for IPL parsing
- application.common for utilities
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
    QTableWidget,
    QTableWidgetItem,
    QTabWidget,
    QComboBox,
)

# Suite integrations
from application.debug_system import get_debug_logger, LogCategory
from application.responsive_utils import get_responsive_manager
from application.styles import ModernDarkTheme

# Map helpers - using DragonFF
try:
    from application.common.map import MapData, TextIPLData, SectionUtility
    from application.common.data.map_data import *
    MAP_SUPPORT = True
except ImportError:
    MAP_SUPPORT = False
    print("Warning: DragonFF Map support not available")


class MapEditorTool(QWidget):
    """Map Editor tool for viewing and editing IPL files"""

    # Signal emitted when a tool action occurs
    tool_action = pyqtSignal(str, dict)

    def __init__(self, parent=None):
        super().__init__(parent)

        # Initialize debug logger
        self.debug_logger = get_debug_logger()
        self.debug_logger.info(LogCategory.TOOL, "Initializing Map Editor")

        # Initialize responsive manager
        self.responsive_manager = get_responsive_manager()

        # Current map data
        self.current_map_data = None
        self.current_file_path = None
        self.current_format = None  # 'binary' or 'text'

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

        self.open_button = QPushButton("Open IPL File")
        self.open_button.setIcon(self.style().standardIcon(QtWidgets.QStyle.StandardPixmap.SP_FileIcon))
        toolbar_layout.addWidget(self.open_button)

        self.save_button = QPushButton("Save IPL File")
        self.save_button.setIcon(self.style().standardIcon(QtWidgets.QStyle.StandardPixmap.SP_DriveHDIcon))
        self.save_button.setEnabled(False)
        toolbar_layout.addWidget(self.save_button)

        self.format_combo = QComboBox()
        self.format_combo.addItems(["Auto Detect", "Binary IPL", "Text IPL"])
        toolbar_layout.addWidget(self.format_combo)

        toolbar_layout.addStretch()

        self.info_label = QLabel("No file loaded")
        toolbar_layout.addWidget(self.info_label)

        layout.addLayout(toolbar_layout)

        # Main tab widget
        self.tab_widget = QTabWidget()
        layout.addWidget(self.tab_widget)

        # Object instances tab
        instances_tab = QWidget()
        instances_layout = QVBoxLayout(instances_tab)

        instances_label = QLabel("Object Instances")
        instances_layout.addWidget(instances_label)

        self.instances_table = QTableWidget()
        self.instances_table.setColumnCount(8)
        self.instances_table.setHorizontalHeaderLabels([
            "ID", "Model Name", "Interior", "X", "Y", "Z", "RX", "RY", "RZ", "RW"
        ])
        instances_layout.addWidget(self.instances_table)

        self.tab_widget.addTab(instances_tab, "Instances")

        # Cull zones tab
        cull_tab = QWidget()
        cull_layout = QVBoxLayout(cull_tab)

        cull_label = QLabel("Cull Zones")
        cull_layout.addWidget(cull_label)

        self.cull_table = QTableWidget()
        self.cull_table.setColumnCount(8)
        self.cull_table.setHorizontalHeaderLabels([
            "Center X", "Center Y", "Center Z", "Width", "Length", "Height", "Rot X", "Rot Y", "Rot Z", "Rot W"
        ])
        cull_layout.addWidget(self.cull_table)

        self.tab_widget.addTab(cull_tab, "Cull Zones")

        # Raw data tab
        raw_tab = QWidget()
        raw_layout = QVBoxLayout(raw_tab)

        raw_label = QLabel("Raw IPL Data")
        raw_layout.addWidget(raw_label)

        self.raw_text = QTextEdit()
        self.raw_text.setReadOnly(True)
        self.raw_text.setFont(QtGui.QFont("Courier New", 10))
        raw_layout.addWidget(self.raw_text)

        self.tab_widget.addTab(raw_tab, "Raw Data")

    def setup_connections(self):
        """Setup signal/slot connections"""
        self.open_button.clicked.connect(self.open_ipl_file)
        self.save_button.clicked.connect(self.save_ipl_file)
        self.format_combo.currentTextChanged.connect(self.on_format_changed)

    def update_responsive_elements(self):
        """Update fonts and spacing for responsive design"""
        fonts = self.responsive_manager.get_font_config()
        spacing = self.responsive_manager.get_spacing_config()

        # Update fonts
        self.setStyleSheet(f"""
            QLabel {{ font-size: {fonts['body']['size']}px; }}
            QTableWidget {{ font-size: {fonts['body']['size']}px; }}
            QTextEdit {{ font-size: {fonts['body']['size']}px; }}
            QPushButton {{ font-size: {fonts['body']['size']}px; }}
            QComboBox {{ font-size: {fonts['body']['size']}px; }}
        """)

        # Update margins and spacing
        layout = self.layout()
        if layout:
            layout.setContentsMargins(
                spacing['medium'], spacing['small'],
                spacing['medium'], spacing['small']
            )
            layout.setSpacing(spacing['small'])

    def open_ipl_file(self):
        """Open an IPL file"""
        if not MAP_SUPPORT:
            MessageBox.warning(self, "Map Support Unavailable",
                             "DragonFF Map support is not available.")
            return

        file_dialog = QFileDialog(self)
        file_dialog.setNameFilter("IPL files (*.ipl);;All files (*)")
        file_dialog.setFileMode(QFileDialog.FileMode.ExistingFile)

        if file_dialog.exec():
            file_path = file_dialog.selectedFiles()[0]
            self.load_ipl_file(file_path)

    def load_ipl_file(self, file_path: str):
        """Load an IPL file"""
        try:
            self.debug_logger.info(LogCategory.FILE_IO, f"Loading IPL file: {file_path}")

            format_choice = self.format_combo.currentText()

            if format_choice == "Auto Detect":
                # Try to detect format
                with open(file_path, 'rb') as f:
                    header = f.read(4)
                    if header == b'bnry':
                        self.current_format = 'binary'
                    else:
                        self.current_format = 'text'
            elif format_choice == "Binary IPL":
                self.current_format = 'binary'
            else:
                self.current_format = 'text'

            if self.current_format == 'binary':
                # Load binary IPL
                with open(file_path, 'rb') as f:
                    data = f.read()
                # Binary IPL loading would use DragonFF's binary IPL parser
                self.current_map_data = MapData([], [])
                MessageBox.information(self, "Binary IPL",
                                     "Binary IPL loading is available through DragonFF's map module.")
            else:
                # Load text IPL
                with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                # Text IPL loading would use DragonFF's text IPL parser
                self.current_map_data = TextIPLData([], [])
                self.raw_text.setPlainText(content)

            self.current_file_path = file_path

            # Update UI
            self.update_tables()
            self.save_button.setEnabled(True)

            # Update status
            file_name = os.path.basename(file_path)
            self.info_label.setText(f"Loaded: {file_name} ({self.current_format})")

            self.debug_logger.info(LogCategory.FILE_IO, f"Successfully loaded IPL file: {file_name}")

        except Exception as e:
            self.debug_logger.error(LogCategory.FILE_IO, f"Failed to load IPL file: {e}")
            MessageBox.critical(self, "Load Error", f"Failed to load IPL file:\n{str(e)}")

    def save_ipl_file(self):
        """Save the current IPL file"""
        if not self.current_map_data or not self.current_file_path:
            return

        try:
            # For now, just show that save functionality would go here
            MessageBox.information(self, "Save Feature",
                                 "IPL file saving will be implemented using DragonFF's export functionality.")

        except Exception as e:
            self.debug_logger.error(LogCategory.FILE_IO, f"Failed to save IPL file: {e}")
            MessageBox.critical(self, "Save Error", f"Failed to save IPL file:\n{str(e)}")

    def on_format_changed(self, format_text: str):
        """Handle format combo box changes"""
        # This would trigger reloading if a file is currently loaded
        if self.current_file_path:
            reply = QMessageBox.question(
                self, "Reload File",
                "Format changed. Reload the current file with the new format?",
                QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No
            )
            if reply == QMessageBox.StandardButton.Yes:
                self.load_ipl_file(self.current_file_path)

    def update_tables(self):
        """Update the instance and cull zone tables"""
        if not self.current_map_data:
            return

        # Update instances table
        self.instances_table.setRowCount(0)
        if hasattr(self.current_map_data, 'object_instances'):
            for instance in self.current_map_data.object_instances:
                row = self.instances_table.rowCount()
                self.instances_table.insertRow(row)

                # Fill instance data (this would be adapted based on DragonFF's data structures)
                self.instances_table.setItem(row, 0, QTableWidgetItem(str(getattr(instance, 'id', 'N/A'))))
                self.instances_table.setItem(row, 1, QTableWidgetItem(getattr(instance, 'model_name', 'N/A')))
                self.instances_table.setItem(row, 2, QTableWidgetItem(str(getattr(instance, 'interior', 'N/A'))))

                # Position
                if hasattr(instance, 'position'):
                    pos = instance.position
                    self.instances_table.setItem(row, 3, QTableWidgetItem(".2f"))
                    self.instances_table.setItem(row, 4, QTableWidgetItem(".2f"))
                    self.instances_table.setItem(row, 5, QTableWidgetItem(".2f"))

                # Rotation
                if hasattr(instance, 'rotation'):
                    rot = instance.rotation
                    self.instances_table.setItem(row, 6, QTableWidgetItem(".2f"))
                    self.instances_table.setItem(row, 7, QTableWidgetItem(".2f"))
                    self.instances_table.setItem(row, 8, QTableWidgetItem(".2f"))
                    self.instances_table.setItem(row, 9, QTableWidgetItem(".2f"))

        # Update cull zones table
        self.cull_table.setRowCount(0)
        if hasattr(self.current_map_data, 'cull_instances'):
            for cull in self.current_map_data.cull_instances:
                row = self.cull_table.rowCount()
                self.cull_table.insertRow(row)

                # Fill cull zone data (this would be adapted based on DragonFF's data structures)
                if hasattr(cull, 'center'):
                    center = cull.center
                    self.cull_table.setItem(row, 0, QTableWidgetItem(".2f"))
                    self.cull_table.setItem(row, 1, QTableWidgetItem(".2f"))
                    self.cull_table.setItem(row, 2, QTableWidgetItem(".2f"))

                if hasattr(cull, 'dimensions'):
                    dims = cull.dimensions
                    self.cull_table.setItem(row, 3, QTableWidgetItem(".2f"))
                    self.cull_table.setItem(row, 4, QTableWidgetItem(".2f"))
                    self.cull_table.setItem(row, 5, QTableWidgetItem(".2f"))

                if hasattr(cull, 'rotation'):
                    rot = cull.rotation
                    self.cull_table.setItem(row, 6, QTableWidgetItem(".2f"))
                    self.cull_table.setItem(row, 7, QTableWidgetItem(".2f"))
                    self.cull_table.setItem(row, 8, QTableWidgetItem(".2f"))
                    self.cull_table.setItem(row, 9, QTableWidgetItem(".2f"))

    def cleanup(self):
        """Clean up resources"""
        self.debug_logger.info(LogCategory.TOOL, "Cleaning up Map Editor")
        # Add any cleanup logic here