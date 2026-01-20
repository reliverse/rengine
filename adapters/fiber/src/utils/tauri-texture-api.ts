import { invoke } from "@tauri-apps/api/core";

/**
 * Tauri texture API for native file operations
 */

export interface ImageMetadata {
  width: number;
  height: number;
  format: string;
  sizeBytes: number;
}

export interface TextureOperation {
  id: string;
  operationType: string; // "copy", "delete", "rename"
  sourcePath?: string;
  destinationPath?: string;
}

export interface TextureOperationResult {
  operationId: string;
  success: boolean;
  error?: string;
}

/**
 * Read an image file as base64 data URL
 */
export async function readImageAsBase64(filePath: string): Promise<string> {
  try {
    return await invoke<string>("read_image_as_base64", { filePath });
  } catch (error) {
    console.error("Failed to read image as base64:", error);
    throw error;
  }
}

/**
 * Get basic metadata for an image file
 */
export async function getImageMetadata(
  filePath: string
): Promise<ImageMetadata> {
  try {
    return await invoke<ImageMetadata>("get_image_metadata", { filePath });
  } catch (error) {
    console.error("Failed to get image metadata:", error);
    throw error;
  }
}

/**
 * Save texture data to file
 */
export async function saveTextureToFile(
  filePath: string,
  base64Data: string,
  format = "png"
): Promise<void> {
  try {
    await invoke("save_texture_to_file", {
      filePath,
      base64Data,
      format,
    });
  } catch (error) {
    console.error("Failed to save texture to file:", error);
    throw error;
  }
}

/**
 * Batch process multiple texture operations
 */
export async function batchProcessTextures(
  operations: TextureOperation[]
): Promise<TextureOperationResult[]> {
  try {
    return await invoke<TextureOperationResult[]>("batch_process_textures", {
      operations: operations.map((op) => ({
        id: op.id,
        operation_type: op.operationType,
        source_path: op.sourcePath,
        destination_path: op.destinationPath,
      })),
    });
  } catch (error) {
    console.error("Failed to batch process textures:", error);
    throw error;
  }
}

/**
 * Check if running in Tauri environment
 */
export function isTauri(): boolean {
  return typeof window !== "undefined" && window.__TAURI__ !== undefined;
}

/**
 * Load texture from file path using Tauri (for desktop builds)
 */
export async function loadTextureFromPath(
  filePath: string
): Promise<HTMLImageElement> {
  if (!isTauri()) {
    throw new Error(
      "Texture loading from path is only available in Tauri builds"
    );
  }

  try {
    const dataUrl = await readImageAsBase64(filePath);
    return new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve(img);
      img.onerror = reject;
      img.src = dataUrl;
    });
  } catch (error) {
    console.error("Failed to load texture from path:", error);
    throw error;
  }
}

/**
 * Show native file dialog for texture selection
 */
export async function showTextureFileDialog(options?: {
  multiple?: boolean;
  directory?: boolean;
  filters?: Array<{
    name: string;
    extensions: string[];
  }>;
}): Promise<string[] | null> {
  if (!isTauri()) {
    // Fallback to web file picker
    return new Promise((resolve) => {
      const input = document.createElement("input");
      input.type = "file";
      input.multiple = options?.multiple ?? false;
      input.accept =
        options?.filters
          ?.flatMap((f) => f.extensions.map((ext) => `.${ext}`))
          .join(",") || "image/*,.tga,.dds,.hdr,.exr";

      input.onchange = (e) => {
        const files = (e.target as HTMLInputElement).files;
        if (files) {
          const paths = Array.from(files).map((f) => f.name); // Web API gives File objects, not paths
          resolve(paths);
        } else {
          resolve(null);
        }
      };

      input.click();
    });
  }

  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const result = await open({
      multiple: options?.multiple ?? false,
      directory: options?.directory ?? false,
      filters: options?.filters ?? [
        {
          name: "Images",
          extensions: [
            "png",
            "jpg",
            "jpeg",
            "webp",
            "tga",
            "dds",
            "hdr",
            "exr",
            "bmp",
            "tiff",
          ],
        },
      ],
    });

    if (Array.isArray(result)) {
      return result;
    }
    if (result) {
      return [result];
    }
    return null;
  } catch (error) {
    console.error("Failed to show file dialog:", error);
    throw error;
  }
}

/**
 * Show native save dialog for texture export
 */
export async function showTextureSaveDialog(options?: {
  defaultPath?: string;
  filters?: Array<{
    name: string;
    extensions: string[];
  }>;
}): Promise<string | null> {
  if (!isTauri()) {
    // Fallback: return default path or null
    return options?.defaultPath || null;
  }

  try {
    const { save } = await import("@tauri-apps/plugin-dialog");
    return await save({
      defaultPath: options?.defaultPath,
      filters: options?.filters ?? [
        {
          name: "PNG Image",
          extensions: ["png"],
        },
        {
          name: "JPEG Image",
          extensions: ["jpg", "jpeg"],
        },
        {
          name: "WebP Image",
          extensions: ["webp"],
        },
      ],
    });
  } catch (error) {
    console.error("Failed to show save dialog:", error);
    throw error;
  }
}
