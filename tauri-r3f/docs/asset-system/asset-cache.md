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
interface CacheManager {
  getAsset: (assetId: string) => Promise<AssetData | null>;
  putAsset: (assetId: string, assetData: AssetData, priority?: CachePriority) => Promise<void>;
  evictAsset: (assetId: string) => Promise<void>;
  clearCache: () => Promise<void>;
}

function createAssetCacheManager(options: CacheOptions): CacheManager {
  const l1Cache = createMemoryCache(options.l1Size);
  const l2Cache = createCompressedCache(options.l2Size);
  const diskCache = createDiskCache(options.diskPath, options.diskSize);
  const networkCache = createNetworkCache(options.networkPath);

  const recordCacheHit = (cacheType: string, assetId: string) => {
    // Implementation for recording cache hits
    console.log(`Cache hit in ${cacheType}: ${assetId}`);
  };

  const recordCacheMiss = (assetId: string) => {
    // Implementation for recording cache misses
    console.log(`Cache miss: ${assetId}`);
  };

  const recordCacheStore = (assetId: string, size: number) => {
    // Implementation for recording cache stores
    console.log(`Stored asset ${assetId} (${size} bytes)`);
  };

  const getAsset = async (assetId: string): Promise<AssetData | null> => {
    // Try L1 cache first
    let asset = await l1Cache.get(assetId);
    if (asset) {
      recordCacheHit('l1', assetId);
      return asset;
    }

    // Try L2 cache
    asset = await l2Cache.get(assetId);
    if (asset) {
      recordCacheHit('l2', assetId);
      // Promote to L1
      await l1Cache.put(assetId, asset);
      return asset;
    }

    // Try disk cache
    asset = await diskCache.get(assetId);
    if (asset) {
      recordCacheHit('disk', assetId);
      // Promote to higher caches
      await l2Cache.put(assetId, asset);
      await l1Cache.put(assetId, asset);
      return asset;
    }

    // Try network cache for remote assets
    asset = await networkCache.get(assetId);
    if (asset) {
      recordCacheHit('network', assetId);
      // Store in other caches
      await diskCache.put(assetId, asset);
      await l2Cache.put(assetId, asset);
      await l1Cache.put(assetId, asset);
      return asset;
    }

    recordCacheMiss(assetId);
    return null;
  };

  const putAsset = async (assetId: string, assetData: AssetData, priority: CachePriority = 'normal'): Promise<void> => {
    // Store in all applicable caches
    const promises = [
      l1Cache.put(assetId, assetData, priority),
      l2Cache.put(assetId, assetData, priority),
      diskCache.put(assetId, assetData, priority)
    ];

    await Promise.all(promises);
    recordCacheStore(assetId, assetData.size || 0);
  };

  const evictAsset = async (assetId: string): Promise<void> => {
    await Promise.all([
      l1Cache.evict(assetId),
      l2Cache.evict(assetId),
      diskCache.evict(assetId),
      networkCache.evict(assetId)
    ]);
  };

  const clearCache = async (): Promise<void> => {
    await Promise.all([
      l1Cache.clear(),
      l2Cache.clear(),
      diskCache.clear(),
      networkCache.clear()
    ]);
  };

  return {
    getAsset,
    putAsset,
    evictAsset,
    clearCache
  };
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
function createMemoryCache(maxSize: number): Cache {
  const cache = new Map<string, CacheEntry>();
  const accessOrder = createDoublyLinkedList<string>();
  let size = 0;

  const calculateEntrySize = (assetData: AssetData): number => {
    // Estimate memory usage
    if (assetData instanceof ArrayBuffer) {
      return assetData.byteLength;
    }
    if (typeof assetData === 'string') {
      return assetData.length * 2; // UTF-16
    }
    // For complex objects, use a rough estimation
    return JSON.stringify(assetData).length * 2;
  };

  const evictLRU = async (): Promise<void> => {
    const lruId = accessOrder.getTail();
    if (lruId) {
      await evict(lruId);
    }
  };

  const get = async (assetId: string): Promise<AssetData | null> => {
    const entry = cache.get(assetId);
    if (!entry) return null;

    // Move to front (most recently used)
    accessOrder.moveToFront(assetId);
    entry.lastAccessed = Date.now();

    return entry.data;
  };

  const put = async (assetId: string, assetData: AssetData, priority: CachePriority = 'normal'): Promise<void> => {
    const entrySize = calculateEntrySize(assetData);

    // Check if we need to evict
    while (size + entrySize > maxSize) {
      await evictLRU();
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
    if (cache.has(assetId)) {
      const oldEntry = cache.get(assetId)!;
      size -= oldEntry.size;
      accessOrder.remove(assetId);
    }

    cache.set(assetId, entry);
    accessOrder.addToFront(assetId);
    size += entrySize;
  };

  const evict = async (assetId: string): Promise<void> => {
    const entry = cache.get(assetId);
    if (!entry) return;

    cache.delete(assetId);
    accessOrder.remove(assetId);
    size -= entry.size;
  };

  const clear = async (): Promise<void> => {
    cache.clear();
    accessOrder.clear();
    size = 0;
  };

  const getStats = (): CacheStats => ({
    size,
    maxSize,
    itemCount: cache.size,
    hitCount: 0, // Would need additional tracking
    missCount: 0,
    evictionCount: 0,
    hitRate: 0
  });

  return {
    get,
    put,
    evict,
    clear,
    getStats
  };
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
function createCompressedCache(maxSize: number): Cache {
  const cache = new Map<string, CompressedEntry>();
  const compressor = createLZ4Compressor();
  let size = 0;

  const serializeAsset = (assetData: AssetData): Uint8Array => {
    // Implementation for serializing assets
    if (typeof assetData === 'string') {
      return new TextEncoder().encode(assetData);
    }
    return new TextEncoder().encode(JSON.stringify(assetData));
  };

  const deserializeAsset = (data: Uint8Array, type: string): AssetData => {
    // Implementation for deserializing assets
    const decoded = new TextDecoder().decode(data);
    if (type === 'string') return decoded;
    return JSON.parse(decoded);
  };

  const getAssetType = (assetData: AssetData): string => {
    if (typeof assetData === 'string') return 'string';
    if (assetData instanceof ArrayBuffer) return 'ArrayBuffer';
    return 'object';
  };

  const evictToFit = async (neededSpace: number): Promise<void> => {
    // Sort entries by priority and access time
    const entries = Array.from(cache.entries()).sort((a, b) => {
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

      cache.delete(assetId);
      size -= entry.compressedSize;
      freedSpace += entry.compressedSize;
    }
  };

  const get = async (assetId: string): Promise<AssetData | null> => {
    const entry = cache.get(assetId);
    if (!entry) return null;

    // Decompress data
    const decompressed = await compressor.decompress(entry.compressedData);

    // Deserialize if needed
    return deserializeAsset(decompressed, entry.originalType);
  };

  const put = async (assetId: string, assetData: AssetData, priority: CachePriority = 'normal'): Promise<void> => {
    // Serialize asset
    const serialized = serializeAsset(assetData);
    const originalSize = serialized.length;

    // Compress data
    const compressed = await compressor.compress(serialized);
    const compressedSize = compressed.length;

    const entry: CompressedEntry = {
      compressedData: compressed,
      originalSize,
      compressedSize,
      originalType: getAssetType(assetData),
      priority,
      created: Date.now()
    };

    // Check size limits
    if (size + compressedSize > maxSize) {
      await evictToFit(compressedSize);
    }

    cache.set(assetId, entry);
    size += compressedSize;
  };

  const evict = async (assetId: string): Promise<void> => {
    const entry = cache.get(assetId);
    if (!entry) return;

    cache.delete(assetId);
    size -= entry.compressedSize;
  };

  const clear = async (): Promise<void> => {
    cache.clear();
    size = 0;
  };

  const getStats = (): CacheStats => ({
    size,
    maxSize,
    itemCount: cache.size,
    hitCount: 0,
    missCount: 0,
    evictionCount: 0,
    hitRate: 0
  });

  return {
    get,
    put,
    evict,
    clear,
    getStats
  };
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
function createDiskCache(cacheDir: string, maxSize: number): Cache {
  const indexFile = path.join(cacheDir, 'index.json');
  const index = new Map<string, DiskEntry>();
  let size = 0;

  const serializeAsset = (assetData: AssetData): Buffer => {
    // Implementation for serializing assets to disk
    if (typeof assetData === 'string') {
      return Buffer.from(assetData, 'utf-8');
    }
    return Buffer.from(JSON.stringify(assetData), 'utf-8');
  };

  const deserializeAsset = (data: Buffer, entry: DiskEntry): AssetData => {
    // Implementation for deserializing assets from disk
    const content = data.toString('utf-8');
    if (entry.assetType === 'string') return content;
    return JSON.parse(content);
  };

  const getAssetType = (assetData: AssetData): string => {
    if (typeof assetData === 'string') return 'string';
    if (assetData instanceof ArrayBuffer) return 'ArrayBuffer';
    return 'object';
  };

  const calculateHash = async (data: Buffer): Promise<string> => {
    // Simple hash implementation
    const crypto = await import('crypto');
    return crypto.createHash('md5').update(data).digest('hex');
  };

  const loadIndex = async (): Promise<void> => {
    try {
      const indexData = await fs.readFile(indexFile, 'utf-8');
      const parsedIndex = JSON.parse(indexData);

      // Clear and repopulate the index
      index.clear();
      Object.entries(parsedIndex).forEach(([key, value]) => {
        index.set(key, value as DiskEntry);
      });

      size = Array.from(index.values()).reduce((sum, entry) => sum + entry.size, 0);
    } catch (error) {
      // Index file doesn't exist or is corrupted
      index.clear();
      size = 0;
    }
  };

  const saveIndex = async (): Promise<void> => {
    const indexObject = Object.fromEntries(index);
    await fs.writeFile(indexFile, JSON.stringify(indexObject, null, 2));
  };

  const ensureCacheDirectory = async (): Promise<void> => {
    try {
      await fs.access(cacheDir);
    } catch {
      await fs.mkdir(cacheDir, { recursive: true });
    }
  };

  const evictToFit = async (neededSpace: number): Promise<void> => {
    // Sort by priority, then by access time
    const entries = Array.from(index.entries()).sort((a, b) => {
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

      await evict(assetId);
      freedSpace += entry.size;
    }
  };

  const generateFilename = (assetId: string): string => {
    // Use asset ID with safe filename characters
    return assetId.replace(/[^a-zA-Z0-9-_]/g, '_') + '.cache';
  };

  // Initialize
  (async () => {
    await loadIndex();
    await ensureCacheDirectory();
  })();

  const get = async (assetId: string): Promise<AssetData | null> => {
    const entry = index.get(assetId);
    if (!entry) return null;

    const filePath = path.join(cacheDir, entry.filename);

    try {
      const data = await fs.readFile(filePath);

      // Update access time
      entry.lastAccessed = Date.now();
      entry.accessCount++;

      await saveIndex();

      return deserializeAsset(data, entry);
    } catch (error) {
      // File corrupted or missing, remove from index
      index.delete(assetId);
      await saveIndex();
      return null;
    }
  };

  const put = async (assetId: string, assetData: AssetData, priority: CachePriority = 'normal'): Promise<void> => {
    const filename = generateFilename(assetId);
    const filePath = path.join(cacheDir, filename);

    // Serialize asset
    const serialized = serializeAsset(assetData);

    // Check if we need to evict
    const newSize = serialized.length;
    if (size + newSize > maxSize) {
      await evictToFit(newSize);
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
      assetType: getAssetType(assetData),
      hash: await calculateHash(serialized)
    };

    index.set(assetId, entry);
    size += newSize;

    await saveIndex();
  };

  const evict = async (assetId: string): Promise<void> => {
    const entry = index.get(assetId);
    if (!entry) return;

    // Remove file
    const filePath = path.join(cacheDir, entry.filename);
    try {
      await fs.unlink(filePath);
    } catch (error) {
      // File might already be gone
    }

    // Update index
    index.delete(assetId);
    size -= entry.size;
    await saveIndex();
  };

  const clear = async (): Promise<void> => {
    // Remove all cache files
    for (const [assetId] of index) {
      await evict(assetId);
    }
  };

  const getStats = (): CacheStats => ({
    size,
    maxSize,
    itemCount: index.size,
    hitCount: 0,
    missCount: 0,
    evictionCount: 0,
    hitRate: 0
  });

  return {
    get,
    put,
    evict,
    clear,
    getStats
  };
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
function createNetworkCache(cacheDir: string): Cache {
  const cache = new Map<string, NetworkEntry>();
  const httpClient = createHttpClient();

  const loadIndex = async (): Promise<void> => {
    try {
      const indexPath = path.join(cacheDir, 'network-index.json');
      const indexData = await fs.readFile(indexPath, 'utf-8');
      const parsedIndex = JSON.parse(indexData);

      cache.clear();
      Object.entries(parsedIndex).forEach(([key, value]) => {
        cache.set(key, value as NetworkEntry);
      });
    } catch (error) {
      // Index doesn't exist or is corrupted
      cache.clear();
    }
  };

  const saveIndex = async (): Promise<void> => {
    const indexPath = path.join(cacheDir, 'network-index.json');
    const indexObject = Object.fromEntries(cache);
    await fs.writeFile(indexPath, JSON.stringify(indexObject, null, 2));
  };

  const isRemoteAsset = (assetId: string): boolean => {
    // Check if asset ID represents a remote URL
    return assetId.startsWith('http://') || assetId.startsWith('https://');
  };

  const getAssetUrl = (assetId: string): string => {
    return assetId; // Asset ID is already a URL
  };

  const getEtag = async (assetId: string): Promise<string | undefined> => {
    try {
      const response = await httpClient.head(assetId);
      return response.headers['etag'];
    } catch {
      return undefined;
    }
  };

  const calculateSize = (assetData: AssetData): number => {
    if (assetData instanceof ArrayBuffer) {
      return assetData.byteLength;
    }
    if (typeof assetData === 'string') {
      return assetData.length * 2;
    }
    return JSON.stringify(assetData).length * 2;
  };

  const loadFromCache = async (entry: NetworkEntry): Promise<AssetData | null> => {
    try {
      const cachePath = path.join(cacheDir, `${entry.assetId.replace(/[^a-zA-Z0-9]/g, '_')}.cache`);
      const data = await fs.readFile(cachePath);
      return JSON.parse(data.toString('utf-8'));
    } catch {
      return null;
    }
  };

  const saveToCache = async (assetId: string, assetData: AssetData): Promise<void> => {
    const cachePath = path.join(cacheDir, `${assetId.replace(/[^a-zA-Z0-9]/g, '_')}.cache`);
    const serialized = JSON.stringify(assetData);
    await fs.writeFile(cachePath, serialized);
  };

  const isCacheValid = async (entry: NetworkEntry): Promise<boolean> => {
    try {
      // Check with server if cache is still valid
      const response = await httpClient.head(entry.url);

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
  };

  // Initialize
  (async () => {
    await loadIndex();
  })();

  const get = async (assetId: string): Promise<AssetData | null> => {
    const entry = cache.get(assetId);
    if (!entry) return null;

    // Check if cached version is still valid
    if (await isCacheValid(entry)) {
      return await loadFromCache(entry);
    }

    // Cache is stale, remove it
    cache.delete(assetId);
    await saveIndex();

    return null;
  };

  const put = async (assetId: string, assetData: AssetData, priority: CachePriority = 'normal'): Promise<void> => {
    // For network cache, we only cache remote assets
    if (!isRemoteAsset(assetId)) return;

    const entry: NetworkEntry = {
      assetId,
      url: getAssetUrl(assetId),
      etag: await getEtag(assetId),
      lastModified: new Date().toISOString(),
      size: calculateSize(assetData),
      priority
    };

    // Save to disk
    await saveToCache(assetId, assetData);

    cache.set(assetId, entry);
    await saveIndex();
  };

  const evict = async (assetId: string): Promise<void> => {
    const entry = cache.get(assetId);
    if (!entry) return;

    // Remove cache file
    const cachePath = path.join(cacheDir, `${assetId.replace(/[^a-zA-Z0-9]/g, '_')}.cache`);
    try {
      await fs.unlink(cachePath);
    } catch {
      // File might not exist
    }

    cache.delete(assetId);
    await saveIndex();
  };

  const clear = async (): Promise<void> => {
    for (const [assetId] of cache) {
      await evict(assetId);
    }
  };

  const getStats = (): CacheStats => ({
    size: Array.from(cache.values()).reduce((sum, entry) => sum + entry.size, 0),
    maxSize: Number.MAX_SAFE_INTEGER, // Network cache doesn't have a fixed size limit
    itemCount: cache.size,
    hitCount: 0,
    missCount: 0,
    evictionCount: 0,
    hitRate: 0
  });

  return {
    get,
    put,
    evict,
    clear,
    getStats
  };
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
function createCacheWarmer(assetLoader: AssetLoader, assetCacheManager: CacheManager) {
  const warmingQueue: string[] = [];
  let isWarming = false;

  const processWarmingQueue = async (priority: CachePriority): Promise<void> => {
    isWarming = true;

    try {
      while (warmingQueue.length > 0) {
        const batchSize = 5; // Load 5 assets at a time
        const batch = warmingQueue.splice(0, batchSize);

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
      isWarming = false;
    }
  };

  const warmCache = async (assetIds: string[], priority: CachePriority = 'normal'): Promise<void> => {
    warmingQueue.push(...assetIds);

    if (!isWarming) {
      await processWarmingQueue(priority);
    }
  };

  return {
    warmCache
  };
}

// Usage: Warm cache for level loading
const levelAssets = await getLevelAssetList('level1');
await cacheWarmer.warmCache(levelAssets, 'high');
```

## Cache Monitoring

### Performance Metrics

```typescript
function createCacheMetrics() {
  const metrics = new Map<string, CachePerformanceMetrics>();

  const recordCacheAccess = (cacheType: string, assetId: string, hit: boolean, accessTime: number): void => {
    const existingMetrics = metrics.get(cacheType) || {
      cacheType,
      totalAccesses: 0,
      hits: 0,
      misses: 0,
      totalAccessTime: 0,
      averageAccessTime: 0,
      minAccessTime: Number.MAX_VALUE,
      maxAccessTime: 0
    };

    existingMetrics.totalAccesses++;
    if (hit) existingMetrics.hits++;
    else existingMetrics.misses++;

    existingMetrics.totalAccessTime += accessTime;
    existingMetrics.averageAccessTime = existingMetrics.totalAccessTime / existingMetrics.totalAccesses;
    existingMetrics.minAccessTime = Math.min(existingMetrics.minAccessTime, accessTime);
    existingMetrics.maxAccessTime = Math.max(existingMetrics.maxAccessTime, accessTime);

    metrics.set(cacheType, existingMetrics);
  };

  const calculateOverallHitRate = (cacheReports: CachePerformanceMetrics[]): number => {
    const totalHits = cacheReports.reduce((sum, r) => sum + r.hits, 0);
    const totalAccesses = cacheReports.reduce((sum, r) => sum + r.totalAccesses, 0);
    return totalAccesses > 0 ? totalHits / totalAccesses : 0;
  };

  const getTotalMemoryUsage = (): number => {
    // Implementation for getting total memory usage
    return 0; // Placeholder
  };

  const getTotalDiskUsage = (): number => {
    // Implementation for getting total disk usage
    return 0; // Placeholder
  };

  const getReport = (): CachePerformanceReport => {
    const reports = Array.from(metrics.values()).map(metrics => ({
      ...metrics,
      hitRate: metrics.hits / metrics.totalAccesses,
      missRate: metrics.misses / metrics.totalAccesses
    }));

    return {
      caches: reports,
      overallHitRate: calculateOverallHitRate(reports),
      totalMemoryUsage: getTotalMemoryUsage(),
      totalDiskUsage: getTotalDiskUsage()
    };
  };

  return {
    recordCacheAccess,
    getReport
  };
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
function createAdaptiveCachePolicy(policyUpdateInterval = 60000) {
  const accessPatterns = new Map<string, AccessPattern>();

  const updatePolicies = (): void => {
    // Clean up old access patterns
    const cutoffTime = Date.now() - 3600000; // 1 hour ago

    for (const [assetId, pattern] of accessPatterns) {
      if (pattern.lastAccess < cutoffTime) {
        accessPatterns.delete(assetId);
      }
    }
  };

  // Set up periodic policy updates
  setInterval(updatePolicies, policyUpdateInterval);

  const getEvictionPolicy = (assetId: string): EvictionPolicy => {
    const pattern = accessPatterns.get(assetId);

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
  };

  const recordAccess = (assetId: string): void => {
    const now = Date.now();
    const pattern = accessPatterns.get(assetId) || {
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

    accessPatterns.set(assetId, pattern);
  };

  return {
    getEvictionPolicy,
    recordAccess
  };
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
