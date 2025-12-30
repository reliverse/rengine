import { ErrorBoundary } from "~/components/error-boundary";
import {
  type SidebarContext,
  UnifiedSidebar,
} from "~/components/unified-sidebar";

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
          {showSidebar && <UnifiedSidebar context={sidebarContext} />}

          <div className="flex-1 overflow-hidden">{children}</div>
        </div>
      </div>
    </ErrorBoundary>
  );
}
