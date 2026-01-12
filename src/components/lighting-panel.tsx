import { useState } from "react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "~/components/ui/accordion";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import { ScrollArea } from "~/components/ui/scroll-area";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { Slider } from "~/components/ui/slider";
import { Switch } from "~/components/ui/switch";
import { useSceneStore } from "~/stores/scene-store";
import { LIGHTING_PRESETS } from "~/utils/lighting-presets";

export function LightingPanel() {
  const [selectedPreset, setSelectedPreset] = useState("default");

  const {
    addLight,
    clearLights,
    lights,
    updateLight,
    lightsVisible,
    setLightsVisible,
    backgroundColor,
    setBackgroundColor,
    fogEnabled,
    setFogEnabled,
    fogColor,
    setFogColor,
    fogNear,
    setFogNear,
    fogFar,
    setFogFar,
    axesVisible,
    setAxesVisible,
    statsVisible,
    setStatsVisible,
    gridVisible,
    setGridVisible,
  } = useSceneStore();

  // Light Controls - control individual lights (first light for now)
  const selectedLight = lights[0];
  const shadowLight = selectedLight?.castShadow ? selectedLight : null;

  const presetOptions = Object.keys(LIGHTING_PRESETS);

  const applyPreset = () => {
    if (LIGHTING_PRESETS[selectedPreset as keyof typeof LIGHTING_PRESETS]) {
      const preset =
        LIGHTING_PRESETS[selectedPreset as keyof typeof LIGHTING_PRESETS];
      clearLights();
      for (const light of preset.lights) {
        const { target, ...restLight } = light as any;
        const lightConfig = {
          ...restLight,
          position: [...light.position] as [number, number, number],
        };
        if (target) {
          lightConfig.target = [...target] as [number, number, number];
        }
        addLight(lightConfig);
      }
    }
  };

  return (
    <ScrollArea className="h-full">
      <div className="space-y-4 p-4">
        {/* Lighting Presets */}
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">🎯 Lighting Presets</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="flex items-center gap-2">
              <Label className="text-xs" htmlFor="preset-select">
                Preset:
              </Label>
              <Select
                onValueChange={(value) => value && setSelectedPreset(value)}
                value={selectedPreset}
              >
                <SelectTrigger className="flex-1" id="preset-select">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {presetOptions.map((preset) => (
                    <SelectItem key={preset} value={preset}>
                      {preset}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <Button className="w-full" onClick={applyPreset} size="sm">
              Apply Preset
            </Button>
          </CardContent>
        </Card>

        {/* Add New Lights */}
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">➕ Add Light</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            <Button
              className="w-full justify-start"
              onClick={() =>
                addLight({
                  type: "ambient",
                  color: "#ffffff",
                  intensity: 0.6,
                  position: [0, 0, 0],
                  visible: true,
                  castShadow: false,
                })
              }
              size="sm"
              variant="outline"
            >
              Ambient Light
            </Button>
            <Button
              className="w-full justify-start"
              onClick={() =>
                addLight({
                  type: "directional",
                  color: "#ffffff",
                  intensity: 1,
                  position: [5, 5, 5],
                  target: [0, 0, 0],
                  visible: true,
                  castShadow: true,
                })
              }
              size="sm"
              variant="outline"
            >
              Directional Light
            </Button>
            <Button
              className="w-full justify-start"
              onClick={() =>
                addLight({
                  type: "point",
                  color: "#ffffff",
                  intensity: 1,
                  position: [2, 2, 2],
                  visible: true,
                  castShadow: true,
                })
              }
              size="sm"
              variant="outline"
            >
              Point Light
            </Button>
            <Button
              className="w-full justify-start"
              onClick={() =>
                addLight({
                  type: "spot",
                  color: "#ffffff",
                  intensity: 1,
                  position: [3, 3, 3],
                  target: [0, 0, 0],
                  visible: true,
                  castShadow: true,
                })
              }
              size="sm"
              variant="outline"
            >
              Spot Light
            </Button>
            <Button
              className="w-full justify-start"
              onClick={() =>
                addLight({
                  type: "hemisphere",
                  color: "#87CEEB",
                  intensity: 0.6,
                  position: [0, 0, 0],
                  visible: true,
                  castShadow: false,
                })
              }
              size="sm"
              variant="outline"
            >
              Hemisphere Light
            </Button>
          </CardContent>
        </Card>

        {/* Individual Light Controls */}
        {selectedLight && (
          <Accordion className="space-y-2">
            {/* Ambient Light Controls */}
            {selectedLight.type === "ambient" && (
              <AccordionItem value="ambient">
                <AccordionTrigger className="text-sm">
                  💡 Ambient Light
                </AccordionTrigger>
                <AccordionContent className="space-y-4">
                  <div className="flex items-center justify-between">
                    <Label className="text-xs" htmlFor="ambient-visible">
                      Visible
                    </Label>
                    <Switch
                      checked={selectedLight.visible}
                      id="ambient-visible"
                      onCheckedChange={(checked) =>
                        updateLight(selectedLight.id, { visible: checked })
                      }
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs" htmlFor="ambient-color">
                      Color
                    </Label>
                    <Input
                      className="h-8"
                      id="ambient-color"
                      onChange={(e) =>
                        updateLight(selectedLight.id, { color: e.target.value })
                      }
                      type="color"
                      value={selectedLight.color}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Intensity: {selectedLight.intensity.toFixed(1)}
                    </Label>
                    <Slider
                      className="w-full"
                      max={10}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, {
                          intensity: numericValue,
                        });
                      }}
                      step={0.1}
                      value={[selectedLight.intensity]}
                    />
                  </div>
                </AccordionContent>
              </AccordionItem>
            )}

            {/* Directional Light Controls */}
            {selectedLight.type === "directional" && (
              <AccordionItem value="directional">
                <AccordionTrigger className="text-sm">
                  💡 Directional Light
                </AccordionTrigger>
                <AccordionContent className="space-y-4">
                  <div className="flex items-center justify-between">
                    <Label className="text-xs" htmlFor="directional-visible">
                      Visible
                    </Label>
                    <Switch
                      checked={selectedLight.visible}
                      id="directional-visible"
                      onCheckedChange={(checked) =>
                        updateLight(selectedLight.id, { visible: checked })
                      }
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs" htmlFor="directional-color">
                      Color
                    </Label>
                    <Input
                      className="h-8"
                      id="directional-color"
                      onChange={(e) =>
                        updateLight(selectedLight.id, { color: e.target.value })
                      }
                      type="color"
                      value={selectedLight.color}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Intensity: {selectedLight.intensity.toFixed(1)}
                    </Label>
                    <Slider
                      className="w-full"
                      max={10}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, {
                          intensity: numericValue,
                        });
                      }}
                      step={0.1}
                      value={[selectedLight.intensity]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">Position</Label>
                    <div className="grid grid-cols-3 gap-2">
                      <Input
                        onChange={(e) =>
                          updateLight(selectedLight.id, {
                            position: [
                              Number.parseFloat(e.target.value) || 0,
                              selectedLight.position[1],
                              selectedLight.position[2],
                            ],
                          })
                        }
                        placeholder="X"
                        step="0.1"
                        type="number"
                        value={selectedLight.position[0]}
                      />
                      <Input
                        onChange={(e) =>
                          updateLight(selectedLight.id, {
                            position: [
                              selectedLight.position[0],
                              Number.parseFloat(e.target.value) || 0,
                              selectedLight.position[2],
                            ],
                          })
                        }
                        placeholder="Y"
                        step="0.1"
                        type="number"
                        value={selectedLight.position[1]}
                      />
                      <Input
                        onChange={(e) =>
                          updateLight(selectedLight.id, {
                            position: [
                              selectedLight.position[0],
                              selectedLight.position[1],
                              Number.parseFloat(e.target.value) || 0,
                            ],
                          })
                        }
                        placeholder="Z"
                        step="0.1"
                        type="number"
                        value={selectedLight.position[2]}
                      />
                    </div>
                  </div>

                  {selectedLight.target && (
                    <div className="space-y-2">
                      <Label className="text-xs">Target</Label>
                      <div className="grid grid-cols-3 gap-2">
                        <Input
                          onChange={(e) =>
                            updateLight(selectedLight.id, {
                              target: [
                                Number.parseFloat(e.target.value) || 0,
                                selectedLight.target?.[1] ?? 0,
                                selectedLight.target?.[2] ?? 0,
                              ],
                            })
                          }
                          placeholder="X"
                          step="0.1"
                          type="number"
                          value={selectedLight.target?.[0] ?? 0}
                        />
                        <Input
                          onChange={(e) =>
                            updateLight(selectedLight.id, {
                              target: [
                                selectedLight.target?.[0] ?? 0,
                                Number.parseFloat(e.target.value) || 0,
                                selectedLight.target?.[2] ?? 0,
                              ],
                            })
                          }
                          placeholder="Y"
                          step="0.1"
                          type="number"
                          value={selectedLight.target?.[1] ?? 0}
                        />
                        <Input
                          onChange={(e) =>
                            updateLight(selectedLight.id, {
                              target: [
                                selectedLight.target?.[0] ?? 0,
                                selectedLight.target?.[1] ?? 0,
                                Number.parseFloat(e.target.value) || 0,
                              ],
                            })
                          }
                          placeholder="Z"
                          step="0.1"
                          type="number"
                          value={selectedLight.target?.[2] ?? 0}
                        />
                      </div>
                    </div>
                  )}

                  <div className="flex items-center justify-between">
                    <Label className="text-xs" htmlFor="directional-shadow">
                      Cast Shadow
                    </Label>
                    <Switch
                      checked={selectedLight.castShadow}
                      id="directional-shadow"
                      onCheckedChange={(checked) =>
                        updateLight(selectedLight.id, { castShadow: checked })
                      }
                    />
                  </div>
                </AccordionContent>
              </AccordionItem>
            )}

            {/* Point Light Controls */}
            {selectedLight.type === "point" && (
              <AccordionItem value="point">
                <AccordionTrigger className="text-sm">
                  💡 Point Light
                </AccordionTrigger>
                <AccordionContent className="space-y-4">
                  <div className="flex items-center justify-between">
                    <Label className="text-xs" htmlFor="point-visible">
                      Visible
                    </Label>
                    <Switch
                      checked={selectedLight.visible}
                      id="point-visible"
                      onCheckedChange={(checked) =>
                        updateLight(selectedLight.id, { visible: checked })
                      }
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs" htmlFor="point-color">
                      Color
                    </Label>
                    <Input
                      className="h-8"
                      id="point-color"
                      onChange={(e) =>
                        updateLight(selectedLight.id, { color: e.target.value })
                      }
                      type="color"
                      value={selectedLight.color}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Intensity: {selectedLight.intensity.toFixed(1)}
                    </Label>
                    <Slider
                      className="w-full"
                      max={10}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, {
                          intensity: numericValue,
                        });
                      }}
                      step={0.1}
                      value={[selectedLight.intensity]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">Position</Label>
                    <div className="grid grid-cols-3 gap-2">
                      <Input
                        onChange={(e) =>
                          updateLight(selectedLight.id, {
                            position: [
                              Number.parseFloat(e.target.value) || 0,
                              selectedLight.position[1],
                              selectedLight.position[2],
                            ],
                          })
                        }
                        placeholder="X"
                        step="0.1"
                        type="number"
                        value={selectedLight.position[0]}
                      />
                      <Input
                        onChange={(e) =>
                          updateLight(selectedLight.id, {
                            position: [
                              selectedLight.position[0],
                              Number.parseFloat(e.target.value) || 0,
                              selectedLight.position[2],
                            ],
                          })
                        }
                        placeholder="Y"
                        step="0.1"
                        type="number"
                        value={selectedLight.position[1]}
                      />
                      <Input
                        onChange={(e) =>
                          updateLight(selectedLight.id, {
                            position: [
                              selectedLight.position[0],
                              selectedLight.position[1],
                              Number.parseFloat(e.target.value) || 0,
                            ],
                          })
                        }
                        placeholder="Z"
                        step="0.1"
                        type="number"
                        value={selectedLight.position[2]}
                      />
                    </div>
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Distance: {selectedLight.distance || 0}
                    </Label>
                    <Slider
                      className="w-full"
                      max={100}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, {
                          distance: numericValue,
                        });
                      }}
                      step={1}
                      value={[selectedLight.distance || 0]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Decay: {(selectedLight.decay || 2).toFixed(1)}
                    </Label>
                    <Slider
                      className="w-full"
                      max={5}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, { decay: numericValue });
                      }}
                      step={0.1}
                      value={[selectedLight.decay || 2]}
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <Label className="text-xs" htmlFor="point-shadow">
                      Cast Shadow
                    </Label>
                    <Switch
                      checked={selectedLight.castShadow}
                      id="point-shadow"
                      onCheckedChange={(checked) =>
                        updateLight(selectedLight.id, { castShadow: checked })
                      }
                    />
                  </div>
                </AccordionContent>
              </AccordionItem>
            )}

            {/* Spot Light Controls */}
            {selectedLight.type === "spot" && (
              <AccordionItem value="spot">
                <AccordionTrigger className="text-sm">
                  💡 Spot Light
                </AccordionTrigger>
                <AccordionContent className="space-y-4">
                  <div className="flex items-center justify-between">
                    <Label className="text-xs" htmlFor="spot-visible">
                      Visible
                    </Label>
                    <Switch
                      checked={selectedLight.visible}
                      id="spot-visible"
                      onCheckedChange={(checked) =>
                        updateLight(selectedLight.id, { visible: checked })
                      }
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs" htmlFor="spot-color">
                      Color
                    </Label>
                    <Input
                      className="h-8"
                      id="spot-color"
                      onChange={(e) =>
                        updateLight(selectedLight.id, { color: e.target.value })
                      }
                      type="color"
                      value={selectedLight.color}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Intensity: {selectedLight.intensity.toFixed(1)}
                    </Label>
                    <Slider
                      className="w-full"
                      max={10}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, {
                          intensity: numericValue,
                        });
                      }}
                      step={0.1}
                      value={[selectedLight.intensity]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">Position</Label>
                    <div className="grid grid-cols-3 gap-2">
                      <Input
                        onChange={(e) =>
                          updateLight(selectedLight.id, {
                            position: [
                              Number.parseFloat(e.target.value) || 0,
                              selectedLight.position[1],
                              selectedLight.position[2],
                            ],
                          })
                        }
                        placeholder="X"
                        step="0.1"
                        type="number"
                        value={selectedLight.position[0]}
                      />
                      <Input
                        onChange={(e) =>
                          updateLight(selectedLight.id, {
                            position: [
                              selectedLight.position[0],
                              Number.parseFloat(e.target.value) || 0,
                              selectedLight.position[2],
                            ],
                          })
                        }
                        placeholder="Y"
                        step="0.1"
                        type="number"
                        value={selectedLight.position[1]}
                      />
                      <Input
                        onChange={(e) =>
                          updateLight(selectedLight.id, {
                            position: [
                              selectedLight.position[0],
                              selectedLight.position[1],
                              Number.parseFloat(e.target.value) || 0,
                            ],
                          })
                        }
                        placeholder="Z"
                        step="0.1"
                        type="number"
                        value={selectedLight.position[2]}
                      />
                    </div>
                  </div>

                  {selectedLight.target && (
                    <div className="space-y-2">
                      <Label className="text-xs">Target</Label>
                      <div className="grid grid-cols-3 gap-2">
                        <Input
                          onChange={(e) =>
                            updateLight(selectedLight.id, {
                              target: [
                                Number.parseFloat(e.target.value) || 0,
                                selectedLight.target?.[1] ?? 0,
                                selectedLight.target?.[2] ?? 0,
                              ],
                            })
                          }
                          placeholder="X"
                          step="0.1"
                          type="number"
                          value={selectedLight.target?.[0] ?? 0}
                        />
                        <Input
                          onChange={(e) =>
                            updateLight(selectedLight.id, {
                              target: [
                                selectedLight.target?.[0] ?? 0,
                                Number.parseFloat(e.target.value) || 0,
                                selectedLight.target?.[2] ?? 0,
                              ],
                            })
                          }
                          placeholder="Y"
                          step="0.1"
                          type="number"
                          value={selectedLight.target?.[1] ?? 0}
                        />
                        <Input
                          onChange={(e) =>
                            updateLight(selectedLight.id, {
                              target: [
                                selectedLight.target?.[0] ?? 0,
                                selectedLight.target?.[1] ?? 0,
                                Number.parseFloat(e.target.value) || 0,
                              ],
                            })
                          }
                          placeholder="Z"
                          step="0.1"
                          type="number"
                          value={selectedLight.target?.[2] ?? 0}
                        />
                      </div>
                    </div>
                  )}

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Angle:{" "}
                      {(
                        ((selectedLight.angle || Math.PI / 6) * 180) /
                        Math.PI
                      ).toFixed(0)}
                      °
                    </Label>
                    <Slider
                      className="w-full"
                      max={90}
                      min={1}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, {
                          angle: (numericValue * Math.PI) / 180,
                        });
                      }}
                      step={1}
                      value={[
                        ((selectedLight.angle || Math.PI / 6) * 180) / Math.PI,
                      ]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Penumbra:{" "}
                      {((selectedLight.penumbra || 0.1) * 100).toFixed(0)}%
                    </Label>
                    <Slider
                      className="w-full"
                      max={100}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, {
                          penumbra: numericValue / 100,
                        });
                      }}
                      step={1}
                      value={[(selectedLight.penumbra || 0.1) * 100]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Distance: {selectedLight.distance || 0}
                    </Label>
                    <Slider
                      className="w-full"
                      max={100}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, {
                          distance: numericValue,
                        });
                      }}
                      step={1}
                      value={[selectedLight.distance || 0]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Decay: {(selectedLight.decay || 2).toFixed(1)}
                    </Label>
                    <Slider
                      className="w-full"
                      max={5}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, { decay: numericValue });
                      }}
                      step={0.1}
                      value={[selectedLight.decay || 2]}
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <Label className="text-xs" htmlFor="spot-shadow">
                      Cast Shadow
                    </Label>
                    <Switch
                      checked={selectedLight.castShadow}
                      id="spot-shadow"
                      onCheckedChange={(checked) =>
                        updateLight(selectedLight.id, { castShadow: checked })
                      }
                    />
                  </div>
                </AccordionContent>
              </AccordionItem>
            )}

            {/* Hemisphere Light Controls */}
            {selectedLight.type === "hemisphere" && (
              <AccordionItem value="hemisphere">
                <AccordionTrigger className="text-sm">
                  💡 Hemisphere Light
                </AccordionTrigger>
                <AccordionContent className="space-y-4">
                  <div className="flex items-center justify-between">
                    <Label className="text-xs" htmlFor="hemisphere-visible">
                      Visible
                    </Label>
                    <Switch
                      checked={selectedLight.visible}
                      id="hemisphere-visible"
                      onCheckedChange={(checked) =>
                        updateLight(selectedLight.id, { visible: checked })
                      }
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs" htmlFor="hemisphere-color">
                      Color
                    </Label>
                    <Input
                      className="h-8"
                      id="hemisphere-color"
                      onChange={(e) =>
                        updateLight(selectedLight.id, { color: e.target.value })
                      }
                      type="color"
                      value={selectedLight.color}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Intensity: {selectedLight.intensity.toFixed(1)}
                    </Label>
                    <Slider
                      className="w-full"
                      max={10}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(selectedLight.id, {
                          intensity: numericValue,
                        });
                      }}
                      step={0.1}
                      value={[selectedLight.intensity]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label
                      className="text-xs"
                      htmlFor="hemisphere-ground-color"
                    >
                      Ground Color
                    </Label>
                    <Input
                      className="h-8"
                      id="hemisphere-ground-color"
                      onChange={(e) =>
                        updateLight(selectedLight.id, {
                          groundColor: e.target.value,
                        })
                      }
                      type="color"
                      value={selectedLight.groundColor || "#444444"}
                    />
                  </div>
                </AccordionContent>
              </AccordionItem>
            )}

            {/* Shadow Controls */}
            {shadowLight && (
              <AccordionItem value="shadow">
                <AccordionTrigger className="text-sm">
                  🎭 Shadow Settings
                </AccordionTrigger>
                <AccordionContent className="space-y-4">
                  <div className="space-y-2">
                    <Label className="text-xs">
                      Shadow Bias:{" "}
                      {(shadowLight.shadowBias || -0.0001).toFixed(4)}
                    </Label>
                    <Slider
                      className="w-full"
                      max={0.01}
                      min={-0.01}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(shadowLight.id, {
                          shadowBias: numericValue,
                        });
                      }}
                      step={0.0001}
                      value={[shadowLight.shadowBias || -0.0001]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Shadow Map Size: {shadowLight.shadowMapSize || 1024}
                    </Label>
                    <Slider
                      className="w-full"
                      max={4096}
                      min={256}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(shadowLight.id, {
                          shadowMapSize: numericValue,
                        });
                      }}
                      step={256}
                      value={[shadowLight.shadowMapSize || 1024]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Shadow Near: {(shadowLight.shadowNear || 0.1).toFixed(1)}
                    </Label>
                    <Slider
                      className="w-full"
                      max={10}
                      min={0.01}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(shadowLight.id, {
                          shadowNear: numericValue,
                        });
                      }}
                      step={0.1}
                      value={[shadowLight.shadowNear || 0.1]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Shadow Far: {shadowLight.shadowFar || 50}
                    </Label>
                    <Slider
                      className="w-full"
                      max={1000}
                      min={1}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(shadowLight.id, {
                          shadowFar: numericValue,
                        });
                      }}
                      step={1}
                      value={[shadowLight.shadowFar || 50]}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">
                      Shadow Radius:{" "}
                      {(shadowLight.shadowRadius || 4).toFixed(1)}
                    </Label>
                    <Slider
                      className="w-full"
                      max={20}
                      min={0}
                      onValueChange={(value) => {
                        const numericValue = Array.isArray(value)
                          ? value[0]
                          : value;
                        updateLight(shadowLight.id, {
                          shadowRadius: numericValue,
                        });
                      }}
                      step={0.1}
                      value={[shadowLight.shadowRadius || 4]}
                    />
                  </div>
                </AccordionContent>
              </AccordionItem>
            )}
          </Accordion>
        )}

        {/* Scene Environment Controls */}
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">🎨 Scene</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label className="text-xs" htmlFor="background-color">
                Background Color
              </Label>
              <Input
                className="h-8"
                id="background-color"
                onChange={(e) => setBackgroundColor(e.target.value)}
                type="color"
                value={backgroundColor}
              />
            </div>

            <div className="flex items-center justify-between">
              <Label className="text-xs" htmlFor="show-lights">
                Show Lights
              </Label>
              <Switch
                checked={lightsVisible}
                id="show-lights"
                onCheckedChange={setLightsVisible}
              />
            </div>

            <div className="flex items-center justify-between">
              <Label className="text-xs" htmlFor="show-grid">
                Show Grid
              </Label>
              <Switch
                checked={gridVisible}
                id="show-grid"
                onCheckedChange={setGridVisible}
              />
            </div>

            <div className="flex items-center justify-between">
              <Label className="text-xs" htmlFor="show-axes">
                Show Axes
              </Label>
              <Switch
                checked={axesVisible}
                id="show-axes"
                onCheckedChange={setAxesVisible}
              />
            </div>

            <div className="flex items-center justify-between">
              <Label className="text-xs" htmlFor="show-stats">
                Show Stats
              </Label>
              <Switch
                checked={statsVisible}
                id="show-stats"
                onCheckedChange={setStatsVisible}
              />
            </div>
          </CardContent>
        </Card>

        {/* Fog Controls */}
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">🌫️ Fog</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between">
              <Label className="text-xs" htmlFor="fog-enabled">
                Fog Enabled
              </Label>
              <Switch
                checked={fogEnabled}
                id="fog-enabled"
                onCheckedChange={setFogEnabled}
              />
            </div>

            <div className="space-y-2">
              <Label className="text-xs" htmlFor="fog-color">
                Fog Color
              </Label>
              <Input
                className="h-8"
                disabled={!fogEnabled}
                id="fog-color"
                onChange={(e) => setFogColor(e.target.value)}
                type="color"
                value={fogColor}
              />
            </div>

            <div className="space-y-2">
              <Label className="text-xs">Fog Near: {fogNear.toFixed(1)}</Label>
              <Slider
                className="w-full"
                disabled={!fogEnabled}
                max={50}
                min={0}
                onValueChange={(value) => {
                  const numericValue = Array.isArray(value) ? value[0] : value;
                  setFogNear(numericValue);
                }}
                step={0.1}
                value={[fogNear]}
              />
            </div>

            <div className="space-y-2">
              <Label className="text-xs">Fog Far: {fogFar}</Label>
              <Slider
                className="w-full"
                disabled={!fogEnabled}
                max={200}
                min={10}
                onValueChange={(value) => {
                  const numericValue = Array.isArray(value) ? value[0] : value;
                  setFogFar(numericValue);
                }}
                step={1}
                value={[fogFar]}
              />
            </div>
          </CardContent>
        </Card>
      </div>
    </ScrollArea>
  );
}
