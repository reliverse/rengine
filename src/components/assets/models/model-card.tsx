import { useState } from "react";
import { Box, X } from "lucide-react";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";
import type { ModelCardProps } from "../types";

export function ModelCard({
  model,
  isSelected,
  showThumbnail,
  onClick,
  onDelete,
}: ModelCardProps) {
  const [showDetails, setShowDetails] = useState(false);

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
  };

  return (
    <div
      aria-label={`Model: ${model.name}`}
      className={cn(
        "group relative cursor-pointer rounded-lg border-2 transition-all hover:shadow-md",
        isSelected ? "border-primary shadow-md" : "border-border",
        "aspect-square overflow-hidden"
      )}
      onClick={onClick}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onClick(e as any);
        }
      }}
      onMouseEnter={() => setShowDetails(true)}
      onMouseLeave={() => setShowDetails(false)}
      role="button"
      tabIndex={0}
    >
      {/* Thumbnail */}
      <div className="flex h-full w-full items-center justify-center bg-muted">
        {showThumbnail && model.thumbnail ? (
          <img
            alt={model.name}
            className="h-full w-full object-cover"
            height={256}
            src={model.thumbnail}
            style={{ imageRendering: "pixelated" }}
            width={256}
          />
        ) : (
          <div className="flex flex-col items-center justify-center text-muted-foreground">
            <Box className="mb-2 h-8 w-8" />
            <span className="px-2 text-center text-xs">{model.name}</span>
          </div>
        )}
      </div>

      {/* Format badge */}
      <div className="absolute top-1 left-1 rounded bg-background/80 px-1 py-0.5 font-medium text-xs">
        {model.format.toUpperCase()}
      </div>

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
            <div className="truncate font-medium">{model.name}</div>
            <div className="text-muted-foreground">
              {model.format.toUpperCase()}
            </div>
          </div>

          <div className="space-y-1">
            <div>{formatFileSize(model.fileSize)}</div>
            {model.metadata.vertices && (
              <div className="flex items-center gap-1">
                <span>{model.metadata.vertices.toLocaleString()} verts</span>
              </div>
            )}
            {model.metadata.triangles && (
              <div className="flex items-center gap-1">
                <span>{model.metadata.triangles.toLocaleString()} tris</span>
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
