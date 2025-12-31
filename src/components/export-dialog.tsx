import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";
import { useState } from "react";
import { Button } from "~/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "~/components/ui/dialog";
import { Label } from "~/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { useToast } from "~/hooks/use-toast";
import { useSceneStore } from "~/stores/scene-store";
import { usePrecision } from "~/stores/settings-store";

type ExportFormat = "samp";

interface ExportDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function ExportDialog({ open, onOpenChange }: ExportDialogProps) {
  const [exportFormat, setExportFormat] = useState<ExportFormat>("samp");
  const [isExporting, setIsExporting] = useState(false);
  const { toast } = useToast();
  const objects = useSceneStore((state) => state.objects);
  const precision = usePrecision();

  const generateSampCode = () => {
    const objectsWithModelId = objects.filter(
      (obj) => obj.modelid !== undefined
    );

    if (objectsWithModelId.length === 0) {
      return "// No objects with model IDs found for export";
    }

    return objectsWithModelId
      .map((obj) => {
        const [x, y, z] = obj.position;
        const [rx, ry, rz] = obj.rotation; // Rotation is already in degrees

        return `CreateDynamicObject(${obj.modelid}, Float:${x.toFixed(precision)}, Float:${y.toFixed(precision)}, Float:${z.toFixed(precision)}, Float:${rx.toFixed(precision)}, Float:${ry.toFixed(precision)}, Float:${rz.toFixed(precision)}, worldid = -1, interiorid = -1, playerid = -1, Float:streamdistance = STREAMER_OBJECT_SD, Float:drawdistance = STREAMER_OBJECT_DD, areaid = -1, priority = 0);`;
      })
      .join("\n");
  };

  const handleExport = async () => {
    setIsExporting(true);
    try {
      let content = "";
      let fileName = "";

      switch (exportFormat) {
        case "samp":
          content = generateSampCode();
          fileName = "scene_objects.pwn";
          break;
        default:
          throw new Error(`Unsupported export format: ${exportFormat}`);
      }

      if (!content) {
        toast({
          title: "Export failed",
          description: "No content to export",
          variant: "destructive",
          duration: 4000,
        });
        return;
      }

      // Show save dialog
      const filePath = await save({
        filters: [
          {
            name: "Pawn Script",
            extensions: ["pwn"],
          },
        ],
        defaultPath: fileName,
      });

      if (!filePath) {
        return; // User cancelled
      }

      // Validate filePath is a string
      if (typeof filePath !== "string" || filePath.trim() === "") {
        toast({
          title: "Export failed",
          description: "Invalid file path selected",
          variant: "destructive",
          duration: 4000,
        });
        return;
      }

      // Validate content is not empty
      if (!content || content.trim() === "") {
        toast({
          title: "Export failed",
          description: "No content to export",
          variant: "destructive",
          duration: 4000,
        });
        return;
      }

      try {
        // Add small delay before file write to ensure dialog is fully closed
        await new Promise((resolve) => setTimeout(resolve, 50));
        await writeTextFile(filePath, content);
        console.log("File write completed successfully");
      } catch (writeError) {
        console.error("File write error:", writeError);
        throw new Error(
          `Failed to write file: ${writeError instanceof Error ? writeError.message : "Unknown error"}`
        );
      }

      // Show success message
      setTimeout(() => {
        toast({
          title: "Export successful",
          description: "Scene exported successfully",
          duration: 3000,
        });
      }, 200);

      // Close dialog after a short delay to prevent state conflicts
      setTimeout(() => {
        onOpenChange(false);
      }, 100);
    } catch (error) {
      console.error("Export error:", error);
      toast({
        title: "Export failed",
        description:
          error instanceof Error ? error.message : "Unknown error occurred",
        variant: "destructive",
        duration: 5000,
      });
    } finally {
      setIsExporting(false);
    }
  };

  const objectsWithModelIdCount = objects.filter(
    (obj) => obj.modelid !== undefined
  ).length;

  return (
    <Dialog onOpenChange={onOpenChange} open={open}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Export Scene</DialogTitle>
          <DialogDescription>
            Export scene objects in the selected format. Only objects with model
            IDs will be exported.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            <Label className="text-right" htmlFor="format">
              Format
            </Label>
            <Select
              onValueChange={(value) =>
                value && setExportFormat(value as ExportFormat)
              }
              value={exportFormat}
            >
              <SelectTrigger className="col-span-3">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="samp">SAMP (Pawn)</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="text-muted-foreground text-sm">
            {objectsWithModelIdCount > 0 ? (
              <span>
                Ready to export {objectsWithModelIdCount} object
                {objectsWithModelIdCount !== 1 ? "s" : ""} with model IDs.
              </span>
            ) : (
              <span>
                No objects with model IDs found. Add model IDs to objects before
                exporting.
              </span>
            )}
          </div>
        </div>

        <DialogFooter>
          <Button onClick={() => onOpenChange(false)} variant="outline">
            Cancel
          </Button>
          <Button
            disabled={isExporting || objectsWithModelIdCount === 0}
            onClick={handleExport}
          >
            {isExporting ? "Exporting..." : "Export"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
