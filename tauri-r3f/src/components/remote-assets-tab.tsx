import {
  Box,
  Filter,
  Globe,
  Image,
  Package,
  Palette,
  Plus,
  Search,
  SortAsc,
  SortDesc,
  Type,
} from "lucide-react";
import { memo, useCallback, useMemo, useState } from "react";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Input } from "~/components/ui/input";
import { ScrollArea } from "~/components/ui/scroll-area";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { Tabs, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { cn } from "~/lib/utils";
import { useToast } from "~/hooks/use-toast";
import { useSceneStore } from "~/stores/scene-store";
import { useTextureStore } from "~/stores/texture-store";

// PMNDrs asset categories
export type PmndrsAssetCategory =
  | "models"
  | "textures"
  | "hdri"
  | "matcaps"
  | "normals"
  | "fonts";

// Asset metadata interface
interface PmndrsAsset {
  id: string;
  name: string;
  category: PmndrsAssetCategory;
  path: string;
  size: number;
  description?: string;
  preview?: string;
}

// PMNDrs asset definitions
const PMNDrs_ASSETS: PmndrsAsset[] = [
  // Models
  {
    id: "suzi",
    name: "Suzi",
    category: "models",
    path: "@pmndrs/assets/models/suzi.glb",
    size: 24_000,
    description: "Classic Suzanne monkey model",
  },

  // HDRIs (PMNDrs high-quality assets - different from preset environments)
  {
    id: "lebombo_apartment",
    name: "Lebombo Apartment",
    category: "hdri",
    path: "@pmndrs/assets/hdri/apartment.exr",
    size: 150_000,
    description: "High-quality indoor apartment environment (4K EXR)",
  },
  {
    id: "bridge",
    name: "Bridge",
    category: "hdri",
    path: "@pmndrs/assets/hdri/bridge.exr",
    size: 180_000,
    description: "Outdoor bridge scene over water",
  },
  {
    id: "potsdamer_city",
    name: "Potsdamer Platz",
    category: "hdri",
    path: "@pmndrs/assets/hdri/city.exr",
    size: 160_000,
    description: "High-quality urban city environment (4K EXR)",
  },
  {
    id: "forest_slope",
    name: "Forest Slope",
    category: "hdri",
    path: "@pmndrs/assets/hdri/forest.exr",
    size: 170_000,
    description: "High-quality forest landscape (4K EXR)",
  },
  {
    id: "studio_small",
    name: "Studio Small",
    category: "hdri",
    path: "@pmndrs/assets/hdri/studio.exr",
    size: 140_000,
    description: "High-quality photography studio (4K EXR)",
  },
  {
    id: "warehouse_empty",
    name: "Empty Warehouse",
    category: "hdri",
    path: "@pmndrs/assets/hdri/warehouse.exr",
    size: 155_000,
    description: "High-quality industrial warehouse (4K EXR)",
  },

  // Matcaps
  {
    id: "matcap-0000",
    name: "Matcap 0000",
    category: "matcaps",
    path: "@pmndrs/assets/matcaps/0000.webp",
    size: 8000,
    description: "Clay material appearance",
  },
  {
    id: "matcap-0001",
    name: "Matcap 0001",
    category: "matcaps",
    path: "@pmndrs/assets/matcaps/0001.webp",
    size: 8500,
    description: "Plastic material",
  },
  {
    id: "matcap-0002",
    name: "Matcap 0002",
    category: "matcaps",
    path: "@pmndrs/assets/matcaps/0002.webp",
    size: 8200,
    description: "Ceramic material",
  },

  // Normals
  {
    id: "normal-0000",
    name: "Normal 0000",
    category: "normals",
    path: "@pmndrs/assets/normals/0000.webp",
    size: 12_000,
    description: "Fabric normal map",
  },
  {
    id: "normal-0001",
    name: "Normal 0001",
    category: "normals",
    path: "@pmndrs/assets/normals/0001.webp",
    size: 11_800,
    description: "Stone normal map",
  },
  {
    id: "normal-0002",
    name: "Normal 0002",
    category: "normals",
    path: "@pmndrs/assets/normals/0002.webp",
    size: 11_500,
    description: "Wood normal map",
  },

  // Textures
  {
    id: "cloud",
    name: "Cloud",
    category: "textures",
    path: "@pmndrs/assets/textures/cloud.webp",
    size: 25_000,
    description: "Procedural cloud texture",
  },
  {
    id: "noise",
    name: "Noise",
    category: "textures",
    path: "@pmndrs/assets/textures/noise.webp",
    size: 15_000,
    description: "Noise texture",
  },

  // Fonts
  {
    id: "inter-regular",
    name: "Inter Regular",
    category: "fonts",
    path: "@pmndrs/assets/fonts/inter_regular.woff",
    size: 20_000,
    description: "Inter font regular weight",
  },
  {
    id: "inter-bold",
    name: "Inter Bold",
    category: "fonts",
    path: "@pmndrs/assets/fonts/inter_bold.woff",
    size: 21_000,
    description: "Inter font bold weight",
  },
  {
    id: "inter-regular-json",
    name: "Inter Regular (JSON)",
    category: "fonts",
    path: "@pmndrs/assets/fonts/inter_regular.json",
    size: 40_000,
    description: "Inter font regular weight (3D)",
  },
  {
    id: "inter-bold-json",
    name: "Inter Bold (JSON)",
    category: "fonts",
    path: "@pmndrs/assets/fonts/inter_bold.json",
    size: 42_000,
    description: "Inter font bold weight (3D)",
  },
];

// Category configurations
const CATEGORY_CONFIG = {
  models: {
    label: "Models",
    icon: Box,
    color: "text-blue-500",
    targetTab: "models",
    description: "3D models optimized for web",
  },
  textures: {
    label: "Textures",
    icon: Image,
    color: "text-green-500",
    targetTab: "textures",
    description: "Compressed textures and images",
  },
  hdri: {
    label: "Premium HDRIs",
    icon: Globe,
    color: "text-purple-500",
    targetTab: "settings", // HDRI goes to skybox in settings
    description: "High-quality 4K EXR environment maps (vs 1K presets)",
  },
  matcaps: {
    label: "Matcaps",
    icon: Palette,
    color: "text-pink-500",
    targetTab: "materials",
    description: "Material capture textures",
  },
  normals: {
    label: "Normals",
    icon: Package,
    color: "text-orange-500",
    targetTab: "textures",
    description: "Normal map textures",
  },
  fonts: {
    label: "Fonts",
    icon: Type,
    color: "text-cyan-500",
    targetTab: "fonts",
    description: "Optimized fonts for 2D and 3D text",
  },
} as const;

interface RemoteAssetsTabProps {
  className?: string;
}

export const RemoteAssetsTab = memo(function RemoteAssetsTab({
  className,
}: RemoteAssetsTabProps) {
  const [activeCategory, setActiveCategory] =
    useState<PmndrsAssetCategory>("models");
  const [searchQuery, setSearchQuery] = useState("");
  const [sortBy, setSortBy] = useState<"name" | "size" | "category">("name");
  const [sortOrder, setSortOrder] = useState<"asc" | "desc">("asc");

  // Store hooks
  const { loadTextureFromUrl } = useTextureStore();
  const { setSkyboxEnabled, setSkyboxPreset, setCustomEnvironmentFile } =
    useSceneStore();
  const { toast } = useToast();

  // Filter and sort assets
  const filteredAssets = useMemo(() => {
    const filtered = PMNDrs_ASSETS.filter((asset) => {
      const matchesCategory = asset.category === activeCategory;
      const matchesSearch =
        !searchQuery ||
        asset.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        asset.description?.toLowerCase().includes(searchQuery.toLowerCase());
      return matchesCategory && matchesSearch;
    });

    // Sort
    filtered.sort((a, b) => {
      let comparison = 0;
      switch (sortBy) {
        case "name":
          comparison = a.name.localeCompare(b.name);
          break;
        case "size":
          comparison = a.size - b.size;
          break;
        case "category":
          comparison = a.category.localeCompare(b.category);
          break;
      }
      return sortOrder === "asc" ? comparison : -comparison;
    });

    return filtered;
  }, [activeCategory, searchQuery, sortBy, sortOrder]);

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${Number.parseFloat((bytes / k ** i).toFixed(1))} ${sizes[i]}`;
  };

  const handleImportAsset = useCallback(
    async (asset: PmndrsAsset) => {
      try {
        // Dynamic import the asset
        const assetModule = await import(/* @vite-ignore */ asset.path);
        const assetUrl = assetModule.default;

        switch (asset.category) {
          case "models":
            // For models, we would need to download the file and create a File object
            // This requires additional implementation for fetching remote files
            console.log("Model import not yet implemented:", asset.name);
            toast({
              title: "Feature Not Available",
              description: `Model import for "${asset.name}" is not yet implemented. Please check back in a future update.`,
              variant: "default",
            });
            break;

          case "textures":
          case "matcaps":
          case "normals": {
            // Import as texture
            const textureType =
              asset.category === "normals"
                ? "normal"
                : asset.category === "matcaps"
                  ? "albedo"
                  : "albedo";
            await loadTextureFromUrl(assetUrl, asset.name, textureType);
            console.log(
              `Successfully imported ${asset.category} texture:`,
              asset.name
            );
            toast({
              title: "Asset Imported",
              description: `Successfully imported ${asset.category} texture: ${asset.name}`,
              variant: "default",
            });
            break;
          }

          case "hdri":
            // Set HDRI as custom environment file
            setSkyboxEnabled(true);
            setSkyboxPreset("custom"); // Use a custom preset mode
            setCustomEnvironmentFile(asset.path);
            console.log("HDRI environment set:", asset.name);
            toast({
              title: "Environment Set",
              description: `HDRI environment "${asset.name}" has been applied to your scene.`,
              variant: "default",
            });
            break;

          case "fonts":
            // Font import would require font management system
            console.log("Font import not yet implemented:", asset.name);
            toast({
              title: "Feature Not Available",
              description: `Font import for "${asset.name}" is not yet implemented. Please check back in a future update.`,
              variant: "default",
            });
            break;

          default:
            console.warn("Unknown asset category:", asset.category);
        }
      } catch (error) {
        console.error("Failed to import asset:", error);
        toast({
          title: "Import Failed",
          description: `Failed to import "${asset.name}". Please try again.`,
          variant: "destructive",
        });
      }
    },
    [
      loadTextureFromUrl,
      toast,
      setCustomEnvironmentFile, // Set HDRI as custom environment file
      setSkyboxEnabled,
      setSkyboxPreset,
    ]
  );

  const getCategoryIcon = (category: PmndrsAssetCategory) => {
    const config = CATEGORY_CONFIG[category];
    const IconComponent = config.icon;
    return <IconComponent className={cn("h-4 w-4", config.color)} />;
  };

  return (
    <Card className={cn("flex h-full w-full flex-col", className)}>
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center gap-2">
          <Globe className="h-5 w-5" />
          Remote Assets
          <span className="font-normal text-muted-foreground text-sm">
            PMNDrs Library - Premium HDRI & Assets
          </span>
        </CardTitle>
      </CardHeader>

      <CardContent className="flex flex-1 flex-col gap-4 p-4 pt-0">
        {/* Category tabs */}
        <Tabs
          onValueChange={(value) =>
            setActiveCategory(value as PmndrsAssetCategory)
          }
          value={activeCategory}
        >
          <TabsList className="grid w-full grid-cols-3 lg:grid-cols-6">
            {Object.entries(CATEGORY_CONFIG).map(([key, config]) => (
              <TabsTrigger
                className="flex flex-col gap-1 px-2 py-2 text-xs"
                key={key}
                value={key}
              >
                <config.icon className="h-4 w-4" />
                <span className="hidden sm:inline">{config.label}</span>
              </TabsTrigger>
            ))}
          </TabsList>

          {/* Toolbar */}
          <div className="mt-4 flex flex-wrap items-center gap-2">
            {/* Search */}
            <div className="relative min-w-48 flex-1">
              <Search className="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 transform text-muted-foreground" />
              <Input
                className="pl-9"
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search assets..."
                value={searchQuery}
              />
            </div>

            {/* Sort */}
            <Select
              onValueChange={(value: any) => setSortBy(value)}
              value={sortBy}
            >
              <SelectTrigger className="w-32">
                <Filter className="mr-2 h-4 w-4" />
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="name">Name</SelectItem>
                <SelectItem value="size">Size</SelectItem>
                <SelectItem value="category">Category</SelectItem>
              </SelectContent>
            </Select>

            <Button
              onClick={() => setSortOrder(sortOrder === "asc" ? "desc" : "asc")}
              size="sm"
              variant="outline"
            >
              {sortOrder === "asc" ? (
                <SortAsc className="h-4 w-4" />
              ) : (
                <SortDesc className="h-4 w-4" />
              )}
            </Button>
          </div>

          {/* Asset grid */}
          <ScrollArea className="flex-1">
            <div className="grid grid-cols-1 gap-4 p-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
              {filteredAssets.map((asset) => (
                <Card className="group overflow-hidden" key={asset.id}>
                  <CardContent className="p-4">
                    <div className="mb-3 flex aspect-square items-center justify-center rounded-lg bg-muted">
                      {getCategoryIcon(asset.category)}
                    </div>

                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <h4
                          className="truncate font-medium text-sm"
                          title={asset.name}
                        >
                          {asset.name}
                        </h4>
                        <Button
                          className="h-8 w-8 p-0 opacity-0 transition-opacity group-hover:opacity-100"
                          onClick={() => handleImportAsset(asset)}
                          size="sm"
                          variant="ghost"
                        >
                          <Plus className="h-4 w-4" />
                        </Button>
                      </div>

                      <p className="line-clamp-2 text-muted-foreground text-xs">
                        {asset.description}
                      </p>

                      <div className="flex items-center justify-between text-muted-foreground text-xs">
                        <span>{formatFileSize(asset.size)}</span>
                        <span className="capitalize">{asset.category}</span>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              ))}

              {filteredAssets.length === 0 && (
                <div className="col-span-full flex flex-col items-center justify-center py-12 text-center">
                  <Globe className="mb-4 h-12 w-12 text-muted-foreground" />
                  <h3 className="mb-2 font-medium text-lg">No assets found</h3>
                  <p className="mb-4 text-muted-foreground">
                    {searchQuery
                      ? "Try adjusting your search terms."
                      : `No ${activeCategory} assets available.`}
                  </p>
                </div>
              )}
            </div>
          </ScrollArea>
        </Tabs>
      </CardContent>
    </Card>
  );
});
