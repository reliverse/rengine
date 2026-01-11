import {
  Download,
  Eye,
  Filter,
  Image,
  Search,
  SortAsc,
  SortDesc,
  Upload,
  X,
} from "lucide-react";
import type React from "react";
import { useCallback, useState } from "react";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
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
import { cn } from "~/lib/utils";
import {
  useFilteredTextures,
  useTextureStats,
  useTextureStore,
} from "~/stores/texture-store";

interface TextureLibraryProps {
  className?: string;
  onTextureSelect?: (textureId: string) => void;
}

const TextureTypeColors = {
  albedo: "bg-red-500",
  normal: "bg-blue-500",
  roughness: "bg-gray-500",
  metallic: "bg-yellow-500",
  ao: "bg-purple-500",
  emissive: "bg-orange-500",
  height: "bg-green-500",
  opacity: "bg-pink-500",
  environment: "bg-cyan-500",
  custom: "bg-slate-500",
} as const;

const TextureTypeLabels = {
  albedo: "Albedo",
  normal: "Normal",
  roughness: "Roughness",
  metallic: "Metallic",
  ao: "AO",
  emissive: "Emissive",
  height: "Height",
  opacity: "Opacity",
  environment: "Environment",
  custom: "Custom",
} as const;

export function TextureLibrary({
  className,
  onTextureSelect,
}: TextureLibraryProps) {
  const {
    libraryVisible,
    filterType,
    filterSearch,
    sortBy,
    sortOrder,
    previewSize,
    showThumbnails,
    setLibraryVisible,
    setFilterType,
    setFilterSearch,
    setSortBy,
    setSortOrder,
    importTextures,
    exportAllTextures,
    selectTexture,
    removeTexture,
  } = useTextureStore();

  const filteredTextures = useFilteredTextures();
  const stats = useTextureStats();

  const [dragOver, setDragOver] = useState(false);
  const [selectedTextures, setSelectedTextures] = useState<Set<string>>(
    new Set()
  );

  const handleFileDrop = useCallback(
    async (e: React.DragEvent) => {
      e.preventDefault();
      setDragOver(false);

      const files = Array.from(e.dataTransfer.files).filter(
        (file) =>
          file.type.startsWith("image/") ||
          file.name.toLowerCase().endsWith(".tga") ||
          file.name.toLowerCase().endsWith(".dds")
      );

      if (files.length > 0) {
        try {
          await importTextures(files);
        } catch (error) {
          console.error("Failed to import textures:", error);
        }
      }
    },
    [importTextures]
  );

  const handleFileSelect = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const files = Array.from(e.target.files || []);
      if (files.length > 0) {
        try {
          await importTextures(files);
        } catch (error) {
          console.error("Failed to import textures:", error);
        }
      }
      // Reset input
      e.target.value = "";
    },
    [importTextures]
  );

  const handleTextureClick = useCallback(
    (textureId: string, e: React.MouseEvent) => {
      if (e.ctrlKey || e.metaKey) {
        // Multi-select
        setSelectedTextures((prev) => {
          const newSet = new Set(prev);
          if (newSet.has(textureId)) {
            newSet.delete(textureId);
          } else {
            newSet.add(textureId);
          }
          return newSet;
        });
      } else {
        // Single select
        setSelectedTextures(new Set([textureId]));
        selectTexture(textureId);
        onTextureSelect?.(textureId);
      }
    },
    [selectTexture, onTextureSelect]
  );

  const handleBulkDelete = useCallback(() => {
    selectedTextures.forEach((id) => removeTexture(id));
    setSelectedTextures(new Set());
  }, [selectedTextures, removeTexture]);

  const handleBulkExport = useCallback(
    async (format: "png" | "jpg" | "webp") => {
      await exportAllTextures(format);
    },
    [exportAllTextures]
  );

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Number.parseFloat((bytes / k ** i).toFixed(2)) + " " + sizes[i];
  };

  const formatDate = (date: Date) => {
    return (
      date.toLocaleDateString() +
      " " +
      date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })
    );
  };

  if (!libraryVisible) {
    return (
      <Button
        className="flex items-center gap-2"
        onClick={() => setLibraryVisible(true)}
        size="sm"
        variant="outline"
      >
        <Image className="h-4 w-4" />
        Texture Library
      </Button>
    );
  }

  return (
    <Card className={cn("flex h-full w-full flex-col", className)}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <Image className="h-5 w-5" />
            Texture Library
          </CardTitle>
          <Button
            onClick={() => setLibraryVisible(false)}
            size="sm"
            variant="ghost"
          >
            <X className="h-4 w-4" />
          </Button>
        </div>

        {/* Stats */}
        <div className="flex items-center gap-4 text-muted-foreground text-sm">
          <span>{stats.totalTextures} textures</span>
          <span>{formatFileSize(stats.totalMemoryUsage)} memory</span>
          {stats.cacheHitRate > 0 && (
            <span>{stats.cacheHitRate.toFixed(1)}% cache hit</span>
          )}
        </div>
      </CardHeader>

      <CardContent className="flex flex-1 flex-col gap-4 p-4 pt-0">
        {/* Toolbar */}
        <div className="flex flex-wrap items-center gap-2">
          {/* Search */}
          <div className="relative min-w-48 flex-1">
            <Search className="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 transform text-muted-foreground" />
            <Input
              className="pl-9"
              onChange={(e) => setFilterSearch(e.target.value)}
              placeholder="Search textures..."
              value={filterSearch}
            />
          </div>

          {/* Filter by type */}
          <Select
            onValueChange={(value: any) => setFilterType(value)}
            value={filterType}
          >
            <SelectTrigger className="w-32">
              <Filter className="mr-2 h-4 w-4" />
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Types</SelectItem>
              {Object.entries(TextureTypeLabels).map(([key, label]) => (
                <SelectItem key={key} value={key}>
                  {label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

          {/* Sort */}
          <Select
            onValueChange={(value: any) => setSortBy(value)}
            value={sortBy}
          >
            <SelectTrigger className="w-32">
              {sortOrder === "asc" ? (
                <SortAsc className="mr-2 h-4 w-4" />
              ) : (
                <SortDesc className="mr-2 h-4 w-4" />
              )}
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="name">Name</SelectItem>
              <SelectItem value="type">Type</SelectItem>
              <SelectItem value="size">Size</SelectItem>
              <SelectItem value="date">Date</SelectItem>
              <SelectItem value="usage">Usage</SelectItem>
            </SelectContent>
          </Select>

          <Button
            onClick={() => setSortOrder(sortOrder === "asc" ? "desc" : "asc")}
            size="sm"
            variant="outline"
          >
            {sortOrder === "asc" ? (
              <SortAsc className="h-4 w-4" />
            ) : (
              <SortDesc className="h-4 w-4" />
            )}
          </Button>

          {/* Import */}
          <div className="relative">
            <input
              accept="image/*,.tga,.dds"
              className="absolute inset-0 h-full w-full cursor-pointer opacity-0"
              multiple
              onChange={handleFileSelect}
              type="file"
            />
            <Button size="sm" variant="outline">
              <Upload className="mr-2 h-4 w-4" />
              Import
            </Button>
          </div>

          {/* Export */}
          <Select onValueChange={handleBulkExport}>
            <SelectTrigger className="w-24">
              <Download className="mr-2 h-4 w-4" />
              <SelectValue placeholder="Export" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="png">PNG</SelectItem>
              <SelectItem value="jpg">JPG</SelectItem>
              <SelectItem value="webp">WebP</SelectItem>
            </SelectContent>
          </Select>

          {/* Bulk actions */}
          {selectedTextures.size > 0 && (
            <>
              <Separator className="h-6" orientation="vertical" />
              <span className="text-muted-foreground text-sm">
                {selectedTextures.size} selected
              </span>
              <Button
                onClick={handleBulkDelete}
                size="sm"
                variant="destructive"
              >
                Delete
              </Button>
            </>
          )}
        </div>

        {/* Texture Grid */}
        <ScrollArea className="flex-1">
          <div
            className={cn(
              "grid gap-4 rounded-lg border-2 border-dashed p-4 transition-colors",
              dragOver ? "border-primary bg-primary/5" : "border-muted",
              "grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 xl:grid-cols-12"
            )}
            onDragLeave={() => setDragOver(false)}
            onDragOver={(e) => {
              e.preventDefault();
              setDragOver(true);
            }}
            onDrop={handleFileDrop}
          >
            {filteredTextures.map((texture) => (
              <TextureCard
                isSelected={selectedTextures.has(texture.id)}
                key={texture.id}
                onClick={(e) => handleTextureClick(texture.id, e)}
                onDelete={() => removeTexture(texture.id)}
                previewSize={previewSize}
                showThumbnail={showThumbnails}
                texture={texture}
              />
            ))}

            {filteredTextures.length === 0 && (
              <div className="col-span-full flex flex-col items-center justify-center py-12 text-center">
                <Image className="mb-4 h-12 w-12 text-muted-foreground" />
                <h3 className="mb-2 font-medium text-lg">No textures found</h3>
                <p className="mb-4 text-muted-foreground">
                  {filterSearch || filterType !== "all"
                    ? "Try adjusting your filters or search terms."
                    : "Import some textures to get started."}
                </p>
                <div className="relative">
                  <input
                    accept="image/*,.tga,.dds"
                    className="absolute inset-0 h-full w-full cursor-pointer opacity-0"
                    multiple
                    onChange={handleFileSelect}
                    type="file"
                  />
                  <Button>
                    <Upload className="mr-2 h-4 w-4" />
                    Import Textures
                  </Button>
                </div>
              </div>
            )}
          </div>
        </ScrollArea>
      </CardContent>
    </Card>
  );
}

interface TextureCardProps {
  texture: any;
  isSelected: boolean;
  previewSize: number;
  showThumbnail: boolean;
  onClick: (e: React.MouseEvent) => void;
  onDelete: () => void;
}

function TextureCard({
  texture,
  isSelected,
  previewSize,
  showThumbnail,
  onClick,
  onDelete,
}: TextureCardProps) {
  const [showDetails, setShowDetails] = useState(false);

  return (
    <div
      className={cn(
        "group relative cursor-pointer rounded-lg border-2 transition-all hover:shadow-md",
        isSelected ? "border-primary shadow-md" : "border-border",
        "aspect-square overflow-hidden"
      )}
      onClick={onClick}
      onMouseEnter={() => setShowDetails(true)}
      onMouseLeave={() => setShowDetails(false)}
    >
      {/* Thumbnail */}
      <div className="flex h-full w-full items-center justify-center bg-muted">
        {showThumbnail && texture.thumbnail ? (
          <img
            alt={texture.name}
            className="h-full w-full object-cover"
            src={texture.thumbnail}
            style={{ imageRendering: "pixelated" }}
          />
        ) : (
          <div className="flex flex-col items-center justify-center text-muted-foreground">
            <Image className="mb-2 h-8 w-8" />
            <span className="px-2 text-center text-xs">{texture.name}</span>
          </div>
        )}
      </div>

      {/* Type indicator */}
      <div
        className={cn(
          "absolute top-1 left-1 h-3 w-3 rounded-full",
          TextureTypeColors[texture.type] || "bg-slate-500"
        )}
      />

      {/* Selection indicator */}
      {isSelected && (
        <div className="absolute top-1 right-1 flex h-4 w-4 items-center justify-center rounded-full bg-primary">
          <div className="h-2 w-2 rounded-full bg-primary-foreground" />
        </div>
      )}

      {/* Details overlay */}
      {showDetails && (
        <div className="absolute inset-0 flex flex-col justify-between bg-black/75 p-2 text-white text-xs">
          <div>
            <div className="truncate font-medium">{texture.name}</div>
            <div className="text-muted-foreground">
              {TextureTypeLabels[texture.type]}
            </div>
          </div>

          <div className="space-y-1">
            <div>
              {texture.metadata.width}×{texture.metadata.height}
            </div>
            <div>{formatFileSize(texture.metadata.sizeBytes)}</div>
            {texture.usage.materialCount > 0 && (
              <div className="flex items-center gap-1">
                <Eye className="h-3 w-3" />
                {texture.usage.materialCount}
              </div>
            )}
          </div>
        </div>
      )}

      {/* Delete button */}
      <Button
        className="absolute top-1 right-1 h-6 w-6 p-0 opacity-0 transition-opacity group-hover:opacity-100"
        onClick={(e) => {
          e.stopPropagation();
          onDelete();
        }}
        size="sm"
        variant="destructive"
      >
        <X className="h-3 w-3" />
      </Button>
    </div>
  );
}
