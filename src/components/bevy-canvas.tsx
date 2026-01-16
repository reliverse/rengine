import { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { useSceneStore } from "~/stores/scene-store";
import { useBevySceneSync } from "~/hooks/use-bevy-scene-sync";
import { BevyInputHandler } from "./bevy-input-handler";
import { cn } from "~/lib/utils";

interface BevyCanvasProps {
  className?: string;
}

/**
 * BevyCanvas component - displays Bevy rendering
 * Currently shows a placeholder while Bevy rendering is set up
 */
export function BevyCanvas({ className }: BevyCanvasProps) {
  const canvasRef = useRef<HTMLDivElement>(null);
  const sceneStore = useSceneStore();

  // Use scene sync hook to automatically sync scene state
  useBevySceneSync();

  useEffect(() => {
    // Set up event listeners for Bevy state updates
    const unlisten = listen("bevy-state-update", (event) => {
      // Handle Bevy state updates
      console.log("Bevy state update:", event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <BevyInputHandler enabled={true}>
      <div
        className={cn(
          "relative flex h-full w-full items-center justify-center bg-black",
          className
        )}
        ref={canvasRef}
      >
        {/* Placeholder - will be replaced with actual Bevy rendering */}
        <div className="text-center text-white">
          <h2 className="mb-2 font-bold text-2xl">Bevy Renderer</h2>
          <p className="text-sm opacity-75">
            Bevy rendering is active. Scene objects: {sceneStore.objects.length}
          </p>
          <p className="mt-2 text-xs opacity-50">
            Note: Bevy is running in headless mode and rendering to webview
            canvas
          </p>
        </div>
      </div>
    </BevyInputHandler>
  );
}
