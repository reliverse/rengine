/**
 * Blueprint File List Component
 * Displays list of Blueprint files in the left sidebar
 */

import { FileText, Plus, FolderOpen, X, Code2 } from "lucide-react";
import { useCallback } from "react";
import { Button } from "~/components/ui/button";
import { ScrollArea } from "~/components/ui/scroll-area";
import { useBlueprintStore } from "~/stores/blueprint-store";
import { useSceneStore } from "~/stores/scene-store";
import { useToast } from "~/hooks/use-toast";
import {
  openBlueprintFile,
  createNewBlueprint,
} from "~/utils/blueprint/file-management";
import type { BlueprintGraph } from "~/utils/blueprint/graph/node-types";
import { cn } from "~/lib/utils";

export function BlueprintFileList() {
  const {
    currentGraph,
    currentFilePath,
    isModified,
    setCurrentGraph,
    setCurrentFilePath,
    markSaved,
  } = useBlueprintStore();
  const { blueprintFiles, addBlueprintFile, removeBlueprintFile } =
    useSceneStore();
  const { toast } = useToast();

  const handleOpenBlueprint = useCallback(async () => {
    try {
      const graph = await openBlueprintFile();
      if (graph) {
        setCurrentGraph(graph);
        setCurrentFilePath(null);
        markSaved();
        toast({
          title: "Blueprint opened",
          description: `"${graph.name}" has been loaded successfully.`,
          duration: 2000,
        });
      }
    } catch (error) {
      console.error("Error opening Blueprint:", error);
      toast({
        title: "Open failed",
        description:
          error instanceof Error ? error.message : "Failed to open Blueprint",
        variant: "destructive",
        duration: 4000,
      });
    }
  }, [setCurrentGraph, setCurrentFilePath, markSaved, toast]);

  const handleNewBlueprint = useCallback(() => {
    const newGraph = createNewBlueprint("pawn");
    const blueprintId = `blueprint_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

    setCurrentGraph(newGraph);
    setCurrentFilePath(null);
    markSaved();

    // Add to scene store
    addBlueprintFile({
      id: blueprintId,
      name: newGraph.name,
      filePath: null,
      isModified: false,
      lastSavedAt: new Date(),
    });

    toast({
      title: "New Blueprint created",
      description: "Started a new empty Blueprint",
      duration: 2000,
    });
  }, [setCurrentGraph, setCurrentFilePath, markSaved, addBlueprintFile, toast]);

  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <div className="border-b p-2">
        <div className="flex items-center justify-between">
          <h3 className="font-semibold text-sm">Blueprints</h3>
          <div className="flex gap-1">
            <Button
              className="h-7 w-7 p-0"
              onClick={handleNewBlueprint}
              size="sm"
              title="New Blueprint"
              variant="ghost"
            >
              <Plus className="h-4 w-4" />
            </Button>
            <Button
              className="h-7 w-7 p-0"
              onClick={handleOpenBlueprint}
              size="sm"
              title="Open Blueprint"
              variant="ghost"
            >
              <FolderOpen className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </div>

      {/* File List */}
      <ScrollArea className="flex-1">
        <div className="space-y-1 p-2">
          {blueprintFiles.length > 0 ? (
            blueprintFiles.map(
              (blueprint: {
                id: string;
                name: string;
                filePath: string | null;
                isModified: boolean;
                lastSavedAt?: Date;
              }) => (
                <div
                  className={cn(
                    "group flex items-center gap-2 rounded-md p-2 text-sm transition-colors",
                    "cursor-pointer hover:bg-muted",
                    currentGraph &&
                      (currentGraph.name === blueprint.name ||
                        (currentFilePath &&
                          currentFilePath === blueprint.filePath)) &&
                      "bg-muted"
                  )}
                  key={blueprint.id}
                  onClick={async () => {
                    try {
                      let graph: BlueprintGraph | null = null;

                      // If Blueprint has a file path, load it from disk
                      if (blueprint.filePath) {
                        const { openBlueprintFile } = await import(
                          "~/utils/blueprint/file-management"
                        );
                        graph = await openBlueprintFile(blueprint.filePath);
                      } else if (
                        // For unsaved Blueprints, we need to check if there's a cached version
                        // In a full implementation, we'd store the graph data in the scene store
                        // For now, if it's the current graph, use it
                        currentGraph &&
                        currentGraph.name === blueprint.name
                      ) {
                        graph = currentGraph;
                      } else {
                        // Create a new empty graph with the same name
                        const { createNewBlueprint } = await import(
                          "~/utils/blueprint/file-management"
                        );
                        graph = createNewBlueprint("pawn");
                        graph.name = blueprint.name;
                      }

                      if (graph) {
                        setCurrentGraph(graph);
                        setCurrentFilePath(blueprint.filePath);
                        markSaved();

                        // Update the Blueprint file's last accessed time
                        const { updateBlueprintFile } =
                          useSceneStore.getState();
                        updateBlueprintFile(blueprint.id, {
                          lastSavedAt: new Date(),
                        });

                        toast({
                          title: "Blueprint loaded",
                          description: `"${graph.name}" has been loaded.`,
                          duration: 2000,
                        });
                      }
                    } catch (error) {
                      console.error("Error loading Blueprint:", error);
                      toast({
                        title: "Load failed",
                        description:
                          error instanceof Error
                            ? error.message
                            : "Failed to load Blueprint",
                        variant: "destructive",
                        duration: 4000,
                      });
                    }
                  }}
                  onKeyDown={(e) => {
                    if (e.key === "Enter" || e.key === " ") {
                      e.preventDefault();
                      // Trigger the onClick handler
                      (e.currentTarget as HTMLDivElement).click();
                    }
                  }}
                  role="button"
                  tabIndex={0}
                >
                  <FileText className="h-4 w-4 flex-shrink-0" />
                  <div className="min-w-0 flex-1">
                    <div className="truncate font-medium">{blueprint.name}</div>
                    <div className="text-muted-foreground text-xs">
                      {blueprint.filePath || "Unsaved"}
                      {blueprint.isModified && " • Modified"}
                    </div>
                  </div>
                  <Button
                    className="h-6 w-6 p-0 opacity-0 group-hover:opacity-100"
                    onClick={(e) => {
                      e.stopPropagation();
                      removeBlueprintFile(blueprint.id);
                      // If this was the current Blueprint, clear it
                      if (
                        currentGraph &&
                        currentGraph.name === blueprint.name
                      ) {
                        setCurrentGraph(null);
                        setCurrentFilePath(null);
                      }
                    }}
                    size="sm"
                    variant="ghost"
                  >
                    <X className="h-3 w-3" />
                  </Button>
                </div>
              )
            )
          ) : currentGraph ? (
            <div
              className={cn(
                "flex items-center gap-2 rounded-md p-2 text-sm transition-colors",
                "cursor-pointer hover:bg-muted",
                "bg-muted"
              )}
            >
              <FileText className="h-4 w-4 flex-shrink-0" />
              <div className="min-w-0 flex-1">
                <div className="truncate font-medium">{currentGraph.name}</div>
                <div className="text-muted-foreground text-xs">
                  {currentFilePath || "Unsaved"}
                  {isModified && " • Modified"}
                </div>
              </div>
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center py-8 text-center text-muted-foreground">
              <Code2 className="mb-2 h-8 w-8 opacity-50" />
              <p className="text-sm">No Blueprint open</p>
              <p className="text-muted-foreground text-xs">
                Create a new Blueprint or open an existing one
              </p>
            </div>
          )}
        </div>
      </ScrollArea>
    </div>
  );
}
