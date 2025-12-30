import { createRouter } from "@tanstack/react-router";
import { AuthProvider } from "./contexts/auth-context";
import { routeTree } from "./routeTree.gen";

let routerInstance: ReturnType<typeof createRouter> | undefined;
export const getRouter = () => {
  if (routerInstance) {
    return routerInstance;
  }

  routerInstance = createRouter({
    routeTree,
    context: {},
    defaultPreload: "intent",
    Wrap: (props: { children: React.ReactNode }) => {
      return <AuthProvider>{props.children}</AuthProvider>;
    },
  });

  return routerInstance;
};
