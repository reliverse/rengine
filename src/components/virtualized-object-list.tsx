import { Copy, Eye, EyeOff, MoreHorizontal, Trash2 } from "lucide-react";
import { memo, useCallback, useMemo } from "react";
import { VList } from "virtua";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "~/components/ui/dropdown-menu";
import { cn } from "~/lib/utils";
import type { SceneObject } from "~/stores/scene-store";
import { useSceneStore } from "~/stores/scene-store";

interface VirtualizedObjectListProps {
  height: number;
  className?: string;
}

// Wrapper component for virtua render prop
const ObjectListItemRenderer = memo(function ObjectListItemRenderer({
  object,
  isSelected,
  onSelect,
  onToggleVisibility,
  onDelete,
  onDuplicate,
}: {
  object: SceneObject;
  isSelected: boolean;
  onSelect: (id: string, multiSelect: boolean) => void;
  onToggleVisibility: (id: string) => void;
  onDelete: (id: string) => void;
  onDuplicate: (id: string) => void;
}) {
  const getObjectIcon = (type: string) => {
    switch (type) {
      case "cube":
        return "⬜";
      case "sphere":
        return "⭕";
      case "plane":
        return "▬";
      case "imported":
        return "📦";
      default:
        return "?";
    }
  };

  const isVisible = object.visible;

  const handleClick = useCallback(() => {
    onSelect(object.id, false);
  }, [object.id, onSelect]);

  const handleContextMenu = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    // Could add context menu here
  }, []);

  const handleToggleVisibility = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      onToggleVisibility(object.id);
    },
    [object.id, onToggleVisibility]
  );

  const handleDelete = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      onDelete(object.id);
    },
    [object.id, onDelete]
  );

  const handleDuplicate = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      onDuplicate(object.id);
    },
    [object.id, onDuplicate]
  );

  return (
    <button
      className={cn(
        "flex w-full cursor-pointer items-center gap-2 border-border border-b px-3 py-2 text-left transition-colors hover:bg-accent/50",
        isSelected && "bg-accent"
      )}
      onClick={handleClick}
      onContextMenu={handleContextMenu}
      type="button"
    >
      {/* Visibility Toggle */}
      <Button
        className="h-6 w-6 p-0"
        onClick={handleToggleVisibility}
        size="sm"
        variant="ghost"
      >
        {isVisible ? (
          <Eye className="h-3 w-3" />
        ) : (
          <EyeOff className="h-3 w-3 opacity-50" />
        )}
      </Button>

      {/* Object Icon/Type Indicator */}
      <div className="flex h-4 w-4 shrink-0 items-center justify-center rounded bg-primary/20">
        <span className="font-medium text-primary text-xs">
          {getObjectIcon(object.type)}
        </span>
      </div>

      {/* Object Name */}
      <div className="min-w-0 flex-1">
        <div className="truncate font-medium text-sm" title={object.name}>
          {object.name}
        </div>
        <div className="text-muted-foreground text-xs">
          {object.type} • ID: {object.id.slice(-6)}
        </div>
      </div>

      {/* Position Info */}
      <div className="font-mono text-muted-foreground text-xs">
        {object.position.map((v: number) => v.toFixed(1)).join(", ")}
      </div>

      {/* Actions Menu */}
      <DropdownMenu>
        <DropdownMenuTrigger>
          <Button
            className="h-6 w-6 p-0 opacity-0 group-hover:opacity-100"
            onClick={(e) => e.stopPropagation()}
            size="sm"
            variant="ghost"
          >
            <MoreHorizontal className="h-3 w-3" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem onClick={handleDuplicate}>
            <Copy className="mr-2 h-4 w-4" />
            Duplicate
          </DropdownMenuItem>
          <DropdownMenuItem
            className="text-muted-foreground"
            onClick={handleToggleVisibility}
          >
            {isVisible ? (
              <EyeOff className="mr-2 h-4 w-4" />
            ) : (
              <Eye className="mr-2 h-4 w-4" />
            )}
            {isVisible ? "Hide" : "Show"}
          </DropdownMenuItem>
          <DropdownMenuItem
            className="text-destructive focus:text-destructive"
            onClick={handleDelete}
          >
            <Trash2 className="mr-2 h-4 w-4" />
            Delete
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      {/* Selection Indicator */}
      {isSelected && (
        <div className="absolute top-0 bottom-0 left-0 w-1 rounded-r bg-primary" />
      )}
    </button>
  );
});

// Main virtualized list component
export const VirtualizedObjectList = memo(function VirtualizedObjectList({
  height,
  className,
}: VirtualizedObjectListProps) {
  const objects = useSceneStore((state) => state.objects);
  const selectedObjectIds = useSceneStore((state) => state.selectedObjectIds);
  const { selectObject, updateObject, removeObject, duplicateObject } =
    useSceneStore();

  const handleSelect = useCallback(
    (id: string, multiSelect: boolean) => {
      selectObject(id, multiSelect);
    },
    [selectObject]
  );

  const handleToggleVisibility = useCallback(
    (id: string) => {
      const object = objects.find((obj) => obj.id === id);
      if (object) {
        updateObject(id, { visible: !object.visible });
      }
    },
    [objects, updateObject]
  );

  const handleDelete = useCallback(
    (id: string) => {
      removeObject(id);
    },
    [removeObject]
  );

  const handleDuplicate = useCallback(
    (id: string) => {
      duplicateObject(id);
    },
    [duplicateObject]
  );

  // Create stable data array for virtua with optimized memoization
  const listData = useMemo(() => {
    // Create a map of selected object IDs for O(1) lookups
    const selectedIdsSet = new Set(selectedObjectIds);

    return objects.map((object) => ({
      // Use stable references for performance
      object,
      isSelected: selectedIdsSet.has(object.id),
      // Use stable callback references
      onSelect: handleSelect,
      onToggleVisibility: handleToggleVisibility,
      onDelete: handleDelete,
      onDuplicate: handleDuplicate,
    }));
  }, [
    objects,
    selectedObjectIds,
    handleSelect,
    handleToggleVisibility,
    handleDelete,
    handleDuplicate,
  ]);

  if (objects.length === 0) {
    return (
      <div
        className={cn(
          "flex items-center justify-center text-muted-foreground",
          className
        )}
      >
        <div className="text-center">
          <div className="mb-2 text-2xl">📦</div>
          <div className="text-sm">No objects in scene</div>
        </div>
      </div>
    );
  }

  return (
    <div className={cn("overflow-hidden rounded-lg border", className)}>
      {/* Header */}
      <div className="flex items-center gap-2 border-b bg-muted/50 px-3 py-2">
        <div className="font-medium text-sm">Scene Objects</div>
        <Badge className="text-xs" variant="secondary">
          {objects.length}
        </Badge>
      </div>

      {/* Virtualized List */}
      <VList
        className="scrollbar-thin scrollbar-thumb-muted scrollbar-track-transparent"
        data={listData}
        style={{ height: height - 40 }}
      >
        {(item: {
          object: SceneObject;
          isSelected: boolean;
          onSelect: (id: string, multiSelect: boolean) => void;
          onToggleVisibility: (id: string) => void;
          onDelete: (id: string) => void;
          onDuplicate: (id: string) => void;
        }) => <ObjectListItemRenderer {...item} />}
      </VList>
    </div>
  );
});
