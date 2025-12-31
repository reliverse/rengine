# Asset Cache

## Overview

Rengine implements a sophisticated multi-level asset caching system that optimizes asset loading performance, reduces memory usage, and provides fast access to frequently used assets. The cache system spans multiple layers from memory to disk, ensuring efficient asset management across different access patterns.

## Core Concepts

### Cache Hierarchy

Rengine uses a multi-tiered caching approach:

1. **L1 Cache (Memory)**: Fast access to hot assets
2. **L2 Cache (Compressed Memory)**: Compressed assets for memory efficiency
3. **Disk Cache**: Persistent storage for frequently used assets
4. **Network Cache**: Cached downloads for remote assets

### Cache Policies

Different caching strategies for different scenarios:

- **LRU (Least Recently Used)**: Evict oldest accessed items
- **LFU (Least Frequently Used)**: Evict least accessed items
- **Size-Based**: Evict largest items when memory constrained
- **Time-Based**: Evict items older than threshold

## Cache Architecture

### Cache Manager

Central coordinator for all caching operations:

```typescript
class AssetCacheManager {
  private l1Cache: MemoryCache;
  private l2Cache: CompressedCache;
  private diskCache: DiskCache;
  private networkCache: NetworkCache;

  constructor(options: CacheOptions) {
    this.l1Cache = new MemoryCache(options.l1Size);
    this.l2Cache = new CompressedCache(options.l2Size);
    this.diskCache = new DiskCache(options.diskPath, options.diskSize);
    this.networkCache = new NetworkCache(options.networkPath);
  }

  async getAsset(assetId: string): Promise<AssetData | null> {
    // Try L1 cache first
    let asset = await this.l1Cache.get(assetId);
    if (asset) {
      this.recordCacheHit('l1', assetId);
      return asset;
    }

    // Try L2 cache
    asset = await this.l2Cache.get(assetId);
    if (asset) {
      this.recordCacheHit('l2', assetId);
      // Promote to L1
      await this.l1Cache.put(assetId, asset);
      return asset;
    }

    // Try disk cache
    asset = await this.diskCache.get(assetId);
    if (asset) {
      this.recordCacheHit('disk', assetId);
      // Promote to higher caches
      await this.l2Cache.put(assetId, asset);
      await this.l1Cache.put(assetId, asset);
      return asset;
    }

    // Try network cache for remote assets
    asset = await this.networkCache.get(assetId);
    if (asset) {
      this.recordCacheHit('network', assetId);
      // Store in other caches
      await this.diskCache.put(assetId, asset);
      await this.l2Cache.put(assetId, asset);
      await this.l1Cache.put(assetId, asset);
      return asset;
    }

    this.recordCacheMiss(assetId);
    return null;
  }

  async putAsset(assetId: string, assetData: AssetData, priority: CachePriority = 'normal'): Promise<void> {
    // Store in all applicable caches
    const promises = [
      this.l1Cache.put(assetId, assetData, priority),
      this.l2Cache.put(assetId, assetData, priority),
      this.diskCache.put(assetId, assetData, priority)
    ];

    await Promise.all(promises);
    this.recordCacheStore(assetId, assetData.size);
  }

  async evictAsset(assetId: string): Promise<void> {
    await Promise.all([
      this.l1Cache.evict(assetId),
      this.l2Cache.evict(assetId),
      this.diskCache.evict(assetId),
      this.networkCache.evict(assetId)
    ]);
  }

  async clearCache(): Promise<void> {
    await Promise.all([
      this.l1Cache.clear(),
      this.l2Cache.clear(),
      this.diskCache.clear(),
      this.networkCache.clear()
    ]);
  }
}
```

### Cache Interfaces

Standard interfaces for all cache implementations:

```typescript
interface Cache {
  get(assetId: string): Promise<AssetData | null>;
  put(assetId: string, assetData: AssetData, priority?: CachePriority): Promise<void>;
  evict(assetId: string): Promise<void>;
  clear(): Promise<void>;
  getStats(): CacheStats;
}

interface CacheStats {
  size: number; // Current cache size in bytes
  maxSize: number; // Maximum cache size in bytes
  itemCount: number; // Number of cached items
  hitCount: number; // Cache hits
  missCount: number; // Cache misses
  evictionCount: number; // Items evicted
  hitRate: number; // Hit rate percentage
}

enum CachePriority {
  LOW = 'low',
  NORMAL = 'normal',
  HIGH = 'high',
  CRITICAL = 'critical'
}
```

## Memory Cache (L1)

### LRU Memory Cache

Fast in-memory cache with LRU eviction:

```typescript
class MemoryCache implements Cache {
  private cache = new Map<string, CacheEntry>();
  private accessOrder = new DoublyLinkedList<string>();
  private size = 0;
  private maxSize: number;

  constructor(maxSize: number) {
    this.maxSize = maxSize;
  }

  async get(assetId: string): Promise<AssetData | null> {
    const entry = this.cache.get(assetId);
    if (!entry) return null;

    // Move to front (most recently used)
    this.accessOrder.moveToFront(assetId);
    entry.lastAccessed = Date.now();

    return entry.data;
  }

  async put(assetId: string, assetData: AssetData, priority: CachePriority = 'normal'): Promise<void> {
    const entrySize = this.calculateEntrySize(assetData);

    // Check if we need to evict
    while (this.size + entrySize > this.maxSize) {
      await this.evictLRU();
    }

    const entry: CacheEntry = {
      data: assetData,
      size: entrySize,
      priority,
      created: Date.now(),
      lastAccessed: Date.now(),
      accessCount: 0
    };

    // Remove existing entry if present
    if (this.cache.has(assetId)) {
      const oldEntry = this.cache.get(assetId)!;
      this.size -= oldEntry.size;
      this.accessOrder.remove(assetId);
    }

    this.cache.set(assetId, entry);
    this.accessOrder.addToFront(assetId);
    this.size += entrySize;
  }

  async evict(assetId: string): Promise<void> {
    const entry = this.cache.get(assetId);
    if (!entry) return;

    this.cache.delete(assetId);
    this.accessOrder.remove(assetId);
    this.size -= entry.size;
  }

  private async evictLRU(): Promise<void> {
    const lruId = this.accessOrder.getTail();
    if (lruId) {
      await this.evict(lruId);
    }
  }

  private calculateEntrySize(assetData: AssetData): number {
    // Estimate memory usage
    if (assetData instanceof ArrayBuffer) {
      return assetData.byteLength;
    }
    if (typeof assetData === 'string') {
      return assetData.length * 2; // UTF-16
    }
    // For complex objects, use a rough estimation
    return JSON.stringify(assetData).length * 2;
  }
}

interface CacheEntry {
  data: AssetData;
  size: number;
  priority: CachePriority;
  created: number;
  lastAccessed: number;
  accessCount: number;
}
```

## Compressed Cache (L2)

### Memory-Efficient Compressed Storage

```typescript
class CompressedCache implements Cache {
  private cache = new Map<string, CompressedEntry>();
  private compressor: Compressor;
  private size = 0;
  private maxSize: number;

  constructor(maxSize: number) {
    this.maxSize = maxSize;
    this.compressor = new LZ4Compressor();
  }

  async get(assetId: string): Promise<AssetData | null> {
    const entry = this.cache.get(assetId);
    if (!entry) return null;

    // Decompress data
    const decompressed = await this.compressor.decompress(entry.compressedData);

    // Deserialize if needed
    return this.deserializeAsset(decompressed, entry.originalType);
  }

  async put(assetId: string, assetData: AssetData, priority: CachePriority = 'normal'): Promise<void> {
    // Serialize asset
    const serialized = this.serializeAsset(assetData);
    const originalSize = serialized.length;

    // Compress data
    const compressed = await this.compressor.compress(serialized);
    const compressedSize = compressed.length;

    const entry: CompressedEntry = {
      compressedData: compressed,
      originalSize,
      compressedSize,
      originalType: this.getAssetType(assetData),
      priority,
      created: Date.now()
    };

    // Check size limits
    if (this.size + compressedSize > this.maxSize) {
      await this.evictToFit(compressedSize);
    }

    this.cache.set(assetId, entry);
    this.size += compressedSize;
  }

  private async evictToFit(neededSpace: number): Promise<void> {
    // Sort entries by priority and access time
    const entries = Array.from(this.cache.entries()).sort((a, b) => {
      const priorityOrder = { low: 0, normal: 1, high: 2, critical: 3 };
      const aPriority = priorityOrder[a[1].priority];
      const bPriority = priorityOrder[b[1].priority];

      if (aPriority !== bPriority) return aPriority - bPriority;

      // Within same priority, evict oldest
      return a[1].created - b[1].created;
    });

    let freedSpace = 0;
    for (const [assetId, entry] of entries) {
      if (freedSpace >= neededSpace) break;

      this.cache.delete(assetId);
      this.size -= entry.compressedSize;
      freedSpace += entry.compressedSize;
    }
  }
}

interface CompressedEntry {
  compressedData: Uint8Array;
  originalSize: number;
  compressedSize: number;
  originalType: string;
  priority: CachePriority;
  created: number;
}
```

## Disk Cache

### Persistent Storage Cache

```typescript
class DiskCache implements Cache {
  private cacheDir: string;
  private indexFile: string;
  private index = new Map<string, DiskEntry>();
  private size = 0;
  private maxSize: number;

  constructor(cacheDir: string, maxSize: number) {
    this.cacheDir = cacheDir;
    this.indexFile = path.join(cacheDir, 'index.json');
    this.maxSize = maxSize;

    this.loadIndex();
    this.ensureCacheDirectory();
  }

  async get(assetId: string): Promise<AssetData | null> {
    const entry = this.index.get(assetId);
    if (!entry) return null;

    const filePath = path.join(this.cacheDir, entry.filename);

    try {
      const data = await fs.readFile(filePath);

      // Update access time
      entry.lastAccessed = Date.now();
      entry.accessCount++;

      await this.saveIndex();

      return this.deserializeAsset(data, entry);
    } catch (error) {
      // File corrupted or missing, remove from index
      this.index.delete(assetId);
      await this.saveIndex();
      return null;
    }
  }

  async put(assetId: string, assetData: AssetData, priority: CachePriority = 'normal'): Promise<void> {
    const filename = this.generateFilename(assetId);
    const filePath = path.join(this.cacheDir, filename);

    // Serialize asset
    const serialized = this.serializeAsset(assetData);

    // Check if we need to evict
    const newSize = serialized.length;
    if (this.size + newSize > this.maxSize) {
      await this.evictToFit(newSize);
    }

    // Write to disk
    await fs.writeFile(filePath, serialized);

    // Update index
    const entry: DiskEntry = {
      filename,
      size: newSize,
      priority,
      created: Date.now(),
      lastAccessed: Date.now(),
      accessCount: 0,
      assetType: this.getAssetType(assetData),
      hash: await this.calculateHash(serialized)
    };

    this.index.set(assetId, entry);
    this.size += newSize;

    await this.saveIndex();
  }

  async evict(assetId: string): Promise<void> {
    const entry = this.index.get(assetId);
    if (!entry) return;

    // Remove file
    const filePath = path.join(this.cacheDir, entry.filename);
    try {
      await fs.unlink(filePath);
    } catch (error) {
      // File might already be gone
    }

    // Update index
    this.index.delete(assetId);
    this.size -= entry.size;
    await this.saveIndex();
  }

  private async evictToFit(neededSpace: number): Promise<void> {
    // Sort by priority, then by access time
    const entries = Array.from(this.index.entries()).sort((a, b) => {
      const priorityOrder = { low: 0, normal: 1, high: 2, critical: 3 };
      const aPriority = priorityOrder[a[1].priority];
      const bPriority = priorityOrder[b[1].priority];

      if (aPriority !== bPriority) return aPriority - bPriority;

      // Within same priority, evict least recently accessed
      return a[1].lastAccessed - b[1].lastAccessed;
    });

    let freedSpace = 0;
    for (const [assetId, entry] of entries) {
      if (freedSpace >= neededSpace) break;

      await this.evict(assetId);
      freedSpace += entry.size;
    }
  }

  private generateFilename(assetId: string): string {
    // Use asset ID with safe filename characters
    return assetId.replace(/[^a-zA-Z0-9-_]/g, '_') + '.cache';
  }

  private async loadIndex(): Promise<void> {
    try {
      const indexData = await fs.readFile(this.indexFile, 'utf-8');
      const index = JSON.parse(indexData);

      this.index = new Map(Object.entries(index));
      this.size = Array.from(this.index.values()).reduce((sum, entry) => sum + entry.size, 0);
    } catch (error) {
      // Index file doesn't exist or is corrupted
      this.index = new Map();
      this.size = 0;
    }
  }

  private async saveIndex(): Promise<void> {
    const indexObject = Object.fromEntries(this.index);
    await fs.writeFile(this.indexFile, JSON.stringify(indexObject, null, 2));
  }

  private async ensureCacheDirectory(): Promise<void> {
    try {
      await fs.access(this.cacheDir);
    } catch {
      await fs.mkdir(this.cacheDir, { recursive: true });
    }
  }
}

interface DiskEntry {
  filename: string;
  size: number;
  priority: CachePriority;
  created: number;
  lastAccessed: number;
  accessCount: number;
  assetType: string;
  hash: string;
}
```

## Network Cache

### Remote Asset Caching

```typescript
class NetworkCache implements Cache {
  private cache = new Map<string, NetworkEntry>();
  private httpClient: HttpClient;

  constructor(cacheDir: string) {
    this.cacheDir = cacheDir;
    this.httpClient = new HttpClient();
    this.loadIndex();
  }

  async get(assetId: string): Promise<AssetData | null> {
    const entry = this.cache.get(assetId);
    if (!entry) return null;

    // Check if cached version is still valid
    if (await this.isCacheValid(entry)) {
      return await this.loadFromCache(entry);
    }

    // Cache is stale, remove it
    this.cache.delete(assetId);
    await this.saveIndex();

    return null;
  }

  async put(assetId: string, assetData: AssetData, priority: CachePriority = 'normal'): Promise<void> {
    // For network cache, we only cache remote assets
    if (!this.isRemoteAsset(assetId)) return;

    const entry: NetworkEntry = {
      assetId,
      url: this.getAssetUrl(assetId),
      etag: await this.getEtag(assetId),
      lastModified: new Date().toISOString(),
      size: this.calculateSize(assetData),
      priority
    };

    // Save to disk
    await this.saveToCache(assetId, assetData);

    this.cache.set(assetId, entry);
    await this.saveIndex();
  }

  private async isCacheValid(entry: NetworkEntry): Promise<boolean> {
    try {
      // Check with server if cache is still valid
      const response = await this.httpClient.head(entry.url);

      const serverEtag = response.headers['etag'];
      const serverLastModified = response.headers['last-modified'];

      // Compare ETags
      if (serverEtag && entry.etag && serverEtag === entry.etag) {
        return true;
      }

      // Compare last modified
      if (serverLastModified && entry.lastModified) {
        const serverDate = new Date(serverLastModified);
        const cacheDate = new Date(entry.lastModified);
        return serverDate.getTime() <= cacheDate.getTime();
      }

      return false;
    } catch (error) {
      // If we can't check with server, assume cache is valid
      return true;
    }
  }
}

interface NetworkEntry {
  assetId: string;
  url: string;
  etag?: string;
  lastModified: string;
  size: number;
  priority: CachePriority;
}
```

## Cache Warming

### Proactive Cache Population

```typescript
class CacheWarmer {
  private warmingQueue: string[] = [];
  private isWarming = false;

  async warmCache(assetIds: string[], priority: CachePriority = 'normal'): Promise<void> {
    this.warmingQueue.push(...assetIds);

    if (!this.isWarming) {
      await this.processWarmingQueue(priority);
    }
  }

  private async processWarmingQueue(priority: CachePriority): Promise<void> {
    this.isWarming = true;

    try {
      while (this.warmingQueue.length > 0) {
        const batchSize = 5; // Load 5 assets at a time
        const batch = this.warmingQueue.splice(0, batchSize);

        const loadPromises = batch.map(async (assetId) => {
          try {
            const assetData = await assetLoader.loadAsset(assetId);
            await assetCacheManager.putAsset(assetId, assetData, priority);
          } catch (error) {
            console.warn(`Failed to warm cache for asset ${assetId}:`, error);
          }
        });

        await Promise.all(loadPromises);

        // Small delay to prevent overwhelming the system
        await new Promise(resolve => setTimeout(resolve, 10));
      }
    } finally {
      this.isWarming = false;
    }
  }
}

// Usage: Warm cache for level loading
const levelAssets = await getLevelAssetList('level1');
await cacheWarmer.warmCache(levelAssets, 'high');
```

## Cache Monitoring

### Performance Metrics

```typescript
class CacheMetrics {
  private metrics = new Map<string, CachePerformanceMetrics>();

  recordCacheAccess(cacheType: string, assetId: string, hit: boolean, accessTime: number): void {
    const metrics = this.metrics.get(cacheType) || {
      cacheType,
      totalAccesses: 0,
      hits: 0,
      misses: 0,
      totalAccessTime: 0,
      averageAccessTime: 0,
      minAccessTime: Number.MAX_VALUE,
      maxAccessTime: 0
    };

    metrics.totalAccesses++;
    if (hit) metrics.hits++;
    else metrics.misses++;

    metrics.totalAccessTime += accessTime;
    metrics.averageAccessTime = metrics.totalAccessTime / metrics.totalAccesses;
    metrics.minAccessTime = Math.min(metrics.minAccessTime, accessTime);
    metrics.maxAccessTime = Math.max(metrics.maxAccessTime, accessTime);

    this.metrics.set(cacheType, metrics);
  }

  getReport(): CachePerformanceReport {
    const reports = Array.from(this.metrics.values()).map(metrics => ({
      ...metrics,
      hitRate: metrics.hits / metrics.totalAccesses,
      missRate: metrics.misses / metrics.totalAccesses
    }));

    return {
      caches: reports,
      overallHitRate: this.calculateOverallHitRate(reports),
      totalMemoryUsage: this.getTotalMemoryUsage(),
      totalDiskUsage: this.getTotalDiskUsage()
    };
  }

  private calculateOverallHitRate(cacheReports: CachePerformanceMetrics[]): number {
    const totalHits = cacheReports.reduce((sum, r) => sum + r.hits, 0);
    const totalAccesses = cacheReports.reduce((sum, r) => sum + r.totalAccesses, 0);
    return totalAccesses > 0 ? totalHits / totalAccesses : 0;
  }
}

interface CachePerformanceMetrics {
  cacheType: string;
  totalAccesses: number;
  hits: number;
  misses: number;
  totalAccessTime: number;
  averageAccessTime: number;
  minAccessTime: number;
  maxAccessTime: number;
}
```

## Cache Policies

### Adaptive Cache Policies

```typescript
class AdaptiveCachePolicy {
  private accessPatterns = new Map<string, AccessPattern>();
  private policyUpdateInterval = 60000; // 1 minute

  constructor() {
    setInterval(() => this.updatePolicies(), this.policyUpdateInterval);
  }

  getEvictionPolicy(assetId: string): EvictionPolicy {
    const pattern = this.accessPatterns.get(assetId);

    if (!pattern) {
      return 'lru'; // Default policy
    }

    // Adaptive policy based on access patterns
    if (pattern.frequency > 10 && pattern.recency < 300000) { // 5 minutes
      return 'keep'; // Frequently and recently accessed
    }

    if (pattern.frequency > 5) {
      return 'lfu'; // Frequently accessed but not recently
    }

    if (pattern.recency < 600000) { // 10 minutes
      return 'lru'; // Recently accessed
    }

    return 'size'; // Old and infrequently accessed
  }

  recordAccess(assetId: string): void {
    const now = Date.now();
    const pattern = this.accessPatterns.get(assetId) || {
      assetId,
      firstAccess: now,
      lastAccess: now,
      accessCount: 0,
      frequency: 0
    };

    pattern.lastAccess = now;
    pattern.accessCount++;

    // Calculate frequency (accesses per minute)
    const timeSpan = (now - pattern.firstAccess) / 60000;
    pattern.frequency = pattern.accessCount / Math.max(timeSpan, 1);

    this.accessPatterns.set(assetId, pattern);
  }

  private updatePolicies(): void {
    // Clean up old access patterns
    const cutoffTime = Date.now() - 3600000; // 1 hour ago

    for (const [assetId, pattern] of this.accessPatterns) {
      if (pattern.lastAccess < cutoffTime) {
        this.accessPatterns.delete(assetId);
      }
    }
  }
}

interface AccessPattern {
  assetId: string;
  firstAccess: number;
  lastAccess: number;
  accessCount: number;
  frequency: number; // Accesses per minute
}

type EvictionPolicy = 'lru' | 'lfu' | 'size' | 'keep';
```

## Best Practices

### Cache Configuration

- **Size Allocation**: Allocate appropriate sizes for each cache level
- **Policy Selection**: Choose policies based on access patterns
- **Monitoring**: Regularly monitor cache performance
- **Invalidation**: Implement proper cache invalidation strategies

### Performance Optimization

- **Memory Limits**: Set reasonable memory limits to prevent bloat
- **Compression**: Use compression for L2 cache to maximize memory efficiency
- **Prioritization**: Prioritize critical assets for caching
- **Preloading**: Use cache warming for predictable asset loading

### Maintenance

- **Regular Cleanup**: Periodically clean up stale cache entries
- **Integrity Checks**: Verify cache integrity and repair corruption
- **Backup**: Maintain backups of critical cached assets
- **Versioning**: Handle cache invalidation for asset updates

## Future Enhancements

### Advanced Features

- **Predictive Caching**: AI-based prediction of asset access patterns
- **Distributed Caching**: Cache synchronization across multiple instances
- **Content Delivery**: Integration with CDN for asset delivery
- **Cache Analytics**: Detailed analytics and optimization recommendations
- **Machine Learning**: ML-based cache policy optimization
