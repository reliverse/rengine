import type { RengineControlPlaneManifest, RengineWorkspacePackage } from "./contracts";

export type {
  RengineControlPlaneFeature,
  RengineControlPlaneManifest,
  RenginePackageRole,
  RengineSurface,
  RengineWorkspacePackage,
} from "./contracts";
export type { RengineRuntimePlaceholder } from "./runtime";
export { createRengineRuntimePlaceholder } from "./runtime";

export const rengineWorkspacePackages: readonly RengineWorkspacePackage[] = [
  {
    name: "@reliverse/rengine",
    role: "control-plane-cli",
    surface: "cli",
    private: false,
  },
  {
    name: "@repo/rengine-wiki",
    role: "docs-wiki",
    surface: "wiki",
    private: true,
  },
  {
    name: "@reliverse/rengine-core",
    role: "runtime-core",
    surface: "cli",
    private: false,
  },
] as const;

export const rengineControlPlaneManifest: RengineControlPlaneManifest = {
  name: "rengine",
  version: "0.0.1",
  mode: "headless-first-control-plane",
  features: ["terminal", "streams"],
  packages: rengineWorkspacePackages,
};
