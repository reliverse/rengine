import {
  createRootRouteWithContext,
  Outlet,
  useLocation,
} from "@tanstack/react-router";
import { MainLayout } from "~/components/main-layout";
import { ToasterProvider } from "~/components/toaster";
import { WelcomeScreen } from "~/components/welcome-screen";

// import { TanStackDevtools } from "@tanstack/react-devtools";
// import { TanStackRouterDevtoolsPanel } from "@tanstack/react-router-devtools";
// import TanStackQueryDevtools from "../integrations/tanstack-query/devtools";

interface MyRouterContext extends Record<string, never> {
  // Context is empty since we're not using TanStack Query
}

export const Route = createRootRouteWithContext<MyRouterContext>()({
  component: RootComponent,
  context: () => ({}),
});

// Simplified component for Tauri/Vite setup (no SSR)
function RootComponent() {
  const location = useLocation();
  // Only show devtools in development to prevent performance issues
  // const isDevelopment = import.meta.env.DEV;

  // Hide sidebar for specific routes (RengineEditor handles its own layout)
  const hideSidebarRoutes = ["/", "/editor", "/settings", "/auth", "/account"];
  const showSidebar = !hideSidebarRoutes.includes(location.pathname);

  // For the home route, render WelcomeScreen full-screen without MainLayout
  if (location.pathname === "/") {
    return (
      <div className="min-h-screen bg-background">
        <WelcomeScreen />
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
