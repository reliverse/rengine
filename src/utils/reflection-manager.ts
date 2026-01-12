import type * as THREE from "three";

// Global reflection texture accessor
let globalReflectionTexture: THREE.Texture | null = null;

export function getReflectionTexture(): THREE.Texture | null {
  return globalReflectionTexture;
}

export function setReflectionTexture(texture: THREE.Texture | null): void {
  globalReflectionTexture = texture;
}
