import { Activity, AlertTriangle, Cpu, HardDrive, Zap } from "lucide-react";
import { memo, useEffect, useMemo, useState } from "react";
import { Badge } from "~/components/ui/badge";
import { useMemoryMonitor } from "~/hooks/use-performance";
import { useSceneStore } from "~/stores/scene-store";
import { performanceMonitor } from "~/utils/performance-monitor";

interface PerformanceMetrics {
  fps: number;
  frameTime: number;
  memoryUsage: number;
  drawCalls: number;
  triangles: number;
  geometries: number;
  textures: number;
}

interface PerformanceFooterProps {
  className?: string;
}

export const PerformanceFooter = memo(function PerformanceFooter({
  className,
}: PerformanceFooterProps) {
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
  const memoryInfo = useMemoryMonitor(true); // Always enabled

  // Get scene stats directly to avoid object creation on every render
  const { objects, lights } = useSceneStore();
  const sceneStats = useMemo(
    () => ({
      objects: objects.length,
      lights: lights.length,
    }),
    [objects.length, lights.length]
  );

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

      setWarnings(performanceMonitor.getPerformanceWarnings());
    };

    // Update every second
    const interval = setInterval(updateMetrics, 1000);
    updateMetrics(); // Initial update

    return () => clearInterval(interval);
  }, []);

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

  const getStatusColor = (status: "good" | "warning" | "critical") => {
    switch (status) {
      case "good":
        return "text-green-600 bg-green-50 border-green-200";
      case "warning":
        return "text-yellow-600 bg-yellow-50 border-yellow-200";
      case "critical":
        return "text-red-600 bg-red-50 border-red-200";
      default:
        return "text-gray-600 bg-gray-50 border-gray-200";
    }
  };

  const getStatusIcon = (status: "good" | "warning" | "critical") => {
    switch (status) {
      case "good":
        return "🟢";
      case "warning":
        return "🟡";
      case "critical":
        return "🔴";
      default:
        return "⚪";
    }
  };

  // Determine overall performance status
  const getOverallStatus = (): "good" | "warning" | "critical" => {
    if (warnings.length > 0 || metrics.fps < 30) return "critical";
    if (metrics.fps < 50 || metrics.frameTime > 16.67) return "warning";
    return "good";
  };

  const overallStatus = getOverallStatus();

  return (
    <div
      className={`fixed right-0 bottom-0 left-0 z-50 border-t bg-background/95 backdrop-blur-sm ${className}`}
    >
      <div className="flex h-12 items-center justify-between px-4 py-2">
        {/* Left Side - Core Performance Metrics */}
        <div className="flex items-center gap-4">
          {/* FPS with status indicator */}
          <div className="flex items-center gap-2">
            <div
              className={`flex items-center gap-1 rounded border px-2 py-1 font-medium text-xs ${getStatusColor(overallStatus)}`}
            >
              {getStatusIcon(overallStatus)}
              <span>Performance</span>
            </div>

            <Badge
              className="font-mono text-xs"
              variant={getFpsVariant(metrics.fps)}
            >
              {metrics.fps.toFixed(0)} FPS
            </Badge>
          </div>

          {/* Frame Time */}
          <div className="flex items-center gap-1 text-xs">
            <Zap className="h-3 w-3" />
            <span className="text-muted-foreground">Frame:</span>
            <span
              className={`font-medium font-mono ${getPerformanceColor(
                metrics.frameTime,
                {
                  good: 8,
                  warning: 16,
                  bad: 33,
                }
              )}`}
            >
              {metrics.frameTime.toFixed(1)}ms
            </span>
          </div>

          {/* Memory Usage */}
          <div className="flex items-center gap-1 text-xs">
            <Cpu className="h-3 w-3" />
            <span className="text-muted-foreground">Mem:</span>
            <span className="font-medium font-mono">
              {(metrics.memoryUsage / 1024 / 1024).toFixed(0)}MB
            </span>
          </div>

          {/* Draw Calls */}
          <div className="flex items-center gap-1 text-xs">
            <Activity className="h-3 w-3" />
            <span className="text-muted-foreground">Draw:</span>
            <span className="font-medium font-mono">{metrics.drawCalls}</span>
          </div>

          {/* Triangles */}
          <div className="flex items-center gap-1 text-xs">
            <span className="text-muted-foreground text-xs">Tri:</span>
            <span className="font-medium font-mono text-xs">
              {metrics.triangles.toLocaleString()}
            </span>
          </div>
        </div>

        {/* Center - Scene Statistics */}
        <div className="flex items-center gap-4 text-xs">
          <div className="flex items-center gap-1">
            <HardDrive className="h-3 w-3" />
            <span className="text-muted-foreground">Scene:</span>
            <span className="font-medium font-mono">
              {sceneStats.objects} obj, {sceneStats.lights} lights
            </span>
          </div>

          <div className="flex items-center gap-1">
            <span className="text-muted-foreground">Resources:</span>
            <span className="font-medium font-mono">
              {metrics.geometries} geo, {metrics.textures} tex
            </span>
          </div>
        </div>

        {/* Right Side - Warnings and Memory Details */}
        <div className="flex items-center gap-4">
          {/* Memory Details */}
          {memoryInfo && (
            <div className="flex items-center gap-1 text-xs">
              <div className="h-2 w-16 rounded-full bg-secondary">
                <div
                  className="h-2 rounded-full bg-primary transition-all duration-300"
                  style={{
                    width: `${(memoryInfo.used / memoryInfo.limit) * 100}%`,
                  }}
                />
              </div>
              <span className="font-mono text-muted-foreground">
                {memoryInfo.used}/{memoryInfo.limit}MB
              </span>
            </div>
          )}

          {/* Performance Warnings */}
          {warnings.length > 0 && (
            <Badge className="gap-1 text-xs" variant="destructive">
              <AlertTriangle className="h-3 w-3" />
              {warnings.length} issues
            </Badge>
          )}

          {/* Scene Title */}
          <div className="flex items-center gap-1">
            <span
              className={`font-medium text-muted-foreground text-xs ${
                useSceneStore.getState().sceneMetadata.isModified
                  ? "text-orange-600"
                  : ""
              }`}
            >
              {useSceneStore.getState().sceneMetadata.name}
              {useSceneStore.getState().sceneMetadata.isModified && " *"}
            </span>
          </div>

          {/* Environment Info */}
          <div className="text-muted-foreground text-xs">
            {process.env.NODE_ENV === "development" ? "DEV" : "PROD"}
          </div>
        </div>
      </div>

      {/* Expanded Warnings Section */}
      {warnings.length > 0 && (
        <div className="border-t bg-destructive/5 px-4 py-1">
          <div className="flex items-start gap-2 text-xs">
            <AlertTriangle className="mt-0.5 h-3 w-3 shrink-0 text-destructive" />
            <div className="flex flex-wrap gap-2">
              {warnings.map((warning) => (
                <span className="text-destructive" key={warning}>
                  {warning}
                </span>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
});
