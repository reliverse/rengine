import { SceneCanvas } from "./scene-canvas";
import { Toolbar } from "./toolbar";
import { UnifiedSidebar } from "./unified-sidebar";

export function RengineEditor() {
  return (
    <div className="flex h-screen flex-col bg-background">
      {/* Toolbar */}
      <Toolbar />

      {/* Main Editor Area */}
      <div className="flex flex-1 overflow-hidden">
        {/* Left Sidebar - Scene Hierarchy */}
        <UnifiedSidebar context="scene" />

        {/* 3D Canvas - Main Viewport */}
        <div className="relative flex-1">
          <SceneCanvas />
        </div>

        {/* Right Sidebar - Properties/Tools */}
        <UnifiedSidebar context="tools" />
      </div>
    </div>
  );
}
