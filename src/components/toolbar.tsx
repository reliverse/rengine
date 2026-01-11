import { useNavigate } from "@tanstack/react-router";
import { documentDir, homeDir, join, tempDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { mkdir, readFile } from "@tauri-apps/plugin-fs";
import {
  Box,
  Circle,
  Download,
  File,
  FolderOpen,
  Lightbulb,
  MousePointer,
  Move,
  Plus,
  RotateCw,
  Save,
  Scaling,
  Settings as SettingsIcon,
  Square,
  Sun,
  Upload,
} from "lucide-react";
import { useState } from "react";
import { ExportDialog } from "~/components/export-dialog";
import { SettingsDialog } from "~/components/settings-panel";
import { Button } from "~/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "~/components/ui/dropdown-menu";
import { Progress } from "~/components/ui/progress";
import { Separator } from "~/components/ui/separator";
import { useToast } from "~/hooks/use-toast";
import { cn } from "~/lib/utils";
import { type TransformTool, useSceneStore } from "~/stores/scene-store";
import { getPresetList } from "~/utils/lighting-presets";
import { type ImportProgress, modelImporter } from "~/utils/model-import";
import { loadScene, saveScene } from "~/utils/scene-persistence";
import type { SidebarContext } from "./unified-sidebar";

const toolIcons = {
  select: MousePointer,
  move: Move,
  rotate: RotateCw,
  scale: Scaling,
};

function getMimeType(fileName: string): string {
  const extension = fileName.split(".").pop()?.toLowerCase();
  switch (extension) {
    case "gltf":
    case "glb":
      return "model/gltf-binary";
    case "obj":
      return "text/plain";
    case "fbx":
      return "application/octet-stream";
    default:
      return "application/octet-stream";
  }
}

interface ToolbarProps {
  rightSidebarContext: SidebarContext;
  setRightSidebarContext: (context: SidebarContext) => void;
}

export function Toolbar({
  rightSidebarContext,
  setRightSidebarContext,
}: ToolbarProps) {
  const [importProgress, setImportProgress] = useState<ImportProgress | null>(
    null
  );
  const [isImporting, setIsImporting] = useState(false);
  const [exportDialogOpen, setExportDialogOpen] = useState(false);
  const [settingsDialogOpen, setSettingsDialogOpen] = useState(false);
  const { toast } = useToast();
  const navigate = useNavigate();

  // Get auto-save directory path and ensure it exists
  const getAutoSaveDir = async (): Promise<string> => {
    const home = await homeDir();
    const autoSaveDir = await join(home, ".reliverse", "rengine", "projects");

    // Try primary directory first
    try {
      console.log("Creating primary auto-save directory:", autoSaveDir);
      await mkdir(autoSaveDir, { recursive: true });
      console.log(
        "✅ Primary auto-save directory created/verified:",
        autoSaveDir
      );
      return autoSaveDir;
    } catch (error) {
      console.warn("❌ Failed to create primary auto-save directory:", error);
    }

    // Fallback to documents directory
    try {
      const documents = await documentDir();
      const fallbackDir = await join(documents, "Rengine", "Projects");

      console.log("Trying fallback auto-save directory:", fallbackDir);
      await mkdir(fallbackDir, { recursive: true });
      console.log(
        "✅ Fallback auto-save directory created/verified:",
        fallbackDir
      );
      return fallbackDir;
    } catch (error) {
      console.warn("❌ Failed to create fallback auto-save directory:", error);
    }

    // Last resort: temp directory
    try {
      const temp = await tempDir();
      const tempDirPath = await join(temp, "rengine-autosave");

      console.log("Using temp directory as last resort:", tempDirPath);
      await mkdir(tempDirPath, { recursive: true });
      console.log("✅ Temp auto-save directory created:", tempDirPath);
      return tempDirPath;
    } catch (error) {
      console.error(
        "❌ All auto-save directory creation attempts failed:",
        error
      );
      throw new Error(
        "Could not create any auto-save directory. Please check file system permissions."
      );
    }
  };

  // Generate auto-save filename
  const generateAutoSaveFilename = (): string => {
    const now = new Date();
    const timestamp = now.toISOString().slice(0, 19).replace(/:/g, "-");
    return `auto_${timestamp}.rengine`;
  };

  const {
    activeTool,
    setActiveTool,
    startPlacement,
    placementMode,
    clearSelection,
    selectAll,
    addLight,
    lightsVisible,
    setLightsVisible,
    applyLightingPreset,
  } = useSceneStore();
  const selectionCount = useSceneStore(
    (state) => state.selectedObjectIds.length
  );
  const sceneMetadata = useSceneStore((state) => state.sceneMetadata);

  const handleToolSelect = (tool: TransformTool) => {
    setActiveTool(tool);
  };

  const handleAddObject = (type: "cube" | "sphere" | "plane") => {
    startPlacement(type);
  };

  const handleImport = async () => {
    if (isImporting) {
      return;
    }

    try {
      setIsImporting(true);

      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "3D Models",
            extensions: ["gltf", "glb", "obj", "fbx"],
          },
        ],
      });

      if (selected && typeof selected === "string") {
        try {
          const fileData = await readFile(selected);
          const fileName = selected.split("/").pop() || "model";
          const mimeType = getMimeType(fileName);

          const blob = new Blob([fileData], { type: mimeType });
          const file = Object.assign(blob, {
            name: fileName,
            lastModified: Date.now(),
          }) as File;

          setImportProgress({
            loaded: 0,
            total: 100,
            stage: "Starting import...",
          });

          const result = await modelImporter.importFromFile(file);

          setImportProgress(null);

          if (result.success && result.object) {
            useSceneStore.getState().addObject(result.object);

            // Delay toast to prevent potential state conflicts
            setTimeout(() => {
              toast({
                title: "Model imported successfully",
                description: `${result.object?.name ?? "Model"} has been added to the scene.`,
                duration: 3000,
              });
            }, 100);

            if (result.warnings && result.warnings.length > 0) {
              setTimeout(() => {
                toast({
                  title: "Import warnings",
                  description: result.warnings?.join("\n"),
                  variant: "default",
                  duration: 5000,
                });
              }, 1000);
            }
          } else {
            // Delay error toast
            setTimeout(() => {
              toast({
                title: "Import failed",
                description: result.error || "Unknown error occurred",
                variant: "destructive",
                duration: 5000,
              });
            }, 100);
          }
        } catch (error) {
          setImportProgress(null);
          console.error("Import error:", error);
          setTimeout(() => {
            toast({
              title: "Import failed",
              description:
                error instanceof Error
                  ? error.message
                  : "Unknown error occurred",
              variant: "destructive",
              duration: 5000,
            });
          }, 100);
        }
      }
    } catch (error) {
      setImportProgress(null);
      console.error("Import error:", error);
      setTimeout(() => {
        toast({
          title: "Import failed",
          description: "Failed to open file dialog",
          variant: "destructive",
          duration: 5000,
        });
      }, 100);
    } finally {
      setIsImporting(false);
      setImportProgress(null);
    }
  };

  const handleNewFile = async () => {
    try {
      const sceneState = useSceneStore.getState();
      sceneState.clearScene();
      sceneState.addObject("cube", [0, 0, 0]);
      sceneState.setCameraPosition([5, 5, 5]);
      sceneState.setCameraTarget([0, 0, 0]);
      sceneState.setSceneMetadata({
        name: "Untitled Scene",
        isModified: false,
      });

      // Auto-save the new project
      const autoSaveDir = await getAutoSaveDir();
      const fileName = generateAutoSaveFilename();
      const filePath = await join(autoSaveDir, fileName);

      const result = await saveScene(sceneState, filePath, {
        name: "Untitled Scene",
        description: "Auto-saved new project",
      });

      if (result.success) {
        sceneState.setCurrentFilePath(filePath);
        sceneState.markSceneSaved();

        // Delay toast to prevent potential state conflicts
        setTimeout(() => {
          toast({
            title: "New project created",
            description: `Started a new scene with a basic cube. Auto-saved to ${fileName}`,
            duration: 3000,
          });
        }, 100);
      } else {
        // If auto-save fails, still show success but warn about save issue
        setTimeout(() => {
          toast({
            title: "New project created",
            description:
              "Started a new scene with a basic cube. (Auto-save failed)",
            duration: 3000,
          });
        }, 100);
      }
    } catch (error) {
      console.error("Error creating new project:", error);
      setTimeout(() => {
        toast({
          title: "Error creating project",
          description: "Failed to create new project",
          variant: "destructive",
          duration: 4000,
        });
      }, 100);
    }
  };

  const handleSave = async () => {
    const sceneState = useSceneStore.getState();
    const result = await saveScene(sceneState);

    if (result.success) {
      sceneState.markSceneSaved();
      // Delay toast to prevent potential state conflicts
      setTimeout(() => {
        toast({
          title: "Scene saved",
          description: "Your scene has been saved successfully.",
          duration: 2000,
        });
      }, 100);
    } else {
      // Delay error toast as well
      setTimeout(() => {
        toast({
          title: "Save failed",
          description: result.error || "Failed to save scene",
          variant: "destructive",
          duration: 4000,
        });
      }, 100);
    }
  };

  const handleLoad = async () => {
    const result = await loadScene();

    if (result.success && result.data) {
      const sceneState = useSceneStore.getState();
      sceneState.loadScene(result.data.scene);
      sceneState.setSceneMetadata({
        name: result.data.metadata.name,
      });
      sceneState.setCurrentFilePath(null);

      // Delay toast to prevent potential state conflicts
      setTimeout(() => {
        toast({
          title: "Scene loaded",
          description: `"${result.data?.metadata?.name ?? "Scene"}" has been loaded successfully.`,
          duration: 2000,
        });
      }, 100);
    } else if (result.error !== "Load cancelled") {
      // Delay error toast as well
      setTimeout(() => {
        toast({
          title: "Load failed",
          description: result.error || "Failed to load scene",
          variant: "destructive",
          duration: 4000,
        });
      }, 100);
    }
  };

  return (
    <div className="flex items-center gap-2 border-b bg-background p-2">
      <div className="flex items-center gap-2 pr-3 pl-22">
        <Button
          className="h-auto p-0 font-bold text-lg transition-all duration-300 hover:scale-105 hover:bg-transparent hover:text-primary hover:shadow-lg"
          onClick={() => navigate({ to: "/welcome" })}
          title="Go to Welcome Screen"
          variant="ghost"
        >
          Rengine
        </Button>
        <Button
          className={cn(
            "h-auto p-0 font-medium text-sm hover:bg-transparent hover:text-primary",
            rightSidebarContext === "lighting" && "text-primary"
          )}
          onClick={() => setRightSidebarContext("lighting")}
          title="Lighting Panel"
          variant="ghost"
        >
          Lighting
        </Button>
        <Button
          className={cn(
            "h-auto p-0 font-medium text-sm hover:bg-transparent hover:text-primary",
            rightSidebarContext === "materials" && "text-primary"
          )}
          onClick={() => setRightSidebarContext("materials")}
          title="Materials Panel"
          variant="ghost"
        >
          Materials
        </Button>
        <Button
          className={cn(
            "h-auto p-0 font-medium text-sm hover:bg-transparent hover:text-primary",
            rightSidebarContext === "tools" && "text-primary"
          )}
          onClick={() => setRightSidebarContext("tools")}
          title="Tools Panel"
          variant="ghost"
        >
          Tools
        </Button>
      </div>

      <Separator className="h-6" orientation="vertical" />

      <div className="flex items-center gap-1">
        <Button
          className="h-8 px-3"
          onClick={handleNewFile}
          size="sm"
          title="New File"
          variant="ghost"
        >
          <File className="mr-2 h-4 w-4" />
          New
        </Button>

        <Button
          className="h-8 px-3"
          onClick={handleSave}
          size="sm"
          title="Save Scene"
          variant="ghost"
        >
          <Save className="mr-2 h-4 w-4" />
          Save
        </Button>

        <Button
          className="h-8 px-3"
          onClick={handleLoad}
          size="sm"
          title="Load Scene"
          variant="ghost"
        >
          <FolderOpen className="mr-2 h-4 w-4" />
          Load
        </Button>
      </div>

      <Separator className="h-6" orientation="vertical" />

      <div className="flex items-center gap-1">
        {(Object.keys(toolIcons) as TransformTool[]).map((tool) => {
          const Icon = toolIcons[tool];
          return (
            <Button
              className="h-8 w-8 p-0"
              key={tool}
              onClick={() => handleToolSelect(tool)}
              size="sm"
              variant={activeTool === tool ? "secondary" : "ghost"}
            >
              <Icon className="h-4 w-4" />
            </Button>
          );
        })}
      </div>

      <Separator className="h-6" orientation="vertical" />

      <DropdownMenu>
        <DropdownMenuTrigger
          className={cn(
            "group/button inline-flex h-8 w-8 shrink-0 select-none items-center justify-center gap-1 whitespace-nowrap rounded-[min(var(--radius-md),10px)] border border-transparent bg-clip-padding p-0 font-medium text-sm outline-none transition-all focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50",
            placementMode.active
              ? "bg-secondary text-secondary-foreground hover:bg-secondary/80 aria-expanded:bg-secondary aria-expanded:text-secondary-foreground"
              : "hover:bg-muted hover:text-foreground aria-expanded:bg-muted aria-expanded:text-foreground dark:hover:bg-muted/50"
          )}
        >
          <Plus className="pointer-events-none h-4 w-4 shrink-0" />
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start">
          <DropdownMenuItem onClick={() => handleAddObject("cube")}>
            <Box className="mr-2 h-4 w-4" />
            {placementMode.active && placementMode.objectType === "cube"
              ? "Placing Cube"
              : "Add Cube"}
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => handleAddObject("sphere")}>
            <Circle className="mr-2 h-4 w-4" />
            {placementMode.active && placementMode.objectType === "sphere"
              ? "Placing Sphere"
              : "Add Sphere"}
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => handleAddObject("plane")}>
            <Square className="mr-2 h-4 w-4" />
            {placementMode.active && placementMode.objectType === "plane"
              ? "Placing Plane"
              : "Add Plane"}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <Separator className="h-6" orientation="vertical" />

      {/* Add Light Dropdown */}
      <DropdownMenu>
        <DropdownMenuTrigger className="group/button inline-flex h-8 w-8 shrink-0 select-none items-center justify-center gap-1 whitespace-nowrap rounded-[min(var(--radius-md),10px)] border border-transparent bg-clip-padding p-0 font-medium text-sm outline-none transition-all hover:bg-muted hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-expanded:bg-muted aria-expanded:text-foreground dark:hover:bg-muted/50">
          <Lightbulb className="pointer-events-none h-4 w-4 shrink-0" />
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start">
          <DropdownMenuItem onClick={() => addLight("directional")}>
            <Sun className="mr-2 h-4 w-4" />
            Directional Light
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => addLight("point")}>
            <Lightbulb className="mr-2 h-4 w-4" />
            Point Light
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => addLight("spot")}>
            <Lightbulb className="mr-2 h-4 w-4" />
            Spot Light
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => addLight("ambient")}>
            <Circle className="mr-2 h-4 w-4" />
            Ambient Light
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => addLight("hemisphere")}>
            <Square className="mr-2 h-4 w-4" />
            Hemisphere Light
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <Button
        className="h-8 w-8 p-0"
        onClick={() => setLightsVisible(!lightsVisible)}
        size="sm"
        title={lightsVisible ? "Hide Lights" : "Show Lights"}
        variant={lightsVisible ? "secondary" : "ghost"}
      >
        <Lightbulb className={`h-4 w-4 ${lightsVisible ? "" : "opacity-50"}`} />
      </Button>

      <DropdownMenu>
        <DropdownMenuTrigger className="group/button inline-flex h-8 shrink-0 select-none items-center justify-center gap-1 whitespace-nowrap rounded-[min(var(--radius-md),10px)] border border-transparent bg-clip-padding px-3 py-0 font-medium text-sm outline-none transition-all hover:bg-muted hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-expanded:bg-muted aria-expanded:text-foreground dark:hover:bg-muted/50">
          <Sun className="pointer-events-none h-4 w-4 shrink-0" />
          Presets
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start">
          {getPresetList().map((preset) => (
            <DropdownMenuItem
              key={preset.id}
              onClick={() => applyLightingPreset(preset.id)}
            >
              <div className="flex flex-col">
                <span className="font-medium">{preset.name}</span>
                <span className="text-muted-foreground text-xs">
                  {preset.description}
                </span>
              </div>
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>

      <Separator className="h-6" orientation="vertical" />

      <div className="flex items-center gap-2 px-3 py-1">
        <span
          className={`font-medium text-sm ${sceneMetadata.isModified ? "text-orange-600" : ""}`}
        >
          {sceneMetadata.name}
          {sceneMetadata.isModified && " *"}
        </span>
      </div>

      {selectionCount > 0 && (
        <div className="flex items-center gap-2 rounded-md bg-secondary px-3 py-1">
          <span className="font-medium text-sm">{selectionCount} selected</span>
          <Button
            className="h-6 px-2"
            onClick={clearSelection}
            size="sm"
            variant="ghost"
          >
            Clear
          </Button>
        </div>
      )}

      <div className="flex items-center gap-1">
        <Button
          className="h-8 px-3"
          disabled={isImporting}
          onClick={handleImport}
          size="sm"
          title="Import 3D Model"
          variant="ghost"
        >
          <Download className="mr-2 h-4 w-4" />
          {isImporting ? "Importing..." : "Import Model"}
        </Button>

        {importProgress && (
          <div className="flex min-w-48 items-center gap-2">
            <Progress
              className="h-2 flex-1"
              value={(importProgress.loaded / importProgress.total) * 100}
            />
            <span className="min-w-0 truncate text-muted-foreground text-xs">
              {importProgress.stage}
            </span>
          </div>
        )}

        <Button
          className="h-8 px-3"
          onClick={() => setExportDialogOpen(true)}
          size="sm"
          title="Export Scene"
          variant="ghost"
        >
          <Upload className="mr-2 h-4 w-4" />
          Export
        </Button>
      </div>

      <Separator className="h-6" orientation="vertical" />

      <div className="flex items-center gap-1">
        <Button
          className="h-8 px-3"
          onClick={() => setSettingsDialogOpen(true)}
          size="sm"
          title="Settings"
          variant="ghost"
        >
          <SettingsIcon className="mr-2 h-4 w-4" />
          Settings
        </Button>
      </div>

      <div className="flex items-center gap-1">
        <Button
          className="h-8 px-3"
          onClick={selectAll}
          size="sm"
          variant="ghost"
        >
          Select All
        </Button>
        <Button
          className="h-8 px-3"
          onClick={() => useSceneStore.getState().clearScene()}
          size="sm"
          variant="ghost"
        >
          Clear Scene
        </Button>
      </div>

      <ExportDialog
        onOpenChange={setExportDialogOpen}
        open={exportDialogOpen}
      />

      <SettingsDialog
        onOpenChange={setSettingsDialogOpen}
        open={settingsDialogOpen}
      />
    </div>
  );
}
