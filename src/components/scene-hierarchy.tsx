import {
  Box,
  ChevronDown,
  ChevronRight,
  Circle,
  Eye,
  EyeOff,
  Lightbulb,
  Moon,
  Package,
  Square,
  Sun,
  Zap,
} from "lucide-react";
import { memo, useCallback, useMemo, useState } from "react";
// import { FixedSizeTree as Tree } from "react-vtree";
import { Badge } from "~/components/ui/badge";
import { cn } from "~/lib/utils";
import { useSceneStore } from "~/stores/scene-store";

// Regex for UUID validation
const UUID_REGEX = /^[0-9a-f-]+$/;

// Tree node data structure
interface TreeNode {
  id: string;
  name: string;
  type: "object" | "light" | "group";
  children?: TreeNode[];
  level: number;
  isExpanded?: boolean;
  isSelected?: boolean;
  isVisible?: boolean;
  objectType?: string;
  data?: unknown;
}

// Tree node component
const TreeNodeComponent = memo(function TreeNodeComponent({
  node,
  onToggle,
  onSelect,
  onToggleVisibility,
}: {
  node: TreeNode;
  onToggle: (nodeId: string) => void;
  onSelect: (nodeId: string) => void;
  onToggleVisibility: (nodeId: string) => void;
}) {
  const hasChildren = node.children && node.children.length > 0;
  const isExpanded = node.isExpanded;

  const handleToggle = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      onToggle(node.id);
    },
    [node.id, onToggle]
  );

  const handleSelect = useCallback(() => {
    onSelect(node.id);
  }, [node.id, onSelect]);

  const handleToggleVisibility = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      onToggleVisibility(node.id);
    },
    [node.id, onToggleVisibility]
  );

  const getIcon = () => {
    if (node.type === "light") {
      switch (node.objectType) {
        case "directional":
          return <Sun className="h-4 w-4" />;
        case "point":
          return <Zap className="h-4 w-4" />;
        case "spot":
          return <Lightbulb className="h-4 w-4" />;
        default:
          return <Moon className="h-4 w-4" />;
      }
    }

    switch (node.objectType) {
      case "cube":
        return <Box className="h-4 w-4" />;
      case "sphere":
        return <Circle className="h-4 w-4" />;
      case "plane":
        return <Square className="h-4 w-4" />;
      case "imported":
        return <Package className="h-4 w-4" />;
      default:
        return <Package className="h-4 w-4" />;
    }
  };

  return (
    <div>
      <button
        className={cn(
          "flex w-full cursor-pointer items-center gap-2 px-2 py-1 text-left transition-colors hover:bg-accent/50",
          node.isSelected && "bg-accent",
          "select-none"
        )}
        onClick={handleSelect}
        style={{ paddingLeft: `${node.level * 16 + 8}px` }}
        type="button"
      >
        {/* Expansion Toggle */}
        <div className="flex h-4 w-4 items-center justify-center">
          {hasChildren && (
            // biome-ignore lint/a11y/useSemanticElements: <role="button" is required here (<button> cannot be used because hydrataion error)>
            <div
              className="inline-flex h-4 w-4 cursor-pointer items-center justify-center rounded-md font-medium text-sm ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
              onClick={handleToggle}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  e.preventDefault();
                  onToggle(node.id);
                }
              }}
              role="button"
              tabIndex={0}
            >
              {isExpanded ? (
                <ChevronDown className="h-3 w-3" />
              ) : (
                <ChevronRight className="h-3 w-3" />
              )}
            </div>
          )}
        </div>

        {/* Visibility Toggle */}
        {/** biome-ignore lint/a11y/useSemanticElements: <role="button" is required here (<button> cannot be used because hydrataion error)> */}
        <div
          className="inline-flex h-4 w-4 cursor-pointer items-center justify-center rounded-md font-medium text-sm ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
          onClick={handleToggleVisibility}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              onToggleVisibility(node.id);
            }
          }}
          role="button"
          tabIndex={0}
        >
          {node.isVisible ? (
            <Eye className="h-3 w-3" />
          ) : (
            <EyeOff className="h-3 w-3 opacity-50" />
          )}
        </div>

        {/* Icon */}
        <div className="shrink-0 text-muted-foreground">{getIcon()}</div>

        {/* Name */}
        <div className="min-w-0 flex-1">
          <div className="truncate text-sm" title={node.name}>
            {node.name}
          </div>
        </div>

        {/* Type Badge */}
        <Badge className="px-1 py-0 text-xs" variant="outline">
          {node.objectType}
        </Badge>
      </button>

      {/* Children */}
      {hasChildren && isExpanded && (
        <div>
          {node.children?.map((child) => (
            <TreeNodeComponent
              key={child.id}
              node={child}
              onSelect={onSelect}
              onToggle={onToggle}
              onToggleVisibility={onToggleVisibility}
            />
          ))}
        </div>
      )}
    </div>
  );
});

// Main scene hierarchy component
export const SceneHierarchy = memo(function SceneHierarchy({
  className,
}: {
  className?: string;
}) {
  const objects = useSceneStore((state) => state.objects);
  const lights = useSceneStore((state) => state.lights);
  const selectedObjectIds = useSceneStore((state) => state.selectedObjectIds);
  const selectedLightIds = useSceneStore((state) => state.selectedLightIds);
  const { selectObject, selectLight, updateObject, updateLight } =
    useSceneStore();

  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(
    new Set(["root"])
  );

  // Build tree structure
  const treeData = useMemo(() => {
    const buildTree = (): TreeNode => {
      const rootNode: TreeNode = {
        id: "root",
        name: "Scene",
        type: "group",
        level: 0,
        isExpanded: expandedNodes.has("root"),
        children: [],
      };

      // Add lights group
      if (lights.length > 0) {
        const lightsGroup: TreeNode = {
          id: "lights",
          name: "Lights",
          type: "group",
          level: 1,
          isExpanded: expandedNodes.has("lights"),
          children: lights.map((light) => ({
            id: light.id,
            name: light.name,
            type: "light" as const,
            objectType: light.type,
            level: 2,
            isSelected: selectedLightIds.includes(light.id),
            isVisible: light.visible,
            data: light,
          })),
        };
        rootNode.children?.push(lightsGroup);
      }

      // Add objects group
      if (objects.length > 0) {
        const objectsGroup: TreeNode = {
          id: "objects",
          name: "Objects",
          type: "group",
          level: 1,
          isExpanded: expandedNodes.has("objects"),
          children: objects.map((object) => ({
            id: object.id,
            name: object.name,
            type: "object" as const,
            objectType: object.type,
            level: 2,
            isSelected: selectedObjectIds.includes(object.id),
            isVisible: object.visible,
            data: object,
          })),
        };
        rootNode.children?.push(objectsGroup);
      }

      return rootNode;
    };

    return buildTree();
  }, [objects, lights, selectedObjectIds, selectedLightIds, expandedNodes]);

  const handleToggle = useCallback((nodeId: string) => {
    setExpandedNodes((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(nodeId)) {
        newSet.delete(nodeId);
      } else {
        newSet.add(nodeId);
      }
      return newSet;
    });
  }, []);

  const handleSelect = useCallback(
    (nodeId: string) => {
      if (nodeId.startsWith("light_")) {
        selectLight(nodeId, false);
      } else if (nodeId.startsWith("object_") || UUID_REGEX.test(nodeId)) {
        selectObject(nodeId, false);
      }
    },
    [selectObject, selectLight]
  );

  const handleToggleVisibility = useCallback(
    (nodeId: string) => {
      if (nodeId.startsWith("light_")) {
        const light = lights.find((l) => l.id === nodeId);
        if (light) {
          updateLight(nodeId, { visible: !light.visible });
        }
      } else {
        const object = objects.find((o) => o.id === nodeId);
        if (object) {
          updateObject(nodeId, { visible: !object.visible });
        }
      }
    },
    [lights, objects, updateLight, updateObject]
  );

  if (objects.length === 0 && lights.length === 0) {
    return (
      <div
        className={cn(
          "flex items-center justify-center text-muted-foreground",
          className
        )}
      >
        <div className="text-center">
          <div className="mb-2 text-2xl">🌳</div>
          <div className="text-sm">Scene hierarchy is empty</div>
        </div>
      </div>
    );
  }

  return (
    <div className={cn("overflow-hidden rounded-lg border", className)}>
      {/* Header */}
      <div className="flex items-center gap-2 border-b bg-muted/50 px-3 py-2">
        <div className="font-medium text-sm">Scene Hierarchy</div>
        <Badge className="text-xs" variant="secondary">
          {objects.length + lights.length}
        </Badge>
      </div>

      {/* Tree */}
      <div className="scrollbar-thin scrollbar-thumb-muted scrollbar-track-transparent max-h-96 overflow-auto">
        {(treeData.children || []).map((node) => (
          <TreeNodeComponent
            key={node.id}
            node={node}
            onSelect={handleSelect}
            onToggle={handleToggle}
            onToggleVisibility={handleToggleVisibility}
          />
        ))}
      </div>
    </div>
  );
});
