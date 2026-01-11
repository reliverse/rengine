import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import { tanstackRouter } from "@tanstack/router-plugin/vite";
import viteReact from "@vitejs/plugin-react";
import type { Plugin } from "vite";
import { defineConfig } from "vite";
import viteTsConfigPaths from "vite-tsconfig-paths";

const host = process.env.TAURI_DEV_HOST;
const PROXY_PATH_REGEX = /^\/api\/proxy/;
const FILE_EXTENSION_REGEX = /\.[^/.]+$/;
const IMAGE_EXTENSIONS_REGEX = /\.(png|jpe?g|svg|gif|tiff|bmp|ico)$/i;
const FONT_EXTENSIONS_REGEX = /\.(woff2?|eot|ttf|otf)$/i;

// Plugin to handle API routes in development
const apiPlugin = (): Plugin => ({
  name: "api-routes",
  configureServer(server) {
    server.middlewares.use("/api/server/status", async (_req, res) => {
      const { getServerStatus } = await import(
        path.resolve(
          path.dirname(fileURLToPath(import.meta.url)),
          "./src/utils/server-status"
        )
      );
      const status = await getServerStatus();
      res.setHeader("Content-Type", "application/json");
      res.end(JSON.stringify(status));
    });
  },
});

// https://vitejs.dev/config/
const config = defineConfig({
  plugins: [
    tanstackRouter({
      target: "react",
      autoCodeSplitting: true,
    }),
    apiPlugin(),
    // devtools(), // import { devtools } from "@tanstack/devtools-vite";
    viteTsConfigPaths({
      // https://github.com/aleclarson/vite-tsconfig-paths#readme
      // this is the plugin that enables path aliases
      projects: ["./tsconfig.json"],
      logFile: false,
    }),
    tailwindcss(),
    viteReact({
      // babel: {
      // 	plugins: ["babel-plugin-react-compiler"],
      // },
    }),
  ],

  // Enable experimental features for better performance
  experimental: {
    renderBuiltUrl(_filename: string, { type }: { type: "public" | "asset" }) {
      // Optimize asset URLs for better caching
      if (type === "asset") {
        return { relative: true };
      }
      return { relative: false };
    },
  },

  // viteTsConfigPaths handles the path aliases from tsconfig.json
  // but it does not supports CSS, so we using the following to enable it
  resolve: {
    alias: {
      "~": path.resolve(path.dirname(fileURLToPath(import.meta.url)), "./src"),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host ?? false,
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
    proxy: {
      // Proxy API requests to avoid CORS issues in development
      "/api/proxy": {
        target: "https://api.rengine.com",
        changeOrigin: true,
        rewrite: (path: string) => path.replace(PROXY_PATH_REGEX, ""),
        configure: (proxy, _options) => {
          proxy.on("error", (err, _req, _res) => {
            console.error("Proxy error:", err);
          });
        },
      },
    },
  },

  // Additional build options
  esbuild: {
    target: "es2020",
    legalComments: "none",
    minifyIdentifiers: true,
    minifySyntax: true,
    minifyWhitespace: true,
    treeShaking: true,
  },

  // Optimize CSS
  css: {
    devSourcemap: false,
    modules: {
      localsConvention: "camelCaseOnly",
    },
  },

  // Bundle optimization with aggressive code splitting
  build: {
    rollupOptions: {
      output: {
        manualChunks: (id) => {
          // Vendor chunks
          if (id.includes("node_modules")) {
            // React ecosystem
            if (id.includes("react") || id.includes("jsx-runtime")) {
              return "react-vendor";
            }

            // Three.js ecosystem
            if (id.includes("three") || id.includes("@react-three")) {
              return "three-vendor";
            }

            // UI libraries
            if (
              id.includes("@radix-ui") ||
              id.includes("@base-ui") ||
              id.includes("lucide-react")
            ) {
              return "ui-vendor";
            }

            // State management
            if (
              id.includes("zustand") ||
              id.includes("@tanstack/react-query") ||
              id.includes("@tanstack/react-router")
            ) {
              return "state-vendor";
            }

            // Utilities
            if (
              id.includes("lodash") ||
              id.includes("date-fns") ||
              id.includes("color2k") ||
              id.includes("clsx")
            ) {
              return "utils-vendor";
            }

            // Large specialized libraries
            if (id.includes("html-to-image") || id.includes("react-dropzone")) {
              return "media-vendor";
            }

            // Everything else goes to vendor
            return "vendor";
          }

          // Application chunks
          if (id.includes("src/components/scene")) {
            return "scene-components";
          }

          if (id.includes("src/utils")) {
            return "utils-app";
          }

          if (id.includes("src/hooks")) {
            return "hooks-app";
          }

          if (id.includes("src/stores")) {
            return "stores-app";
          }

          // Default chunk
          return "app";
        },

        // Optimize chunk file names for caching
        chunkFileNames: (chunkInfo) => {
          const facadeModuleId = chunkInfo.facadeModuleId
            ? chunkInfo.facadeModuleId
                .split("/")
                .pop()
                ?.replace(FILE_EXTENSION_REGEX, "") || "chunk"
            : "chunk";
          return `assets/${facadeModuleId}-[hash].js`;
        },

        // Optimize asset names
        assetFileNames: (assetInfo) => {
          const info = assetInfo.name?.split(".") || [];
          const ext = info.at(-1);
          if (IMAGE_EXTENSIONS_REGEX.test(assetInfo.name || "")) {
            return "assets/images/[name]-[hash][extname]";
          }
          if (FONT_EXTENSIONS_REGEX.test(assetInfo.name || "")) {
            return "assets/fonts/[name]-[hash][extname]";
          }
          return `assets/${ext}/[name]-[hash][extname]`;
        },
      },

      // External dependencies that shouldn't be bundled
      external: [],

      // Optimize tree shaking
      treeshake: {
        moduleSideEffects: false,
        propertyReadSideEffects: false,
        tryCatchDeoptimization: false,
      },
    },

    // Enable source maps for better debugging in production
    sourcemap: false,

    // Use Terser for better minification (more aggressive than esbuild)
    minify: "terser",

    // Optimize chunk size limits
    chunkSizeWarningLimit: 1000,

    // Enable CSS code splitting
    cssCodeSplit: true,

    // Target modern browsers for better optimization
    target: "es2020",

    // Optimize dependencies
    commonjsOptions: {
      include: [/node_modules/],
    },

    // Optimize build performance
    reportCompressedSize: false, // Skip compression size reporting for faster builds
    emptyOutDir: true,
  },
});

export default config;
