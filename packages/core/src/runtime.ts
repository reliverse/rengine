export type RengineRuntimePlaceholder = {
  name: "rengine";
  mode: "headless-first-game-engine";
  entrypoint: "cli" | "server";
};

export const createRengineRuntimePlaceholder = (
  input: Pick<RengineRuntimePlaceholder, "entrypoint">,
): RengineRuntimePlaceholder => ({
  name: "rengine",
  mode: "headless-first-game-engine",
  entrypoint: input.entrypoint,
});
