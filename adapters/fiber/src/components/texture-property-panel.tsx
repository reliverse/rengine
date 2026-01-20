import {
  Copy,
  Download,
  Eye,
  Image,
  Info,
  RotateCw,
  Settings,
  Sliders,
  Trash2,
} from "lucide-react";
import { useCallback, useState } from "react";
import * as THREE from "three";
import { Badge } from "~/components/ui/badge";
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
import { Separator } from "~/components/ui/separator";
import { Slider } from "~/components/ui/slider";
import { Switch } from "~/components/ui/switch";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { cn } from "~/lib/utils";
import { useSelectedTexture, useTextureStore } from "~/stores/texture-store";

interface TexturePropertyPanelProps {
  className?: string;
}

const TextureTypeLabels = {
  albedo: "Albedo",
  normal: "Normal",
  roughness: "Roughness",
  metallic: "Metallic",
  ao: "Ambient Occlusion",
  emissive: "Emissive",
  height: "Height",
  opacity: "Opacity",
  environment: "Environment",
  custom: "Custom",
} as const;

const WrappingModeLabels = {
  [THREE.RepeatWrapping]: "Repeat",
  [THREE.ClampToEdgeWrapping]: "Clamp to Edge",
  [THREE.MirroredRepeatWrapping]: "Mirrored Repeat",
} as const;

const FilterModeLabels = {
  [THREE.NearestFilter]: "Nearest",
  [THREE.LinearFilter]: "Linear",
  [THREE.NearestMipmapNearestFilter]: "Nearest (Mipmap Nearest)",
  [THREE.NearestMipmapLinearFilter]: "Nearest (Mipmap Linear)",
  [THREE.LinearMipmapNearestFilter]: "Linear (Mipmap Nearest)",
  [THREE.LinearMipmapLinearFilter]: "Linear (Mipmap Linear)",
} as const;

export function TexturePropertyPanel({ className }: TexturePropertyPanelProps) {
  const selectedTexture = useSelectedTexture();
  const {
    updateTexture,
    updateTextureSettings,
    duplicateTexture,
    removeTexture,
    exportTexture,
    generateThumbnail,
    optimizeTexture,
    validateTextureAsset,
  } = useTextureStore();

  const [activeTab, setActiveTab] = useState("properties");

  const handleNameChange = useCallback(
    (name: string) => {
      if (selectedTexture) {
        updateTexture(selectedTexture.id, { name });
      }
    },
    [selectedTexture, updateTexture]
  );

  const handleTypeChange = useCallback(
    (type: string) => {
      if (selectedTexture) {
        updateTexture(selectedTexture.id, { type: type as any });
      }
    },
    [selectedTexture, updateTexture]
  );

  const handleSettingsChange = useCallback(
    (settings: Partial<any>) => {
      if (selectedTexture) {
        updateTextureSettings(selectedTexture.id, settings);
      }
    },
    [selectedTexture, updateTextureSettings]
  );

  const handleDuplicate = useCallback(() => {
    if (selectedTexture) {
      duplicateTexture(selectedTexture.id);
    }
  }, [selectedTexture, duplicateTexture]);

  const handleDelete = useCallback(() => {
    if (selectedTexture) {
      removeTexture(selectedTexture.id);
    }
  }, [selectedTexture, removeTexture]);

  const handleExport = useCallback(
    async (format: "png" | "jpg" | "webp") => {
      if (selectedTexture) {
        await exportTexture(selectedTexture.id, format);
      }
    },
    [selectedTexture, exportTexture]
  );

  const handleOptimize = useCallback(async () => {
    if (selectedTexture) {
      await optimizeTexture(selectedTexture.id);
    }
  }, [selectedTexture, optimizeTexture]);

  const handleRegenerateThumbnail = useCallback(async () => {
    if (selectedTexture) {
      await generateThumbnail(selectedTexture.id);
    }
  }, [selectedTexture, generateThumbnail]);

  const validation = selectedTexture
    ? validateTextureAsset(selectedTexture.id)
    : null;

  if (!selectedTexture) {
    return (
      <Card className={cn("h-full w-full", className)}>
        <CardContent className="flex h-full items-center justify-center">
          <div className="text-center text-muted-foreground">
            <Image className="mx-auto mb-4 h-12 w-12" />
            <h3 className="mb-2 font-medium text-lg">No Texture Selected</h3>
            <p>Select a texture from the library to view its properties.</p>
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className={cn("flex h-full w-full flex-col", className)}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2 truncate">
            <Image className="h-5 w-5 flex-shrink-0" />
            {selectedTexture.name}
          </CardTitle>
          <div className="flex items-center gap-1">
            <Button onClick={handleDuplicate} size="sm" variant="ghost">
              <Copy className="h-4 w-4" />
            </Button>
            <Button onClick={handleDelete} size="sm" variant="ghost">
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </CardHeader>

      <CardContent className="flex-1 p-4 pt-0">
        <Tabs
          className="flex h-full flex-col"
          onValueChange={setActiveTab}
          value={activeTab}
        >
          <TabsList className="grid w-full grid-cols-4">
            <TabsTrigger className="flex items-center gap-1" value="properties">
              <Settings className="h-3 w-3" />
              Properties
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-1" value="settings">
              <Sliders className="h-3 w-3" />
              Settings
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-1" value="preview">
              <Eye className="h-3 w-3" />
              Preview
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-1" value="info">
              <Info className="h-3 w-3" />
              Info
            </TabsTrigger>
          </TabsList>

          <ScrollArea className="mt-4 flex-1">
            <TabsContent className="space-y-4" value="properties">
              <div className="space-y-2">
                <Label htmlFor="texture-name">Name</Label>
                <Input
                  id="texture-name"
                  onChange={(e) => handleNameChange(e.target.value)}
                  value={selectedTexture.name}
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="texture-type">Type</Label>
                <Select
                  onValueChange={(value) => value && handleTypeChange(value)}
                  value={selectedTexture.type}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {Object.entries(TextureTypeLabels).map(([key, label]) => (
                      <SelectItem key={key} value={key}>
                        {label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <Separator />

              <div className="space-y-4">
                <h4 className="font-medium">Actions</h4>

                <div className="flex flex-wrap gap-2">
                  <Button
                    onClick={handleRegenerateThumbnail}
                    size="sm"
                    variant="outline"
                  >
                    <RotateCw className="mr-2 h-4 w-4" />
                    Regenerate Thumbnail
                  </Button>

                  <Button onClick={handleOptimize} size="sm" variant="outline">
                    <Sliders className="mr-2 h-4 w-4" />
                    Optimize
                  </Button>
                </div>

                <div className="flex flex-wrap gap-2">
                  <Select
                    onValueChange={(value) =>
                      handleExport(value as "png" | "jpg" | "webp")
                    }
                  >
                    <SelectTrigger className="w-32">
                      <div className="flex items-center">
                        <Download className="mr-2 h-4 w-4" />
                        <span>Export</span>
                      </div>
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="png">PNG</SelectItem>
                      <SelectItem value="jpg">JPG</SelectItem>
                      <SelectItem value="webp">WebP</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </TabsContent>

            <TabsContent className="space-y-4" value="settings">
              <div className="space-y-4">
                <h4 className="font-medium">Wrapping</h4>

                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label>U Wrap</Label>
                    <Select
                      onValueChange={(value) =>
                        value &&
                        handleSettingsChange({
                          wrapS: Number.parseInt(value, 10),
                        })
                      }
                      value={selectedTexture.settings.wrapS.toString()}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {Object.entries(WrappingModeLabels).map(
                          ([key, label]) => (
                            <SelectItem key={key} value={key}>
                              {label}
                            </SelectItem>
                          )
                        )}
                      </SelectContent>
                    </Select>
                  </div>

                  <div className="space-y-2">
                    <Label>V Wrap</Label>
                    <Select
                      onValueChange={(value) =>
                        value &&
                        handleSettingsChange({
                          wrapT: Number.parseInt(value, 10),
                        })
                      }
                      value={selectedTexture.settings.wrapT.toString()}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {Object.entries(WrappingModeLabels).map(
                          ([key, label]) => (
                            <SelectItem key={key} value={key}>
                              {label}
                            </SelectItem>
                          )
                        )}
                      </SelectContent>
                    </Select>
                  </div>
                </div>
              </div>

              <Separator />

              <div className="space-y-4">
                <h4 className="font-medium">Filtering</h4>

                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label>Mag Filter</Label>
                    <Select
                      onValueChange={(value) =>
                        value &&
                        handleSettingsChange({
                          magFilter: Number.parseInt(value, 10),
                        })
                      }
                      value={selectedTexture.settings.magFilter.toString()}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value={THREE.NearestFilter.toString()}>
                          Nearest
                        </SelectItem>
                        <SelectItem value={THREE.LinearFilter.toString()}>
                          Linear
                        </SelectItem>
                      </SelectContent>
                    </Select>
                  </div>

                  <div className="space-y-2">
                    <Label>Min Filter</Label>
                    <Select
                      onValueChange={(value) =>
                        value &&
                        handleSettingsChange({
                          minFilter: Number.parseInt(value, 10),
                        })
                      }
                      value={selectedTexture.settings.minFilter.toString()}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {Object.entries(FilterModeLabels).map(
                          ([key, label]) => (
                            <SelectItem key={key} value={key}>
                              {label}
                            </SelectItem>
                          )
                        )}
                      </SelectContent>
                    </Select>
                  </div>
                </div>
              </div>

              <Separator />

              <div className="space-y-4">
                <h4 className="font-medium">Quality Settings</h4>

                <div className="space-y-4">
                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <Label>Anisotropy</Label>
                      <span className="text-muted-foreground text-sm">
                        {selectedTexture.settings.anisotropy}
                      </span>
                    </div>
                    <Slider
                      className="w-full"
                      max={16}
                      min={1}
                      onValueChange={(values) =>
                        handleSettingsChange({
                          anisotropy: Array.isArray(values)
                            ? values[0]
                            : values,
                        })
                      }
                      step={1}
                      value={[selectedTexture.settings.anisotropy]}
                    />
                  </div>

                  <div className="flex items-center space-x-2">
                    <Switch
                      checked={selectedTexture.settings.generateMipmaps}
                      id="generate-mipmaps"
                      onCheckedChange={(checked) =>
                        handleSettingsChange({ generateMipmaps: checked })
                      }
                    />
                    <Label htmlFor="generate-mipmaps">Generate Mipmaps</Label>
                  </div>

                  <div className="flex items-center space-x-2">
                    <Switch
                      checked={selectedTexture.settings.flipY}
                      id="flip-y"
                      onCheckedChange={(checked) =>
                        handleSettingsChange({ flipY: checked })
                      }
                    />
                    <Label htmlFor="flip-y">Flip Y</Label>
                  </div>
                </div>
              </div>
            </TabsContent>

            <TabsContent className="space-y-4" value="preview">
              <div className="space-y-4">
                <h4 className="font-medium">Texture Preview</h4>

                <div className="aspect-square overflow-hidden rounded-lg border bg-muted">
                  {selectedTexture.thumbnail ? (
                    <img
                      alt={selectedTexture.name}
                      className="h-full w-full object-contain"
                      height={256}
                      src={selectedTexture.thumbnail}
                      width={256}
                    />
                  ) : (
                    <div className="flex h-full w-full items-center justify-center text-muted-foreground">
                      <div className="text-center">
                        <Image className="mx-auto mb-2 h-12 w-12" />
                        <p>No preview available</p>
                      </div>
                    </div>
                  )}
                </div>

                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <Label className="text-muted-foreground">Resolution</Label>
                    <p>
                      {selectedTexture.metadata.width} ×{" "}
                      {selectedTexture.metadata.height}
                    </p>
                  </div>
                  <div>
                    <Label className="text-muted-foreground">Format</Label>
                    <p>
                      {selectedTexture.metadata.format === THREE.RGBAFormat
                        ? "RGBA"
                        : "RGB"}
                    </p>
                  </div>
                  <div>
                    <Label className="text-muted-foreground">Size</Label>
                    <p>
                      {(selectedTexture.metadata.sizeBytes / 1024).toFixed(2)}{" "}
                      KB
                    </p>
                  </div>
                  <div>
                    <Label className="text-muted-foreground">Encoding</Label>
                    <p>
                      {selectedTexture.metadata.encoding ===
                      THREE.SRGBColorSpace
                        ? "sRGB"
                        : "Linear"}
                    </p>
                  </div>
                </div>
              </div>
            </TabsContent>

            <TabsContent className="space-y-4" value="info">
              <div className="space-y-4">
                <h4 className="font-medium">Texture Information</h4>

                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <Label className="text-muted-foreground">ID</Label>
                    <p className="break-all font-mono text-xs">
                      {selectedTexture.id}
                    </p>
                  </div>
                  <div>
                    <Label className="text-muted-foreground">Type</Label>
                    <p>{TextureTypeLabels[selectedTexture.type]}</p>
                  </div>
                  <div>
                    <Label className="text-muted-foreground">Source</Label>
                    <p className="capitalize">
                      {selectedTexture.metadata.source}
                    </p>
                  </div>
                  <div>
                    <Label className="text-muted-foreground">Created</Label>
                    <p>
                      {selectedTexture.metadata.createdAt.toLocaleDateString()}
                    </p>
                  </div>
                  <div>
                    <Label className="text-muted-foreground">Modified</Label>
                    <p>
                      {selectedTexture.metadata.modifiedAt.toLocaleDateString()}
                    </p>
                  </div>
                  <div>
                    <Label className="text-muted-foreground">Usage Count</Label>
                    <p>{selectedTexture.usage.materialCount} materials</p>
                  </div>
                </div>

                {selectedTexture.metadata.originalFilename && (
                  <div>
                    <Label className="text-muted-foreground">
                      Original Filename
                    </Label>
                    <p>{selectedTexture.metadata.originalFilename}</p>
                  </div>
                )}

                {selectedTexture.metadata.tags.length > 0 && (
                  <div>
                    <Label className="text-muted-foreground">Tags</Label>
                    <div className="mt-1 flex flex-wrap gap-1">
                      {selectedTexture.metadata.tags.map((tag) => (
                        <Badge key={tag} variant="secondary">
                          {tag}
                        </Badge>
                      ))}
                    </div>
                  </div>
                )}

                {validation && (
                  <>
                    <Separator />
                    <div>
                      <Label className="text-muted-foreground">
                        Validation
                      </Label>
                      <div className="mt-2 space-y-2">
                        {validation.errors.length > 0 && (
                          <div className="text-red-600">
                            <h5 className="font-medium">Errors:</h5>
                            <ul className="list-inside list-disc text-sm">
                              {validation.errors.map((error) => (
                                <li key={error}>{error}</li>
                              ))}
                            </ul>
                          </div>
                        )}

                        {validation.warnings.length > 0 && (
                          <div className="text-yellow-600">
                            <h5 className="font-medium">Warnings:</h5>
                            <ul className="list-inside list-disc text-sm">
                              {validation.warnings.map((warning) => (
                                <li key={warning}>{warning}</li>
                              ))}
                            </ul>
                          </div>
                        )}

                        {validation.isValid &&
                          validation.errors.length === 0 &&
                          validation.warnings.length === 0 && (
                            <div className="text-green-600">
                              <p className="text-sm">✓ Texture is valid</p>
                            </div>
                          )}
                      </div>
                    </div>
                  </>
                )}
              </div>
            </TabsContent>
          </ScrollArea>
        </Tabs>
      </CardContent>
    </Card>
  );
}
