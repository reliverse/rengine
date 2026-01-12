/**
 * TXD Texture Editor Component
 * Multi-TXD tabs with texture preview and management
 */

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import {
  AlertCircle,
  CheckSquare,
  Download,
  Eye,
  EyeOff,
  Image as ImageIcon,
  Info,
  Loader2,
  Palette,
  Search,
  Square,
  X,
} from "lucide-react";
import { useCallback, useState } from "react";
import { Alert, AlertDescription } from "~/components/ui/alert";
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { Separator } from "~/components/ui/separator";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";

// Import TXD types
import {
  type TextureInfo,
  type TexturePreview,
  type TxdArchive,
  type TxdArchiveTab,
  type TxdFilterOptions,
  type TxdFilterType,
  TxdFilterTypeValues,
  TxdSortDirectionValues,
  type TxdSortField,
  TxdSortFieldValues,
} from "~/types/txd";

interface TxdEditorProps {
  className?: string;
}

// Default filter options
const DEFAULT_FILTER_OPTIONS = {
  filter_type: TxdFilterTypeValues.ALL,
  search_text: "",
  sort_field: TxdSortFieldValues.NAME,
  sort_direction: TxdSortDirectionValues.ASC,
};

export function TxdEditor({ className }: TxdEditorProps) {
  // State for multi-TXD tabs
  const [tabs, setTabs] = useState<TxdArchiveTab[]>([]);
  const [activeTabId, setActiveTabId] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | undefined>();

  // Preview state
  const [previewLoading, setPreviewLoading] = useState(false);
  const [currentPreview, setCurrentPreview] = useState<
    TexturePreview | undefined
  >();

  // Load TXD archive from file
  const loadTxdArchive = useCallback(async () => {
    try {
      setIsLoading(true);
      setErrorMessage(undefined);

      const filePath = await open({
        filters: [
          {
            name: "TXD Texture Archive",
            extensions: ["txd"],
          },
        ],
      });

      if (!filePath) return;

      const archive: TxdArchive = await invoke("load_txd_file", {
        path: filePath,
      });

      // Create new tab
      const tabId = `txd_tab_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      const tabName =
        filePath.split("/").pop() ||
        filePath.split("\\").pop() ||
        "Unknown.txd";

      const newTab: TxdArchiveTab = {
        id: tabId,
        archive,
        name: tabName,
        is_modified: false,
        filter_options: { ...DEFAULT_FILTER_OPTIONS },
        selected_textures: [],
      };

      setTabs((prev) => [...prev, newTab]);
      setActiveTabId(tabId);
    } catch (error) {
      console.error("Failed to load TXD archive:", error);
      setErrorMessage(
        error instanceof Error ? error.message : "Failed to load TXD archive"
      );
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Close tab
  const closeTab = useCallback(
    (tabId: string) => {
      setTabs((prev) => prev.filter((tab) => tab.id !== tabId));
      if (activeTabId === tabId) {
        const remainingTabs = tabs.filter((tab) => tab.id !== tabId);
        setActiveTabId(remainingTabs.length > 0 ? remainingTabs[0].id : null);
      }
    },
    [activeTabId, tabs]
  );

  // Get filtered textures for a tab
  const getFilteredTextures = useCallback(
    (tab: TxdArchiveTab): TextureInfo[] => {
      let textures = [...tab.archive.textures];

      // Apply type filter
      if (tab.filter_options.filter_type !== TxdFilterTypeValues.ALL) {
        switch (tab.filter_options.filter_type) {
          case TxdFilterTypeValues.COMPRESSED:
            textures = textures.filter((t) => t.format === "Compressed");
            break;
          case TxdFilterTypeValues.PALETTE:
            textures = textures.filter(
              (t) => t.format === "Palette4" || t.format === "Palette8"
            );
            break;
          case TxdFilterTypeValues.RGBA:
            textures = textures.filter(
              (t) =>
                t.format.startsWith("RGBA") ||
                t.format === "Luminance8" ||
                t.format === "LuminanceAlpha8"
            );
            break;
          case TxdFilterTypeValues.OTHER:
            textures = textures.filter(
              (t) =>
                !(
                  ["Compressed", "Palette4", "Palette8"].includes(t.format) ||
                  t.format.startsWith("RGBA") ||
                  t.format === "Luminance8" ||
                  t.format === "LuminanceAlpha8"
                )
            );
            break;
        }
      }

      // Apply search filter
      if (tab.filter_options.search_text.trim()) {
        const searchLower = tab.filter_options.search_text.toLowerCase();
        textures = textures.filter((texture) =>
          texture.name.toLowerCase().includes(searchLower)
        );
      }

      // Apply dimension filters
      const { min_width, max_width, min_height, max_height } =
        tab.filter_options;
      if (min_width !== undefined) {
        textures = textures.filter((t) => t.width >= min_width);
      }
      if (max_width !== undefined) {
        textures = textures.filter((t) => t.width <= max_width);
      }
      if (min_height !== undefined) {
        textures = textures.filter((t) => t.height >= min_height);
      }
      if (max_height !== undefined) {
        textures = textures.filter((t) => t.height <= max_height);
      }

      // Apply format filter
      if (
        tab.filter_options.format_filter &&
        tab.filter_options.format_filter.length > 0
      ) {
        textures = textures.filter((t) =>
          tab.filter_options.format_filter?.includes(t.format)
        );
      }

      // Apply sorting
      textures.sort((a, b) => {
        let aValue: any, bValue: any;

        switch (tab.filter_options.sort_field) {
          case TxdSortFieldValues.NAME:
            aValue = a.name.toLowerCase();
            bValue = b.name.toLowerCase();
            break;
          case TxdSortFieldValues.SIZE:
            aValue = a.data_size;
            bValue = b.data_size;
            break;
          case TxdSortFieldValues.WIDTH:
            aValue = a.width;
            bValue = b.width;
            break;
          case TxdSortFieldValues.HEIGHT:
            aValue = a.height;
            bValue = b.height;
            break;
          case TxdSortFieldValues.FORMAT:
            aValue = a.format;
            bValue = b.format;
            break;
          default:
            return 0;
        }

        if (aValue < bValue)
          return tab.filter_options.sort_direction ===
            TxdSortDirectionValues.ASC
            ? -1
            : 1;
        if (aValue > bValue)
          return tab.filter_options.sort_direction ===
            TxdSortDirectionValues.ASC
            ? 1
            : -1;
        return 0;
      });

      return textures;
    },
    []
  );

  // Update tab filter options
  const updateTabFilterOptions = useCallback(
    (tabId: string, options: Partial<TxdFilterOptions>) => {
      setTabs((prev) =>
        prev.map((tab) =>
          tab.id === tabId
            ? { ...tab, filter_options: { ...tab.filter_options, ...options } }
            : tab
        )
      );
    },
    []
  );

  // Toggle texture selection
  const toggleTextureSelection = useCallback(
    (tabId: string, textureName: string) => {
      setTabs((prev) =>
        prev.map((tab) =>
          tab.id === tabId
            ? {
                ...tab,
                selected_textures: tab.selected_textures.includes(textureName)
                  ? tab.selected_textures.filter((name) => name !== textureName)
                  : [...tab.selected_textures, textureName],
              }
            : tab
        )
      );
    },
    []
  );

  // Select all textures in current tab
  const selectAllTextures = useCallback(() => {
    if (!activeTabId) return;

    const tab = tabs.find((t) => t.id === activeTabId);
    if (!tab) return;

    const filteredTextures = getFilteredTextures(tab);
    const allSelected = filteredTextures.every((texture) =>
      tab.selected_textures.includes(texture.name)
    );

    if (allSelected) {
      // Deselect all
      setTabs((prev) =>
        prev.map((t) =>
          t.id === activeTabId ? { ...t, selected_textures: [] } : t
        )
      );
    } else {
      // Select all filtered textures
      setTabs((prev) =>
        prev.map((t) =>
          t.id === activeTabId
            ? {
                ...t,
                selected_textures: filteredTextures.map(
                  (texture) => texture.name
                ),
              }
            : t
        )
      );
    }
  }, [activeTabId, tabs, getFilteredTextures]);

  // Preview texture
  const previewTexture = useCallback(
    (textureName: string) => {
      if (!activeTabId) return;

      const tab = tabs.find((t) => t.id === activeTabId);
      if (!tab) return;

      setPreviewLoading(true);
      try {
        // For now, just show texture info since PNG export is not implemented
        const textureInfo = tab.archive.textures.find(
          (t) => t.name === textureName
        );
        if (textureInfo) {
          setCurrentPreview({
            texture_name: textureName,
            image_data: "", // Placeholder - would need image processing
            width: textureInfo.width,
            height: textureInfo.height,
            format: textureInfo.format,
            mipmap_levels: textureInfo.mipmap_count,
          });
        }
      } catch (error) {
        console.error("Failed to preview texture:", error);
      } finally {
        setPreviewLoading(false);
      }
    },
    [activeTabId, tabs]
  );

  // Export texture
  const exportTexture = useCallback(
    async (textureName: string) => {
      if (!activeTabId) return;

      const tab = tabs.find((t) => t.id === activeTabId);
      if (!tab) return;

      try {
        const outputPath = await open({
          title: `Export ${textureName} as...`,
          defaultPath: `${textureName}.png`,
        });

        if (!outputPath) return;

        await invoke("export_texture_to_png", {
          archivePath: tab.archive.file_path,
          textureName,
          outputPath,
        });

        console.log(`Successfully exported texture: ${textureName}`);
      } catch (error) {
        console.error(`Failed to export texture ${textureName}:`, error);
        setErrorMessage(
          error instanceof Error
            ? error.message
            : `Failed to export texture ${textureName}`
        );
      }
    },
    [activeTabId, tabs]
  );

  // Get format badge color
  const getFormatBadgeColor = (format: string) => {
    switch (format.toLowerCase()) {
      case "compressed":
        return "bg-purple-100 text-purple-800";
      case "rgba32":
        return "bg-blue-100 text-blue-800";
      case "rgba16":
        return "bg-green-100 text-green-800";
      case "palette4":
      case "palette8":
        return "bg-yellow-100 text-yellow-800";
      case "luminance8":
      case "luminancealpha8":
        return "bg-gray-100 text-gray-800";
      default:
        return "bg-red-100 text-red-800";
    }
  };

  const activeTab = tabs.find((tab) => tab.id === activeTabId);
  const filteredTextures = activeTab ? getFilteredTextures(activeTab) : [];

  return (
    <div className={`flex h-full flex-col ${className}`}>
      {/* Toolbar */}
      <div className="flex items-center gap-2 border-b p-4">
        <Button
          disabled={isLoading}
          onClick={loadTxdArchive}
          size="sm"
          variant="outline"
        >
          {isLoading ? (
            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
          ) : (
            <Palette className="mr-2 h-4 w-4" />
          )}
          Open TXD
        </Button>

        <Separator className="h-6" orientation="vertical" />

        <Button
          disabled={!activeTab || activeTab.selected_textures.length === 0}
          size="sm"
          variant="outline"
        >
          <Download className="mr-2 h-4 w-4" />
          Export Selected ({activeTab?.selected_textures.length || 0})
        </Button>

        <div className="ml-auto flex items-center gap-2">
          <Button
            disabled={!activeTab}
            onClick={selectAllTextures}
            size="sm"
            variant="ghost"
          >
            {activeTab &&
            filteredTextures.length > 0 &&
            filteredTextures.every((texture) =>
              activeTab.selected_textures.includes(texture.name)
            ) ? (
              <CheckSquare className="h-4 w-4" />
            ) : (
              <Square className="h-4 w-4" />
            )}
          </Button>
        </div>
      </div>

      {/* Error message */}
      {errorMessage && (
        <Alert className="m-4">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{errorMessage}</AlertDescription>
        </Alert>
      )}

      {/* Main Content */}
      <div className="flex flex-1">
        {/* Texture List Panel */}
        <div className="flex flex-1 flex-col border-r">
          {/* Tabs */}
          {tabs.length > 0 && (
            <Tabs
              className="flex flex-1 flex-col"
              onValueChange={setActiveTabId}
              value={activeTabId || ""}
            >
              <TabsList className="grid w-full grid-cols-[repeat(auto-fit,minmax(120px,1fr))] rounded-none border-b">
                {tabs.map((tab) => (
                  <TabsTrigger
                    className="group relative"
                    key={tab.id}
                    value={tab.id}
                  >
                    <Palette className="mr-1 h-3 w-3" />
                    <span className="max-w-[80px] truncate">{tab.name}</span>
                    {tab.is_modified && (
                      <div className="absolute -top-1 -right-1 h-2 w-2 rounded-full bg-orange-500" />
                    )}
                    <Button
                      className="absolute -top-1 -right-1 h-4 w-4 p-0 opacity-0 hover:bg-destructive hover:text-destructive-foreground group-hover:opacity-100"
                      onClick={(e) => {
                        e.stopPropagation();
                        closeTab(tab.id);
                      }}
                      size="sm"
                      variant="ghost"
                    >
                      <X className="h-3 w-3" />
                    </Button>
                  </TabsTrigger>
                ))}
              </TabsList>

              {/* Tab Content */}
              {tabs.map((tab) => {
                const filtered = getFilteredTextures(tab);

                return (
                  <TabsContent
                    className="m-0 flex flex-1 flex-col"
                    key={tab.id}
                    value={tab.id}
                  >
                    {/* Filters */}
                    <div className="flex items-center gap-2 border-b bg-muted/50 p-4">
                      <Search className="h-4 w-4" />
                      <Input
                        className="max-w-xs flex-1"
                        onChange={(e) =>
                          updateTabFilterOptions(tab.id, {
                            search_text: e.target.value,
                          })
                        }
                        placeholder="Search textures..."
                        value={tab.filter_options.search_text}
                      />

                      <Select
                        onValueChange={(value) =>
                          value &&
                          updateTabFilterOptions(tab.id, {
                            filter_type: value as TxdFilterType,
                          })
                        }
                        value={tab.filter_options.filter_type}
                      >
                        <SelectTrigger className="w-32">
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value={TxdFilterTypeValues.ALL}>
                            All Formats
                          </SelectItem>
                          <SelectItem value={TxdFilterTypeValues.COMPRESSED}>
                            Compressed
                          </SelectItem>
                          <SelectItem value={TxdFilterTypeValues.PALETTE}>
                            Palette
                          </SelectItem>
                          <SelectItem value={TxdFilterTypeValues.RGBA}>
                            RGBA
                          </SelectItem>
                          <SelectItem value={TxdFilterTypeValues.OTHER}>
                            Other
                          </SelectItem>
                        </SelectContent>
                      </Select>

                      <Select
                        onValueChange={(value) =>
                          value &&
                          updateTabFilterOptions(tab.id, {
                            sort_field: value as TxdSortField,
                          })
                        }
                        value={tab.filter_options.sort_field}
                      >
                        <SelectTrigger className="w-32">
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value={TxdSortFieldValues.NAME}>
                            Name
                          </SelectItem>
                          <SelectItem value={TxdSortFieldValues.SIZE}>
                            Size
                          </SelectItem>
                          <SelectItem value={TxdSortFieldValues.WIDTH}>
                            Width
                          </SelectItem>
                          <SelectItem value={TxdSortFieldValues.HEIGHT}>
                            Height
                          </SelectItem>
                          <SelectItem value={TxdSortFieldValues.FORMAT}>
                            Format
                          </SelectItem>
                        </SelectContent>
                      </Select>

                      <Button
                        onClick={() =>
                          updateTabFilterOptions(tab.id, {
                            sort_direction:
                              tab.filter_options.sort_direction ===
                              TxdSortDirectionValues.ASC
                                ? TxdSortDirectionValues.DESC
                                : TxdSortDirectionValues.ASC,
                          })
                        }
                        size="sm"
                        variant="ghost"
                      >
                        {tab.filter_options.sort_direction ===
                        TxdSortDirectionValues.ASC
                          ? "↑"
                          : "↓"}
                      </Button>
                    </div>

                    {/* Archive Info */}
                    <div className="border-b bg-muted/30 px-4 py-2">
                      <div className="flex items-center gap-4 text-sm">
                        <span>
                          <strong>Textures:</strong>{" "}
                          {tab.archive.total_textures.toLocaleString()}
                        </span>
                        <span>
                          <strong>Path:</strong>{" "}
                          {tab.archive.file_path.split("/").pop() ||
                            tab.archive.file_path.split("\\").pop()}
                        </span>
                        {tab.archive.renderware_version && (
                          <span>
                            <strong>RW:</strong>{" "}
                            {tab.archive.renderware_version}
                          </span>
                        )}
                      </div>
                    </div>

                    {/* Texture List */}
                    <ScrollArea className="flex-1">
                      <div className="p-2">
                        {filtered.length === 0 ? (
                          <div className="py-8 text-center text-muted-foreground">
                            <Palette className="mx-auto mb-4 h-12 w-12 opacity-50" />
                            <p>No textures found</p>
                            {tab.filter_options.search_text && (
                              <p className="text-sm">
                                Try adjusting your search or filters
                              </p>
                            )}
                          </div>
                        ) : (
                          <div className="space-y-1">
                            {filtered.map((texture) => (
                              <div
                                aria-label={`Select texture ${texture.name}`}
                                className={`flex cursor-pointer items-center gap-3 rounded p-2 hover:bg-muted/50 ${
                                  tab.selected_textures.includes(texture.name)
                                    ? "bg-muted"
                                    : ""
                                }`}
                                key={texture.name}
                                onClick={() =>
                                  toggleTextureSelection(tab.id, texture.name)
                                }
                                onKeyDown={(e) => {
                                  if (e.key === "Enter" || e.key === " ") {
                                    e.preventDefault();
                                    toggleTextureSelection(
                                      tab.id,
                                      texture.name
                                    );
                                  }
                                }}
                                role="button"
                                tabIndex={0}
                              >
                                <input
                                  checked={tab.selected_textures.includes(
                                    texture.name
                                  )}
                                  className="h-4 w-4"
                                  readOnly
                                  type="checkbox" // Handled by parent click
                                />

                                <div className="flex h-8 w-8 items-center justify-center rounded bg-muted">
                                  <ImageIcon className="h-4 w-4" />
                                </div>

                                <div className="min-w-0 flex-1">
                                  <div className="truncate font-medium">
                                    {texture.name}
                                  </div>
                                  <div className="text-muted-foreground text-xs">
                                    {texture.width}×{texture.height} •{" "}
                                    {(texture.data_size / 1024).toFixed(1)} KB
                                  </div>
                                </div>

                                <Badge
                                  className={`text-xs ${getFormatBadgeColor(texture.format)}`}
                                  variant="secondary"
                                >
                                  {texture.format}
                                </Badge>

                                <div className="flex gap-1">
                                  <Button
                                    disabled={previewLoading}
                                    onClick={(e) => {
                                      e.stopPropagation();
                                      previewTexture(texture.name);
                                    }}
                                    size="sm"
                                    variant="ghost"
                                  >
                                    <Eye className="h-3 w-3" />
                                  </Button>
                                  <Button
                                    onClick={(e) => {
                                      e.stopPropagation();
                                      exportTexture(texture.name);
                                    }}
                                    size="sm"
                                    variant="ghost"
                                  >
                                    <Download className="h-3 w-3" />
                                  </Button>
                                </div>
                              </div>
                            ))}
                          </div>
                        )}
                      </div>
                    </ScrollArea>
                  </TabsContent>
                );
              })}
            </Tabs>
          )}
        </div>

        {/* Preview Panel */}
        <div className="w-80 border-l bg-muted/30">
          <div className="border-b p-4">
            <h3 className="font-semibold">Texture Preview</h3>
          </div>

          <div className="p-4">
            {previewLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="h-6 w-6 animate-spin" />
                <span className="ml-2">Loading preview...</span>
              </div>
            ) : currentPreview ? (
              <div className="space-y-4">
                <div className="text-center">
                  <div className="mb-2 flex aspect-square w-full items-center justify-center rounded bg-muted">
                    <ImageIcon className="h-12 w-12 text-muted-foreground" />
                  </div>
                  <h4 className="font-medium">{currentPreview.texture_name}</h4>
                </div>

                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Dimensions:</span>
                    <span>
                      {currentPreview.width} × {currentPreview.height}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Format:</span>
                    <Badge
                      className={`text-xs ${getFormatBadgeColor(currentPreview.format)}`}
                      variant="secondary"
                    >
                      {currentPreview.format}
                    </Badge>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Mipmaps:</span>
                    <span>{currentPreview.mipmap_levels}</span>
                  </div>
                </div>

                <Alert>
                  <Info className="h-4 w-4" />
                  <AlertDescription className="text-xs">
                    PNG export functionality requires additional image
                    processing libraries.
                  </AlertDescription>
                </Alert>
              </div>
            ) : (
              <div className="py-8 text-center text-muted-foreground">
                <EyeOff className="mx-auto mb-4 h-12 w-12 opacity-50" />
                <p>Select a texture to preview</p>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Empty state */}
      {tabs.length === 0 && !isLoading && (
        <div className="flex flex-1 items-center justify-center">
          <Card className="w-96">
            <CardHeader className="text-center">
              <Palette className="mx-auto mb-4 h-12 w-12 text-muted-foreground" />
              <CardTitle>TXD Texture Editor</CardTitle>
              <CardDescription>
                Open RenderWare TXD files to view and manage textures
              </CardDescription>
            </CardHeader>
            <CardContent className="text-center">
              <Button className="w-full" onClick={loadTxdArchive}>
                <Palette className="mr-2 h-4 w-4" />
                Open TXD Archive
              </Button>
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}
