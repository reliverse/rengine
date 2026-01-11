import type { LucideIcon } from "lucide-react";
import {
  Activity,
  Grid3X3,
  Monitor,
  Palette,
  Settings as SettingsIcon,
} from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "~/components/ui/dialog";
import { useSceneStore } from "~/stores/scene-store";
import {
  useAdaptiveQualityEnabled,
  useGraphicsSettings,
  usePerformanceMonitorEnabled,
  useSettingsStore,
} from "~/stores/settings-store";
import { QualityLevel, type QualityLevelType } from "~/utils/adaptive-quality";
import { useTheme } from "./theme-provider";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Label } from "./ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import { Slider } from "./ui/slider";
import { Switch } from "./ui/switch";

interface SettingSectionProps {
  icon: LucideIcon;
  title: string;
  description: string;
  children: React.ReactNode;
}

function SettingSection({
  icon: Icon,
  title,
  description,
  children,
}: SettingSectionProps) {
  return (
    <Card>
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center gap-2 text-lg">
          <Icon className="h-5 w-5" />
          {title}
        </CardTitle>
        <p className="text-muted-foreground text-sm">{description}</p>
      </CardHeader>
      <CardContent className="space-y-4">{children}</CardContent>
    </Card>
  );
}

interface SettingsDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function SettingsDialog({ open, onOpenChange }: SettingsDialogProps) {
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

  return (
    <Dialog onOpenChange={onOpenChange} open={open}>
      <DialogContent className="max-h-[90vh] max-w-4xl overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Rengine Settings</DialogTitle>
          <DialogDescription>Customize your experience</DialogDescription>
        </DialogHeader>

        <div className="space-y-6">
          <SettingSection
            description="Customize the look and feel"
            icon={Palette}
            title="Appearance"
          >
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
          </SettingSection>

          <SettingSection
            description="Configure graphics quality and rendering performance"
            icon={Monitor}
            title="Graphics Quality"
          >
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
                      <p className="text-muted-foreground text-xs">
                        Smooth edges
                      </p>
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
                      <p className="text-muted-foreground text-xs">
                        Dynamic shadows
                      </p>
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
                          textureQuality: Array.isArray(value)
                            ? value[0]
                            : value,
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
                          cullingDistance: Array.isArray(value)
                            ? value[0]
                            : value,
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
                          particleCount: Array.isArray(value)
                            ? value[0]
                            : value,
                        })
                      }
                      step={0.25}
                      value={[graphicsSettings.particleCount]}
                    />
                  </div>
                </div>
              </div>
            </div>
          </SettingSection>

          <SettingSection
            description="Monitor performance metrics and diagnostics"
            icon={Activity}
            title="Performance"
          >
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
                    useSceneStore
                      .getState()
                      .setPerformanceRegressionOnMove(checked)
                  }
                />
              </div>
            </div>
          </SettingSection>

          <SettingSection
            description="Customize grid and visual guides"
            icon={Grid3X3}
            title="Grid & Guides"
          >
            <div className="space-y-4">
              <div className="text-muted-foreground text-sm">
                Grid and guide settings coming soon...
              </div>
            </div>
          </SettingSection>

          <SettingSection
            description="Configure numeric precision for values and exports"
            icon={SettingsIcon}
            title="Precision"
          >
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
                  Controls precision for position, rotation, scale values and
                  exports.
                </p>
              </div>
            </div>
          </SettingSection>
        </div>
      </DialogContent>
    </Dialog>
  );
}

// Legacy component for backward compatibility
export default function SettingsPanel() {
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

  return (
    <div className="space-y-6 p-6">
      <div className="mb-6">
        <h2 className="mb-2 font-bold text-3xl">Rengine Settings</h2>
        <p className="text-muted-foreground">Customize your experience</p>
      </div>

      <SettingSection
        description="Customize the look and feel"
        icon={Palette}
        title="Appearance"
      >
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
      </SettingSection>

      <SettingSection
        description="Configure graphics quality and rendering performance"
        icon={Monitor}
        title="Graphics Quality"
      >
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
                  <p className="text-muted-foreground text-xs">
                    Dynamic shadows
                  </p>
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
      </SettingSection>

      <SettingSection
        description="Monitor performance metrics and diagnostics"
        icon={Activity}
        title="Performance"
      >
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
      </SettingSection>

      <SettingSection
        description="Customize grid and visual guides"
        icon={Grid3X3}
        title="Grid & Guides"
      >
        <div className="space-y-4">
          <div className="text-muted-foreground text-sm">
            Grid and guide settings coming soon...
          </div>
        </div>
      </SettingSection>

      <SettingSection
        description="Configure numeric precision for values and exports"
        icon={SettingsIcon}
        title="Precision"
      >
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
              Controls precision for position, rotation, scale values and
              exports.
            </p>
          </div>
        </div>
      </SettingSection>
    </div>
  );
}
