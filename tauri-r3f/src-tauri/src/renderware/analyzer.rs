use crate::renderware::versions::RenderWareVersionManager;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Seek, SeekFrom};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkHeader {
    pub chunk_type: u32,
    pub chunk_size: u32,
    pub rw_version: u32,
    pub offset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkNode {
    pub header: ChunkHeader,
    pub children: Vec<ChunkNode>,
    pub data_offset: u64,
    pub data_size: u64,
    pub display_name: String,
    pub is_corrupt: bool,
    pub corruption_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwAnalysis {
    pub file_path: String,
    pub file_size: u64,
    pub format: String,
    pub format_description: String,
    pub rw_version: u32,
    pub root_chunk: ChunkNode,
    pub total_chunks: usize,
    pub max_depth: usize,
    pub corruption_warnings: Vec<String>,
    pub analysis_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkExportResult {
    pub success: bool,
    pub file_path: String,
    pub chunk_type: String,
    pub data_size: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ChunkImportResult {
    pub success: bool,
    pub error: Option<String>,
}

pub struct RwAnalyzer {
    version_manager: RenderWareVersionManager,
}

impl RwAnalyzer {
    pub fn new() -> Self {
        Self {
            version_manager: RenderWareVersionManager::new(),
        }
    }

    pub fn with_version_manager(mut self, manager: RenderWareVersionManager) -> Self {
        self.version_manager = manager;
        self
    }

    pub fn analyze_file(&self, file_path: &str, data: &[u8]) -> Result<RwAnalysis, String> {
        let start_time = std::time::Instant::now();

        let file_size = data.len() as u64;

        // Detect file format and version
        let (format, format_description, rw_version) = self
            .version_manager
            .detect_file_format_version(data, file_path)?;

        // Parse the chunk tree
        let mut cursor = Cursor::new(data);
        let root_chunk = self.parse_chunk_tree(&mut cursor, file_size, 0)?;

        let total_chunks = self.count_chunks(&root_chunk);
        let max_depth = self.calculate_max_depth(&root_chunk);
        let corruption_warnings = self.collect_corruption_warnings(&root_chunk);

        let analysis_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(RwAnalysis {
            file_path: file_path.to_string(),
            file_size,
            format,
            format_description,
            rw_version: rw_version.try_into().unwrap_or(0),
            root_chunk,
            total_chunks,
            max_depth,
            corruption_warnings,
            analysis_time_ms,
        })
    }

    fn parse_chunk_tree(
        &self,
        cursor: &mut Cursor<&[u8]>,
        parent_limit: u64,
        depth: usize,
    ) -> Result<ChunkNode, String> {
        if cursor.position() + 12 > parent_limit {
            return Err("Not enough data for chunk header".to_string());
        }

        // Read chunk header (12 bytes)
        let mut header_bytes = [0u8; 12];
        cursor
            .read_exact(&mut header_bytes)
            .map_err(|e| format!("Failed to read chunk header: {}", e))?;

        let chunk_type = u32::from_le_bytes([
            header_bytes[0],
            header_bytes[1],
            header_bytes[2],
            header_bytes[3],
        ]);
        let chunk_size = u32::from_le_bytes([
            header_bytes[4],
            header_bytes[5],
            header_bytes[6],
            header_bytes[7],
        ]);
        let rw_version = u32::from_le_bytes([
            header_bytes[8],
            header_bytes[9],
            header_bytes[10],
            header_bytes[11],
        ]);

        let offset = cursor.position() - 12;

        let header = ChunkHeader {
            chunk_type,
            chunk_size,
            rw_version: rw_version.try_into().unwrap_or(0),
            offset,
        };

        let data_offset = cursor.position();
        let chunk_end = data_offset + chunk_size as u64;

        // Check for corruption
        let mut is_corrupt = false;
        let mut corruption_reason = None;

        if chunk_end > parent_limit {
            is_corrupt = true;
            corruption_reason = Some(format!(
                "Chunk ends at 0x{:X}, which is outside parent limit of 0x{:X}",
                chunk_end, parent_limit
            ));
        }

        // Generate display name
        let display_name = self.get_chunk_display_name(&header, cursor)?;

        // Parse children if this chunk has a valid size and we're not corrupted
        let mut children = Vec::new();
        if chunk_size >= 12 && !is_corrupt {
            let child_limit = chunk_end;
            while cursor.position() + 12 <= child_limit {
                match self.parse_chunk_tree(cursor, child_limit, depth + 1) {
                    Ok(child) => children.push(child),
                    Err(_) => {
                        is_corrupt = true;
                        corruption_reason = Some("Failed to parse child chunks".to_string());
                        break;
                    }
                }
            }
        }

        // Seek to end of chunk
        if chunk_end <= cursor.get_ref().len() as u64 {
            cursor
                .seek(SeekFrom::Start(chunk_end))
                .map_err(|e| format!("Failed to seek to chunk end: {}", e))?;
        }

        Ok(ChunkNode {
            header,
            children,
            data_offset,
            data_size: chunk_size as u64,
            display_name,
            is_corrupt,
            corruption_reason,
        })
    }

    fn get_chunk_display_name(
        &self,
        header: &ChunkHeader,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<String, String> {
        let chunk_name = self.get_chunk_type_name(header.chunk_type);

        // Special handling for ASSET chunks
        if header.chunk_type == 0x716 {
            // ASSET
            let current_pos = cursor.position();

            // Try to read asset name
            if cursor.get_ref().len() >= current_pos as usize + 8 {
                // Skip some bytes to get to name
                cursor
                    .seek(SeekFrom::Start(current_pos + 4))
                    .map_err(|e| format!("Failed to seek for asset name: {}", e))?;

                let mut name_size_bytes = [0u8; 4];
                if cursor.read_exact(&mut name_size_bytes).is_ok() {
                    let name_size = u32::from_le_bytes(name_size_bytes) as usize;

                    if name_size > 0
                        && name_size < 256
                        && cursor.get_ref().len() >= cursor.position() as usize + name_size
                    {
                        let mut name_bytes = vec![0u8; name_size];
                        cursor
                            .read_exact(&mut name_bytes)
                            .map_err(|e| format!("Failed to read asset name: {}", e))?;

                        // Convert to string, cleaning up null terminators and invalid chars
                        let name = String::from_utf8_lossy(&name_bytes)
                            .trim_end_matches('\0')
                            .chars()
                            .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
                            .collect::<String>();

                        if !name.is_empty() {
                            // Restore cursor position
                            cursor
                                .seek(SeekFrom::Start(current_pos))
                                .map_err(|e| format!("Failed to restore cursor: {}", e))?;
                            return Ok(format!("ASSET: {}", name.trim()));
                        }
                    }
                }
            }

            // Restore cursor position
            cursor
                .seek(SeekFrom::Start(current_pos))
                .map_err(|e| format!("Failed to restore cursor: {}", e))?;
        }

        Ok(chunk_name)
    }

    fn get_chunk_type_name(&self, chunk_type: u32) -> String {
        match chunk_type {
            0x0 => "NAOBJECT".to_string(),
            0x1 => "STRUCT".to_string(),
            0x2 => "STRING".to_string(),
            0x3 => "EXTENSION".to_string(),
            0x5 => "CAMERA".to_string(),
            0x6 => "TEXTURE".to_string(),
            0x7 => "MATERIAL".to_string(),
            0x8 => "MATLIST".to_string(),
            0x9 => "ATOMICSECT".to_string(),
            0xA => "PLANESECT".to_string(),
            0xB => "WORLD".to_string(),
            0xC => "SPLINE".to_string(),
            0xD => "MATRIX".to_string(),
            0xE => "FRAMELIST".to_string(),
            0xF => "GEOMETRY".to_string(),
            0x10 => "CLUMP".to_string(),
            0x12 => "LIGHT".to_string(),
            0x13 => "UNICODESTRING".to_string(),
            0x14 => "ATOMIC".to_string(),
            0x15 => "TEXTURENATIVE".to_string(),
            0x16 => "TEXDICTIONARY".to_string(),
            0x17 => "ANIMDATABASE".to_string(),
            0x18 => "IMAGE".to_string(),
            0x19 => "SKINANIMATION".to_string(),
            0x1A => "GEOMETRYLIST".to_string(),
            0x1B => "ANIMANIMATION".to_string(),
            0x1C => "TEAM".to_string(),
            0x1D => "CROWD".to_string(),
            0x1F => "RIGHTTORENDER".to_string(),
            0x20 => "MTEFFECTNATIVE".to_string(),
            0x21 => "MTEFFECTDICT".to_string(),
            0x22 => "TEAMDICTIONARY".to_string(),
            0x23 => "PITEXDICTIONARY".to_string(),
            0x24 => "TOC".to_string(),
            0x25 => "PRTSTDGLOBALDATA".to_string(),
            0x26 => "ALTPIPE".to_string(),
            0x27 => "PIPEDS".to_string(),
            0x28 => "PATCHMESH".to_string(),
            0x29 => "CHUNKGROUPSTART".to_string(),
            0x2A => "CHUNKGROUPEND".to_string(),
            0x2B => "UVANIMDICT".to_string(),
            0x2D => "ENVIRONMENT".to_string(),
            0x50E => "BINMESH".to_string(),
            0x510 => "NATIVEDATA".to_string(),
            0x704 => "SCRIPT".to_string(),
            0x716 => "ASSET".to_string(),
            0x71C => "CONTAINER".to_string(),
            0x253F2F3 => "PIPELINESET".to_string(),
            0x253F2F6 => "SPECULARMAT".to_string(),
            0x253F2F8 => "CHUNK_2DFX".to_string(),
            0x253F2F9 => "NIGHTVERTEXCOLOR".to_string(),
            0x253F2FA => "COLLISIONMODEL".to_string(),
            0x253F2FC => "REFLECTIONMAT".to_string(),
            0x253F2FD => "MESHEXTENSION".to_string(),
            0x253F2FE => "FRAME".to_string(),
            _ => format!("UNKNOWN(0x{:X})", chunk_type),
        }
    }

    fn count_chunks(&self, node: &ChunkNode) -> usize {
        1 + node
            .children
            .iter()
            .map(|child| self.count_chunks(child))
            .sum::<usize>()
    }

    fn calculate_max_depth(&self, node: &ChunkNode) -> usize {
        if node.children.is_empty() {
            1
        } else {
            1 + node
                .children
                .iter()
                .map(|child| self.calculate_max_depth(child))
                .max()
                .unwrap_or(0)
        }
    }

    fn collect_corruption_warnings(&self, node: &ChunkNode) -> Vec<String> {
        let mut warnings = Vec::new();

        if node.is_corrupt {
            if let Some(reason) = &node.corruption_reason {
                warnings.push(format!(
                    "{} (offset 0x{:X}): {}",
                    node.display_name, node.header.offset, reason
                ));
            }
        }

        for child in &node.children {
            warnings.extend(self.collect_corruption_warnings(child));
        }

        warnings
    }

    pub fn export_chunk(
        &self,
        data: &[u8],
        chunk_offset: u64,
        include_header: bool,
    ) -> Result<Vec<u8>, String> {
        if chunk_offset as usize + 12 > data.len() {
            return Err("Invalid chunk offset".to_string());
        }

        let header_start = chunk_offset as usize;
        let chunk_size = u32::from_le_bytes([
            data[header_start + 4],
            data[header_start + 5],
            data[header_start + 6],
            data[header_start + 7],
        ]) as usize;

        let data_start = if include_header {
            header_start
        } else {
            header_start + 12
        };

        let data_end = header_start + 12 + chunk_size;

        if data_end > data.len() {
            return Err("Chunk data extends beyond file".to_string());
        }

        Ok(data[data_start..data_end].to_vec())
    }

    pub fn export_chunk_to_file(
        &self,
        data: &[u8],
        chunk_offset: u64,
        output_path: &str,
        include_header: bool,
    ) -> Result<ChunkExportResult, String> {
        let chunk_data = self.export_chunk(data, chunk_offset, include_header)?;

        std::fs::write(output_path, &chunk_data)
            .map_err(|e| format!("Failed to write chunk to file: {}", e))?;

        // Get chunk type name for the result
        if chunk_offset as usize + 4 > data.len() {
            return Err("Invalid chunk offset for type detection".to_string());
        }

        let chunk_type = u32::from_le_bytes([
            data[chunk_offset as usize],
            data[chunk_offset as usize + 1],
            data[chunk_offset as usize + 2],
            data[chunk_offset as usize + 3],
        ]);

        let chunk_type_name = self.get_chunk_type_name(chunk_type);

        Ok(ChunkExportResult {
            success: true,
            file_path: output_path.to_string(),
            chunk_type: chunk_type_name,
            data_size: chunk_data.len(),
            error: None,
        })
    }

    pub fn import_chunk_payload(
        &self,
        original_data: &[u8],
        chunk_offset: u64,
        new_payload: &[u8],
    ) -> Result<Vec<u8>, String> {
        if chunk_offset as usize + 12 > original_data.len() {
            return Err("Invalid chunk offset".to_string());
        }

        let header_start = chunk_offset as usize;
        let payload_start = header_start + 12;

        // Create new header with updated size
        let new_size = new_payload.len() as u32;
        let mut new_header = [0u8; 12];
        new_header[0..4].copy_from_slice(&original_data[header_start..header_start + 4]); // type
        new_header[4..8].copy_from_slice(&new_size.to_le_bytes()); // new size
        new_header[8..12].copy_from_slice(&original_data[header_start + 8..header_start + 12]); // version

        // Build new file data
        let before_chunk = &original_data[0..header_start];
        let after_chunk_start = payload_start
            + (u32::from_le_bytes([
                original_data[header_start + 4],
                original_data[header_start + 5],
                original_data[header_start + 6],
                original_data[header_start + 7],
            ]) as usize);

        if after_chunk_start > original_data.len() {
            return Err("Original chunk size calculation error".to_string());
        }

        let after_chunk = &original_data[after_chunk_start..];

        let mut result = Vec::new();
        result.extend_from_slice(before_chunk);
        result.extend_from_slice(&new_header);
        result.extend_from_slice(new_payload);
        result.extend_from_slice(after_chunk);

        Ok(result)
    }
}

impl Default for RwAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
