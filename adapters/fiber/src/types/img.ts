/**
 * IMG Archive Types for Rengine Integration
 * Based on GTA IMG archive format (v1 and v2)
 */

export type ImgVersion = "V1" | "V2";

export interface ImgEntry {
  offset: number;
  size: number;
  name: string;
  renderware_version?: string; // Will be populated by version detection
  file_type?: string; // DFF, TXD, COL, IPL, IDE, etc.
}

export interface ImgArchive {
  version: ImgVersion;
  file_path: string;
  entries: ImgEntry[];
  total_entries: number;
}

export interface ImgArchiveInfo {
  version: string; // Human-readable version string
  total_entries: number;
  file_path: string;
  type_counts: Record<string, number>; // File type counts (DFF: 123, TXD: 456, etc.)
  rw_version_counts: Record<string, number>; // RW version counts
}

export interface ImgOperation {
  entry_name: string;
  output_path: string;
}

export interface OperationResult {
  entry_name: string;
  success: boolean;
  error?: string;
}

// Filter and search types
export type ImgFilterType =
  | "all"
  | "dff"
  | "txd"
  | "col"
  | "ipl"
  | "ide"
  | "other";

export const ImgFilterTypeValues = {
  ALL: "all" as const,
  DFF: "dff" as const,
  TXD: "txd" as const,
  COL: "col" as const,
  IPL: "ipl" as const,
  IDE: "ide" as const,
  OTHER: "other" as const,
} as const;

export type ImgSortField = "name" | "size" | "offset" | "type" | "rw_version";

export const ImgSortFieldValues = {
  NAME: "name" as const,
  SIZE: "size" as const,
  OFFSET: "offset" as const,
  TYPE: "type" as const,
  RW_VERSION: "rw_version" as const,
} as const;

export type ImgSortDirection = "asc" | "desc";

export const ImgSortDirectionValues = {
  ASC: "asc" as const,
  DESC: "desc" as const,
} as const;

export interface ImgFilterOptions {
  filter_type: ImgFilterType;
  search_text: string;
  sort_field: ImgSortField;
  sort_direction: ImgSortDirection;
  rw_version_filter?: string[]; // Array of RW version strings to filter by
}

// Tab management types
export interface ImgArchiveTab {
  id: string;
  archive: ImgArchive;
  name: string; // Display name (filename)
  is_modified: boolean;
  filter_options: ImgFilterOptions;
  selected_entries: string[]; // Array of entry names
}

// Batch operations
export interface BatchExtractOptions {
  archive_path: string;
  operations: ImgOperation[];
  create_subdirectories?: boolean; // Create subdirs based on file type
  overwrite_existing?: boolean;
}

export interface BatchExtractProgress {
  total: number;
  completed: number;
  current_entry?: string;
  errors: OperationResult[];
}

// Export/import operations
export interface ImgExportOptions {
  archive_path: string;
  entries: string[]; // Entry names to export
  output_directory: string;
  create_subdirectories: boolean;
  overwrite_existing: boolean;
}

export interface ImgImportOptions {
  archive_path: string;
  files: File[]; // Files to import
  compression_level?: number; // For future use
  update_existing?: boolean;
}

// RW Version filtering
export interface RwVersionFilter {
  versions: string[];
  games: string[];
  platforms: string[];
  enabled: boolean;
}

// UI State types
export interface ImgEditorState {
  tabs: ImgArchiveTab[];
  active_tab_id: string | null;
  is_loading: boolean;
  error_message?: string;
  global_filter_options: ImgFilterOptions;
  rw_version_filters: RwVersionFilter;
  batch_operation_in_progress: boolean;
  batch_progress?: BatchExtractProgress;
}
