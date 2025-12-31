import { Lock, Unlock } from "lucide-react";
import { useState } from "react";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import { Separator } from "~/components/ui/separator";
import { Switch } from "~/components/ui/switch";
import { useSceneStore, useSelectedObject } from "~/stores/scene-store";
import { usePrecision, useStepValue } from "~/stores/settings-store";

export function PropertyPanel() {
  const selectedObject = useSelectedObject();
  const { updateObject } = useSceneStore();
  const [proportionalScale, setProportionalScale] = useState(false);
  const stepValue = useStepValue();
  const precision = usePrecision();

  if (!selectedObject) {
    return (
      <div className="p-4">
        <Card>
          <CardContent className="pt-6">
            <p className="text-center text-muted-foreground text-sm">
              Select an object to edit its properties
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  const handlePropertyChange = (
    property: string,
    value: string | number | boolean | [number, number, number] | undefined
  ) => {
    updateObject(selectedObject.id, { [property]: value });
  };

  const getAxisIndex = (axis: "x" | "y" | "z") => {
    switch (axis) {
      case "x":
        return 0;
      case "y":
        return 1;
      case "z":
        return 2;
      default:
        return 0;
    }
  };

  const handlePositionChange = (axis: "x" | "y" | "z", value: number) => {
    const axisIndex = getAxisIndex(axis);
    const newPosition = [
      axisIndex === 0 ? value : selectedObject.position[0],
      axisIndex === 1 ? value : selectedObject.position[1],
      axisIndex === 2 ? value : selectedObject.position[2],
    ] as [number, number, number];
    updateObject(selectedObject.id, { position: newPosition });
  };

  const handleRotationChange = (axis: "x" | "y" | "z", value: number) => {
    const axisIndex = getAxisIndex(axis);
    const newRotation = [
      axisIndex === 0 ? value : selectedObject.rotation[0],
      axisIndex === 1 ? value : selectedObject.rotation[1],
      axisIndex === 2 ? value : selectedObject.rotation[2],
    ] as [number, number, number];
    updateObject(selectedObject.id, { rotation: newRotation });
  };

  const handleScaleChange = (axis: "x" | "y" | "z", value: number) => {
    if (proportionalScale) {
      // Calculate proportional scaling based on the original scale
      const originalScale = selectedObject.scale;
      const currentValue = originalScale[getAxisIndex(axis)];
      const ratio = value / currentValue;

      const newScale: [number, number, number] = [
        originalScale[0] * ratio,
        originalScale[1] * ratio,
        originalScale[2] * ratio,
      ];

      updateObject(selectedObject.id, { scale: newScale });
    } else {
      // Individual axis scaling
      const axisIndex = getAxisIndex(axis);
      const newScale = [
        axisIndex === 0 ? value : selectedObject.scale[0],
        axisIndex === 1 ? value : selectedObject.scale[1],
        axisIndex === 2 ? value : selectedObject.scale[2],
      ] as [number, number, number];
      updateObject(selectedObject.id, { scale: newScale });
    }
  };

  return (
    <div className="space-y-4 p-4">
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="font-medium text-sm">
            Object Properties
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Object Name */}
          <div className="space-y-2">
            <Label className="font-medium text-xs" htmlFor="object-name">
              Name
            </Label>
            <Input
              className="h-8"
              id="object-name"
              onChange={(e) => handlePropertyChange("name", e.target.value)}
              value={selectedObject.name}
            />
          </div>

          {/* Object Type */}
          <div className="space-y-2">
            <Label className="font-medium text-xs">Type</Label>
            <div className="text-muted-foreground text-sm capitalize">
              {selectedObject.type}
            </div>
          </div>

          {/* Model ID */}
          <div className="space-y-2">
            <Label className="font-medium text-xs" htmlFor="model-id">
              Model ID (SAMP)
            </Label>
            <Input
              className="h-8"
              id="model-id"
              onChange={(e) => {
                const value = e.target.value;
                const numValue =
                  value === "" ? undefined : Number.parseInt(value, 10);
                handlePropertyChange("modelid", numValue);
              }}
              placeholder="Optional"
              type="number"
              value={selectedObject.modelid ?? ""}
            />
            <div className="text-muted-foreground text-xs">
              Set this to export object to SAMP format
            </div>
          </div>

          <Separator />

          {/* Transform Properties */}
          <div className="space-y-4">
            {/* Position */}
            <div className="space-y-2">
              <Label className="font-medium text-xs">Position</Label>
              <div className="grid grid-cols-3 gap-2">
                <div>
                  <Label
                    className="text-muted-foreground text-xs"
                    htmlFor="pos-x"
                  >
                    X
                  </Label>
                  <Input
                    className="h-8"
                    id="pos-x"
                    onChange={(e) =>
                      handlePositionChange(
                        "x",
                        Number.parseFloat(e.target.value) || 0
                      )
                    }
                    step={stepValue}
                    type="number"
                    value={selectedObject.position[0].toFixed(precision)}
                  />
                </div>
                <div>
                  <Label
                    className="text-muted-foreground text-xs"
                    htmlFor="pos-y"
                  >
                    Y
                  </Label>
                  <Input
                    className="h-8"
                    id="pos-y"
                    onChange={(e) =>
                      handlePositionChange(
                        "y",
                        Number.parseFloat(e.target.value) || 0
                      )
                    }
                    step={stepValue}
                    type="number"
                    value={selectedObject.position[1].toFixed(precision)}
                  />
                </div>
                <div>
                  <Label
                    className="text-muted-foreground text-xs"
                    htmlFor="pos-z"
                  >
                    Z
                  </Label>
                  <Input
                    className="h-8"
                    id="pos-z"
                    onChange={(e) =>
                      handlePositionChange(
                        "z",
                        Number.parseFloat(e.target.value) || 0
                      )
                    }
                    step={stepValue}
                    type="number"
                    value={selectedObject.position[2].toFixed(precision)}
                  />
                </div>
              </div>
            </div>

            {/* Rotation */}
            <div className="space-y-2">
              <Label className="font-medium text-xs">Rotation</Label>
              <div className="grid grid-cols-3 gap-2">
                <div>
                  <Label
                    className="text-muted-foreground text-xs"
                    htmlFor="rot-x"
                  >
                    X
                  </Label>
                  <Input
                    className="h-8"
                    id="rot-x"
                    onChange={(e) =>
                      handleRotationChange(
                        "x",
                        Number.parseFloat(e.target.value) || 0
                      )
                    }
                    step={stepValue}
                    type="number"
                    value={selectedObject.rotation[0].toFixed(precision)}
                  />
                </div>
                <div>
                  <Label
                    className="text-muted-foreground text-xs"
                    htmlFor="rot-y"
                  >
                    Y
                  </Label>
                  <Input
                    className="h-8"
                    id="rot-y"
                    onChange={(e) =>
                      handleRotationChange(
                        "y",
                        Number.parseFloat(e.target.value) || 0
                      )
                    }
                    step={stepValue}
                    type="number"
                    value={selectedObject.rotation[1].toFixed(precision)}
                  />
                </div>
                <div>
                  <Label
                    className="text-muted-foreground text-xs"
                    htmlFor="rot-z"
                  >
                    Z
                  </Label>
                  <Input
                    className="h-8"
                    id="rot-z"
                    onChange={(e) =>
                      handleRotationChange(
                        "z",
                        Number.parseFloat(e.target.value) || 0
                      )
                    }
                    step={stepValue}
                    type="number"
                    value={selectedObject.rotation[2].toFixed(precision)}
                  />
                </div>
              </div>
            </div>

            {/* Scale */}
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label className="font-medium text-xs">Scale</Label>
                <Button
                  className="h-6 w-6 p-0"
                  onClick={() => setProportionalScale(!proportionalScale)}
                  size="sm"
                  variant="ghost"
                >
                  {proportionalScale ? (
                    <Lock className="h-3 w-3" />
                  ) : (
                    <Unlock className="h-3 w-3" />
                  )}
                </Button>
              </div>
              <div className="grid grid-cols-3 gap-2">
                <div>
                  <Label
                    className="text-muted-foreground text-xs"
                    htmlFor="scale-x"
                  >
                    X
                  </Label>
                  <Input
                    className="h-8"
                    id="scale-x"
                    min="0.1"
                    onChange={(e) =>
                      handleScaleChange(
                        "x",
                        Number.parseFloat(e.target.value) || 1
                      )
                    }
                    step={stepValue}
                    type="number"
                    value={selectedObject.scale[0].toFixed(precision)}
                  />
                </div>
                <div>
                  <Label
                    className="text-muted-foreground text-xs"
                    htmlFor="scale-y"
                  >
                    Y
                  </Label>
                  <Input
                    className="h-8"
                    id="scale-y"
                    min="0.1"
                    onChange={(e) =>
                      handleScaleChange(
                        "y",
                        Number.parseFloat(e.target.value) || 1
                      )
                    }
                    step={stepValue}
                    type="number"
                    value={selectedObject.scale[1].toFixed(precision)}
                  />
                </div>
                <div>
                  <Label
                    className="text-muted-foreground text-xs"
                    htmlFor="scale-z"
                  >
                    Z
                  </Label>
                  <Input
                    className="h-8"
                    id="scale-z"
                    min="0.1"
                    onChange={(e) =>
                      handleScaleChange(
                        "z",
                        Number.parseFloat(e.target.value) || 1
                      )
                    }
                    step={stepValue}
                    type="number"
                    value={selectedObject.scale[2].toFixed(precision)}
                  />
                </div>
              </div>
            </div>
          </div>

          <Separator />

          {/* Visual Properties */}
          <div className="space-y-4">
            {/* Color */}
            <div className="space-y-2">
              <Label className="font-medium text-xs" htmlFor="object-color">
                Color
              </Label>
              <Input
                className="h-8 w-full"
                id="object-color"
                onChange={(e) => handlePropertyChange("color", e.target.value)}
                type="color"
                value={selectedObject.color}
              />
            </div>

            {/* Visibility */}
            <div className="flex items-center justify-between">
              <Label className="font-medium text-xs" htmlFor="object-visible">
                Visible
              </Label>
              <Switch
                checked={selectedObject.visible}
                id="object-visible"
                onCheckedChange={(checked) =>
                  handlePropertyChange("visible", checked)
                }
              />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
