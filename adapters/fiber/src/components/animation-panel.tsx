import { Pause, Play, RotateCcw, Square } from "lucide-react";
import { useEffect, useState } from "react";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { Slider } from "~/components/ui/slider";
import { useSceneStore } from "~/stores/scene-store";
import type { AnimationController } from "~/utils/animation-system";

export function AnimationPanel() {
  const selectedObjectIds = useSceneStore((state) => state.selectedObjectIds);
  const objects = useSceneStore((state) => state.objects);

  const [animationController, setAnimationController] =
    useState<AnimationController | null>(null);
  const [animationState, setAnimationState] = useState<any>(null);

  // Update animation controller when selection changes
  useEffect(() => {
    if (selectedObjectIds.length === 1) {
      const selectedObject = objects.find(
        (obj) => obj.id === selectedObjectIds[0]
      );
      if (selectedObject?.animationController) {
        setAnimationController(selectedObject.animationController);
      } else {
        setAnimationController(null);
      }
    } else {
      setAnimationController(null);
    }
  }, [selectedObjectIds, objects]);

  // Update animation state
  useEffect(() => {
    if (animationController) {
      const updateState = () => {
        setAnimationState(animationController.getState());
      };

      // Update immediately and then periodically
      updateState();
      const interval = setInterval(updateState, 100);
      return () => clearInterval(interval);
    }
    setAnimationState(null);
  }, [animationController]);

  if (
    !(animationController && animationState) ||
    animationState.clips.length === 0
  ) {
    return null;
  }

  const handlePlay = () => {
    if (animationState.currentClipId) {
      animationController.resume();
    } else if (animationState.clips.length > 0) {
      animationController.play(animationState.clips[0].id);
    }
  };

  const handlePause = () => {
    animationController.pause();
  };

  const handleStop = () => {
    animationController.stop();
  };

  const handleClipChange = (clipId: string) => {
    animationController.play(clipId);
  };

  const handleTimeChange = (value: number | readonly number[]) => {
    animationController.setTime(Array.isArray(value) ? value[0] : value);
  };

  const handleSpeedChange = (value: number | readonly number[]) => {
    animationController.setSpeed(Array.isArray(value) ? value[0] : value);
  };

  const handleLoopModeChange = (mode: string) => {
    animationController.setLoopMode(mode as "once" | "repeat" | "pingpong");
  };

  return (
    <Card className="w-full">
      <CardHeader className="pb-3">
        <CardTitle className="font-medium text-sm">Animation</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Animation Controls */}
        <div className="flex items-center gap-2">
          <Button
            disabled={animationState.isPlaying}
            onClick={handlePlay}
            size="sm"
            variant="outline"
          >
            <Play className="h-4 w-4" />
          </Button>
          <Button
            disabled={!animationState.isPlaying}
            onClick={handlePause}
            size="sm"
            variant="outline"
          >
            <Pause className="h-4 w-4" />
          </Button>
          <Button onClick={handleStop} size="sm" variant="outline">
            <Square className="h-4 w-4" />
          </Button>
          <Button
            onClick={() => animationController.setTime(0)}
            size="sm"
            variant="outline"
          >
            <RotateCcw className="h-4 w-4" />
          </Button>
        </div>

        {/* Animation Clip Selector */}
        <div className="space-y-2">
          <label className="font-medium text-xs" htmlFor="animation-clip">
            Animation Clip
          </label>
          <Select
            onValueChange={handleClipChange}
            value={animationState.currentClipId || ""}
          >
            <SelectTrigger className="h-8" id="animation-clip">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {animationState.clips.map((clip: any) => (
                <SelectItem key={clip.id} value={clip.id}>
                  {clip.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {/* Timeline */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <span className="font-medium text-xs">Time</span>
            <span className="text-muted-foreground text-xs">
              {animationState.time?.toFixed(2)}s /{" "}
              {animationState.currentClipId
                ? animationState.clips
                    .find((c: any) => c.id === animationState.currentClipId)
                    ?.duration.toFixed(2)
                : 0}
              s
            </span>
          </div>
          <Slider
            className="w-full"
            max={
              animationState.currentClipId
                ? animationState.clips.find(
                    (c: any) => c.id === animationState.currentClipId
                  )?.duration || 0
                : 0
            }
            min={0}
            onValueChange={handleTimeChange}
            step={0.01}
            value={[animationState.time || 0]}
          />
        </div>

        {/* Speed Control */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <span className="font-medium text-xs">Speed</span>
            <span className="text-muted-foreground text-xs">
              {animationState.speed?.toFixed(2)}x
            </span>
          </div>
          <Slider
            className="w-full"
            max={3}
            min={0.1}
            onValueChange={handleSpeedChange}
            step={0.1}
            value={[animationState.speed || 1]}
          />
        </div>

        {/* Loop Mode */}
        <div className="space-y-2">
          <label className="font-medium text-xs" htmlFor="loop-mode">
            Loop Mode
          </label>
          <Select
            onValueChange={handleLoopModeChange}
            value={animationState.loopMode || "repeat"}
          >
            <SelectTrigger className="h-8" id="loop-mode">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="once">Once</SelectItem>
              <SelectItem value="repeat">Repeat</SelectItem>
              <SelectItem value="pingpong">Ping Pong</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </CardContent>
    </Card>
  );
}
