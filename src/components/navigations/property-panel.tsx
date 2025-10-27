import { Button } from "~/components/ui/button";
import { Checkbox } from "~/components/ui/checkbox";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import { Select } from "~/components/ui/select";
import { toast } from "~/lib/toast";
import { useEditorStore } from "~/store/editor-store";
import { MapObject } from "~/types/map";

export default function PropertyPanel() {
  const { mapData, selectedObjectId, updateObject, deleteObject, syncWithBackend } =
    useEditorStore();

  const selectedObject = mapData?.objects.find((obj) => obj.id === selectedObjectId);

  const handlePropertyChange = async (property: keyof MapObject, value: any) => {
    if (selectedObjectId) {
      try {
        updateObject(selectedObjectId, { [property]: value });
        await syncWithBackend();
        toast.success("Property updated successfully!");
      } catch (error) {
        console.error("Failed to update property:", error);
        toast.error("Failed to update property");
      }
    }
  };

  const handleVectorChange = async (
    vector: "position" | "rotation" | "scale",
    axis: "x" | "y" | "z",
    value: number,
  ) => {
    if (selectedObjectId && selectedObject) {
      try {
        const newVector = {
          ...selectedObject[vector],
          [axis]: value,
        };
        updateObject(selectedObjectId, { [vector]: newVector });
        await syncWithBackend();
        toast.success(`${vector} updated successfully!`);
      } catch (error) {
        console.error("Failed to update vector:", error);
        toast.error("Failed to update vector");
      }
    }
  };

  const handleDelete = async () => {
    if (selectedObjectId) {
      try {
        deleteObject(selectedObjectId);
        await syncWithBackend();
        toast.success("Object deleted successfully!");
      } catch (error) {
        console.error("Failed to delete object:", error);
        toast.error("Failed to delete object");
      }
    }
  };

  if (!selectedObject) {
    return (
      <div className="w-80 bg-gray-800 border-l border-gray-700 p-4">
        <h3 className="text-lg font-semibold text-white mb-4">Properties</h3>
        <p className="text-gray-400">No object selected</p>
      </div>
    );
  }

  return (
    <div className="w-80 bg-gray-800 border-l border-gray-700 p-4 overflow-y-auto">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-white">Properties</h3>
        <Button onClick={handleDelete} size="sm" variant="destructive">
          Delete
        </Button>
      </div>

      <div className="space-y-4">
        {/* Basic Properties */}
        <div>
          <Label className="block text-sm font-medium text-gray-300 mb-2">Name</Label>
          <Input
            onChange={(e) => handlePropertyChange("name", e.target.value)}
            type="text"
            value={selectedObject.name}
          />
        </div>

        <div>
          <Label className="block text-sm font-medium text-gray-300 mb-2">Type</Label>
          <Select
            onChange={(e) => handlePropertyChange("object_type", e.target.value)}
            value={selectedObject.object_type}
          >
            <option value="cube">Cube</option>
            <option value="sphere">Sphere</option>
            <option value="plane">Plane</option>
            <option value="custom">Custom</option>
          </Select>
        </div>

        <div>
          <Label className="block text-sm font-medium text-gray-300 mb-2">Color</Label>
          <div className="flex gap-2">
            <input
              className="w-12 h-10 bg-gray-700 border border-gray-600 rounded cursor-pointer"
              onChange={(e) => handlePropertyChange("color", e.target.value)}
              type="color"
              value={selectedObject.color}
            />
            <Input
              className="flex-1"
              onChange={(e) => handlePropertyChange("color", e.target.value)}
              type="text"
              value={selectedObject.color}
            />
          </div>
        </div>

        <div className="flex items-center gap-2">
          <Checkbox
            checked={selectedObject.visible}
            id="visible"
            onChange={(e) => handlePropertyChange("visible", e.target.checked)}
          />
          <Label className="text-sm text-gray-300" htmlFor="visible">
            Visible
          </Label>
        </div>

        {/* Position */}
        <div>
          <Label className="block text-sm font-medium text-gray-300 mb-2">Position</Label>
          <div className="grid grid-cols-3 gap-2">
            <div>
              <Label className="block text-xs text-gray-400 mb-1">X</Label>
              <Input
                className="text-sm"
                onChange={(e) =>
                  handleVectorChange("position", "x", parseFloat(e.target.value) || 0)
                }
                step="0.1"
                type="number"
                value={selectedObject.position.x}
              />
            </div>
            <div>
              <Label className="block text-xs text-gray-400 mb-1">Y</Label>
              <Input
                className="text-sm"
                onChange={(e) =>
                  handleVectorChange("position", "y", parseFloat(e.target.value) || 0)
                }
                step="0.1"
                type="number"
                value={selectedObject.position.y}
              />
            </div>
            <div>
              <Label className="block text-xs text-gray-400 mb-1">Z</Label>
              <Input
                className="text-sm"
                onChange={(e) =>
                  handleVectorChange("position", "z", parseFloat(e.target.value) || 0)
                }
                step="0.1"
                type="number"
                value={selectedObject.position.z}
              />
            </div>
          </div>
        </div>

        {/* Rotation */}
        <div>
          <Label className="block text-sm font-medium text-gray-300 mb-2">Rotation</Label>
          <div className="grid grid-cols-3 gap-2">
            <div>
              <Label className="block text-xs text-gray-400 mb-1">X</Label>
              <Input
                className="text-sm"
                onChange={(e) =>
                  handleVectorChange("rotation", "x", parseFloat(e.target.value) || 0)
                }
                step="1"
                type="number"
                value={selectedObject.rotation.x}
              />
            </div>
            <div>
              <Label className="block text-xs text-gray-400 mb-1">Y</Label>
              <Input
                className="text-sm"
                onChange={(e) =>
                  handleVectorChange("rotation", "y", parseFloat(e.target.value) || 0)
                }
                step="1"
                type="number"
                value={selectedObject.rotation.y}
              />
            </div>
            <div>
              <Label className="block text-xs text-gray-400 mb-1">Z</Label>
              <Input
                className="text-sm"
                onChange={(e) =>
                  handleVectorChange("rotation", "z", parseFloat(e.target.value) || 0)
                }
                step="1"
                type="number"
                value={selectedObject.rotation.z}
              />
            </div>
          </div>
        </div>

        {/* Scale */}
        <div>
          <Label className="block text-sm font-medium text-gray-300 mb-2">Scale</Label>
          <div className="grid grid-cols-3 gap-2">
            <div>
              <Label className="block text-xs text-gray-400 mb-1">X</Label>
              <Input
                className="text-sm"
                min="0.1"
                onChange={(e) => handleVectorChange("scale", "x", parseFloat(e.target.value) || 1)}
                step="0.1"
                type="number"
                value={selectedObject.scale.x}
              />
            </div>
            <div>
              <Label className="block text-xs text-gray-400 mb-1">Y</Label>
              <Input
                className="text-sm"
                min="0.1"
                onChange={(e) => handleVectorChange("scale", "y", parseFloat(e.target.value) || 1)}
                step="0.1"
                type="number"
                value={selectedObject.scale.y}
              />
            </div>
            <div>
              <Label className="block text-xs text-gray-400 mb-1">Z</Label>
              <Input
                className="text-sm"
                min="0.1"
                onChange={(e) => handleVectorChange("scale", "z", parseFloat(e.target.value) || 1)}
                step="0.1"
                type="number"
                value={selectedObject.scale.z}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
