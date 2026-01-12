import { save } from "@tauri-apps/plugin-dialog";
import { writeFile, writeTextFile } from "@tauri-apps/plugin-fs";
import { useState } from "react";
import { Button } from "~/components/ui/button";
import { Checkbox } from "~/components/ui/checkbox";
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
import { exportSceneToGLTF } from "~/utils/gltf-export";

type ExportFormat = "samp" | "gltf" | "glb";

interface ExportDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function ExportDialog({ open, onOpenChange }: ExportDialogProps) {
  const [exportFormat, setExportFormat] = useState<ExportFormat>("glb");
  const [isExporting, setIsExporting] = useState(false);
  const [includeLights, setIncludeLights] = useState(true);
  const [embedTextures, setEmbedTextures] = useState(true);
  const { toast } = useToast();
  const objects = useSceneStore((state) => state.objects);
  const lights = useSceneStore((state) => state.lights);
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
      let fileName = "";
      let filePath: string | null = null;

      switch (exportFormat) {
        case "samp": {
          const content = generateSampCode();
          if (!content) {
            toast({
              title: "Export failed",
              description: "No content to export",
              variant: "destructive",
              duration: 4000,
            });
            return;
          }
          fileName = "scene_objects.pwn";
          break;
        }

        case "gltf":
        case "glb":
          fileName = `scene.${exportFormat}`;
          break;

        default:
          throw new Error(`Unsupported export format: ${exportFormat}`);
      }

      // Show save dialog
      const filters =
        exportFormat === "samp"
          ? [{ name: "Pawn Script", extensions: ["pwn"] }]
          : exportFormat === "gltf"
            ? [{ name: "GLTF", extensions: ["gltf"] }]
            : [{ name: "GLB", extensions: ["glb"] }];

      filePath = await save({
        filters,
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

      if (exportFormat === "samp") {
        const content = generateSampCode();
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
      } else {
        // GLTF/GLB export
        const result = await exportSceneToGLTF(objects, lights, {
          binary: exportFormat === "glb",
          includeLights,
          includeCameras: false, // TODO: Implement camera export
          embedTextures,
          truncateDrawRange: true,
        });

        if (!result.success) {
          throw new Error(result.error || "Export failed");
        }

        if (!result.data) {
          throw new Error("No export data generated");
        }

        try {
          // Add small delay before file write to ensure dialog is fully closed
          await new Promise((resolve) => setTimeout(resolve, 50));

          if (exportFormat === "glb" && result.data instanceof ArrayBuffer) {
            await writeFile(filePath, new Uint8Array(result.data));
          } else if (
            exportFormat === "gltf" &&
            typeof result.data === "string"
          ) {
            await writeTextFile(filePath, result.data);
          } else {
            throw new Error("Invalid export data format");
          }

          console.log("GLTF/GLB export completed successfully");
        } catch (writeError) {
          console.error("File write error:", writeError);
          throw new Error(
            `Failed to write file: ${writeError instanceof Error ? writeError.message : "Unknown error"}`
          );
        }

        // Show warnings if any
        if (result.warnings && result.warnings.length > 0) {
          setTimeout(() => {
            toast({
              title: "Export warnings",
              description: result.warnings?.join("\n"),
              variant: "default",
              duration: 5000,
            });
          }, 1000);
        }
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
                <SelectItem value="glb">GLB (Binary)</SelectItem>
                <SelectItem value="gltf">GLTF (JSON)</SelectItem>
                <SelectItem value="samp">SAMP (Pawn)</SelectItem>
              </SelectContent>
            </Select>
          </div>

          {exportFormat === "glb" || exportFormat === "gltf" ? (
            <div className="space-y-3">
              <div className="flex items-center space-x-2">
                <Checkbox
                  checked={includeLights}
                  id="includeLights"
                  onCheckedChange={(checked) =>
                    setIncludeLights(checked as boolean)
                  }
                />
                <Label className="text-sm" htmlFor="includeLights">
                  Include lights ({lights.length} lights)
                </Label>
              </div>

              <div className="flex items-center space-x-2">
                <Checkbox
                  checked={embedTextures}
                  id="embedTextures"
                  onCheckedChange={(checked) =>
                    setEmbedTextures(checked as boolean)
                  }
                />
                <Label className="text-sm" htmlFor="embedTextures">
                  Embed textures in file
                </Label>
              </div>

              <div className="text-muted-foreground text-sm">
                Ready to export {objects.length} object
                {objects.length !== 1 ? "s" : ""}
                {includeLights
                  ? ` and ${lights.length} light${lights.length !== 1 ? "s" : ""}`
                  : ""}
                .
              </div>
            </div>
          ) : (
            <div className="text-muted-foreground text-sm">
              {objectsWithModelIdCount > 0 ? (
                <span>
                  Ready to export {objectsWithModelIdCount} object
                  {objectsWithModelIdCount !== 1 ? "s" : ""} with model IDs.
                </span>
              ) : (
                <span>
                  No objects with model IDs found. Add model IDs to objects
                  before exporting.
                </span>
              )}
            </div>
          )}
        </div>

        <DialogFooter>
          <Button onClick={() => onOpenChange(false)} variant="outline">
            Cancel
          </Button>
          <Button
            disabled={
              isExporting ||
              (exportFormat === "samp" && objectsWithModelIdCount === 0) ||
              ((exportFormat === "gltf" || exportFormat === "glb") &&
                objects.length === 0)
            }
            onClick={handleExport}
          >
            {isExporting ? "Exporting..." : "Export"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
