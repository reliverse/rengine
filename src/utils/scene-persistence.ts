import { open, save } from "@tauri-apps/plugin-dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import type { SceneLight, SceneObject, SceneState } from "~/stores/scene-store";

// Top-level regex for performance
const RENGINE_EXTENSION_REGEX = /\.rengine$/;

export interface SceneFileData {
  version: string;
  metadata: {
    name: string;
    description?: string;
    createdAt: string;
    modifiedAt: string;
    rengineVersion: string;
  };
  scene: {
    objects: SceneObject[];
    lights: SceneLight[];
    cameraPosition: [number, number, number];
    cameraTarget: [number, number, number];
    gridVisible: boolean;
    snapToGrid: boolean;
    gridSize: number;
    lightsVisible: boolean;
  };
}

interface SavedProject {
  id: string;
  name: string;
  filePath: string;
  lastSaved: Date;
  objectCount: number;
  description?: string;
}

export interface SaveResult {
  success: boolean;
  error?: string;
}

export interface LoadResult {
  success: boolean;
  data?: SceneFileData;
  error?: string;
}

/**
 * Serializes the current scene state to a SceneFileData object
 */
export function serializeScene(
  sceneState: SceneState,
  metadata?: Partial<SceneFileData["metadata"]>
): SceneFileData {
  const now = new Date().toISOString();

  return {
    version: "1.0.0",
    metadata: {
      name: metadata?.name || "Untitled Scene",
      description: metadata?.description,
      createdAt: metadata?.createdAt || now,
      modifiedAt: now,
      rengineVersion: "1.0.0",
    },
    scene: {
      objects: sceneState.objects,
      lights: sceneState.lights,
      cameraPosition: sceneState.cameraPosition,
      cameraTarget: sceneState.cameraTarget,
      gridVisible: sceneState.gridVisible,
      snapToGrid: sceneState.snapToGrid,
      gridSize: sceneState.gridSize,
      lightsVisible: sceneState.lightsVisible,
    },
  };
}

/**
 * Deserializes a SceneFileData object back to scene state
 */
export function deserializeScene(fileData: SceneFileData): Partial<SceneState> {
  // Validate version compatibility
  if (!isVersionCompatible(fileData.version)) {
    throw new Error(
      `Incompatible file version: ${fileData.version}. Current version: 1.0.0`
    );
  }

  return {
    objects: fileData.scene.objects,
    lights: fileData.scene.lights || [], // Fallback for older files without lights
    cameraPosition: fileData.scene.cameraPosition,
    cameraTarget: fileData.scene.cameraTarget,
    gridVisible: fileData.scene.gridVisible,
    snapToGrid: fileData.scene.snapToGrid,
    gridSize: fileData.scene.gridSize,
    lightsVisible: fileData.scene.lightsVisible ?? true, // Default to visible
  };
}

/**
 * Checks if a file version is compatible with the current version
 */
function isVersionCompatible(fileVersion: string): boolean {
  const [major] = fileVersion.split(".").map(Number);
  const currentMajor = 1;

  // For now, only allow same major version
  return major === currentMajor;
}

/**
 * Validates scene file data structure
 */
export function validateSceneFile(data: SceneFileData) {
  try {
    if (!data || typeof data !== "object") {
      return false;
    }
    if (data.version !== "1.0.0") {
      return false;
    }
    if (!(data.metadata && data.scene)) {
      return false;
    }
    if (!Array.isArray(data.scene.objects)) {
      return false;
    }
    if (
      !Array.isArray(data.scene.cameraPosition) ||
      data.scene.cameraPosition.length !== 3
    ) {
      return false;
    }
    if (
      !Array.isArray(data.scene.cameraTarget) ||
      data.scene.cameraTarget.length !== 3
    ) {
      return false;
    }

    // Validate each object has required properties
    for (const obj of data.scene.objects) {
      if (!(obj.id && obj.name && obj.type)) {
        return false;
      }
      if (!Array.isArray(obj.position) || obj.position.length !== 3) {
        return false;
      }
      if (!Array.isArray(obj.rotation) || obj.rotation.length !== 3) {
        return false;
      }
      if (!Array.isArray(obj.scale) || obj.scale.length !== 3) {
        return false;
      }
    }

    return true;
  } catch {
    return false;
  }
}

/**
 * Generates a default filename for saving
 */
export function generateDefaultFilename(): string {
  const now = new Date();
  const timestamp = now.toISOString().slice(0, 19).replace(/:/g, "-");
  return `scene_${timestamp}.rengine`;
}

/**
 * Extracts scene name from filename
 */
export function extractSceneNameFromFilename(filename: string): string {
  return filename.replace(RENGINE_EXTENSION_REGEX, "").replace(/_/g, " ");
}

/**
 * Saves a project to the saved projects list for quick loading
 */
function saveToSavedProjects(
  filePath: string,
  sceneName: string,
  objectCount: number,
  description?: string
) {
  try {
    const saved = localStorage.getItem("rengine_saved_projects");
    let existingProjects: Array<
      Omit<SavedProject, "lastSaved"> & { lastSaved: string }
    > = [];

    if (saved) {
      existingProjects = JSON.parse(saved);
    }

    const project = {
      id: `saved_${Date.now()}`,
      name: sceneName,
      filePath,
      lastSaved: new Date().toISOString(),
      objectCount,
      description,
    };

    const updated = [
      project,
      ...existingProjects.filter((p) => p.filePath !== filePath),
    ].slice(0, 10); // Keep up to 10 saved projects

    localStorage.setItem("rengine_saved_projects", JSON.stringify(updated));
  } catch (error) {
    console.warn("Failed to save project to recent list:", error);
  }
}

/**
 * Saves the current scene to a file
 */
export async function saveScene(
  sceneState: SceneState,
  filePath?: string,
  metadata?: Partial<SceneFileData["metadata"]>
): Promise<SaveResult> {
  try {
    let savePath = filePath;

    // If no path provided, show save dialog
    if (!savePath) {
      const result = await save({
        filters: [
          {
            name: "Rengine Scene",
            extensions: ["rengine"],
          },
        ],
        defaultPath: generateDefaultFilename(),
      });

      if (!result) {
        return { success: false, error: "Save cancelled" };
      }

      savePath = result;
    }

    const sceneData = serializeScene(sceneState, metadata);
    const jsonString = JSON.stringify(sceneData, null, 2);

    // Add small delay before file write to prevent timing issues
    await new Promise((resolve) => setTimeout(resolve, 50));
    await writeTextFile(savePath, jsonString);

    // Add to saved projects list for quick loading
    saveToSavedProjects(
      savePath,
      sceneData.metadata.name,
      sceneState.objects.length,
      sceneData.metadata.description
    );

    return { success: true };
  } catch (error) {
    console.error("Save error:", error);
    return {
      success: false,
      error: error instanceof Error ? error.message : "Unknown save error",
    };
  }
}

/**
 * Loads a scene from a file
 */
export async function loadScene(filePath?: string): Promise<LoadResult> {
  try {
    let loadPath = filePath;

    // If no path provided, show open dialog
    if (!loadPath) {
      const result = await open({
        filters: [
          {
            name: "Rengine Scene",
            extensions: ["rengine"],
          },
        ],
      });

      if (!result || typeof result !== "string") {
        return { success: false, error: "Load cancelled" };
      }

      loadPath = result;
    }

    const fileContent = await readTextFile(loadPath);

    let parsedData: SceneFileData;
    try {
      parsedData = JSON.parse(fileContent);
    } catch (_parseError) {
      return {
        success: false,
        error: "Invalid JSON format in scene file",
      };
    }

    if (!validateSceneFile(parsedData)) {
      return {
        success: false,
        error: "Invalid scene file format or corrupted data",
      };
    }

    return {
      success: true,
      data: parsedData,
    };
  } catch (error) {
    console.error("Load error:", error);
    return {
      success: false,
      error: error instanceof Error ? error.message : "Unknown load error",
    };
  }
}

/**
 * Quick save functionality (saves to a temporary location or last saved path)
 */
export async function quickSave(sceneState: SceneState): Promise<SaveResult> {
  // For now, this is the same as regular save but could be optimized for quick saves
  // In the future, this could save to a temp location and defer the "real" save
  return await saveScene(sceneState);
}
