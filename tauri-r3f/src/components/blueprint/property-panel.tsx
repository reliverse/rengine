/**
 * Property Panel Component
 * Editor for node properties and settings
 */

import { useState } from "react";
import { Settings, X } from "lucide-react";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import { ScrollArea } from "~/components/ui/scroll-area";
import { cn } from "~/lib/utils";
import type { BlueprintNode } from "~/utils/blueprint/graph/node-types";

interface PropertyPanelProps {
  node: BlueprintNode | null;
  onUpdateNode?: (nodeId: string, updates: Partial<BlueprintNode>) => void;
  onClose?: () => void;
  className?: string;
}

export function PropertyPanel({
  node,
  onUpdateNode,
  onClose,
  className,
}: PropertyPanelProps) {
  const [localProperties, setLocalProperties] = useState<Record<string, any>>(
    node?.properties || {}
  );

  if (!node) {
    return (
      <div
        className={cn(
          "flex h-full w-80 flex-col border-l bg-background",
          className
        )}
      >
        <div className="flex items-center justify-between border-b p-4">
          <div className="flex items-center gap-2">
            <Settings className="h-5 w-5" />
            <h2 className="font-semibold text-lg">Properties</h2>
          </div>
        </div>
        <div className="flex flex-1 items-center justify-center p-8 text-center text-muted-foreground">
          <div>
            <p className="mb-2 font-medium">No node selected</p>
            <p className="text-sm">
              Select a node to view and edit its properties
            </p>
          </div>
        </div>
      </div>
    );
  }

  const handlePropertyChange = (key: string, value: any) => {
    const updated = { ...localProperties, [key]: value };
    setLocalProperties(updated);
    onUpdateNode?.(node.id, { properties: updated });
  };

  const handleTitleChange = (title: string) => {
    onUpdateNode?.(node.id, { title });
  };

  return (
    <div
      className={cn(
        "flex h-full w-80 flex-col border-l bg-background",
        className
      )}
    >
      {/* Header */}
      <div className="flex items-center justify-between border-b p-4">
        <div className="flex items-center gap-2">
          <Settings className="h-5 w-5" />
          <h2 className="font-semibold text-lg">Properties</h2>
        </div>
        {onClose && (
          <Button onClick={onClose} size="icon" variant="ghost">
            <X className="h-4 w-4" />
          </Button>
        )}
      </div>

      {/* Content */}
      <ScrollArea className="flex-1">
        <div className="space-y-6 p-4">
          {/* Node Info */}
          <div className="space-y-2">
            <Label htmlFor="node-title">Node Title</Label>
            <Input
              id="node-title"
              onChange={(e) => handleTitleChange(e.target.value)}
              placeholder="Enter node title"
              value={node.title}
            />
          </div>

          {/* Node Type */}
          <div className="space-y-2">
            <Label>Node Type</Label>
            <div className="rounded-md border bg-muted p-2 text-sm">
              {node.type}
            </div>
          </div>

          {/* Position */}
          <div className="space-y-2">
            <Label>Position</Label>
            <div className="grid grid-cols-2 gap-2">
              <div>
                <Label className="text-xs" htmlFor="pos-x">
                  X
                </Label>
                <Input
                  id="pos-x"
                  onChange={(e) =>
                    onUpdateNode?.(node.id, {
                      position: {
                        ...node.position,
                        x: Number.parseFloat(e.target.value) || 0,
                      },
                    })
                  }
                  type="number"
                  value={node.position.x}
                />
              </div>
              <div>
                <Label className="text-xs" htmlFor="pos-y">
                  Y
                </Label>
                <Input
                  id="pos-y"
                  onChange={(e) =>
                    onUpdateNode?.(node.id, {
                      position: {
                        ...node.position,
                        y: Number.parseFloat(e.target.value) || 0,
                      },
                    })
                  }
                  type="number"
                  value={node.position.y}
                />
              </div>
            </div>
          </div>

          {/* Custom Properties */}
          {Object.keys(localProperties).length > 0 && (
            <div className="space-y-2">
              <Label>Custom Properties</Label>
              <div className="space-y-2">
                {Object.entries(localProperties).map(([key, value]) => (
                  <div className="space-y-1" key={key}>
                    <Label className="text-xs" htmlFor={`prop-${key}`}>
                      {key}
                    </Label>
                    <Input
                      id={`prop-${key}`}
                      onChange={(e) => {
                        // Try to parse as number if possible
                        const numValue = Number.parseFloat(e.target.value);
                        const newValue =
                          !Number.isNaN(numValue) &&
                          e.target.value.trim() !== ""
                            ? numValue
                            : e.target.value;
                        handlePropertyChange(key, newValue);
                      }}
                      placeholder="Enter value"
                      value={String(value)}
                    />
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Pins Info */}
          <div className="space-y-2">
            <Label>Pins</Label>
            <div className="space-y-2">
              {node.inputs.length > 0 && (
                <div>
                  <div className="mb-1 font-medium text-muted-foreground text-xs">
                    Inputs ({node.inputs.length})
                  </div>
                  <div className="space-y-1">
                    {node.inputs.map((pin) => (
                      <div
                        className="rounded-md border bg-muted p-2 text-xs"
                        key={pin.id}
                      >
                        <div className="font-medium">{pin.name}</div>
                        <div className="text-muted-foreground">
                          Type: {pin.type}
                        </div>
                        {pin.required && (
                          <div className="text-muted-foreground">Required</div>
                        )}
                      </div>
                    ))}
                  </div>
                </div>
              )}
              {node.outputs.length > 0 && (
                <div>
                  <div className="mb-1 font-medium text-muted-foreground text-xs">
                    Outputs ({node.outputs.length})
                  </div>
                  <div className="space-y-1">
                    {node.outputs.map((pin) => (
                      <div
                        className="rounded-md border bg-muted p-2 text-xs"
                        key={pin.id}
                      >
                        <div className="font-medium">{pin.name}</div>
                        <div className="text-muted-foreground">
                          Type: {pin.type}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </ScrollArea>
    </div>
  );
}
