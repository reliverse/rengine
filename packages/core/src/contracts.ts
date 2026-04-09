export type RengineSurface = "cli" | "wiki";

export type RenginePackageRole =
  | "game-engine-cli"
  | "docs-wiki"
  | "runtime-core";

export type RengineWorkspacePackage = {
  name: string;
  role: RenginePackageRole;
  surface: RengineSurface;
  private: boolean;
};

export type RengineGameEngineFeature = "terminal" | "streams";

export type RengineGameEngineManifest = {
  name: "rengine";
  version: string;
  mode: "headless-first-game-engine";
  features: readonly RengineGameEngineFeature[];
  packages: readonly RengineWorkspacePackage[];
};
