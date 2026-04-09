export type RengineRuntimePlaceholder = {
  name: "rengine";
  mode: "headless-first-control-plane";
  entrypoint: "cli";
};

export const createRengineRuntimePlaceholder = (
  input: Pick<RengineRuntimePlaceholder, "entrypoint">,
): RengineRuntimePlaceholder => ({
  name: "rengine",
  mode: "headless-first-control-plane",
  entrypoint: input.entrypoint,
});
