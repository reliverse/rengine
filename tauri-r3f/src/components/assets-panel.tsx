import { memo, useCallback, useState } from "react";
import { Folder } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { cn } from "~/lib/utils";
import { TextureLibrary } from "./texture-library";
import { RemoteAssetsTab } from "./remote-assets-tab";
import { ModelsTab } from "./assets/models/models-tab";
import { AudioTab } from "./assets/audio/audio-tab";
import { MaterialsTab } from "./assets/materials/materials-tab";
import { ScriptsTab } from "./assets/scripts/scripts-tab";
import { FontsTab } from "./assets/fonts/fonts-tab";
import { PrefabsTab } from "./assets/prefabs/prefabs-tab";
import { RenderWareTab } from "./assets/renderware/renderware-tab";
import { ASSET_TYPES } from "./assets/constants";
import type { AssetType } from "./assets/types";

interface AssetsPanelProps {
  className?: string;
}

export const AssetsPanel = memo(function AssetsPanel({
  className,
}: AssetsPanelProps) {
  const [activeTab, setActiveTab] = useState<AssetType>("textures");

  const handleTabChange = useCallback((value: string) => {
    setActiveTab(value as AssetType);
  }, []);

  return (
    <Card className={cn("flex h-full w-full flex-col", className)}>
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center gap-2">
          <Folder className="h-5 w-5" />
          Assets
        </CardTitle>
      </CardHeader>

      <CardContent className="flex flex-1 flex-col gap-4 p-4 pt-0">
        <Tabs
          className="flex h-full flex-col"
          onValueChange={handleTabChange}
          value={activeTab}
        >
          <TabsList className="mb-42 grid w-full grid-cols-3">
            {Object.entries(ASSET_TYPES).map(([key, config]) => {
              const Icon = config.icon;
              return (
                <TabsTrigger
                  className="flex flex-col gap-1 px-2 py-2 text-xs"
                  key={key}
                  value={key}
                >
                  <Icon className="h-4 w-4" />
                  <span className="hidden sm:inline">{config.label}</span>
                </TabsTrigger>
              );
            })}
          </TabsList>

          <TabsContent className="mt-0 flex-1" value="models">
            <ModelsTab />
          </TabsContent>

          <TabsContent className="mt-0 flex-1 overflow-hidden" value="textures">
            <TextureLibrary />
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="materials">
            <MaterialsTab />
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="audio">
            <AudioTab />
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="scripts">
            <ScriptsTab />
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="fonts">
            <FontsTab />
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="prefabs">
            <PrefabsTab />
          </TabsContent>

          <TabsContent className="mt-0 flex-1" value="renderware">
            <RenderWareTab />
          </TabsContent>

          <TabsContent className="mt-0 flex-1 overflow-hidden" value="remote">
            <RemoteAssetsTab />
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  );
});
