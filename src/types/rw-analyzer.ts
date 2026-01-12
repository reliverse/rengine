export interface ChunkHeader {
  chunk_type: number;
  chunk_size: number;
  rw_version: number;
  offset: number;
}

export interface ChunkNode {
  header: ChunkHeader;
  children: ChunkNode[];
  data_offset: number;
  data_size: number;
  display_name: string;
  is_corrupt: boolean;
  corruption_reason?: string;
}

export interface RwAnalysis {
  file_path: string;
  file_size: number;
  format: string;
  format_description: string;
  rw_version: number;
  root_chunk: ChunkNode;
  total_chunks: number;
  max_depth: number;
  corruption_warnings: string[];
  analysis_time_ms: number;
}

export interface ChunkExportResult {
  success: boolean;
  file_path: string;
  chunk_type: string;
  data_size: number;
  error?: string;
}

export interface ChunkImportResult {
  success: boolean;
  error?: string;
}

export interface RwAnalyzerState {
  currentFile: string | null;
  currentAnalysis: RwAnalysis | null;
  isLoading: boolean;
  selectedChunkPath: number[] | null; // Array of indices representing path to chunk
  expandedChunks: Set<string>; // Set of chunk paths that are expanded
}

// Tree node for the chunk tree visualization
export interface ChunkTreeNode {
  id: string;
  name: string;
  header: ChunkHeader;
  children: ChunkTreeNode[];
  level: number;
  isExpanded: boolean;
  isSelected: boolean;
  isCorrupt: boolean;
  hasChildren: boolean;
}

// Context menu actions for chunks
export interface ChunkContextAction {
  label: string;
  action:
    | "export_full"
    | "export_payload"
    | "import_payload"
    | "show_properties";
  enabled: boolean;
}

// Properties display for a chunk
export interface ChunkProperties {
  chunkType: string;
  rawTypeId: string;
  payloadSize: string;
  rwVersion: string;
  rawVersion: string;
  fileOffset: string;
  endOfChunk: string;
}
