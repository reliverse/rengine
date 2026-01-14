import type { ImgArchive, ImportedAsset } from "../../types";

interface RenderWareHeaderProps {
  loadedArchives: Map<string, ImgArchive>;
  importedAssets: Map<string, ImportedAsset>;
  config: {
    label: string;
    description: string;
  };
}

export function RenderWareHeader({
  loadedArchives,
  importedAssets,
  config,
}: RenderWareHeaderProps) {
  return (
    <div className="mb-4 flex items-center justify-between">
      <div>
        <h3 className="mb-1 font-medium text-lg">{config.label}</h3>
        <p className="text-muted-foreground text-sm">{config.description}</p>
      </div>
      <div className="text-right text-muted-foreground text-xs">
        <div>{loadedArchives.size} archives loaded</div>
        <div>
          {Array.from(loadedArchives.values()).reduce(
            (sum, archive) => sum + archive.total_entries,
            0
          )}{" "}
          archive entries
        </div>
        <div>{importedAssets.size} imported assets</div>
      </div>
    </div>
  );
}
