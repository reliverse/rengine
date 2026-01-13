import { documentDir, homeDir, join, tempDir } from "@tauri-apps/api/path";
import { mkdir } from "@tauri-apps/plugin-fs";
import { Package, Settings as SettingsIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { cn } from "~/lib/utils";
import { useSceneStore } from "~/stores/scene-store";
import { saveScene } from "~/utils/scene-persistence";
import { LeftSidebar } from "./left-sidebar";
import { ModelViewerCanvas } from "./model-viewer-canvas";
import { PerformanceFooter } from "./performance-footer";
import { RightSidebar, type SidebarContext } from "./right-sidebar";
import { SceneCanvas } from "./scene-canvas";
import { Toolbar } from "./toolbar";
import { Button } from "./ui/button";

export type ViewportMode = "editor" | "model-viewer";

export function RengineEditor() {
  const [rightSidebarContext, setRightSidebarContext] =
    useState<SidebarContext>("tools");
  const [viewportMode, setViewportMode] = useState<ViewportMode>("editor");
  const [showLeftSidebar, setShowLeftSidebar] = useState(false);
  const [showRightSidebar, setShowRightSidebar] = useState(false);
  useEffect(() => {
    const initializeNewProject = async () => {
      try {
        const sceneState = useSceneStore.getState();

        // Add ground plane to empty project
        if (sceneState.objects.length === 0) {
          console.log("Adding ground plane to new project...");
          sceneState.addObject("plane", [0, 0, 0]);

          // Update the ground plane properties for better appearance
          const groundPlane = sceneState.objects.find(
            (obj) => obj.name === "Plane"
          );
          if (groundPlane) {
            sceneState.updateObject(groundPlane.id, {
              name: "Ground Plane",
              scale: [20, 1, 20], // Make it a larger ground platform
              color: "#2a2a2a", // Darker gray color for ground
              position: [0, -0.01, 0], // Slightly below origin to avoid z-fighting
            });
          }
        }

        if (!sceneState.currentFilePath) {
          let autoSaveDir: string;
          try {
            const home = await homeDir();
            autoSaveDir = await join(home, ".reliverse", "rengine", "projects");
            console.log("Creating primary auto-save directory:", autoSaveDir);
            await mkdir(autoSaveDir, { recursive: true });
            console.log(
              "✅ Primary auto-save directory created/verified:",
              autoSaveDir
            );
          } catch (error) {
            console.warn(
              "❌ Failed to create primary auto-save directory:",
              error
            );

            try {
              const documents = await documentDir();
              autoSaveDir = await join(documents, "Rengine", "Projects");
              console.log("Trying fallback auto-save directory:", autoSaveDir);
              await mkdir(autoSaveDir, { recursive: true });
              console.log(
                "✅ Fallback auto-save directory created/verified:",
                autoSaveDir
              );
            } catch (error2) {
              console.warn(
                "❌ Failed to create fallback auto-save directory:",
                error2
              );

              // Last resort: temp directory
              try {
                const temp = await tempDir();
                autoSaveDir = await join(temp, "rengine-autosave");
                console.log(
                  "Using temp directory as last resort:",
                  autoSaveDir
                );
                await mkdir(autoSaveDir, { recursive: true });
                console.log(
                  "✅ Temp auto-save directory created:",
                  autoSaveDir
                );
              } catch (error3) {
                console.error(
                  "❌ All auto-save directory creation attempts failed:",
                  error3
                );
                throw new Error(
                  "Could not create any auto-save directory. Please check file system permissions."
                );
              }
            }
          }

          const now = new Date();
          const timestamp = now.toISOString().slice(0, 19).replace(/:/g, "-");
          const fileName = `initial_${timestamp}.rengine`;
          const filePath = await join(autoSaveDir, fileName);

          const result = await saveScene(sceneState, filePath, {
            name: "Welcome Scene",
            description: "Auto-saved initial scene",
          });

          if (result.success) {
            sceneState.setCurrentFilePath(filePath);
            sceneState.markSceneSaved();
            console.log("Initial scene auto-saved:", filePath);
          }
        }
      } catch (error) {
        console.error("Failed to initialize new project:", error);
      }
    };

    initializeNewProject();
  }, []);

  return (
    <div className="flex h-screen flex-col bg-background">
      {/* Toolbar */}
      <Toolbar />

      {/* Main Editor Area */}
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Viewport Mode Tabs */}
        <div className="flex border-b bg-background/50">
          <Button
            className={cn(
              "h-8 flex-1 rounded-none px-3 font-medium text-xs",
              viewportMode === "editor" &&
                "border-primary border-b-2 bg-secondary text-secondary-foreground"
            )}
            onClick={() => setViewportMode("editor")}
            size="sm"
            variant={viewportMode === "editor" ? "secondary" : "ghost"}
          >
            Editor
          </Button>
          <Button
            className={cn(
              "h-8 flex-1 rounded-none px-3 font-medium text-xs",
              viewportMode === "model-viewer" &&
                "border-primary border-b-2 bg-secondary text-secondary-foreground"
            )}
            onClick={() => setViewportMode("model-viewer")}
            size="sm"
            variant={viewportMode === "model-viewer" ? "secondary" : "ghost"}
          >
            Model Viewer
          </Button>
        </div>

        {/* Main Content */}
        <div className="flex flex-1 overflow-hidden">
          {/* Left Sidebar - Scene Hierarchy */}
          <div className={cn("md:flex", showLeftSidebar ? "flex" : "hidden")}>
            <LeftSidebar />
          </div>

          {/* 3D Canvas - Main Viewport */}
          <div className="relative min-w-0 flex-1">
            {/* Mobile Sidebar Toggles */}
            <div className="absolute top-2 left-2 z-20 flex gap-1 md:hidden">
              <Button
                className="h-8 w-8 p-0"
                onClick={() => setShowLeftSidebar(!showLeftSidebar)}
                size="sm"
                title="Toggle Scene Hierarchy"
                variant="secondary"
              >
                <Package className="h-4 w-4" />
              </Button>
            </div>

            <div className="absolute top-2 right-2 z-20 flex gap-1 lg:hidden">
              <Button
                className="h-8 w-8 p-0"
                onClick={() => setShowRightSidebar(!showRightSidebar)}
                size="sm"
                title="Toggle Properties Panel"
                variant="secondary"
              >
                <SettingsIcon className="h-4 w-4" />
              </Button>
            </div>

            {viewportMode === "editor" ? (
              <SceneCanvas />
            ) : (
              <ModelViewerCanvas />
            )}
          </div>

          {/* Right Sidebar - Properties/Tools */}
          <div className={cn("lg:flex", showRightSidebar ? "flex" : "hidden")}>
            <RightSidebar
              context={rightSidebarContext}
              isRightSidebar={true}
              onContextChange={setRightSidebarContext}
            />
          </div>
        </div>

        {/* Performance Footer with Scene Info */}
        <PerformanceFooter />
      </div>
    </div>
  );
}
