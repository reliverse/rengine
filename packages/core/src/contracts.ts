export type RengineSurface = "cli" | "wiki";

export type RenginePackageRole =
  | "control-plane-cli"
  | "docs-wiki"
  | "runtime-core";

export type RengineWorkspacePackage = {
  name: string;
  role: RenginePackageRole;
  surface: RengineSurface;
  private: boolean;
};

export type RengineControlPlaneFeature = "terminal" | "streams";

export type RengineControlPlaneManifest = {
  name: "rengine";
  version: string;
  mode: "headless-first-control-plane";
  features: readonly RengineControlPlaneFeature[];
  packages: readonly RengineWorkspacePackage[];
};
