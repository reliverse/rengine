import { reseControlPlaneManifest } from "@reliverse/rese-core";

export const controlPlaneShell = {
  appName: "Rese Control Plane",
  appSlug: "rese-control-plane",
  mode: reseControlPlaneManifest.mode,
  routes: ["/terminal", "/streams"],
  ui: {
    designSystem: "shadcn/ui",
    layout: "sidebar-with-topbar",
  },
} as const;
