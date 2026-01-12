import type * as THREE from "three";

// Performance monitoring utilities for the game engine
interface PerformanceMetrics {
  fps: number;
  frameTime: number;
  memoryUsage: number;
  drawCalls: number;
  triangles: number;
  geometries: number;
  textures: number;
  timestamp: number;
}

interface PerformanceSnapshot {
  metrics: PerformanceMetrics;
  sceneStats: {
    objects: number;
    lights: number;
    materials: number;
    geometries: number;
  };
  memoryStats: {
    geometries: number;
    materials: number;
    textures: number;
  };
}

class PerformanceMonitor {
  private metrics: PerformanceMetrics[] = [];
  private readonly maxMetricsHistory = 300; // Keep last 5 minutes at 60fps
  private lastFrameTime = 0;
  private frameCount = 0;
  private readonly fpsUpdateInterval = 1000; // Update FPS every second
  private lastFpsUpdate = 0;
  private currentFps = 0;

  // Renderer reference for monitoring
  private renderer: THREE.WebGLRenderer | null = null;

  setRenderer(renderer: THREE.WebGLRenderer) {
    this.renderer = renderer;
  }

  // Call this every frame
  update() {
    const now = performance.now();
    const deltaTime = now - this.lastFrameTime;

    if (this.lastFrameTime > 0) {
      this.frameCount++;

      // Update FPS every second
      if (now - this.lastFpsUpdate >= this.fpsUpdateInterval) {
        this.currentFps = (this.frameCount * 1000) / (now - this.lastFpsUpdate);
        this.frameCount = 0;
        this.lastFpsUpdate = now;
      }

      // Collect metrics
      const metrics: PerformanceMetrics = {
        fps: this.currentFps,
        frameTime: deltaTime,
        memoryUsage:
          (performance as { memory?: { usedJSHeapSize?: number } }).memory
            ?.usedJSHeapSize || 0,
        drawCalls: this.renderer?.info?.render?.calls || 0,
        triangles: this.renderer?.info?.render?.triangles || 0,
        geometries: this.renderer?.info?.memory?.geometries || 0,
        textures: this.renderer?.info?.memory?.textures || 0,
        timestamp: now,
      };

      this.metrics.push(metrics);

      // Keep only recent metrics
      if (this.metrics.length > this.maxMetricsHistory) {
        this.metrics.shift();
      }
    }

    this.lastFrameTime = now;
  }

  // Get current performance snapshot
  getSnapshot(): PerformanceSnapshot {
    const latestMetrics = this.metrics.at(-1) || {
      fps: 0,
      frameTime: 0,
      memoryUsage: 0,
      drawCalls: 0,
      triangles: 0,
      geometries: 0,
      textures: 0,
      timestamp: Date.now(),
    };

    return {
      metrics: latestMetrics,
      sceneStats: {
        objects: 0, // Will be filled by scene store
        lights: 0, // Will be filled by scene store
        materials: 0, // Will be filled by geometry pool
        geometries: 0, // Will be filled by geometry pool
      },
      memoryStats: {
        geometries: latestMetrics.geometries,
        materials: 0, // Will be filled by geometry pool
        textures: latestMetrics.textures,
      },
    };
  }

  // Get average metrics over the last N frames
  getAverageMetrics(frames = 60): Partial<PerformanceMetrics> {
    if (this.metrics.length === 0) return {};

    const startIndex = Math.max(0, this.metrics.length - frames);
    const recentMetrics = this.metrics.slice(startIndex);

    const averages = recentMetrics.reduce(
      (acc, metric) => ({
        fps: acc.fps + metric.fps,
        frameTime: acc.frameTime + metric.frameTime,
        memoryUsage: acc.memoryUsage + metric.memoryUsage,
        drawCalls: acc.drawCalls + metric.drawCalls,
        triangles: acc.triangles + metric.triangles,
        geometries: acc.geometries + metric.geometries,
        textures: acc.textures + metric.textures,
      }),
      {
        fps: 0,
        frameTime: 0,
        memoryUsage: 0,
        drawCalls: 0,
        triangles: 0,
        geometries: 0,
        textures: 0,
      }
    );

    const count = recentMetrics.length;
    return {
      fps: averages.fps / count,
      frameTime: averages.frameTime / count,
      memoryUsage: averages.memoryUsage / count,
      drawCalls: averages.drawCalls / count,
      triangles: averages.triangles / count,
      geometries: averages.geometries / count,
      textures: averages.textures / count,
    };
  }

  // Performance warnings
  getPerformanceWarnings(): string[] {
    const warnings: string[] = [];
    const avgMetrics = this.getAverageMetrics(60);

    // FPS warnings disabled TODO: Enable back when scene rendering is optimized
    // if (avgMetrics.fps && avgMetrics.fps < 30) {
    //   warnings.push(`Low FPS: ${avgMetrics.fps.toFixed(1)} (target: 60+)`);
    // }

    // Frame time warnings disabled TODO: Enable back when scene rendering is optimized
    // if (avgMetrics.frameTime && avgMetrics.frameTime > 16.67) {
    //   warnings.push(
    //     `High frame time: ${avgMetrics.frameTime.toFixed(2)}ms (target: <16.67ms)`
    //   );
    // }

    if (avgMetrics.memoryUsage && avgMetrics.memoryUsage > 100 * 1024 * 1024) {
      warnings.push(
        `High memory usage: ${(avgMetrics.memoryUsage / (1024 * 1024)).toFixed(1)}MB`
      );
    }

    if (avgMetrics.drawCalls && avgMetrics.drawCalls > 1000) {
      warnings.push(
        `High draw calls: ${avgMetrics.drawCalls} (consider instancing/batching)`
      );
    }

    if (avgMetrics.triangles && avgMetrics.triangles > 100_000) {
      warnings.push(
        `High triangle count: ${avgMetrics.triangles.toLocaleString()} (consider LOD/optimization)`
      );
    }

    return warnings;
  }

  // Memory usage formatter
  formatMemory(bytes: number): string {
    if (bytes === 0) return "0 B";

    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return `${(bytes / k ** i).toFixed(1)} ${sizes[i]}`;
  }

  // Reset monitoring data
  reset() {
    this.metrics = [];
    this.lastFrameTime = 0;
    this.frameCount = 0;
    this.lastFpsUpdate = 0;
    this.currentFps = 0;
  }
}

// Singleton instance
export const performanceMonitor = new PerformanceMonitor();
