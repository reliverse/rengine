import { controlPlaneShell } from "./control-plane-shell";
import { appShell } from "./app-shell";
import { controlPlaneHome } from "./components/control-plane-home";
import { webStack } from "./lib/stack";
import { routes } from "./routes";

export { appShell } from "./app-shell";
export { controlPlaneHome } from "./components/control-plane-home";
export { webStack } from "./lib/stack";
export { routes } from "./routes";
export { controlPlaneShell } from "./control-plane-shell";

export const webAppScaffold = {
  name: "@repo/rese-web",
  surface: "web",
  purpose: "future browser-based control plane app",
  shell: controlPlaneShell,
  appShell,
  home: controlPlaneHome,
  routes,
  stack: webStack,
} as const;
