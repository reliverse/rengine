use crate::RengineError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

// IMG archive sector size (standard for GTA games)
// All offsets and sizes in IMG entries are stored in sectors, not bytes
const SECTOR_SIZE: u32 = 2048;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ImgVersion {
    V1, // GTA III, Vice City
    V2, // GTA San Andreas
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgEntry {
    pub offset: u32,
    pub size: u32,
    pub name: String,
    pub renderware_version: Option<String>, // Will be populated by version detection
    pub file_type: Option<String>,          // DFF, TXD, COL, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgArchive {
    pub version: ImgVersion,
    pub file_path: String,
    pub entries: Vec<ImgEntry>,
    pub total_entries: usize,
    pub file_size: u64,
    // Modification tracking
    pub modified: bool,
    pub deleted_entries: Vec<ImgEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub entry_name: String,
    pub success: bool,
    pub error: Option<String>,
}

impl ImgArchive {
    /// Load and parse an IMG archive from file path
    pub fn load_from_path(path: &str) -> Result<Self, RengineError> {
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(RengineError::FileReadFailed {
                path: path.to_string(),
                details: "File does not exist".to_string(),
            });
        }

        let mut file = File::open(path_obj).map_err(|e| RengineError::FileReadFailed {
            path: path.to_string(),
            details: e.to_string(),
        })?;

        Self::parse_archive(&mut file, path)
    }

    /// Parse IMG archive from an open file handle
    pub fn parse_archive(file: &mut File, path: &str) -> Result<Self, RengineError> {
        // Read the entire file to determine version and parse entries
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| RengineError::FileReadFailed {
                path: path.to_string(),
                details: e.to_string(),
            })?;

        if buffer.len() < 8 {
            return Err(RengineError::FileReadFailed {
                path: path.to_string(),
                details: "File too small to be a valid IMG archive".to_string(),
            });
        }

        // Detect version based on file structure
        let version = Self::detect_version(&buffer)?;
        let entries = Self::parse_entries(&buffer, version)?;

        let total_entries = entries.len();
        Ok(ImgArchive {
            version,
            file_path: path.to_string(),
            entries,
            total_entries,
            file_size: buffer.len() as u64,
            modified: false,
            deleted_entries: Vec::new(),
        })
    }

    /// Detect IMG version from file header
    fn detect_version(buffer: &[u8]) -> Result<ImgVersion, RengineError> {
        // Version 2 (SA) has a different structure with more entries
        // We can detect by trying to parse as both versions

        // Check for V2 signature first (VER2)
        if buffer.len() >= 4 {
            let signature = &buffer[0..4];
            if signature == b"VER2" {
                return Ok(ImgVersion::V2);
            }
        }

        // Try version 1 first (GTA III/VC format)
        if buffer.len() >= 8 {
            let entry_count =
                u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;

            // Version 1: entries start at offset 8, each entry is 32 bytes (name: 24, offset: 4, size: 4)
            let expected_size_v1 = 8 + (entry_count * 32);
            if buffer.len() >= expected_size_v1 {
                // Check if all entries have valid offsets within file bounds
                let mut valid_v1 = true;
                for i in 0..entry_count {
                    let entry_offset = 8 + (i * 32);
                    if entry_offset + 32 > buffer.len() {
                        valid_v1 = false;
                        break;
                    }

                    let offset_start = entry_offset + 24;
                    let size_start = entry_offset + 28;

                    let offset = u32::from_le_bytes([
                        buffer[offset_start],
                        buffer[offset_start + 1],
                        buffer[offset_start + 2],
                        buffer[offset_start + 3],
                    ]);

                    let size = u32::from_le_bytes([
                        buffer[size_start],
                        buffer[size_start + 1],
                        buffer[size_start + 2],
                        buffer[size_start + 3],
                    ]);

                    // Check if offset + size is within file bounds
                    if (offset as usize) + (size as usize) > buffer.len() {
                        valid_v1 = false;
                        break;
                    }
                }

                if valid_v1 {
                    return Ok(ImgVersion::V1);
                }
            }
        }

        // Try version 2 without signature (fallback for files that don't have VER2 header)
        // Version 2 starts directly with entries, each 32 bytes
        if buffer.len() >= 32 {
            let mut valid_v2 = true;
            let mut entry_count = 0;

            // Count entries until we find an invalid one or reach a reasonable limit
            while entry_count < 10000 {
                // Reasonable upper bound
                let entry_offset = entry_count * 32;
                if entry_offset + 32 > buffer.len() {
                    break;
                }

                // Check if name is valid (not all zeros and null-terminated)
                let name_bytes = &buffer[entry_offset..entry_offset + 24];
                let name_end = name_bytes.iter().position(|&b| b == 0).unwrap_or(24);
                if name_end == 0 {
                    // Empty name means end of entries for v2
                    break;
                }

                let offset_start = entry_offset + 24;
                let size_start = entry_offset + 28;

                let offset = u32::from_le_bytes([
                    buffer[offset_start],
                    buffer[offset_start + 1],
                    buffer[offset_start + 2],
                    buffer[offset_start + 3],
                ]);

                let size = u32::from_le_bytes([
                    buffer[size_start],
                    buffer[size_start + 1],
                    buffer[size_start + 2],
                    buffer[size_start + 3],
                ]);

                // Check if offset + size is within file bounds
                if (offset as usize) + (size as usize) > buffer.len() {
                    valid_v2 = false;
                    break;
                }

                entry_count += 1;
            }

            if valid_v2 && entry_count > 0 {
                return Ok(ImgVersion::V2);
            }
        }

        Err(RengineError::FileReadFailed {
            path: "unknown".to_string(),
            details: "Unable to detect IMG archive version".to_string(),
        })
    }

    /// Parse entries based on detected version
    fn parse_entries(buffer: &[u8], version: ImgVersion) -> Result<Vec<ImgEntry>, RengineError> {
        let mut entries = Vec::new();

        match version {
            ImgVersion::V1 => {
                // Version 1: 4-byte entry count at offset 0, entries start at offset 8
                if buffer.len() < 8 {
                    return Err(RengineError::FileReadFailed {
                        path: "unknown".to_string(),
                        details: "Version 1 IMG file too small".to_string(),
                    });
                }

                let entry_count =
                    u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;
                let entries_start = 8;

                for i in 0..entry_count {
                    let entry_offset = entries_start + (i * 32);
                    if entry_offset + 32 > buffer.len() {
                        break;
                    }

                    let entry = Self::parse_entry(&buffer[entry_offset..entry_offset + 32])?;
                    entries.push(entry);
                }
            }
            ImgVersion::V2 => {
                // Check if file starts with VER2 signature
                let entries_start = if buffer.len() >= 8 && &buffer[0..8] == b"VER2!6\x00\x00" {
                    // Extended V2 header: VER2!6 + nulls (8 bytes) + entry_count (4) + unknown (4) = 16 bytes
                    16
                } else if buffer.len() >= 4 && &buffer[0..4] == b"VER2" {
                    // Standard V2 header: VER2 (4 bytes) - entries follow immediately
                    4
                } else {
                    // No header: entries start at offset 0
                    0
                };

                let mut entry_offset = entries_start;

                // Read entries until we can't read more
                while entry_offset + 32 <= buffer.len() {
                    let entry_data = &buffer[entry_offset..entry_offset + 32];
                    let entry = Self::parse_entry(entry_data)?;

                    // Empty name indicates end of entries in v2
                    if entry.name.is_empty() {
                        break;
                    }

                    entries.push(entry);
                    entry_offset += 32;
                }
            }
        }

        Ok(entries)
    }

    /// Parse a single 32-byte entry
    fn parse_entry(entry_data: &[u8]) -> Result<ImgEntry, RengineError> {
        if entry_data.len() != 32 {
            return Err(RengineError::FileReadFailed {
                path: "unknown".to_string(),
                details: "Invalid entry data size".to_string(),
            });
        }

        // Name: first 24 bytes, null-terminated
        let name_bytes = &entry_data[0..24];
        let name_end = name_bytes.iter().position(|&b| b == 0).unwrap_or(24);
        let name = String::from_utf8_lossy(&name_bytes[0..name_end]).to_string();

        // Offset: bytes 24-27 (little endian)
        let offset = u32::from_le_bytes([
            entry_data[24],
            entry_data[25],
            entry_data[26],
            entry_data[27],
        ]);

        // Size: bytes 28-31 (little endian)
        let size = u32::from_le_bytes([
            entry_data[28],
            entry_data[29],
            entry_data[30],
            entry_data[31],
        ]);

        // Determine file type from extension
        let file_type = Self::determine_file_type(&name);

        Ok(ImgEntry {
            offset,
            size,
            name,
            renderware_version: None, // Will be populated by version scanner
            file_type,
        })
    }

    /// Determine file type from filename extension
    fn determine_file_type(filename: &str) -> Option<String> {
        let filename_lower = filename.to_lowercase();
        if filename_lower.ends_with(".dff") {
            Some("DFF".to_string())
        } else if filename_lower.ends_with(".txd") {
            Some("TXD".to_string())
        } else if filename_lower.ends_with(".col") {
            Some("COL".to_string())
        } else if filename_lower.ends_with(".ipl") {
            Some("IPL".to_string())
        } else if filename_lower.ends_with(".ide") {
            Some("IDE".to_string())
        } else if filename_lower.ends_with(".ifp") {
            Some("IFP".to_string())
        } else if filename_lower.ends_with(".dat") {
            Some("DAT".to_string())
        } else {
            None
        }
    }

    /// Extract an entry to a file
    pub fn extract_entry(&self, entry_name: &str, output_path: &str) -> Result<(), RengineError> {
        let entry = self
            .entries
            .iter()
            .find(|e| e.name == entry_name)
            .ok_or_else(|| RengineError::FileReadFailed {
                path: entry_name.to_string(),
                details: "Entry not found in archive".to_string(),
            })?;

        let mut file = File::open(&self.file_path).map_err(|e| RengineError::FileReadFailed {
            path: self.file_path.clone(),
            details: e.to_string(),
        })?;

        // Seek to entry offset (convert from sectors to bytes)
        let byte_offset = entry.offset as u64 * SECTOR_SIZE as u64;
        file.seek(SeekFrom::Start(byte_offset))
            .map_err(|e| RengineError::FileReadFailed {
                path: self.file_path.clone(),
                details: format!("Seek error: {}", e),
            })?;

        // Read entry data (convert from sectors to bytes)
        let byte_size = entry.size as usize * SECTOR_SIZE as usize;
        let mut data = vec![0u8; byte_size];
        file.read_exact(&mut data)
            .map_err(|e| RengineError::FileReadFailed {
                path: self.file_path.clone(),
                details: format!("Read error: {}", e),
            })?;

        // Write to output file
        std::fs::write(output_path, &data).map_err(|e| RengineError::FileWriteFailed {
            path: output_path.to_string(),
            details: e.to_string(),
        })?;

        Ok(())
    }

    /// Get entry data without writing to file
    pub fn get_entry_data(&self, entry_name: &str) -> Result<Vec<u8>, RengineError> {
        let entry = self
            .entries
            .iter()
            .find(|e| e.name == entry_name)
            .ok_or_else(|| RengineError::FileReadFailed {
                path: entry_name.to_string(),
                details: "Entry not found in archive".to_string(),
            })?;

        let mut file = File::open(&self.file_path).map_err(|e| RengineError::FileReadFailed {
            path: self.file_path.clone(),
            details: e.to_string(),
        })?;

        // Seek to entry offset (convert from sectors to bytes)
        let byte_offset = entry.offset as u64 * SECTOR_SIZE as u64;
        file.seek(SeekFrom::Start(byte_offset))
            .map_err(|e| RengineError::FileReadFailed {
                path: self.file_path.clone(),
                details: format!("Seek error: {}", e),
            })?;

        // Read entry data (convert from sectors to bytes)
        let byte_size = entry.size as usize * SECTOR_SIZE as usize;
        let mut data = vec![0u8; byte_size];
        file.read_exact(&mut data)
            .map_err(|e| RengineError::FileReadFailed {
                path: self.file_path.clone(),
                details: format!("Read error: {}", e),
            })?;

        Ok(data)
    }

    /// Analyze RenderWare versions for all entries
    pub fn analyze_renderware_versions(
        &mut self,
        version_manager: &super::versions::RenderWareVersionManager,
    ) {
        for i in 0..self.entries.len() {
            let entry_name = self.entries[i].name.clone();
            if let Some(data) = self.get_entry_data(&entry_name).ok() {
                if let Ok((file_type, version_str, _version_num)) =
                    version_manager.detect_file_format_version(&data, &entry_name)
                {
                    self.entries[i].renderware_version = Some(version_str);
                    // Update file_type if it was detected as RenderWare
                    if file_type != "Unknown" && self.entries[i].file_type.is_none() {
                        self.entries[i].file_type = Some(file_type);
                    }
                }
            }
        }
    }

    /// Add a new entry to the archive (in-memory operation)
    pub fn add_entry(&mut self, filename: &str, data: &[u8]) -> Result<(), crate::RengineError> {
        // Validate inputs
        if filename.is_empty() || data.is_empty() {
            return Err(crate::RengineError::FileReadFailed {
                path: filename.to_string(),
                details: "Invalid filename or empty data provided".to_string(),
            });
        }

        // Ensure filename length is valid (24 bytes for name + null terminator space)
        let entry_name = if filename.len() >= 24 {
            filename[..23].to_string() // Leave room for potential null terminator
        } else {
            filename.to_string()
        };

        // Check for duplicate entries (replace if exists)
        if let Some(existing_index) = self
            .entries
            .iter()
            .position(|e| e.name.to_lowercase() == entry_name.to_lowercase())
        {
            // Replace existing entry
            self.entries[existing_index] = ImgEntry {
                offset: 0, // Will be recalculated on save
                size: data.len() as u32,
                name: entry_name.clone(),
                renderware_version: None,
                file_type: Self::determine_file_type(&entry_name),
            };
        } else {
            // Add new entry
            let new_entry = ImgEntry {
                offset: 0, // Will be recalculated on save
                size: data.len() as u32,
                name: entry_name.clone(),
                renderware_version: None,
                file_type: Self::determine_file_type(&entry_name),
            };
            self.entries.push(new_entry);
            self.total_entries += 1;
        }

        // Mark archive as modified
        self.modified = true;

        Ok(())
    }

    /// Delete an entry from the archive (in-memory operation)
    pub fn delete_entry(&mut self, entry_name: &str) -> Result<(), crate::RengineError> {
        // Find the entry to delete
        if let Some(index) = self
            .entries
            .iter()
            .position(|e| e.name.to_lowercase() == entry_name.to_lowercase())
        {
            // Move entry to deleted_entries for tracking
            let deleted_entry = self.entries.remove(index);
            self.deleted_entries.push(deleted_entry);
            self.total_entries -= 1;
            self.modified = true;
            Ok(())
        } else {
            Err(crate::RengineError::FileReadFailed {
                path: entry_name.to_string(),
                details: "Entry not found in archive".to_string(),
            })
        }
    }

    /// Restore a deleted entry
    pub fn restore_entry(&mut self, entry_name: &str) -> Result<(), crate::RengineError> {
        // Find the entry in deleted_entries
        if let Some(index) = self
            .deleted_entries
            .iter()
            .position(|e| e.name.to_lowercase() == entry_name.to_lowercase())
        {
            // Move entry back to entries
            let restored_entry = self.deleted_entries.remove(index);
            self.entries.push(restored_entry);
            self.total_entries += 1;
            self.modified = true;
            Ok(())
        } else {
            Err(crate::RengineError::FileReadFailed {
                path: entry_name.to_string(),
                details: "Entry not found in deleted entries".to_string(),
            })
        }
    }

    /// Get modification status information
    pub fn get_modification_info(&self) -> serde_json::Value {
        let new_entries_count = self.entries.iter().filter(|e| e.offset == 0).count();

        serde_json::json!({
            "modified": self.modified,
            "total_entries": self.total_entries,
            "deleted_count": self.deleted_entries.len(),
            "new_entries_count": new_entries_count,
            "deleted_entries": self.deleted_entries.iter().map(|e| e.name.clone()).collect::<Vec<_>>(),
            "has_unsaved_changes": self.modified
        })
    }

    /// Clear modification tracking (call after successful save)
    pub fn clear_modification_tracking(&mut self) {
        self.modified = false;
        self.deleted_entries.clear();

        // Clear "new entry" flags by setting proper offsets
        // In a full implementation, this would recalculate all offsets
        // For now, we'll just mark as not modified
    }

    /// Check if archive has unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        self.modified || !self.deleted_entries.is_empty()
    }
}
