import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import {
  ArrowUpLeft,
  Box,
  Circle,
  FileText,
  FolderOpen,
  Move,
  Palette,
  RotateCcw,
  Save,
  Square,
  SquareStack,
} from "lucide-react";
import { useEffect, useRef } from "react";
import { ModeToggle, ModeToggleRef } from "~/components/mode-toggle";
import { Button } from "~/components/ui/button";
import { toast } from "~/lib/toast";
import { useEditorStore } from "~/store/editor-store";
import { EditorState, MapData, ToolbarItem } from "~/types/map";

const toolbarItems: ToolbarItem[] = [
  {
    id: "select",
    name: "Select",
    icon: ArrowUpLeft,
    tool: "select",
    shortcut: "Q",
  },
  {
    id: "move",
    name: "Move",
    icon: Move,
    tool: "move",
    shortcut: "W",
  },
  {
    id: "rotate",
    name: "Rotate",
    icon: RotateCcw,
    tool: "rotate",
    shortcut: "E",
  },
  {
    id: "scale",
    name: "Scale",
    icon: SquareStack,
    tool: "scale",
    shortcut: "R",
  },
  {
    id: "add-cube",
    name: "Add Cube",
    icon: Box,
    tool: "add",
    shortcut: "C",
  },
  {
    id: "add-sphere",
    name: "Add Sphere",
    icon: Circle,
    tool: "add",
    shortcut: "Shift+C",
  },
  {
    id: "add-plane",
    name: "Add Plane",
    icon: Square,
    tool: "add",
    shortcut: "P",
  },
  {
    id: "mode-toggle",
    name: "Mode Toggle",
    icon: Palette,
    tool: "mode-toggle",
    shortcut: "M",
  },
];

export default function Toolbar() {
  const {
    tool,
    setTool,
    isGridVisible,
    setGridVisible,
    snapToGrid,
    setSnapToGrid,
    mapData,
    setMapData,
    syncWithBackend,
    setPendingObjectType,
  } = useEditorStore();

  const modeToggleRef = useRef<ModeToggleRef>(null);

  const handleAddObject = (type: "cube" | "sphere" | "plane") => {
    console.log("Adding object type:", type);
    setPendingObjectType(type);
    setTool("add");
    toast.info(`Click on the canvas to place a ${type}`, `Selected ${type} tool`);
  };

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const key = event.key.toLowerCase();

      // Prevent default for our shortcuts
      if (["q", "w", "e", "r", "c", "p", "m", "f", "g", " "].includes(key)) {
        event.preventDefault();
      }

      // Tool shortcuts
      switch (key) {
        case "q":
          setTool("select");
          break;
        case "w":
          setTool("move");
          break;
        case "e":
          setTool("rotate");
          break;
        case "r":
          setTool("scale");
          break;
        case "c":
          if (event.shiftKey) {
            handleAddObject("sphere");
          } else {
            handleAddObject("cube");
          }
          break;
        case "p":
          handleAddObject("plane");
          break;
        case "m":
          modeToggleRef.current?.toggleTheme();
          break;
        case "f":
          // Focus on selected object (handled by viewport controls)
          break;
        case "g":
          // Toggle game mode (handled by viewport controls)
          break;
        case " ":
          // Cycle through tools
          {
            const tools: Array<EditorState["tool"]> = ["select", "move", "rotate", "scale"];
            const currentIndex = tools.indexOf(tool);
            const nextIndex = (currentIndex + 1) % tools.length;
            setTool(tools[nextIndex] ?? "select");
          }
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [tool, setTool, handleAddObject]);

  const handleToolClick = (item: ToolbarItem) => {
    if (item.tool === "mode-toggle") {
      // Mode toggle is handled by the ModeToggle component, not as a tool
      return;
    }
    setTool(item.tool);
  };

  const handleNewMap = async () => {
    try {
      const newMap = await invoke<MapData>("create_new_map");
      setMapData(newMap);
      toast.success("New map created successfully!");
    } catch (error) {
      console.error("Failed to create new map:", error);
      toast.error("Failed to create new map");
    }
  };

  const handleOpenMap = async () => {
    try {
      const filePath = await open({
        filters: [
          {
            name: "Map Files",
            extensions: ["json"],
          },
        ],
      });

      if (filePath) {
        const mapData = await invoke<MapData>("load_map", { path: filePath });
        setMapData(mapData);
        toast.success("Map opened successfully!");
      }
    } catch (error) {
      console.error("Failed to open map:", error);
      toast.error("Failed to open map");
    }
  };

  const handleSaveMap = async () => {
    try {
      if (!mapData) return;

      // Sync current map data to backend first
      await syncWithBackend();

      const filePath = await save({
        filters: [
          {
            name: "Map Files",
            extensions: ["json"],
          },
        ],
        defaultPath: `${mapData.name}.json`,
      });

      if (filePath) {
        await invoke("save_map", { path: filePath });
        toast.success("Map saved successfully!");
      }
    } catch (error) {
      console.error("Failed to save map:", error);
      toast.error("Failed to save map");
    }
  };

  return (
    <div className="bg-gray-800 border-b border-gray-700 p-4 flex items-center gap-4">
      {/* Main Tools */}
      <div className="flex gap-2">
        {toolbarItems.slice(0, 4).map((item) => {
          if (item.tool === "mode-toggle") {
            return (
              <div className="flex items-center" key={item.id}>
                <ModeToggle ref={modeToggleRef} />
              </div>
            );
          }
          const IconComponent = item.icon;
          return (
            <Button
              key={item.id}
              onClick={() => handleToolClick(item)}
              title={`${item.name} (${item.shortcut})`}
              variant={tool === item.tool ? "default" : "outline"}
            >
              <IconComponent className="w-4 h-4 mr-2" />
              {item.name}
            </Button>
          );
        })}
      </div>

      <div className="w-px h-8 bg-gray-600" />

      {/* Add Object Tools */}
      <div className="flex gap-2">
        <Button onClick={() => handleAddObject("cube")} title="Add Cube (C)" variant="link">
          <Box className="w-4 h-4 mr-2" />
          Add Cube
        </Button>
        <Button
          onClick={() => handleAddObject("sphere")}
          title="Add Sphere (Shift+C)"
          variant="link"
        >
          <Circle className="w-4 h-4 mr-2" />
          Add Sphere
        </Button>
        <Button onClick={() => handleAddObject("plane")} title="Add Plane (P)" variant="link">
          <Square className="w-4 h-4 mr-2" />
          Add Plane
        </Button>
      </div>

      <div className="w-px h-8 bg-gray-600" />

      {/* Grid Controls */}
      <div className="flex items-center gap-4">
        <label className="flex items-center gap-2 text-sm text-gray-300">
          <input
            checked={isGridVisible}
            className="rounded"
            onChange={(e) => setGridVisible(e.target.checked)}
            type="checkbox"
          />
          Show Grid
        </label>
        <label className="flex items-center gap-2 text-sm text-gray-300">
          <input
            checked={snapToGrid}
            className="rounded"
            onChange={(e) => setSnapToGrid(e.target.checked)}
            type="checkbox"
          />
          Snap to Grid
        </label>
      </div>

      <div className="flex-1" />

      {/* File Operations */}
      <div className="flex gap-2">
        <Button onClick={handleNewMap} variant="default">
          <FileText className="w-4 h-4 mr-2" />
          New
        </Button>
        <Button onClick={handleOpenMap} variant="default">
          <FolderOpen className="w-4 h-4 mr-2" />
          Open
        </Button>
        <Button onClick={handleSaveMap} variant="default">
          <Save className="w-4 h-4 mr-2" />
          Save
        </Button>
      </div>
    </div>
  );
}
