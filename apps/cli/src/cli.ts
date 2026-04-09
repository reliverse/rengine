import { createRengineRuntimePlaceholder, rengineControlPlaneManifest } from "@reliverse/rengine-core";

export const cliScaffold = {
  name: "@reliverse/rengine",
  surface: "cli",
  purpose: "future CLI entrypoint for starting and managing rengine",
  manifest: rengineControlPlaneManifest,
} as const;

export const runCliPlaceholder = () =>
  createRengineRuntimePlaceholder({
    entrypoint: "cli",
  });
