import { Upload } from "lucide-react";
import { Button } from "~/components/ui/button";
import { Card } from "~/components/ui/card";
import { useArchiveOperations } from "../hooks/use-archive-operations";
import { formatFileSize } from "../utils/utils";
import type { ImgArchive } from "../../types";

interface LoadedArchivesSectionProps {
  loadedArchives: Map<string, ImgArchive>;
  expandedArchives: Set<string>;
  isImporting: boolean;
  setIsImporting: (importing: boolean) => void;
  onToggleArchiveExpansion: (archivePath: string) => void;
}

export function LoadedArchivesSection({
  loadedArchives,
  expandedArchives,
  isImporting,
  setIsImporting,
  onToggleArchiveExpansion,
}: LoadedArchivesSectionProps) {
  const {
    extractMultipleEntries,
    selectOutputDirectory,
    selectOutputFile,
    extractImgEntry,
  } = useArchiveOperations();

  const handleExtractAll = async (archive: ImgArchive, archivePath: string) => {
    try {
      const outputDir = await selectOutputDirectory();
      if (outputDir && typeof outputDir === "string") {
        setIsImporting(true);
        await extractMultipleEntries(
          archive,
          archivePath,
          outputDir,
          archive.entries.length
        );
      }
    } catch (error) {
      console.error("Batch extraction error:", error);
    } finally {
      setIsImporting(false);
    }
  };

  const handleExtractEntry = async (archivePath: string, entryName: string) => {
    try {
      const outputPath = await selectOutputFile(entryName);
      if (outputPath && typeof outputPath === "string") {
        await extractImgEntry(archivePath, entryName, outputPath);
      }
    } catch (error) {
      console.error(`Failed to extract ${entryName}:`, error);
    }
  };

  if (loadedArchives.size === 0) {
    return null;
  }

  return (
    <div className="mb-4">
      <h4 className="mb-2 font-medium text-sm">Loaded Archives</h4>
      <div className="space-y-2">
        {Array.from(loadedArchives.entries()).map(([path, archive]) => (
          <Card className="p-3" key={path}>
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Button
                  className="h-6 w-6 p-0"
                  onClick={() => onToggleArchiveExpansion(path)}
                  size="sm"
                  variant="ghost"
                >
                  {expandedArchives.has(path) ? "▼" : "▶"}
                </Button>
                <div>
                  <div className="font-medium text-sm">
                    {path.split("/").pop()?.split("\\").pop()}
                  </div>
                  <div className="text-muted-foreground text-xs">
                    {archive.total_entries} entries •{" "}
                    {formatFileSize(archive.file_size)} • {archive.version}
                  </div>
                </div>
              </div>
              <Button
                disabled={isImporting}
                onClick={() => handleExtractAll(archive, path)}
                size="sm"
                variant="outline"
              >
                Extract All
              </Button>
            </div>

            {expandedArchives.has(path) && (
              <div className="mt-3 max-h-64 overflow-y-auto">
                <div className="space-y-1">
                  {archive.entries.slice(0, 50).map((entry) => (
                    <div
                      className="flex items-center justify-between rounded bg-muted/50 px-2 py-1 text-xs"
                      key={entry.name}
                    >
                      <div className="flex items-center gap-2">
                        <span className="font-mono">{entry.name}</span>
                        {entry.is_compressed && (
                          <span className="rounded bg-blue-100 px-1 text-blue-700">
                            compressed
                          </span>
                        )}
                        {entry.rw_version && (
                          <span className="rounded bg-green-100 px-1 text-green-700">
                            {entry.rw_version}
                          </span>
                        )}
                      </div>
                      <div className="flex items-center gap-2">
                        <span className="text-muted-foreground">
                          {formatFileSize(entry.size * 2048)}{" "}
                          {/* entry.size is in sectors, multiply by 2048 for bytes */}
                        </span>
                        <Button
                          className="h-6 w-6 p-0"
                          onClick={() => handleExtractEntry(path, entry.name)}
                          size="sm"
                          variant="ghost"
                        >
                          <Upload className="h-3 w-3" />
                        </Button>
                      </div>
                    </div>
                  ))}
                  {archive.entries.length > 50 && (
                    <div className="py-1 text-center text-muted-foreground text-xs">
                      ... and {archive.entries.length - 50} more entries
                    </div>
                  )}
                </div>
              </div>
            )}
          </Card>
        ))}
      </div>
    </div>
  );
}
