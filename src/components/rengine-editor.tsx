import { documentDir, homeDir, join, tempDir } from "@tauri-apps/api/path";
import { mkdir } from "@tauri-apps/plugin-fs";
import { useEffect, useState } from "react";
import { useSceneStore } from "~/stores/scene-store";
import { saveScene } from "~/utils/scene-persistence";
import { SceneCanvas } from "./scene-canvas";
import { Toolbar } from "./toolbar";
import { type SidebarContext, UnifiedSidebar } from "./unified-sidebar";

export function RengineEditor() {
  const [rightSidebarContext, setRightSidebarContext] =
    useState<SidebarContext>("tools");
  useEffect(() => {
    const autoSaveInitialScene = async () => {
      try {
        const sceneState = useSceneStore.getState();

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
        console.error("Failed to auto-save initial scene:", error);
      }
    };

    autoSaveInitialScene();
  }, []);

  return (
    <div className="flex h-screen flex-col bg-background">
      {/* Toolbar */}
      <Toolbar
        rightSidebarContext={rightSidebarContext}
        setRightSidebarContext={setRightSidebarContext}
      />

      {/* Main Editor Area */}
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Main Content */}
        <div className="flex flex-1 overflow-hidden">
          {/* Left Sidebar - Scene Hierarchy */}
          <UnifiedSidebar context="scene" />

          {/* 3D Canvas - Main Viewport */}
          <div className="relative flex-1">
            <SceneCanvas />
          </div>

          {/* Right Sidebar - Properties/Tools */}
          <UnifiedSidebar context={rightSidebarContext} />
        </div>

        {/* Status Bar */}
        <div className="flex h-12 items-center justify-between border-t bg-background px-4 py-2">
          {/* Left Side - Basic Info */}
          <div className="flex items-center gap-4">
            {/* Basic Scene Info */}
            <div className="flex items-center gap-3 text-muted-foreground text-xs">
              <div className="flex items-center gap-1">
                <span>Objects:</span>
                <span className="font-medium font-mono">
                  {useSceneStore.getState().objects.length}
                </span>
              </div>
              <div className="flex items-center gap-1">
                <span>Lights:</span>
                <span className="font-medium font-mono">
                  {useSceneStore.getState().lights.length}
                </span>
              </div>
            </div>
          </div>

          {/* Right Side - Additional Info */}
          <div className="flex items-center gap-3 text-muted-foreground text-xs">
            <div className="flex items-center gap-1">
              <span>Mode:</span>
              <span className="font-medium">Editor</span>
            </div>
            <div className="flex items-center gap-1">
              <span>Scene:</span>
              <span className="font-medium">
                {useSceneStore.getState().currentFilePath?.split("/").pop() ||
                  "Untitled"}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
