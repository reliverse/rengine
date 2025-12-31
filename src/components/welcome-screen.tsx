import { useNavigate } from "@tanstack/react-router";
import {
  File,
  FolderOpen,
  Lightbulb,
  LogIn,
  Settings,
  User,
  Zap,
} from "lucide-react";
import { useState } from "react";
import { Button } from "~/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { ScrollArea } from "~/components/ui/scroll-area";
import { useAuth } from "~/contexts/auth-context";
import { useSceneStore } from "~/stores/scene-store";
import { loadScene } from "~/utils/scene-persistence";

interface SceneTemplate {
  id: string;
  name: string;
  description: string;
  icon: React.ReactNode;
  category: "basic" | "lighting" | "character" | "advanced";
  setupScene: () => void;
}

export function WelcomeScreen() {
  const navigate = useNavigate();
  const { isAuthenticated } = useAuth();
  const { clearScene, addObject, setCameraPosition, setCameraTarget } =
    useSceneStore();
  const [selectedTemplate, setSelectedTemplate] = useState<string | null>(null);

  const createBasicScene = () => {
    clearScene();
    addObject("cube", [0, 0, 0]);
    setCameraPosition([5, 5, 5]);
    setCameraTarget([0, 0, 0]);
  };

  const createLightingScene = () => {
    clearScene();
    addObject("plane", [0, 0, 0]);
    addObject("cube", [0, 0, 0]);
    addObject("sphere", [2, 1, 0]);
    addObject("cube", [-2, 0, 1]);
    const sceneState = useSceneStore.getState();
    sceneState.applyLightingPreset("studio");
    setCameraPosition([5, 3, 5]);
    setCameraTarget([0, 0, 0]);
  };

  const createCharacterScene = () => {
    clearScene();
    addObject("sphere", [0, 1, 0]);
    addObject("plane", [0, 0, 0]);
    setCameraPosition([3, 2, 3]);
    setCameraTarget([0, 1, 0]);
  };

  const createOutdoorScene = () => {
    clearScene();
    addObject("plane", [0, 0, 0]);
    addObject("cube", [0, 0, 0]);
    addObject("sphere", [3, 1, 2]);
    const sceneState = useSceneStore.getState();
    sceneState.applyLightingPreset("outdoor");
    setCameraPosition([8, 5, 8]);
    setCameraTarget([0, 0, 0]);
  };

  const createNeonScene = () => {
    clearScene();
    addObject("cube", [0, 0, 0]);
    addObject("sphere", [2, 1, 0]);
    addObject("plane", [0, 0, 0]);
    const sceneState = useSceneStore.getState();
    sceneState.applyLightingPreset("neon");
    setCameraPosition([6, 4, 6]);
    setCameraTarget([0, 0, 0]);
  };

  const createDramaticScene = () => {
    clearScene();
    addObject("cube", [0, 0, 0]);
    addObject("sphere", [1.5, 1, 1]);
    addObject("plane", [0, 0, 0]);
    const sceneState = useSceneStore.getState();
    sceneState.applyLightingPreset("dramatic");
    setCameraPosition([7, 5, 7]);
    setCameraTarget([0, 0, 0]);
  };

  const sceneTemplates: SceneTemplate[] = [
    {
      id: "blank",
      name: "Blank Scene",
      description: "Start with an empty 3D scene",
      icon: <File className="h-8 w-8" />,
      category: "basic",
      setupScene: () => {
        clearScene();
        setCameraPosition([5, 5, 5]);
        setCameraTarget([0, 0, 0]);
      },
    },
    {
      id: "basic",
      name: "Basic Scene",
      description: "A simple scene with a cube",
      icon: <div className="text-2xl">⬜</div>,
      category: "basic",
      setupScene: createBasicScene,
    },
    {
      id: "lighting",
      name: "Studio Lighting",
      description: "Professional studio setup with multiple lights",
      icon: <Lightbulb className="h-8 w-8" />,
      category: "lighting",
      setupScene: createLightingScene,
    },
    {
      id: "outdoor",
      name: "Outdoor Scene",
      description: "Bright sunlight with realistic outdoor lighting",
      icon: <div className="text-2xl">☀️</div>,
      category: "lighting",
      setupScene: createOutdoorScene,
    },
    {
      id: "neon",
      name: "Neon Lights",
      description: "Colorful neon lighting for dynamic scenes",
      icon: <div className="text-2xl">💡</div>,
      category: "lighting",
      setupScene: createNeonScene,
    },
    {
      id: "dramatic",
      name: "Dramatic Lighting",
      description: "High contrast lighting with strong shadows",
      icon: <div className="text-2xl">🎭</div>,
      category: "lighting",
      setupScene: createDramaticScene,
    },
    {
      id: "character",
      name: "Character Scene",
      description: "Scene optimized for character modeling",
      icon: <Zap className="h-8 w-8" />,
      category: "character",
      setupScene: createCharacterScene,
    },
  ];

  const handleTemplateSelect = (template: SceneTemplate) => {
    setSelectedTemplate(template.id);
    template.setupScene();
    setTimeout(() => {
      navigate({ to: "/" });
    }, 300);
  };

  const handleLoadSavedProject = async () => {
    const result = await loadScene();

    if (result.success && result.data) {
      const sceneState = useSceneStore.getState();
      sceneState.loadScene(result.data.scene);
      sceneState.setSceneMetadata({
        name: result.data.metadata.name,
      });
      sceneState.setCurrentFilePath(null);

      navigate({ to: "/" });
    } else if (result.error !== "Load cancelled") {
      console.error("Load failed:", result.error);
    }
  };

  const handleSettings = () => {
    navigate({ to: "/settings" });
  };

  const getCategoryBadgeClass = (category: string) => {
    switch (category) {
      case "basic":
        return "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200";
      case "lighting":
        return "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200";
      case "character":
        return "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200";
      default:
        return "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200";
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-background p-8">
      <div className="w-full max-w-6xl">
        <div className="mb-12 text-center">
          <h1 className="mb-4 cursor-pointer bg-linear-to-r from-primary to-primary/60 bg-clip-text font-bold text-4xl text-transparent transition-all duration-300 hover:scale-110 hover:from-primary/80 hover:to-primary hover:shadow-2xl">
            Rengine
          </h1>
          <p className="text-muted-foreground text-xl">Game Engine</p>
        </div>

        <div className="grid grid-cols-1 gap-8 lg:grid-cols-2">
          <div>
            <h2 className="mb-6 flex items-center gap-2 font-semibold text-2xl">
              <File className="h-6 w-6" />
              New Project
            </h2>
            <ScrollArea className="h-96">
              <div className="grid grid-cols-1 gap-3 pr-4 sm:grid-cols-2">
                {sceneTemplates.map((template) => (
                  <Card
                    className={`cursor-pointer transition-all duration-200 hover:shadow-lg ${
                      selectedTemplate === template.id
                        ? "shadow-lg ring-2 ring-primary"
                        : "hover:border-primary/50"
                    }`}
                    key={template.id}
                    onClick={() => handleTemplateSelect(template)}
                  >
                    <CardHeader className="px-3 pt-3 pb-1.5">
                      <div className="flex items-center gap-1.5">
                        <div className="rounded-md bg-primary/10 p-1 text-primary">
                          {template.icon}
                        </div>
                        <div className="min-w-0 flex-1">
                          <CardTitle className="truncate text-sm">
                            {template.name}
                          </CardTitle>
                          <CardDescription className="truncate text-xs">
                            {template.description}
                          </CardDescription>
                        </div>
                      </div>
                    </CardHeader>
                    <CardContent className="px-3 pt-0 pb-2">
                      <div
                        className={`inline-block rounded-full px-1.5 py-0.5 text-xs ${getCategoryBadgeClass(
                          template.category
                        )}`}
                      >
                        {template.category}
                      </div>
                    </CardContent>
                  </Card>
                ))}
              </div>
            </ScrollArea>
          </div>

          <div>
            <h2 className="mb-6 flex items-center gap-2 font-semibold text-2xl">
              <Settings className="h-6 w-6" />
              Quick Actions
            </h2>
            <div className="space-y-3">
              <Button
                className="w-full justify-start"
                onClick={handleLoadSavedProject}
                variant="outline"
              >
                <FolderOpen className="mr-3 h-4 w-4" />
                Load Saved Project
              </Button>

              <Button
                className="w-full justify-start"
                onClick={handleSettings}
                variant="outline"
              >
                <Settings className="mr-3 h-4 w-4" />
                Editor Settings
              </Button>

              {/* Authentication Buttons */}
              {isAuthenticated ? (
                <Button
                  className="w-full justify-start"
                  onClick={() => navigate({ to: "/account" })}
                  variant="outline"
                >
                  <User className="mr-3 h-4 w-4" />
                  Account
                </Button>
              ) : (
                <Button
                  className="w-full justify-start"
                  onClick={() => navigate({ to: "/auth" })}
                  variant="outline"
                >
                  <LogIn className="mr-3 h-4 w-4" />
                  Sign In
                </Button>
              )}
            </div>
          </div>
        </div>

        <div className="mt-12 text-center">
          <p className="text-muted-foreground text-sm">
            Drag and drop 3D models directly into the scene • Supports GLTF,
            OBJ, FBX formats
          </p>
        </div>
      </div>
    </div>
  );
}
