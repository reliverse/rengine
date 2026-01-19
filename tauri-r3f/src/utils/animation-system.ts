import * as THREE from "three";

// Animation system for GLTF/GLB models
export interface AnimationClip {
  id: string;
  name: string;
  clip: THREE.AnimationClip;
  mixer: THREE.AnimationMixer | null;
  action?: THREE.AnimationAction;
  duration: number;
}

export interface AnimationState {
  clips: AnimationClip[];
  currentClipId: string | null;
  isPlaying: boolean;
  loopMode: "once" | "repeat" | "pingpong";
  speed: number;
  time: number;
}

export class AnimationController {
  private mixer: THREE.AnimationMixer | null = null;
  private clips: AnimationClip[] = [];
  private currentAction: THREE.AnimationAction | null = null;
  private readonly state: AnimationState;
  private readonly clock = new THREE.Clock();

  constructor() {
    this.state = {
      clips: [],
      currentClipId: null,
      isPlaying: false,
      loopMode: "repeat",
      speed: 1,
      time: 0,
    };
  }

  /**
   * Load animations from GLTF data
   */
  loadFromGLTF(gltf: any, rootObject: THREE.Object3D): void {
    if (!gltf.animations || gltf.animations.length === 0) {
      console.log("No animations found in GLTF");
      return;
    }

    // Create animation mixer for the root object
    const mixer = new THREE.AnimationMixer(rootObject);
    this.mixer = mixer;

    // Convert GLTF animations to our format
    this.clips = gltf.animations.map(
      (animation: THREE.AnimationClip, index: number) => {
        const clip: AnimationClip = {
          id: `animation_${index}`,
          name: animation.name || `Animation ${index + 1}`,
          clip: animation,
          mixer,
          duration: animation.duration,
        };

        // Create animation action
        const action = mixer.clipAction(animation);
        action.clampWhenFinished = true;
        clip.action = action;

        return clip;
      }
    );

    this.updateState();
  }

  /**
   * Play a specific animation
   */
  play(clipId: string): void {
    const clip = this.clips.find((c) => c.id === clipId);
    if (!clip?.action) return;

    // Stop current animation
    if (this.currentAction) {
      this.currentAction.stop();
    }

    // Start new animation
    this.currentAction = clip.action;
    this.currentAction.reset();

    // Set loop mode
    switch (this.state.loopMode) {
      case "once":
        this.currentAction.setLoop(THREE.LoopOnce, 1);
        break;
      case "repeat":
        this.currentAction.setLoop(THREE.LoopRepeat, Number.POSITIVE_INFINITY);
        break;
      case "pingpong":
        this.currentAction.setLoop(
          THREE.LoopPingPong,
          Number.POSITIVE_INFINITY
        );
        break;
    }

    this.currentAction.setEffectiveTimeScale(this.state.speed);
    this.currentAction.play();

    this.state.currentClipId = clipId;
    this.state.isPlaying = true;
  }

  /**
   * Pause the current animation
   */
  pause(): void {
    if (this.currentAction) {
      this.currentAction.paused = true;
      this.state.isPlaying = false;
    }
  }

  /**
   * Resume the current animation
   */
  resume(): void {
    if (this.currentAction) {
      this.currentAction.paused = false;
      this.state.isPlaying = true;
    }
  }

  /**
   * Stop the current animation
   */
  stop(): void {
    if (this.currentAction) {
      this.currentAction.stop();
      this.currentAction = null;
    }
    this.state.currentClipId = null;
    this.state.isPlaying = false;
    this.state.time = 0;
  }

  /**
   * Set animation time
   */
  setTime(time: number): void {
    this.state.time = Math.max(
      0,
      Math.min(time, this.getCurrentClipDuration())
    );
    if (this.currentAction) {
      this.currentAction.time = this.state.time;
    }
  }

  /**
   * Set playback speed
   */
  setSpeed(speed: number): void {
    this.state.speed = speed;
    if (this.currentAction) {
      this.currentAction.setEffectiveTimeScale(speed);
    }
  }

  /**
   * Set loop mode
   */
  setLoopMode(mode: "once" | "repeat" | "pingpong"): void {
    this.state.loopMode = mode;
    if (this.currentAction) {
      switch (mode) {
        case "once":
          this.currentAction.setLoop(THREE.LoopOnce, 1);
          break;
        case "repeat":
          this.currentAction.setLoop(
            THREE.LoopRepeat,
            Number.POSITIVE_INFINITY
          );
          break;
        case "pingpong":
          this.currentAction.setLoop(
            THREE.LoopPingPong,
            Number.POSITIVE_INFINITY
          );
          break;
      }
    }
  }

  /**
   * Update animation state (call this in render loop)
   */
  update(): void {
    if (this.mixer && this.state.isPlaying) {
      const delta = this.clock.getDelta();
      this.mixer.update(delta);
      this.state.time = this.currentAction?.time || 0;
    }
  }

  /**
   * Get current animation state
   */
  getState(): AnimationState {
    this.updateState();
    return { ...this.state };
  }

  /**
   * Get available animation clips
   */
  getClips(): AnimationClip[] {
    return [...this.clips];
  }

  /**
   * Get current clip duration
   */
  getCurrentClipDuration(): number {
    const clip = this.clips.find((c) => c.id === this.state.currentClipId);
    return clip?.duration || 0;
  }

  /**
   * Check if animations are available
   */
  hasAnimations(): boolean {
    return this.clips.length > 0;
  }

  /**
   * Clean up resources
   */
  dispose(): void {
    this.stop();
    if (this.mixer) {
      this.mixer.stopAllAction();
      // Note: THREE.AnimationMixer doesn't have a dispose method
    }
    this.clips = [];
  }

  private updateState(): void {
    this.state.clips = this.clips.map((clip) => ({
      ...clip,
      // Don't include mixer and action in state to avoid circular references
      mixer: null,
      action: undefined,
    }));
  }
}

/**
 * Extract animations from GLTF data and create animation controller
 */
export function createAnimationController(
  gltf: any,
  rootObject: THREE.Object3D
): AnimationController {
  const controller = new AnimationController();
  controller.loadFromGLTF(gltf, rootObject);
  return controller;
}

/**
 * Check if GLTF has animations
 */
export function hasAnimations(gltf: any): boolean {
  return gltf.animations && gltf.animations.length > 0;
}

/**
 * Get animation names from GLTF
 */
export function getAnimationNames(gltf: any): string[] {
  if (!gltf.animations) return [];
  return gltf.animations.map(
    (anim: THREE.AnimationClip) => anim.name || "Unnamed"
  );
}
