/// <reference types="vite/client" />

// Extend JSX namespace for React Three Fiber
import "@react-three/fiber";

declare global {
  // React Three Fiber automatically extends JSX.IntrinsicElements

  interface Window {
    __TAURI__?: any;
  }
}
