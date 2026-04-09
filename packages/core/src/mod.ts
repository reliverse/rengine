import type { RengineGameEngineManifest, RengineWorkspacePackage } from "./contracts";

export type {
  RengineGameEngineFeature,
  RengineGameEngineManifest,
  RenginePackageRole,
  RengineSurface,
  RengineWorkspacePackage,
} from "./contracts";
export type { RengineRuntimePlaceholder } from "./runtime";
export { createRengineRuntimePlaceholder } from "./runtime";
 
export const rengineWorkspacePackages: readonly RengineWorkspacePackage[] = [
  {
    name: "@reliverse/rengine",
    role: "game-engine-cli",
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

export const rengineGameEngineManifest: RengineGameEngineManifest = {
  name: "rengine",
  version: "0.0.1",
  mode: "headless-first-game-engine",
  features: ["terminal", "streams"],
  packages: rengineWorkspacePackages,
};
