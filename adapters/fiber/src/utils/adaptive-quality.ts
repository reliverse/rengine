import * as THREE from "three";
import { performanceMonitor } from "./performance-monitor";

// Regex patterns for GPU detection
const INTEGRATED_GPU_REGEX = /intel|amd/i;
const DEDICATED_GPU_REGEX = /rx|radeon|geforce|rtx/i;
const HIGH_END_GPU_REGEX = /rtx|radeon.*rx|geforce.*gtx|geforce.*rtx/i;

// Quality levels for adaptive scaling
export const QualityLevel = {
  ULTRA: 0,
  HIGH: 1,
  MEDIUM: 2,
  LOW: 3,
  POTATO: 4,
} as const;

export type QualityLevelType = (typeof QualityLevel)[keyof typeof QualityLevel];

const QualityLevelNames: Record<QualityLevelType, string> = {
  [QualityLevel.ULTRA]: "ULTRA",
  [QualityLevel.HIGH]: "HIGH",
  [QualityLevel.MEDIUM]: "MEDIUM",
  [QualityLevel.LOW]: "LOW",
  [QualityLevel.POTATO]: "POTATO",
};

export interface QualitySettings {
  pixelRatio: number;
  shadowMapSize: number;
  antialias: boolean;
  anisotropy: number;
  textureSize: number;
  lodDistanceMultiplier: number;
  cullingDistanceMultiplier: number;
  particleCountMultiplier: number;
  postProcessingEnabled: boolean;
  reflectionQuality: number;
}

export interface AdaptiveQualityConfig {
  targetFps: number;
  minFps: number;
  maxFps: number;
  qualityCheckInterval: number; // ms
  stabilizationFrames: number;
  aggressiveDownscaling: boolean;
}

export class AdaptiveQualityManager {
  private currentQuality: QualityLevelType = QualityLevel.MEDIUM;
  private targetQuality: QualityLevelType = QualityLevel.MEDIUM;
  private renderer: THREE.WebGLRenderer | null = null;
  private lastQualityCheck = 0;
  private readonly fpsHistory: number[] = [];
  private stabilizationCounter = 0;

  private readonly config: AdaptiveQualityConfig = {
    targetFps: 60,
    minFps: 30,
    maxFps: 120,
    qualityCheckInterval: 5000, // Check every 5 seconds
    stabilizationFrames: 60, // Stabilize for 1 second at 60fps
    aggressiveDownscaling: false,
  };

  private readonly qualitySettings: Record<QualityLevelType, QualitySettings> =
    {
      [QualityLevel.ULTRA]: {
        pixelRatio: Math.min(window.devicePixelRatio, 2),
        shadowMapSize: 2048,
        antialias: true,
        anisotropy: 16,
        textureSize: 1.0,
        lodDistanceMultiplier: 1.0,
        cullingDistanceMultiplier: 1.2,
        particleCountMultiplier: 1.0,
        postProcessingEnabled: true,
        reflectionQuality: 1.0,
      },
      [QualityLevel.HIGH]: {
        pixelRatio: Math.min(window.devicePixelRatio, 1.5),
        shadowMapSize: 1024,
        antialias: window.devicePixelRatio <= 1,
        anisotropy: 8,
        textureSize: 0.875,
        lodDistanceMultiplier: 0.9,
        cullingDistanceMultiplier: 1.0,
        particleCountMultiplier: 0.9,
        postProcessingEnabled: true,
        reflectionQuality: 0.9,
      },
      [QualityLevel.MEDIUM]: {
        pixelRatio: 1,
        shadowMapSize: 512,
        antialias: false,
        anisotropy: 4,
        textureSize: 0.75,
        lodDistanceMultiplier: 0.8,
        cullingDistanceMultiplier: 0.8,
        particleCountMultiplier: 0.7,
        postProcessingEnabled: false,
        reflectionQuality: 0.7,
      },
      [QualityLevel.LOW]: {
        pixelRatio: 1,
        shadowMapSize: 256,
        antialias: false,
        anisotropy: 2,
        textureSize: 0.5,
        lodDistanceMultiplier: 0.6,
        cullingDistanceMultiplier: 0.6,
        particleCountMultiplier: 0.5,
        postProcessingEnabled: false,
        reflectionQuality: 0.5,
      },
      [QualityLevel.POTATO]: {
        pixelRatio: 0.75,
        shadowMapSize: 128,
        antialias: false,
        anisotropy: 1,
        textureSize: 0.25,
        lodDistanceMultiplier: 0.4,
        cullingDistanceMultiplier: 0.4,
        particleCountMultiplier: 0.25,
        postProcessingEnabled: false,
        reflectionQuality: 0.25,
      },
    };

  constructor(config?: Partial<AdaptiveQualityConfig>) {
    this.config = { ...this.config, ...config };
  }

  setRenderer(renderer: THREE.WebGLRenderer): void {
    this.renderer = renderer;
    this.applyQualitySettings();
  }

  // Call this every frame
  update(): void {
    const now = performance.now();
    if (now - this.lastQualityCheck >= this.config.qualityCheckInterval) {
      this.checkAndAdjustQuality();
      this.lastQualityCheck = now;
    }

    // Apply quality changes with stabilization
    if (this.currentQuality !== this.targetQuality) {
      if (this.stabilizationCounter >= this.config.stabilizationFrames) {
        this.setQuality(this.targetQuality);
        this.stabilizationCounter = 0;
      } else {
        this.stabilizationCounter++;
      }
    }
  }

  private checkAndAdjustQuality(): void {
    const metrics = performanceMonitor.getSnapshot();
    const currentFps = metrics.metrics.fps;

    // Maintain FPS history for stability
    this.fpsHistory.push(currentFps);
    if (this.fpsHistory.length > 10) {
      this.fpsHistory.shift();
    }

    const avgFps =
      this.fpsHistory.reduce((a, b) => a + b, 0) / this.fpsHistory.length;

    // Determine target quality based on performance
    let newTargetQuality = this.currentQuality;

    if (avgFps < this.config.minFps) {
      // Performance is too low, decrease quality
      newTargetQuality = Math.min(
        this.currentQuality + 1,
        QualityLevel.POTATO
      ) as QualityLevelType;

      if (
        this.config.aggressiveDownscaling &&
        avgFps < this.config.minFps * 0.5
      ) {
        // Very poor performance, aggressive downscaling
        newTargetQuality = Math.min(
          this.currentQuality + 2,
          QualityLevel.POTATO
        ) as QualityLevelType;
      }
    } else if (avgFps > this.config.targetFps + 10) {
      // Performance is good, try increasing quality
      newTargetQuality = Math.max(
        this.currentQuality - 1,
        QualityLevel.ULTRA
      ) as QualityLevelType;
    }

    if (newTargetQuality !== this.targetQuality) {
      this.targetQuality = newTargetQuality;
      this.stabilizationCounter = 0; // Reset stabilization counter
      console.log(
        `Adaptive Quality: Adjusting to ${QualityLevelNames[newTargetQuality]} (${avgFps.toFixed(1)} FPS)`
      );
    }
  }

  setQuality(level: QualityLevelType): void {
    if (!this.renderer) return;

    this.currentQuality = level;
    this.applyQualitySettings();

    // Dispatch event for other systems to react
    window.dispatchEvent(
      new CustomEvent("quality-changed", {
        detail: { level, settings: this.qualitySettings[level] },
      })
    );
  }

  private applyQualitySettings(): void {
    if (!this.renderer) return;

    const settings = this.qualitySettings[this.currentQuality];

    // Apply renderer settings
    this.renderer.setPixelRatio(settings.pixelRatio);

    // Apply shadow settings
    if (this.renderer.shadowMap) {
      this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
      // Note: Individual shadow map sizes would need to be set on lights
    }

    console.log(
      `Applied quality settings: ${QualityLevelNames[this.currentQuality]}`,
      settings
    );
  }

  getCurrentQuality(): QualityLevelType {
    return this.currentQuality;
  }

  getCurrentSettings(): QualitySettings {
    return this.qualitySettings[this.currentQuality];
  }

  // Manual quality override
  forceQuality(level: QualityLevelType): void {
    this.targetQuality = level;
    this.stabilizationCounter = this.config.stabilizationFrames; // Apply immediately
  }

  // Get recommended quality based on hardware detection
  static detectOptimalQuality(): QualityLevelType {
    const canvas = document.createElement("canvas");
    const gl = canvas.getContext("webgl2") || canvas.getContext("webgl");

    if (!gl) return QualityLevel.POTATO;

    const debugInfo = gl.getExtension("WEBGL_debug_renderer_info");
    const renderer = debugInfo
      ? gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL)
      : "";

    // Detect integrated vs dedicated GPU
    const isIntegrated =
      INTEGRATED_GPU_REGEX.test(renderer) &&
      !DEDICATED_GPU_REGEX.test(renderer);
    const isHighEnd = HIGH_END_GPU_REGEX.test(renderer);

    // Check memory
    const memory = (
      performance as Performance & { memory?: { jsHeapSizeLimit: number } }
    ).memory;
    const hasLowMemory = memory && memory.jsHeapSizeLimit < 500 * 1024 * 1024; // < 500MB

    if (hasLowMemory) return QualityLevel.LOW;
    if (isHighEnd) return QualityLevel.ULTRA;
    if (isIntegrated) return QualityLevel.MEDIUM;

    return QualityLevel.HIGH;
  }

  // Get performance info
  getPerformanceInfo() {
    return {
      currentQuality: QualityLevelNames[this.currentQuality],
      targetQuality: QualityLevelNames[this.targetQuality],
      fps: performanceMonitor.getSnapshot().metrics.fps,
      isStable: this.currentQuality === this.targetQuality,
      stabilizationProgress:
        this.stabilizationCounter / this.config.stabilizationFrames,
    };
  }
}

// Singleton instance
export const adaptiveQuality = new AdaptiveQualityManager();
