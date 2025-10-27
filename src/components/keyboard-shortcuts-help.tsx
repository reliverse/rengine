import { X } from "lucide-react";
import { useEffect, useState } from "react";
import { Button } from "~/components/ui/button";

interface ShortcutGroup {
  title: string;
  shortcuts: Array<{
    keys: string[];
    description: string;
  }>;
}

const shortcutGroups: ShortcutGroup[] = [
  {
    title: "Navigation",
    shortcuts: [
      { keys: ["LMB", "Drag"], description: "Move camera forward/backward and rotate left/right" },
      { keys: ["RMB", "Drag"], description: "Rotate camera to look around" },
      { keys: ["MMB", "Drag"], description: "Pan camera up, down, left, right" },
      { keys: ["RMB", "WASD"], description: "Move camera with WASD keys" },
      { keys: ["RMB", "Q", "E"], description: "Move camera up/down" },
      { keys: ["RMB", "Scroll"], description: "Adjust camera movement speed" },
      { keys: ["Alt", "LMB", "Drag"], description: "Orbit around target" },
      { keys: ["Alt", "RMB", "Drag"], description: "Dolly (zoom) toward/away from target" },
      { keys: ["Alt", "MMB", "Drag"], description: "Track camera left, right, up, down" },
      { keys: ["F"], description: "Focus camera on selected object" },
    ],
  },
  {
    title: "Tools",
    shortcuts: [
      { keys: ["Q"], description: "Select Tool" },
      { keys: ["W"], description: "Move Tool" },
      { keys: ["E"], description: "Rotate Tool" },
      { keys: ["R"], description: "Scale Tool" },
      { keys: ["Space"], description: "Cycle through tools" },
    ],
  },
  {
    title: "Object Creation",
    shortcuts: [
      { keys: ["C"], description: "Add Cube" },
      { keys: ["Shift", "C"], description: "Add Sphere" },
      { keys: ["P"], description: "Add Plane" },
    ],
  },
  {
    title: "Selection",
    shortcuts: [
      { keys: ["LMB"], description: "Select object under cursor" },
      { keys: ["Ctrl", "LMB"], description: "Add object to selection" },
      { keys: ["Shift", "LMB"], description: "Add object to selection" },
      { keys: ["LMB", "Drag"], description: "Marquee selection box" },
      { keys: ["Shift", "LMB", "Drag"], description: "Add objects to selection" },
      { keys: ["Ctrl", "RMB", "Drag"], description: "Remove objects from selection" },
    ],
  },
  {
    title: "Display",
    shortcuts: [
      { keys: ["G"], description: "Toggle Game Mode" },
      { keys: ["F11"], description: "Toggle Fullscreen" },
      { keys: ["Ctrl", "R"], description: "Toggle Real-time Playback" },
      { keys: ["F3"], description: "Toggle Stats Display" },
    ],
  },
  {
    title: "Transform",
    shortcuts: [
      { keys: ["LMB", "on", "Axis"], description: "Move/rotate/scale along specific axis" },
      { keys: ["LMB", "on", "Center"], description: "Move/scale uniformly" },
      { keys: ["V"], description: "Toggle vertex snapping (Move tool only)" },
      { keys: ["MMB", "Drag", "on", "Pivot"], description: "Move object pivot temporarily" },
    ],
  },
];

export default function KeyboardShortcutsHelp() {
  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "F1") {
        event.preventDefault();
        setIsOpen((prev) => !prev);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 max-w-4xl max-h-[80vh] overflow-y-auto">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-2xl font-bold text-white">Keyboard Shortcuts</h2>
          <Button
            className="text-gray-400 hover:text-white"
            onClick={() => setIsOpen(false)}
            size="sm"
            variant="ghost"
          >
            <X className="w-4 h-4" />
          </Button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {shortcutGroups.map((group) => (
            <div className="space-y-3" key={group.title}>
              <h3 className="text-lg font-semibold text-blue-400">{group.title}</h3>
              <div className="space-y-2">
                {group.shortcuts.map((shortcut, index) => (
                  <div className="flex items-start gap-3" key={index}>
                    <div className="flex gap-1 flex-wrap">
                      {shortcut.keys.map((key, keyIndex) => (
                        <span
                          className="bg-gray-700 text-gray-200 px-2 py-1 rounded text-sm font-mono"
                          key={keyIndex}
                        >
                          {key}
                        </span>
                      ))}
                    </div>
                    <span className="text-gray-300 text-sm flex-1">{shortcut.description}</span>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>

        <div className="mt-6 pt-4 border-t border-gray-700">
          <p className="text-gray-400 text-sm">
            Press <kbd className="bg-gray-700 px-2 py-1 rounded text-xs">F1</kbd> to toggle this
            help
          </p>
        </div>
      </div>
    </div>
  );
}
