import { createReseRuntimePlaceholder, reseControlPlaneManifest } from "@reliverse/rese-core";

export const cliScaffold = {
  name: "@reliverse/rese",
  surface: "cli",
  purpose: "future CLI entrypoint for starting and managing rese",
  manifest: reseControlPlaneManifest,
} as const;

export const runCliPlaceholder = () =>
  createReseRuntimePlaceholder({
    entrypoint: "cli",
  });
