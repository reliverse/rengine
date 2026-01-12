import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
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
import { useToast } from "~/hooks/use-toast";
import { useSceneStore } from "~/stores/scene-store";

interface PwnObjectData {
  modelid: number;
  x: number;
  y: number;
  z: number;
  rx: number;
  ry: number;
  rz: number;
  worldid?: number;
  interiorid?: number;
  playerid?: number;
  streamdistance?: number;
  drawdistance?: number;
  areaid?: number;
  priority?: number;
}

interface PwnImportResult {
  objects: PwnObjectData[];
  line_count: number;
  parsed_count: number;
  errors: string[];
}

interface PwnImportDialogProps {
  isOpen: boolean;
  onOpenChange: (isOpen: boolean) => void;
}

export function PwnImportDialog({
  isOpen,
  onOpenChange,
}: PwnImportDialogProps) {
  const [isImporting, setIsImporting] = useState(false);
  const { toast } = useToast();
  const addObject = useSceneStore((state) => state.addObject);

  const handleImport = async () => {
    setIsImporting(true);
    try {
      // Show open dialog for PWN files
      const filePath = await open({
        filters: [
          {
            name: "Pawn Script",
            extensions: ["pwn"],
          },
        ],
      } as any);

      if (!filePath || typeof filePath !== "string") {
        return; // User cancelled
      }

      // Parse the PWN file using the Tauri backend
      const result: PwnImportResult = await invoke("parse_pwn_file", {
        filePath,
      });

      if (result.errors.length > 0) {
        // Show warnings for parsing errors
        toast({
          title: "Import warnings",
          description: `Parsed ${result.parsed_count}/${result.line_count} lines. Some errors occurred: ${result.errors.slice(0, 3).join(", ")}${result.errors.length > 3 ? "..." : ""}`,
          variant: "default",
          duration: 5000,
        });
      }

      if (result.objects.length === 0) {
        toast({
          title: "Import failed",
          description: "No valid CreateDynamicObject calls found in the file",
          variant: "destructive",
          duration: 4000,
        });
        return;
      }

      // Convert PWN objects to scene objects and add them
      let importedCount = 0;
      for (const pwnObj of result.objects) {
        const sceneObject = {
          id: `pwn_import_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
          name: `Object_${pwnObj.modelid}`,
          type: "imported" as const,
          modelid: pwnObj.modelid,
          position: [pwnObj.x, pwnObj.y, pwnObj.z] as [number, number, number],
          rotation: [pwnObj.rx, pwnObj.ry, pwnObj.rz] as [
            number,
            number,
            number,
          ],
          scale: [1, 1, 1] as [number, number, number],
          color: "#ffffff", // Default white color for imported objects
          visible: true,
          // Store additional PWN data in a custom property
          pwnData: {
            worldid: pwnObj.worldid,
            interiorid: pwnObj.interiorid,
            playerid: pwnObj.playerid,
            streamdistance: pwnObj.streamdistance,
            drawdistance: pwnObj.drawdistance,
            areaid: pwnObj.areaid,
            priority: pwnObj.priority,
          },
        };

        addObject(sceneObject);
        importedCount++;
      }

      // Show success message
      toast({
        title: "Import successful",
        description: `Imported ${importedCount} object${importedCount !== 1 ? "s" : ""} from PWN file`,
        duration: 3000,
      });

      // Close dialog
      setTimeout(() => {
        onOpenChange(false);
      }, 100);
    } catch (error) {
      console.error("Import error:", error);
      toast({
        title: "Import failed",
        description:
          error instanceof Error ? error.message : "Unknown error occurred",
        variant: "destructive",
        duration: 5000,
      });
    } finally {
      setIsImporting(false);
    }
  };

  return (
    <Dialog onOpenChange={onOpenChange} open={isOpen}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Import PWN File</DialogTitle>
          <DialogDescription>
            Import scene objects from a SA-MP Pawn script file (.pwn). The file
            should contain CreateDynamicObject function calls.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          <div className="text-muted-foreground text-sm">
            <p className="mb-2">Expected format:</p>
            <code className="block rounded bg-muted p-2 text-xs">
              CreateDynamicObject(modelid, Float:x, Float:y, Float:z, Float:rx,
              Float:ry, Float:rz, worldid = value, interiorid = value, ...);
            </code>
          </div>

          <div className="text-muted-foreground text-sm">
            <p>Objects will be imported with their model IDs and positions.</p>
            <p>
              Additional PWN parameters (worldid, interiorid, etc.) will be
              preserved in the object data.
            </p>
          </div>
        </div>

        <DialogFooter>
          <Button onClick={() => onOpenChange(false)} variant="outline">
            Cancel
          </Button>
          <Button disabled={isImporting} onClick={handleImport}>
            {isImporting ? "Importing..." : "Import PWN File"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
