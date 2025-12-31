# Import Pipeline (Source → Runtime)

## Overview

Rengine implements a comprehensive import pipeline that transforms source assets (raw files) into optimized runtime assets suitable for game execution. This pipeline handles format conversion, optimization, compression, and platform-specific processing to ensure assets are ready for efficient runtime loading and rendering.

## Core Concepts

### Pipeline Stages

The import pipeline consists of several sequential stages:

1. **Discovery**: Find and catalog source assets
2. **Analysis**: Examine asset properties and dependencies
3. **Preprocessing**: Initial format conversion and validation
4. **Processing**: Core optimization and transformation
5. **Optimization**: Platform-specific optimizations
6. **Packaging**: Final packaging for runtime use

### Asset Types

Different asset types have specialized import pipelines:

- **Textures**: Compression, mipmapping, format conversion
- **Models**: LOD generation, bone optimization, material assignment
- **Audio**: Compression, format conversion, streaming setup
- **Animations**: Keyframe optimization, compression
- **Shaders**: Compilation, optimization, platform-specific code generation

## Pipeline Architecture

### Pipeline Manager

Central coordinator for asset import operations:

```typescript
class AssetImportPipeline {
  private processors = new Map<AssetType, AssetProcessor[]>();
  private cache = new Map<string, PipelineCacheEntry>();

  constructor() {
    this.initializeProcessors();
  }

  async importAsset(sourcePath: string, options: ImportOptions = {}): Promise<ImportedAsset> {
    const assetType = this.detectAssetType(sourcePath);
    const cacheKey = this.generateCacheKey(sourcePath, options);

    // Check cache first
    if (this.cache.has(cacheKey) && !options.forceReimport) {
      return this.cache.get(cacheKey)!.result;
    }

    // Run import pipeline
    const result = await this.runPipeline(sourcePath, assetType, options);

    // Cache result
    this.cache.set(cacheKey, {
      sourcePath,
      options,
      result,
      timestamp: Date.now()
    });

    return result;
  }

  private async runPipeline(
    sourcePath: string,
    assetType: AssetType,
    options: ImportOptions
  ): Promise<ImportedAsset> {
    const processors = this.processors.get(assetType) || [];

    let currentAsset: PipelineAsset = {
      sourcePath,
      type: assetType,
      data: await this.loadSourceAsset(sourcePath),
      metadata: {}
    };

    // Run each processor in sequence
    for (const processor of processors) {
      currentAsset = await processor.process(currentAsset, options);
    }

    return this.createImportedAsset(currentAsset);
  }

  private initializeProcessors(): void {
    // Texture processors
    this.processors.set(AssetType.TEXTURE, [
      new TextureValidator(),
      new TextureCompressor(),
      new MipmapGenerator(),
      new TextureOptimizer()
    ]);

    // Model processors
    this.processors.set(AssetType.MODEL, [
      new ModelValidator(),
      new ModelOptimizer(),
      new LODGenerator(),
      new MaterialProcessor()
    ]);

    // Audio processors
    this.processors.set(AssetType.AUDIO, [
      new AudioValidator(),
      new AudioCompressor(),
      new AudioNormalizer()
    ]);
  }
}
```

### Processor Interface

Standard interface for all asset processors:

```typescript
interface AssetProcessor {
  readonly name: string;
  readonly supportedTypes: AssetType[];

  process(asset: PipelineAsset, options: ImportOptions): Promise<PipelineAsset>;
  getCapabilities(): ProcessorCapabilities;
}

interface PipelineAsset {
  sourcePath: string;
  type: AssetType;
  data: any; // Source asset data
  metadata: Record<string, any>; // Processing metadata
  dependencies?: string[]; // Asset dependencies
  warnings?: string[]; // Non-fatal issues
  errors?: string[]; // Fatal errors
}

interface ProcessorCapabilities {
  canProcess(assetType: AssetType): boolean;
  getSupportedFormats(): string[];
  getOutputFormats(): string[];
  isLossy(): boolean; // Whether processing loses data
}
```

## Texture Import Pipeline

### Texture Processing Stages

```typescript
class TextureValidator implements AssetProcessor {
  readonly name = 'TextureValidator';

  async process(asset: PipelineAsset, options: ImportOptions): Promise<PipelineAsset> {
    const imageData = asset.data as ImageData;

    // Validate image dimensions
    if (imageData.width > 8192 || imageData.height > 8192) {
      asset.warnings = asset.warnings || [];
      asset.warnings.push('Texture dimensions exceed recommended maximum (8192x8192)');
    }

    // Check for power-of-two dimensions
    const isPOT = this.isPowerOfTwo(imageData.width) && this.isPowerOfTwo(imageData.height);
    if (!isPOT && options.requirePOT) {
      asset.errors = asset.errors || [];
      asset.errors.push('Texture dimensions must be power-of-two');
    }

    // Validate color space
    if (imageData.colorSpace !== 'srgb') {
      asset.metadata.colorSpace = imageData.colorSpace;
    }

    return asset;
  }

  private isPowerOfTwo(n: number): boolean {
    return (n & (n - 1)) === 0;
  }
}

class TextureCompressor implements AssetProcessor {
  readonly name = 'TextureCompressor';

  async process(asset: PipelineAsset, options: ImportOptions): Promise<PipelineAsset> {
    const imageData = asset.data as ImageData;

    // Choose compression format based on platform and options
    const compressionFormat = this.selectCompressionFormat(options.targetPlatform);

    // Compress texture
    const compressedData = await this.compressTexture(imageData, compressionFormat);

    return {
      ...asset,
      data: compressedData,
      metadata: {
        ...asset.metadata,
        compressionFormat,
        originalSize: imageData.width * imageData.height * 4, // Assuming RGBA
        compressedSize: compressedData.length
      }
    };
  }

  private selectCompressionFormat(platform: Platform): TextureFormat {
    switch (platform) {
      case Platform.WEBGL:
        return TextureFormat.DXT5; // WebGL supports DXT
      case Platform.METAL:
        return TextureFormat.ASTC; // Metal prefers ASTC
      case Platform.VULKAN:
        return TextureFormat.BC7; // Vulkan supports BC7
      default:
        return TextureFormat.RGBA; // Fallback to uncompressed
    }
  }
}

class MipmapGenerator implements AssetProcessor {
  readonly name = 'MipmapGenerator';

  async process(asset: PipelineAsset, options: ImportOptions): Promise<PipelineAsset> {
    if (!options.generateMipmaps) return asset;

    const imageData = asset.data as CompressedTextureData;

    // Generate mipmap chain
    const mipmaps = await this.generateMipmaps(imageData);

    return {
      ...asset,
      data: {
        ...imageData,
        mipmaps
      },
      metadata: {
        ...asset.metadata,
        mipmapCount: mipmaps.length,
        hasMipmaps: true
      }
    };
  }

  private async generateMipmaps(textureData: CompressedTextureData): Promise<MipmapLevel[]> {
    const mipmaps: MipmapLevel[] = [];
    let currentData = textureData.data;
    let width = textureData.width;
    let height = textureData.height;

    while (width > 1 || height > 1) {
      // Generate next mipmap level
      const mipmapData = await this.downsampleTexture(currentData, width, height);
      mipmaps.push({
        data: mipmapData,
        width: Math.max(1, width / 2),
        height: Math.max(1, height / 2)
      });

      currentData = mipmapData;
      width = Math.max(1, width / 2);
      height = Math.max(1, height / 2);
    }

    return mipmaps;
  }
}
```

## Model Import Pipeline

### Model Processing Stages

```typescript
class ModelValidator implements AssetProcessor {
  readonly name = 'ModelValidator';

  async process(asset: PipelineAsset, options: ImportOptions): Promise<PipelineAsset> {
    const modelData = asset.data as ModelData;

    // Validate mesh data
    for (const mesh of modelData.meshes) {
      if (!mesh.vertices || mesh.vertices.length === 0) {
        asset.errors = asset.errors || [];
        asset.errors.push(`Mesh "${mesh.name}" has no vertices`);
      }

      if (!mesh.indices || mesh.indices.length === 0) {
        asset.warnings = asset.warnings || [];
        asset.warnings.push(`Mesh "${mesh.name}" has no indices`);
      }
    }

    // Check for degenerate triangles
    const degenerateCount = this.countDegenerateTriangles(modelData);
    if (degenerateCount > 0) {
      asset.warnings = asset.warnings || [];
      asset.warnings.push(`Found ${degenerateCount} degenerate triangles`);
    }

    // Validate skeleton
    if (modelData.skeleton) {
      this.validateSkeleton(modelData.skeleton, asset);
    }

    return asset;
  }
}

class ModelOptimizer implements AssetProcessor {
  readonly name = 'ModelOptimizer';

  async process(asset: PipelineAsset, options: ImportOptions): Promise<PipelineAsset> {
    const modelData = asset.data as ModelData;

    // Optimize vertex cache
    for (const mesh of modelData.meshes) {
      mesh.indices = this.optimizeVertexCache(mesh.vertices, mesh.indices);
    }

    // Merge similar materials
    modelData.materials = this.mergeSimilarMaterials(modelData.materials);

    // Optimize skeleton
    if (modelData.skeleton) {
      modelData.skeleton = this.optimizeSkeleton(modelData.skeleton);
    }

    return {
      ...asset,
      metadata: {
        ...asset.metadata,
        optimized: true,
        originalTriangleCount: this.countTriangles(modelData),
        optimizedTriangleCount: this.countTriangles(modelData)
      }
    };
  }
}

class LODGenerator implements AssetProcessor {
  readonly name = 'LODGenerator';

  async process(asset: PipelineAsset, options: ImportOptions): Promise<PipelineAsset> {
    if (!options.generateLODs) return asset;

    const modelData = asset.data as ModelData;

    // Generate LOD levels
    const lods = await this.generateLODs(modelData, options.lodLevels || 3);

    return {
      ...asset,
      data: {
        ...modelData,
        lods
      },
      metadata: {
        ...asset.metadata,
        lodCount: lods.length,
        hasLODs: true
      }
    };
  }

  private async generateLODs(modelData: ModelData, levelCount: number): Promise<LODLevel[]> {
    const lods: LODLevel[] = [];

    for (let i = 0; i < levelCount; i++) {
      const reductionFactor = Math.pow(0.5, i + 1); // 50%, 25%, 12.5%, etc.
      const lodModel = await this.reduceModel(modelData, reductionFactor);

      lods.push({
        distance: 10 * Math.pow(2, i), // 10, 20, 40, etc.
        model: lodModel
      });
    }

    return lods;
  }
}
```

## Audio Import Pipeline

### Audio Processing Stages

```typescript
class AudioValidator implements AssetProcessor {
  readonly name = 'AudioValidator';

  async process(asset: PipelineAsset, options: ImportOptions): Promise<PipelineAsset> {
    const audioData = asset.data as AudioData;

    // Validate sample rate
    if (audioData.sampleRate < 8000 || audioData.sampleRate > 192000) {
      asset.warnings = asset.warnings || [];
      asset.warnings.push(`Unusual sample rate: ${audioData.sampleRate}Hz`);
    }

    // Check bit depth
    if (audioData.bitDepth !== 16 && audioData.bitDepth !== 24) {
      asset.warnings = asset.warnings || [];
      asset.warnings.push(`Non-standard bit depth: ${audioData.bitDepth} bits`);
    }

    // Validate duration
    if (audioData.duration > 300) { // 5 minutes
      asset.warnings = asset.warnings || [];
      asset.warnings.push('Audio file is very long, consider streaming');
    }

    return asset;
  }
}

class AudioCompressor implements AssetProcessor {
  readonly name = 'AudioCompressor';

  async process(asset: PipelineAsset, options: ImportOptions): Promise<PipelineAsset> {
    const audioData = asset.data as AudioData;

    // Choose compression format
    const format = this.selectAudioFormat(options.targetPlatform, audioData);

    // Compress audio
    const compressedData = await this.compressAudio(audioData, format);

    return {
      ...asset,
      data: compressedData,
      metadata: {
        ...asset.metadata,
        compressionFormat: format,
        originalSize: audioData.data.length,
        compressedSize: compressedData.length,
        compressionRatio: audioData.data.length / compressedData.length
      }
    };
  }

  private selectAudioFormat(platform: Platform, audioData: AudioData): AudioFormat {
    // For short sounds, use compressed formats
    if (audioData.duration < 5.0) {
      switch (platform) {
        case Platform.WEB:
          return AudioFormat.MP3;
        case Platform.MOBILE:
          return AudioFormat.AAC;
        default:
          return AudioFormat.VORBIS;
      }
    }

    // For long sounds, consider streaming
    return AudioFormat.UNCOMPRESSED;
  }
}
```

## Pipeline Configuration

### Import Settings

Configurable import options:

```typescript
interface ImportOptions {
  // General options
  forceReimport?: boolean;
  targetPlatform?: Platform;

  // Texture options
  generateMipmaps?: boolean;
  textureFormat?: TextureFormat;
  maxTextureSize?: number;
  requirePOT?: boolean;

  // Model options
  generateLODs?: boolean;
  lodLevels?: number;
  optimizeMeshes?: boolean;
  mergeMeshes?: boolean;

  // Audio options
  audioFormat?: AudioFormat;
  normalizeAudio?: boolean;
  enableStreaming?: boolean;

  // Quality options
  quality?: 'low' | 'medium' | 'high' | 'ultra';
  compressionLevel?: number;
}

interface PlatformPreset {
  name: string;
  options: ImportOptions;
}

// Platform-specific presets
const platformPresets: Record<Platform, PlatformPreset> = {
  [Platform.WEB]: {
    name: 'Web',
    options: {
      generateMipmaps: true,
      textureFormat: TextureFormat.DXT5,
      maxTextureSize: 2048,
      generateLODs: true,
      lodLevels: 3,
      audioFormat: AudioFormat.MP3
    }
  },
  [Platform.MOBILE]: {
    name: 'Mobile',
    options: {
      generateMipmaps: true,
      textureFormat: TextureFormat.ASTC,
      maxTextureSize: 1024,
      generateLODs: true,
      lodLevels: 2,
      audioFormat: AudioFormat.AAC,
      quality: 'medium'
    }
  },
  [Platform.PC]: {
    name: 'PC',
    options: {
      generateMipmaps: true,
      textureFormat: TextureFormat.BC7,
      maxTextureSize: 4096,
      generateLODs: true,
      lodLevels: 4,
      audioFormat: AudioFormat.VORBIS,
      quality: 'high'
    }
  }
};
```

## Error Handling and Recovery

### Pipeline Error Handling

Robust error handling throughout the pipeline:

```typescript
class PipelineErrorHandler {
  async handleProcessorError(
    processor: AssetProcessor,
    asset: PipelineAsset,
    error: Error,
    options: ErrorHandlingOptions
  ): Promise<PipelineAsset> {
    console.error(`Pipeline error in ${processor.name}:`, error);

    // Log error details
    await this.logError(processor, asset, error);

    // Attempt recovery based on error type
    if (error instanceof ValidationError) {
      return this.handleValidationError(asset, error, options);
    } else if (error instanceof ProcessingError) {
      return this.handleProcessingError(asset, error, options);
    } else if (error instanceof MemoryError) {
      return this.handleMemoryError(asset, error, options);
    }

    // Unrecoverable error
    asset.errors = asset.errors || [];
    asset.errors.push(`Unrecoverable error: ${error.message}`);

    return asset;
  }

  private async handleValidationError(
    asset: PipelineAsset,
    error: ValidationError,
    options: ErrorHandlingOptions
  ): Promise<PipelineAsset> {
    if (options.failOnWarnings) {
      asset.errors = asset.errors || [];
      asset.errors.push(`Validation failed: ${error.message}`);
      return asset;
    }

    // Attempt to fix validation issues
    return this.attemptValidationFix(asset, error);
  }

  private async handleProcessingError(
    asset: PipelineAsset,
    error: ProcessingError,
    options: ErrorHandlingOptions
  ): Promise<PipelineAsset> {
    if (options.skipOnError) {
      // Skip this processor and continue
      asset.warnings = asset.warnings || [];
      asset.warnings.push(`Skipped ${error.processor}: ${error.message}`);
      return asset;
    }

    // Try alternative processor
    return this.tryAlternativeProcessor(asset, error);
  }
}
```

## Performance Optimization

### Parallel Processing

Process multiple assets concurrently:

```typescript
class ParallelPipelineProcessor {
  private maxConcurrency = 4;
  private activeJobs = 0;
  private jobQueue: PipelineJob[] = [];

  async processAssets(assets: PipelineAsset[], options: ImportOptions): Promise<ImportedAsset[]> {
    const results: ImportedAsset[] = [];
    const jobPromises: Promise<void>[] = [];

    for (const asset of assets) {
      const jobPromise = this.scheduleJob(asset, options, results);
      jobPromises.push(jobPromise);
    }

    await Promise.all(jobPromises);
    return results;
  }

  private async scheduleJob(
    asset: PipelineAsset,
    options: ImportOptions,
    results: ImportedAsset[]
  ): Promise<void> {
    return new Promise((resolve) => {
      const job = {
        asset,
        options,
        resolve: (result: ImportedAsset) => {
          results.push(result);
          resolve();
        }
      };

      this.jobQueue.push(job);
      this.processQueue();
    });
  }

  private async processQueue(): Promise<void> {
    while (this.activeJobs < this.maxConcurrency && this.jobQueue.length > 0) {
      const job = this.jobQueue.shift()!;
      this.activeJobs++;

      try {
        const result = await assetImportPipeline.importAsset(job.asset.sourcePath, job.options);
        job.resolve(result);
      } catch (error) {
        console.error(`Failed to process asset ${job.asset.sourcePath}:`, error);
        job.resolve(null as any); // Error result
      } finally {
        this.activeJobs--;
        this.processQueue(); // Process next job
      }
    }
  }
}
```

## Caching and Incremental Builds

### Pipeline Cache

Avoid reprocessing unchanged assets:

```typescript
class PipelineCache {
  private cacheFile = '.rengine/pipeline-cache.json';
  private cache = new Map<string, CacheEntry>();

  constructor() {
    this.loadCache();
  }

  getCacheKey(asset: PipelineAsset, options: ImportOptions): string {
    const optionHash = this.hashOptions(options);
    const assetHash = this.hashAsset(asset);
    return `${asset.sourcePath}:${optionHash}:${assetHash}`;
  }

  isCached(cacheKey: string): boolean {
    const entry = this.cache.get(cacheKey);
    if (!entry) return false;

    // Check if source file has changed
    const sourceStat = fs.statSync(entry.sourcePath);
    return sourceStat.mtime.getTime() === entry.sourceModified;
  }

  getCachedResult(cacheKey: string): ImportedAsset | null {
    const entry = this.cache.get(cacheKey);
    return entry ? entry.result : null;
  }

  storeResult(cacheKey: string, asset: PipelineAsset, result: ImportedAsset): void {
    const entry: CacheEntry = {
      cacheKey,
      sourcePath: asset.sourcePath,
      sourceModified: fs.statSync(asset.sourcePath).mtime.getTime(),
      options: {}, // Would store relevant options
      result,
      created: Date.now()
    };

    this.cache.set(cacheKey, entry);
    this.saveCache();
  }

  private hashOptions(options: ImportOptions): string {
    // Create hash of relevant options
    const relevantOptions = {
      targetPlatform: options.targetPlatform,
      quality: options.quality,
      generateMipmaps: options.generateMipmaps,
      generateLODs: options.generateLODs
    };
    return createHash('md5').update(JSON.stringify(relevantOptions)).digest('hex');
  }

  private hashAsset(asset: PipelineAsset): string {
    const stats = fs.statSync(asset.sourcePath);
    return createHash('md5')
      .update(`${stats.size}:${stats.mtime.getTime()}`)
      .digest('hex');
  }
}
```

## Monitoring and Debugging

### Pipeline Metrics

Track pipeline performance and issues:

```typescript
class PipelineMetrics {
  private metrics = new Map<string, ProcessorMetrics>();

  recordProcessorTime(processorName: string, duration: number): void {
    const metrics = this.metrics.get(processorName) || {
      name: processorName,
      totalTime: 0,
      callCount: 0,
      averageTime: 0,
      maxTime: 0,
      minTime: Number.MAX_VALUE,
      errorCount: 0
    };

    metrics.totalTime += duration;
    metrics.callCount++;
    metrics.averageTime = metrics.totalTime / metrics.callCount;
    metrics.maxTime = Math.max(metrics.maxTime, duration);
    metrics.minTime = Math.min(metrics.minTime, duration);

    this.metrics.set(processorName, metrics);
  }

  recordProcessorError(processorName: string): void {
    const metrics = this.metrics.get(processorName);
    if (metrics) {
      metrics.errorCount++;
    }
  }

  getReport(): PipelineReport {
    return {
      processors: Array.from(this.metrics.values()),
      totalProcessingTime: Array.from(this.metrics.values())
        .reduce((sum, m) => sum + m.totalTime, 0),
      mostExpensiveProcessor: this.findMostExpensive(),
      errorRate: this.calculateErrorRate()
    };
  }
}
```

## Integration with Build System

### Build Pipeline Integration

Work with project build systems:

```typescript
class BuildSystemIntegration {
  async processAssetsForBuild(buildConfig: BuildConfig): Promise<void> {
    // Get all assets that need processing
    const assetsToProcess = await this.getAssetsNeedingProcessing(buildConfig);

    // Process assets based on platform
    const platformOptions = platformPresets[buildConfig.targetPlatform];

    // Process in parallel with progress reporting
    const progressCallback = (progress: number) => {
      console.log(`Asset processing: ${Math.round(progress * 100)}%`);
    };

    await parallelProcessor.processAssets(
      assetsToProcess,
      platformOptions.options,
      progressCallback
    );

    // Generate asset manifest
    await this.generateAssetManifest(buildConfig);
  }

  private async getAssetsNeedingProcessing(buildConfig: BuildConfig): Promise<PipelineAsset[]> {
    const allAssets = assetRegistry.getAllAssets();
    const needingProcessing: PipelineAsset[] = [];

    for (const asset of allAssets) {
      const cacheKey = pipelineCache.getCacheKey(asset, buildConfig.options);

      if (!pipelineCache.isCached(cacheKey)) {
        needingProcessing.push(asset);
      }
    }

    return needingProcessing;
  }

  private async generateAssetManifest(buildConfig: BuildConfig): Promise<void> {
    const allAssets = assetRegistry.getAllAssets();
    const manifest: AssetManifest = {
      version: buildConfig.version,
      platform: buildConfig.targetPlatform,
      assets: allAssets.map(asset => ({
        id: asset.id,
        path: asset.path,
        type: asset.type,
        size: asset.size,
        hash: asset.hash
      })),
      dependencies: this.buildDependencyGraph(allAssets)
    };

    await fs.writeFile(
      path.join(buildConfig.outputDir, 'assets.json'),
      JSON.stringify(manifest, null, 2)
    );
  }
}
```

## Best Practices

### Pipeline Design

- **Modular Processors**: Keep processors focused on single responsibilities
- **Error Resilience**: Handle errors gracefully without stopping the entire pipeline
- **Caching Strategy**: Cache results to avoid redundant processing
- **Platform Awareness**: Optimize for target platform capabilities

### Asset Preparation

- **Consistent Naming**: Use clear, consistent naming conventions
- **Quality Guidelines**: Provide quality guidelines for artists
- **Format Selection**: Choose appropriate source formats
- **Metadata**: Include useful metadata with source assets

### Performance Considerations

- **Parallel Processing**: Process independent assets concurrently
- **Incremental Builds**: Only reprocess changed assets
- **Memory Management**: Monitor memory usage during processing
- **Quality Trade-offs**: Balance quality against build time and size

## Future Enhancements

### Advanced Features

- **Machine Learning Optimization**: AI-assisted asset optimization
- **Real-time Processing**: Process assets while the game is running
- **Cloud Processing**: Offload processing to cloud services
- **Custom Processors**: User-defined processing pipeline extensions
- **Visual Pipeline Editor**: GUI for creating custom pipelines
