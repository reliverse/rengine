import { Trash2 } from "lucide-react";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import { Separator } from "~/components/ui/separator";
import { Slider } from "~/components/ui/slider";
import { Switch } from "~/components/ui/switch";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { useSceneStore, useSelectedLight } from "~/stores/scene-store";
import { usePrecision, useStepValue } from "~/stores/settings-store";

export function LightPropertyPanel() {
  const selectedLight = useSelectedLight();
  const { updateLight, removeLight } = useSceneStore();
  const precision = usePrecision();
  const stepValue = useStepValue();

  if (!selectedLight) {
    return (
      <div className="p-4">
        <Card>
          <CardContent className="pt-6">
            <p className="text-center text-muted-foreground text-sm">
              Select a light to edit its properties
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  const handlePropertyChange = (
    property: string,
    value: string | number | boolean | [number, number, number]
  ) => {
    updateLight(selectedLight.id, { [property]: value });
  };

  const handlePositionChange = (axis: "x" | "y" | "z", value: number) => {
    const axisIndex = { x: 0, y: 1, z: 2 }[axis];
    const newPosition = [
      axisIndex === 0 ? value : selectedLight.position[0],
      axisIndex === 1 ? value : selectedLight.position[1],
      axisIndex === 2 ? value : selectedLight.position[2],
    ] as [number, number, number];
    updateLight(selectedLight.id, { position: newPosition });
  };

  const handleTargetChange = (axis: "x" | "y" | "z", value: number) => {
    if (!selectedLight.target) {
      return;
    }
    const axisIndex = { x: 0, y: 1, z: 2 }[axis];
    const newTarget = [
      axisIndex === 0 ? value : selectedLight.target[0],
      axisIndex === 1 ? value : selectedLight.target[1],
      axisIndex === 2 ? value : selectedLight.target[2],
    ] as [number, number, number];
    updateLight(selectedLight.id, { target: newTarget });
  };

  const handleShadowPropertyChange = (property: string, value: number) => {
    updateLight(selectedLight.id, { [property]: value });
  };

  return (
    <div className="space-y-4 p-4">
      <Card>
        <CardHeader className="pb-3">
          <div className="flex items-center justify-between">
            <CardTitle className="font-medium text-sm">
              Light Properties
            </CardTitle>
            <Button
              className="h-6 w-6 p-0 text-destructive hover:text-destructive"
              onClick={() => removeLight(selectedLight.id)}
              size="sm"
              variant="ghost"
            >
              <Trash2 className="h-3 w-3" />
            </Button>
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Light Name */}
          <div className="space-y-2">
            <Label className="font-medium text-xs" htmlFor="light-name">
              Name
            </Label>
            <Input
              className="h-8"
              id="light-name"
              onChange={(e) => handlePropertyChange("name", e.target.value)}
              value={selectedLight.name}
            />
          </div>

          {/* Light Type */}
          <div className="space-y-2">
            <Label className="font-medium text-xs">Type</Label>
            <div className="text-muted-foreground text-sm capitalize">
              {selectedLight.type}
            </div>
          </div>

          <Separator />

          <Tabs className="w-full" defaultValue="basic">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="basic">Basic</TabsTrigger>
              <TabsTrigger value="advanced">Advanced</TabsTrigger>
            </TabsList>

            <TabsContent className="space-y-4" value="basic">
              {/* Basic Properties */}
              <div className="space-y-4">
                {/* Color */}
                <div className="space-y-2">
                  <Label className="font-medium text-xs" htmlFor="light-color">
                    Color
                  </Label>
                  <div className="flex gap-2">
                    <Input
                      className="h-8 flex-1"
                      id="light-color"
                      onChange={(e) =>
                        handlePropertyChange("color", e.target.value)
                      }
                      type="color"
                      value={selectedLight.color}
                    />
                    <Input
                      className="h-8 w-20"
                      onChange={(e) =>
                        handlePropertyChange("color", e.target.value)
                      }
                      type="text"
                      value={selectedLight.color}
                    />
                  </div>
                </div>

                {/* Intensity */}
                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <Label className="font-medium text-xs">Intensity</Label>
                    <span className="text-muted-foreground text-xs">
                      {selectedLight.intensity.toFixed(precision)}
                    </span>
                  </div>
                  <Slider
                    className="w-full"
                    max={10}
                    min={0}
                    onValueChange={(value) =>
                      handlePropertyChange(
                        "intensity",
                        Array.isArray(value) ? value[0] : (value ?? 1)
                      )
                    }
                    step={0.1}
                    value={[selectedLight.intensity] as [number]}
                  />
                </div>

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
                        value={(selectedLight.position[0] ?? 0).toFixed(
                          precision
                        )}
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
                        value={(selectedLight.position[1] ?? 0).toFixed(
                          precision
                        )}
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
                        value={(selectedLight.position[2] ?? 0).toFixed(
                          precision
                        )}
                      />
                    </div>
                  </div>
                </div>

                {/* Target (for directional and spot lights) */}
                {(selectedLight.type === "directional" ||
                  selectedLight.type === "spot") &&
                  selectedLight.target && (
                    <div className="space-y-2">
                      <Label className="font-medium text-xs">Target</Label>
                      <div className="grid grid-cols-3 gap-2">
                        <div>
                          <Label
                            className="text-muted-foreground text-xs"
                            htmlFor="target-x"
                          >
                            X
                          </Label>
                          <Input
                            className="h-8"
                            id="target-x"
                            onChange={(e) =>
                              handleTargetChange(
                                "x",
                                Number.parseFloat(e.target.value) || 0
                              )
                            }
                            step={stepValue}
                            type="number"
                            value={(selectedLight.target?.[0] ?? 0).toFixed(
                              precision
                            )}
                          />
                        </div>
                        <div>
                          <Label
                            className="text-muted-foreground text-xs"
                            htmlFor="target-y"
                          >
                            Y
                          </Label>
                          <Input
                            className="h-8"
                            id="target-y"
                            onChange={(e) =>
                              handleTargetChange(
                                "y",
                                Number.parseFloat(e.target.value) || 0
                              )
                            }
                            step={stepValue}
                            type="number"
                            value={(selectedLight.target?.[1] ?? 0).toFixed(
                              precision
                            )}
                          />
                        </div>
                        <div>
                          <Label
                            className="text-muted-foreground text-xs"
                            htmlFor="target-z"
                          >
                            Z
                          </Label>
                          <Input
                            className="h-8"
                            id="target-z"
                            onChange={(e) =>
                              handleTargetChange(
                                "z",
                                Number.parseFloat(e.target.value) || 0
                              )
                            }
                            step={stepValue}
                            type="number"
                            value={(selectedLight.target?.[2] ?? 0).toFixed(
                              precision
                            )}
                          />
                        </div>
                      </div>
                    </div>
                  )}

                {/* Visibility */}
                <div className="flex items-center justify-between">
                  <Label
                    className="font-medium text-xs"
                    htmlFor="light-visible"
                  >
                    Visible
                  </Label>
                  <Switch
                    checked={selectedLight.visible}
                    id="light-visible"
                    onCheckedChange={(checked) =>
                      handlePropertyChange("visible", checked)
                    }
                  />
                </div>
              </div>
            </TabsContent>

            <TabsContent className="space-y-4" value="advanced">
              {/* Advanced Properties */}
              <div className="space-y-4">
                {/* Cast Shadow */}
                <div className="flex items-center justify-between">
                  <Label className="font-medium text-xs" htmlFor="cast-shadow">
                    Cast Shadow
                  </Label>
                  <Switch
                    checked={selectedLight.castShadow}
                    id="cast-shadow"
                    onCheckedChange={(checked) =>
                      handlePropertyChange("castShadow", checked)
                    }
                  />
                </div>

                {/* Type-specific properties */}
                {selectedLight.type === "point" && (
                  <>
                    {/* Distance */}
                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <Label className="font-medium text-xs">Distance</Label>
                        <span className="text-muted-foreground text-xs">
                          {selectedLight.distance === 0
                            ? "Infinite"
                            : selectedLight.distance?.toFixed(1)}
                        </span>
                      </div>
                      <Slider
                        className="w-full"
                        max={100}
                        min={0}
                        onValueChange={(value) =>
                          handlePropertyChange(
                            "distance",
                            Array.isArray(value) ? value[0] : (value ?? 0)
                          )
                        }
                        step={1}
                        value={[selectedLight.distance || 0] as [number]}
                      />
                    </div>

                    {/* Decay */}
                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <Label className="font-medium text-xs">Decay</Label>
                        <span className="text-muted-foreground text-xs">
                          {selectedLight.decay?.toFixed(precision)}
                        </span>
                      </div>
                      <Slider
                        className="w-full"
                        max={5}
                        min={0}
                        onValueChange={(value) =>
                          handlePropertyChange(
                            "decay",
                            Array.isArray(value) ? value[0] : (value ?? 0)
                          )
                        }
                        step={0.1}
                        value={[selectedLight.decay || 2] as [number]}
                      />
                    </div>
                  </>
                )}

                {selectedLight.type === "spot" && (
                  <>
                    {/* Angle */}
                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <Label className="font-medium text-xs">Angle</Label>
                        <span className="text-muted-foreground text-xs">
                          {(
                            (selectedLight.angle ?? Math.PI / 6) *
                            (180 / Math.PI)
                          ).toFixed(1)}
                          °
                        </span>
                      </div>
                      <Slider
                        className="w-full"
                        max={90}
                        min={1}
                        onValueChange={(value) =>
                          handlePropertyChange(
                            "angle",
                            ((Array.isArray(value) ? value[0] : (value ?? 0)) *
                              Math.PI) /
                              180
                          )
                        }
                        step={1}
                        value={[
                          (selectedLight.angle ?? Math.PI / 6) *
                            (180 / Math.PI),
                        ]}
                      />
                    </div>

                    {/* Penumbra */}
                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <Label className="font-medium text-xs">Penumbra</Label>
                        <span className="text-muted-foreground text-xs">
                          {((selectedLight.penumbra ?? 0.1) * 100).toFixed(0)}%
                        </span>
                      </div>
                      <Slider
                        className="w-full"
                        max={100}
                        min={0}
                        onValueChange={(value) =>
                          handlePropertyChange(
                            "penumbra",
                            Array.isArray(value) ? value[0] : (value ?? 0 / 100)
                          )
                        }
                        step={1}
                        value={
                          [(selectedLight.penumbra ?? 0.1) * 100] as [number]
                        }
                      />
                    </div>

                    {/* Distance */}
                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <Label className="font-medium text-xs">Distance</Label>
                        <span className="text-muted-foreground text-xs">
                          {selectedLight.distance === 0
                            ? "Infinite"
                            : selectedLight.distance?.toFixed(1)}
                        </span>
                      </div>
                      <Slider
                        className="w-full"
                        max={100}
                        min={0}
                        onValueChange={(value) =>
                          handlePropertyChange(
                            "distance",
                            Array.isArray(value) ? value[0] : (value ?? 0)
                          )
                        }
                        step={1}
                        value={[selectedLight.distance || 0] as [number]}
                      />
                    </div>

                    {/* Decay */}
                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <Label className="font-medium text-xs">Decay</Label>
                        <span className="text-muted-foreground text-xs">
                          {selectedLight.decay?.toFixed(precision)}
                        </span>
                      </div>
                      <Slider
                        className="w-full"
                        max={5}
                        min={0}
                        onValueChange={(value) =>
                          handlePropertyChange(
                            "decay",
                            Array.isArray(value) ? value[0] : (value ?? 0)
                          )
                        }
                        step={0.1}
                        value={[selectedLight.decay || 2] as [number]}
                      />
                    </div>
                  </>
                )}

                {selectedLight.type === "hemisphere" && (
                  <>
                    {/* Ground Color */}
                    <div className="space-y-2">
                      <Label
                        className="font-medium text-xs"
                        htmlFor="ground-color"
                      >
                        Ground Color
                      </Label>
                      <div className="flex gap-2">
                        <Input
                          className="h-8 flex-1"
                          id="ground-color"
                          onChange={(e) =>
                            handlePropertyChange("groundColor", e.target.value)
                          }
                          type="color"
                          value={selectedLight.groundColor || "#444444"}
                        />
                        <Input
                          className="h-8 w-20"
                          onChange={(e) =>
                            handlePropertyChange("groundColor", e.target.value)
                          }
                          type="text"
                          value={selectedLight.groundColor || "#444444"}
                        />
                      </div>
                    </div>
                  </>
                )}

                {/* Shadow Properties */}
                {selectedLight.castShadow && (
                  <>
                    <Separator />
                    <div className="space-y-4">
                      <Label className="font-medium text-xs">
                        Shadow Settings
                      </Label>

                      {/* Shadow Bias */}
                      <div className="space-y-2">
                        <div className="flex items-center justify-between">
                          <Label className="font-medium text-xs">
                            Shadow Bias
                          </Label>
                          <span className="text-muted-foreground text-xs">
                            {selectedLight.shadowBias?.toFixed(4)}
                          </span>
                        </div>
                        <Slider
                          className="w-full"
                          max={0.01}
                          min={-0.01}
                          onValueChange={(value) =>
                            handleShadowPropertyChange(
                              "shadowBias",
                              Array.isArray(value) ? value[0] : (value ?? 0)
                            )
                          }
                          step={0.0001}
                          value={
                            [selectedLight.shadowBias || -0.0001] as [number]
                          }
                        />
                      </div>

                      {/* Shadow Map Size */}
                      <div className="space-y-2">
                        <div className="flex items-center justify-between">
                          <Label className="font-medium text-xs">
                            Shadow Map Size
                          </Label>
                          <span className="text-muted-foreground text-xs">
                            {selectedLight.shadowMapSize || 1024}
                          </span>
                        </div>
                        <Slider
                          className="w-full"
                          max={4096}
                          min={256}
                          onValueChange={(value) =>
                            handleShadowPropertyChange(
                              "shadowMapSize",
                              Array.isArray(value) ? value[0] : (value ?? 0)
                            )
                          }
                          step={256}
                          value={
                            [selectedLight.shadowMapSize || 1024] as [number]
                          }
                        />
                      </div>

                      {/* Shadow Near */}
                      <div className="space-y-2">
                        <div className="flex items-center justify-between">
                          <Label className="font-medium text-xs">
                            Shadow Near
                          </Label>
                          <span className="text-muted-foreground text-xs">
                            {selectedLight.shadowNear?.toFixed(precision)}
                          </span>
                        </div>
                        <Slider
                          className="w-full"
                          max={10}
                          min={0.01}
                          onValueChange={(value) =>
                            handleShadowPropertyChange(
                              "shadowNear",
                              Array.isArray(value) ? value[0] : (value ?? 0)
                            )
                          }
                          step={0.1}
                          value={[selectedLight.shadowNear || 0.1] as [number]}
                        />
                      </div>

                      {/* Shadow Far */}
                      <div className="space-y-2">
                        <div className="flex items-center justify-between">
                          <Label className="font-medium text-xs">
                            Shadow Far
                          </Label>
                          <span className="text-muted-foreground text-xs">
                            {selectedLight.shadowFar?.toFixed(1)}
                          </span>
                        </div>
                        <Slider
                          className="w-full"
                          max={1000}
                          min={1}
                          onValueChange={(value) =>
                            handleShadowPropertyChange(
                              "shadowFar",
                              Array.isArray(value) ? value[0] : (value ?? 0)
                            )
                          }
                          step={1}
                          value={[selectedLight.shadowFar || 50] as [number]}
                        />
                      </div>

                      {/* Shadow Radius */}
                      <div className="space-y-2">
                        <div className="flex items-center justify-between">
                          <Label className="font-medium text-xs">
                            Shadow Radius
                          </Label>
                          <span className="text-muted-foreground text-xs">
                            {selectedLight.shadowRadius?.toFixed(1)}
                          </span>
                        </div>
                        <Slider
                          className="w-full"
                          max={20}
                          min={0}
                          onValueChange={(value) =>
                            handleShadowPropertyChange(
                              "shadowRadius",
                              Array.isArray(value) ? value[0] : (value ?? 0)
                            )
                          }
                          step={0.1}
                          value={[selectedLight.shadowRadius || 4] as [number]}
                        />
                      </div>
                    </div>
                  </>
                )}
              </div>
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
}
