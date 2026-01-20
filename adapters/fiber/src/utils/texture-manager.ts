import * as THREE from "three";
import type { TextureChannel } from "~/types/materials";

// Texture cache to avoid loading the same texture multiple times
const textureCache = new Map<string, THREE.Texture>();

// Texture loader instance
const textureLoader = new THREE.TextureLoader();

// Loading promises cache to prevent duplicate loads
const loadingPromises = new Map<string, Promise<THREE.Texture>>();

/**
 * Texture loading options
 */
export interface TextureLoadOptions {
  /** Generate mipmaps */
  generateMipmaps?: boolean;
  /** Texture wrapping mode */
  wrapS?: THREE.Wrapping;
  /** Texture wrapping mode */
  wrapT?: THREE.Wrapping;
  /** Texture filtering mode */
  magFilter?: THREE.MagnificationTextureFilter;
  /** Texture filtering mode */
  minFilter?: THREE.MinificationTextureFilter;
  /** Texture format */
  format?: THREE.PixelFormat;
  /** Texture encoding */
  encoding?: THREE.ColorSpace;
  /** Anisotropy level */
  anisotropy?: number;
  /** Flip Y coordinate */
  flipY?: boolean;
}

/**
 * Load a texture from URL with caching
 */
export function loadTexture(
  url: string,
  options: TextureLoadOptions = {}
): Promise<THREE.Texture> {
  // Check cache first
  if (textureCache.has(url)) {
    return Promise.resolve(textureCache.get(url) as THREE.Texture);
  }

  // Check if already loading
  if (loadingPromises.has(url)) {
    return loadingPromises.get(url) as Promise<THREE.Texture>;
  }

  // Create loading promise
  const loadPromise = new Promise<THREE.Texture>((resolve, reject) => {
    textureLoader.load(
      url,
      (texture) => {
        // Apply options
        applyTextureOptions(texture, options);

        // Cache the texture
        textureCache.set(url, texture);

        // Remove from loading promises
        loadingPromises.delete(url);

        resolve(texture);
      },
      (progress) => {
        // Could emit progress events here
        console.log(
          `Loading texture ${url}: ${(progress.loaded / progress.total) * 100}%`
        );
      },
      (error) => {
        // Remove from loading promises on error
        loadingPromises.delete(url);
        reject(error);
      }
    );
  });

  // Cache the loading promise
  loadingPromises.set(url, loadPromise);

  return loadPromise;
}

/**
 * Load texture from file (for Tauri/desktop environment)
 */
export function loadTextureFromFile(
  file: File,
  options: TextureLoadOptions = {}
): Promise<THREE.Texture> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();

    reader.onload = async (event) => {
      try {
        const url = event.target?.result as string;

        // Check cache using file name and size as key
        const cacheKey = `${file.name}_${file.size}`;
        if (textureCache.has(cacheKey)) {
          resolve(textureCache.get(cacheKey) as THREE.Texture);
          return;
        }

        // Load texture from data URL
        const texture = await new Promise<THREE.Texture>(
          (resolveTexture, rejectTexture) => {
            textureLoader.load(
              url,
              (texture) => {
                applyTextureOptions(texture, options);
                resolveTexture(texture);
              },
              undefined,
              rejectTexture
            );
          }
        );

        // Cache with file-based key
        textureCache.set(cacheKey, texture);
        resolve(texture);
      } catch (error) {
        reject(error);
      }
    };

    reader.onerror = () => reject(new Error("Failed to read file"));
    reader.readAsDataURL(file);
  });
}

/**
 * Create texture from canvas/image data
 */
export function createTextureFromCanvas(
  canvas: HTMLCanvasElement,
  options: TextureLoadOptions = {}
): THREE.Texture {
  const texture = new THREE.CanvasTexture(canvas);
  applyTextureOptions(texture, options);
  return texture;
}

/**
 * Create texture from image element
 */
export function createTextureFromImage(
  image: HTMLImageElement,
  options: TextureLoadOptions = {}
): THREE.Texture {
  const texture = new THREE.Texture(image);
  applyTextureOptions(texture, options);
  texture.needsUpdate = true;
  return texture;
}

/**
 * Apply texture options to a Three.js texture
 */
export function applyTextureOptions(
  texture: THREE.Texture,
  options: TextureLoadOptions
): void {
  if (options.wrapS !== undefined) texture.wrapS = options.wrapS;
  if (options.wrapT !== undefined) texture.wrapT = options.wrapT;
  if (options.magFilter !== undefined) texture.magFilter = options.magFilter;
  if (options.minFilter !== undefined) texture.minFilter = options.minFilter;
  if (options.format !== undefined) texture.format = options.format;
  if (options.encoding !== undefined)
    (texture as THREE.Texture & { encoding: THREE.ColorSpace }).encoding =
      options.encoding;
  if (options.anisotropy !== undefined) texture.anisotropy = options.anisotropy;
  if (options.flipY !== undefined) texture.flipY = options.flipY;

  // Generate mipmaps by default unless explicitly disabled
  if (options.generateMipmaps !== false) {
    texture.generateMipmaps = true;
  } else {
    texture.generateMipmaps = false;
  }

  texture.needsUpdate = true;
}

/**
 * Apply texture channel properties to a texture
 */
export function applyTextureChannelProperties(
  texture: THREE.Texture,
  channel: TextureChannel
): void {
  texture.offset.set(channel.offset[0], channel.offset[1]);
  texture.repeat.set(channel.repeat[0], channel.repeat[1]);
  texture.rotation = channel.rotation;
  texture.wrapS = channel.wrapS;
  texture.wrapT = channel.wrapT;
  texture.magFilter = channel.magFilter as THREE.MagnificationTextureFilter;
  texture.minFilter = channel.minFilter as THREE.MinificationTextureFilter;
  texture.anisotropy = channel.anisotropy;
  (texture as THREE.Texture & { encoding: THREE.ColorSpace }).encoding =
    channel.encoding;
  texture.flipY = channel.flipY;
  texture.needsUpdate = true;
}

/**
 * Create a default texture with specified color
 */
export function createColorTexture(
  color: string | number = 0xff_ff_ff,
  size = 64,
  options: TextureLoadOptions = {}
): THREE.Texture {
  const canvas = document.createElement("canvas");
  canvas.width = size;
  canvas.height = size;
  const ctx = canvas.getContext("2d");
  if (!ctx) throw new Error("Failed to get 2D context");

  // Fill with color
  ctx.fillStyle =
    typeof color === "string"
      ? color
      : `#${color.toString(16).padStart(6, "0")}`;
  ctx.fillRect(0, 0, size, size);

  return createTextureFromCanvas(canvas, options);
}

/**
 * Create a checkerboard texture for debugging
 */
export function createCheckerboardTexture(
  color1 = "#ffffff",
  color2 = "#cccccc",
  size = 64,
  options: TextureLoadOptions = {}
): THREE.Texture {
  const canvas = document.createElement("canvas");
  canvas.width = size;
  canvas.height = size;
  const ctx = canvas.getContext("2d");
  if (!ctx) throw new Error("Failed to get 2D context");

  const squareSize = size / 8;

  for (let x = 0; x < 8; x++) {
    for (let y = 0; y < 8; y++) {
      ctx.fillStyle = (x + y) % 2 === 0 ? color1 : color2;
      ctx.fillRect(x * squareSize, y * squareSize, squareSize, squareSize);
    }
  }

  return createTextureFromCanvas(canvas, options);
}

/**
 * Create a normal map texture (pointing upwards)
 */
export function createNormalTexture(
  size = 64,
  options: TextureLoadOptions = {}
): THREE.Texture {
  const canvas = document.createElement("canvas");
  canvas.width = size;
  canvas.height = size;
  const ctx = canvas.getContext("2d");
  if (!ctx) throw new Error("Failed to get 2D context");
  const imageData = ctx.createImageData(size, size);

  // Fill with normal pointing upwards (0.5, 0.5, 1.0) in RGB
  for (let i = 0; i < imageData.data.length; i += 4) {
    imageData.data[i] = 128; // R: 0.5 * 255
    imageData.data[i + 1] = 128; // G: 0.5 * 255
    imageData.data[i + 2] = 255; // B: 1.0 * 255
    imageData.data[i + 3] = 255; // A: 1.0
  }

  ctx.putImageData(imageData, 0, 0);
  return createTextureFromCanvas(canvas, {
    ...options,
    encoding: THREE.LinearSRGBColorSpace,
  });
}

/**
 * Create a roughness map texture (constant value)
 */
export function createRoughnessTexture(
  roughness = 0.5,
  size = 64,
  options: TextureLoadOptions = {}
): THREE.Texture {
  const canvas = document.createElement("canvas");
  canvas.width = size;
  canvas.height = size;
  const ctx = canvas.getContext("2d");
  if (!ctx) throw new Error("Failed to get 2D context");
  const imageData = ctx.createImageData(size, size);
  const value = Math.round(roughness * 255);

  // Fill with constant roughness value
  for (let i = 0; i < imageData.data.length; i += 4) {
    imageData.data[i] = value; // R
    imageData.data[i + 1] = value; // G
    imageData.data[i + 2] = value; // B
    imageData.data[i + 3] = 255; // A
  }

  ctx.putImageData(imageData, 0, 0);
  return createTextureFromCanvas(canvas, {
    ...options,
    encoding: THREE.LinearSRGBColorSpace,
  });
}

/**
 * Create a metalness map texture (constant value)
 */
export function createMetalnessTexture(
  metalness = 0.0,
  size = 64,
  options: TextureLoadOptions = {}
): THREE.Texture {
  const canvas = document.createElement("canvas");
  canvas.width = size;
  canvas.height = size;
  const ctx = canvas.getContext("2d");
  if (!ctx) throw new Error("Failed to get 2D context");
  const imageData = ctx.createImageData(size, size);
  const value = Math.round(metalness * 255);

  // Fill with constant metalness value
  for (let i = 0; i < imageData.data.length; i += 4) {
    imageData.data[i] = value; // R
    imageData.data[i + 1] = value; // G
    imageData.data[i + 2] = value; // B
    imageData.data[i + 3] = 255; // A
  }

  ctx.putImageData(imageData, 0, 0);
  return createTextureFromCanvas(canvas, {
    ...options,
    encoding: THREE.LinearSRGBColorSpace,
  });
}

/**
 * Get texture from cache by URL, returning null if not found or if url is null
 */
export function getCachedTexture(url: string | null): THREE.Texture | null {
  if (!url) return null;
  return textureCache.get(url) || null;
}

/**
 * Clear texture cache and dispose of textures
 */
export function clearTextureCache(): void {
  for (const texture of textureCache.values()) {
    texture.dispose();
  }
  textureCache.clear();
  loadingPromises.clear();
}

/**
 * Get texture cache statistics
 */
export function getTextureCacheStats(): {
  count: number;
  loadingCount: number;
  memoryUsage: number;
} {
  return {
    count: textureCache.size,
    loadingCount: loadingPromises.size,
    memoryUsage: 0, // Would need to calculate actual memory usage based on texture sizes
  };
}

/**
 * Preload commonly used textures
 */
export async function preloadCommonTextures(): Promise<void> {
  const commonTextures: string[] = [
    // Could preload default normal, roughness, metalness textures here
  ];

  await Promise.all(commonTextures.map((url) => loadTexture(url)));
}

/**
 * Create texture preview/thumbnail
 */
export function createTexturePreview(
  texture: THREE.Texture,
  size = 64
): Promise<string> {
  return new Promise((resolve) => {
    const canvas = document.createElement("canvas");
    canvas.width = size;
    canvas.height = size;
    const ctx = canvas.getContext("2d");
    if (!ctx) {
      resolve(""); // Return empty string if context unavailable
      return;
    }

    // If texture has an image, draw it
    if (texture.image) {
      ctx.drawImage(texture.image as CanvasImageSource, 0, 0, size, size);
      resolve(canvas.toDataURL());
    } else {
      // Fallback: create a placeholder
      ctx.fillStyle = "#cccccc";
      ctx.fillRect(0, 0, size, size);
      ctx.fillStyle = "#000000";
      ctx.font = "12px Arial";
      ctx.textAlign = "center";
      ctx.fillText("No Preview", size / 2, size / 2);
      resolve(canvas.toDataURL());
    }
  });
}

/**
 * Validate texture format and properties
 */
export function validateTexture(texture: THREE.Texture): {
  isValid: boolean;
  errors: string[];
  warnings: string[];
} {
  const errors: string[] = [];
  const warnings: string[] = [];

  // Check if texture has image data
  if (!texture.image) {
    errors.push("Texture has no image data");
  }

  // Check power of two dimensions for mipmaps
  if (texture.generateMipmaps) {
    const image = texture.image as { width?: number; height?: number };
    const { width, height } = image;
    if (width && height) {
      // biome-ignore lint/suspicious/noBitwiseOperators: Bitwise AND is appropriate for power-of-two check
      const isPowerOfTwo = (n: number) => n > 0 && (n & (n - 1)) === 0;
      if (!(isPowerOfTwo(width) && isPowerOfTwo(height))) {
        warnings.push(
          "Texture dimensions are not power of two, mipmaps may not work correctly"
        );
      }
    }
  }

  // Check encoding for different texture types
  if (
    (texture as THREE.Texture & { encoding: THREE.ColorSpace }).encoding ===
      THREE.SRGBColorSpace &&
    texture.format === THREE.RGBAFormat
  ) {
    warnings.push("sRGB encoding with alpha channel may cause issues");
  }

  return {
    isValid: errors.length === 0,
    errors,
    warnings,
  };
}
