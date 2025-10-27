import { invoke } from "@tauri-apps/api/core";
import { lazy, Suspense, useEffect } from "react";
import ErrorBoundary from "~/components/error-boundary";
import Toolbar from "~/components/navigations/toolbar";
import { ThemeProvider, useTheme } from "~/components/providers/theme-provider";
import { Toaster } from "~/components/ui/sonner";
import { toast } from "~/lib/toast";
import { useEditorStore } from "~/store/editor-store";
import { MapData } from "~/types/map";
import "./index.css";

// Lazy load the heavy components
const MapCanvas = lazy(() => import("~/components/map-canvas"));
const PropertyPanel = lazy(() => import("~/components/navigations/property-panel"));
const KeyboardShortcutsHelp = lazy(() => import("~/components/keyboard-shortcuts-help"));

function AppContent() {
  const { setMapData, mapData } = useEditorStore();
  const { isThemeReady } = useTheme();

  useEffect(() => {
    // Initialize with a new map
    const initializeMap = async () => {
      try {
        const mapData = await invoke<MapData>("create_new_map");
        setMapData(mapData);
        // toast.success("Map editor initialized successfully!");
      } catch (error) {
        console.error("Failed to create new map:", error);
        toast.error("Failed to initialize map editor");
      }
    };

    initializeMap();
  }, []);

  // Show loading screen until theme and map data are ready
  if (!isThemeReady || !mapData) {
    return (
      <div className="h-screen flex flex-col bg-gray-900">
        <div className="flex-1 flex items-center justify-center">
          <div className="text-white text-lg">Initializing Map Editor...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="h-screen flex flex-col bg-gray-900">
      <Toolbar />
      <div className="flex-1 flex">
        <div className="flex-1">
          <ErrorBoundary>
            <Suspense
              fallback={
                <div className="w-full h-full bg-gray-900 flex items-center justify-center">
                  <div className="text-white text-lg">Loading 3D Editor...</div>
                </div>
              }
            >
              <MapCanvas />
            </Suspense>
          </ErrorBoundary>
        </div>
        <Suspense
          fallback={
            <div className="w-80 bg-gray-800 border-l border-gray-700 p-4 flex items-center justify-center">
              <div className="text-white">Loading Properties...</div>
            </div>
          }
        >
          <PropertyPanel />
        </Suspense>
      </div>
      <Toaster />
      <Suspense fallback={null}>
        <KeyboardShortcutsHelp />
      </Suspense>
    </div>
  );
}

function App() {
  return (
    <ThemeProvider>
      <AppContent />
    </ThemeProvider>
  );
}

export default App;
