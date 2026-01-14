import { cn } from "~/lib/utils";
import { useAssetImport } from "../hooks/use-asset-import";

interface RenderWareDropZoneProps {
  dragOver: boolean;
  setDragOver: (dragOver: boolean) => void;
  isImporting: boolean;
  config: {
    label: string;
    icon: React.ComponentType<any>;
    accept: string;
    color: string;
  };
}

export function RenderWareDropZone({
  dragOver,
  setDragOver,
  isImporting,
  config,
}: RenderWareDropZoneProps) {
  const { handleFileDrop, handleImportFiles } = useAssetImport();

  const Icon = config.icon;

  const onFileDrop = (e: React.DragEvent) => {
    handleFileDrop(e, handleImportFiles);
    setDragOver(false);
  };

  return (
    <div className="flex-1">
      <div
        className={cn(
          "flex h-full flex-col items-center justify-center rounded-lg border-2 border-dashed p-8 transition-colors",
          dragOver ? "border-primary bg-primary/5" : "border-muted",
          isImporting && "pointer-events-none opacity-50"
        )}
        onDragLeave={() => setDragOver(false)}
        onDragOver={(e) => {
          e.preventDefault();
          setDragOver(true);
        }}
        onDrop={onFileDrop}
      >
        <div className={cn("mb-4 rounded-full bg-muted p-4", config.color)}>
          <Icon className="h-8 w-8" />
        </div>
        <h3 className="mb-2 font-medium text-lg">
          {isImporting ? "Importing..." : `Import ${config.label} Assets`}
        </h3>
        <p className="mb-4 text-center text-muted-foreground">
          Drag and drop {config.label} files here, or use the buttons above
        </p>

        <p className="text-center text-muted-foreground text-xs">
          Supported formats: {config.accept}
        </p>

        {/* PWN specific info */}
        <div className="mt-4 rounded bg-muted p-3 text-left text-xs">
          <p className="mb-2 font-medium">PWN Script Support:</p>
          <p className="text-muted-foreground">
            Import objects from SA-MP Pawn scripts containing
            CreateDynamicObject calls.
          </p>
        </div>
      </div>
    </div>
  );
}
