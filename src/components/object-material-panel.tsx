import { Palette, Plus, Trash2, Unlink } from "lucide-react";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Label } from "~/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { Separator } from "~/components/ui/separator";
import { useMaterialStore } from "~/stores/material-store";
import { useSceneStore, useSelectedObject } from "~/stores/scene-store";
import { MaterialPropertyPanel } from "./material-property-panel";

export function ObjectMaterialPanel() {
  const selectedObject = useSelectedObject();
  const { updateObject } = useSceneStore();
  const { materials, createMaterial, selectMaterial, deleteMaterial } =
    useMaterialStore();

  if (!selectedObject) {
    return (
      <div className="p-4">
        <Card>
          <CardContent className="pt-6">
            <p className="text-center text-muted-foreground text-sm">
              Select an object to edit its materials
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  const assignedMaterialId = selectedObject.materialId;
  const assignedMaterial = assignedMaterialId
    ? materials.find((m) => m.id === assignedMaterialId)
    : null;

  const handleAssignMaterial = (materialId: string | null) => {
    updateObject(selectedObject.id, { materialId: materialId || undefined });
    if (materialId) {
      selectMaterial(materialId);
    }
  };

  const handleCreateNewMaterial = () => {
    const materialId = createMaterial(
      `${selectedObject.name} Material`,
      "standard"
    );
    updateObject(selectedObject.id, { materialId });
    selectMaterial(materialId);
  };

  const handleUnassignMaterial = () => {
    updateObject(selectedObject.id, { materialId: undefined });
  };

  const handleDeleteMaterial = () => {
    if (assignedMaterialId) {
      deleteMaterial(assignedMaterialId);
      updateObject(selectedObject.id, { materialId: undefined });
    }
  };

  return (
    <div className="space-y-4 p-4">
      {/* Object Material Assignment */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="flex items-center gap-2 font-medium text-sm">
            <Palette className="h-4 w-4" />
            Object Material
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Current Material Status */}
          <div className="space-y-2">
            <Label className="font-medium text-xs">Assigned Material</Label>
            {assignedMaterial ? (
              <div className="flex items-center gap-2 rounded border p-2">
                <div
                  className="h-4 w-4 rounded border"
                  style={{ backgroundColor: assignedMaterial.properties.color }}
                />
                <span className="flex-1 text-sm">
                  {assignedMaterial.properties.name}
                </span>
                <Button
                  className="h-6 w-6 p-0"
                  onClick={handleUnassignMaterial}
                  size="sm"
                  title="Unassign material"
                  variant="ghost"
                >
                  <Unlink className="h-3 w-3" />
                </Button>
              </div>
            ) : (
              <div className="rounded border border-dashed p-2 text-muted-foreground text-sm">
                No material assigned
              </div>
            )}
          </div>

          <Separator />

          {/* Material Assignment Controls */}
          <div className="space-y-3">
            <div className="space-y-2">
              <Label className="font-medium text-xs">
                Assign Existing Material
              </Label>
              <Select
                onValueChange={(value) =>
                  handleAssignMaterial(value === "none" ? null : value)
                }
                value={assignedMaterialId || "none"}
              >
                <SelectTrigger className="h-8">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="none">None</SelectItem>
                  {materials.map((material) => (
                    <SelectItem key={material.id} value={material.id}>
                      <div className="flex items-center gap-2">
                        <div
                          className="h-3 w-3 rounded border"
                          style={{ backgroundColor: material.properties.color }}
                        />
                        {material.properties.name}
                      </div>
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="flex gap-2">
              <Button
                className="flex-1"
                onClick={handleCreateNewMaterial}
                size="sm"
              >
                <Plus className="mr-2 h-4 w-4" />
                Create New Material
              </Button>
              {assignedMaterial && (
                <Button
                  onClick={handleDeleteMaterial}
                  size="sm"
                  title="Delete this material"
                  variant="destructive"
                >
                  <Trash2 className="h-4 w-4" />
                </Button>
              )}
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Material Properties (only show if material is assigned) */}
      {assignedMaterial && (
        <>
          <Separator />
          <div className="mb-2 font-medium text-muted-foreground text-sm">
            Material Properties
          </div>
          <MaterialPropertyPanel />
        </>
      )}
    </div>
  );
}
