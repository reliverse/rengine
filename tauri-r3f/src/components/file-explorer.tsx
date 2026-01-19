/**
 * File Explorer Component
 */

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import {
  File,
  FileText,
  Folder,
  FolderOpen,
  HardDrive,
  RefreshCw,
  Search,
} from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { ScrollArea } from "~/components/ui/scroll-area";
import { Separator } from "~/components/ui/separator";
import { ModernDarkTheme } from "~/styles/theme";
import { getResponsiveManager } from "~/utils/responsive-utils";

interface FileEntry {
  name: string;
  path: string;
  is_directory: boolean;
  is_file: boolean;
  is_symlink: boolean;
  size?: number;
  modified_at?: string;
  created_at?: string;
  permissions?: string;
  extension?: string;
}

interface DirectoryContents {
  directories: FileEntry[];
  files: FileEntry[];
}

interface FileExplorerProps {
  className?: string;
  defaultPath?: string;
  onFileSelect?: (file: FileEntry) => void;
  onDirectorySelect?: (directory: FileEntry) => void;
  showHiddenFiles?: boolean;
}

interface TreeNode {
  entry: FileEntry;
  children?: TreeNode[];
  isExpanded?: boolean;
  isLoading?: boolean;
}

export function FileExplorer({
  className = "",
  defaultPath,
  onFileSelect,
  onDirectorySelect,
  showHiddenFiles = false,
}: FileExplorerProps) {
  const [currentPath, setCurrentPath] = useState<string>(defaultPath || "");
  const [contents, setContents] = useState<DirectoryContents | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [_treeRoots, _setTreeRoots] = useState<TreeNode[]>([]);
  const [selectedPath, setSelectedPath] = useState<string | null>(null);

  const rm = getResponsiveManager();

  // Load directory contents
  const loadDirectory = useCallback(
    async (path: string) => {
      if (!path) return;

      try {
        setIsLoading(true);
        setError(null);

        const result: DirectoryContents = await invoke("read_directory", {
          path,
        });

        // Filter hidden files if needed
        let filteredResult = result;
        if (!showHiddenFiles) {
          filteredResult = {
            directories: result.directories.filter(
              (dir) => !dir.name.startsWith(".")
            ),
            files: result.files.filter((file) => !file.name.startsWith(".")),
          };
        }

        setContents(filteredResult);
        setCurrentPath(path);
      } catch (err) {
        setError(
          err instanceof Error ? err.message : "Failed to load directory"
        );
        console.error("Failed to load directory:", err);
      } finally {
        setIsLoading(false);
      }
    },
    [showHiddenFiles]
  );

  // Initialize with default drives/directories
  useEffect(() => {
    const initializeExplorer = async () => {
      try {
        // Try to get home directory first
        const homePath = (await invoke("get_home_directory")) as string;
        await loadDirectory(homePath);
      } catch (_err) {
        console.warn("Could not get home directory, using fallback");
        // Fallback to current directory or root
        setCurrentPath("/");
        loadDirectory("/");
      }
    };

    initializeExplorer();
  }, [loadDirectory]);

  // Handle directory selection
  const handleDirectoryClick = useCallback(
    async (directory: FileEntry) => {
      setSelectedPath(directory.path);
      onDirectorySelect?.(directory);
      await loadDirectory(directory.path);
    },
    [loadDirectory, onDirectorySelect]
  );

  // Handle file selection
  const handleFileClick = useCallback(
    (file: FileEntry) => {
      setSelectedPath(file.path);
      onFileSelect?.(file);
    },
    [onFileSelect]
  );

  // Navigate to parent directory
  const goToParent = useCallback(async () => {
    if (!currentPath || currentPath === "/") return;

    const parentPath = currentPath.split("/").slice(0, -1).join("/") || "/";
    await loadDirectory(parentPath);
  }, [currentPath, loadDirectory]);

  // Choose directory via dialog
  const chooseDirectory = useCallback(async () => {
    try {
      const selectedPath = await open({
        directory: true,
        title: "Choose directory",
      });

      if (selectedPath) {
        await loadDirectory(selectedPath as string);
      }
    } catch (err) {
      console.error("Failed to choose directory:", err);
    }
  }, [loadDirectory]);

  // Filter contents based on search
  const getFilteredContents = useCallback(() => {
    if (!(contents && searchQuery)) return contents;

    const query = searchQuery.toLowerCase();
    return {
      directories: contents.directories.filter((dir) =>
        dir.name.toLowerCase().includes(query)
      ),
      files: contents.files.filter((file) =>
        file.name.toLowerCase().includes(query)
      ),
    };
  }, [contents, searchQuery]);

  // Get file icon based on type
  const getFileIcon = (entry: FileEntry) => {
    if (entry.is_directory) {
      return <Folder className="h-4 w-4" />;
    }

    const ext = entry.extension?.toLowerCase();
    switch (ext) {
      case "dff":
      case "txd":
      case "col":
      case "ipl":
      case "ide":
        return <FileText className="h-4 w-4 text-blue-400" />;
      case "img":
        return <HardDrive className="h-4 w-4 text-orange-400" />;
      default:
        return <File className="h-4 w-4" />;
    }
  };

  // Format file size
  const formatFileSize = (size?: number): string => {
    if (!size) return "";

    if (size >= 1024 * 1024 * 1024) {
      return `${(size / (1024 * 1024 * 1024)).toFixed(1)} GB`;
    }
    if (size >= 1024 * 1024) {
      return `${(size / (1024 * 1024)).toFixed(1)} MB`;
    }
    if (size >= 1024) {
      return `${(size / 1024).toFixed(1)} KB`;
    }
    return `${size} B`;
  };

  const filteredContents = getFilteredContents();

  const explorerStyle = {
    backgroundColor: ModernDarkTheme.BACKGROUND_SECONDARY,
    border: `1px solid ${ModernDarkTheme.BORDER_SECONDARY}`,
    borderRadius: "3px",
    display: "flex",
    flexDirection: "column" as const,
    height: "100%",
  };

  const headerStyle = {
    padding: `${rm.getSpacingConfig().small}px`,
    borderBottom: `1px solid ${ModernDarkTheme.BORDER_SECONDARY}`,
    backgroundColor: ModernDarkTheme.BACKGROUND_TERTIARY,
  };

  const toolbarStyle = {
    display: "flex",
    alignItems: "center",
    gap: rm.getSpacingConfig().small,
    marginBottom: rm.getSpacingConfig().small,
  };

  const contentStyle = {
    flex: 1,
    overflow: "hidden",
  };

  const itemStyle = (isSelected: boolean) => ({
    display: "flex",
    alignItems: "center",
    gap: rm.getSpacingConfig().small,
    padding: `${rm.getSpacingConfig().small}px`,
    cursor: "pointer",
    borderRadius: "3px",
    backgroundColor: isSelected
      ? ModernDarkTheme.SELECTION_COLOR
      : "transparent",
    color: ModernDarkTheme.TEXT_PRIMARY,
    fontSize: `${rm.getFontConfig().body.size}px`,
    transition: "background-color 0.1s ease",
  });

  return (
    <div className={`file-explorer ${className}`} style={explorerStyle}>
      {/* Header */}
      <div style={headerStyle}>
        <div style={toolbarStyle}>
          <Button
            disabled={!currentPath || currentPath === "/"}
            onClick={goToParent}
            size="sm"
            style={{ padding: "4px" }}
            variant="ghost"
          >
            <ChevronUp className="h-4 w-4" />
          </Button>

          <Button
            disabled={isLoading}
            onClick={() => loadDirectory(currentPath)}
            size="sm"
            style={{ padding: "4px" }}
            variant="ghost"
          >
            <RefreshCw
              className={`h-4 w-4 ${isLoading ? "animate-spin" : ""}`}
            />
          </Button>

          <Button
            onClick={chooseDirectory}
            size="sm"
            style={{ padding: "4px" }}
            variant="ghost"
          >
            <FolderOpen className="h-4 w-4" />
          </Button>

          <div style={{ flex: 1, position: "relative" }}>
            <Search className="absolute top-1/2 left-2 h-4 w-4 -translate-y-1/2 transform text-muted-foreground" />
            <Input
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search files..."
              style={{
                paddingLeft: "28px",
                height: rm.getScaledSize(28),
                fontSize: `${rm.getFontConfig().body.size}px`,
              }}
              value={searchQuery}
            />
          </div>
        </div>

        {/* Current path */}
        <div
          style={{
            fontSize: `${rm.getFontConfig().small.size}px`,
            color: ModernDarkTheme.TEXT_SECONDARY,
            fontFamily: "monospace",
            padding: `${rm.getSpacingConfig().small}px 0`,
            wordBreak: "break-all",
          }}
        >
          {currentPath || "No directory selected"}
        </div>
      </div>

      {/* Content */}
      <div style={contentStyle}>
        <ScrollArea className="h-full">
          {error && (
            <div
              style={{
                padding: rm.getSpacingConfig().medium,
                color: ModernDarkTheme.TEXT_ERROR,
                fontSize: `${rm.getFontConfig().small.size}px`,
              }}
            >
              Error: {error}
            </div>
          )}

          {isLoading && (
            <div
              style={{
                padding: rm.getSpacingConfig().medium,
                color: ModernDarkTheme.TEXT_SECONDARY,
                fontSize: `${rm.getFontConfig().small.size}px`,
              }}
            >
              Loading...
            </div>
          )}

          {filteredContents && (
            <div>
              {/* Directories */}
              {filteredContents.directories.length > 0 && (
                <div>
                  <div
                    style={{
                      padding: `${rm.getSpacingConfig().small}px ${rm.getSpacingConfig().medium}px`,
                      fontSize: `${rm.getFontConfig().small.size}px`,
                      fontWeight: "bold",
                      color: ModernDarkTheme.TEXT_SECONDARY,
                      textTransform: "uppercase",
                      letterSpacing: "0.5px",
                    }}
                  >
                    Directories ({filteredContents.directories.length})
                  </div>
                  {filteredContents.directories.map((dir) => (
                    <div
                      key={dir.path}
                      onClick={() => handleDirectoryClick(dir)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter" || e.key === " ") {
                          e.preventDefault();
                          handleDirectoryClick(dir);
                        }
                      }}
                      onMouseEnter={(e) => {
                        if (selectedPath !== dir.path) {
                          e.currentTarget.style.backgroundColor =
                            ModernDarkTheme.HOVER_COLOR;
                        }
                      }}
                      onMouseLeave={(e) => {
                        if (selectedPath !== dir.path) {
                          e.currentTarget.style.backgroundColor = "transparent";
                        }
                      }}
                      role="button"
                      style={itemStyle(selectedPath === dir.path)}
                      tabIndex={0}
                    >
                      <Folder className="h-4 w-4 text-blue-400" />
                      <span style={{ flex: 1 }}>{dir.name}</span>
                    </div>
                  ))}
                  <Separator />
                </div>
              )}

              {/* Files */}
              {filteredContents.files.length > 0 && (
                <div>
                  <div
                    style={{
                      padding: `${rm.getSpacingConfig().small}px ${rm.getSpacingConfig().medium}px`,
                      fontSize: `${rm.getFontConfig().small.size}px`,
                      fontWeight: "bold",
                      color: ModernDarkTheme.TEXT_SECONDARY,
                      textTransform: "uppercase",
                      letterSpacing: "0.5px",
                    }}
                  >
                    Files ({filteredContents.files.length})
                  </div>
                  {filteredContents.files.map((file) => (
                    <div
                      key={file.path}
                      onClick={() => handleFileClick(file)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter" || e.key === " ") {
                          e.preventDefault();
                          handleFileClick(file);
                        }
                      }}
                      onMouseEnter={(e) => {
                        if (selectedPath !== file.path) {
                          e.currentTarget.style.backgroundColor =
                            ModernDarkTheme.HOVER_COLOR;
                        }
                      }}
                      onMouseLeave={(e) => {
                        if (selectedPath !== file.path) {
                          e.currentTarget.style.backgroundColor = "transparent";
                        }
                      }}
                      role="button"
                      style={itemStyle(selectedPath === file.path)}
                      tabIndex={0}
                    >
                      {getFileIcon(file)}
                      <span style={{ flex: 1 }}>{file.name}</span>
                      <span
                        style={{
                          fontSize: `${rm.getFontConfig().small.size}px`,
                          color: ModernDarkTheme.TEXT_TERTIARY,
                        }}
                      >
                        {formatFileSize(file.size)}
                      </span>
                    </div>
                  ))}
                </div>
              )}

              {filteredContents.directories.length === 0 &&
                filteredContents.files.length === 0 &&
                !isLoading && (
                  <div
                    style={{
                      padding: rm.getSpacingConfig().large,
                      textAlign: "center",
                      color: ModernDarkTheme.TEXT_TERTIARY,
                      fontSize: `${rm.getFontConfig().body.size}px`,
                    }}
                  >
                    {searchQuery
                      ? "No files match your search"
                      : "Directory is empty"}
                  </div>
                )}
            </div>
          )}
        </ScrollArea>
      </div>
    </div>
  );
}

// Import ChevronUp that I used but didn't import
import { ChevronUp } from "lucide-react";
