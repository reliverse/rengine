import { Button } from "~/components/ui/button";
import { Card } from "~/components/ui/card";
import type { ModelSearchResult } from "../hooks/use-renderware-state";

interface ModelSearchResultsProps {
  modelSearchResults: ModelSearchResult[];
}

export function ModelSearchResults({
  modelSearchResults,
}: ModelSearchResultsProps) {
  if (modelSearchResults.length === 0) {
    return null;
  }

  return (
    <div className="mb-4">
      <h4 className="mb-2 font-medium text-sm">
        SA:MP Model Search Results ({modelSearchResults.length})
      </h4>
      <div className="max-h-64 space-y-2 overflow-y-auto">
        {modelSearchResults.map((model) => (
          <Card className="p-3" key={model.id}>
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <div className="font-medium text-primary text-sm">
                  ID: {model.id}
                </div>
                <div className="font-mono text-sm">{model.name}</div>
              </div>
              <div className="flex items-center gap-2">
                <div className="text-muted-foreground text-xs">
                  Radius: {model.radius}
                </div>
                <Button
                  onClick={() => {
                    navigator.clipboard?.writeText(model.id.toString());
                    console.log(`Copied model ID ${model.id} to clipboard`);
                  }}
                  size="sm"
                  variant="outline"
                >
                  Copy ID
                </Button>
              </div>
            </div>
            <div className="mt-2 text-muted-foreground text-xs">
              <div>DFF: {model.dff}</div>
              <div>TXD: {model.txd}</div>
            </div>
          </Card>
        ))}
      </div>
    </div>
  );
}
