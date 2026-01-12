import { useEffect, useState } from "react";
import { AnimationPanel } from "~/components/animation-panel";
import { LightPropertyPanel } from "~/components/light-property-panel";
import { MaterialPropertyPanel } from "~/components/material-property-panel";
import { ObjectMaterialPanel } from "~/components/object-material-panel";
import { PropertyPanel } from "~/components/property-panel";
import { ColEditor } from "~/components/tools/col-editor";
import { IdeEditor } from "~/components/tools/ide-editor";
import { ImgEditor } from "~/components/tools/img-editor";
import { RwAnalyzer } from "~/components/tools/rw-analyzer";
import { TxdEditor } from "~/components/tools/txd-editor";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";
import { useSelectedMaterial } from "~/stores/material-store";
import { useSelectedLight, useSelectedObject } from "~/stores/scene-store";
import { debugLog } from "~/utils/debug";
import { LightingPanel } from "./lighting-panel";
import { SceneHierarchy } from "./scene-hierarchy";
import { DffViewer } from "./tools/dff-viewer";

export type SidebarContext =
  | "scene"
  | "tools"
  | "lighting"
  | "materials"
  | "renderware"
  | "debug"
  | "settings";

interface RightSidebarProps {
  context: SidebarContext;
  className?: string;
  isRightSidebar?: boolean;
  onContextChange?: (context: SidebarContext) => void;
}

function SceneSidebarContent() {
  return <SceneHierarchy />;
}

function ToolsSidebarContent() {
  const selectedLight = useSelectedLight();
  const selectedMaterial = useSelectedMaterial();
  const selectedObject = useSelectedObject();

  // Check if selected object has animations
  const hasAnimations = selectedObject?.animationController?.hasAnimations();

  return (
    <div className="flex h-full flex-col">
      {/* Priority: Material > Light > Object */}
      {selectedMaterial ? (
        <MaterialPropertyPanel />
      ) : selectedLight ? (
        <LightPropertyPanel />
      ) : selectedObject ? (
        <PropertyPanel />
      ) : (
        <div className="p-4">
          <h3 className="mb-4 font-semibold text-muted-foreground text-sm">
            Object Properties
          </h3>
          <p className="text-muted-foreground text-xs">
            Select an object, light, or material to edit its properties
          </p>
        </div>
      )}

      {/* Animation panel for animated objects */}
      {hasAnimations && (
        <div className="mt-4 px-4">
          <AnimationPanel />
        </div>
      )}
    </div>
  );
}

function LightingSidebarContent() {
  return <LightingPanel />;
}

function MaterialsSidebarContent() {
  return <ObjectMaterialPanel />;
}

function RenderWareSidebarContent() {
  const [activeEditor, setActiveEditor] = useState<
    "img" | "txd" | "dff" | "col" | "analyzer" | "ide"
  >("img");

  return (
    <div className="flex h-full flex-col">
      {/* Editor selector tabs */}
      <div className="flex border-b">
        <Button
          className={`h-8 flex-1 rounded-none px-2 font-medium text-xs ${
            activeEditor === "img"
              ? "border-primary border-b-2 bg-secondary text-secondary-foreground"
              : "hover:bg-muted"
          }`}
          onClick={() => setActiveEditor("img")}
          size="sm"
          variant="ghost"
        >
          IMG Archives
        </Button>
        <Button
          className={`h-8 flex-1 rounded-none px-2 font-medium text-xs ${
            activeEditor === "txd"
              ? "border-primary border-b-2 bg-secondary text-secondary-foreground"
              : "hover:bg-muted"
          }`}
          onClick={() => setActiveEditor("txd")}
          size="sm"
          variant="ghost"
        >
          TXD Textures
        </Button>
        <Button
          className={`h-8 flex-1 rounded-none px-2 font-medium text-xs ${
            activeEditor === "dff"
              ? "border-primary border-b-2 bg-secondary text-secondary-foreground"
              : "hover:bg-muted"
          }`}
          onClick={() => setActiveEditor("dff")}
          size="sm"
          variant="ghost"
        >
          DFF Viewer
        </Button>
        <Button
          className={`h-8 flex-1 rounded-none px-2 font-medium text-xs ${
            activeEditor === "analyzer"
              ? "border-primary border-b-2 bg-secondary text-secondary-foreground"
              : "hover:bg-muted"
          }`}
          onClick={() => setActiveEditor("analyzer")}
          size="sm"
          variant="ghost"
        >
          RW Analyze
        </Button>
        <Button
          className={`h-8 flex-1 rounded-none px-2 font-medium text-xs ${
            activeEditor === "col"
              ? "border-primary border-b-2 bg-secondary text-secondary-foreground"
              : "hover:bg-muted"
          }`}
          onClick={() => setActiveEditor("col")}
          size="sm"
          variant="ghost"
        >
          COL Collision
        </Button>
        <Button
          className={`h-8 flex-1 rounded-none px-2 font-medium text-xs ${
            activeEditor === "ide"
              ? "border-primary border-b-2 bg-secondary text-secondary-foreground"
              : "hover:bg-muted"
          }`}
          onClick={() => setActiveEditor("ide")}
          size="sm"
          variant="ghost"
        >
          IDE Editor
        </Button>
      </div>

      {/* Editor content */}
      <div className="flex-1 overflow-hidden">
        {activeEditor === "img" ? (
          <ImgEditor />
        ) : activeEditor === "txd" ? (
          <TxdEditor />
        ) : activeEditor === "dff" ? (
          <DffViewer />
        ) : activeEditor === "col" ? (
          <ColEditor />
        ) : activeEditor === "analyzer" ? (
          <RwAnalyzer />
        ) : (
          <IdeEditor />
        )}
      </div>
    </div>
  );
}

function DebugSidebarContent() {
  const [sessionInfo, setSessionInfo] = useState<any>(null);
  const [memoryStats, setMemoryStats] = useState<any>(null);

  useEffect(() => {
    const loadDebugInfo = async () => {
      try {
        const [session, memory] = await Promise.all([
          debugLog.getSessionInfo(),
          debugLog.getMemoryStats(),
        ]);
        setSessionInfo(session);
        setMemoryStats(memory);
      } catch (error) {
        console.error("Failed to load debug info:", error);
      }
    };

    loadDebugInfo();
  }, []);

  return (
    <div className="flex h-full flex-col p-4">
      <h3 className="mb-4 font-semibold text-muted-foreground text-sm">
        Debug System
      </h3>

      <div className="space-y-4">
        {/* Session Info */}
        <div>
          <h4 className="mb-2 font-medium text-sm">Session Info</h4>
          <div className="space-y-1 text-muted-foreground text-xs">
            {sessionInfo && (
              <>
                <div>Session: {sessionInfo.session_id?.slice(0, 8)}...</div>
                <div>
                  Started:{" "}
                  {new Date(sessionInfo.session_start).toLocaleString()}
                </div>
                <div>Log: {sessionInfo.log_file_path}</div>
              </>
            )}
          </div>
        </div>

        {/* Memory Stats */}
        <div>
          <h4 className="mb-2 font-medium text-sm">Memory Usage</h4>
          <div className="space-y-1 text-muted-foreground text-xs">
            {memoryStats && (
              <>
                <div>
                  Process: {memoryStats.process_memory_mb?.toFixed(1)} MB
                </div>
                <div>
                  System: {memoryStats.used_mb?.toFixed(1)} /{" "}
                  {memoryStats.total_mb?.toFixed(1)} MB
                </div>
                <div>Available: {memoryStats.available_mb?.toFixed(1)} MB</div>
              </>
            )}
          </div>
        </div>

        {/* Quick Actions */}
        <div>
          <h4 className="mb-2 font-medium text-sm">Quick Actions</h4>
          <div className="space-y-2">
            <Button
              className="w-full text-xs"
              onClick={async () => {
                await debugLog.userAction("debug_manual_refresh");
                const memory = await debugLog.getMemoryStats();
                setMemoryStats(memory);
              }}
              size="sm"
              variant="outline"
            >
              Refresh Memory Stats
            </Button>
            <Button
              className="w-full text-xs"
              onClick={async () => {
                await debugLog.userAction("debug_system_info");
                const session = await debugLog.getSessionInfo();
                setSessionInfo(session);
              }}
              size="sm"
              variant="outline"
            >
              Refresh Session Info
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

function SettingsSidebarContent() {
  return (
    <div className="p-4">
      <h3 className="mb-4 font-semibold text-muted-foreground text-sm">
        Settings
      </h3>
      <p className="text-muted-foreground text-xs">Editor settings</p>
    </div>
  );
}

export function RightSidebar({
  context,
  className,
  isRightSidebar,
  onContextChange,
}: RightSidebarProps) {
  const renderSidebarContent = () => {
    switch (context) {
      case "scene":
        return <SceneSidebarContent />;
      case "tools":
        return <ToolsSidebarContent />;
      case "lighting":
        return <LightingSidebarContent />;
      case "materials":
        return <MaterialsSidebarContent />;
      case "renderware":
        return <RenderWareSidebarContent />;
      case "debug":
        return <DebugSidebarContent />;
      case "settings":
        return <SettingsSidebarContent />;
      default:
        return null;
    }
  };

  return (
    <div
      className={`w-64 border-border border-r bg-muted/30 ${className || ""}`}
      style={{ zIndex: 10 }}
    >
      {/* Tab buttons for right sidebar */}
      {isRightSidebar && (
        <div className="flex border-b bg-background/50 p-1">
          <Button
            className={cn(
              "h-8 flex-1 px-2 font-medium text-xs",
              context === "tools" && "bg-secondary text-secondary-foreground"
            )}
            onClick={() => onContextChange?.("tools")}
            size="sm"
            variant={context === "tools" ? "secondary" : "ghost"}
          >
            Tools
          </Button>
          <Button
            className={cn(
              "h-8 flex-1 px-2 font-medium text-xs",
              context === "lighting" && "bg-secondary text-secondary-foreground"
            )}
            onClick={() => onContextChange?.("lighting")}
            size="sm"
            variant={context === "lighting" ? "secondary" : "ghost"}
          >
            Lighting
          </Button>
          <Button
            className={cn(
              "h-8 flex-1 px-2 font-medium text-xs",
              context === "materials" &&
                "bg-secondary text-secondary-foreground"
            )}
            onClick={() => onContextChange?.("materials")}
            size="sm"
            variant={context === "materials" ? "secondary" : "ghost"}
          >
            Materials
          </Button>
          <Button
            className={cn(
              "h-8 flex-1 px-2 font-medium text-xs",
              context === "renderware" &&
                "bg-secondary text-secondary-foreground"
            )}
            onClick={() => onContextChange?.("renderware")}
            size="sm"
            variant={context === "renderware" ? "secondary" : "ghost"}
          >
            RW
          </Button>
          <Button
            className={cn(
              "h-8 flex-1 px-2 font-medium text-xs",
              context === "debug" && "bg-secondary text-secondary-foreground"
            )}
            onClick={() => onContextChange?.("debug")}
            size="sm"
            variant={context === "debug" ? "secondary" : "ghost"}
          >
            Debug
          </Button>
        </div>
      )}
      {renderSidebarContent()}
    </div>
  );
}
