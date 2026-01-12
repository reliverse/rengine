import * as THREE from "three";
import { applyTextureOptions } from "~/utils/texture-manager";

/**
 * Texture processing and conversion utilities
 */
export interface TextureProcessingOptions {
  /** Target format */
  format?: THREE.PixelFormat;
  /** Target encoding */
  encoding?: THREE.ColorSpace;
  /** Resize dimensions */
  width?: number;
  height?: number;
  /** Quality for compression (0-1) */
  quality?: number;
  /** Flip Y coordinate */
  flipY?: boolean;
  /** Generate mipmaps */
  generateMipmaps?: boolean;
  /** Premultiply alpha */
  premultiplyAlpha?: boolean;
}

/**
 * Convert texture to different format
 */
export function convertTextureFormat(
  texture: THREE.Texture,
  targetFormat: "png" | "jpg" | "webp" | "tga" | "dds",
  options: TextureProcessingOptions = {}
): Promise<Blob> {
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");

  if (!(ctx && texture.image)) {
    throw new Error("Unable to process texture");
  }

  const image = texture.image as HTMLImageElement | HTMLCanvasElement;

  // Set canvas size
  canvas.width = options.width || image.width;
  canvas.height = options.height || image.height;

  // Apply transformations
  ctx.save();
  if (options.flipY) {
    ctx.scale(1, -1);
    ctx.translate(0, -canvas.height);
  }

  // Draw texture to canvas
  ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
  ctx.restore();

  // Convert to blob
  return new Promise((resolve, reject) => {
    canvas.toBlob(
      (blob) => {
        if (blob) {
          resolve(blob);
        } else {
          reject(new Error("Failed to convert texture"));
        }
      },
      `image/${targetFormat}`,
      options.quality || 0.9
    );
  });
}

/**
 * Resize texture to new dimensions
 */
export function resizeTexture(
  texture: THREE.Texture,
  width: number,
  height: number,
  filter: "nearest" | "linear" = "linear"
): THREE.Texture {
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");

  if (!(ctx && texture.image)) {
    throw new Error("Unable to resize texture");
  }

  const image = texture.image as HTMLImageElement | HTMLCanvasElement;

  canvas.width = width;
  canvas.height = height;

  // Set image smoothing
  ctx.imageSmoothingEnabled = filter === "linear";
  if (filter === "nearest") {
    ctx.imageSmoothingQuality = "low";
  }

  // Draw resized image
  ctx.drawImage(image, 0, 0, width, height);

  // Create new texture from canvas
  const resizedTexture = new THREE.CanvasTexture(canvas);

  // Copy original texture properties
  applyTextureOptions(resizedTexture, {
    wrapS: texture.wrapS,
    wrapT: texture.wrapT,
    magFilter: texture.magFilter,
    minFilter: texture.minFilter,
    anisotropy: texture.anisotropy,
    flipY: texture.flipY,
    generateMipmaps: texture.generateMipmaps,
  });

  return resizedTexture;
}

/**
 * Apply image processing effects
 */
export interface ImageEffect {
  type:
    | "brightness"
    | "contrast"
    | "saturation"
    | "hue"
    | "blur"
    | "sharpen"
    | "noise";
  value: number;
}

export function applyImageEffects(
  texture: THREE.Texture,
  effects: ImageEffect[]
): THREE.Texture {
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");

  if (!(ctx && texture.image)) {
    throw new Error("Unable to apply effects");
  }

  const image = texture.image as HTMLImageElement | HTMLCanvasElement;

  canvas.width = image.width;
  canvas.height = image.height;

  // Draw original image
  ctx.drawImage(image, 0, 0);

  // Get image data
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  const data = imageData.data;

  // Apply effects
  for (const effect of effects) {
    switch (effect.type) {
      case "brightness":
        applyBrightness(data, effect.value);
        break;
      case "contrast":
        applyContrast(data, effect.value);
        break;
      case "saturation":
        applySaturation(data, effect.value);
        break;
      case "hue":
        applyHue(data, effect.value);
        break;
      case "blur":
        applyBlur(ctx, canvas, effect.value);
        break;
      case "sharpen":
        applySharpen(ctx, canvas, effect.value);
        break;
      case "noise":
        applyNoise(data, effect.value);
        break;
    }
  }

  // Put processed data back
  ctx.putImageData(imageData, 0, 0);

  // Create new texture
  const processedTexture = new THREE.CanvasTexture(canvas);
  applyTextureOptions(processedTexture, {
    wrapS: texture.wrapS,
    wrapT: texture.wrapT,
    magFilter: texture.magFilter,
    minFilter: texture.minFilter,
    anisotropy: texture.anisotropy,
    flipY: texture.flipY,
    generateMipmaps: texture.generateMipmaps,
  });

  return processedTexture;
}

// Effect implementation functions
function applyBrightness(data: Uint8ClampedArray, value: number): void {
  const factor = value / 100 + 1; // Convert percentage to factor

  for (let i = 0; i < data.length; i += 4) {
    data[i] = Math.min(255, data[i] * factor); // R
    data[i + 1] = Math.min(255, data[i + 1] * factor); // G
    data[i + 2] = Math.min(255, data[i + 2] * factor); // B
  }
}

function applyContrast(data: Uint8ClampedArray, value: number): void {
  const factor = (value + 100) / 100; // Convert percentage to factor
  const intercept = 128 * (1 - factor);

  for (let i = 0; i < data.length; i += 4) {
    data[i] = Math.max(0, Math.min(255, data[i] * factor + intercept)); // R
    data[i + 1] = Math.max(0, Math.min(255, data[i + 1] * factor + intercept)); // G
    data[i + 2] = Math.max(0, Math.min(255, data[i + 2] * factor + intercept)); // B
  }
}

function applySaturation(data: Uint8ClampedArray, value: number): void {
  const factor = value / 100 + 1;

  for (let i = 0; i < data.length; i += 4) {
    const r = data[i];
    const g = data[i + 1];
    const b = data[i + 2];

    // Convert to HSL-like saturation adjustment
    const gray = (r + g + b) / 3;
    data[i] = Math.max(0, Math.min(255, gray + (r - gray) * factor));
    data[i + 1] = Math.max(0, Math.min(255, gray + (g - gray) * factor));
    data[i + 2] = Math.max(0, Math.min(255, gray + (b - gray) * factor));
  }
}

function applyHue(data: Uint8ClampedArray, value: number): void {
  const hueShift = (value / 100) * 360; // Convert percentage to degrees

  for (let i = 0; i < data.length; i += 4) {
    const r = data[i] / 255;
    const g = data[i + 1] / 255;
    const b = data[i + 2] / 255;

    // Simple hue shift (basic implementation)
    const max = Math.max(r, g, b);
    const min = Math.min(r, g, b);
    const delta = max - min;

    if (delta === 0) continue; // No hue for grayscale

    let h = 0;
    if (max === r) h = ((g - b) / delta) % 6;
    else if (max === g) h = (b - r) / delta + 2;
    else h = (r - g) / delta + 4;

    h = (h + hueShift / 60) % 6;

    // Convert back to RGB (simplified)
    const c = delta;
    const x = c * (1 - Math.abs((h % 2) - 1));
    const m = max - c;

    let newR = 0,
      newG = 0,
      newB = 0;

    if (h >= 0 && h < 1) {
      newR = c;
      newG = x;
      newB = 0;
    } else if (h >= 1 && h < 2) {
      newR = x;
      newG = c;
      newB = 0;
    } else if (h >= 2 && h < 3) {
      newR = 0;
      newG = c;
      newB = x;
    } else if (h >= 3 && h < 4) {
      newR = 0;
      newG = x;
      newB = c;
    } else if (h >= 4 && h < 5) {
      newR = x;
      newG = 0;
      newB = c;
    } else {
      newR = c;
      newG = 0;
      newB = x;
    }

    data[i] = Math.round((newR + m) * 255);
    data[i + 1] = Math.round((newG + m) * 255);
    data[i + 2] = Math.round((newB + m) * 255);
  }
}

function applyBlur(
  ctx: CanvasRenderingContext2D,
  canvas: HTMLCanvasElement,
  value: number
): void {
  // Simple box blur implementation
  const radius = Math.max(1, Math.round(value / 10));

  for (let y = 0; y < canvas.height; y++) {
    for (let x = 0; x < canvas.width; x++) {
      let r = 0,
        g = 0,
        b = 0,
        a = 0,
        count = 0;

      // Sample neighboring pixels
      for (let dy = -radius; dy <= radius; dy++) {
        for (let dx = -radius; dx <= radius; dx++) {
          const nx = Math.max(0, Math.min(canvas.width - 1, x + dx));
          const ny = Math.max(0, Math.min(canvas.height - 1, y + dy));

          const pixel = ctx.getImageData(nx, ny, 1, 1).data;
          r += pixel[0];
          g += pixel[1];
          b += pixel[2];
          a += pixel[3];
          count++;
        }
      }

      // Set averaged pixel
      const imageData = ctx.createImageData(1, 1);
      imageData.data[0] = r / count;
      imageData.data[1] = g / count;
      imageData.data[2] = b / count;
      imageData.data[3] = a / count;
      ctx.putImageData(imageData, x, y);
    }
  }
}

function applySharpen(
  ctx: CanvasRenderingContext2D,
  canvas: HTMLCanvasElement,
  value: number
): void {
  // Simple sharpen using unsharp mask effect
  const strength = value / 100;
  const originalData = ctx.getImageData(0, 0, canvas.width, canvas.height);

  // Create blurred version
  const blurredCanvas = document.createElement("canvas");
  blurredCanvas.width = canvas.width;
  blurredCanvas.height = canvas.height;
  const blurredCtx = blurredCanvas.getContext("2d");
  if (!blurredCtx) {
    throw new Error("Unable to get 2d context for blurred canvas");
  }
  blurredCtx.drawImage(canvas, 0, 0);
  applyBlur(blurredCtx, blurredCanvas, 2);

  const blurredData = blurredCtx.getImageData(
    0,
    0,
    canvas.width,
    canvas.height
  );

  // Apply unsharp mask
  for (let i = 0; i < originalData.data.length; i += 4) {
    originalData.data[i] +=
      (originalData.data[i] - blurredData.data[i]) * strength; // R
    originalData.data[i + 1] +=
      (originalData.data[i + 1] - blurredData.data[i + 1]) * strength; // G
    originalData.data[i + 2] +=
      (originalData.data[i + 2] - blurredData.data[i + 2]) * strength; // B
  }

  ctx.putImageData(originalData, 0, 0);
}

function applyNoise(data: Uint8ClampedArray, value: number): void {
  const intensity = value / 100;

  for (let i = 0; i < data.length; i += 4) {
    const noise = (Math.random() - 0.5) * intensity * 255;
    data[i] = Math.max(0, Math.min(255, data[i] + noise)); // R
    data[i + 1] = Math.max(0, Math.min(255, data[i + 1] + noise)); // G
    data[i + 2] = Math.max(0, Math.min(255, data[i + 2] + noise)); // B
  }
}

/**
 * Create normal map from height map
 */
export function createNormalMapFromHeight(
  heightTexture: THREE.Texture,
  strength = 1
): THREE.Texture {
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");

  if (!(ctx && heightTexture.image)) {
    throw new Error("Unable to create normal map");
  }

  const image = heightTexture.image as HTMLImageElement | HTMLCanvasElement;
  canvas.width = image.width;
  canvas.height = image.height;

  // Draw height map
  ctx.drawImage(image, 0, 0);
  const heightData = ctx.getImageData(0, 0, canvas.width, canvas.height);

  // Create normal map data
  const normalData = ctx.createImageData(canvas.width, canvas.height);

  for (let y = 0; y < canvas.height; y++) {
    for (let x = 0; x < canvas.width; x++) {
      const i = (y * canvas.width + x) * 4;

      // Sample neighboring heights
      const hL =
        x > 0
          ? heightData.data[(y * canvas.width + (x - 1)) * 4]
          : heightData.data[i];
      const hR =
        x < canvas.width - 1
          ? heightData.data[(y * canvas.width + (x + 1)) * 4]
          : heightData.data[i];
      const hU =
        y > 0
          ? heightData.data[((y - 1) * canvas.width + x) * 4]
          : heightData.data[i];
      const hD =
        y < canvas.height - 1
          ? heightData.data[((y + 1) * canvas.width + x) * 4]
          : heightData.data[i];

      // Calculate normal
      const dx = (hR - hL) * strength;
      const dy = (hD - hU) * strength;
      const dz = 1.0 / strength;

      // Normalize
      const length = Math.sqrt(dx * dx + dy * dy + dz * dz);
      const nx = dx / length;
      const ny = dy / length;
      const nz = dz / length;

      // Convert to RGB (normal maps use 0-1 range mapped to 0-255)
      normalData.data[i] = (nx + 1) * 127.5; // R
      normalData.data[i + 1] = (ny + 1) * 127.5; // G
      normalData.data[i + 2] = (nz + 1) * 127.5; // B
      normalData.data[i + 3] = 255; // A
    }
  }

  ctx.putImageData(normalData, 0, 0);

  const normalTexture = new THREE.CanvasTexture(canvas);
  normalTexture.colorSpace = THREE.LinearSRGBColorSpace; // Normal maps are linear

  return normalTexture;
}

/**
 * Generate mipmaps manually for textures
 */
export function generateMipmaps(texture: THREE.Texture): THREE.Texture[] {
  const mipmaps: THREE.Texture[] = [];
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");

  if (!(ctx && texture.image)) {
    return mipmaps;
  }

  const image = texture.image as HTMLImageElement | HTMLCanvasElement;
  let width = image.width;
  let height = image.height;

  // Generate mipmaps by successively halving dimensions
  while (width > 1 || height > 1) {
    canvas.width = width;
    canvas.height = height;
    ctx.drawImage(image, 0, 0, width, height);

    const mipmapTexture = new THREE.CanvasTexture(canvas);
    applyTextureOptions(mipmapTexture, {
      generateMipmaps: false, // Don't generate further mipmaps
    });

    mipmaps.push(mipmapTexture);

    width = Math.max(1, Math.floor(width / 2));
    height = Math.max(1, Math.floor(height / 2));
  }

  return mipmaps;
}

/**
 * Compress texture using various algorithms
 */
export function compressTexture(
  _texture: THREE.Texture,
  _format: "astc" | "dxt" | "etc" | "pvrtc",
  _quality: "low" | "medium" | "high" = "medium"
): THREE.CompressedTexture | null {
  // Note: Actual compression would require WebGL extensions or external libraries
  // This is a placeholder for future implementation

  console.warn(
    `Texture compression (${_format}) not yet implemented. Using uncompressed texture.`
  );

  // For now, return null to indicate compression is not available
  // In a full implementation, this would:
  // 1. Use WebGL compressed texture extensions
  // 2. Use libraries like basis_universal
  // 3. Return a CompressedTexture with the appropriate format

  return null;
}

/**
 * Get texture memory usage estimation
 */
export function estimateTextureMemoryUsage(texture: THREE.Texture): number {
  if (!texture.image) return 0;

  const image = texture.image as any;
  const width = image.width || 0;
  const height = image.height || 0;

  // Estimate bytes per pixel based on format
  let bytesPerPixel = 4; // Default RGBA
  switch (texture.format) {
    case THREE.RGBFormat:
      bytesPerPixel = 3;
      break;
    case THREE.RGBAFormat:
      bytesPerPixel = 4;
      break;
    // LuminanceFormat and LuminanceAlphaFormat were removed in newer Three.js versions
  }

  let totalBytes = width * height * bytesPerPixel;

  // Add mipmap memory if generated
  if (texture.generateMipmaps) {
    let mipWidth = width;
    let mipHeight = height;
    while (mipWidth > 1 || mipHeight > 1) {
      mipWidth = Math.max(1, Math.floor(mipWidth / 2));
      mipHeight = Math.max(1, Math.floor(mipHeight / 2));
      totalBytes += mipWidth * mipHeight * bytesPerPixel;
    }
  }

  return totalBytes;
}

/**
 * Validate texture for common issues
 */
export function validateTextureForUse(texture: THREE.Texture): {
  isValid: boolean;
  warnings: string[];
  recommendations: string[];
} {
  const warnings: string[] = [];
  const recommendations: string[] = [];

  if (!texture.image) {
    return {
      isValid: false,
      warnings: ["Texture has no image data"],
      recommendations: ["Ensure texture is properly loaded"],
    };
  }

  const image = texture.image as any;
  const width = image.width || 0;
  const height = image.height || 0;

  // Check dimensions
  if (width === 0 || height === 0) {
    warnings.push("Texture has zero dimensions");
  }

  // Check power of two for mipmaps
  if (texture.generateMipmaps) {
    const isPowerOfTwo = (n: number) => n > 0 && Number.isInteger(Math.log2(n));
    if (!(isPowerOfTwo(width) && isPowerOfTwo(height))) {
      warnings.push(
        "Non-power-of-two dimensions with mipmaps may cause issues"
      );
      recommendations.push("Resize texture to power-of-two dimensions");
    }
  }

  // Check texture size
  const maxSize = 4096; // Common GPU limit
  if (width > maxSize || height > maxSize) {
    warnings.push(
      `Texture dimensions exceed common GPU limits (${maxSize}x${maxSize})`
    );
    recommendations.push("Consider resizing texture or using texture atlases");
  }

  // Memory usage warning
  const memoryUsage = estimateTextureMemoryUsage(texture);
  if (memoryUsage > 50 * 1024 * 1024) {
    // 50MB
    warnings.push("Texture uses significant memory");
    recommendations.push("Consider compressing or resizing texture");
  }

  return {
    isValid: warnings.length === 0,
    warnings,
    recommendations,
  };
}
