export type ReseSurface = "cli" | "web" | "wiki";

export type ResePackageRole =
  | "control-plane-cli"
  | "control-plane-web"
  | "docs-wiki"
  | "runtime-core";

export type ReseWorkspacePackage = {
  name: string;
  role: ResePackageRole;
  surface: ReseSurface;
  private: boolean;
};

export type ReseControlPlaneFeature = "terminal" | "streams";

export type ReseControlPlaneManifest = {
  name: "rese";
  version: string;
  mode: "headless-first-control-plane";
  features: readonly ReseControlPlaneFeature[];
  packages: readonly ReseWorkspacePackage[];
};
