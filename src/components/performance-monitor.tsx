import type { PerformanceMonitorProps as DreiPerformanceMonitorProps } from "@react-three/drei";
import { AlertTriangle, Settings } from "lucide-react";
import { memo, useEffect, useState } from "react";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";
import { performanceMonitor } from "~/utils/performance-monitor";

interface PerformanceMonitorProps extends DreiPerformanceMonitorProps {
  className?: string;
  showWarnings?: boolean;
}

interface PerformanceMetrics {
  fps: number;
  frameTime: number;
  memoryUsage: number;
  drawCalls: number;
  triangles: number;
  geometries: number;
  textures: number;
}

export const PerformanceMonitor = memo(function PerformanceMonitor({
  className,
  showWarnings = true,
}: PerformanceMonitorProps) {
  const [metrics, setMetrics] = useState<PerformanceMetrics>({
    fps: 0,
    frameTime: 0,
    memoryUsage: 0,
    drawCalls: 0,
    triangles: 0,
    geometries: 0,
    textures: 0,
  });

  const [warnings, setWarnings] = useState<string[]>([]);
  const [isVisible, setIsVisible] = useState(false);

  // Update metrics at regular intervals
  useEffect(() => {
    const updateMetrics = () => {
      const avgMetrics = performanceMonitor.getAverageMetrics(60); // Last 60 frames

      setMetrics({
        fps: avgMetrics.fps || 0,
        frameTime: avgMetrics.frameTime || 0,
        memoryUsage: avgMetrics.memoryUsage || 0,
        drawCalls: avgMetrics.drawCalls || 0,
        triangles: avgMetrics.triangles || 0,
        geometries: avgMetrics.geometries || 0,
        textures: avgMetrics.textures || 0,
      });

      if (showWarnings) {
        setWarnings(performanceMonitor.getPerformanceWarnings());
      }
    };

    // Update every second
    const interval = setInterval(updateMetrics, 1000);
    updateMetrics(); // Initial update

    return () => clearInterval(interval);
  }, [showWarnings]);

  const getPerformanceColor = (
    value: number,
    thresholds: { good: number; warning: number; bad: number }
  ) => {
    if (value >= thresholds.good) return "text-green-600";
    if (value >= thresholds.warning) return "text-yellow-600";
    return "text-red-600";
  };

  const getFpsVariant = (fps: number) => {
    if (fps >= 50) return "default";
    if (fps >= 30) return "secondary";
    return "destructive";
  };

  // Show all performance metrics in a horizontal layout
  return (
    <div className={cn("flex items-center gap-3", className)}>
      {/* FPS with color coding */}
      <div className="flex items-center gap-1">
        <span className="text-muted-foreground text-xs">FPS:</span>
        <Badge
          className="px-1.5 py-0.5 font-mono text-xs"
          variant={getFpsVariant(metrics.fps)}
        >
          {metrics.fps.toFixed(1)}
        </Badge>
      </div>

      {/* Frame Time */}
      <div className="flex items-center gap-1">
        <span className="text-muted-foreground text-xs">Frame:</span>
        <span
          className={cn(
            "font-medium font-mono text-xs",
            getPerformanceColor(metrics.frameTime, {
              good: 8,
              warning: 16,
              bad: 33,
            })
          )}
        >
          {metrics.frameTime.toFixed(1)}ms
        </span>
      </div>

      {/* Memory Usage */}
      <div className="flex items-center gap-1">
        <span className="text-muted-foreground text-xs">Mem:</span>
        <span className="font-medium font-mono text-xs">
          {(metrics.memoryUsage / 1024 / 1024).toFixed(0)}MB
        </span>
      </div>

      {/* Draw Calls */}
      <div className="flex items-center gap-1">
        <span className="text-muted-foreground text-xs">Draw:</span>
        <span className="font-medium font-mono text-xs">
          {metrics.drawCalls}
        </span>
      </div>

      {/* Triangles */}
      <div className="flex items-center gap-1">
        <span className="text-muted-foreground text-xs">Tri:</span>
        <span className="font-medium font-mono text-xs">
          {metrics.triangles.toLocaleString()}
        </span>
      </div>

      {/* Performance Warnings */}
      {warnings.length > 0 && (
        <Badge className="gap-1 text-xs" variant="destructive">
          <AlertTriangle className="h-3 w-3" />
          {warnings.length} issues
        </Badge>
      )}

      {/* Expand Button */}
      <Button
        className="h-6 w-6 p-0 opacity-60 hover:opacity-100"
        onClick={() => setIsVisible(!isVisible)}
        size="sm"
        variant="ghost"
      >
        <Settings className="h-3 w-3" />
      </Button>
    </div>
  );
});
