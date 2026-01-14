import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useSceneStore } from "~/stores/scene-store";
import { FILE_EXTENSION_REGEX, PATH_SEPARATOR_REGEX } from "../../constants";
import type {
  ImportedAsset,
  ImportedDffAsset,
  ImportedTxdAsset,
  ImportedColAsset,
  ImportedIplAsset,
} from "../../types";

interface SampModel {
  id: number;
  name: string;
  dff: string;
  txd: string;
  radius: number;
}

export function useAssetImport() {
  const addObject = useSceneStore((state) => state.addObject);

  const getSampModelByName = async (
    baseName: string
  ): Promise<SampModel | null> => {
    try {
      const result = await invoke("get_samp_model_by_name", {
        name: baseName,
      });
      if (result) {
        return result as SampModel;
      }
    } catch (error) {
      console.log(`⚠️ SA:MP model lookup failed for "${baseName}":`, error);
    }
    return null;
  };

  const importIndividualFile = async (
    fileType: "dff" | "txd" | "col" | "ipl",
    onAssetImported: (asset: ImportedAsset) => void,
    setIsImporting: (importing: boolean) => void
  ): Promise<void> => {
    try {
      const filters = {
        dff: [{ name: "DFF Model", extensions: ["dff", "obj"] }],
        txd: [{ name: "TXD Texture Archive", extensions: ["txd"] }],
        col: [{ name: "COL Collision", extensions: ["col"] }],
        ipl: [{ name: "IPL Placement", extensions: ["ipl"] }],
      };

      const filePath = await open({
        filters: filters[fileType],
      });

      if (!filePath || typeof filePath !== "string") {
        return;
      }

      setIsImporting(true);

      const fileNameOnly = filePath.split(PATH_SEPARATOR_REGEX).pop() || "";
      const baseName = fileNameOnly.replace(FILE_EXTENSION_REGEX, "");

      const sampModel = await getSampModelByName(baseName);

      if (sampModel) {
        console.log(
          `✅ Auto-matched "${baseName}" to SA:MP model ID ${sampModel.id} (${sampModel.dff})`
        );
      } else {
        console.log(`ℹ️ No SA:MP model found for "${baseName}"`);
      }

      const command = `import_${fileType}_file`;
      const result = (await invoke(command, { filePath })) as any;

      console.log(`${fileType.toUpperCase()} file imported:`, result);

      let asset: ImportedAsset;

      if (fileType === "dff") {
        asset = {
          type: "dff",
          file_path: filePath,
          rw_version: result.rw_version,
          frame_count: result.frames?.length || 0,
          geometry_count: result.geometries?.length || 0,
          atomic_count: result.atomics?.length || 0,
          material_count:
            result.geometries?.reduce(
              (sum: number, g: any) => sum + (g.materials?.length || 0),
              0
            ) || 0,
          texture_count:
            result.geometries?.reduce(
              (sum: number, g: any) => sum + (g.textures?.length || 0),
              0
            ) || 0,
          samp_model_id: sampModel?.id || null,
          samp_model_name: sampModel?.name || null,
          loaded_at: Date.now(),
        } as ImportedDffAsset;
      } else if (fileType === "txd") {
        asset = {
          type: "txd",
          file_path: filePath,
          texture_count: result.total_textures || result.textures?.length || 0,
          textures: result.textures || [],
          renderware_version: result.renderware_version,
          samp_model_id: sampModel?.id || null,
          samp_model_name: sampModel?.name || null,
          loaded_at: Date.now(),
        } as ImportedTxdAsset;
      } else if (fileType === "col") {
        asset = {
          type: "col",
          file_path: filePath,
          version: result.version || "Unknown",
          model_count: result.models?.length || 0,
          models:
            result.models?.map((m: any) => ({
              name: m.name || "Unknown",
              face_count: m.faces?.length || 0,
              vertex_count: m.vertices?.length || 0,
            })) || [],
          samp_model_id: sampModel?.id || null,
          samp_model_name: sampModel?.name || null,
          loaded_at: Date.now(),
        } as ImportedColAsset;
      } else if (fileType === "ipl") {
        asset = {
          type: "ipl",
          file_path: filePath,
          instance_count: result.instances?.length || 0,
          zone_count: result.zones?.length || 0,
          cull_count: result.culls?.length || 0,
          pick_count: result.picks?.length || 0,
          samp_model_id: sampModel?.id || null,
          samp_model_name: sampModel?.name || null,
          loaded_at: Date.now(),
        } as ImportedIplAsset;
      } else {
        throw new Error(`Unknown asset type: ${fileType}`);
      }

      onAssetImported(asset);

      if (sampModel) {
        console.log(
          `✅ Auto-matched ${baseName} to SA:MP model ID ${sampModel.id}`
        );
      }
    } catch (error) {
      console.error(`${fileType.toUpperCase()} import error:`, error);
    } finally {
      setIsImporting(false);
    }
  };

  const handleImportFiles = (files: File[]): void => {
    for (const file of files) {
      const extension = file.name.toLowerCase().split(".").pop();
      console.log(
        `${extension?.toUpperCase()} file dropped: ${file.name} - use the dedicated import button for this file type`
      );
    }
  };

  const handlePwnImport = async (
    setIsImporting: (importing: boolean) => void
  ): Promise<void> => {
    try {
      const filePath = await open({
        filters: [
          {
            name: "Pawn Script",
            extensions: ["pwn"],
          },
        ],
      });

      if (!filePath || typeof filePath !== "string") {
        return;
      }

      setIsImporting(true);

      const result = await invoke("parse_pwn_file", {
        filePath,
      });

      const pwnResult = result as any;

      if (pwnResult.errors.length > 0) {
        console.warn(
          "PWN parsing warnings:",
          pwnResult.errors.slice(0, 3).join(", ")
        );
      }

      if (pwnResult.objects.length === 0) {
        console.warn("No valid CreateDynamicObject calls found in the file");
        return;
      }

      for (const pwnObj of pwnResult.objects) {
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
          color: "#ffffff",
          visible: true,
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
      }

      console.log(`Imported ${pwnResult.objects.length} objects from PWN file`);
    } catch (error) {
      console.error("PWN import error:", error);
    } finally {
      setIsImporting(false);
    }
  };

  const handleFileDrop = (
    e: React.DragEvent,
    handleImportFiles: (files: File[]) => void
  ): void => {
    e.preventDefault();

    const files = Array.from(e.dataTransfer.files).filter((file) => {
      const extension = file.name.toLowerCase().split(".").pop();
      return ["img", "txd", "dff", "col", "ide", "pwn"].includes(
        extension || ""
      );
    });

    if (files.length > 0) {
      handleImportFiles(files);
    }
  };

  const importViaIde = async (
    setIsImporting: (importing: boolean) => void
  ): Promise<void> => {
    try {
      setIsImporting(true);

      const idePath = await open({
        filters: [{ name: "IDE Definitions", extensions: ["ide"] }],
      });

      if (!idePath || typeof idePath !== "string") {
        setIsImporting(false);
        return;
      }

      const imgPath = await open({
        filters: [{ name: "IMG Archive", extensions: ["img"] }],
      });

      if (!imgPath || typeof imgPath !== "string") {
        setIsImporting(false);
        return;
      }

      const result = (await invoke("import_via_ide", {
        imgArchivePath: imgPath,
        ideFilePath: idePath,
        modelsDirectory: undefined,
      })) as any;

      console.log("IDE-based import completed:", result);
    } catch (error) {
      console.error("IDE-based import error:", error);
    } finally {
      setIsImporting(false);
    }
  };

  return {
    importIndividualFile,
    handleImportFiles,
    handlePwnImport,
    handleFileDrop,
    importViaIde,
  };
}
