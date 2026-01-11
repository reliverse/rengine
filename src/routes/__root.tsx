import {
  createRootRouteWithContext,
  Outlet,
  useLocation,
} from "@tanstack/react-router";
import { MainLayout } from "~/components/main-layout";
import { ToasterProvider } from "~/components/toaster";

// import { TanStackDevtools } from "@tanstack/react-devtools";
// import { TanStackRouterDevtoolsPanel } from "@tanstack/react-router-devtools";
// import TanStackQueryDevtools from "../integrations/tanstack-query/devtools";

interface MyRouterContext extends Record<string, never> {}

export const Route = createRootRouteWithContext<MyRouterContext>()({
  component: RootComponent,
  context: () => ({}),
});

function RootComponent() {
  const location = useLocation();

  const hideSidebarRoutes = ["/", "/editor", "/auth", "/account"];
  const showSidebar = !hideSidebarRoutes.includes(location.pathname);

  return (
    <div className="min-h-screen bg-background">
      <MainLayout showSidebar={showSidebar}>
        <Outlet />
      </MainLayout>
      <ToasterProvider />
      {/* {isDevelopment && (
        <TanStackDevtools
          config={{
            position: "bottom-right",
          }}
          plugins={[
            {
              name: "Tanstack Router",
              render: <TanStackRouterDevtoolsPanel />,
            },
            TanStackQueryDevtools,
          ]}
        />
      )} */}
    </div>
  );
}
