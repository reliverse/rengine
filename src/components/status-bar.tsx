/**
 * Professional Status Bar Component with Memory Monitoring
 * Based on RWMS status bar with real-time memory tracking
 */

import { invoke } from "@tauri-apps/api/core";
import { Activity, Cpu, HardDrive, MemoryStick } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { Badge } from "~/components/ui/badge";
import { Separator } from "~/components/ui/separator";
import { ModernDarkTheme } from "~/styles/theme";
import { getResponsiveManager } from "~/utils/responsive-utils";

interface MemoryStats {
  process_memory_mb: number;
  system_memory_used_mb: number;
  system_memory_total_mb: number;
  system_memory_percentage: number;
  timestamp: string;
}

interface StatusBarProps {
  className?: string;
  showMemoryStats?: boolean;
  showSystemStats?: boolean;
  updateInterval?: number; // in milliseconds
}

export function StatusBar({
  className = "",
  showMemoryStats = true,
  showSystemStats = true,
  updateInterval = 5000, // 5 seconds
}: StatusBarProps) {
  const [memoryStats, setMemoryStats] = useState<MemoryStats | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const rm = getResponsiveManager();

  // Update memory stats
  const updateMemoryStats = useCallback(async () => {
    if (!showMemoryStats) return;

    try {
      setIsLoading(true);
      setError(null);

      const stats: MemoryStats = await invoke("get_memory_stats");
      setMemoryStats(stats);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Failed to get memory stats"
      );
      console.error("Failed to get memory stats:", err);
    } finally {
      setIsLoading(false);
    }
  }, [showMemoryStats]);

  // Initial load and periodic updates
  useEffect(() => {
    updateMemoryStats();

    if (showMemoryStats && updateInterval > 0) {
      const interval = setInterval(updateMemoryStats, updateInterval);
      return () => clearInterval(interval);
    }
  }, [updateMemoryStats, showMemoryStats, updateInterval]);

  const formatMemory = (mb: number): string => {
    if (mb >= 1024) {
      return `${(mb / 1024).toFixed(1)} GB`;
    }
    return `${mb.toFixed(0)} MB`;
  };

  const getMemoryUsageColor = (percentage: number): string => {
    if (percentage >= 90) return ModernDarkTheme.STATUS_ERROR;
    if (percentage >= 75) return ModernDarkTheme.STATUS_WARNING;
    if (percentage >= 50) return ModernDarkTheme.STATUS_INFO;
    return ModernDarkTheme.STATUS_SUCCESS;
  };

  const statusBarStyle = {
    backgroundColor: ModernDarkTheme.BACKGROUND_SECONDARY,
    borderTop: `1px solid ${ModernDarkTheme.BORDER_SECONDARY}`,
    color: ModernDarkTheme.TEXT_SECONDARY,
    fontSize: `${rm.getFontConfig().status.size}px`,
    height: rm.getScaledSize(24),
    padding: `0 ${rm.getSpacingConfig().small}px`,
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    gap: rm.getSpacingConfig().medium,
  };

  const sectionStyle = {
    display: "flex",
    alignItems: "center",
    gap: rm.getSpacingConfig().small,
  };

  return (
    <div className={`status-bar ${className}`} style={statusBarStyle}>
      {/* Left section - Tool status and operations */}
      <div style={sectionStyle}>
        <Badge className="text-xs" variant="outline">
          <Activity className="mr-1 h-3 w-3" />
          Ready
        </Badge>

        {error && (
          <Badge className="text-xs" variant="destructive">
            Error: {error}
          </Badge>
        )}
      </div>

      {/* Center section - Memory and system stats */}
      {(showMemoryStats || showSystemStats) && (
        <div style={sectionStyle}>
          {showMemoryStats && memoryStats && (
            <div style={sectionStyle}>
              <MemoryStick className="h-3 w-3" />
              <span className="text-xs">
                Process: {formatMemory(memoryStats.process_memory_mb)}
              </span>
              {showSystemStats && (
                <Separator className="h-3" orientation="vertical" />
              )}
            </div>
          )}

          {showSystemStats &&
            memoryStats &&
            memoryStats.system_memory_total_mb > 0 && (
              <div style={sectionStyle}>
                <HardDrive className="h-3 w-3" />
                <span className="text-xs">
                  System: {formatMemory(memoryStats.system_memory_used_mb)} /{" "}
                  {formatMemory(memoryStats.system_memory_total_mb)}
                </span>
                <div
                  className="h-1.5 w-16 overflow-hidden rounded-full"
                  style={{
                    backgroundColor: ModernDarkTheme.BACKGROUND_TERTIARY,
                  }}
                >
                  <div
                    className="h-full rounded-full transition-all duration-300 ease-in-out"
                    style={{
                      width: `${memoryStats.system_memory_percentage}%`,
                      backgroundColor: getMemoryUsageColor(
                        memoryStats.system_memory_percentage
                      ),
                    }}
                  />
                </div>
                <span
                  className="font-medium text-xs"
                  style={{
                    color: getMemoryUsageColor(
                      memoryStats.system_memory_percentage
                    ),
                  }}
                >
                  {memoryStats.system_memory_percentage.toFixed(1)}%
                </span>
              </div>
            )}

          {isLoading && (
            <div style={sectionStyle}>
              <Cpu className="h-3 w-3 animate-spin" />
              <span className="text-xs">Updating...</span>
            </div>
          )}
        </div>
      )}

      {/* Right section - Additional info */}
      <div style={sectionStyle}>
        <span className="text-xs opacity-75">Rengine v0.2.0</span>
      </div>
    </div>
  );
}
