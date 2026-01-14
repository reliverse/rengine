import { Upload, X } from "lucide-react";
import { Button } from "~/components/ui/button";
import { Card } from "~/components/ui/card";
import type { ImportedAsset } from "../../types";

interface ImportedAssetsSectionProps {
  importedAssets: Map<string, ImportedAsset>;
  expandedAssets: Set<string>;
  onToggleAssetExpansion: (assetId: string) => void;
  onRemoveAsset: (assetId: string) => void;
  onAddToScene: (asset: ImportedAsset) => void;
}

export function ImportedAssetsSection({
  importedAssets,
  expandedAssets,
  onToggleAssetExpansion,
  onRemoveAsset,
  onAddToScene,
}: ImportedAssetsSectionProps) {
  if (importedAssets.size === 0) {
    return null;
  }

  return (
    <div className="mb-4">
      <h4 className="mb-2 font-medium text-sm">Imported Assets</h4>
      <div className="space-y-2">
        {Array.from(importedAssets.entries()).map(([assetId, asset]) => (
          <Card className="p-3" key={assetId}>
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Button
                  className="h-6 w-6 p-0"
                  onClick={() => onToggleAssetExpansion(assetId)}
                  size="sm"
                  variant="ghost"
                >
                  {expandedAssets.has(assetId) ? "▼" : "▶"}
                </Button>
                <div>
                  <div className="flex items-center gap-2 font-medium text-sm">
                    <span className="rounded bg-primary/10 px-1 py-0.5 text-primary text-xs uppercase">
                      {asset.type}
                    </span>
                    {asset.file_path.split("/").pop()?.split("\\").pop()}
                    {asset.samp_model_id && (
                      <span className="rounded bg-green-100 px-1 py-0.5 font-bold text-green-700 text-xs">
                        ID: {asset.samp_model_id}
                      </span>
                    )}
                  </div>
                  <div className="text-muted-foreground text-xs">
                    {asset.type === "dff" && (
                      <>
                        {asset.frame_count} frames • {asset.geometry_count}{" "}
                        geometries • {asset.atomic_count} atomics
                        {asset.samp_model_name && (
                          <> • SA:MP: {asset.samp_model_name}</>
                        )}
                      </>
                    )}
                    {asset.type === "txd" && (
                      <>
                        {asset.texture_count} textures
                        {asset.samp_model_name && (
                          <> • SA:MP: {asset.samp_model_name}</>
                        )}
                      </>
                    )}
                    {asset.type === "col" && (
                      <>
                        {asset.model_count} models • {asset.version}
                        {asset.samp_model_name && (
                          <> • SA:MP: {asset.samp_model_name}</>
                        )}
                      </>
                    )}
                    {asset.type === "ipl" && (
                      <>
                        {asset.instance_count} instances • {asset.zone_count}{" "}
                        zones
                        {asset.samp_model_name && (
                          <> • SA:MP: {asset.samp_model_name}</>
                        )}
                      </>
                    )}
                  </div>
                </div>
              </div>
              <Button
                className="mr-1 h-6 w-6 p-0"
                onClick={() => onAddToScene(asset)}
                size="sm"
                title="Add to Scene"
                variant="ghost"
              >
                <Upload className="h-3 w-3" />
              </Button>
              <Button
                className="h-6 w-6 p-0"
                onClick={() => onRemoveAsset(assetId)}
                size="sm"
                title="Remove Asset"
                variant="ghost"
              >
                <X className="h-3 w-3" />
              </Button>
            </div>

            {expandedAssets.has(assetId) && (
              <div className="mt-3">
                <div className="space-y-1 text-xs">
                  <div className="flex justify-between">
                    <span>File:</span>
                    <span className="font-mono text-muted-foreground">
                      {asset.file_path}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span>Imported:</span>
                    <span className="text-muted-foreground">
                      {new Date(asset.loaded_at).toLocaleString()}
                    </span>
                  </div>

                  {asset.type === "dff" && (
                    <>
                      <div className="flex justify-between">
                        <span>RW Version:</span>
                        <span className="text-muted-foreground">
                          {asset.rw_version}
                        </span>
                      </div>
                      <div className="flex justify-between">
                        <span>Materials:</span>
                        <span className="text-muted-foreground">
                          {asset.material_count}
                        </span>
                      </div>
                      <div className="flex justify-between">
                        <span>Textures:</span>
                        <span className="text-muted-foreground">
                          {asset.texture_count}
                        </span>
                      </div>
                      {asset.samp_model_id && (
                        <>
                          <div className="flex justify-between">
                            <span>SA:MP ID:</span>
                            <span className="font-bold text-green-600">
                              {asset.samp_model_id}
                            </span>
                          </div>
                          <div className="flex justify-between">
                            <span>SA:MP Name:</span>
                            <span className="text-green-600">
                              {asset.samp_model_name}
                            </span>
                          </div>
                        </>
                      )}
                    </>
                  )}

                  {asset.type === "txd" && (
                    <>
                      {asset.samp_model_id && (
                        <>
                          <div className="flex justify-between">
                            <span>SA:MP ID:</span>
                            <span className="font-bold text-green-600">
                              {asset.samp_model_id}
                            </span>
                          </div>
                          <div className="flex justify-between">
                            <span>SA:MP Name:</span>
                            <span className="text-green-600">
                              {asset.samp_model_name}
                            </span>
                          </div>
                        </>
                      )}
                      {asset.textures.length > 0 && (
                        <div className="mt-2">
                          <div className="mb-1 font-medium">Textures:</div>
                          <div className="max-h-32 space-y-1 overflow-y-auto">
                            {asset.textures.slice(0, 10).map((texture) => (
                              <div
                                className="flex justify-between text-xs"
                                key={`${assetId}_texture_${texture.name}`}
                              >
                                <span className="font-mono">
                                  {texture.name}
                                </span>
                                <span className="text-muted-foreground">
                                  {texture.width}×{texture.height}{" "}
                                  {texture.format}
                                </span>
                              </div>
                            ))}
                            {asset.textures.length > 10 && (
                              <div className="py-1 text-center text-muted-foreground text-xs">
                                ... and {asset.textures.length - 10} more
                                textures
                              </div>
                            )}
                          </div>
                        </div>
                      )}
                    </>
                  )}

                  {asset.type === "col" && (
                    <>
                      {asset.samp_model_id && (
                        <>
                          <div className="flex justify-between">
                            <span>SA:MP ID:</span>
                            <span className="font-bold text-green-600">
                              {asset.samp_model_id}
                            </span>
                          </div>
                          <div className="flex justify-between">
                            <span>SA:MP Name:</span>
                            <span className="text-green-600">
                              {asset.samp_model_name}
                            </span>
                          </div>
                        </>
                      )}
                      {asset.models.length > 0 && (
                        <div className="mt-2">
                          <div className="mb-1 font-medium">Models:</div>
                          <div className="max-h-32 space-y-1 overflow-y-auto">
                            {asset.models.slice(0, 10).map((model) => (
                              <div
                                className="flex justify-between text-xs"
                                key={`${assetId}_model_${model.name}`}
                              >
                                <span className="font-mono">{model.name}</span>
                                <span className="text-muted-foreground">
                                  {model.vertex_count} verts •{" "}
                                  {model.face_count} faces
                                </span>
                              </div>
                            ))}
                            {asset.models.length > 10 && (
                              <div className="py-1 text-center text-muted-foreground text-xs">
                                ... and {asset.models.length - 10} more models
                              </div>
                            )}
                          </div>
                        </div>
                      )}
                    </>
                  )}

                  {asset.type === "ipl" && (
                    <>
                      {asset.samp_model_id && (
                        <>
                          <div className="flex justify-between">
                            <span>SA:MP ID:</span>
                            <span className="font-bold text-green-600">
                              {asset.samp_model_id}
                            </span>
                          </div>
                          <div className="flex justify-between">
                            <span>SA:MP Name:</span>
                            <span className="text-green-600">
                              {asset.samp_model_name}
                            </span>
                          </div>
                        </>
                      )}
                      <div className="flex justify-between">
                        <span>Zones:</span>
                        <span className="text-muted-foreground">
                          {asset.zone_count}
                        </span>
                      </div>
                      <div className="flex justify-between">
                        <span>Cull zones:</span>
                        <span className="text-muted-foreground">
                          {asset.cull_count}
                        </span>
                      </div>
                      <div className="flex justify-between">
                        <span>Pick zones:</span>
                        <span className="text-muted-foreground">
                          {asset.pick_count}
                        </span>
                      </div>
                    </>
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
