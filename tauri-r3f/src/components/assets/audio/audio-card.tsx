import { useState } from "react";
import { Music, X } from "lucide-react";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";
import type { AudioCardProps } from "../types";

export function AudioCard({
  audio,
  isSelected,
  showWaveform,
  currentlyPlaying,
  onClick,
  onPlayPause,
  onDelete,
}: AudioCardProps) {
  const [showDetails, setShowDetails] = useState(false);

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${Number.parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
  };

  const formatDuration = (seconds?: number) => {
    if (!seconds) return "0:00";
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  };

  const isPlaying = currentlyPlaying === audio.id;

  return (
    <div
      className={cn(
        "group relative cursor-pointer rounded-lg border-2 p-4 transition-all hover:shadow-md",
        isSelected ? "border-primary bg-primary/5 shadow-md" : "border-border"
      )}
      onClick={onClick}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onClick(e as any);
        }
      }}
      onMouseEnter={() => setShowDetails(true)}
      onMouseLeave={() => setShowDetails(false)}
    >
      {/* Header */}
      <div className="mb-2 flex items-center justify-between">
        <div className="flex min-w-0 flex-1 items-center gap-2">
          <Music className="h-4 w-4 shrink-0 text-muted-foreground" />
          <span className="truncate font-medium text-sm" title={audio.name}>
            {audio.name}
          </span>
        </div>

        {/* Play/Pause button */}
        <Button
          className="h-8 w-8 shrink-0 p-0"
          onClick={onPlayPause}
          size="sm"
          variant="ghost"
        >
          {isPlaying ? (
            <div className="flex gap-0.5">
              <div className="w-1 animate-pulse rounded-sm bg-current" />
              <div
                className="w-1 animate-pulse rounded-sm bg-current"
                style={{ animationDelay: "0.1s" }}
              />
              <div
                className="w-1 animate-pulse rounded-sm bg-current"
                style={{ animationDelay: "0.2s" }}
              />
            </div>
          ) : (
            <div className="ml-0.5 h-0 w-0 border-t border-t-transparent border-b border-b-transparent border-l-2 border-l-current" />
          )}
        </Button>
      </div>

      {/* Waveform visualization */}
      {showWaveform && (
        <div className="mb-3 h-8 overflow-hidden rounded bg-muted">
          <div className="flex h-full items-end gap-0.5 p-1">
            {Array.from({ length: 20 }, (_, i) => (
              <div
                className={cn(
                  "rounded-sm bg-primary/60 transition-all duration-300",
                  isPlaying ? "animate-pulse" : ""
                )}
                key={`${audio.id}-wave-${i}`}
                style={{
                  height: `${Math.random() * 100}%`,
                  width: "2px",
                  animationDelay: `${i * 0.1}s`,
                }}
              />
            ))}
          </div>
        </div>
      )}

      {/* Metadata */}
      <div className="space-y-1 text-muted-foreground text-xs">
        <div className="flex justify-between">
          <span>{audio.format.toUpperCase()}</span>
          <span>{formatDuration(audio.duration)}</span>
        </div>
        <div className="flex justify-between">
          <span>{formatFileSize(audio.fileSize)}</span>
          {audio.channels && (
            <span>{audio.channels === 1 ? "Mono" : "Stereo"}</span>
          )}
        </div>
        {audio.sampleRate && (
          <div className="text-right">{audio.sampleRate / 1000}kHz</div>
        )}
      </div>

      {/* Selection indicator */}
      {isSelected && (
        <div className="absolute top-2 right-2 flex h-4 w-4 items-center justify-center rounded-full bg-primary">
          <div className="h-2 w-2 rounded-full bg-primary-foreground" />
        </div>
      )}

      {/* Details overlay */}
      {showDetails && (
        <div className="absolute inset-0 flex flex-col justify-between rounded-lg bg-black/75 p-3 text-white text-xs">
          <div>
            <div className="mb-1 truncate font-medium">{audio.name}</div>
            <div className="text-muted-foreground">
              {audio.format.toUpperCase()} • {formatDuration(audio.duration)}
            </div>
          </div>

          <div className="space-y-1">
            <div className="flex justify-between">
              <span>Size:</span>
              <span>{formatFileSize(audio.fileSize)}</span>
            </div>
            {audio.channels && (
              <div className="flex justify-between">
                <span>Channels:</span>
                <span>{audio.channels === 1 ? "Mono" : "Stereo"}</span>
              </div>
            )}
            {audio.sampleRate && (
              <div className="flex justify-between">
                <span>Sample Rate:</span>
                <span>{audio.sampleRate}Hz</span>
              </div>
            )}
            {audio.bitrate && (
              <div className="flex justify-between">
                <span>Bitrate:</span>
                <span>{audio.bitrate}kbps</span>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Delete button */}
      <Button
        className="absolute top-2 right-10 h-6 w-6 p-0 opacity-0 transition-opacity group-hover:opacity-100"
        onClick={(e) => {
          e.stopPropagation();
          onDelete();
        }}
        size="sm"
        variant="destructive"
      >
        <X className="h-3 w-3" />
      </Button>
    </div>
  );
}
