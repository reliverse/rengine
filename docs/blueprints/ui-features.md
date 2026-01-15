# UI Features and Controls

## Overview

The Blueprint editor provides a comprehensive set of UI features for creating and managing visual scripts efficiently.

## Canvas Controls

### Pan and Zoom

#### Mouse Wheel Zoom
- **Scroll Up**: Zoom in
- **Scroll Down**: Zoom out
- **Zoom Range**: 0.1x to 3x
- **Zoom Center**: Zooms toward mouse cursor position

#### Pan
- **Middle Mouse Button**: Click and drag to pan
- **Ctrl/Cmd + Left Click**: Alternative pan mode
- **Pan Speed**: Smooth, responsive movement

#### Zoom Controls UI
- **Zoom In Button**: Incremental zoom in
- **Zoom Out Button**: Incremental zoom out
- **Reset View Button**: Return to default view (1x zoom, centered)
- **Zoom Percentage**: Shows current zoom level

### Grid

- **Visual Grid**: Helps align nodes
- **Grid Size**: 20px (scales with zoom)
- **Snap to Grid**: Optional snapping for precise placement

## Node Interaction

### Selecting Nodes

#### Single Select
- **Click Node**: Selects the node
- **Click Empty Space**: Deselects all

#### Multi-Select
- **Shift + Click**: Toggle node selection
- **Drag Selection Box**: Select multiple nodes
- **Visual Feedback**: Selected nodes highlighted in blue

### Moving Nodes

- **Click and Drag**: Move node on canvas
- **Precise Movement**: Grid snapping available
- **Multi-Node Move**: Move multiple selected nodes together

### Connecting Nodes

#### Creating Connections
1. Click on an output pin
2. Drag to an input pin
3. Release to create connection

#### Connection Rules
- **Type Matching**: Pins must have compatible types
- **Exec Pins**: Can only connect to exec pins
- **Data Pins**: Must match data types (or use `any`)
- **One-to-One**: Input pins can only have one connection

#### Visual Feedback
- **Valid Connection**: Blue line preview
- **Invalid Connection**: Red line or no connection
- **Connected Pins**: Highlighted when connected

## Context Menus

### Right-Click Menu

Right-click on nodes or empty space for context menu:

#### Node Context Menu
- **Duplicate**: Create a copy of the node
- **Delete**: Remove the node and its connections
- **Properties**: Open property panel (if available)

#### Multi-Select Context Menu
- **Delete Selected**: Remove all selected nodes
- **Duplicate Selected**: Copy all selected nodes

### Keyboard Shortcuts

- **Escape**: Close context menu
- **Delete**: Delete selected nodes
- **Ctrl/Cmd + D**: Duplicate selected nodes
- **Ctrl/Cmd + C**: Copy nodes (future)
- **Ctrl/Cmd + V**: Paste nodes (future)

## Node Palette

### Browsing Nodes

- **Categories**: Organized by function type
- **Search**: Filter nodes by name or description
- **Icons**: Visual identification of node types
- **Descriptions**: Tooltips explain node purpose

### Adding Nodes

1. **Drag from Palette**: Drag node to canvas
2. **Click to Place**: Click in palette, then click on canvas
3. **Auto-Position**: New nodes placed near cursor

### Node Categories

- **Flow Control**: If, While, For loops
- **Variables**: Get/Set variables, Literals
- **Operators**: Math, Comparison, Logical
- **Functions**: User-defined functions
- **Events**: SAMP callbacks
- **Natives**: SAMP native functions
- **Utilities**: Comments, Groups

## Property Panel

### Node Properties

Edit node properties in the right-side panel:

- **Node Title**: Display name
- **Node Type**: Read-only type information
- **Position**: X, Y coordinates
- **Custom Properties**: Node-specific settings
- **Pins Info**: View input/output pins

### Property Editing

- **Text Inputs**: For names and values
- **Number Inputs**: For numeric properties
- **Type Selectors**: For type properties
- **Live Updates**: Changes apply immediately

## Minimap

### Overview

The minimap shows a bird's-eye view of your entire Blueprint:

- **Location**: Top-left corner
- **Size**: 200x200 pixels
- **Auto-Scale**: Fits entire graph
- **Node Visualization**: Nodes shown as colored rectangles
- **Viewport Indicator**: Yellow rectangle shows current view

### Using the Minimap

- **Click to Jump**: Click anywhere to center viewport there
- **Viewport Indicator**: See what part of graph you're viewing
- **Selection Highlight**: Selected nodes highlighted
- **Connection Overview**: See overall graph structure

## File Management

### Creating Blueprints

- **New Button**: Create empty Blueprint
- **Template Selection**: Choose language (Pawn, TypeScript, etc.)
- **Auto-Naming**: Default names provided

### Opening Files

- **Open Button**: Browse for `.blueprint` or `.pwn` files
- **Recent Files**: Quick access to recent files
- **Import**: Convert code files to Blueprint

### Saving

- **Save Button**: Save current Blueprint
- **Save As**: Save with new name
- **Auto-Save**: Optional automatic saving
- **Export**: Export to code format (`.pwn`)

## Search and Navigation

### Node Search

- **Search Bar**: In node palette
- **Filter by Type**: Category dropdown
- **Fuzzy Search**: Finds nodes by partial name
- **Quick Access**: Keyboard shortcut (future)

### Graph Navigation

- **Minimap**: Quick overview and navigation
- **Zoom to Fit**: Fit all nodes in view
- **Zoom to Selection**: Focus on selected nodes
- **Find Node**: Search for specific nodes (future)

## Visual Feedback

### Node States

- **Normal**: Default appearance
- **Selected**: Blue highlight border
- **Hovered**: Subtle highlight
- **Error**: Red border for invalid nodes
- **Warning**: Yellow border for warnings

### Connection States

- **Valid**: Blue line
- **Invalid**: Red line or dashed
- **Hovered**: Thicker line
- **Selected**: Highlighted

### Status Indicators

- **Modified**: Indicator for unsaved changes
- **Sync Status**: Shows sync state
- **Error Count**: Number of errors
- **Warning Count**: Number of warnings

## Performance

### Large Graphs

- **Virtual Scrolling**: Only render visible nodes
- **Lazy Loading**: Load nodes as needed
- **Optimization**: Efficient rendering for 1000+ nodes

### Responsiveness

- **Smooth Interactions**: 60 FPS target
- **Debounced Updates**: Prevent excessive re-renders
- **Efficient Updates**: Only update changed nodes

## Accessibility

### Keyboard Navigation

- **Tab**: Navigate between nodes
- **Arrow Keys**: Move selected node
- **Enter**: Activate node
- **Escape**: Cancel/deselect

### Screen Readers

- **ARIA Labels**: Proper labeling for assistive tech
- **Role Attributes**: Semantic HTML roles
- **Keyboard Support**: Full keyboard navigation

## Tips and Tricks

1. **Use Minimap**: Quickly navigate large graphs
2. **Multi-Select**: Work with multiple nodes at once
3. **Context Menus**: Right-click for quick actions
4. **Zoom Levels**: Use different zoom for detail vs overview
5. **Grid Snapping**: Enable for precise alignment
6. **Search**: Quickly find nodes in palette
7. **Keyboard Shortcuts**: Learn shortcuts for speed
