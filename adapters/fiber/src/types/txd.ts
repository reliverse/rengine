/**
 * TXD Texture Archive Types for Rengine Integration
 * Based on RenderWare TXD (Texture Dictionary) format
 */

export type TextureFormat =
  | "RGBA32"
  | "RGBA16"
  | "Luminance8"
  | "LuminanceAlpha8"
  | "Palette4"
  | "Palette8"
  | "Compressed";

export type TextureFilterMode =
  | "Nearest"
  | "Linear"
  | "MipNearest"
  | "MipLinear"
  | "LinearMipNearest"
  | "LinearMipLinear";

export type TextureAddressingMode = "Wrap" | "Mirror" | "Clamp" | "Border";

export interface TextureInfo {
  name: string;
  width: number;
  height: number;
  depth: number;
  format: TextureFormat;
  mipmap_count: number;
  raster_type: number;
  filter_mode: TextureFilterMode;
  addressing_u: TextureAddressingMode;
  addressing_v: TextureAddressingMode;
  data_size: number;
  data_offset: number;
  renderware_version?: string;
  platform_flags: number;
}

export interface TxdArchive {
  file_path: string;
  textures: TextureInfo[];
  total_textures: number;
  renderware_version?: string;
}

export interface TxdStatistics {
  total_textures: number;
  total_size_bytes: number;
  average_width: number;
  average_height: number;
  format_counts: Record<string, number>;
  renderware_version?: string;
}

// Filter and search types
export type TxdFilterType = "all" | "compressed" | "palette" | "rgba" | "other";

export const TxdFilterTypeValues = {
  ALL: "all" as const,
  COMPRESSED: "compressed" as const,
  PALETTE: "palette" as const,
  RGBA: "rgba" as const,
  OTHER: "other" as const,
} as const;

export type TxdSortField = "name" | "size" | "width" | "height" | "format";

export const TxdSortFieldValues = {
  NAME: "name" as const,
  SIZE: "size" as const,
  WIDTH: "width" as const,
  HEIGHT: "height" as const,
  FORMAT: "format" as const,
} as const;

export type TxdSortDirection = "asc" | "desc";

export const TxdSortDirectionValues = {
  ASC: "asc" as const,
  DESC: "desc" as const,
} as const;

export interface TxdFilterOptions {
  filter_type: TxdFilterType;
  search_text: string;
  sort_field: TxdSortField;
  sort_direction: TxdSortDirection;
  min_width?: number;
  max_width?: number;
  min_height?: number;
  max_height?: number;
  format_filter?: TextureFormat[];
}

// Tab management types
export interface TxdArchiveTab {
  id: string;
  archive: TxdArchive;
  name: string; // Display name (filename)
  is_modified: boolean;
  filter_options: TxdFilterOptions;
  selected_textures: string[]; // Array of texture names
  preview_texture?: string; // Currently previewed texture name
}

// Export/import operations
export interface TxdExportOptions {
  archive_path: string;
  textures: string[]; // Texture names to export
  output_directory: string;
  format: "png" | "dds" | "tga";
  create_subdirectories: boolean;
  overwrite_existing: boolean;
}

export interface TxdImportOptions {
  archive_path: string;
  texture_files: File[]; // Files to import as textures
  compression_format?: TextureFormat;
  generate_mipmaps: boolean;
  update_existing: boolean;
}

// Texture preview and editing
export interface TexturePreview {
  texture_name: string;
  image_data: string; // Base64 encoded image data
  width: number;
  height: number;
  format: TextureFormat;
  mipmap_levels: number;
}

export interface TextureEditOptions {
  name?: string;
  filter_mode?: TextureFilterMode;
  addressing_u?: TextureAddressingMode;
  addressing_v?: TextureAddressingMode;
  compression_format?: TextureFormat;
}

// UI State types
export interface TxdEditorState {
  tabs: TxdArchiveTab[];
  active_tab_id: string | null;
  is_loading: boolean;
  error_message?: string;
  global_filter_options: TxdFilterOptions;
  preview_loading: boolean;
  current_preview?: TexturePreview;
}

// Batch operations
export interface BatchTextureOperation {
  texture_name: string;
  operation_type: "export" | "replace" | "delete" | "rename";
  output_path?: string;
  new_name?: string;
  replacement_file?: File;
}

export interface BatchTextureResult {
  texture_name: string;
  success: boolean;
  error?: string;
  operation_type: string;
}
