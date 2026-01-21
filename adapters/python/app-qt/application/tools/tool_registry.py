"""
Tool Registry for Rengine
Manages all available tools and their instantiation
"""

from application.tools.IMG_Editor import ImgEditorTool
from application.tools.DFF_Viewer.DFF_Viewer import DFFViewerTool
from application.tools.RW_Analyze.RW_Analyze import RWAnalyzeTool
from application.tools.IDE_Editor.IDE_Editor import IDEEditorTool
from application.tools.TXD_Editor import TXDEditorTool

# Import DragonFF-based tools
try:
    from application.tools.COL_Editor.COL_Editor import COLEditorTool
    from application.tools.Map_Editor.Map_Editor import MapEditorTool
    DRAGONFF_TOOLS_AVAILABLE = True
except ImportError:
    COLEditorTool = None
    MapEditorTool = None
    DRAGONFF_TOOLS_AVAILABLE = False

# Import DFF to OBJ converter
try:
    from application.tools.DFF_to_OBJ_Converter.DFF_to_OBJ_Converter import DFFtoOBJConverterTool
    DFF_TO_OBJ_AVAILABLE = True
except ImportError:
    DFFtoOBJConverterTool = None
    DFF_TO_OBJ_AVAILABLE = False





class ToolRegistry:
    """Registry for all available tools"""
    
    _tools = {
        'IMG_Editor': {
            'name': 'IMG_Editor',
            'class': ImgEditorTool,
            'description': 'Edit and manage IMG archive files',
            'icon': '📁'
        },
        'txd_editor': {
            'name': 'TXD Editor',
            'class': TXDEditorTool,
            'description': 'Edit and view TXD texture dictionary files',
            'icon': '🖼️'
        },
        'dff_viewer': {
            'name': 'DFF Viewer',
            'class': DFFViewerTool,
            'description': 'View and analyze 3D model files (DFF/OBJ)',
            'icon': '📦'
        },
        'rw_analyze': {
            'name': 'RW Analyze',
            'class': RWAnalyzeTool,
            'description': 'Analyze RenderWare chunks (DFF/TXD/COL) with tree and details',
            'icon': '🧩'
        },
        'ide_editor': {
            'name': 'IDE Editor',
            'class': IDEEditorTool,
            'description': 'Edit and validate IDE item definition files with table and raw views',
            'icon': '📋'
        },
        'col_editor': {
            'name': 'COL Editor',
            'class': COLEditorTool if DRAGONFF_TOOLS_AVAILABLE else None,
            'description': 'View and edit collision files (spheres, boxes, meshes)',
            'icon': '🔳'
        },
        'map_editor': {
            'name': 'Map Editor',
            'class': MapEditorTool if DRAGONFF_TOOLS_AVAILABLE else None,
            'description': 'View and edit IPL map files (object placement and cull zones)',
            'icon': '🗺️'
        },
        'dff_to_obj_converter': {
            'name': 'DFF to OBJ Converter',
            'class': DFFtoOBJConverterTool if DFF_TO_OBJ_AVAILABLE else None,
            'description': 'Convert DFF 3D models to OBJ format with optional TXD texture extraction',
            'icon': '🔄'
        },
        # Add other tools here as they are implemented
    }

    @classmethod
    def get_tool_info(cls, tool_name):
        """Get information about a tool"""
        return cls._tools.get(tool_name)
    
    @classmethod
    def get_all_tools(cls):
        """Get all registered tools"""
        return cls._tools.copy()
    
    @classmethod
    def create_tool(cls, tool_name, parent=None):
        """Create an instance of a tool"""
        tool_info = cls._tools.get(tool_name)
        if tool_info and tool_info['class']:
            return tool_info['class'](parent)
        return None
    
    @classmethod
    def is_tool_available(cls, tool_name):
        """Check if a tool is available and implemented"""
        tool_info = cls._tools.get(tool_name)
        return tool_info is not None and tool_info['class'] is not None
    
    @classmethod
    def register_tool(cls, tool_name, tool_class, name, description, icon):
        """Register a new tool"""
        cls._tools[tool_name] = {
            'name': name,
            'class': tool_class,
            'description': description,
            'icon': icon
        }
        