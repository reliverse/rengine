// Asset types supported by the engine
export type AssetType =
  | "models"
  | "textures"
  | "materials"
  | "audio"
  | "scripts"
  | "fonts"
  | "prefabs"
  | "renderware"
  | "remote";

export interface AssetsPanelProps {
  className?: string;
}

// IMG Archive interface
export interface ImgArchive {
  path: string;
  entries: Array<{
    name: string;
    size: number;
    offset: number;
    is_compressed?: boolean;
    rw_version?: string;
  }>;
  version: string;
  total_entries: number;
  file_size: number;
}

// Imported asset interfaces
export interface ImportedDffAsset {
  type: "dff";
  file_path: string;
  rw_version: number;
  frame_count: number;
  geometry_count: number;
  atomic_count: number;
  material_count?: number;
  texture_count?: number;
  samp_model_id?: number | null;
  samp_model_name?: string | null;
  loaded_at: number;
}

export interface ImportedTxdAsset {
  type: "txd";
  file_path: string;
  texture_count: number;
  textures: Array<{
    name: string;
    width: number;
    height: number;
    format: string;
  }>;
  renderware_version?: string;
  samp_model_id?: number | null;
  samp_model_name?: string | null;
  loaded_at: number;
}

export interface ImportedColAsset {
  type: "col";
  file_path: string;
  version: string;
  model_count: number;
  models: Array<{
    name: string;
    face_count: number;
    vertex_count: number;
  }>;
  samp_model_id?: number | null;
  samp_model_name?: string | null;
  loaded_at: number;
}

export interface ImportedIplAsset {
  type: "ipl";
  file_path: string;
  instance_count: number;
  zone_count: number;
  cull_count: number;
  pick_count: number;
  samp_model_id?: number | null;
  samp_model_name?: string | null;
  loaded_at: number;
}

export type ImportedAsset =
  | ImportedDffAsset
  | ImportedTxdAsset
  | ImportedColAsset
  | ImportedIplAsset;

// Model card props
export interface ModelCardProps {
  model: any;
  isSelected: boolean;
  showThumbnail: boolean;
  onClick: (e: React.MouseEvent) => void;
  onDelete: () => void;
}

// Audio card props
export interface AudioCardProps {
  audio: any;
  isSelected: boolean;
  showWaveform: boolean;
  currentlyPlaying: string | null;
  onClick: (e: React.MouseEvent) => void;
  onPlayPause: (e: React.MouseEvent) => void;
  onDelete: () => void;
}
