import { LightPropertyPanel } from "~/components/light-property-panel";
import { PropertyPanel } from "~/components/property-panel";
import { useSelectedLight } from "~/stores/scene-store";
import { LightingPanel } from "./lighting-panel";
import { SceneHierarchy } from "./scene-hierarchy";

export type SidebarContext = "scene" | "tools" | "lighting" | "settings";

interface UnifiedSidebarProps {
  context: SidebarContext;
  className?: string;
}

function SceneSidebarContent() {
  return <SceneHierarchy />;
}

function ToolsSidebarContent() {
  const selectedLight = useSelectedLight();

  // If a light is selected, show the light property panel
  // Otherwise show the object property panel
  const showLightPanel = selectedLight !== null;

  return (
    <div className="h-full">
      {showLightPanel ? <LightPropertyPanel /> : <PropertyPanel />}
    </div>
  );
}

function LightingSidebarContent() {
  return <LightingPanel />;
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

export function UnifiedSidebar({ context, className }: UnifiedSidebarProps) {
  const renderSidebarContent = () => {
    switch (context) {
      case "scene":
        return <SceneSidebarContent />;
      case "tools":
        return <ToolsSidebarContent />;
      case "lighting":
        return <LightingSidebarContent />;
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
      {renderSidebarContent()}
    </div>
  );
}
