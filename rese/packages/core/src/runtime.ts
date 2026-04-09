export type ReseRuntimePlaceholder = {
  name: "rese";
  mode: "headless-first-control-plane";
  entrypoint: "cli" | "server";
};

export const createReseRuntimePlaceholder = (
  input: Pick<ReseRuntimePlaceholder, "entrypoint">,
): ReseRuntimePlaceholder => ({
  name: "rese",
  mode: "headless-first-control-plane",
  entrypoint: input.entrypoint,
});
