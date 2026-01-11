import {
  AlertCircle,
  CheckCircle,
  FileImage,
  FolderOpen,
  Settings,
  Upload,
  X,
} from "lucide-react";
import type React from "react";
import { useCallback, useRef, useState } from "react";
import { Badge } from "~/components/ui/badge";
import { Button } from "~/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "~/components/ui/dialog";
import { Progress } from "~/components/ui/progress";
import { ScrollArea } from "~/components/ui/scroll-area";
import { cn } from "~/lib/utils";
import { useTextureStore } from "~/stores/texture-store";
import { isTauri } from "~/utils/tauri-texture-api";

interface TextureImportDialogProps {
  children: React.ReactNode;
}

interface ImportFile {
  file: File;
  status: "pending" | "processing" | "success" | "error";
  error?: string;
  textureId?: string;
  preview?: string;
}

const SUPPORTED_FORMATS = [
  "image/png",
  "image/jpeg",
  "image/jpg",
  "image/webp",
  "image/tiff",
  "image/bmp",
  ".tga",
  ".dds",
  ".hdr",
  ".exr",
];

const TEXTURE_TYPE_SUGGESTIONS = {
  normal: ["normal", "norm", "_n.", "_normal."],
  roughness: ["roughness", "rough", "_r.", "_rough."],
  metallic: ["metallic", "metal", "_m.", "_metallic."],
  ao: ["ao", "ambient", "occlusion", "_ao.", "_occ."],
  emissive: ["emissive", "emission", "_e.", "_emissive."],
  height: ["height", "displacement", "_h.", "_height."],
  opacity: ["opacity", "alpha", "_a.", "_opacity."],
} as const;

export function TextureImportDialog({ children }: TextureImportDialogProps) {
  const { importTextures, importTexturesFromDialog } = useTextureStore();
  const [open, setOpen] = useState(false);
  const [files, setFiles] = useState<ImportFile[]>([]);
  const [isImporting, setIsImporting] = useState(false);
  const [importProgress, setImportProgress] = useState(0);
  const [autoDetectTypes, setAutoDetectTypes] = useState(true);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileSelect = useCallback((selectedFiles: FileList | null) => {
    if (!selectedFiles) return;

    const newFiles: ImportFile[] = Array.from(selectedFiles)
      .filter((file) => {
        const isSupported = SUPPORTED_FORMATS.some(
          (format) =>
            file.type.includes(format) ||
            file.name.toLowerCase().endsWith(format)
        );

        if (!isSupported) {
          console.warn(`Unsupported file type: ${file.type} for ${file.name}`);
        }

        return isSupported;
      })
      .map((file) => ({
        file,
        status: "pending" as const,
        preview: URL.createObjectURL(file),
      }));

    setFiles((prev) => [...prev, ...newFiles]);
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
  }, []);

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      handleFileSelect(e.dataTransfer.files);
    },
    [handleFileSelect]
  );

  const removeFile = useCallback((index: number) => {
    setFiles((prev) => {
      const newFiles = [...prev];
      const removed = newFiles.splice(index, 1)[0];
      if (removed.preview) {
        URL.revokeObjectURL(removed.preview);
      }
      return newFiles;
    });
  }, []);

  const suggestTextureType = useCallback(
    (filename: string): string => {
      if (!autoDetectTypes) return "albedo";

      const name = filename.toLowerCase();

      for (const [type, patterns] of Object.entries(TEXTURE_TYPE_SUGGESTIONS)) {
        if (patterns.some((pattern) => name.includes(pattern))) {
          return type;
        }
      }

      return "albedo"; // Default fallback
    },
    [autoDetectTypes]
  );

  const handleImport = useCallback(async () => {
    if (files.length === 0) return;

    setIsImporting(true);
    setImportProgress(0);

    try {
      // Update files to processing status
      setFiles((prev) =>
        prev.map((f) => ({ ...f, status: "processing" as const }))
      );

      let textureIds: string[];

      if (isTauri() && files.every((f) => f.path)) {
        // Use Tauri native import method
        textureIds = await importTexturesFromDialog();
      } else {
        // Use web-based import method
        textureIds = await importTextures(files.map((f) => f.file));
      }

      // Update files with results
      setFiles((prev) =>
        prev.map((f, index) => ({
          ...f,
          status:
            index < textureIds.length
              ? ("success" as const)
              : ("error" as const),
          textureId: index < textureIds.length ? textureIds[index] : undefined,
          error: index >= textureIds.length ? "Import failed" : undefined,
        }))
      );

      setImportProgress(100);

      // Close dialog after a short delay
      setTimeout(() => {
        setOpen(false);
        setFiles([]);
        setImportProgress(0);
      }, 1500);
    } catch (error) {
      console.error("Import failed:", error);

      // Mark all files as failed
      setFiles((prev) =>
        prev.map((f) => ({
          ...f,
          status: "error" as const,
          error: error instanceof Error ? error.message : "Unknown error",
        }))
      );
    } finally {
      setIsImporting(false);
    }
  }, [files, importTextures, importTexturesFromDialog]);

  const handleClose = useCallback(() => {
    // Clean up object URLs
    files.forEach((f) => {
      if (f.preview) {
        URL.revokeObjectURL(f.preview);
      }
    });
    setFiles([]);
    setImportProgress(0);
    setOpen(false);
  }, [files]);

  const getStatusIcon = (status: ImportFile["status"]) => {
    switch (status) {
      case "success":
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case "error":
        return <AlertCircle className="h-4 w-4 text-red-500" />;
      case "processing":
        return (
          <div className="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent" />
        );
      default:
        return <FileImage className="h-4 w-4 text-muted-foreground" />;
    }
  };

  const getStatusColor = (status: ImportFile["status"]) => {
    switch (status) {
      case "success":
        return "text-green-600";
      case "error":
        return "text-red-600";
      case "processing":
        return "text-blue-600";
      default:
        return "text-muted-foreground";
    }
  };

  const totalSize = files.reduce((sum, f) => sum + f.file.size, 0);
  const processedCount = files.filter(
    (f) => f.status === "success" || f.status === "error"
  ).length;

  return (
    <Dialog onOpenChange={setOpen} open={open}>
      <DialogTrigger asChild>{children}</DialogTrigger>
      <DialogContent className="flex max-h-[80vh] max-w-4xl flex-col">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Upload className="h-5 w-5" />
            Import Textures
          </DialogTitle>
          <DialogDescription>
            Import texture files to your project. Supported formats: PNG, JPEG,
            WebP, TIFF, BMP, TGA, DDS, HDR, EXR
          </DialogDescription>
        </DialogHeader>

        <div className="flex min-h-0 flex-1 flex-col gap-4">
          {/* Settings */}
          <div className="flex items-center gap-4 rounded-lg border bg-muted/50 p-4">
            <div className="flex items-center gap-2">
              <Settings className="h-4 w-4" />
              <span className="font-medium text-sm">Import Settings</span>
            </div>
            <div className="flex items-center gap-2">
              <input
                checked={autoDetectTypes}
                className="rounded"
                id="auto-detect"
                onChange={(e) => setAutoDetectTypes(e.target.checked)}
                type="checkbox"
              />
              <label className="text-sm" htmlFor="auto-detect">
                Auto-detect texture types
              </label>
            </div>
          </div>

          {/* Drop zone */}
          <div
            className={cn(
              "rounded-lg border-2 border-dashed p-8 text-center transition-colors",
              files.length === 0
                ? "border-muted-foreground/25"
                : "border-muted",
              "hover:border-primary/50 hover:bg-primary/5"
            )}
            onDragOver={handleDragOver}
            onDrop={handleDrop}
          >
            <input
              accept={SUPPORTED_FORMATS.join(",")}
              className="hidden"
              multiple
              onChange={(e) => handleFileSelect(e.target.files)}
              ref={fileInputRef}
              type="file"
            />

            {files.length === 0 ? (
              <div>
                <Upload className="mx-auto mb-4 h-12 w-12 text-muted-foreground" />
                <h3 className="mb-2 font-medium text-lg">
                  Drop texture files here
                </h3>
                <p className="mb-4 text-muted-foreground">
                  or click to browse your files
                </p>
                <Button onClick={() => fileInputRef.current?.click()}>
                  <FolderOpen className="mr-2 h-4 w-4" />
                  Browse Files
                </Button>
              </div>
            ) : (
              <div>
                <FileImage className="mx-auto mb-2 h-8 w-8 text-muted-foreground" />
                <p className="text-muted-foreground text-sm">
                  {files.length} file{files.length !== 1 ? "s" : ""} selected (
                  {(totalSize / 1024 / 1024).toFixed(2)} MB)
                </p>
                <Button
                  className="mt-2"
                  onClick={() => fileInputRef.current?.click()}
                  size="sm"
                  variant="outline"
                >
                  Add More Files
                </Button>
              </div>
            )}
          </div>

          {/* File list */}
          {files.length > 0 && (
            <ScrollArea className="flex-1 rounded-lg border">
              <div className="space-y-2 p-4">
                {files.map((fileData, index) => (
                  <div
                    className="flex items-center gap-3 rounded-lg border bg-card p-3"
                    key={index}
                  >
                    {/* Preview */}
                    <div className="h-12 w-12 flex-shrink-0 overflow-hidden rounded border bg-muted">
                      {fileData.preview ? (
                        <img
                          alt={fileData.file.name}
                          className="h-full w-full object-cover"
                          src={fileData.preview}
                        />
                      ) : (
                        <div className="flex h-full w-full items-center justify-center">
                          <FileImage className="h-6 w-6 text-muted-foreground" />
                        </div>
                      )}
                    </div>

                    {/* File info */}
                    <div className="min-w-0 flex-1">
                      <div className="mb-1 flex items-center gap-2">
                        <span className="truncate font-medium">
                          {fileData.file.name}
                        </span>
                        {getStatusIcon(fileData.status)}
                      </div>

                      <div className="flex items-center gap-4 text-muted-foreground text-sm">
                        <span>{(fileData.file.size / 1024).toFixed(1)} KB</span>
                        {autoDetectTypes && (
                          <Badge className="text-xs" variant="outline">
                            {suggestTextureType(fileData.file.name)}
                          </Badge>
                        )}
                        <span
                          className={cn(
                            "capitalize",
                            getStatusColor(fileData.status)
                          )}
                        >
                          {fileData.status}
                        </span>
                      </div>

                      {fileData.error && (
                        <p className="mt-1 text-red-600 text-sm">
                          {fileData.error}
                        </p>
                      )}
                    </div>

                    {/* Remove button */}
                    {!isImporting && (
                      <Button
                        className="flex-shrink-0"
                        onClick={() => removeFile(index)}
                        size="sm"
                        variant="ghost"
                      >
                        <X className="h-4 w-4" />
                      </Button>
                    )}
                  </div>
                ))}
              </div>
            </ScrollArea>
          )}

          {/* Progress */}
          {isImporting && (
            <div className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span>Importing textures...</span>
                <span>
                  {processedCount} / {files.length}
                </span>
              </div>
              <Progress className="w-full" value={importProgress} />
            </div>
          )}
        </div>

        <DialogFooter>
          <Button
            disabled={isImporting}
            onClick={handleClose}
            variant="outline"
          >
            Cancel
          </Button>
          <Button
            disabled={
              files.length === 0 ||
              isImporting ||
              files.every((f) => f.status !== "pending")
            }
            onClick={handleImport}
          >
            {isImporting
              ? "Importing..."
              : `Import ${files.length} Texture${files.length !== 1 ? "s" : ""}`}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
