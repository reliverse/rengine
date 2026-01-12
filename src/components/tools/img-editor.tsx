/**
 * IMG Archive Editor Component
 * Multi-archive tabs support for GTA IMG files
 */

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import {
  AlertCircle,
  Archive,
  CheckSquare,
  Download,
  FolderOpen,
  Loader2,
  Search,
  Square,
  Upload,
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
import { Checkbox } from "~/components/ui/checkbox";
import { Input } from "~/components/ui/input";
import { Progress } from "~/components/ui/progress";
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

// Import IMG types
import type {
  BatchExtractProgress,
  ImgArchive,
  ImgArchiveTab,
  ImgEntry,
  ImgFilterOptions,
  ImgFilterType,
  ImgOperation,
  ImgSortField,
  OperationResult,
} from "~/types/img";
import {
  ImgFilterTypeValues,
  ImgSortDirectionValues,
  ImgSortFieldValues,
} from "~/types/img";

// Default filter options
const DEFAULT_FILTER_OPTIONS = {
  filter_type: ImgFilterTypeValues.ALL,
  search_text: "",
  sort_field: ImgSortFieldValues.NAME,
  sort_direction: ImgSortDirectionValues.ASC,
};

// Regex for extracting game name from RW version strings
const GAME_NAME_REGEX = /\(([^)]+)\)/;

interface ImgEditorProps {
  className?: string;
}

export function ImgEditor({ className }: ImgEditorProps) {
  // State for multi-archive tabs
  const [tabs, setTabs] = useState<ImgArchiveTab[]>([]);
  const [activeTabId, setActiveTabId] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | undefined>();

  // Batch operations
  const [batchOperationInProgress, setBatchOperationInProgress] =
    useState(false);
  const [batchProgress, setBatchProgress] = useState<
    BatchExtractProgress | undefined
  >();

  // Available RW versions for filtering (will be populated from archives)
  const [availableRwVersions, setAvailableRwVersions] = useState<string[]>([]);

  // Load IMG archive from file
  const loadImgArchive = useCallback(async () => {
    try {
      setIsLoading(true);
      setErrorMessage(undefined);

      const filePath = await open({
        filters: [
          {
            name: "IMG Archive",
            extensions: ["img"],
          },
        ],
      });

      if (!filePath) return;

      const archive: ImgArchive = await invoke("load_img_archive", {
        path: filePath,
      });

      // Create new tab
      const tabId = `tab_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      const tabName =
        filePath.split("/").pop() ||
        filePath.split("\\").pop() ||
        "Unknown.img";

      const newTab: ImgArchiveTab = {
        id: tabId,
        archive,
        name: tabName,
        is_modified: false,
        filter_options: { ...DEFAULT_FILTER_OPTIONS },
        selected_entries: [],
      };

      setTabs((prev) => [...prev, newTab]);
      setActiveTabId(tabId);

      // Update available RW versions
      const rwVersions = new Set(availableRwVersions);
      for (const entry of archive.entries) {
        if (entry.renderware_version) {
          rwVersions.add(entry.renderware_version);
        }
      }
      setAvailableRwVersions(Array.from(rwVersions).sort());
    } catch (error) {
      console.error("Failed to load IMG archive:", error);
      setErrorMessage(
        error instanceof Error ? error.message : "Failed to load IMG archive"
      );
    } finally {
      setIsLoading(false);
    }
  }, [availableRwVersions]);

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

  // Get filtered entries for a tab
  const getFilteredEntries = useCallback((tab: ImgArchiveTab): ImgEntry[] => {
    let entries = [...tab.archive.entries];

    // Apply type filter
    if (tab.filter_options.filter_type !== ImgFilterTypeValues.ALL) {
      if (tab.filter_options.filter_type === ImgFilterTypeValues.OTHER) {
        entries = entries.filter(
          (entry) =>
            !["DFF", "TXD", "COL", "IPL", "IDE"].includes(entry.file_type || "")
        );
      } else {
        entries = entries.filter(
          (entry) =>
            entry.file_type === tab.filter_options.filter_type.toUpperCase()
        );
      }
    }

    // Apply search filter
    if (tab.filter_options.search_text.trim()) {
      const searchLower = tab.filter_options.search_text.toLowerCase();
      entries = entries.filter((entry) =>
        entry.name.toLowerCase().includes(searchLower)
      );
    }

    // TODO: Apply RW version filter (not implemented yet)

    // Apply sorting
    entries.sort((a, b) => {
      let aValue: any, bValue: any;

      switch (tab.filter_options.sort_field) {
        case ImgSortFieldValues.NAME:
          aValue = a.name.toLowerCase();
          bValue = b.name.toLowerCase();
          break;
        case ImgSortFieldValues.SIZE:
          aValue = a.size;
          bValue = b.size;
          break;
        case ImgSortFieldValues.OFFSET:
          aValue = a.offset;
          bValue = b.offset;
          break;
        case ImgSortFieldValues.TYPE:
          aValue = a.file_type || "";
          bValue = b.file_type || "";
          break;
        case ImgSortFieldValues.RW_VERSION:
          aValue = a.renderware_version || "";
          bValue = b.renderware_version || "";
          break;
        default:
          return 0;
      }

      if (aValue < bValue)
        return tab.filter_options.sort_direction === ImgSortDirectionValues.ASC
          ? -1
          : 1;
      if (aValue > bValue)
        return tab.filter_options.sort_direction === ImgSortDirectionValues.ASC
          ? 1
          : -1;
      return 0;
    });

    return entries;
  }, []);

  // Update tab filter options
  const updateTabFilterOptions = useCallback(
    (tabId: string, options: Partial<ImgFilterOptions>) => {
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

  // Toggle entry selection
  const toggleEntrySelection = useCallback(
    (tabId: string, entryName: string) => {
      setTabs((prev) =>
        prev.map((tab) =>
          tab.id === tabId
            ? {
                ...tab,
                selected_entries: tab.selected_entries.includes(entryName)
                  ? tab.selected_entries.filter((name) => name !== entryName)
                  : [...tab.selected_entries, entryName],
              }
            : tab
        )
      );
    },
    []
  );

  // Select all entries in current tab
  const selectAllEntries = useCallback(() => {
    if (!activeTabId) return;

    const tab = tabs.find((t) => t.id === activeTabId);
    if (!tab) return;

    const filteredEntries = getFilteredEntries(tab);
    const allSelected = filteredEntries.every((entry) =>
      tab.selected_entries.includes(entry.name)
    );

    if (allSelected) {
      // Deselect all
      setTabs((prev) =>
        prev.map((t) =>
          t.id === activeTabId ? { ...t, selected_entries: [] } : t
        )
      );
    } else {
      // Select all filtered entries
      setTabs((prev) =>
        prev.map((t) =>
          t.id === activeTabId
            ? {
                ...t,
                selected_entries: filteredEntries.map((entry) => entry.name),
              }
            : t
        )
      );
    }
  }, [activeTabId, tabs, getFilteredEntries]);

  // Batch extract selected entries
  const batchExtractSelected = useCallback(async () => {
    if (!activeTabId) return;

    const tab = tabs.find((t) => t.id === activeTabId);
    if (!tab || tab.selected_entries.length === 0) return;

    try {
      setBatchOperationInProgress(true);
      setBatchProgress({
        total: tab.selected_entries.length,
        completed: 0,
        errors: [],
      });

      // Choose output directory
      const outputDir = await open({
        directory: true,
        title: "Choose output directory for extracted files",
      });

      if (!outputDir) {
        setBatchOperationInProgress(false);
        return;
      }

      // Prepare operations
      const operations: ImgOperation[] = tab.selected_entries.map(
        (entryName) => ({
          entry_name: entryName,
          output_path: `${outputDir}/${entryName}`,
        })
      );

      // Start batch extraction
      const results: OperationResult[] = await invoke(
        "batch_extract_img_entries",
        {
          archivePath: tab.archive.file_path,
          operations,
        }
      );

      // Update progress and show results
      setBatchProgress({
        total: operations.length,
        completed: operations.length,
        errors: results.filter((r) => !r.success),
      });

      const successCount = results.filter((r) => r.success).length;
      console.log(
        `Batch extraction completed: ${successCount}/${operations.length} successful`
      );

      // Clear selection after successful extraction
      if (successCount === operations.length) {
        setTabs((prev) =>
          prev.map((t) =>
            t.id === activeTabId ? { ...t, selected_entries: [] } : t
          )
        );
      }
    } catch (error) {
      console.error("Batch extraction failed:", error);
      setErrorMessage(
        error instanceof Error ? error.message : "Batch extraction failed"
      );
    } finally {
      setBatchOperationInProgress(false);
    }
  }, [activeTabId, tabs]);

  // Extract single entry
  const extractEntry = useCallback(
    async (entryName: string) => {
      if (!activeTabId) return;

      const tab = tabs.find((t) => t.id === activeTabId);
      if (!tab) return;

      try {
        const outputPath = await open({
          title: `Save ${entryName} as...`,
          defaultPath: entryName,
        });

        if (!outputPath) return;

        await invoke("extract_img_entry", {
          archivePath: tab.archive.file_path,
          entryName,
          outputPath,
        });

        console.log(`Successfully extracted: ${entryName}`);
      } catch (error) {
        console.error(`Failed to extract ${entryName}:`, error);
        setErrorMessage(
          error instanceof Error
            ? error.message
            : `Failed to extract ${entryName}`
        );
      }
    },
    [activeTabId, tabs]
  );

  // Get file type icon
  const getFileTypeIcon = (fileType?: string) => {
    switch (fileType?.toLowerCase()) {
      case "dff":
        return "📦"; // 3D Model
      case "txd":
        return "🎨"; // Texture
      case "col":
        return "🔶"; // Collision
      case "ipl":
        return "📍"; // Placement
      case "ide":
        return "📋"; // Definition
      default:
        return "📄"; // Generic file
    }
  };

  // Get file type color
  const getFileTypeColor = (fileType?: string) => {
    switch (fileType?.toLowerCase()) {
      case "dff":
        return "bg-blue-100 text-blue-800";
      case "txd":
        return "bg-green-100 text-green-800";
      case "col":
        return "bg-red-100 text-red-800";
      case "ipl":
        return "bg-purple-100 text-purple-800";
      case "ide":
        return "bg-yellow-100 text-yellow-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  };

  const activeTab = tabs.find((tab) => tab.id === activeTabId);
  const filteredEntries = activeTab ? getFilteredEntries(activeTab) : [];

  return (
    <div className={`flex h-full flex-col ${className}`}>
      {/* Toolbar */}
      <div className="flex items-center gap-2 border-b p-4">
        <Button
          disabled={isLoading}
          onClick={loadImgArchive}
          size="sm"
          variant="outline"
        >
          {isLoading ? (
            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
          ) : (
            <FolderOpen className="mr-2 h-4 w-4" />
          )}
          Open IMG
        </Button>

        <Separator className="h-6" orientation="vertical" />

        <Button
          disabled={
            !activeTab ||
            activeTab.selected_entries.length === 0 ||
            batchOperationInProgress
          }
          onClick={batchExtractSelected}
          size="sm"
          variant="outline"
        >
          <Download className="mr-2 h-4 w-4" />
          Extract Selected ({activeTab?.selected_entries.length || 0})
        </Button>

        <Button disabled={!activeTab} size="sm" variant="outline">
          <Upload className="mr-2 h-4 w-4" />
          Import Files
        </Button>

        <div className="ml-auto flex items-center gap-2">
          <Button
            disabled={!activeTab}
            onClick={selectAllEntries}
            size="sm"
            variant="ghost"
          >
            {activeTab &&
            filteredEntries.length > 0 &&
            filteredEntries.every((entry) =>
              activeTab.selected_entries.includes(entry.name)
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
                <Archive className="mr-1 h-3 w-3" />
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
            const filtered = getFilteredEntries(tab);

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
                    placeholder="Search entries..."
                    value={tab.filter_options.search_text}
                  />

                  <Select
                    onValueChange={(value) =>
                      value &&
                      updateTabFilterOptions(tab.id, {
                        filter_type: value as ImgFilterType,
                      })
                    }
                    value={tab.filter_options.filter_type}
                  >
                    <SelectTrigger className="w-32">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value={ImgFilterTypeValues.ALL}>
                        All Types
                      </SelectItem>
                      <SelectItem value={ImgFilterTypeValues.DFF}>
                        DFF Models
                      </SelectItem>
                      <SelectItem value={ImgFilterTypeValues.TXD}>
                        TXD Textures
                      </SelectItem>
                      <SelectItem value={ImgFilterTypeValues.COL}>
                        COL Collision
                      </SelectItem>
                      <SelectItem value={ImgFilterTypeValues.IPL}>
                        IPL Placement
                      </SelectItem>
                      <SelectItem value={ImgFilterTypeValues.IDE}>
                        IDE Definition
                      </SelectItem>
                      <SelectItem value={ImgFilterTypeValues.OTHER}>
                        Other
                      </SelectItem>
                    </SelectContent>
                  </Select>

                  <Select
                    onValueChange={(value) =>
                      value &&
                      updateTabFilterOptions(tab.id, {
                        sort_field: value as ImgSortField,
                      })
                    }
                    value={tab.filter_options.sort_field}
                  >
                    <SelectTrigger className="w-32">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value={ImgSortFieldValues.NAME}>
                        Name
                      </SelectItem>
                      <SelectItem value={ImgSortFieldValues.SIZE}>
                        Size
                      </SelectItem>
                      <SelectItem value={ImgSortFieldValues.TYPE}>
                        Type
                      </SelectItem>
                      <SelectItem value={ImgSortFieldValues.RW_VERSION}>
                        RW Version
                      </SelectItem>
                    </SelectContent>
                  </Select>

                  <Button
                    onClick={() =>
                      updateTabFilterOptions(tab.id, {
                        sort_direction:
                          tab.filter_options.sort_direction ===
                          ImgSortDirectionValues.ASC
                            ? ImgSortDirectionValues.DESC
                            : ImgSortDirectionValues.ASC,
                      })
                    }
                    size="sm"
                    variant="ghost"
                  >
                    {tab.filter_options.sort_direction ===
                    ImgSortDirectionValues.ASC
                      ? "↑"
                      : "↓"}
                  </Button>
                </div>

                {/* Archive Info */}
                {activeTab && (
                  <div className="border-b bg-muted/30 px-4 py-2">
                    <div className="flex items-center gap-4 text-sm">
                      <span>
                        <strong>Version:</strong>{" "}
                        {activeTab.archive.version === "V1"
                          ? "Version 1 (GTA III/VC)"
                          : "Version 2 (GTA SA)"}
                      </span>
                      <span>
                        <strong>Entries:</strong>{" "}
                        {activeTab.archive.total_entries.toLocaleString()}
                      </span>
                      <span>
                        <strong>Path:</strong>{" "}
                        {activeTab.archive.file_path.split("/").pop() ||
                          activeTab.archive.file_path.split("\\").pop()}
                      </span>
                    </div>

                    {/* RW Version Dashboard */}
                    <div className="mt-3">
                      <div className="grid grid-cols-2 gap-4 md:grid-cols-4">
                        {/* File Type Breakdown */}
                        <Card>
                          <CardHeader className="pb-2">
                            <CardTitle className="text-sm">
                              File Types
                            </CardTitle>
                          </CardHeader>
                          <CardContent className="pt-0">
                            <div className="space-y-1 text-xs">
                              {(() => {
                                const typeCounts: Record<string, number> = {};
                                for (const entry of activeTab.archive.entries) {
                                  const type = entry.file_type || "Unknown";
                                  typeCounts[type] =
                                    (typeCounts[type] || 0) + 1;
                                }
                                return Object.entries(typeCounts)
                                  .sort(([, a], [, b]) => b - a)
                                  .slice(0, 3)
                                  .map(([type, count]) => (
                                    <div
                                      className="flex justify-between"
                                      key={type}
                                    >
                                      <span className="font-medium">
                                        {type}:
                                      </span>
                                      <span>{count}</span>
                                    </div>
                                  ));
                              })()}
                            </div>
                          </CardContent>
                        </Card>

                        {/* RW Version Breakdown */}
                        <Card>
                          <CardHeader className="pb-2">
                            <CardTitle className="text-sm">
                              RW Versions
                            </CardTitle>
                          </CardHeader>
                          <CardContent className="pt-0">
                            <div className="space-y-1 text-xs">
                              {(() => {
                                const versionCounts: Record<string, number> =
                                  {};
                                for (const entry of activeTab.archive.entries) {
                                  const version =
                                    entry.renderware_version || "Unknown";
                                  versionCounts[version] =
                                    (versionCounts[version] || 0) + 1;
                                }
                                return Object.entries(versionCounts)
                                  .sort(([, a], [, b]) => b - a)
                                  .slice(0, 3)
                                  .map(([version, count]) => (
                                    <div
                                      className="flex justify-between"
                                      key={version}
                                    >
                                      <span
                                        className="max-w-[80px] truncate font-medium"
                                        title={version}
                                      >
                                        {version}:
                                      </span>
                                      <span>{count}</span>
                                    </div>
                                  ));
                              })()}
                            </div>
                          </CardContent>
                        </Card>

                        {/* Game Breakdown */}
                        <Card>
                          <CardHeader className="pb-2">
                            <CardTitle className="text-sm">Games</CardTitle>
                          </CardHeader>
                          <CardContent className="pt-0">
                            <div className="space-y-1 text-xs">
                              {(() => {
                                const gameCounts: Record<string, number> = {};
                                for (const entry of activeTab.archive.entries) {
                                  if (entry.renderware_version) {
                                    // Extract game name from RW version string (e.g., "3.6.0.3 (San Andreas)" -> "San Andreas")
                                    const gameMatch =
                                      entry.renderware_version.match(
                                        GAME_NAME_REGEX
                                      );
                                    const game = gameMatch
                                      ? gameMatch[1]
                                      : "Unknown";
                                    gameCounts[game] =
                                      (gameCounts[game] || 0) + 1;
                                  }
                                }
                                return Object.entries(gameCounts)
                                  .sort(([, a], [, b]) => b - a)
                                  .slice(0, 3)
                                  .map(([game, count]) => (
                                    <div
                                      className="flex justify-between"
                                      key={game}
                                    >
                                      <span
                                        className="max-w-[80px] truncate font-medium"
                                        title={game}
                                      >
                                        {game}:
                                      </span>
                                      <span>{count}</span>
                                    </div>
                                  ));
                              })()}
                            </div>
                          </CardContent>
                        </Card>

                        {/* Size Statistics */}
                        <Card>
                          <CardHeader className="pb-2">
                            <CardTitle className="text-sm">
                              Size Stats
                            </CardTitle>
                          </CardHeader>
                          <CardContent className="pt-0">
                            <div className="space-y-1 text-xs">
                              {(() => {
                                const sizes = activeTab.archive.entries.map(
                                  (e) => e.size
                                );
                                const totalSize = sizes.reduce(
                                  (a, b) => a + b,
                                  0
                                );
                                const avgSize =
                                  sizes.length > 0
                                    ? totalSize / sizes.length
                                    : 0;
                                const maxSize =
                                  sizes.length > 0 ? Math.max(...sizes) : 0;

                                return [
                                  {
                                    label: "Total",
                                    value: `${(totalSize / 1024 / 1024).toFixed(1)} MB`,
                                  },
                                  {
                                    label: "Average",
                                    value: `${(avgSize / 1024).toFixed(1)} KB`,
                                  },
                                  {
                                    label: "Largest",
                                    value: `${(maxSize / 1024 / 1024).toFixed(1)} MB`,
                                  },
                                ].map(({ label, value }) => (
                                  <div
                                    className="flex justify-between"
                                    key={label}
                                  >
                                    <span className="font-medium">
                                      {label}:
                                    </span>
                                    <span>{value}</span>
                                  </div>
                                ));
                              })()}
                            </div>
                          </CardContent>
                        </Card>
                      </div>
                    </div>
                  </div>
                )}

                {/* Entries List */}
                <ScrollArea className="flex-1">
                  <div className="p-2">
                    {filtered.length === 0 ? (
                      <div className="py-8 text-center text-muted-foreground">
                        <Archive className="mx-auto mb-4 h-12 w-12 opacity-50" />
                        <p>No entries found</p>
                        {tab.filter_options.search_text && (
                          <p className="text-sm">
                            Try adjusting your search or filters
                          </p>
                        )}
                      </div>
                    ) : (
                      <div className="space-y-1">
                        {filtered.map((entry) => (
                          <div
                            className={`flex cursor-pointer items-center gap-3 rounded p-2 hover:bg-muted/50 ${
                              tab.selected_entries.includes(entry.name)
                                ? "bg-muted"
                                : ""
                            }`}
                            key={entry.name}
                            onClick={() =>
                              toggleEntrySelection(tab.id, entry.name)
                            }
                            onDoubleClick={() => extractEntry(entry.name)}
                            onKeyDown={(e) => {
                              if (e.key === "Enter" || e.key === " ") {
                                e.preventDefault();
                                toggleEntrySelection(tab.id, entry.name);
                              }
                            }}
                          >
                            <Checkbox
                              checked={tab.selected_entries.includes(
                                entry.name
                              )}
                            />

                            <div
                              className={`text-lg ${getFileTypeColor(entry.file_type)} rounded px-2 py-1 font-medium text-xs`}
                            >
                              {getFileTypeIcon(entry.file_type)}
                            </div>

                            <div className="min-w-0 flex-1">
                              <div className="truncate font-medium">
                                {entry.name}
                              </div>
                              <div className="text-muted-foreground text-xs">
                                {entry.renderware_version && (
                                  <span className="mr-2">
                                    {entry.renderware_version}
                                  </span>
                                )}
                                <span>{(entry.size / 1024).toFixed(1)} KB</span>
                              </div>
                            </div>

                            {entry.file_type && (
                              <Badge className="text-xs" variant="secondary">
                                {entry.file_type}
                              </Badge>
                            )}
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

      {/* Empty state */}
      {tabs.length === 0 && !isLoading && (
        <div className="flex flex-1 items-center justify-center">
          <Card className="w-96">
            <CardHeader className="text-center">
              <Archive className="mx-auto mb-4 h-12 w-12 text-muted-foreground" />
              <CardTitle>IMG Archive Editor</CardTitle>
              <CardDescription>
                Open GTA IMG archive files to view and extract their contents
              </CardDescription>
            </CardHeader>
            <CardContent className="text-center">
              <Button className="w-full" onClick={loadImgArchive}>
                <FolderOpen className="mr-2 h-4 w-4" />
                Open IMG Archive
              </Button>
            </CardContent>
          </Card>
        </div>
      )}

      {/* Batch operation progress */}
      {batchOperationInProgress && batchProgress && (
        <div className="fixed right-4 bottom-4 w-80">
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm">Batch Extraction</CardTitle>
            </CardHeader>
            <CardContent>
              <Progress
                className="mb-2"
                value={(batchProgress.completed / batchProgress.total) * 100}
              />
              <p className="text-muted-foreground text-xs">
                {batchProgress.completed} / {batchProgress.total} files
                {batchProgress.current_entry && (
                  <span className="block truncate">
                    {batchProgress.current_entry}
                  </span>
                )}
              </p>
              {batchProgress.errors.length > 0 && (
                <p className="mt-1 text-destructive text-xs">
                  {batchProgress.errors.length} errors
                </p>
              )}
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}
