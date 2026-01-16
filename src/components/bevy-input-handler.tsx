import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

interface BevyInputHandlerProps {
  enabled: boolean;
  children: React.ReactNode;
}

/**
 * Component that captures input events and forwards them to Bevy
 */
export function BevyInputHandler({ enabled, children }: BevyInputHandlerProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!(enabled && containerRef.current)) return;

    const container = containerRef.current;

    const sendInputEvent = async (
      event: string,
      data: Record<string, unknown>
    ) => {
      try {
        await invoke("send_bevy_input", {
          event,
          data,
        });
      } catch (error) {
        console.error("Failed to send input to Bevy:", error);
      }
    };

    // Mouse events
    const handleMouseMove = (e: MouseEvent) => {
      const rect = container.getBoundingClientRect();
      sendInputEvent("mouse_move", {
        x: e.clientX - rect.left,
        y: e.clientY - rect.top,
        clientX: e.clientX,
        clientY: e.clientY,
        buttons: e.buttons,
      });
    };

    const handleMouseDown = (e: MouseEvent) => {
      const rect = container.getBoundingClientRect();
      sendInputEvent("mouse_down", {
        button: e.button,
        x: e.clientX - rect.left,
        y: e.clientY - rect.top,
        clientX: e.clientX,
        clientY: e.clientY,
      });
    };

    const handleMouseUp = (e: MouseEvent) => {
      const rect = container.getBoundingClientRect();
      sendInputEvent("mouse_up", {
        button: e.button,
        x: e.clientX - rect.left,
        y: e.clientY - rect.top,
        clientX: e.clientX,
        clientY: e.clientY,
      });
    };

    const handleWheel = (e: WheelEvent) => {
      sendInputEvent("mouse_wheel", {
        deltaX: e.deltaX,
        deltaY: e.deltaY,
        deltaZ: e.deltaZ,
        deltaMode: e.deltaMode,
      });
    };

    // Keyboard events
    const handleKeyDown = (e: KeyboardEvent) => {
      sendInputEvent("key_down", {
        key: e.key,
        code: e.code,
        keyCode: e.keyCode,
        ctrlKey: e.ctrlKey,
        shiftKey: e.shiftKey,
        altKey: e.altKey,
        metaKey: e.metaKey,
      });
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      sendInputEvent("key_up", {
        key: e.key,
        code: e.code,
        keyCode: e.keyCode,
        ctrlKey: e.ctrlKey,
        shiftKey: e.shiftKey,
        altKey: e.altKey,
        metaKey: e.metaKey,
      });
    };

    // Add event listeners
    container.addEventListener("mousemove", handleMouseMove);
    container.addEventListener("mousedown", handleMouseDown);
    container.addEventListener("mouseup", handleMouseUp);
    container.addEventListener("wheel", handleWheel);
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);

    return () => {
      container.removeEventListener("mousemove", handleMouseMove);
      container.removeEventListener("mousedown", handleMouseDown);
      container.removeEventListener("mouseup", handleMouseUp);
      container.removeEventListener("wheel", handleWheel);
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [enabled]);

  return (
    <div className="h-full w-full" ref={containerRef}>
      {children}
    </div>
  );
}
