import { memo } from "react";
import { Upload } from "lucide-react";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";
import { ASSET_TYPES } from "../constants";
import type { AssetType } from "../types";

// Placeholder components for asset types not yet implemented
export const PlaceholderAssetTab = memo(function PlaceholderAssetTab({
  assetType,
}: {
  assetType: AssetType;
}) {
  const config = ASSET_TYPES[assetType];
  const Icon = config.icon;

  return (
    <div className="flex h-full flex-col items-center justify-center p-8 text-center">
      <div className={cn("mb-4 rounded-full bg-muted p-4", config.color)}>
        <Icon className="h-8 w-8" />
      </div>
      <h3 className="mb-2 font-medium text-lg">{config.label}</h3>
      <p className="mb-4 text-muted-foreground">{config.description}</p>
      {config.accept && (
        <div className="relative">
          <input
            accept={config.accept}
            className="absolute inset-0 h-full w-full cursor-pointer opacity-0"
            multiple
            type="file"
          />
          <Button>
            <Upload className="mr-2 h-4 w-4" />
            Import {config.label}
          </Button>
        </div>
      )}
      {config.accept && (
        <p className="mt-2 text-muted-foreground text-xs">
          Supported formats: {config.accept}
        </p>
      )}
    </div>
  );
});
