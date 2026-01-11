import {
  Check,
  FileImage,
  Image as ImageIcon,
  Plus,
  Search,
  X,
} from "lucide-react";
import type React from "react";
import { useCallback, useState } from "react";
import { TextureThumbnail } from "~/components/texture-preview";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "~/components/ui/dialog";
import { Input } from "~/components/ui/input";
import { ScrollArea } from "~/components/ui/scroll-area";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { cn } from "~/lib/utils";
import { useFilteredTextures, useTextureStore } from "~/stores/texture-store";

interface TextureSelectorProps {
  /** Currently selected texture ID */
  selectedTextureId?: string;
  /** Callback when texture is selected */
  onTextureSelect: (textureId: string | null) => void;
  /** Filter textures by type */
  textureType?: string;
  /** Custom button content */
  children?: React.ReactNode;
  /** Additional CSS classes */
  className?: string;
  /** Whether to show the clear option */
  showClear?: boolean;
}

export function TextureSelector({
  selectedTextureId,
  onTextureSelect,
  textureType,
  children,
  className,
  showClear = true,
}: TextureSelectorProps) {
  const [open, setOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedTab, setSelectedTab] = useState("library");

  const {
    getTextureById,
    setFilterSearch,
    setFilterType,
    selectTexture,
    createProceduralTexture,
  } = useTextureStore();

  const filteredTextures = useFilteredTextures();
  const selectedTexture = selectedTextureId
    ? getTextureById(selectedTextureId)
    : null;

  const handleTextureClick = useCallback(
    (textureId: string) => {
      selectTexture(textureId);
      onTextureSelect(textureId);
      setOpen(false);
    },
    [selectTexture, onTextureSelect]
  );

  const handleClearTexture = useCallback(() => {
    onTextureSelect(null);
    setOpen(false);
  }, [onTextureSelect]);

  const handleCreateProcedural = useCallback(
    (type: string) => {
      const textureId = createProceduralTexture(
        type as any,
        type === "color"
          ? { color: "#808080" }
          : type === "checkerboard"
            ? { color1: "#ffffff", color2: "#cccccc", size: 256 }
            : type === "normal"
              ? { size: 256 }
              : type === "roughness"
                ? { roughness: 0.5, size: 256 }
                : { metalness: 0.0, size: 256 },
        `${type.charAt(0).toUpperCase() + type.slice(1)} Texture`
      );
      handleTextureClick(textureId);
    },
    [createProceduralTexture, handleTextureClick]
  );

  const handleSearchChange = useCallback(
    (query: string) => {
      setSearchQuery(query);
      setFilterSearch(query);
    },
    [setFilterSearch]
  );

  const _handleTypeFilterChange = useCallback(
    (type: string) => {
      setFilterType(type === "all" ? "all" : type);
    },
    [setFilterType]
  );

  const proceduralTypes = [
    { id: "color", name: "Color", description: "Solid color texture" },
    {
      id: "checkerboard",
      name: "Checkerboard",
      description: "Checkerboard pattern",
    },
    { id: "normal", name: "Normal", description: "Default normal map" },
    { id: "roughness", name: "Roughness", description: "Constant roughness" },
    { id: "metallic", name: "Metallic", description: "Constant metalness" },
  ];

  return (
    <Dialog onOpenChange={setOpen} open={open}>
      <DialogTrigger asChild>
        {children || (
          <Button
            className={cn("flex items-center gap-2", className)}
            variant="outline"
          >
            {selectedTexture ? (
              <>
                <div className="h-4 w-4 flex-shrink-0 overflow-hidden rounded border">
                  {selectedTexture.thumbnail && (
                    <img
                      alt={selectedTexture.name}
                      className="h-full w-full object-cover"
                      height={16}
                      src={selectedTexture.thumbnail}
                      width={16}
                    />
                  )}
                </div>
                <span className="truncate">{selectedTexture.name}</span>
              </>
            ) : (
              <>
                <FileImage className="h-4 w-4" />
                Select Texture
              </>
            )}
          </Button>
        )}
      </DialogTrigger>

      <DialogContent className="flex max-h-[80vh] max-w-4xl flex-col">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <ImageIcon className="h-5 w-5" />
            Select Texture
          </DialogTitle>
          <DialogDescription>
            Choose a texture from your library or create a procedural one.
          </DialogDescription>
        </DialogHeader>

        <div className="flex min-h-0 flex-1 flex-col gap-4">
          <Tabs
            className="flex flex-1 flex-col"
            onValueChange={setSelectedTab}
            value={selectedTab}
          >
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="library">Texture Library</TabsTrigger>
              <TabsTrigger value="procedural">Procedural</TabsTrigger>
            </TabsList>

            <TabsContent
              className="flex flex-1 flex-col space-y-4"
              value="library"
            >
              {/* Search and filters */}
              <div className="flex items-center gap-2">
                <div className="relative flex-1">
                  <Search className="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 transform text-muted-foreground" />
                  <Input
                    className="pl-9"
                    onChange={(e) => handleSearchChange(e.target.value)}
                    placeholder="Search textures..."
                    value={searchQuery}
                  />
                </div>

                {textureType && (
                  <Badge variant="secondary">{textureType} textures only</Badge>
                )}
              </div>

              {/* Texture grid */}
              <ScrollArea className="flex-1">
                <div className="grid grid-cols-4 gap-2 p-1 sm:grid-cols-6 md:grid-cols-8">
                  {/* Clear option */}
                  {showClear && (
                    <button
                      className={cn(
                        "flex aspect-square cursor-pointer flex-col items-center justify-center rounded-lg border-2 text-muted-foreground transition-all hover:border-primary hover:text-foreground hover:shadow-md focus:outline-none focus:ring-2 focus:ring-primary",
                        selectedTextureId
                          ? "border-border"
                          : "border-primary bg-primary/5"
                      )}
                      onClick={handleClearTexture}
                      type="button"
                    >
                      <X className="mb-1 h-6 w-6" />
                      <span className="text-center text-xs">No Texture</span>
                    </button>
                  )}

                  {/* Texture thumbnails */}
                  {filteredTextures.map((texture) => (
                    <button
                      className={cn(
                        "relative cursor-pointer overflow-hidden rounded-lg border-2 transition-all hover:shadow-md focus:outline-none focus:ring-2 focus:ring-primary",
                        texture.id === selectedTextureId
                          ? "border-primary shadow-md"
                          : "border-border"
                      )}
                      key={texture.id}
                      onClick={() => handleTextureClick(texture.id)}
                      type="button"
                    >
                      <TextureThumbnail
                        className="h-full w-full"
                        size={80}
                        texture={texture.texture}
                        textureName={texture.name}
                      />

                      {/* Selection indicator */}
                      {texture.id === selectedTextureId && (
                        <div className="absolute top-1 right-1 flex h-4 w-4 items-center justify-center rounded-full bg-primary">
                          <Check className="h-3 w-3 text-primary-foreground" />
                        </div>
                      )}

                      {/* Type badge */}
                      <Badge
                        className="absolute bottom-1 left-1 px-1 py-0 text-xs"
                        variant="secondary"
                      >
                        {texture.type}
                      </Badge>
                    </button>
                  ))}
                </div>

                {filteredTextures.length === 0 && (
                  <div className="flex flex-col items-center justify-center py-12 text-center">
                    <ImageIcon className="mb-4 h-12 w-12 text-muted-foreground" />
                    <h3 className="mb-2 font-medium text-lg">
                      No textures found
                    </h3>
                    <p className="mb-4 text-muted-foreground">
                      {searchQuery
                        ? "Try a different search term."
                        : "Import some textures to get started."}
                    </p>
                  </div>
                )}
              </ScrollArea>
            </TabsContent>

            <TabsContent className="flex-1" value="procedural">
              <ScrollArea className="h-full">
                <div className="grid grid-cols-1 gap-4 p-1 sm:grid-cols-2">
                  {proceduralTypes.map((type) => (
                    <button
                      className="cursor-pointer rounded-lg border p-4 transition-all hover:border-primary hover:shadow-md focus:outline-none focus:ring-2 focus:ring-primary"
                      key={type.id}
                      onClick={() => handleCreateProcedural(type.id)}
                      type="button"
                    >
                      <div className="mb-2 flex items-center gap-3">
                        <div className="flex h-12 w-12 items-center justify-center rounded border bg-muted">
                          <Plus className="h-6 w-6 text-muted-foreground" />
                        </div>
                        <div>
                          <h4 className="font-medium">{type.name}</h4>
                          <p className="text-muted-foreground text-sm">
                            {type.description}
                          </p>
                        </div>
                      </div>
                    </button>
                  ))}
                </div>
              </ScrollArea>
            </TabsContent>
          </Tabs>
        </div>
      </DialogContent>
    </Dialog>
  );
}

// Compact texture selector for use in property panels
interface CompactTextureSelectorProps {
  selectedTextureId?: string;
  onTextureSelect: (textureId: string | null) => void;
  textureType?: string;
  placeholder?: string;
  className?: string;
}

export function CompactTextureSelector({
  selectedTextureId,
  onTextureSelect,
  textureType,
  placeholder = "Select texture...",
  className,
}: CompactTextureSelectorProps) {
  const { getTextureById } = useTextureStore();
  const selectedTexture = selectedTextureId
    ? getTextureById(selectedTextureId)
    : null;

  return (
    <TextureSelector
      onTextureSelect={onTextureSelect}
      selectedTextureId={selectedTextureId}
      textureType={textureType}
    >
      <Button
        className={cn("justify-start", className)}
        size="sm"
        variant="outline"
      >
        {selectedTexture ? (
          <>
            <div className="mr-2 h-4 w-4 flex-shrink-0 overflow-hidden rounded border">
              {selectedTexture.thumbnail && (
                <img
                  alt={selectedTexture.name}
                  className="h-full w-full object-cover"
                  height={16}
                  src={selectedTexture.thumbnail}
                  width={16}
                />
              )}
            </div>
            <span className="flex-1 truncate text-left">
              {selectedTexture.name}
            </span>
          </>
        ) : (
          <>
            <FileImage className="mr-2 h-4 w-4" />
            <span className="flex-1 truncate text-left">{placeholder}</span>
          </>
        )}
      </Button>
    </TextureSelector>
  );
}
