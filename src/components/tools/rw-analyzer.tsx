import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import {
  AlertTriangle,
  ChevronDown,
  ChevronRight,
  Download,
  FileText,
  FolderOpen,
  Info,
  Upload,
  X,
} from "lucide-react";
import type React from "react";
import { useCallback, useMemo, useState } from "react";
import { toast } from "sonner";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from "~/components/ui/context-menu";
import { ScrollArea } from "~/components/ui/scroll-area";
import { Separator } from "~/components/ui/separator";
import type {
  ChunkNode,
  ChunkProperties,
  ChunkTreeNode,
  RwAnalysis,
  RwAnalyzerState,
} from "~/types/rw-analyzer";

export function RwAnalyzer() {
  const [state, setState] = useState<RwAnalyzerState>({
    currentFile: null,
    currentAnalysis: null,
    isLoading: false,
    selectedChunkPath: null,
    expandedChunks: new Set(),
  });

  const chunkTree = useMemo((): ChunkTreeNode[] => {
    if (!state.currentAnalysis) return [];

    const buildTree = (
      node: ChunkNode,
      path: number[] = [],
      level = 0
    ): ChunkTreeNode => {
      const nodePath = path.join("-");
      const isExpanded = state.expandedChunks.has(nodePath);
      const isSelected = state.selectedChunkPath?.join("-") === nodePath;

      return {
        id: nodePath,
        name: node.display_name,
        header: node.header,
        children: node.children.map((child, index) =>
          buildTree(child, [...path, index], level + 1)
        ),
        level,
        isExpanded,
        isSelected,
        isCorrupt: node.is_corrupt,
        hasChildren: node.children.length > 0,
      };
    };

    return [buildTree(state.currentAnalysis.root_chunk)];
  }, [state.currentAnalysis, state.expandedChunks, state.selectedChunkPath]);

  const selectedChunk = useMemo((): ChunkNode | null => {
    if (!(state.currentAnalysis && state.selectedChunkPath)) return null;

    let current: ChunkNode = state.currentAnalysis.root_chunk;
    for (const index of state.selectedChunkPath.slice(1)) {
      // Skip root
      if (index < current.children.length) {
        current = current.children[index];
      } else {
        return null;
      }
    }
    return current;
  }, [state.currentAnalysis, state.selectedChunkPath]);

  const chunkProperties = useMemo((): ChunkProperties | null => {
    if (!selectedChunk) return null;

    const header = selectedChunk.header;

    return {
      chunkType: selectedChunk.display_name.split(":")[0].trim(), // Remove asset name suffix
      rawTypeId: `0x${header.chunk_type.toString(16).toUpperCase().padStart(8, "0")}`,
      payloadSize: `${header.chunk_size.toLocaleString()} bytes`,
      rwVersion: getRwVersionDisplay(header.rw_version),
      rawVersion: `0x${header.rw_version.toString(16).toUpperCase().padStart(8, "0")}`,
      fileOffset: `0x${header.offset.toString(16).toUpperCase().padStart(8, "0")} (${header.offset.toLocaleString()})`,
      endOfChunk: `0x${(header.offset + 12 + header.chunk_size).toString(16).toUpperCase().padStart(8, "0")}`,
    };
  }, [selectedChunk]);

  const handleOpenFile = useCallback(async () => {
    try {
      const filePath = (await open({
        multiple: false,
        filters: [
          {
            name: "RenderWare Files",
            extensions: ["dff", "txd", "col", "anm", "bsp"],
          },
          {
            name: "All Files",
            extensions: ["*"],
          },
        ],
      })) as string | null;

      if (!filePath) return;

      setState((prev: RwAnalyzerState) => ({ ...prev, isLoading: true }));

      const analysis: RwAnalysis = await invoke("analyze_rw_file", {
        path: filePath,
      });

      setState((prev: RwAnalyzerState) => ({
        ...prev,
        currentFile: filePath,
        currentAnalysis: analysis,
        isLoading: false,
        selectedChunkPath: [0], // Select root
        expandedChunks: new Set(["0"]), // Expand root
      }));

      toast.success(
        `Analyzed ${analysis.format} file with ${analysis.total_chunks} chunks`
      );
    } catch (error) {
      console.error("Failed to analyze file:", error);
      toast.error(`Failed to analyze file: ${error}`);
      setState((prev: RwAnalyzerState) => ({ ...prev, isLoading: false }));
    }
  }, []);

  const handleCloseFile = useCallback(() => {
    setState({
      currentFile: null,
      currentAnalysis: null,
      isLoading: false,
      selectedChunkPath: null,
      expandedChunks: new Set(),
    });
  }, []);

  const handleChunkToggle = useCallback(
    (nodePath: string, hasChildren: boolean) => {
      setState((prev: RwAnalyzerState) => {
        const newExpanded = new Set(prev.expandedChunks);
        if (newExpanded.has(nodePath)) {
          newExpanded.delete(nodePath);
        } else if (hasChildren) {
          newExpanded.add(nodePath);
        }
        return { ...prev, expandedChunks: newExpanded };
      });
    },
    []
  );

  const handleChunkSelect = useCallback((nodePath: string) => {
    const pathIndices = nodePath.split("-").map(Number);
    setState((prev: RwAnalyzerState) => ({
      ...prev,
      selectedChunkPath: pathIndices,
    }));
  }, []);

  const handleExportChunk = useCallback(
    async (includeHeader: boolean) => {
      if (!(state.currentFile && selectedChunk)) return;

      try {
        const outputPath = (await open({
          multiple: false,
          mode: "save",
          defaultPath: `${selectedChunk.display_name.replace(/[^a-zA-Z0-9]/g, "_")}_${selectedChunk.header.offset.toString(16)}.bin`,
        })) as string | null;

        if (!outputPath) return;

        await invoke("export_rw_chunk", {
          filePath: state.currentFile,
          chunkOffset: selectedChunk.header.offset,
          outputPath,
          includeHeader,
        });

        toast.success(
          `Exported ${includeHeader ? "full chunk" : "payload"} to ${outputPath}`
        );
      } catch (error) {
        console.error("Failed to export chunk:", error);
        toast.error(`Failed to export chunk: ${error}`);
      }
    },
    [state.currentFile, selectedChunk]
  );

  const handleImportPayload = useCallback(async () => {
    if (!(state.currentFile && selectedChunk)) return;

    try {
      const payloadPath = (await open({
        multiple: false,
        filters: [
          {
            name: "Binary Files",
            extensions: ["bin"],
          },
          {
            name: "All Files",
            extensions: ["*"],
          },
        ],
      })) as string | null;

      if (!payloadPath) return;

      const outputPath = (await open({
        multiple: false,
        mode: "save",
        defaultPath: `${state.currentFile}_modified${getFileExtension(state.currentFile)}`,
      })) as string | null;

      if (!outputPath) return;

      await invoke("import_rw_chunk_payload", {
        filePath: state.currentFile,
        chunkOffset: selectedChunk.header.offset,
        payloadPath,
        outputPath,
      });

      toast.success(`Modified file saved as ${outputPath}`);
    } catch (error) {
      console.error("Failed to import payload:", error);
      toast.error(`Failed to import payload: ${error}`);
    }
  }, [state.currentFile, selectedChunk]);

  const renderChunkTree = useCallback(
    (nodes: ChunkTreeNode[]): React.ReactNode => {
      return nodes.map((node) => (
        <div key={node.id}>
          <ContextMenu>
            <ContextMenuTrigger>
              <div
                aria-label={`Select chunk ${node.name}`}
                className={`flex cursor-pointer items-center rounded-sm px-2 py-1 hover:bg-accent ${
                  node.isSelected ? "bg-accent" : ""
                }`}
                onClick={() => handleChunkSelect(node.id)}
                onKeyDown={(e) => {
                  if (e.key === "Enter" || e.key === " ") {
                    e.preventDefault();
                    handleChunkSelect(node.id);
                  }
                }}
                role="button"
                style={{ paddingLeft: `${node.level * 16 + 8}px` }}
                tabIndex={0}
              >
                {node.hasChildren ? (
                  <button
                    className="mr-1 flex h-4 w-4 items-center justify-center"
                    onClick={(e) => {
                      e.stopPropagation();
                      handleChunkToggle(node.id, node.hasChildren);
                    }}
                    type="button"
                  >
                    {node.isExpanded ? (
                      <ChevronDown className="h-3 w-3" />
                    ) : (
                      <ChevronRight className="h-3 w-3" />
                    )}
                  </button>
                ) : (
                  <div className="mr-1 h-4 w-4" />
                )}

                {node.isCorrupt && (
                  <AlertTriangle className="mr-2 h-4 w-4 text-destructive" />
                )}

                <span
                  className={`text-sm ${node.isCorrupt ? "text-destructive" : ""}`}
                >
                  {node.name}
                </span>

                <span className="ml-auto text-muted-foreground text-xs">
                  {node.header.chunk_size.toLocaleString()} bytes
                </span>
              </div>
            </ContextMenuTrigger>

            <ContextMenuContent>
              <ContextMenuItem onClick={() => handleExportChunk(true)}>
                <Download className="mr-2 h-4 w-4" />
                Export Full Chunk...
              </ContextMenuItem>
              <ContextMenuItem onClick={() => handleExportChunk(false)}>
                <Download className="mr-2 h-4 w-4" />
                Export Payload Only...
              </ContextMenuItem>
              <ContextMenuSeparator />
              <ContextMenuItem onClick={handleImportPayload}>
                <Upload className="mr-2 h-4 w-4" />
                Import and Replace Payload...
              </ContextMenuItem>
              <ContextMenuSeparator />
              <ContextMenuItem onClick={() => handleChunkSelect(node.id)}>
                <Info className="mr-2 h-4 w-4" />
                Show Properties
              </ContextMenuItem>
            </ContextMenuContent>
          </ContextMenu>

          {node.isExpanded &&
            node.children.length > 0 &&
            renderChunkTree(node.children)}
        </div>
      ));
    },
    [
      handleChunkToggle,
      handleChunkSelect,
      handleExportChunk,
      handleImportPayload,
    ]
  );

  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <div className="flex items-center justify-between border-b p-4">
        <div className="flex items-center gap-2">
          <FileText className="h-5 w-5" />
          <h2 className="font-semibold text-lg">RW File Analyzer</h2>
        </div>

        <div className="flex items-center gap-2">
          <Button disabled={state.isLoading} onClick={handleOpenFile} size="sm">
            <FolderOpen className="mr-2 h-4 w-4" />
            Open RW File...
          </Button>

          {state.currentFile && (
            <Button onClick={handleCloseFile} size="sm" variant="outline">
              <X className="mr-2 h-4 w-4" />
              Close
            </Button>
          )}
        </div>
      </div>

      {/* File Info */}
      {state.currentAnalysis && (
        <div className="border-b bg-muted/50 p-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div>
                <p className="font-medium">{state.currentAnalysis.format}</p>
                <p className="text-muted-foreground text-sm">
                  {state.currentAnalysis.format_description}
                </p>
              </div>

              <Separator className="h-8" orientation="vertical" />

              <div className="flex items-center gap-4 text-sm">
                <div>
                  <span className="text-muted-foreground">Size:</span>
                  <span className="ml-1 font-mono">
                    {state.currentAnalysis.file_size.toLocaleString()} bytes
                  </span>
                </div>
                <div>
                  <span className="text-muted-foreground">Chunks:</span>
                  <span className="ml-1 font-mono">
                    {state.currentAnalysis.total_chunks}
                  </span>
                </div>
                <div>
                  <span className="text-muted-foreground">Depth:</span>
                  <span className="ml-1 font-mono">
                    {state.currentAnalysis.max_depth}
                  </span>
                </div>
                <div>
                  <span className="text-muted-foreground">RW Version:</span>
                  <span className="ml-1 font-mono">
                    0x
                    {state.currentAnalysis.rw_version
                      .toString(16)
                      .toUpperCase()}
                  </span>
                </div>
              </div>
            </div>

            <div className="text-muted-foreground text-xs">
              Analysis time: {state.currentAnalysis.analysis_time_ms}ms
            </div>
          </div>

          {state.currentAnalysis.corruption_warnings.length > 0 && (
            <div className="mt-2">
              <Badge className="text-xs" variant="destructive">
                <AlertTriangle className="mr-1 h-3 w-3" />
                {state.currentAnalysis.corruption_warnings.length} corruption
                warning(s)
              </Badge>
            </div>
          )}
        </div>
      )}

      {/* Main Content */}
      <div className="flex flex-1 overflow-hidden">
        {/* Chunk Tree */}
        <div className="w-1/2 border-r">
          <div className="border-b p-2">
            <h3 className="font-medium">Chunk Structure</h3>
          </div>

          <ScrollArea className="h-full">
            {state.currentAnalysis ? (
              <div className="p-2">{renderChunkTree(chunkTree)}</div>
            ) : (
              <div className="flex h-full items-center justify-center text-muted-foreground">
                <div className="text-center">
                  <FileText className="mx-auto mb-4 h-12 w-12 opacity-50" />
                  <p>Open a RenderWare file to begin analysis</p>
                  <p className="mt-1 text-sm">
                    Supports DFF, TXD, COL, and other RW formats
                  </p>
                </div>
              </div>
            )}
          </ScrollArea>
        </div>

        {/* Properties Panel */}
        <div className="w-1/2">
          <div className="border-b p-2">
            <h3 className="font-medium">Properties</h3>
          </div>

          <ScrollArea className="h-full">
            {chunkProperties ? (
              <div className="space-y-4 p-4">
                <Card>
                  <CardHeader className="pb-3">
                    <CardTitle className="text-base">
                      Chunk Information
                    </CardTitle>
                  </CardHeader>
                  <CardContent className="space-y-3">
                    <div className="grid grid-cols-2 gap-4">
                      <div>
                        {/* biome-ignore lint/a11y/noLabelWithoutControl: This is just an informational display label */}
                        <label className="font-medium text-muted-foreground text-sm">
                          Chunk Type
                        </label>
                        <p className="font-mono text-sm">
                          {chunkProperties.chunkType}
                        </p>
                      </div>
                      <div>
                        {/* biome-ignore lint/a11y/noLabelWithoutControl: This is just an informational display label */}
                        <label className="font-medium text-muted-foreground text-sm">
                          Raw Type ID
                        </label>
                        <p className="font-mono text-sm">
                          {chunkProperties.rawTypeId}
                        </p>
                      </div>
                      <div>
                        {/* biome-ignore lint/a11y/noLabelWithoutControl: This is just an informational display label */}
                        <label className="font-medium text-muted-foreground text-sm">
                          Payload Size
                        </label>
                        <p className="font-mono text-sm">
                          {chunkProperties.payloadSize}
                        </p>
                      </div>
                      <div>
                        {/* biome-ignore lint/a11y/noLabelWithoutControl: This is just an informational display label */}
                        <label className="font-medium text-muted-foreground text-sm">
                          RW Version
                        </label>
                        <p className="font-mono text-sm">
                          {chunkProperties.rwVersion}
                        </p>
                      </div>
                      <div>
                        {/* biome-ignore lint/a11y/noLabelWithoutControl: This is just an informational display label */}
                        <label className="font-medium text-muted-foreground text-sm">
                          Raw Version
                        </label>
                        <p className="font-mono text-sm">
                          {chunkProperties.rawVersion}
                        </p>
                      </div>
                      <div>
                        {/* biome-ignore lint/a11y/noLabelWithoutControl: This is just an informational display label */}
                        <label className="font-medium text-muted-foreground text-sm">
                          File Offset
                        </label>
                        <p className="font-mono text-sm">
                          {chunkProperties.fileOffset}
                        </p>
                      </div>
                    </div>

                    <Separator />

                    <div>
                      {/* biome-ignore lint/a11y/noLabelWithoutControl: This is just an informational display label */}
                      <label className="font-medium text-muted-foreground text-sm">
                        End of Chunk
                      </label>
                      <p className="font-mono text-sm">
                        {chunkProperties.endOfChunk}
                      </p>
                    </div>
                  </CardContent>
                </Card>

                {selectedChunk?.is_corrupt &&
                  selectedChunk.corruption_reason && (
                    <Card className="border-destructive">
                      <CardHeader className="pb-3">
                        <CardTitle className="flex items-center text-base text-destructive">
                          <AlertTriangle className="mr-2 h-4 w-4" />
                          Corruption Warning
                        </CardTitle>
                      </CardHeader>
                      <CardContent>
                        <p className="text-destructive text-sm">
                          {selectedChunk.corruption_reason}
                        </p>
                      </CardContent>
                    </Card>
                  )}
              </div>
            ) : (
              <div className="flex h-full items-center justify-center text-muted-foreground">
                <div className="text-center">
                  <Info className="mx-auto mb-4 h-12 w-12 opacity-50" />
                  <p>Select a chunk to view its properties</p>
                </div>
              </div>
            )}
          </ScrollArea>
        </div>
      </div>
    </div>
  );
}

// Helper function to get RW version display string
function getRwVersionDisplay(version: number): string {
  // This would ideally use the version manager from Rust
  // For now, return a basic hex representation
  return `0x${version.toString(16).toUpperCase().padStart(8, "0")}`;
}

// Helper function to get file extension
function getFileExtension(filePath: string): string {
  const parts = filePath.split(".");
  return parts.length > 1 ? `.${parts.at(-1)}` : "";
}
