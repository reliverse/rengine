@tool
extends EditorPlugin

var dock = null

func _enter_tree():
    print("Rengine RW Plugin _enter_tree")

    # Create the dock UI
    var dock_scene = load("res://addons/rengine_rw/rw_dock.tscn")
    if dock_scene:
        print("Dock scene loaded successfully")
        dock = dock_scene.instantiate()
        if dock:
            print("Dock instantiated successfully")
            # Try to add to dock
            add_control_to_dock(DOCK_SLOT_RIGHT_UL, dock)
            print("Rengine RW Plugin dock added")
        else:
            print("Failed to instantiate dock")
    else:
        print("Failed to load dock scene")

func _exit_tree():
    print("Rengine RW Plugin _exit_tree")
    if dock:
        remove_control_from_docks(dock)
        dock.free()