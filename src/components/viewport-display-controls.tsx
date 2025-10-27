import { Html } from "@react-three/drei";
import { useThree } from "@react-three/fiber";
import { useCallback, useEffect, useState } from "react";

interface ViewportDisplayControlsProps {
  children: React.ReactNode;
}

export default function ViewportDisplayControls({ children }: ViewportDisplayControlsProps) {
  const { scene } = useThree();

  const [isGameMode, setIsGameMode] = useState(false);
  const [isRealTime, setIsRealTime] = useState(true);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [showStats, setShowStats] = useState(false);

  // Toggle game mode (G key)
  const toggleGameMode = useCallback(() => {
    setIsGameMode((prev) => !prev);
  }, []);

  // Toggle real-time playback (Ctrl+R)
  const toggleRealTime = useCallback(() => {
    setIsRealTime((prev) => !prev);
  }, []);

  // Toggle fullscreen (F11)
  const toggleFullscreen = useCallback(() => {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen();
      setIsFullscreen(true);
    } else {
      document.exitFullscreen();
      setIsFullscreen(false);
    }
  }, []);

  // Toggle stats display
  const toggleStats = useCallback(() => {
    setShowStats((prev) => !prev);
  }, []);

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const key = event.key.toLowerCase();

      if (key === "g") {
        event.preventDefault();
        toggleGameMode();
      } else if (key === "f11") {
        event.preventDefault();
        toggleFullscreen();
      } else if (key === "r" && event.ctrlKey) {
        event.preventDefault();
        toggleRealTime();
      } else if (key === "f3") {
        event.preventDefault();
        toggleStats();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [toggleGameMode, toggleFullscreen, toggleRealTime, toggleStats]);

  // Handle fullscreen change events
  useEffect(() => {
    const handleFullscreenChange = () => {
      setIsFullscreen(!!document.fullscreenElement);
    };

    document.addEventListener("fullscreenchange", handleFullscreenChange);
    return () => document.removeEventListener("fullscreenchange", handleFullscreenChange);
  }, []);

  // Apply game mode effects
  useEffect(() => {
    if (isGameMode) {
      // Hide editor-only elements
      scene.traverse((child: any) => {
        if (child.userData?.isEditorOnly) {
          child.visible = false;
        }
      });
    } else {
      // Show all elements
      scene.traverse((child: any) => {
        if (child.userData?.isEditorOnly) {
          child.visible = true;
        }
      });
    }
  }, [isGameMode, scene]);

  return (
    <>
      {children}
      <Html>
        {/* Game mode overlay */}
        {isGameMode && (
          <div className="absolute top-4 left-4 bg-black bg-opacity-75 text-white px-3 py-1 rounded text-sm font-mono">
            GAME MODE
          </div>
        )}

        {/* Real-time indicator */}
        {!isRealTime && (
          <div className="absolute top-4 right-4 bg-yellow-600 bg-opacity-75 text-white px-3 py-1 rounded text-sm font-mono">
            PAUSED
          </div>
        )}

        {/* Stats display */}
        {showStats && (
          <div className="absolute bottom-4 left-4 bg-black bg-opacity-75 text-white p-2 rounded text-xs font-mono">
            <div>FPS: --</div>
            <div>Objects: {scene.children.length}</div>
            <div>Mode: {isGameMode ? "Game" : "Editor"}</div>
          </div>
        )}

        {/* Fullscreen indicator */}
        {isFullscreen && (
          <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-black bg-opacity-50 text-white px-4 py-2 rounded text-sm">
            Press F11 to exit fullscreen
          </div>
        )}
      </Html>
    </>
  );
}
