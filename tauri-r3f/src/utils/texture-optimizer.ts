import * as THREE from "three";

// Texture optimization and compression utilities
export interface TextureOptimizationOptions {
  generateMipmaps?: boolean;
  anisotropy?: number;
  format?: THREE.PixelFormat;
  compression?: "none" | "dxt" | "etc" | "pvrtc" | "astc";
  quality?: "low" | "medium" | "high";
  maxSize?: number;
  minFilter?: THREE.TextureFilter;
  magFilter?: THREE.TextureFilter;
}

export class TextureOptimizer {
  private readonly textureCache = new Map<string, THREE.Texture>();
  private static readonly DEFAULT_ANISOTROPY = 4;
  private static readonly MAX_TEXTURE_SIZE = 2048;

  // Get WebGL capabilities for format support detection
  private getWebGLCapabilities(renderer: THREE.WebGLRenderer): {
    supportsDXT: boolean;
    supportsETC: boolean;
    supportsPVRTC: boolean;
    supportsASTC: boolean;
    maxAnisotropy: number;
  } {
    const gl = renderer.getContext();
    const ext = renderer.capabilities;

    return {
      supportsDXT: !!(
        gl.getExtension("WEBGL_compressed_texture_s3tc") ||
        gl.getExtension("MOZ_WEBGL_compressed_texture_s3tc") ||
        gl.getExtension("WEBKIT_WEBGL_compressed_texture_s3tc")
      ),
      supportsETC: !!gl.getExtension("WEBGL_compressed_texture_etc"),
      supportsPVRTC: !!gl.getExtension("WEBGL_compressed_texture_pvrtc"),
      supportsASTC: !!gl.getExtension("WEBGL_compressed_texture_astc"),
      maxAnisotropy: ext.getMaxAnisotropy?.() || 1,
    };
  }

  // Optimize texture based on options and hardware capabilities
  optimizeTexture(
    texture: THREE.Texture,
    options: TextureOptimizationOptions = {},
    renderer?: THREE.WebGLRenderer
  ): THREE.Texture {
    const caps = renderer ? this.getWebGLCapabilities(renderer) : null;

    // Set mipmapping
    texture.generateMipmaps = options.generateMipmaps !== false;
    texture.minFilter =
      options.minFilter ||
      (texture.generateMipmaps
        ? THREE.LinearMipmapLinearFilter
        : THREE.LinearFilter);
    texture.magFilter =
      (options.magFilter as THREE.MagnificationTextureFilter) ||
      THREE.LinearFilter;

    // Set anisotropy for better quality at angles
    const anisotropy = Math.min(
      options.anisotropy || TextureOptimizer.DEFAULT_ANISOTROPY,
      caps?.maxAnisotropy || TextureOptimizer.DEFAULT_ANISOTROPY
    );
    texture.anisotropy = anisotropy;

    // Resize if too large
    const image = texture.image as { width?: number; height?: number };
    if (
      options.maxSize &&
      image &&
      typeof image.width === "number" &&
      typeof image.height === "number" &&
      (image.width > options.maxSize || image.height > options.maxSize)
    ) {
      this.resizeTexture(texture, options.maxSize);
    }

    // Set encoding based on format preference
    if (texture.format === THREE.RGBAFormat) {
      (texture as { colorSpace?: string }).colorSpace = "srgb";
    }

    // Configure wrapping and repeat
    texture.wrapS = THREE.RepeatWrapping;
    texture.wrapT = THREE.RepeatWrapping;

    // Mark as needing update
    texture.needsUpdate = true;

    return texture;
  }

  // Resize texture to maximum size while maintaining aspect ratio
  private resizeTexture(texture: THREE.Texture, maxSize: number): void {
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d");
    if (!(ctx && texture.image)) return;

    const img = texture.image as { width?: number; height?: number };
    if (!img || typeof img.width !== "number" || typeof img.height !== "number")
      return;
    const aspectRatio = img.width / img.height;

    let newWidth = maxSize;
    let newHeight = maxSize;

    if (aspectRatio > 1) {
      newHeight = maxSize / aspectRatio;
    } else {
      newWidth = maxSize * aspectRatio;
    }

    canvas.width = newWidth;
    canvas.height = newHeight;

    ctx.drawImage(img as CanvasImageSource, 0, 0, newWidth, newHeight);
    texture.image = canvas;
  }

  // Create optimized texture from image
  async createOptimizedTexture(
    imageUrl: string,
    options: TextureOptimizationOptions = {},
    renderer?: THREE.WebGLRenderer
  ): Promise<THREE.Texture> {
    const cacheKey = `${imageUrl}_${JSON.stringify(options)}`;

    // Check cache first
    const cached = this.textureCache.get(cacheKey);
    if (cached) {
      return cached.clone();
    }

    // Load image
    const loader = new THREE.TextureLoader();
    const texture = await new Promise<THREE.Texture>((resolve, reject) => {
      loader.load(imageUrl, resolve, undefined, reject);
    });

    // Optimize texture
    const optimized = this.optimizeTexture(texture, options, renderer);

    // Cache optimized texture
    this.textureCache.set(cacheKey, optimized.clone());

    return optimized;
  }

  // Compress texture using available compression formats
  compressTexture(
    _texture: THREE.Texture,
    compression: "dxt" | "etc" | "pvrtc" | "astc",
    renderer: THREE.WebGLRenderer
  ): THREE.CompressedTexture | null {
    const caps = this.getWebGLCapabilities(renderer);

    // Check if compression is supported
    switch (compression) {
      case "dxt":
        if (!caps.supportsDXT) return null;
        break;
      case "etc":
        if (!caps.supportsETC) return null;
        break;
      case "pvrtc":
        if (!caps.supportsPVRTC) return null;
        break;
      case "astc":
        if (!caps.supportsASTC) return null;
        break;
      default:
        return null; // Unsupported compression format
    }

    // Note: Actual compression would require additional libraries or WebGL extensions
    // This is a placeholder for future implementation with libraries like:
    // - basis_universal for Universal Texture Compression
    // - WebGL compressed texture extensions

    console.warn(
      `Texture compression (${compression}) not yet implemented. Using uncompressed texture.`
    );
    return null;
  }

  // Preload and optimize commonly used textures
  async preloadTextures(
    textureUrls: string[],
    options: TextureOptimizationOptions = {},
    renderer?: THREE.WebGLRenderer
  ): Promise<Map<string, THREE.Texture>> {
    const promises = textureUrls.map((url) =>
      this.createOptimizedTexture(url, options, renderer).then(
        (texture) => [url, texture] as const
      )
    );

    const results = await Promise.all(promises);
    return new Map(results);
  }

  // Clear texture cache
  clearCache(): void {
    for (const texture of this.textureCache.values()) {
      texture.dispose();
    }
    this.textureCache.clear();
  }

  // Get cache statistics
  getCacheStats(): { size: number; memoryUsage: number } {
    let memoryUsage = 0;

    for (const texture of this.textureCache.values()) {
      // Estimate memory usage (rough calculation)
      const image = texture.image as { width?: number; height?: number };
      const width = image && typeof image.width === "number" ? image.width : 0;
      const height =
        image && typeof image.height === "number" ? image.height : 0;
      const bytesPerPixel = 4; // RGBA
      memoryUsage += width * height * bytesPerPixel;
    }

    return {
      size: this.textureCache.size,
      memoryUsage,
    };
  }

  // Get optimal texture settings for current hardware
  getOptimalSettings(
    renderer?: THREE.WebGLRenderer
  ): TextureOptimizationOptions {
    const caps = renderer ? this.getWebGLCapabilities(renderer) : null;

    return {
      generateMipmaps: true,
      anisotropy: Math.min(
        TextureOptimizer.DEFAULT_ANISOTROPY,
        caps?.maxAnisotropy || 1
      ),
      maxSize: TextureOptimizer.MAX_TEXTURE_SIZE,
      minFilter: THREE.LinearMipmapLinearFilter,
      magFilter: THREE.LinearFilter,
      compression: this.getOptimalCompression(caps || undefined),
    };
  }

  private getOptimalCompression(caps?: {
    supportsDXT: boolean;
    supportsETC: boolean;
    supportsPVRTC: boolean;
    supportsASTC: boolean;
  }): "none" | "dxt" | "etc" | "pvrtc" | "astc" {
    if (caps?.supportsASTC) return "astc";
    if (caps?.supportsDXT) return "dxt";
    if (caps?.supportsETC) return "etc";
    if (caps?.supportsPVRTC) return "pvrtc";
    return "none";
  }
}

// Singleton instance
export const textureOptimizer = new TextureOptimizer();
