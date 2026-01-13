import {
  Bug,
  Lightbulb,
  Palette,
  Settings,
  Terminal,
  Wrench,
} from "lucide-react";
import { memo, useEffect, useState } from "react";
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
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { cn } from "~/lib/utils";
import { useSelectedMaterial } from "~/stores/material-store";
import { useSelectedLight, useSelectedObject } from "~/stores/scene-store";
import { debugLog } from "~/utils/debug";
import { LightingPanel } from "./lighting-panel";
import { DffViewer } from "./tools/dff-viewer";

export type SidebarContext =
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

function ToolsSidebarContent() {
  const selectedLight = useSelectedLight();
  const selectedMaterial = useSelectedMaterial();
  const selectedObject = useSelectedObject();

  // Check if selected object has animations
  const hasAnimations = selectedObject?.animationController?.hasAnimations();

  return (
    <div className="scrollbar-thin scrollbar-thumb-muted scrollbar-track-transparent h-full overflow-auto">
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
  return (
    <div className="scrollbar-thin scrollbar-thumb-muted scrollbar-track-transparent h-full overflow-auto">
      <LightingPanel />
    </div>
  );
}

function MaterialsSidebarContent() {
  return (
    <div className="scrollbar-thin scrollbar-thumb-muted scrollbar-track-transparent h-full overflow-auto">
      <ObjectMaterialPanel />
    </div>
  );
}

function RenderWareSidebarContent() {
  const [activeEditor, setActiveEditor] = useState<
    "img" | "txd" | "dff" | "col" | "analyzer" | "ide"
  >("img");

  return (
    <div className="flex h-full flex-col">
      <Tabs
        className="flex h-full flex-col"
        onValueChange={(value) => setActiveEditor(value as typeof activeEditor)}
        value={activeEditor}
      >
        <TabsList className="mt-2 grid w-full shrink-0 grid-cols-3 gap-2 bg-muted/50 p-1">
          <TabsTrigger
            className="flex items-center gap-1 px-1 text-xs"
            value="img"
          >
            IMG
          </TabsTrigger>
          <TabsTrigger
            className="flex items-center gap-1 px-1 text-xs"
            value="txd"
          >
            TXD
          </TabsTrigger>
          <TabsTrigger
            className="flex items-center gap-1 px-1 text-xs"
            value="dff"
          >
            DFF
          </TabsTrigger>
          <TabsTrigger
            className="flex items-center gap-1 px-1 text-xs"
            value="col"
          >
            COL
          </TabsTrigger>
          <TabsTrigger
            className="flex items-center gap-1 px-1 text-xs"
            value="analyzer"
          >
            RW
          </TabsTrigger>
          <TabsTrigger
            className="flex items-center gap-1 px-1 text-xs"
            value="ide"
          >
            IDE
          </TabsTrigger>
        </TabsList>

        <TabsContent className="mt-4 flex-1 overflow-hidden" value="img">
          <ImgEditor />
        </TabsContent>

        <TabsContent className="mt-4 flex-1 overflow-hidden" value="txd">
          <TxdEditor />
        </TabsContent>

        <TabsContent className="mt-4 flex-1 overflow-hidden" value="dff">
          <DffViewer />
        </TabsContent>

        <TabsContent className="mt-4 flex-1 overflow-hidden" value="col">
          <ColEditor />
        </TabsContent>

        <TabsContent className="mt-4 flex-1 overflow-hidden" value="analyzer">
          <RwAnalyzer />
        </TabsContent>

        <TabsContent className="mt-4 flex-1 overflow-hidden" value="ide">
          <IdeEditor />
        </TabsContent>
      </Tabs>
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
    <div className="scrollbar-thin scrollbar-thumb-muted scrollbar-track-transparent h-full overflow-auto p-4">
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
    <div className="scrollbar-thin scrollbar-thumb-muted scrollbar-track-transparent h-full overflow-auto p-4">
      <h3 className="mb-4 font-semibold text-muted-foreground text-sm">
        Settings
      </h3>
      <p className="text-muted-foreground text-xs">Editor settings</p>
    </div>
  );
}

export const RightSidebar = memo(function RightSidebar({
  context,
  className,
  isRightSidebar,
  onContextChange,
}: RightSidebarProps) {
  const [activeTab, setActiveTab] = useState(context);

  // Sync with external context changes
  useEffect(() => {
    setActiveTab(context);
  }, [context]);

  const handleTabChange = (value: string) => {
    setActiveTab(value as SidebarContext);
    onContextChange?.(value as SidebarContext);
  };

  return (
    <div
      className={cn(
        "flex h-[calc(100%-3.5rem)] w-72 shrink-0 flex-col overflow-x-auto overflow-y-hidden rounded-lg border md:w-80 lg:w-96",
        className
      )}
    >
      {isRightSidebar && (
        <Tabs
          className="flex h-full flex-col"
          onValueChange={handleTabChange}
          value={activeTab}
        >
          <TabsList className="grid w-full shrink-0 grid-cols-3 gap-2 bg-muted/50 p-1">
            <TabsTrigger className="flex items-center gap-2" value="tools">
              <Wrench className="h-4 w-4" />
              Tools
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-2" value="lighting">
              <Lightbulb className="h-4 w-4" />
              Lighting
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-2" value="materials">
              <Palette className="h-4 w-4" />
              Materials
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-2" value="renderware">
              <Terminal className="h-4 w-4" />
              RenderWare
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-2" value="debug">
              <Bug className="h-4 w-4" />
              Debug
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-2" value="settings">
              <Settings className="h-4 w-4" />
              Settings
            </TabsTrigger>
          </TabsList>

          <TabsContent className="mt-4 flex-1 overflow-hidden" value="tools">
            <ToolsSidebarContent />
          </TabsContent>

          <TabsContent className="mt-4 flex-1 overflow-hidden" value="lighting">
            <LightingSidebarContent />
          </TabsContent>

          <TabsContent
            className="mt-4 flex-1 overflow-hidden"
            value="materials"
          >
            <MaterialsSidebarContent />
          </TabsContent>

          <TabsContent
            className="mt-4 flex-1 overflow-hidden"
            value="renderware"
          >
            <RenderWareSidebarContent />
          </TabsContent>

          <TabsContent className="mt-4 flex-1 overflow-hidden" value="debug">
            <DebugSidebarContent />
          </TabsContent>

          <TabsContent className="mt-4 flex-1 overflow-hidden" value="settings">
            <SettingsSidebarContent />
          </TabsContent>
        </Tabs>
      )}
    </div>
  );
});
