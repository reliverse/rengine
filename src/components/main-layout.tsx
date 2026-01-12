import { ErrorBoundary } from "~/components/error-boundary";
import { StatusBar } from "~/components/status-bar";
import {
  type SidebarContext,
  RightSidebar,
} from "~/components/right-sidebar";

interface MainLayoutProps {
  children: React.ReactNode;
  showSidebar?: boolean;
  sidebarContext?: SidebarContext;
}

export function MainLayout({
  children,
  showSidebar = true,
  sidebarContext = "scene",
}: MainLayoutProps) {
  return (
    <ErrorBoundary>
      <div className="flex h-screen flex-col">
        <div className="flex flex-1 overflow-hidden">
          {showSidebar && <RightSidebar context={sidebarContext} />}

          <div className="flex-1 overflow-hidden">{children}</div>
        </div>

        <StatusBar />
      </div>
    </ErrorBoundary>
  );
}
