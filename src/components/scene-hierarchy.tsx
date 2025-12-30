import {
  ChevronDown,
  ChevronRight,
  Eye,
  EyeOff,
  Plus,
  Search,
  Trash2,
} from "lucide-react";
import { useState } from "react";
import { Button } from "~/components/ui/button";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from "~/components/ui/context-menu";
import { Input } from "~/components/ui/input";
import { ScrollArea } from "~/components/ui/scroll-area";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "~/components/ui/tooltip";
import { cn } from "~/lib/utils";
import type { SceneLight, SceneObject } from "~/stores/scene-store";
import { useSceneStore } from "~/stores/scene-store";

interface HierarchyItemProps {
  object: SceneObject;
  level: number;
  searchTerm: string;
}

interface LightHierarchyItemProps {
  light: SceneLight;
  level: number;
  searchTerm: string;
}

function HierarchyItem({ object, level, searchTerm }: HierarchyItemProps) {
  const [isExpanded, setIsExpanded] = useState(true);
  const {
    selectedObjectIds,
    selectObject,
    updateObject,
    removeObject,
    duplicateObject,
  } = useSceneStore();

  const isSelected = selectedObjectIds.includes(object.id);
  const hren = false; // For now, no grouping support yet

  const handleSelect = (event: React.MouseEvent) => {
    event.stopPropagation();
    const multiSelect = event.ctrlKey || event.metaKey;
    selectObject(object.id, multiSelect);
  };

  const handleToggleVisibility = (event: React.MouseEvent) => {
    event.stopPropagation();
    updateObject(object.id, { visible: !object.visible });
  };

  const handleDuplicate = () => {
    duplicateObject(object.id);
  };

  const handleDelete = () => {
    removeObject(object.id);
  };

  const getObjectIcon = (type: SceneObject["type"]) => {
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
        return "❓";
    }
  };

  // Highlight search matches
  const displayName = object.name;
  const isMatch =
    searchTerm && displayName.toLowerCase().includes(searchTerm.toLowerCase());

  if (searchTerm && !isMatch) {
    return null;
  }

  return (
    <ContextMenu>
      <ContextMenuTrigger>
        <button
          className={cn(
            "group flex w-full cursor-pointer items-center gap-1 rounded px-2 py-1 text-left text-sm hover:bg-accent",
            isSelected && "bg-accent text-accent-foreground",
            isMatch && "bg-yellow-100 dark:bg-yellow-900"
          )}
          onClick={handleSelect}
          style={{ paddingLeft: `${level * 12 + 8}px` }}
          type="button"
        >
          {/* Expand/Collapse for future grouping */}
          {hren ? (
            <Button
              className="h-4 w-4 p-0 hover:bg-muted"
              onClick={(e) => {
                e.stopPropagation();
                setIsExpanded(!isExpanded);
              }}
              size="sm"
              variant="ghost"
            >
              {isExpanded ? (
                <ChevronDown className="h-3 w-3" />
              ) : (
                <ChevronRight className="h-3 w-3" />
              )}
            </Button>
          ) : (
            <div className="w-4" />
          )}

          {/* Object Icon */}
          <span className="text-xs">{getObjectIcon(object.type)}</span>

          {/* Object Name */}
          <span className="flex-1 truncate">{displayName}</span>

          {/* Visibility Toggle */}
          <Tooltip>
            <TooltipTrigger
              render={
                <Button
                  className="h-4 w-4 p-0 opacity-0 hover:bg-muted group-hover:opacity-100"
                  onClick={handleToggleVisibility}
                  size="sm"
                  variant="ghost"
                >
                  {object.visible ? (
                    <Eye className="h-3 w-3" />
                  ) : (
                    <EyeOff className="h-3 w-3 opacity-50" />
                  )}
                </Button>
              }
            />
            <TooltipContent>
              {object.visible ? "Hide object" : "Show object"}
            </TooltipContent>
          </Tooltip>
        </button>
      </ContextMenuTrigger>

      <ContextMenuContent>
        <ContextMenuItem onClick={handleSelect}>Select</ContextMenuItem>
        <ContextMenuItem onClick={handleDuplicate}>Duplicate</ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem
          className="text-muted-foreground"
          onClick={handleToggleVisibility}
        >
          {object.visible ? "Hide" : "Show"}
        </ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem
          className="text-destructive focus:text-destructive"
          onClick={handleDelete}
        >
          <Trash2 className="mr-2 h-4 w-4" />
          Delete
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}

function LightHierarchyItem({
  light,
  level,
  searchTerm,
}: LightHierarchyItemProps) {
  const {
    selectedLightIds,
    selectLight,
    updateLight,
    removeLight,
    duplicateLight,
  } = useSceneStore();

  const isSelected = selectedLightIds.includes(light.id);

  const handleSelect = (event: React.MouseEvent) => {
    event.stopPropagation();
    const multiSelect = event.ctrlKey || event.metaKey;
    selectLight(light.id, multiSelect);
  };

  const handleToggleVisibility = (event: React.MouseEvent) => {
    event.stopPropagation();
    updateLight(light.id, { visible: !light.visible });
  };

  const handleDuplicate = () => {
    duplicateLight(light.id);
  };

  const handleDelete = () => {
    removeLight(light.id);
  };

  const getLightIcon = (type: SceneLight["type"]) => {
    switch (type) {
      case "directional":
        return "☀️";
      case "point":
        return "💡";
      case "spot":
        return "🔦";
      case "ambient":
        return "🌟";
      case "hemisphere":
        return "🌅";
      default:
        return "❓";
    }
  };

  // Highlight search matches
  const displayName = light.name;
  const isMatch =
    searchTerm && displayName.toLowerCase().includes(searchTerm.toLowerCase());

  if (searchTerm && !isMatch) {
    return null;
  }

  return (
    <ContextMenu>
      <ContextMenuTrigger>
        <button
          className={cn(
            "group flex w-full cursor-pointer items-center gap-1 rounded px-2 py-1 text-left text-sm hover:bg-accent",
            isSelected && "bg-accent text-accent-foreground",
            isMatch && "bg-yellow-100 dark:bg-yellow-900"
          )}
          onClick={handleSelect}
          style={{ paddingLeft: `${level * 12 + 8}px` }}
          type="button"
        >
          {/* Light Icon */}
          <span className="text-xs">{getLightIcon(light.type)}</span>

          {/* Light Name */}
          <span className="flex-1 truncate">{displayName}</span>

          {/* Visibility Toggle */}
          <Tooltip>
            <TooltipTrigger
              render={
                <Button
                  className="h-4 w-4 p-0 opacity-0 hover:bg-muted group-hover:opacity-100"
                  onClick={handleToggleVisibility}
                  size="sm"
                  variant="ghost"
                >
                  {light.visible ? (
                    <Eye className="h-3 w-3" />
                  ) : (
                    <EyeOff className="h-3 w-3 opacity-50" />
                  )}
                </Button>
              }
            />
            <TooltipContent>
              {light.visible ? "Hide light" : "Show light"}
            </TooltipContent>
          </Tooltip>
        </button>
      </ContextMenuTrigger>

      <ContextMenuContent>
        <ContextMenuItem onClick={handleSelect}>Select</ContextMenuItem>
        <ContextMenuItem onClick={handleDuplicate}>Duplicate</ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem
          className="text-muted-foreground"
          onClick={handleToggleVisibility}
        >
          {light.visible ? "Hide" : "Show"}
        </ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem
          className="text-destructive focus:text-destructive"
          onClick={handleDelete}
        >
          <Trash2 className="mr-2 h-4 w-4" />
          Delete
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}

interface SceneHierarchyProps {
  className?: string;
}

export function SceneHierarchy({ className }: SceneHierarchyProps) {
  const [searchTerm, setSearchTerm] = useState("");
  const objects = useSceneStore((state) => state.objects);
  const lights = useSceneStore((state) => state.lights);
  const objectCount = objects.length;
  const lightCount = lights.length;
  const visibleObjectCount = objects.filter((obj) => obj.visible).length;
  const visibleLightCount = lights.filter((light) => light.visible).length;

  const filteredObjects = objects.filter((obj) =>
    searchTerm
      ? obj.name.toLowerCase().includes(searchTerm.toLowerCase())
      : true
  );

  const filteredLights = lights.filter((light) =>
    searchTerm
      ? light.name.toLowerCase().includes(searchTerm.toLowerCase())
      : true
  );

  return (
    <div className={cn("flex h-full flex-col", className)}>
      {/* Header */}
      <div className="flex items-center justify-between border-b p-4">
        <div>
          <h3 className="font-semibold text-sm">Scene Hierarchy</h3>
          <p className="text-muted-foreground text-xs">
            {visibleObjectCount + visibleLightCount}/{objectCount + lightCount}{" "}
            visible
          </p>
        </div>
        <Tooltip>
          <TooltipTrigger
            render={
              <Button className="h-8 w-8 p-0" size="sm" variant="ghost">
                <Plus className="h-4 w-4" />
              </Button>
            }
          />
          <TooltipContent>Add object/light (use toolbar)</TooltipContent>
        </Tooltip>
      </div>

      {/* Search */}
      <div className="border-b p-4">
        <div className="relative">
          <Search className="absolute top-2.5 left-2 h-4 w-4 text-muted-foreground" />
          <Input
            className="h-8 pl-8"
            onChange={(e) => setSearchTerm(e.target.value)}
            placeholder="Search objects and lights..."
            value={searchTerm}
          />
        </div>
      </div>

      {/* Scene Elements List */}
      <ScrollArea className="flex-1">
        <div className="p-2">
          {filteredObjects.length === 0 && filteredLights.length === 0 ? (
            <div className="py-8 text-center">
              <p className="text-muted-foreground text-sm">
                {searchTerm
                  ? "No objects or lights match your search"
                  : "No objects or lights in scene"}
              </p>
              <p className="mt-1 text-muted-foreground text-xs">
                {searchTerm
                  ? "Try a different search term"
                  : "Add objects and lights using the toolbar"}
              </p>
            </div>
          ) : (
            <>
              {/* Lights Section */}
              {filteredLights.length > 0 && (
                <div className="mb-4">
                  <h4 className="mb-2 px-2 font-medium text-muted-foreground text-xs">
                    Lights ({filteredLights.length})
                  </h4>
                  {filteredLights.map((light) => (
                    <LightHierarchyItem
                      key={light.id}
                      level={0}
                      light={light}
                      searchTerm={searchTerm}
                    />
                  ))}
                </div>
              )}

              {/* Objects Section */}
              {filteredObjects.length > 0 && (
                <div>
                  <h4 className="mb-2 px-2 font-medium text-muted-foreground text-xs">
                    Objects ({filteredObjects.length})
                  </h4>
                  {filteredObjects.map((object) => (
                    <HierarchyItem
                      key={object.id}
                      level={0}
                      object={object}
                      searchTerm={searchTerm}
                    />
                  ))}
                </div>
              )}
            </>
          )}
        </div>
      </ScrollArea>

      {/* Footer with stats */}
      {(objectCount > 0 || lightCount > 0) && (
        <div className="border-t bg-muted/50 p-2">
          <div className="grid grid-cols-2 gap-2 text-muted-foreground text-xs">
            <div className="text-center">
              <span>
                Objects: {visibleObjectCount}/{objectCount}
              </span>
            </div>
            <div className="text-center">
              <span>
                Lights: {visibleLightCount}/{lightCount}
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
