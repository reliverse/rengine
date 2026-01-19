import { RotateCcw, ZoomIn, ZoomOut } from "lucide-react";
import type React from "react";
import { useEffect, useRef, useState } from "react";
import type * as THREE from "three";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import { Card, CardContent } from "~/components/ui/card";
import { cn } from "~/lib/utils";

interface TexturePreviewProps {
  texture: THREE.Texture | null;
  textureName?: string;
  className?: string;
  showControls?: boolean;
  maxSize?: number;
}

export function TexturePreview({
  texture,
  textureName,
  className,
  showControls = true,
  maxSize = 256,
}: TexturePreviewProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [zoom, setZoom] = useState(1);
  const [offset, setOffset] = useState({ x: 0, y: 0 });
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const [_imageData, setImageData] = useState<ImageData | null>(null);

  useEffect(() => {
    if (!(texture && canvasRef.current)) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Get texture image
    const image = texture.image as HTMLImageElement | HTMLCanvasElement;
    if (!image) return;

    // Set canvas size
    const aspectRatio = image.width / image.height;
    let width = maxSize;
    let height = maxSize;

    if (aspectRatio > 1) {
      height = maxSize / aspectRatio;
    } else {
      width = maxSize * aspectRatio;
    }

    canvas.width = width;
    canvas.height = height;

    // Clear canvas
    ctx.clearRect(0, 0, width, height);

    // Draw texture
    ctx.save();
    ctx.scale(zoom, zoom);
    ctx.translate(offset.x, offset.y);
    ctx.drawImage(image, 0, 0, width / zoom, height / zoom);
    ctx.restore();

    // Store image data for zoom operations
    const imgData = ctx.getImageData(0, 0, width, height);
    setImageData(imgData);
  }, [texture, zoom, offset, maxSize]);

  const handleZoomIn = () => {
    setZoom((prev) => Math.min(prev * 1.2, 10));
  };

  const handleZoomOut = () => {
    setZoom((prev) => Math.max(prev / 1.2, 0.1));
  };

  const handleReset = () => {
    setZoom(1);
    setOffset({ x: 0, y: 0 });
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    if (zoom > 1) {
      setIsDragging(true);
      setDragStart({ x: e.clientX - offset.x, y: e.clientY - offset.y });
    }
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (isDragging) {
      setOffset({
        x: e.clientX - dragStart.x,
        y: e.clientY - dragStart.y,
      });
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  const handleWheel = (e: React.WheelEvent) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    setZoom((prev) => Math.max(0.1, Math.min(prev * delta, 10)));
  };

  if (!texture) {
    return (
      <Card
        className={cn(
          "flex aspect-square items-center justify-center",
          className
        )}
      >
        <CardContent className="text-center text-muted-foreground">
          <div className="text-sm">No texture</div>
          {textureName && (
            <div className="mt-1 text-xs opacity-70">{textureName}</div>
          )}
        </CardContent>
      </Card>
    );
  }

  const image = texture.image as HTMLImageElement | HTMLCanvasElement;

  return (
    <Card className={cn("overflow-hidden", className)}>
      <CardContent className="p-0">
        <div className="relative">
          {/* Texture canvas */}
          <canvas
            className={cn(
              "block cursor-move",
              zoom > 1 ? "cursor-grab" : "",
              isDragging ? "cursor-grabbing" : ""
            )}
            onMouseDown={handleMouseDown}
            onMouseLeave={handleMouseUp}
            onMouseMove={handleMouseMove}
            onMouseUp={handleMouseUp}
            onWheel={handleWheel}
            ref={canvasRef}
            style={{
              maxWidth: "100%",
              maxHeight: "100%",
              imageRendering: zoom <= 1 ? "auto" : "pixelated",
            }}
          />

          {/* Controls overlay */}
          {showControls && (
            <div className="absolute top-2 right-2 flex gap-1">
              <Button
                className="h-6 w-6 p-0"
                disabled={zoom >= 10}
                onClick={handleZoomIn}
                size="sm"
                variant="secondary"
              >
                <ZoomIn className="h-3 w-3" />
              </Button>
              <Button
                className="h-6 w-6 p-0"
                disabled={zoom <= 0.1}
                onClick={handleZoomOut}
                size="sm"
                variant="secondary"
              >
                <ZoomOut className="h-3 w-3" />
              </Button>
              <Button
                className="h-6 w-6 p-0"
                disabled={zoom === 1 && offset.x === 0 && offset.y === 0}
                onClick={handleReset}
                size="sm"
                variant="secondary"
              >
                <RotateCcw className="h-3 w-3" />
              </Button>
            </div>
          )}

          {/* Info overlay */}
          <div className="absolute bottom-2 left-2 flex gap-2">
            {textureName && (
              <Badge className="text-xs" variant="secondary">
                {textureName}
              </Badge>
            )}
            {image && (
              <Badge className="text-xs" variant="outline">
                {image.width}×{image.height}
              </Badge>
            )}
            <Badge className="text-xs" variant="outline">
              {zoom.toFixed(1)}x
            </Badge>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}

// Texture thumbnail component for use in lists/grids
interface TextureThumbnailProps {
  texture: THREE.Texture | null;
  textureName?: string;
  size?: number;
  className?: string;
  onClick?: () => void;
}

export function TextureThumbnail({
  texture,
  textureName,
  size = 64,
  className,
  onClick,
}: TextureThumbnailProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!(texture && canvasRef.current)) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const image = texture.image as HTMLImageElement | HTMLCanvasElement;
    if (!image) return;

    canvas.width = size;
    canvas.height = size;

    // Calculate aspect ratio and fit image
    const aspectRatio = image.width / image.height;
    let drawWidth = size;
    let drawHeight = size;
    let offsetX = 0;
    let offsetY = 0;

    if (aspectRatio > 1) {
      // Image is wider than tall
      drawHeight = size / aspectRatio;
      offsetY = (size - drawHeight) / 2;
    } else {
      // Image is taller than wide
      drawWidth = size * aspectRatio;
      offsetX = (size - drawWidth) / 2;
    }

    ctx.drawImage(image, offsetX, offsetY, drawWidth, drawHeight);
  }, [texture, size]);

  return (
    <button
      className={cn(
        "relative cursor-pointer overflow-hidden rounded border bg-muted transition-shadow hover:shadow-md focus:outline-none focus:ring-2 focus:ring-primary",
        className
      )}
      onClick={onClick}
      style={{ width: size, height: size }}
      type="button"
    >
      <canvas
        className="block h-full w-full"
        ref={canvasRef}
        style={{ imageRendering: "pixelated" }}
      />

      {!texture && (
        <div className="absolute inset-0 flex items-center justify-center text-muted-foreground">
          <div className="text-center text-xs">No texture</div>
        </div>
      )}

      {textureName && (
        <div className="absolute right-0 bottom-0 left-0 truncate bg-black/75 p-1 text-white text-xs">
          {textureName}
        </div>
      )}
    </button>
  );
}

// Texture comparison component for before/after views
interface TextureComparisonProps {
  beforeTexture: THREE.Texture | null;
  afterTexture: THREE.Texture | null;
  beforeLabel?: string;
  afterLabel?: string;
  className?: string;
}

export function TextureComparison({
  beforeTexture,
  afterTexture,
  beforeLabel = "Before",
  afterLabel = "After",
  className,
}: TextureComparisonProps) {
  return (
    <div className={cn("grid grid-cols-2 gap-4", className)}>
      <div>
        <div className="mb-2 font-medium text-sm">{beforeLabel}</div>
        <TexturePreview
          maxSize={200}
          showControls={false}
          texture={beforeTexture}
        />
      </div>
      <div>
        <div className="mb-2 font-medium text-sm">{afterLabel}</div>
        <TexturePreview
          maxSize={200}
          showControls={false}
          texture={afterTexture}
        />
      </div>
    </div>
  );
}
