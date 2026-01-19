import { memo } from "react";
import { Box } from "lucide-react";
import { Card } from "~/components/ui/card";
import { cn } from "~/lib/utils";
import { convertFileSrc } from "@tauri-apps/api/core";
import { Preloaded3DPreview } from "./preloaded-3d-preview";

interface PreloadedModelCardProps {
  model: {
    model: {
      id: number;
      name: string;
      modelFile: string; // Rust serializes with camelCase via serde rename
    };
    model_file_path: string;
    preview_path?: string | null;
  };
  isSelected: boolean;
  onClick: () => void;
  show3dPreview?: boolean;
  onMouseEnter?: () => void;
  onMouseLeave?: () => void;
}

export const PreloadedModelCard = memo(function PreloadedModelCard({
  model,
  isSelected,
  onClick,
  show3dPreview = false,
  onMouseEnter,
  onMouseLeave,
}: PreloadedModelCardProps) {
  const previewUrl = model.preview_path
    ? convertFileSrc(model.preview_path)
    : "/not-found.png";

  return (
    <Card
      className={cn(
        "cursor-pointer transition-all hover:shadow-md",
        isSelected && "ring-2 ring-primary"
      )}
      onClick={onClick}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
    >
      <div className="flex flex-col">
        <div className="relative aspect-square w-full overflow-hidden rounded-t-lg bg-muted">
          {model.preview_path ? (
            <img
              alt={model.model.name}
              className="h-full w-full object-cover"
              height={128}
              src={previewUrl}
              width={128}
            />
          ) : show3dPreview ? (
            <div className="h-full w-full">
              <Preloaded3DPreview modelPath={model.model_file_path} />
            </div>
          ) : (
            <div className="flex h-full w-full items-center justify-center">
              <Box className="h-12 w-12 text-muted-foreground" />
            </div>
          )}
        </div>
        <div className="p-2">
          <div className="truncate font-medium text-sm">{model.model.name}</div>
          <div className="text-muted-foreground text-xs">
            ID: {model.model.id}
          </div>
        </div>
      </div>
    </Card>
  );
});
