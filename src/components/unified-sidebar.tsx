import { LightPropertyPanel } from "~/components/light-property-panel";
import { MaterialPropertyPanel } from "~/components/material-property-panel";
import { PropertyPanel } from "~/components/property-panel";
import { useSelectedMaterial } from "~/stores/material-store";
import { useSelectedLight, useSelectedObject } from "~/stores/scene-store";
import { LightingPanel } from "./lighting-panel";
import { SceneHierarchy } from "./scene-hierarchy";

export type SidebarContext =
  | "scene"
  | "tools"
  | "lighting"
  | "materials"
  | "settings";

interface UnifiedSidebarProps {
  context: SidebarContext;
  className?: string;
}

function SceneSidebarContent() {
  return <SceneHierarchy />;
}

function ToolsSidebarContent() {
  const selectedLight = useSelectedLight();
  const selectedMaterial = useSelectedMaterial();
  const selectedObject = useSelectedObject();

  // Priority: Material > Light > Object
  let content: React.ReactNode;
  if (selectedMaterial) {
    content = <MaterialPropertyPanel />;
  } else if (selectedLight) {
    content = <LightPropertyPanel />;
  } else if (selectedObject) {
    content = <PropertyPanel />;
  } else {
    content = (
      <div className="p-4">
        <h3 className="mb-4 font-semibold text-muted-foreground text-sm">
          Object Properties
        </h3>
        <p className="text-muted-foreground text-xs">
          Select an object, light, or material to edit its properties
        </p>
      </div>
    );
  }

  return <div className="h-full">{content}</div>;
}

function LightingSidebarContent() {
  return <LightingPanel />;
}

function MaterialsSidebarContent() {
  return <MaterialPropertyPanel />;
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
      case "materials":
        return <MaterialsSidebarContent />;
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
