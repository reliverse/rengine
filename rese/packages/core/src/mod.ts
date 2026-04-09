import type { ReseControlPlaneManifest, ReseWorkspacePackage } from "./contracts";

export type {
  ReseControlPlaneFeature,
  ReseControlPlaneManifest,
  ResePackageRole,
  ReseSurface,
  ReseWorkspacePackage,
} from "./contracts";
export type { ReseRuntimePlaceholder } from "./runtime";
export { createReseRuntimePlaceholder } from "./runtime";

export const reseWorkspacePackages: readonly ReseWorkspacePackage[] = [
  {
    name: "@reliverse/rese",
    role: "control-plane-cli",
    surface: "cli",
    private: false,
  },
  {
    name: "@repo/rese-web",
    role: "control-plane-web",
    surface: "web",
    private: true,
  },
  {
    name: "@repo/rese-wiki",
    role: "docs-wiki",
    surface: "wiki",
    private: true,
  },
  {
    name: "@reliverse/rese-core",
    role: "runtime-core",
    surface: "cli",
    private: false,
  },
] as const;

export const reseControlPlaneManifest: ReseControlPlaneManifest = {
  name: "rese",
  version: "0.0.1",
  mode: "headless-first-control-plane",
  features: ["terminal", "streams"],
  packages: reseWorkspacePackages,
};
