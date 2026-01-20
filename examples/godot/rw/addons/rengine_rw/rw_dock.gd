@tool
extends Control

var analyzer = null
var _mouse_over = false

func _ready():
    # Set focus mode to allow keyboard input when focused
    focus_mode = Control.FOCUS_CLICK
    print("RwDock _ready called")

    # Check if our nodes exist
    var vbox = $VBoxContainer
    if vbox:
        print("VBoxContainer found")
        # Set up tooltips and connect signals
        var analyze_btn = vbox.get_node_or_null("AnalyzeFileButton")
        if analyze_btn:
            analyze_btn.tooltip_text = "Analyze any file to detect RenderWare format and structure (Ctrl+A)"
            analyze_btn.connect("pressed", Callable(self, "on_analyze_file_pressed"))
            analyze_btn.disabled = false
            print("Analyze button connected and enabled")
        else:
            print("Analyze button not found")

        var img_btn = vbox.get_node_or_null("ParseImgButton")
        if img_btn:
            img_btn.tooltip_text = "Parse GTA IMG archive files (Ctrl+I)"
            img_btn.connect("pressed", Callable(self, "on_parse_img_pressed"))
            img_btn.disabled = false
            print("IMG button connected and enabled")

        var dff_btn = vbox.get_node_or_null("ParseDffButton")
        if dff_btn:
            dff_btn.tooltip_text = "Parse 3D model DFF files (Ctrl+D)"
            var connected = dff_btn.connect("pressed", Callable(self, "on_parse_dff_pressed"))
            if connected == OK:
                print("DFF button connected successfully")
            else:
                print("Failed to connect DFF button: ", connected)
            dff_btn.disabled = false
            print("DFF button enabled")
        else:
            print("DFF button not found")

        var clear_btn = vbox.get_node_or_null("ClearButton")
        if clear_btn:
            clear_btn.tooltip_text = "Clear the output display (Ctrl+C)"
            clear_btn.connect("pressed", Callable(self, "on_clear_pressed"))
            clear_btn.disabled = false
            print("Clear button connected and enabled")
    else:
        print("VBoxContainer not found")

    # Try to create our own analyzer instance
    print("Checking for RwAnalyzer class...")
    if ClassDB.class_exists("RwAnalyzer"):
        print("RwAnalyzer class found in ClassDB!")
        analyzer = ClassDB.instantiate("RwAnalyzer")
        if analyzer:
            print("RwAnalyzer instantiated successfully!")
            print("Analyzer object: ", analyzer)
            # Test if methods exist
            if analyzer.has_method("parse_dff_model"):
                print("parse_dff_model method found on analyzer")
            else:
                print("WARNING: parse_dff_model method not found on analyzer")
        else:
            printerr("Failed to instantiate RwAnalyzer")
            analyzer = null
    else:
        printerr("RwAnalyzer class not found in ClassDB. Make sure the GDExtension is loaded.")
        # List all available classes for debugging
        var classes = ClassDB.get_class_list()
        print("Available classes count: ", classes.size())
        print("First few classes: ", classes.slice(0, 10))

# Proper minimum size for layout
func _get_minimum_size():
    return Vector2(250, 300)

# Handle notifications for focus and mouse events
func _notification(what):
    match what:
        NOTIFICATION_MOUSE_ENTER:
            _mouse_over = true
            queue_redraw()
        NOTIFICATION_MOUSE_EXIT:
            _mouse_over = false
            queue_redraw()
        NOTIFICATION_FOCUS_ENTER:
            # Focus gained - update and redraw
            queue_redraw()
        NOTIFICATION_FOCUS_EXIT:
            # Focus lost - update and redraw
            queue_redraw()
        NOTIFICATION_THEME_CHANGED:
            # Theme changed - redraw with new theme
            queue_redraw()
        NOTIFICATION_VISIBILITY_CHANGED:
            # Visibility changed - could affect focus/drawing
            queue_redraw()
        NOTIFICATION_RESIZED:
            # Control changed size - ensure content fits
            var vbox = $VBoxContainer
            if vbox:
                vbox.size = size

# Custom drawing for focus indication
func _draw():
    # Check control size for proper bounds
    var rect = Rect2(Vector2.ZERO, size)

    if has_focus():
        # Draw a focus border using theme
        var style = get_theme_stylebox("focus", "Control")
        if style:
            draw_style_box(style, rect)
    elif _mouse_over:
        # Subtle hover effect using theme colors
        var hover_color = get_theme_color("accent_color", "Editor") * 0.1
        draw_rect(rect, hover_color)

# Handle GUI input - only works when mouse is over control, button pressed over it, or has focus
func _gui_input(event):
    if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
        # Accept focus on click - this allows the control to receive keyboard input
        grab_focus()
        accept_event()

func on_analyze_file_pressed():
    print("on_analyze_file_pressed called - using FileDialog")
    # Use regular FileDialog with proper configuration
    var dialog = FileDialog.new()
    if dialog:
        dialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE
        dialog.access = FileDialog.ACCESS_FILESYSTEM
        dialog.title = "Select File to Analyze"
        dialog.size = Vector2i(800, 600)
        dialog.connect("file_selected", Callable(self, "on_file_selected_for_analysis"))
        add_child(dialog)
        dialog.popup_centered_ratio(0.8)
        print("Analyze FileDialog should be open")
    else:
        print("Failed to create FileDialog")
        # Fallback: just show a message
        var output_text = $VBoxContainer/OutputText
        if output_text:
            output_text.text = "File dialog could not be opened."

func on_file_selected_for_analysis(path: String):
    if analyzer:
        var result = analyzer.analyze_file(path)
        var output_text = $VBoxContainer/OutputText

        if result and result.has("success"):
            if result["success"]:
                output_text.text = "File Analysis Results:\n" + _format_analysis_result(result)
            else:
                output_text.text = "Analysis Failed:\n" + str(result.get("error", "Unknown error"))
        else:
            output_text.text = "Invalid response from analyzer"

        # Scroll to top
        output_text.scroll_vertical = 0
    else:
        var output_text = $VBoxContainer/OutputText
        output_text.text = "Error: Analyzer not available"

func on_parse_img_pressed():
    print("on_parse_img_pressed called")
    var dialog = FileDialog.new()
    if dialog:
        dialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE
        dialog.access = FileDialog.ACCESS_FILESYSTEM
        dialog.filters = PackedStringArray(["*.img ; IMG Archive Files"])
        dialog.title = "Select IMG Archive File"
        dialog.size = Vector2i(800, 600)
        dialog.connect("file_selected", Callable(self, "on_img_file_selected"))
        add_child(dialog)
        dialog.popup_centered_ratio(0.8)
        print("IMG FileDialog should be open")
    else:
        print("Failed to create IMG FileDialog")
        var output_text = $VBoxContainer/OutputText
        if output_text:
            output_text.text = "File dialog could not be opened."

func on_img_file_selected(path: String):
    if analyzer:
        var result = analyzer.parse_img_archive(path)
        var output_text = $VBoxContainer/OutputText

        if result and result.has("success"):
            if result["success"]:
                output_text.text = "IMG Archive Analysis:\n" + _format_img_result(result)
            else:
                output_text.text = "IMG Parsing Failed:\n" + str(result.get("error", "Unknown error"))
        else:
            output_text.text = "Invalid response from analyzer"

        output_text.scroll_vertical = 0
    else:
        var output_text = $VBoxContainer/OutputText
        output_text.text = "Error: Analyzer not available"

func on_parse_dff_pressed():
    print("on_parse_dff_pressed called - trying FileDialog with better configuration")

    # Give immediate feedback that button was pressed
    var output_text = $VBoxContainer/OutputText
    if output_text:
        output_text.text = "Opening file dialog to select DFF file...\nPlease wait..."

    # Try regular FileDialog with proper configuration
    var dialog = FileDialog.new()
    if dialog:
        print("FileDialog created successfully")
        dialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE
        dialog.access = FileDialog.ACCESS_FILESYSTEM
        dialog.filters = PackedStringArray(["*.dff ; DFF Model Files"])
        dialog.title = "Select DFF Model File"
        # Set a reasonable size and position
        dialog.size = Vector2i(800, 600)
        var connected = dialog.connect("file_selected", Callable(self, "on_dff_file_selected"))
        if connected == OK:
            print("Signal connected successfully")
        else:
            print("Failed to connect signal: ", connected)
            if output_text:
                output_text.text = "Error: Could not connect file dialog signal."
            return
        add_child(dialog)
        # Try popup_centered_ratio instead of popup_centered
        dialog.popup_centered_ratio(0.8)
        print("DFF FileDialog popup called - dialog should be open now")
    else:
        print("Failed to create FileDialog")
        # Fallback: just show a message
        if output_text:
            output_text.text = "Error: Could not create file dialog."

func on_dff_file_selected(path: String):
    if analyzer:
        var result = analyzer.parse_dff_model(path)
        var output_text = $VBoxContainer/OutputText

        if result and result.has("success"):
            if result["success"]:
                output_text.text = "DFF Model Analysis:\n" + _format_dff_result(result)
            else:
                output_text.text = "DFF Parsing Failed:\n" + str(result.get("error", "Unknown error"))
        else:
            output_text.text = "Invalid response from analyzer"

        output_text.scroll_vertical = 0
    else:
        var output_text = $VBoxContainer/OutputText
        output_text.text = "Error: Analyzer not available"

func on_clear_pressed():
    var output_text = $VBoxContainer/OutputText
    output_text.text = "Select a file to analyze..."
    output_text.scroll_vertical = 0

# Keyboard shortcuts
func _input(event):
    if not has_focus():
        return

    if event is InputEventKey and event.pressed:
        match event.keycode:
            KEY_A:
                if event.ctrl_pressed:
                    on_analyze_file_pressed()
                    accept_event()
            KEY_I:
                if event.ctrl_pressed:
                    on_parse_img_pressed()
                    accept_event()
            KEY_D:
                if event.ctrl_pressed:
                    on_parse_dff_pressed()
                    accept_event()
            KEY_C:
                if event.ctrl_pressed:
                    on_clear_pressed()
                    accept_event()

# Format analysis results for better readability
func _format_analysis_result(result):
    var formatted = ""

    # Basic info
    formatted += "Format: " + str(result.get("format", "Unknown")) + "\n"
    formatted += "Format Description: " + str(result.get("format_description", "Unknown")) + "\n"
    formatted += "File Size: " + _format_file_size(result.get("size", 0)) + "\n"
    formatted += "RW Version: " + str(result.get("rw_version", "Unknown")) + "\n"
    formatted += "Total Chunks: " + str(result.get("total_chunks", 0)) + "\n"
    formatted += "Max Depth: " + str(result.get("max_depth", 0)) + "\n"

    var analysis_time = result.get("analysis_time_ms", 0)
    formatted += "Analysis Time: " + str(analysis_time) + "ms\n"

    var warnings = result.get("corruption_warnings_count", 0)
    if warnings > 0:
        formatted += "Corruption Warnings: " + str(warnings) + "\n"

    return formatted

func _format_img_result(result):
    var formatted = ""
    formatted += "Version: " + str(result.get("version", "Unknown")) + "\n"
    formatted += "Total Entries: " + str(result.get("entries_count", 0)) + "\n"
    formatted += "Total Size: " + _format_file_size(result.get("total_size", 0)) + "\n"
    return formatted

func _format_dff_result(result):
    var formatted = ""
    formatted += "RW Version: " + str(result.get("rw_version", "Unknown")) + "\n"
    formatted += "Geometry Count: " + str(result.get("geometry_count", 0)) + "\n"
    formatted += "Material Count: " + str(result.get("material_count", 0)) + "\n"

    var has_anim = result.get("has_animations", false)
    formatted += "Has Animations: " + ("Yes" if has_anim else "No") + "\n"

    formatted += "Atomic Count: " + str(result.get("atomic_count", 0)) + "\n"
    formatted += "Frame Count: " + str(result.get("frame_count", 0)) + "\n"
    return formatted

func _format_file_size(bytes):
    if bytes < 1024:
        return str(bytes) + " bytes"
    elif bytes < 1024 * 1024:
        return str(bytes / 1024.0).pad_decimals(1) + " KB"
    else:
        return str(bytes / (1024.0 * 1024.0)).pad_decimals(1) + " MB"

# Make sure we can receive input events
func _can_drop_data(at_position, data):
    return false

func _get_drag_data(at_position):
    return null