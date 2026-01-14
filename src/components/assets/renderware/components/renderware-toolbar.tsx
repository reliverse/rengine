import { Code, Search, Settings, Upload } from "lucide-react";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { useArchiveOperations } from "../hooks/use-archive-operations";
import { useAssetImport } from "../hooks/use-asset-import";

interface RenderWareToolbarProps {
  isImporting: boolean;
  setIsImporting: (importing: boolean) => void;
  modelSearchQuery: string;
  isSearchingModels: boolean;
  onAssetImported: (asset: any) => void;
  onModelSearchInput: (value: string) => void;
  setLoadedArchives: React.Dispatch<React.SetStateAction<Map<string, any>>>;
}

export function RenderWareToolbar({
  isImporting,
  setIsImporting,
  modelSearchQuery,
  isSearchingModels,
  onAssetImported,
  onModelSearchInput,
  setLoadedArchives,
}: RenderWareToolbarProps) {
  const { loadImgArchive } = useArchiveOperations();
  const { importIndividualFile, handlePwnImport, importViaIde } =
    useAssetImport();

  const handleLoadImgArchive = async () => {
    try {
      setIsImporting(true);
      const filePath = await open({
        filters: [{ name: "IMG Archive", extensions: ["img"] }],
      });
      if (filePath && typeof filePath === "string") {
        const archive = await loadImgArchive(filePath);
        setLoadedArchives((prev) => new Map(prev).set(filePath, archive));
        console.log("Archive loaded:", archive);
      }
    } catch (error) {
      console.error("IMG archive load error:", error);
    } finally {
      setIsImporting(false);
    }
  };

  return (
    <div className="mb-4 flex flex-wrap gap-2">
      {/* SA:MP Model Search */}
      <div className="relative min-w-64 flex-1">
        <Search className="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 transform text-muted-foreground" />
        <Input
          className="pl-9"
          onChange={(e) => onModelSearchInput(e.target.value)}
          placeholder="Search SA:MP models by name..."
          value={modelSearchQuery}
        />
        {isSearchingModels && (
          <div className="absolute top-1/2 right-3 -translate-y-1/2">
            <div className="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent" />
          </div>
        )}
      </div>

      <Button
        disabled={isImporting}
        onClick={handleLoadImgArchive}
        size="sm"
        variant="outline"
      >
        <Upload className="mr-2 h-4 w-4" />
        Load IMG Archive
      </Button>

      <Button
        disabled={isImporting}
        onClick={() =>
          importIndividualFile("dff", onAssetImported, setIsImporting)
        }
        size="sm"
        variant="outline"
      >
        Import DFF
      </Button>

      <Button
        disabled={isImporting}
        onClick={() =>
          importIndividualFile("txd", onAssetImported, setIsImporting)
        }
        size="sm"
        variant="outline"
      >
        Import TXD
      </Button>

      <Button
        disabled={isImporting}
        onClick={() =>
          importIndividualFile("col", onAssetImported, setIsImporting)
        }
        size="sm"
        variant="outline"
      >
        Import COL
      </Button>

      <Button
        disabled={isImporting}
        onClick={() =>
          importIndividualFile("ipl", onAssetImported, setIsImporting)
        }
        size="sm"
        variant="outline"
      >
        Import IPL
      </Button>

      <Button
        disabled={isImporting}
        onClick={() => importViaIde(setIsImporting)}
        size="sm"
        variant="outline"
      >
        <Settings className="mr-2 h-4 w-4" />
        IDE Import
      </Button>

      <Button
        disabled={isImporting}
        onClick={() => handlePwnImport(setIsImporting)}
        size="sm"
        variant="outline"
      >
        <Code className="mr-2 h-4 w-4" />
        Import PWN
      </Button>
    </div>
  );
}
