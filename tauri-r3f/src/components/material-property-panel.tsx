import {
  Copy,
  Image as ImageIcon,
  Layers,
  Library,
  Palette,
  Settings,
  Sliders,
  Sparkles,
  Trash2,
  Zap,
} from "lucide-react";
import { useState } from "react";
import * as THREE from "three";
import { CompactTextureSelector } from "~/components/texture-selector";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "~/components/ui/collapsible";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
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
import {
  type MaterialType,
  useMaterialStore,
  useSelectedMaterial,
} from "~/stores/material-store";
import { useTextureStore } from "~/stores/texture-store";
import type {
  MaterialProperties,
  StandardMaterialProperties,
} from "~/types/materials";

/**
 * Material presets for quick material setup
 */
const MATERIAL_PRESETS = [
  {
    id: "plastic_white",
    name: "White Plastic",
    category: "Plastic",
    description: "Smooth white plastic material",
    properties: {
      color: "#ffffff",
      metalness: 0.0,
      roughness: 0.4,
      opacity: 1.0,
      transparent: false,
    },
  },
  {
    id: "plastic_black",
    name: "Black Plastic",
    category: "Plastic",
    description: "Smooth black plastic material",
    properties: {
      color: "#000000",
      metalness: 0.0,
      roughness: 0.4,
      opacity: 1.0,
      transparent: false,
    },
  },
  {
    id: "plastic_red",
    name: "Red Plastic",
    category: "Plastic",
    description: "Smooth red plastic material",
    properties: {
      color: "#ff0000",
      metalness: 0.0,
      roughness: 0.4,
      opacity: 1.0,
      transparent: false,
    },
  },
  {
    id: "metal_gold",
    name: "Gold",
    category: "Metal",
    description: "Shiny gold metal material",
    properties: {
      color: "#ffd700",
      metalness: 1.0,
      roughness: 0.1,
      opacity: 1.0,
      transparent: false,
    },
  },
  {
    id: "metal_silver",
    name: "Silver",
    category: "Metal",
    description: "Shiny silver metal material",
    properties: {
      color: "#c0c0c0",
      metalness: 1.0,
      roughness: 0.1,
      opacity: 1.0,
      transparent: false,
    },
  },
  {
    id: "wood_oak",
    name: "Oak Wood",
    category: "Wood",
    description: "Natural oak wood texture",
    properties: {
      color: "#8b4513",
      metalness: 0.0,
      roughness: 0.8,
      opacity: 1.0,
      transparent: false,
    },
  },
];

/**
 * Get unique material preset categories
 */
function getMaterialPresetCategories(): string[] {
  const categories = new Set(MATERIAL_PRESETS.map((preset) => preset.category));
  return Array.from(categories).sort();
}

/**
 * Get material presets by category
 */
function getMaterialPresetsByCategory(category: string) {
  return MATERIAL_PRESETS.filter((preset) => preset.category === category);
}

export function MaterialPropertyPanel() {
  const selectedMaterial = useSelectedMaterial();
  const {
    updateMaterial,
    duplicateMaterial,
    deleteMaterial,
    selectMaterial,
    createMaterial,
  } = useMaterialStore();
  const { trackTextureUsage, untrackTextureUsage } = useTextureStore();

  const [expandedSections, setExpandedSections] = useState<Set<string>>(
    new Set(["basic", "pbr"])
  );
  const [selectedPresetCategory, setSelectedPresetCategory] =
    useState<string>("Plastic");

  if (!selectedMaterial) {
    return (
      <div className="p-4">
        <Card>
          <CardContent className="pt-6">
            <div className="space-y-4 text-center text-muted-foreground text-sm">
              <Palette className="mx-auto h-8 w-8 opacity-50" />
              <p>Select a material to edit its properties</p>
              <div className="space-x-2">
                <Button
                  onClick={() => {
                    const id = createMaterial("New Material", "standard");
                    selectMaterial(id);
                  }}
                  size="sm"
                >
                  Create Standard Material
                </Button>
                <Button
                  onClick={() => {
                    const id = createMaterial("New Basic Material", "basic");
                    selectMaterial(id);
                  }}
                  size="sm"
                  variant="outline"
                >
                  Create Basic Material
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    );
  }

  const handlePropertyChange = (
    property: string,
    value: string | number | boolean | THREE.Side | null
  ) => {
    updateMaterial(selectedMaterial.id, { [property]: value });
  };

  const handleApplyPreset = (presetId: string) => {
    const preset = MATERIAL_PRESETS.find(
      (p: (typeof MATERIAL_PRESETS)[0]) => p.id === presetId
    );
    if (preset && selectedMaterial) {
      updateMaterial(selectedMaterial.id, preset.properties);
    }
  };

  const toggleSection = (section: string) => {
    const newExpanded = new Set(expandedSections);
    if (newExpanded.has(section)) {
      newExpanded.delete(section);
    } else {
      newExpanded.add(section);
    }
    setExpandedSections(newExpanded);
  };

  const renderBasicProperties = () => (
    <Collapsible
      onOpenChange={() => toggleSection("basic")}
      open={expandedSections.has("basic")}
    >
      <CollapsibleTrigger>
        <Card className="w-full">
          <CardHeader className="pb-3">
            <div className="flex items-center justify-between">
              <CardTitle className="flex items-center gap-2 font-medium text-sm">
                <Settings className="h-4 w-4" />
                Basic Properties
              </CardTitle>
              <Badge variant="secondary">
                {selectedMaterial.properties.type}
              </Badge>
            </div>
          </CardHeader>
        </Card>
      </CollapsibleTrigger>
      <CollapsibleContent>
        <Card className="w-full">
          <CardContent className="space-y-4 pt-0">
            {/* Material Name */}
            <div className="space-y-2">
              <Label className="font-medium text-xs" htmlFor="material-name">
                Name
              </Label>
              <Input
                className="h-8"
                id="material-name"
                onChange={(e) => handlePropertyChange("name", e.target.value)}
                value={selectedMaterial.properties.name}
              />
            </div>

            {/* Material Type */}
            <div className="space-y-2">
              <Label className="font-medium text-xs">Type</Label>
              <Select
                onValueChange={(value) => {
                  if (value) {
                    const materialType = value as MaterialType;
                    // Note: In a full implementation, we'd need to convert properties
                    // For now, we'll create a new material of the selected type
                    const newId = createMaterial(
                      `${selectedMaterial.properties.name} (${materialType})`,
                      materialType
                    );
                    selectMaterial(newId);
                  }
                }}
                value={selectedMaterial.properties.type}
              >
                <SelectTrigger className="h-8">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {/* Standard Materials */}
                  <SelectItem value="basic">Basic</SelectItem>
                  <SelectItem value="standard">Standard (PBR)</SelectItem>
                  <SelectItem value="physical">Physical</SelectItem>
                  <SelectItem value="lambert">Lambert</SelectItem>
                  <SelectItem value="phong">Phong</SelectItem>
                  <SelectItem value="toon">Toon</SelectItem>
                  <SelectItem value="normal">Normal</SelectItem>
                  <SelectItem value="depth">Depth</SelectItem>

                  {/* Custom Materials */}
                  <SelectItem value="subsurface">
                    Subsurface Scattering
                  </SelectItem>
                  <SelectItem value="anisotropic">Anisotropic</SelectItem>
                  <SelectItem value="iridescent">Iridescent</SelectItem>
                  <SelectItem value="velvet">Velvet</SelectItem>
                  <SelectItem value="holographic">Holographic</SelectItem>
                  <SelectItem value="gem">Gem</SelectItem>
                  <SelectItem value="fabric">Fabric</SelectItem>
                  <SelectItem value="skin">Skin</SelectItem>
                  <SelectItem value="advanced-toon">Advanced Toon</SelectItem>
                  <SelectItem value="realistic-glass">
                    Realistic Glass
                  </SelectItem>
                  <SelectItem value="metallic-paint">Metallic Paint</SelectItem>
                </SelectContent>
              </Select>
            </div>

            {/* Base Color */}
            <div className="space-y-2">
              <Label className="font-medium text-xs" htmlFor="base-color">
                Base Color
              </Label>
              <div className="flex gap-2">
                <Input
                  className="h-8 flex-1"
                  id="base-color"
                  onChange={(e) =>
                    handlePropertyChange("color", e.target.value)
                  }
                  type="color"
                  value={selectedMaterial.properties.color}
                />
                <Input
                  className="h-8 w-20"
                  onChange={(e) =>
                    handlePropertyChange("color", e.target.value)
                  }
                  value={selectedMaterial.properties.color}
                />
              </div>
            </div>

            {/* Opacity */}
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label className="font-medium text-xs">Opacity</Label>
                <span className="text-muted-foreground text-xs">
                  {(selectedMaterial.properties.opacity ?? 1).toFixed(2)}
                </span>
              </div>
              <Slider
                className="w-full"
                max={1}
                min={0}
                onValueChange={(values) =>
                  handlePropertyChange(
                    "opacity",
                    Array.isArray(values) ? values[0] : values
                  )
                }
                step={0.01}
                value={[selectedMaterial.properties.opacity ?? 1]}
              />
            </div>

            {/* Transparency */}
            <div className="flex items-center justify-between">
              <Label className="font-medium text-xs" htmlFor="transparent">
                Transparent
              </Label>
              <Switch
                checked={selectedMaterial.properties.transparent}
                id="transparent"
                onCheckedChange={(checked) =>
                  handlePropertyChange("transparent", checked)
                }
              />
            </div>

            {/* Wireframe */}
            <div className="flex items-center justify-between">
              <Label className="font-medium text-xs" htmlFor="wireframe">
                Wireframe
              </Label>
              <Switch
                checked={selectedMaterial.properties.wireframe}
                id="wireframe"
                onCheckedChange={(checked) =>
                  handlePropertyChange("wireframe", checked)
                }
              />
            </div>

            {/* Double Sided */}
            <div className="space-y-2">
              <Label className="font-medium text-xs">Side Rendering</Label>
              <Select
                onValueChange={(value) =>
                  handlePropertyChange(
                    "side",
                    value ? Number.parseInt(value, 10) : THREE.FrontSide
                  )
                }
                value={selectedMaterial.properties.side.toString()}
              >
                <SelectTrigger className="h-8">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value={THREE.FrontSide.toString()}>
                    Front Side
                  </SelectItem>
                  <SelectItem value={THREE.BackSide.toString()}>
                    Back Side
                  </SelectItem>
                  <SelectItem value={THREE.DoubleSide.toString()}>
                    Double Side
                  </SelectItem>
                </SelectContent>
              </Select>
            </div>
          </CardContent>
        </Card>
      </CollapsibleContent>
    </Collapsible>
  );

  const renderPBRProperties = () => {
    if (!["standard", "physical"].includes(selectedMaterial.properties.type)) {
      return null;
    }

    const props = selectedMaterial.properties as StandardMaterialProperties;

    return (
      <Collapsible
        onOpenChange={() => toggleSection("pbr")}
        open={expandedSections.has("pbr")}
      >
        <CollapsibleTrigger>
          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="flex items-center gap-2 font-medium text-sm">
                <Sparkles className="h-4 w-4" />
                PBR Properties
              </CardTitle>
            </CardHeader>
          </Card>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <Card>
            <CardContent className="space-y-4 pt-0">
              {/* Metalness */}
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label className="font-medium text-xs">Metalness</Label>
                  <span className="text-muted-foreground text-xs">
                    {props.metalness.toFixed(2)}
                  </span>
                </div>
                <Slider
                  className="w-full"
                  max={1}
                  min={0}
                  onValueChange={(values) =>
                    handlePropertyChange(
                      "metalness",
                      Array.isArray(values) ? values[0] : values
                    )
                  }
                  step={0.01}
                  value={[props.metalness]}
                />
              </div>

              {/* Roughness */}
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label className="font-medium text-xs">Roughness</Label>
                  <span className="text-muted-foreground text-xs">
                    {props.roughness.toFixed(2)}
                  </span>
                </div>
                <Slider
                  className="w-full"
                  max={1}
                  min={0}
                  onValueChange={(values) =>
                    handlePropertyChange(
                      "roughness",
                      Array.isArray(values) ? values[0] : values
                    )
                  }
                  step={0.01}
                  value={[props.roughness]}
                />
              </div>

              {/* Emissive */}
              <div className="space-y-2">
                <Label className="font-medium text-xs" htmlFor="emissive">
                  Emissive Color
                </Label>
                <div className="flex gap-2">
                  <Input
                    className="h-8 flex-1"
                    id="emissive"
                    onChange={(e) =>
                      handlePropertyChange("emissive", e.target.value)
                    }
                    type="color"
                    value={props.emissive}
                  />
                  <Input
                    className="h-8 w-20"
                    onChange={(e) =>
                      handlePropertyChange("emissive", e.target.value)
                    }
                    value={props.emissive}
                  />
                </div>
              </div>

              {/* Emissive Intensity */}
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label className="font-medium text-xs">
                    Emissive Intensity
                  </Label>
                  <span className="text-muted-foreground text-xs">
                    {props.emissiveIntensity.toFixed(2)}
                  </span>
                </div>
                <Slider
                  className="w-full"
                  max={10}
                  min={0}
                  onValueChange={(values) =>
                    handlePropertyChange(
                      "emissiveIntensity",
                      Array.isArray(values) ? values[0] : values
                    )
                  }
                  step={0.1}
                  value={[props.emissiveIntensity]}
                />
              </div>

              {/* Clearcoat */}
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label className="font-medium text-xs">Clearcoat</Label>
                  <span className="text-muted-foreground text-xs">
                    {props.clearcoat.toFixed(2)}
                  </span>
                </div>
                <Slider
                  className="w-full"
                  max={1}
                  min={0}
                  onValueChange={(values) =>
                    handlePropertyChange(
                      "clearcoat",
                      Array.isArray(values) ? values[0] : values
                    )
                  }
                  step={0.01}
                  value={[props.clearcoat]}
                />
              </div>

              {/* Transmission */}
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label className="font-medium text-xs">Transmission</Label>
                  <span className="text-muted-foreground text-xs">
                    {props.transmission.toFixed(2)}
                  </span>
                </div>
                <Slider
                  className="w-full"
                  max={1}
                  min={0}
                  onValueChange={(values) =>
                    handlePropertyChange(
                      "transmission",
                      Array.isArray(values) ? values[0] : values
                    )
                  }
                  step={0.01}
                  value={[props.transmission]}
                />
              </div>
            </CardContent>
          </Card>
        </CollapsibleContent>
      </Collapsible>
    );
  };

  const renderTextureProperties = () => {
    const handleTextureChange = (
      textureSlot: keyof StandardMaterialProperties,
      textureId: string | null
    ) => {
      if (!selectedMaterial) return;

      const materialId = selectedMaterial.id;

      // Handle texture assignment
      if (textureId) {
        // Track usage for the new texture
        trackTextureUsage(textureId, materialId);

        // Update material properties
        updateMaterial(selectedMaterial.id, {
          [textureSlot]: {
            ...(selectedMaterial.properties[
              textureSlot as keyof MaterialProperties
            ] as any),
            texture: textureId,
          },
        } as Partial<MaterialProperties>);
      } else {
        // Clear texture assignment
        const currentTextureId = (selectedMaterial.properties as any)[
          textureSlot
        ]?.texture;
        if (currentTextureId) {
          untrackTextureUsage(currentTextureId, materialId);
        }

        updateMaterial(selectedMaterial.id, {
          [textureSlot]: {
            ...(selectedMaterial.properties[
              textureSlot as keyof MaterialProperties
            ] as any),
            texture: null,
          },
        } as Partial<MaterialProperties>);
      }
    };

    const getCurrentTextureId = (
      textureSlot: keyof StandardMaterialProperties
    ) => {
      return (
        (selectedMaterial?.properties as any)?.[textureSlot]?.texture || null
      );
    };

    return (
      <Collapsible
        onOpenChange={() => toggleSection("textures")}
        open={expandedSections.has("textures")}
      >
        <CollapsibleTrigger>
          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="flex items-center gap-2 font-medium text-sm">
                <ImageIcon className="h-4 w-4" />
                Textures
              </CardTitle>
            </CardHeader>
          </Card>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <Card>
            <CardContent className="space-y-4 pt-0">
              <Tabs className="w-full" defaultValue="base">
                <TabsList className="grid w-full grid-cols-4">
                  <TabsTrigger value="base">Base</TabsTrigger>
                  <TabsTrigger value="normal">Normal</TabsTrigger>
                  <TabsTrigger value="metal">Metal</TabsTrigger>
                  <TabsTrigger value="advanced">Advanced</TabsTrigger>
                </TabsList>

                <TabsContent className="space-y-4" value="base">
                  {/* Base Color Map */}
                  <div className="space-y-2">
                    <Label className="font-medium text-xs">
                      Base Color Map
                    </Label>
                    <CompactTextureSelector
                      onTextureSelect={(textureId) =>
                        handleTextureChange("map", textureId)
                      }
                      placeholder="Select base color texture..."
                      selectedTextureId={getCurrentTextureId("map")}
                      textureType="albedo"
                    />
                  </div>

                  {/* Alpha Map */}
                  <div className="space-y-2">
                    <Label className="font-medium text-xs">Alpha Map</Label>
                    <CompactTextureSelector
                      onTextureSelect={(textureId) =>
                        handleTextureChange("alphaMap", textureId)
                      }
                      placeholder="Select alpha texture..."
                      selectedTextureId={getCurrentTextureId("alphaMap")}
                      textureType="opacity"
                    />
                  </div>

                  {/* AO Map */}
                  <div className="space-y-2">
                    <Label className="font-medium text-xs">
                      Ambient Occlusion Map
                    </Label>
                    <CompactTextureSelector
                      onTextureSelect={(textureId) =>
                        handleTextureChange("aoMap", textureId)
                      }
                      placeholder="Select AO texture..."
                      selectedTextureId={getCurrentTextureId("aoMap")}
                      textureType="ao"
                    />
                  </div>
                </TabsContent>

                <TabsContent className="space-y-4" value="normal">
                  {/* Normal Map */}
                  <div className="space-y-2">
                    <Label className="font-medium text-xs">Normal Map</Label>
                    <CompactTextureSelector
                      onTextureSelect={(textureId) =>
                        handleTextureChange("normalMap", textureId)
                      }
                      placeholder="Select normal texture..."
                      selectedTextureId={getCurrentTextureId("normalMap")}
                      textureType="normal"
                    />
                  </div>

                  {/* Bump Map */}
                  <div className="space-y-2">
                    <Label className="font-medium text-xs">Bump Map</Label>
                    <CompactTextureSelector
                      onTextureSelect={(textureId) =>
                        handleTextureChange("bumpMap", textureId)
                      }
                      placeholder="Select bump texture..."
                      selectedTextureId={getCurrentTextureId("bumpMap")}
                      textureType="height"
                    />
                  </div>
                </TabsContent>

                <TabsContent className="space-y-4" value="metal">
                  {/* Metalness Map */}
                  <div className="space-y-2">
                    <Label className="font-medium text-xs">Metalness Map</Label>
                    <CompactTextureSelector
                      onTextureSelect={(textureId) =>
                        handleTextureChange("metalnessMap", textureId)
                      }
                      placeholder="Select metalness texture..."
                      selectedTextureId={getCurrentTextureId("metalnessMap")}
                      textureType="metallic"
                    />
                  </div>

                  {/* Roughness Map */}
                  <div className="space-y-2">
                    <Label className="font-medium text-xs">Roughness Map</Label>
                    <CompactTextureSelector
                      onTextureSelect={(textureId) =>
                        handleTextureChange("roughnessMap", textureId)
                      }
                      placeholder="Select roughness texture..."
                      selectedTextureId={getCurrentTextureId("roughnessMap")}
                      textureType="roughness"
                    />
                  </div>
                </TabsContent>

                <TabsContent className="space-y-4" value="advanced">
                  {/* Emissive Map */}
                  <div className="space-y-2">
                    <Label className="font-medium text-xs">Emissive Map</Label>
                    <CompactTextureSelector
                      onTextureSelect={(textureId) =>
                        handleTextureChange("emissiveMap", textureId)
                      }
                      placeholder="Select emissive texture..."
                      selectedTextureId={getCurrentTextureId("emissiveMap")}
                      textureType="emissive"
                    />
                  </div>

                  {/* Environment Map */}
                  <div className="space-y-2">
                    <Label className="font-medium text-xs">
                      Environment Map
                    </Label>
                    <CompactTextureSelector
                      onTextureSelect={(textureId) =>
                        handleTextureChange("envMap", textureId)
                      }
                      placeholder="Select environment texture..."
                      selectedTextureId={getCurrentTextureId("envMap")}
                      textureType="environment"
                    />
                  </div>

                  {/* Light Map */}
                  <div className="space-y-2">
                    <Label className="font-medium text-xs">Light Map</Label>
                    <CompactTextureSelector
                      onTextureSelect={(textureId) =>
                        handleTextureChange("lightMap", textureId)
                      }
                      placeholder="Select light texture..."
                      selectedTextureId={getCurrentTextureId("lightMap")}
                    />
                  </div>
                </TabsContent>
              </Tabs>
            </CardContent>
          </Card>
        </CollapsibleContent>
      </Collapsible>
    );
  };

  const renderAdvancedProperties = () => (
    <Collapsible
      onOpenChange={() => toggleSection("advanced")}
      open={expandedSections.has("advanced")}
    >
      <CollapsibleTrigger>
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="flex items-center gap-2 font-medium text-sm">
              <Sliders className="h-4 w-4" />
              Advanced Properties
            </CardTitle>
          </CardHeader>
        </Card>
      </CollapsibleTrigger>
      <CollapsibleContent>
        <Card>
          <CardContent className="space-y-4 pt-0">
            {/* Shadow Settings */}
            <div className="space-y-3">
              <Label className="font-medium text-xs">Shadow Settings</Label>

              <div className="flex items-center justify-between">
                <Label className="text-xs" htmlFor="cast-shadow">
                  Cast Shadow
                </Label>
                <Switch
                  checked={selectedMaterial.properties.castShadow}
                  id="cast-shadow"
                  onCheckedChange={(checked) =>
                    handlePropertyChange("castShadow", checked)
                  }
                />
              </div>

              <div className="flex items-center justify-between">
                <Label className="text-xs" htmlFor="receive-shadow">
                  Receive Shadow
                </Label>
                <Switch
                  checked={selectedMaterial.properties.receiveShadow}
                  id="receive-shadow"
                  onCheckedChange={(checked) =>
                    handlePropertyChange("receiveShadow", checked)
                  }
                />
              </div>
            </div>

            <Separator />

            {/* Rendering Settings */}
            <div className="space-y-3">
              <Label className="font-medium text-xs">Rendering Settings</Label>

              <div className="flex items-center justify-between">
                <Label className="text-xs" htmlFor="fog">
                  Affected by Fog
                </Label>
                <Switch
                  checked={selectedMaterial.properties.fog}
                  id="fog"
                  onCheckedChange={(checked) =>
                    handlePropertyChange("fog", checked)
                  }
                />
              </div>

              <div className="flex items-center justify-between">
                <Label className="text-xs" htmlFor="flat-shading">
                  Flat Shading
                </Label>
                <Switch
                  checked={selectedMaterial.properties.flatShading}
                  id="flat-shading"
                  onCheckedChange={(checked) =>
                    handlePropertyChange("flatShading", checked)
                  }
                />
              </div>

              <div className="flex items-center justify-between">
                <Label className="text-xs" htmlFor="vertex-colors">
                  Use Vertex Colors
                </Label>
                <Switch
                  checked={selectedMaterial.properties.vertexColors}
                  id="vertex-colors"
                  onCheckedChange={(checked) =>
                    handlePropertyChange("vertexColors", checked)
                  }
                />
              </div>
            </div>
          </CardContent>
        </Card>
      </CollapsibleContent>
    </Collapsible>
  );

  const renderPresetsSection = () => (
    <Collapsible
      onOpenChange={() => toggleSection("presets")}
      open={expandedSections.has("presets")}
    >
      <CollapsibleTrigger>
        <Card className="w-full">
          <CardHeader className="pb-3">
            <div className="flex items-center justify-between">
              <CardTitle className="flex items-center gap-2 font-medium text-sm">
                <Library className="h-4 w-4" />
                Material Presets
              </CardTitle>
            </div>
          </CardHeader>
        </Card>
      </CollapsibleTrigger>
      <CollapsibleContent>
        <Card className="w-full">
          <CardContent className="space-y-4 pt-0">
            {/* Category Selection */}
            <div className="space-y-2">
              <Label className="font-medium text-xs">Category</Label>
              <Select
                onValueChange={(value) =>
                  setSelectedPresetCategory(value || "Plastic")
                }
                value={selectedPresetCategory}
              >
                <SelectTrigger className="h-8">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {getMaterialPresetCategories().map((category) => (
                    <SelectItem key={category} value={category}>
                      {category}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            {/* Presets Grid */}
            <div className="space-y-2">
              <Label className="font-medium text-xs">Presets</Label>
              <div className="grid max-h-48 grid-cols-2 gap-2 overflow-y-auto">
                {getMaterialPresetsByCategory(selectedPresetCategory).map(
                  (preset) => (
                    <Button
                      className="flex h-auto flex-col items-start gap-1 p-2 text-left"
                      key={preset.id}
                      onClick={() => handleApplyPreset(preset.id)}
                      size="sm"
                      title={preset.description}
                      variant="outline"
                    >
                      <div className="flex w-full items-center gap-2">
                        <div
                          className="h-4 w-4 rounded border"
                          style={{ backgroundColor: preset.properties.color }}
                        />
                        <span className="flex-1 truncate font-medium text-xs">
                          {preset.name}
                        </span>
                      </div>
                      {preset.description && (
                        <span className="w-full truncate text-muted-foreground text-xs">
                          {preset.description}
                        </span>
                      )}
                    </Button>
                  )
                )}
              </div>
            </div>
          </CardContent>
        </Card>
      </CollapsibleContent>
    </Collapsible>
  );

  return (
    <div className="space-y-4 p-4">
      {/* Material Header */}
      <Card>
        <CardHeader className="pb-3">
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2 font-medium text-sm">
              <Palette className="h-4 w-4" />
              Material Editor
            </CardTitle>
            <div className="flex items-center gap-1">
              <Button
                onClick={() => {
                  const newId = duplicateMaterial(selectedMaterial.id);
                  selectMaterial(newId);
                }}
                size="sm"
                variant="ghost"
              >
                <Copy className="h-4 w-4" />
              </Button>
              <Button
                onClick={() => deleteMaterial(selectedMaterial.id)}
                size="sm"
                variant="ghost"
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </CardHeader>
      </Card>

      {/* Material Properties */}
      {renderPresetsSection()}

      {renderBasicProperties()}

      {renderPBRProperties()}

      {renderTextureProperties()}

      {renderAdvancedProperties()}

      {/* Material Actions */}
      <Card>
        <CardContent className="pt-6">
          <div className="flex gap-2">
            <Button
              onClick={() => {
                const newId = createMaterial("New Material", "standard");
                selectMaterial(newId);
              }}
              size="sm"
            >
              <Zap className="mr-2 h-4 w-4" />
              New Material
            </Button>
            <Button size="sm" variant="outline">
              <Layers className="mr-2 h-4 w-4" />
              Import Material
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
