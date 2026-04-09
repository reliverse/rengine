import { createRengineRuntimePlaceholder, rengineGameEngineManifest } from "@reliverse/rengine-core";
 
export const cliScaffold = {
  name: "@reliverse/rengine",
  surface: "cli",
  purpose: "future CLI entrypoint for starting and managing rengine",
  manifest: rengineGameEngineManifest,
} as const;

export const runCliPlaceholder = () =>
  createRengineRuntimePlaceholder({
    entrypoint: "cli",
  });
