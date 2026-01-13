import {
  Box,
  ChevronDown,
  ChevronRight,
  Circle,
  Eye,
  EyeOff,
  ImageIcon,
  Lightbulb,
  Moon,
  Package,
  Settings as SettingsIcon,
  Square,
  Sun,
  Zap,
} from "lucide-react";
import { memo, useCallback, useMemo, useState } from "react";
// import { FixedSizeTree as Tree } from "react-vtree";
import { Badge } from "~/components/ui/badge";
import { Label } from "~/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { Slider } from "~/components/ui/slider";
import { Switch } from "~/components/ui/switch";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { Input } from "~/components/ui/input";
import { cn } from "~/lib/utils";
import { ENVIRONMENT_PRESETS } from "~/components/skybox";
import { useSceneStore } from "~/stores/scene-store";
import {
  useAdaptiveQualityEnabled,
  useGraphicsSettings,
  usePerformanceMonitorEnabled,
  useSettingsStore,
} from "~/stores/settings-store";
import { QualityLevel, type QualityLevelType } from "~/utils/adaptive-quality";
import { AssetsPanel } from "./assets-panel";
import { RemoteAssetsTab } from "./remote-assets-tab";
import { useTheme } from "./theme-provider";

// Regex for UUID validation
const UUID_REGEX = /^[0-9a-f-]+$/;

// Settings content component
function SettingsContent() {
  const { theme, setTheme } = useTheme();
  const {
    precision,
    setPrecision,
    setPerformanceMonitorEnabled,
    setGraphicsSettings,
    setQualityLevel,
    setAdaptiveQualityEnabled,
  } = useSettingsStore();
  const performanceMonitorEnabled = usePerformanceMonitorEnabled();
  const graphicsSettings = useGraphicsSettings();
  const adaptiveQualityEnabled = useAdaptiveQualityEnabled();

  // Environment settings (expanded from skybox)
  const {
    skyboxEnabled,
    skyboxPreset,
    skyboxIntensity,
    environmentIntensity,
    backgroundBlurriness,
    backgroundRotation,
    groundProjection,
    groundProjectionHeight,
    groundProjectionRadius,
    groundProjectionScale,
    liveEnvironment,
    liveEnvironmentResolution,
    cameraShakeEnabled,
    cameraShakeIntensity,
    cameraShakeSpeed,
    setSkyboxEnabled,
    setSkyboxPreset,
    setSkyboxIntensity,
    setEnvironmentIntensity,
    setBackgroundBlurriness,
    setBackgroundRotation,
    setGroundProjection,
    setGroundProjectionHeight,
    setGroundProjectionRadius,
    setGroundProjectionScale,
    setLiveEnvironment,
    setLiveEnvironmentResolution,
    setCameraShakeEnabled,
    setCameraShakeIntensity,
    setCameraShakeSpeed,
  } = useSceneStore();

  return (
    <div className="space-y-6 p-4">
      {/* Appearance Section */}
      <div className="space-y-4">
        <div className="space-y-2">
          <Label>Theme</Label>
          <Select
            onValueChange={(value) =>
              setTheme(value as "light" | "dark" | "system")
            }
            value={theme}
          >
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="light">Light</SelectItem>
              <SelectItem value="dark">Dark</SelectItem>
              <SelectItem value="system">System</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      {/* Graphics Quality Section */}
      <div className="space-y-6">
        {/* Quality Level */}
        <div className="space-y-3">
          <div className="space-y-1">
            <Label>Quality Preset</Label>
            <p className="text-muted-foreground text-sm">
              Overall graphics quality level. Higher settings use more
              resources.
            </p>
          </div>
          <Select
            onValueChange={(value) =>
              setQualityLevel(Number(value) as QualityLevelType)
            }
            value={graphicsSettings.qualityLevel.toString()}
          >
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value={QualityLevel.ULTRA.toString()}>
                Ultra - Maximum quality (High-end GPUs only)
              </SelectItem>
              <SelectItem value={QualityLevel.HIGH.toString()}>
                High - Balanced performance and quality
              </SelectItem>
              <SelectItem value={QualityLevel.MEDIUM.toString()}>
                Medium - Good quality with better performance
              </SelectItem>
              <SelectItem value={QualityLevel.LOW.toString()}>
                Low - Basic quality for lower-end hardware
              </SelectItem>
              <SelectItem value={QualityLevel.POTATO.toString()}>
                Potato - Minimum settings for very low-end hardware
              </SelectItem>
            </SelectContent>
          </Select>
        </div>

        {/* Adaptive Quality */}
        <div className="flex items-center justify-between">
          <div className="space-y-0.5">
            <Label>Adaptive Quality</Label>
            <p className="text-muted-foreground text-sm">
              Automatically adjust quality based on performance
            </p>
          </div>
          <Switch
            checked={adaptiveQualityEnabled}
            onCheckedChange={setAdaptiveQualityEnabled}
          />
        </div>

        {/* Rendering Features */}
        <div className="space-y-4">
          <h4 className="font-medium text-sm">Rendering Features</h4>
          <div className="grid grid-cols-2 gap-4">
            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <Label className="text-sm">Anti-aliasing</Label>
                <p className="text-muted-foreground text-xs">Smooth edges</p>
              </div>
              <Switch
                checked={graphicsSettings.antialias}
                onCheckedChange={(checked) =>
                  setGraphicsSettings({ antialias: checked })
                }
              />
            </div>

            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <Label className="text-sm">Shadows</Label>
                <p className="text-muted-foreground text-xs">Dynamic shadows</p>
              </div>
              <Switch
                checked={graphicsSettings.shadows}
                onCheckedChange={(checked) =>
                  setGraphicsSettings({ shadows: checked })
                }
              />
            </div>

            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <Label className="text-sm">Post-processing</Label>
                <p className="text-muted-foreground text-xs">
                  Effects & filters
                </p>
              </div>
              <Switch
                checked={graphicsSettings.postProcessing}
                onCheckedChange={(checked) =>
                  setGraphicsSettings({ postProcessing: checked })
                }
              />
            </div>
          </div>
        </div>

        {/* Quality Sliders */}
        <div className="space-y-4">
          <h4 className="font-medium text-sm">Quality Settings</h4>
          <div className="space-y-4">
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label className="text-sm">Texture Quality</Label>
                <span className="text-muted-foreground text-xs">
                  {Math.round(graphicsSettings.textureQuality * 100)}%
                </span>
              </div>
              <Slider
                max={1}
                min={0.25}
                onValueChange={(value) =>
                  setGraphicsSettings({
                    textureQuality: Array.isArray(value) ? value[0] : value,
                  })
                }
                step={0.25}
                value={[graphicsSettings.textureQuality]}
              />
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label className="text-sm">LOD Distance</Label>
                <span className="text-muted-foreground text-xs">
                  {Math.round(graphicsSettings.lodDistance * 100)}%
                </span>
              </div>
              <Slider
                max={1}
                min={0.4}
                onValueChange={(value) =>
                  setGraphicsSettings({
                    lodDistance: Array.isArray(value) ? value[0] : value,
                  })
                }
                step={0.1}
                value={[graphicsSettings.lodDistance]}
              />
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label className="text-sm">Culling Distance</Label>
                <span className="text-muted-foreground text-xs">
                  {Math.round(graphicsSettings.cullingDistance * 100)}%
                </span>
              </div>
              <Slider
                max={1.5}
                min={0.4}
                onValueChange={(value) =>
                  setGraphicsSettings({
                    cullingDistance: Array.isArray(value) ? value[0] : value,
                  })
                }
                step={0.1}
                value={[graphicsSettings.cullingDistance]}
              />
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label className="text-sm">Particle Count</Label>
                <span className="text-muted-foreground text-xs">
                  {Math.round(graphicsSettings.particleCount * 100)}%
                </span>
              </div>
              <Slider
                max={1}
                min={0.25}
                onValueChange={(value) =>
                  setGraphicsSettings({
                    particleCount: Array.isArray(value) ? value[0] : value,
                  })
                }
                step={0.25}
                value={[graphicsSettings.particleCount]}
              />
            </div>
          </div>
        </div>
      </div>

      {/* Environment Section */}
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div className="space-y-0.5">
            <Label>Enable Environment</Label>
            <p className="text-muted-foreground text-sm">
              Use advanced environment mapping with HDR lighting
            </p>
          </div>
          <Switch checked={skyboxEnabled} onCheckedChange={setSkyboxEnabled} />
        </div>

        {skyboxEnabled && (
          <>
            {/* Environment Presets */}
            <div className="space-y-3">
              <div className="space-y-1">
                <Label>Environment Preset</Label>
                <p className="text-muted-foreground text-sm">
                  Quick presets (1K HDR) • Use Remote tab for premium 4K EXR
                  assets
                </p>
              </div>
              <Select
                onValueChange={(value) => {
                  if (value) setSkyboxPreset(value);
                }}
                value={skyboxPreset}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {Object.entries(ENVIRONMENT_PRESETS).map(([key, preset]) => (
                    <SelectItem key={key} value={key}>
                      {key === "custom"
                        ? "Custom HDRI"
                        : typeof preset === "string"
                          ? preset
                              .replace(/_/g, " ")
                              .replace(/\b\w/g, (l) => l.toUpperCase())
                          : preset?.name || key}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            {/* Intensity Controls */}
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label className="text-sm">Background Intensity</Label>
                  <span className="text-muted-foreground text-xs">
                    {Math.round(skyboxIntensity * 100)}%
                  </span>
                </div>
                <Slider
                  max={2}
                  min={0}
                  onValueChange={(value) =>
                    setSkyboxIntensity(Array.isArray(value) ? value[0] : value)
                  }
                  step={0.1}
                  value={[skyboxIntensity]}
                />
              </div>

              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label className="text-sm">Environment Intensity</Label>
                  <span className="text-muted-foreground text-xs">
                    {Math.round(environmentIntensity * 100)}%
                  </span>
                </div>
                <Slider
                  max={2}
                  min={0}
                  onValueChange={(value) =>
                    setEnvironmentIntensity(
                      Array.isArray(value) ? value[0] : value
                    )
                  }
                  step={0.1}
                  value={[environmentIntensity]}
                />
              </div>
            </div>

            {/* Background Blurriness */}
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label className="text-sm">Background Blurriness</Label>
                <span className="text-muted-foreground text-xs">
                  {Math.round(backgroundBlurriness * 100)}%
                </span>
              </div>
              <Slider
                max={1}
                min={0}
                onValueChange={(value) =>
                  setBackgroundBlurriness(
                    Array.isArray(value) ? value[0] : value
                  )
                }
                step={0.01}
                value={[backgroundBlurriness]}
              />
            </div>

            {/* Rotation Controls */}
            <div className="space-y-3">
              <h4 className="font-medium text-sm">Rotation</h4>
              <div className="grid grid-cols-3 gap-2 text-xs">
                <div className="space-y-1">
                  <Label>X</Label>
                  <Input
                    onChange={(e) => {
                      const newRotation: [number, number, number] = [
                        Number.parseFloat(e.target.value) || 0,
                        backgroundRotation[1],
                        backgroundRotation[2],
                      ];
                      setBackgroundRotation(newRotation);
                    }}
                    step="0.1"
                    type="number"
                    value={backgroundRotation[0]}
                  />
                </div>
                <div className="space-y-1">
                  <Label>Y</Label>
                  <Input
                    onChange={(e) => {
                      const newRotation: [number, number, number] = [
                        backgroundRotation[0],
                        Number.parseFloat(e.target.value) || 0,
                        backgroundRotation[2],
                      ];
                      setBackgroundRotation(newRotation);
                    }}
                    step="0.1"
                    type="number"
                    value={backgroundRotation[1]}
                  />
                </div>
                <div className="space-y-1">
                  <Label>Z</Label>
                  <Input
                    onChange={(e) => {
                      const newRotation: [number, number, number] = [
                        backgroundRotation[0],
                        backgroundRotation[1],
                        Number.parseFloat(e.target.value) || 0,
                      ];
                      setBackgroundRotation(newRotation);
                    }}
                    step="0.1"
                    type="number"
                    value={backgroundRotation[2]}
                  />
                </div>
              </div>
            </div>

            {/* Ground Projection */}
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>Ground Projection</Label>
                  <p className="text-muted-foreground text-sm">
                    Project environment onto ground plane
                  </p>
                </div>
                <Switch
                  checked={groundProjection}
                  onCheckedChange={setGroundProjection}
                />
              </div>

              {groundProjection && (
                <div className="space-y-3 border-muted border-l-2 pl-4">
                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <Label className="text-sm">Height</Label>
                      <span className="text-muted-foreground text-xs">
                        {groundProjectionHeight}
                      </span>
                    </div>
                    <Slider
                      max={50}
                      min={1}
                      onValueChange={(value) =>
                        setGroundProjectionHeight(
                          Array.isArray(value) ? value[0] : value
                        )
                      }
                      step={1}
                      value={[groundProjectionHeight]}
                    />
                  </div>

                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <Label className="text-sm">Radius</Label>
                      <span className="text-muted-foreground text-xs">
                        {groundProjectionRadius}
                      </span>
                    </div>
                    <Slider
                      max={200}
                      min={10}
                      onValueChange={(value) =>
                        setGroundProjectionRadius(
                          Array.isArray(value) ? value[0] : value
                        )
                      }
                      step={5}
                      value={[groundProjectionRadius]}
                    />
                  </div>

                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <Label className="text-sm">Scale</Label>
                      <span className="text-muted-foreground text-xs">
                        {groundProjectionScale}
                      </span>
                    </div>
                    <Slider
                      max={5000}
                      min={100}
                      onValueChange={(value) =>
                        setGroundProjectionScale(
                          Array.isArray(value) ? value[0] : value
                        )
                      }
                      step={100}
                      value={[groundProjectionScale]}
                    />
                  </div>
                </div>
              )}
            </div>

            {/* Live Environment */}
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>Live Environment</Label>
                  <p className="text-muted-foreground text-sm">
                    Generate environment from scene in real-time
                  </p>
                </div>
                <Switch
                  checked={liveEnvironment}
                  onCheckedChange={setLiveEnvironment}
                />
              </div>

              {liveEnvironment && (
                <div className="space-y-2 border-muted border-l-2 pl-4">
                  <div className="flex items-center justify-between">
                    <Label className="text-sm">Resolution</Label>
                    <span className="text-muted-foreground text-xs">
                      {liveEnvironmentResolution}px
                    </span>
                  </div>
                  <Slider
                    max={1024}
                    min={128}
                    onValueChange={(value) =>
                      setLiveEnvironmentResolution(
                        Array.isArray(value) ? value[0] : value
                      )
                    }
                    step={64}
                    value={[liveEnvironmentResolution]}
                  />
                </div>
              )}
            </div>

            {/* Camera Shake */}
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>Camera Shake</Label>
                  <p className="text-muted-foreground text-sm">
                    Add camera shake effect for dynamic scenes
                  </p>
                </div>
                <Switch
                  checked={cameraShakeEnabled}
                  onCheckedChange={setCameraShakeEnabled}
                />
              </div>

              {cameraShakeEnabled && (
                <div className="space-y-3 border-muted border-l-2 pl-4">
                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <Label className="text-sm">Intensity</Label>
                      <span className="text-muted-foreground text-xs">
                        {cameraShakeIntensity.toFixed(2)}
                      </span>
                    </div>
                    <Slider
                      max={1}
                      min={0.01}
                      onValueChange={(value) =>
                        setCameraShakeIntensity(
                          Array.isArray(value) ? value[0] : value
                        )
                      }
                      step={0.01}
                      value={[cameraShakeIntensity]}
                    />
                  </div>

                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <Label className="text-sm">Speed</Label>
                      <span className="text-muted-foreground text-xs">
                        {cameraShakeSpeed.toFixed(1)}x
                      </span>
                    </div>
                    <Slider
                      max={5}
                      min={0.1}
                      onValueChange={(value) =>
                        setCameraShakeSpeed(
                          Array.isArray(value) ? value[0] : value
                        )
                      }
                      step={0.1}
                      value={[cameraShakeSpeed]}
                    />
                  </div>
                </div>
              )}
            </div>
          </>
        )}
      </div>

      {/* Performance Section */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <div className="space-y-0.5">
            <Label>Show Performance Monitor</Label>
            <p className="text-muted-foreground text-sm">
              Display FPS, memory usage, and other metrics in the viewport
            </p>
          </div>
          <Switch
            checked={performanceMonitorEnabled}
            onCheckedChange={setPerformanceMonitorEnabled}
          />
        </div>

        <div className="flex items-center justify-between">
          <div className="space-y-0.5">
            <Label>Performance Regression on Move</Label>
            <p className="text-muted-foreground text-sm">
              Reduce graphics quality while moving the camera for smoother
              navigation
            </p>
          </div>
          <Switch
            checked={useSceneStore.getState().performanceRegressionOnMove}
            onCheckedChange={(checked) =>
              useSceneStore.getState().setPerformanceRegressionOnMove(checked)
            }
          />
        </div>
      </div>

      {/* Precision Section */}
      <div className="space-y-4">
        <div className="space-y-2">
          <Label>Decimal Places</Label>
          <Select
            onValueChange={(value) =>
              value && setPrecision(Number.parseInt(value, 10))
            }
            value={precision.toString()}
          >
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="1">1 decimal place</SelectItem>
              <SelectItem value="2">2 decimal places</SelectItem>
              <SelectItem value="3">3 decimal places</SelectItem>
              <SelectItem value="4">4 decimal places</SelectItem>
              <SelectItem value="5">5 decimal places</SelectItem>
              <SelectItem value="6">6 decimal places</SelectItem>
            </SelectContent>
          </Select>
          <p className="text-muted-foreground text-sm">
            Controls precision for position, rotation, scale values and exports.
          </p>
        </div>
      </div>
    </div>
  );
}

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

// Main left sidebar component
export const LeftSidebar = memo(function LeftSidebar({
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
  const [activeTab, setActiveTab] = useState("hierarchy");

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
      } else if (
        nodeId.startsWith("object_") ||
        nodeId.startsWith("imported_") ||
        UUID_REGEX.test(nodeId)
      ) {
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
    <div
      className={cn(
        "flex h-[calc(100%-3.5rem)] w-72 shrink-0 flex-col overflow-x-auto overflow-y-hidden rounded-lg border md:w-80 lg:w-96",
        className
      )}
    >
      <Tabs
        className="flex h-full flex-col"
        onValueChange={setActiveTab}
        value={activeTab}
      >
        <TabsList className="grid w-full shrink-0 grid-cols-3 gap-2 bg-muted/50 p-1">
          <TabsTrigger className="flex items-center gap-2" value="hierarchy">
            <Package className="h-4 w-4" />
            Scene
            <Badge className="text-xs" variant="secondary">
              {objects.length + lights.length}
            </Badge>
          </TabsTrigger>
          <TabsTrigger className="flex items-center gap-2" value="assets">
            <ImageIcon className="h-4 w-4" />
            Assets
          </TabsTrigger>
          <TabsTrigger className="flex items-center gap-2" value="settings">
            <SettingsIcon className="h-4 w-4" />
            Settings
          </TabsTrigger>
        </TabsList>

        <TabsContent className="mt-4 flex-1 overflow-hidden" value="hierarchy">
          {/* Tree */}
          <div className="scrollbar-thin scrollbar-thumb-muted scrollbar-track-transparent h-full overflow-auto">
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
        </TabsContent>

        <TabsContent className="mt-4 flex-1 overflow-hidden" value="assets">
          <AssetsPanel />
        </TabsContent>

        <TabsContent className="mt-4 flex-1 overflow-hidden" value="settings">
          <div className="scrollbar-thin scrollbar-thumb-muted scrollbar-track-transparent h-full overflow-auto">
            <SettingsContent />
          </div>
        </TabsContent>

        <TabsContent className="mt-4 flex-1 overflow-hidden" value="remote">
          <RemoteAssetsTab />
        </TabsContent>
      </Tabs>
    </div>
  );
});
