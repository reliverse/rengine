import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "path";
import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "~": path.resolve(__dirname, "./src"),
    },
  },

  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          // Three.js core (separate from react-three)
          three: ["three"],
          // React Three Fiber and Drei
          "react-three": ["@react-three/fiber", "@react-three/drei"],
          // Tauri APIs
          tauri: ["@tauri-apps/api", "@tauri-apps/plugin-dialog", "@tauri-apps/plugin-opener"],
          // UI libraries
          ui: ["@radix-ui/react-dropdown-menu", "@radix-ui/react-slot", "lucide-react"],
          // State management and utilities
          utils: ["zustand", "immer", "clsx", "tailwind-merge", "class-variance-authority"],
          // Other dependencies
          vendor: ["react", "react-dom", "sonner", "next-themes"],
        },
      },
    },
    // Increase chunk size warning limit to 1MB for now
    chunkSizeWarningLimit: 1000,
    // Enable minification and source maps for better debugging
    minify: "esbuild",
    sourcemap: false,
    // Target modern browsers for better optimization
    target: "esnext",
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
});
