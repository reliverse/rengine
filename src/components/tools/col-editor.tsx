/**
 * COL Collision File Editor Component
 * View and inspect GTA collision files (COL1, COL2, COL3, COL4)
 */

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { FileText, FolderOpen, Loader2, Search } from "lucide-react";
import { useCallback, useState } from "react";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { Input } from "~/components/ui/input";
import { ScrollArea } from "~/components/ui/scroll-area";
import { Separator } from "~/components/ui/separator";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";

// Import COL types
import type { ColFile, ColModel, ColStatistics } from "~/types/col";
import { getColVersionDisplayName } from "~/types/col";

interface ColEditorTab {
  id: string;
  filePath: string;
  colFile: ColFile;
  statistics: ColStatistics;
}

export function ColEditor() {
  const [tabs, setTabs] = useState<ColEditorTab[]>([]);
  const [activeTab, setActiveTab] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");

  const openColFile = useCallback(async () => {
    try {
      setLoading(true);

      const filePath = await open({
        multiple: false,
        filters: [
          {
            name: "COL Files",
            extensions: ["col"],
          },
          {
            name: "All Files",
            extensions: ["*"],
          },
        ],
      });

      if (!filePath || Array.isArray(filePath)) {
        return;
      }

      // Load COL file
      const colFile: ColFile = await invoke("load_col_file", {
        path: filePath,
      });

      // Get statistics
      const statistics: ColStatistics = await invoke("get_col_file_info", {
        path: filePath,
      });

      const tabId = `col-${Date.now()}`;
      const newTab: ColEditorTab = {
        id: tabId,
        filePath,
        colFile,
        statistics,
      };

      setTabs((prev) => [...prev, newTab]);
      setActiveTab(tabId);
    } catch (error) {
      console.error("Failed to open COL file:", error);
    } finally {
      setLoading(false);
    }
  }, []);

  const closeTab = useCallback(
    (tabId: string) => {
      setTabs((prev) => prev.filter((tab) => tab.id !== tabId));
      if (activeTab === tabId) {
        const remainingTabs = tabs.filter((tab) => tab.id !== tabId);
        setActiveTab(remainingTabs.length > 0 ? remainingTabs[0].id : "");
      }
    },
    [activeTab, tabs]
  );

  const activeTabData = tabs.find((tab) => tab.id === activeTab);
  const filteredModels =
    activeTabData?.colFile.models.filter((model) =>
      model.model_name.toLowerCase().includes(searchQuery.toLowerCase())
    ) || [];

  return (
    <div className="flex h-full flex-col">
      {/* Toolbar */}
      <div className="flex items-center gap-2 border-b p-2">
        <Button
          disabled={loading}
          onClick={openColFile}
          size="sm"
          variant="outline"
        >
          {loading ? (
            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
          ) : (
            <FolderOpen className="mr-2 h-4 w-4" />
          )}
          Open COL File
        </Button>

        {tabs.length > 0 && (
          <>
            <Separator className="h-6" orientation="vertical" />
            <div className="relative max-w-sm flex-1">
              <Search className="absolute top-1/2 left-2 h-4 w-4 -translate-y-1/2 transform text-muted-foreground" />
              <Input
                className="pl-8"
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search models..."
                value={searchQuery}
              />
            </div>
          </>
        )}
      </div>

      {/* Tab bar */}
      {tabs.length > 0 && (
        <div className="flex items-center border-b bg-muted/50">
          <ScrollArea className="flex-1">
            <div className="flex">
              {tabs.map((tab) => (
                <Button
                  className="h-8 rounded-none border-r px-3"
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  size="sm"
                  variant={activeTab === tab.id ? "secondary" : "ghost"}
                >
                  <FileText className="mr-2 h-3 w-3" />
                  {tab.filePath.split("/").pop()}
                  <button
                    className="ml-2 rounded-sm p-0.5 hover:bg-destructive hover:text-destructive-foreground"
                    onClick={(e) => {
                      e.stopPropagation();
                      closeTab(tab.id);
                    }}
                    type="button"
                  >
                    ×
                  </button>
                </Button>
              ))}
            </div>
          </ScrollArea>
        </div>
      )}

      {/* Content */}
      <div className="flex-1 overflow-hidden">
        {activeTabData ? (
          <Tabs className="h-full" value="overview">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="overview">Overview</TabsTrigger>
              <TabsTrigger value="models">
                Models ({activeTabData.colFile.models.length})
              </TabsTrigger>
            </TabsList>

            <TabsContent className="mt-0 h-full" value="overview">
              <ScrollArea className="h-full">
                <div className="space-y-4 p-4">
                  {/* File Info */}
                  <Card>
                    <CardHeader>
                      <CardTitle className="flex items-center gap-2">
                        <FileText className="h-5 w-5" />
                        File Information
                      </CardTitle>
                    </CardHeader>
                    <CardContent className="space-y-2">
                      <div className="grid grid-cols-2 gap-4">
                        <div>
                          <div className="text-muted-foreground text-sm">
                            File Path
                          </div>
                          <div className="font-mono text-sm">
                            {activeTabData.filePath}
                          </div>
                        </div>
                        <div>
                          <div className="text-muted-foreground text-sm">
                            Version
                          </div>
                          <Badge variant="outline">
                            {getColVersionDisplayName(
                              activeTabData.colFile.version
                            )}
                          </Badge>
                        </div>
                      </div>
                    </CardContent>
                  </Card>

                  {/* Statistics */}
                  <Card>
                    <CardHeader>
                      <CardTitle>Statistics</CardTitle>
                      <CardDescription>Collision data summary</CardDescription>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-2 gap-4 md:grid-cols-4">
                        <div className="text-center">
                          <div className="font-bold text-2xl">
                            {activeTabData.statistics.model_count}
                          </div>
                          <div className="text-muted-foreground text-sm">
                            Models
                          </div>
                        </div>
                        <div className="text-center">
                          <div className="font-bold text-2xl">
                            {activeTabData.statistics.total_spheres}
                          </div>
                          <div className="text-muted-foreground text-sm">
                            Spheres
                          </div>
                        </div>
                        <div className="text-center">
                          <div className="font-bold text-2xl">
                            {activeTabData.statistics.total_boxes}
                          </div>
                          <div className="text-muted-foreground text-sm">
                            Boxes
                          </div>
                        </div>
                        <div className="text-center">
                          <div className="font-bold text-2xl">
                            {activeTabData.statistics.total_faces}
                          </div>
                          <div className="text-muted-foreground text-sm">
                            Faces
                          </div>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </div>
              </ScrollArea>
            </TabsContent>

            <TabsContent className="mt-0 h-full" value="models">
              <ScrollArea className="h-full">
                <div className="p-4">
                  <div className="grid gap-4">
                    {filteredModels.map((model) => (
                      <ColModelCard key={model.model_id} model={model} />
                    ))}
                  </div>
                  {filteredModels.length === 0 && (
                    <div className="py-8 text-center text-muted-foreground">
                      {searchQuery
                        ? "No models match your search."
                        : "No models found."}
                    </div>
                  )}
                </div>
              </ScrollArea>
            </TabsContent>
          </Tabs>
        ) : (
          <div className="flex h-full items-center justify-center">
            <div className="space-y-4 text-center">
              <FileText className="mx-auto h-16 w-16 text-muted-foreground" />
              <div>
                <h3 className="font-semibold text-lg">No COL file opened</h3>
                <p className="text-muted-foreground">
                  Open a GTA collision file to view its contents
                </p>
              </div>
              <Button disabled={loading} onClick={openColFile}>
                {loading ? (
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                ) : (
                  <FolderOpen className="mr-2 h-4 w-4" />
                )}
                Open COL File
              </Button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

interface ColModelCardProps {
  model: ColModel;
}

function ColModelCard({ model }: ColModelCardProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <span className="font-mono">Model {model.model_id}</span>
          <Badge variant="secondary">{model.model_name}</Badge>
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="grid grid-cols-2 gap-4 text-sm md:grid-cols-4">
          <div>
            <div className="text-muted-foreground">Spheres</div>
            <div className="font-semibold">{model.spheres.length}</div>
          </div>
          <div>
            <div className="text-muted-foreground">Boxes</div>
            <div className="font-semibold">{model.boxes.length}</div>
          </div>
          <div>
            <div className="text-muted-foreground">Vertices</div>
            <div className="font-semibold">{model.vertices.length}</div>
          </div>
          <div>
            <div className="text-muted-foreground">Faces</div>
            <div className="font-semibold">{model.faces.length}</div>
          </div>
        </div>

        {model.suspension_lines.length > 0 && (
          <div className="mt-2 text-sm">
            <span className="text-muted-foreground">Suspension Lines: </span>
            <span className="font-semibold">
              {model.suspension_lines.length}
            </span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
