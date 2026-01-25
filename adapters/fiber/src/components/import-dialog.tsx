import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import { Box, FileCode, MapIcon, Upload, FileJson } from "lucide-react";
import { useState } from "react";
import * as THREE from "three";
import { Button } from "~/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "~/components/ui/dialog";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { useToast } from "~/hooks/use-toast";
import { useSceneStore } from "~/stores/scene-store";
import { type ImportProgress, modelImporter } from "~/utils/model-import";
import { Progress } from "~/components/ui/progress";
import { deepCloneModel, loadModelsByIdsBatch } from "~/utils/model-loader";
import { loadScene, deserializeScene } from "~/utils/scene-persistence";

// ============================================================================
// Types
// ============================================================================

interface ImportDialogProps {
  isOpen: boolean;
  onOpenChange: (isOpen: boolean) => void;
}

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

interface IPLInstance {
  id: number;
  model_name: string;
  interior: number;
  position: { x: number; y: number; z: number };
  rotation: { x: number; y: number; z: number };
  lod: number;
}

interface IPLFile {
  instances: IPLInstance[];
  culls: any[];
  zones: any[];
  picks: any[];
  path_count: number[];
}

// ============================================================================
// Helper Functions
// ============================================================================

const FILE_EXTENSION_REGEX = /\.(dff|gltf|glb|obj|fbx)$/i;

function getMimeType(fileName: string): string {
  const extension = fileName.split(".").pop()?.toLowerCase();
  switch (extension) {
    case "gltf":
    case "glb":
      return "model/gltf-binary";
    case "obj":
      return "text/plain";
    case "fbx":
    case "dff":
      return "application/octet-stream";
    default:
      return "application/octet-stream";
  }
}

async function createMeshFromDffData(
  dffResult: any,
  txdArchive: any | null = null
): Promise<THREE.Group> {
  const group = new THREE.Group();

  if (!dffResult.geometries || dffResult.geometries.length === 0) {
    return group;
  }

  // Create a map of texture names to Three.js textures from TXD
  const textureMap = new Map<string, THREE.Texture>();
  if (txdArchive?.textures) {
    for (const txdTexture of txdArchive.textures) {
      // For now, we'll create placeholder textures
      // TODO: Decode actual pixel data from TXD format
      // The Rust parser doesn't decode pixel data yet, so we create a colored texture
      const canvas = document.createElement("canvas");
      canvas.width = txdTexture.width || 64;
      canvas.height = txdTexture.height || 64;
      const ctx = canvas.getContext("2d");
      if (ctx) {
        // Create a placeholder colored texture (will be replaced with actual data later)
        ctx.fillStyle = "#888888"; // Grey placeholder
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = "#ffffff";
        ctx.font = "12px Arial";
        ctx.textAlign = "center";
        ctx.fillText(
          txdTexture.name.substring(0, 10),
          canvas.width / 2,
          canvas.height / 2
        );
      }
      const texture = new THREE.CanvasTexture(canvas);
      texture.flipY = false;
      texture.needsUpdate = true;
      textureMap.set(txdTexture.name.toLowerCase(), texture);
    }
  }

  for (
    let geomIndex = 0;
    geomIndex < dffResult.geometries.length;
    geomIndex++
  ) {
    const geometry = dffResult.geometries[geomIndex];
    try {
      const threeGeometry = new THREE.BufferGeometry();

      if (geometry.vertices && geometry.vertices.length > 0) {
        const positions = geometry.vertices.flatMap((v: any) => [
          v.x,
          v.y,
          v.z,
        ]);
        threeGeometry.setAttribute(
          "position",
          new THREE.Float32BufferAttribute(positions, 3)
        );
      }

      if (geometry.normals && geometry.normals.length > 0) {
        const normals = geometry.normals.flatMap((v: any) => [v.x, v.y, v.z]);
        threeGeometry.setAttribute(
          "normal",
          new THREE.Float32BufferAttribute(normals, 3)
        );
      }

      if (
        geometry.uv_layers &&
        geometry.uv_layers.length > 0 &&
        geometry.uv_layers[0].length > 0
      ) {
        const uvs = geometry.uv_layers[0].flatMap((uv: any) => [
          uv.u,
          1.0 - uv.v,
        ]);
        threeGeometry.setAttribute(
          "uv",
          new THREE.Float32BufferAttribute(uvs, 2)
        );
      }

      if (geometry.triangles && geometry.triangles.length > 0) {
        const indices = geometry.triangles.flatMap((t: any) => [t.a, t.b, t.c]);
        threeGeometry.setIndex(indices);
      }

      if (!geometry.normals || geometry.normals.length === 0) {
        threeGeometry.computeVertexNormals();
      }

      threeGeometry.computeBoundingSphere();
      threeGeometry.computeBoundingBox();

      let material: THREE.Material;
      if (geometry.materials && geometry.materials.length > 0) {
        const dffMaterial = geometry.materials[0];
        const color = new THREE.Color(
          dffMaterial.color.r / 255,
          dffMaterial.color.g / 255,
          dffMaterial.color.b / 255
        );

        // Try to find and apply texture from TXD
        let texture: THREE.Texture | null = null;
        if (dffMaterial.textures && dffMaterial.textures.length > 0) {
          const textureName = dffMaterial.textures[0].name.toLowerCase();
          texture = textureMap.get(textureName) || null;
        }

        const materialProps: any = {
          color,
          transparent: dffMaterial.color.a < 255,
          opacity: dffMaterial.color.a / 255,
          roughness: 0.7,
          metalness: 0.1,
          side: THREE.DoubleSide,
        };
        if (texture) {
          materialProps.map = texture;
        }
        material = new THREE.MeshStandardMaterial(materialProps);
      } else {
        material = new THREE.MeshStandardMaterial({
          color: 0xcc_cc_cc,
          roughness: 0.7,
          metalness: 0.1,
          side: THREE.DoubleSide,
        });
      }

      const mesh = new THREE.Mesh(threeGeometry, material);
      mesh.castShadow = true;
      mesh.receiveShadow = true;
      mesh.name = `geometry_${geomIndex}`;

      group.add(mesh);
    } catch (err) {
      console.warn(`Failed to create geometry ${geomIndex}:`, err);
    }
  }

  const box = new THREE.Box3().setFromObject(group);
  const center = box.getCenter(new THREE.Vector3());
  group.position.sub(center);

  return group;
}

// ============================================================================
// Import Dialog Component
// ============================================================================

export function ImportDialog({ isOpen, onOpenChange }: ImportDialogProps) {
  const [activeTab, setActiveTab] = useState("model");
  const [isImporting, setIsImporting] = useState(false);
  const [importProgress, setImportProgress] = useState<ImportProgress | null>(
    null
  );
  const { toast } = useToast();
  const addObject = useSceneStore((state) => state.addObject);

  // ========================================================================
  // Model Import Handler
  // ========================================================================
  const handleModelImport = async () => {
    if (isImporting) return;

    try {
      setIsImporting(true);

      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "3D Models",
            extensions: ["gltf", "glb", "obj", "fbx", "dff"],
          },
        ],
      });

      if (selected && typeof selected === "string") {
        const fileName = selected.split("/").pop() || "model";
        const extension = fileName.split(".").pop()?.toLowerCase();

        if (extension === "dff") {
          setImportProgress({
            loaded: 0,
            total: 100,
            stage: "Importing DFF file...",
          });

          const result = (await invoke("import_dff_file", {
            filePath: selected,
          })) as any;

          // Look up TXD file from CSV database
          setImportProgress({
            loaded: 50,
            total: 100,
            stage: "Looking up TXD file...",
          });

          let txdArchive: any | null = null;
          const baseName = fileName.replace(FILE_EXTENSION_REGEX, "");
          try {
            const sampModel = (await invoke("get_samp_model_by_name", {
              name: baseName,
            })) as {
              id: number;
              name: string;
              dff: string;
              txd: string;
            } | null;

            if (sampModel?.txd) {
              // Try to load TXD file from same directory as DFF
              const dffDir = selected.substring(0, selected.lastIndexOf("/"));
              const txdPath = `${dffDir}/${sampModel.txd}`;

              setImportProgress({
                loaded: 60,
                total: 100,
                stage: "Loading TXD file...",
              });

              try {
                txdArchive = (await invoke("import_txd_file", {
                  filePath: txdPath,
                })) as any;
              } catch (txdError) {
                console.warn(`Failed to load TXD file ${txdPath}:`, txdError);
              }
            }
          } catch (lookupError) {
            console.warn("Failed to lookup model in CSV:", lookupError);
          }

          setImportProgress({
            loaded: 80,
            total: 100,
            stage: "Creating scene object...",
          });

          const importedModel = await createMeshFromDffData(result, txdArchive);
          const box = new THREE.Box3().setFromObject(importedModel);
          const size = box.getSize(new THREE.Vector3());
          const maxDimension = Math.max(size.x, size.y, size.z);
          const initialScale =
            maxDimension > 0 ? Math.min(10 / maxDimension, 1) : 1;
          importedModel.scale.setScalar(initialScale);

          const sceneObject = {
            id: `dff_import_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
            name: fileName.replace(FILE_EXTENSION_REGEX, ""),
            type: "imported" as const,
            position: [0, 0, 0] as [number, number, number],
            rotation: [0, 0, 0] as [number, number, number],
            scale: [1, 1, 1] as [number, number, number],
            color: "#ffffff",
            visible: true,
            importedModel,
            initialScale,
            dffData: {
              rw_version: result.rw_version,
              frame_count: result.frames?.length || 0,
              geometry_count: result.geometries?.length || 0,
              atomic_count: result.atomics?.length || 0,
              material_count:
                result.geometries?.reduce(
                  (sum: number, g: any) => sum + (g.materials?.length || 0),
                  0
                ) || 0,
            },
          };

          addObject(sceneObject);
          setImportProgress(null);

          toast({
            title: "DFF model imported successfully",
            description: `${sceneObject.name} has been added to the scene.`,
            duration: 3000,
          });

          onOpenChange(false);
        } else {
          const fileData = await readFile(selected);
          const mimeType = getMimeType(fileName);
          const blob = new Blob([fileData], { type: mimeType });
          const file = Object.assign(blob, {
            name: fileName,
            lastModified: Date.now(),
          }) as File;

          setImportProgress({
            loaded: 0,
            total: 100,
            stage: "Starting import...",
          });

          const result = await modelImporter.importFromFile(file);

          setImportProgress(null);

          if (result.success && result.object) {
            addObject(result.object);

            toast({
              title: "Model imported successfully",
              description: `${result.object?.name ?? "Model"} has been added to the scene.`,
              duration: 3000,
            });

            onOpenChange(false);
          } else {
            toast({
              title: "Import failed",
              description: result.error || "Unknown error occurred",
              variant: "destructive",
              duration: 5000,
            });
          }
        }
      }
    } catch (error) {
      setImportProgress(null);
      console.error("Model import error:", error);
      toast({
        title: "Import failed",
        description:
          error instanceof Error ? error.message : "Unknown error occurred",
        variant: "destructive",
        duration: 5000,
      });
    } finally {
      setIsImporting(false);
      setImportProgress(null);
    }
  };

  // ========================================================================
  // PWN Import Handler
  // ========================================================================
  const handlePwnImport = async () => {
    if (isImporting) return;

    try {
      setIsImporting(true);

      const filePath = await open({
        filters: [{ name: "Pawn Script", extensions: ["pwn"] }],
      } as any);

      if (!filePath || typeof filePath !== "string") {
        return;
      }

      setImportProgress({
        loaded: 0,
        total: 100,
        stage: "Parsing PWN file...",
      });

      const result: PwnImportResult = await invoke("parse_pwn_file", {
        filePath,
      });

      if (result.errors.length > 0) {
        toast({
          title: "Import warnings",
          description: `Parsed ${result.parsed_count}/${result.line_count} lines. Some errors occurred.`,
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

      let importedCount = 0;
      let placeholderCount = 0;

      // Extract unique model IDs for batch loading
      const uniqueModelIds = Array.from(
        new Set(result.objects.map((obj) => obj.modelid))
      );

      setImportProgress({
        loaded: 0,
        total: result.objects.length,
        stage: `Loading ${uniqueModelIds.length} unique models...`,
      });

      // Batch load all models in parallel
      const loadedModels = await loadModelsByIdsBatch(uniqueModelIds, 20);

      setImportProgress({
        loaded: 0,
        total: result.objects.length,
        stage: "Creating scene objects...",
      });

      // Create scene objects (fast, no I/O)
      const sceneObjects: any[] = [];
      for (let i = 0; i < result.objects.length; i++) {
        const pwnObj = result.objects[i];

        // Update progress every 10 items to avoid excessive re-renders
        if (i % 10 === 0 || i === result.objects.length - 1) {
          setImportProgress({
            loaded: i + 1,
            total: result.objects.length,
            stage: `Creating object ${i + 1}/${result.objects.length}...`,
          });
        }

        try {
          const modelResult = loadedModels.get(pwnObj.modelid);

          let sceneObject: any;

          if (modelResult) {
            // Clone the model for each instance (each needs its own THREE.Group)
            const clonedModel = deepCloneModel(modelResult.importedModel);

            sceneObject = {
              id: `pwn_import_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
              name: modelResult.modelData.name,
              type: "imported" as const,
              modelid: pwnObj.modelid,
              position: [pwnObj.x, pwnObj.y, pwnObj.z] as [
                number,
                number,
                number,
              ],
              rotation: [pwnObj.rx, pwnObj.ry, pwnObj.rz] as [
                number,
                number,
                number,
              ],
              scale: [1, 1, 1] as [number, number, number],
              color: "#ffffff",
              visible: true,
              importedModel: clonedModel,
              initialScale: modelResult.initialScale,
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
            importedCount++;
          } else {
            placeholderCount++;
            sceneObject = {
              id: `pwn_import_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
              name: `Object_${pwnObj.modelid}`,
              type: "cube" as const,
              modelid: pwnObj.modelid,
              position: [pwnObj.x, pwnObj.y, pwnObj.z] as [
                number,
                number,
                number,
              ],
              rotation: [pwnObj.rx, pwnObj.ry, pwnObj.rz] as [
                number,
                number,
                number,
              ],
              scale: [1, 1, 1] as [number, number, number],
              color: "#ff6b6b",
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
          }

          sceneObjects.push(sceneObject);
        } catch (error) {
          console.error(
            `Failed to process PWN object with modelid ${pwnObj.modelid}:`,
            error
          );
          placeholderCount++;
        }
      }

      // Batch add all objects at once (single state update)
      useSceneStore.getState().addObjects(sceneObjects);

      setImportProgress(null);

      let description = `Imported ${importedCount} object${importedCount !== 1 ? "s" : ""} from PWN file`;
      if (placeholderCount > 0) {
        description += `. ${placeholderCount} object${placeholderCount !== 1 ? "s" : ""} shown as placeholder${placeholderCount !== 1 ? "s" : ""}.`;
      }

      toast({
        title: "Import successful",
        description,
        duration: placeholderCount > 0 ? 5000 : 3000,
      });

      onOpenChange(false);
    } catch (error) {
      console.error("PWN import error:", error);
      toast({
        title: "Import failed",
        description:
          error instanceof Error ? error.message : "Unknown error occurred",
        variant: "destructive",
        duration: 5000,
      });
    } finally {
      setIsImporting(false);
      setImportProgress(null);
    }
  };

  // ========================================================================
  // JSON Import Handler
  // ========================================================================
  const handleJsonImport = async () => {
    if (isImporting) return;

    try {
      setIsImporting(true);

      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "JSON Scene",
            extensions: ["json"],
          },
        ],
      });

      if (selected && typeof selected === "string") {
        setImportProgress({
          loaded: 0,
          total: 100,
          stage: "Loading JSON scene file...",
        });

        const result = await loadScene(selected);

        if (result.success && result.data) {
          setImportProgress({
            loaded: 50,
            total: 100,
            stage: "Deserializing scene data...",
          });

          const sceneState = useSceneStore.getState();
          const deserialized = deserializeScene(result.data);

          // For import, we add objects to existing scene rather than replacing it
          const objectsToAdd =
            deserialized?.objects?.map((obj) => ({
              ...obj,
              id: `json_import_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
              name: `${obj.name} (imported)`,
            })) ?? [];

          sceneState.addObjects(objectsToAdd);

          setImportProgress({
            loaded: 100,
            total: 100,
            stage: "Import complete",
          });

          toast({
            title: "JSON scene imported successfully",
            description: `Imported ${objectsToAdd.length} objects from JSON file.`,
            duration: 3000,
          });

          onOpenChange(false);
        } else {
          toast({
            title: "Import failed",
            description: result.error || "Failed to load JSON scene file",
            variant: "destructive",
            duration: 5000,
          });
        }
      }
    } catch (error) {
      console.error("JSON import error:", error);
      toast({
        title: "Import failed",
        description:
          error instanceof Error ? error.message : "Unknown error occurred",
        variant: "destructive",
        duration: 5000,
      });
    } finally {
      setIsImporting(false);
      setImportProgress(null);
    }
  };

  // ========================================================================
  // IPL Import Handler
  // ========================================================================
  const handleIplImport = async () => {
    if (isImporting) return;

    try {
      setIsImporting(true);

      const filePath = await open({
        filters: [{ name: "Item Placement", extensions: ["ipl"] }],
      } as any);

      if (!filePath || typeof filePath !== "string") {
        return;
      }

      setImportProgress({
        loaded: 0,
        total: 100,
        stage: "Parsing IPL file...",
      });

      const result: IPLFile = await invoke("import_ipl_file", { filePath });

      if (result.instances.length === 0) {
        toast({
          title: "Import info",
          description:
            "No object instances (INST section) found in the IPL file",
          variant: "default",
          duration: 4000,
        });
        return;
      }

      let importedCount = 0;
      let placeholderCount = 0;

      // Extract unique model IDs for batch loading
      const uniqueModelIds = Array.from(
        new Set(result.instances.map((inst) => inst.id))
      );

      setImportProgress({
        loaded: 0,
        total: result.instances.length,
        stage: `Loading ${uniqueModelIds.length} unique models...`,
      });

      // Batch load all models in parallel
      const loadedModels = await loadModelsByIdsBatch(uniqueModelIds, 20);

      setImportProgress({
        loaded: 0,
        total: result.instances.length,
        stage: "Creating scene objects...",
      });

      // Create scene objects (fast, no I/O)
      const sceneObjects: any[] = [];
      for (let i = 0; i < result.instances.length; i++) {
        const inst = result.instances[i];

        // Update progress every 10 items to avoid excessive re-renders
        if (i % 10 === 0 || i === result.instances.length - 1) {
          setImportProgress({
            loaded: i + 1,
            total: result.instances.length,
            stage: `Creating object ${i + 1}/${result.instances.length}...`,
          });
        }

        try {
          const modelResult = loadedModels.get(inst.id);

          let sceneObject: any;

          if (modelResult) {
            // Clone the model for each instance (each needs its own THREE.Group)
            const clonedModel = deepCloneModel(modelResult.importedModel);

            sceneObject = {
              id: `ipl_import_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
              name: modelResult.modelData.name || inst.model_name,
              type: "imported" as const,
              modelid: inst.id,
              position: [inst.position.x, inst.position.y, inst.position.z] as [
                number,
                number,
                number,
              ],
              rotation: [inst.rotation.x, inst.rotation.y, inst.rotation.z] as [
                number,
                number,
                number,
              ],
              scale: [1, 1, 1] as [number, number, number],
              color: "#ffffff",
              visible: true,
              importedModel: clonedModel,
              initialScale: modelResult.initialScale,
              iplData: {
                interior: inst.interior,
                lod: inst.lod,
                model_name: inst.model_name,
              },
            };
            importedCount++;
          } else {
            placeholderCount++;
            sceneObject = {
              id: `ipl_import_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
              name: inst.model_name || `Object_${inst.id}`,
              type: "cube" as const,
              modelid: inst.id,
              position: [inst.position.x, inst.position.y, inst.position.z] as [
                number,
                number,
                number,
              ],
              rotation: [inst.rotation.x, inst.rotation.y, inst.rotation.z] as [
                number,
                number,
                number,
              ],
              scale: [1, 1, 1] as [number, number, number],
              color: "#6b9fff", // Blueish color for IPL placeholders
              visible: true,
              iplData: {
                interior: inst.interior,
                lod: inst.lod,
                model_name: inst.model_name,
              },
            };
          }

          sceneObjects.push(sceneObject);
        } catch (error) {
          console.error(`Failed to process IPL instance ${inst.id}:`, error);
          placeholderCount++;
        }
      }

      // Batch add all objects at once (single state update)
      useSceneStore.getState().addObjects(sceneObjects);

      setImportProgress(null);

      let description = `Imported ${importedCount} instance${importedCount !== 1 ? "s" : ""} from IPL file`;
      if (placeholderCount > 0) {
        description += `. ${placeholderCount} shown as placeholder${placeholderCount !== 1 ? "s" : ""}.`;
      }

      // Add info about other sections if present
      const extraInfo: string[] = [];
      if (result.culls.length > 0)
        extraInfo.push(`${result.culls.length} cull zones`);
      if (result.zones.length > 0)
        extraInfo.push(`${result.zones.length} zones`);
      if (result.picks.length > 0)
        extraInfo.push(`${result.picks.length} pickups`);

      if (extraInfo.length > 0) {
        description += ` Also found: ${extraInfo.join(", ")}.`;
      }

      toast({
        title: "IPL Import successful",
        description,
        duration: 5000,
      });

      onOpenChange(false);
    } catch (error) {
      console.error("IPL import error:", error);
      toast({
        title: "Import failed",
        description:
          error instanceof Error ? error.message : "Unknown error occurred",
        variant: "destructive",
        duration: 5000,
      });
    } finally {
      setIsImporting(false);
      setImportProgress(null);
    }
  };

  // ========================================================================
  // Render
  // ========================================================================
  return (
    <Dialog onOpenChange={onOpenChange} open={isOpen}>
      <DialogContent className="sm:max-w-[550px]">
        <DialogHeader>
          <DialogTitle>Import</DialogTitle>
          <DialogDescription>
            Import 3D models, SA-MP scripts, or GTA map placement files into
            your scene.
          </DialogDescription>
        </DialogHeader>

        <Tabs
          defaultValue="model"
          onValueChange={setActiveTab}
          value={activeTab}
        >
          <TabsList className="grid w-full grid-cols-4">
            <TabsTrigger className="flex items-center gap-2" value="model">
              <Box className="h-4 w-4" />
              Model
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-2" value="pwn">
              <FileCode className="h-4 w-4" />
              PWN
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-2" value="ipl">
              <MapIcon className="h-4 w-4" />
              IPL
            </TabsTrigger>
            <TabsTrigger className="flex items-center gap-2" value="json">
              <FileJson className="h-4 w-4" />
              JSON
            </TabsTrigger>
          </TabsList>

          {/* Model Tab */}
          <TabsContent className="space-y-4" value="model">
            <div className="text-muted-foreground text-sm">
              <p className="mb-2">
                Import 3D model files directly into the scene.
              </p>
              <p className="mb-2 font-medium">Supported formats:</p>
              <ul className="ml-4 list-disc space-y-1 text-xs">
                <li>
                  <strong>DFF</strong> - RenderWare model format (GTA SA)
                </li>
                <li>
                  <strong>GLTF/GLB</strong> - GL Transmission Format
                </li>
                <li>
                  <strong>OBJ</strong> - Wavefront OBJ
                </li>
                <li>
                  <strong>FBX</strong> - Autodesk FBX
                </li>
              </ul>
            </div>
          </TabsContent>

          {/* PWN Tab */}
          <TabsContent className="space-y-4" value="pwn">
            <div className="text-muted-foreground text-sm">
              <p className="mb-2">
                Import objects from SA-MP Pawn script files (.pwn). The parser
                extracts CreateDynamicObject calls and places them in the scene.
              </p>
              <p className="mb-2 font-medium">Supported format:</p>
              <code className="mb-2 block rounded bg-muted p-2 text-xs">
                CreateDynamicObject(modelid, x, y, z, rx, ry, rz, ...);
              </code>
              <p className="text-xs">
                Objects will be loaded with their actual 3D models when
                available.
              </p>
            </div>
          </TabsContent>

          {/* IPL Tab */}
          <TabsContent className="space-y-4" value="ipl">
            <div className="text-muted-foreground text-sm">
              <p className="mb-2">
                Import GTA Item Placement files (.ipl). These files define
                object instances, zones, culling, and more for GTA maps.
              </p>
              <p className="mb-2 font-medium">Supported IPL sections:</p>
              <ul className="ml-4 list-disc space-y-1 text-xs">
                <li>
                  <strong>INST</strong> - Object instances (fully supported)
                </li>
                <li>
                  <strong>CULL</strong> - Culling zones (parsed, not visualized)
                </li>
                <li>
                  <strong>ZONE</strong> - Map zones (parsed, not visualized)
                </li>
                <li>
                  <strong>PICK</strong> - Pickups (parsed, not visualized)
                </li>
              </ul>
              <p className="mt-2 text-xs">
                Both text-based and binary IPL formats are supported.
              </p>
            </div>
          </TabsContent>

          {/* JSON Tab */}
          <TabsContent className="space-y-4" value="json">
            <div className="text-muted-foreground text-sm">
              <p className="mb-2">
                Import Rengine scene files (.json). These files contain complete
                scene data including objects, lights, and metadata.
              </p>
              <p className="mb-2 font-medium">Import behavior:</p>
              <ul className="ml-4 list-disc space-y-1 text-xs">
                <li>
                  <strong>Import</strong> - Adds objects to your current scene
                </li>
                <li>
                  <strong>Open</strong> - Replaces your current scene completely
                </li>
              </ul>
              <p className="mt-2 text-xs">
                Use the Import button to add objects from a JSON file to your
                existing scene, or use the Open button in the toolbar to replace
                the entire scene.
              </p>
            </div>
          </TabsContent>
        </Tabs>

        {importProgress && (
          <div className="flex flex-col gap-2 py-2">
            <Progress
              className="h-2"
              value={(importProgress.loaded / importProgress.total) * 100}
            />
            <span className="text-center text-muted-foreground text-xs">
              {importProgress.stage}
            </span>
          </div>
        )}

        <DialogFooter>
          <Button onClick={() => onOpenChange(false)} variant="outline">
            Cancel
          </Button>
          <Button
            disabled={isImporting}
            onClick={() => {
              if (activeTab === "model") handleModelImport();
              else if (activeTab === "pwn") handlePwnImport();
              else if (activeTab === "ipl") handleIplImport();
              else if (activeTab === "json") handleJsonImport();
            }}
          >
            <Upload className="mr-2 h-4 w-4" />
            {isImporting ? "Importing..." : `Import ${activeTab.toUpperCase()}`}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
