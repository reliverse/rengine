use crate::RengineError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextureFormat {
    RGBA32 = 0x00,
    RGBA16 = 0x01,
    Luminance8 = 0x02,
    LuminanceAlpha8 = 0x03,
    Palette4 = 0x04,
    Palette8 = 0x05,
    Compressed = 0x06,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextureFilterMode {
    Nearest = 0x00,
    Linear = 0x01,
    MipNearest = 0x02,
    MipLinear = 0x03,
    LinearMipNearest = 0x04,
    LinearMipLinear = 0x05,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextureAddressingMode {
    Wrap = 0x00,
    Mirror = 0x01,
    Clamp = 0x02,
    Border = 0x03,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureInfo {
    pub name: String,
    pub width: u16,
    pub height: u16,
    pub depth: u8,
    pub format: TextureFormat,
    pub mipmap_count: u8,
    pub raster_type: u32,
    pub filter_mode: TextureFilterMode,
    pub addressing_u: TextureAddressingMode,
    pub addressing_v: TextureAddressingMode,
    pub data_size: u32,
    pub data_offset: u32,
    pub renderware_version: Option<String>,
    pub platform_flags: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxdArchive {
    pub file_path: String,
    pub textures: Vec<TextureInfo>,
    pub total_textures: usize,
    pub renderware_version: Option<String>,
}

impl TxdArchive {
    /// Load and parse a TXD archive from file path
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

        Self::parse_txd_archive(&mut file, path)
    }

    /// Parse TXD archive from an open file handle
    pub fn parse_txd_archive(file: &mut File, path: &str) -> Result<Self, RengineError> {
        // Read the entire file
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| RengineError::FileReadFailed {
                path: path.to_string(),
                details: e.to_string(),
            })?;

        if buffer.len() < 12 {
            return Err(RengineError::FileReadFailed {
                path: path.to_string(),
                details: "File too small to be a valid TXD archive".to_string(),
            });
        }

        // Check for TEXDICTIONARY section
        let section_type = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        if section_type != 0x0016 {
            return Err(RengineError::FileReadFailed {
                path: path.to_string(),
                details: "Not a valid TXD file (missing TEXDICTIONARY section)".to_string(),
            });
        }

        let version = u32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);
        let rw_version =
            super::versions::RenderWareVersionManager::new().get_rw_version(Some(version as i32));

        let textures = Self::parse_textures(&buffer)?;
        let renderware_version = super::versions::RenderWareVersionManager::new()
            .get_version_display_string(rw_version, true);

        let total_textures = textures.len();
        Ok(TxdArchive {
            file_path: path.to_string(),
            textures,
            total_textures,
            renderware_version: Some(renderware_version),
        })
    }

    /// Parse texture entries from TXD data
    fn parse_textures(buffer: &[u8]) -> Result<Vec<TextureInfo>, RengineError> {
        let mut offset = 12; // Skip TEXDICTIONARY header (12 bytes: type(4) + size(4) + version(4))

        eprintln!(
            "[TXD DEBUG] Buffer size: {}, Starting offset: {}",
            buffer.len(),
            offset
        );

        // Read TEXDICTIONARY STRUCT section (contains texture count)
        if offset + 12 > buffer.len() {
            eprintln!("[TXD DEBUG] Buffer too small for STRUCT header");
            return Ok(Vec::new());
        }

        let struct_type = u32::from_le_bytes([
            buffer[offset],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
        ]);
        eprintln!(
            "[TXD DEBUG] STRUCT type at offset {}: 0x{:08X}",
            offset, struct_type
        );

        // Try different STRUCT type values (RenderWare uses different values)
        let is_struct = struct_type == 0x0001 || struct_type == 0x01 || struct_type == 0x00000001;

        if !is_struct {
            eprintln!(
                "[TXD DEBUG] No STRUCT section found (type 0x{:08X}), trying legacy parser",
                struct_type
            );
            // No STRUCT section, try to find TEXTURENATIVE directly
            return Self::parse_textures_legacy(buffer);
        }

        let struct_size = u32::from_le_bytes([
            buffer[offset + 4],
            buffer[offset + 5],
            buffer[offset + 6],
            buffer[offset + 7],
        ]) as usize;
        eprintln!("[TXD DEBUG] STRUCT header size field: {}", struct_size);
        offset += 12; // Skip STRUCT header

        // Read texture count from STRUCT
        // STRUCT data size might be 4 or 8 bytes depending on RenderWare version
        if offset + struct_size > buffer.len() {
            eprintln!(
                "[TXD DEBUG] Buffer too small for STRUCT data (need {}, have {})",
                struct_size,
                buffer.len() - offset
            );
            return Ok(Vec::new());
        }

        let texture_count = if struct_size >= 4 {
            u32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]) as usize
        } else {
            eprintln!("[TXD DEBUG] STRUCT size too small: {}", struct_size);
            return Ok(Vec::new());
        };

        let device_id = if struct_size >= 8 {
            Some(u32::from_le_bytes([
                buffer[offset + 4],
                buffer[offset + 5],
                buffer[offset + 6],
                buffer[offset + 7],
            ]))
        } else {
            None
        };

        eprintln!(
            "[TXD DEBUG] Texture count: {}, Device ID: {:?}, STRUCT data size: {}",
            texture_count, device_id, struct_size
        );

        // The texture count from STRUCT is likely wrong (STRUCT size is only 4 bytes)
        // Always use scanning to find TEXTURENATIVE sections
        eprintln!(
            "[TXD DEBUG] Using scanning method to find textures (STRUCT size was only {} bytes)",
            struct_size
        );
        return Self::parse_textures_by_scanning(buffer, offset + struct_size);
    }

    /// Scan for TEXTURENATIVE sections without relying on texture count
    fn parse_textures_by_scanning(
        buffer: &[u8],
        start_offset: usize,
    ) -> Result<Vec<TextureInfo>, RengineError> {
        eprintln!(
            "[TXD DEBUG] Scanning for textures from offset {} (buffer size: {})",
            start_offset,
            buffer.len()
        );
        let mut textures = Vec::new();
        let mut offset = start_offset;
        let mut found_count = 0;
        let mut consecutive_zeros = 0;

        // Scan through the buffer looking for TEXTURENATIVE sections
        let mut scan_count = 0;
        while offset + 12 <= buffer.len() && scan_count < 10000 {
            scan_count += 1;
            let section_type = u32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]);

            // Log first few sections to understand structure
            if scan_count <= 20 {
                let section_size = u32::from_le_bytes([
                    buffer[offset + 4],
                    buffer[offset + 5],
                    buffer[offset + 6],
                    buffer[offset + 7],
                ]) as usize;
                eprintln!(
                    "[TXD DEBUG] Scan {}: offset {}, type 0x{:08X}, size {}",
                    scan_count, offset, section_type, section_size
                );
            }

            // If we see many zero sections in a row, we've probably reached the end
            if section_type == 0 {
                consecutive_zeros += 1;
                if consecutive_zeros > 50 {
                    // Increased threshold
                    eprintln!(
                        "[TXD DEBUG] Too many zero sections ({}), stopping scan",
                        consecutive_zeros
                    );
                    break;
                }
                offset += 12; // Skip zero section header
                continue;
            } else {
                consecutive_zeros = 0;
            }

            // TEXTURENATIVE can be 0x15 (21) or 0x0253FF01 (extended format)
            let is_texture_native =
                section_type == 0x0253FF01 || section_type == 0x15 || section_type == 0x00000015;

            if is_texture_native {
                eprintln!(
                    "[TXD DEBUG] Found TEXTURENATIVE (type 0x{:08X}) at offset {}",
                    section_type, offset
                );
                found_count += 1;

                let section_size = u32::from_le_bytes([
                    buffer[offset + 4],
                    buffer[offset + 5],
                    buffer[offset + 6],
                    buffer[offset + 7],
                ]) as usize;
                let version = u32::from_le_bytes([
                    buffer[offset + 8],
                    buffer[offset + 9],
                    buffer[offset + 10],
                    buffer[offset + 11],
                ]);

                if offset + 12 + section_size > buffer.len() {
                    eprintln!(
                        "[TXD DEBUG] Section size {} exceeds buffer, skipping",
                        section_size
                    );
                    offset += 12; // Skip header only
                    continue;
                }

                // TEXTURENATIVE contains STRUCT
                // The STRUCT header is 12 bytes, then the STRUCT data starts
                let texture_start = offset + 12; // After TEXTURENATIVE header
                if texture_start + 12 <= buffer.len() {
                    // Read STRUCT header
                    let struct_type = u32::from_le_bytes([
                        buffer[texture_start],
                        buffer[texture_start + 1],
                        buffer[texture_start + 2],
                        buffer[texture_start + 3],
                    ]);
                    let struct_size = u32::from_le_bytes([
                        buffer[texture_start + 4],
                        buffer[texture_start + 5],
                        buffer[texture_start + 6],
                        buffer[texture_start + 7],
                    ]) as usize;
                    eprintln!(
                        "[TXD DEBUG] TEXTURENATIVE STRUCT: type=0x{:08X}, size={}",
                        struct_type, struct_size
                    );

                    // STRUCT data starts after STRUCT header (12 bytes)
                    let texture_data_start = texture_start + 12;

                    // The STRUCT data contains the texture native data
                    // But we need to read the full STRUCT section, not just struct_size
                    // Actually, struct_size tells us how much data to read
                    if texture_data_start + struct_size <= buffer.len() {
                        let texture_data =
                            &buffer[texture_data_start..texture_data_start + struct_size];
                        eprintln!(
                            "[TXD DEBUG] Reading texture data from offset {}, size {}",
                            texture_data_start, struct_size
                        );
                        eprintln!(
                            "[TXD DEBUG] First 16 bytes: {:02X?}",
                            &texture_data[0..std::cmp::min(16, texture_data.len())]
                        );
                        match Self::parse_texture_native(texture_data, version) {
                            Ok(texture) => {
                                eprintln!(
                                    "[TXD DEBUG] Successfully parsed texture {}: '{}' ({}x{})",
                                    found_count, texture.name, texture.width, texture.height
                                );
                                textures.push(texture);
                            }
                            Err(e) => {
                                eprintln!("[TXD DEBUG] Failed to parse texture native: {}", e);
                            }
                        }
                    } else {
                        eprintln!("[TXD DEBUG] Texture data extends beyond buffer: start={}, size={}, buffer_len={}", texture_data_start, struct_size, buffer.len());
                    }
                }

                offset += 12 + section_size;
            } else {
                // Skip this section
                let section_size = u32::from_le_bytes([
                    buffer[offset + 4],
                    buffer[offset + 5],
                    buffer[offset + 6],
                    buffer[offset + 7],
                ]) as usize;

                // Safety check - if section size is 0, skip just the header (12 bytes)
                if section_size == 0 {
                    eprintln!(
                        "[TXD DEBUG] Section size 0 at offset {}, skipping header only",
                        offset
                    );
                    offset += 12;
                    continue;
                }

                // If section size is too large, skip header and try next position
                if section_size > buffer.len() || offset + 12 + section_size > buffer.len() {
                    eprintln!(
                        "[TXD DEBUG] Invalid section size {} at offset {}, skipping header",
                        section_size, offset
                    );
                    offset += 12;
                    continue;
                }

                // Skip this section
                let section_size = u32::from_le_bytes([
                    buffer[offset + 4],
                    buffer[offset + 5],
                    buffer[offset + 6],
                    buffer[offset + 7],
                ]) as usize;

                // Handle invalid section sizes gracefully
                if section_size == 0 {
                    eprintln!(
                        "[TXD DEBUG] Section 0x{:08X} has size 0 at offset {}, skipping header",
                        section_type, offset
                    );
                    offset += 12;
                    continue;
                }

                if section_size > buffer.len() || offset + 12 + section_size > buffer.len() {
                    eprintln!("[TXD DEBUG] Section 0x{:08X} has invalid size {} at offset {}, skipping header", section_type, section_size, offset);
                    offset += 12;
                    continue;
                }

                offset += 12 + section_size;
            }

            // Safety limit - don't scan forever
            if found_count > 100 {
                eprintln!("[TXD DEBUG] Found {} textures, stopping scan", found_count);
                break;
            }

            // Continue scanning until we've checked the whole buffer
            // Don't stop early - textures might be at the end
        }

        eprintln!(
            "[TXD DEBUG] Scan complete after {} iterations, final offset: {}",
            scan_count, offset
        );

        eprintln!(
            "[TXD DEBUG] Scan complete: found {} textures",
            textures.len()
        );
        Ok(textures)
    }

    /// Legacy parser for TXD files without STRUCT section
    fn parse_textures_legacy(buffer: &[u8]) -> Result<Vec<TextureInfo>, RengineError> {
        eprintln!("[TXD DEBUG] Using legacy parser");
        let mut textures = Vec::new();
        let mut offset = 12; // Skip TEXDICTIONARY header

        // Try to read STRUCT section first (might be present but with different type)
        if offset + 12 <= buffer.len() {
            let possible_struct_type = u32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]);
            let possible_struct_size = u32::from_le_bytes([
                buffer[offset + 4],
                buffer[offset + 5],
                buffer[offset + 6],
                buffer[offset + 7],
            ]) as usize;

            eprintln!(
                "[TXD DEBUG] Legacy: Possible STRUCT at offset {}: type 0x{:08X}, size {}",
                offset, possible_struct_type, possible_struct_size
            );

            // If it looks like a STRUCT (small size, reasonable), try to read texture count
            if possible_struct_size > 0 && possible_struct_size < 100 {
                offset += 12; // Skip header
                if offset + 4 <= buffer.len() {
                    let possible_texture_count = u32::from_le_bytes([
                        buffer[offset],
                        buffer[offset + 1],
                        buffer[offset + 2],
                        buffer[offset + 3],
                    ]) as usize;
                    eprintln!(
                        "[TXD DEBUG] Legacy: Possible texture count: {}",
                        possible_texture_count
                    );
                    // If texture count is reasonable, use it
                    if possible_texture_count > 0 && possible_texture_count < 1000 {
                        offset += possible_struct_size; // Skip STRUCT data
                                                        // Now look for TEXTURENATIVE sections
                        for _i in 0..possible_texture_count {
                            if offset + 12 > buffer.len() {
                                break;
                            }
                            let section_type = u32::from_le_bytes([
                                buffer[offset],
                                buffer[offset + 1],
                                buffer[offset + 2],
                                buffer[offset + 3],
                            ]);
                            eprintln!("[TXD DEBUG] Legacy: Looking for texture, found section type 0x{:08X} at offset {}", section_type, offset);

                            // TEXTURENATIVE can be 0x15 (21) or 0x0253FF01 (extended format)
                            let is_texture_native = section_type == 0x0253FF01
                                || section_type == 0x15
                                || section_type == 0x00000015;
                            if is_texture_native {
                                // TEXTURENATIVE
                                let section_size = u32::from_le_bytes([
                                    buffer[offset + 4],
                                    buffer[offset + 5],
                                    buffer[offset + 6],
                                    buffer[offset + 7],
                                ]) as usize;
                                let version = u32::from_le_bytes([
                                    buffer[offset + 8],
                                    buffer[offset + 9],
                                    buffer[offset + 10],
                                    buffer[offset + 11],
                                ]);

                                // TEXTURENATIVE contains STRUCT
                                let texture_start = offset + 12;
                                if texture_start + 12 <= buffer.len() {
                                    let struct_size = u32::from_le_bytes([
                                        buffer[texture_start + 4],
                                        buffer[texture_start + 5],
                                        buffer[texture_start + 6],
                                        buffer[texture_start + 7],
                                    ])
                                        as usize;
                                    let texture_data_start = texture_start + 12;
                                    if texture_data_start + struct_size <= buffer.len() {
                                        let texture_data = &buffer
                                            [texture_data_start..texture_data_start + struct_size];
                                        if let Ok(texture) =
                                            Self::parse_texture_native(texture_data, version)
                                        {
                                            eprintln!("[TXD DEBUG] Legacy: Successfully parsed texture: {}", texture.name);
                                            textures.push(texture);
                                        }
                                    }
                                }
                                offset += 12 + section_size;
                            } else {
                                // Skip this section
                                let section_size = u32::from_le_bytes([
                                    buffer[offset + 4],
                                    buffer[offset + 5],
                                    buffer[offset + 6],
                                    buffer[offset + 7],
                                ]) as usize;
                                offset += 12 + section_size;
                            }
                        }
                        return Ok(textures);
                    }
                }
                // Reset offset if we didn't use the STRUCT
                offset = 12;
            }
        }

        // Skip TEXDICTIONARY data section (usually empty or contains version info)
        if offset + 4 <= buffer.len() {
            let data_size = u32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]) as usize;
            offset += 4 + data_size;
        }

        // Parse texture native sections - search for any TEXTURENATIVE
        eprintln!(
            "[TXD DEBUG] Legacy: Searching for TEXTURENATIVE sections from offset {}",
            offset
        );
        while offset + 12 <= buffer.len() {
            let section_type = u32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]);

            eprintln!(
                "[TXD DEBUG] Legacy: Section at offset {}: type 0x{:08X}",
                offset, section_type
            );

            if section_type != 0x0253FF01 {
                // TEXTURENATIVE
                // Skip this section
                let section_size = u32::from_le_bytes([
                    buffer[offset + 4],
                    buffer[offset + 5],
                    buffer[offset + 6],
                    buffer[offset + 7],
                ]) as usize;
                offset += 12 + section_size;
                continue;
            }

            let section_size = u32::from_le_bytes([
                buffer[offset + 4],
                buffer[offset + 5],
                buffer[offset + 6],
                buffer[offset + 7],
            ]) as usize;

            let version = u32::from_le_bytes([
                buffer[offset + 8],
                buffer[offset + 9],
                buffer[offset + 10],
                buffer[offset + 11],
            ]);

            if offset + 12 + section_size > buffer.len() {
                break; // Invalid section size
            }

            let texture_data = &buffer[offset + 12..offset + 12 + section_size];
            if let Ok(texture) = Self::parse_texture_native(texture_data, version) {
                textures.push(texture);
            }

            offset += 12 + section_size;
        }

        Ok(textures)
    }

    /// Parse a single TEXTURENATIVE section
    fn parse_texture_native(data: &[u8], _version: u32) -> Result<TextureInfo, RengineError> {
        // Texture native data structure (from Python reference):
        // platform_id (4 bytes) + filter_mode (1) + uv_addressing (1) + unk (1) + name (32) + mask (32) = 72 bytes
        // Then: raster_format_flags (4) + d3d_format (4) + width (2) + height (2) + depth (1) + num_levels (1) + raster_type (1) = 15 bytes
        if data.len() < 87 {
            return Err(RengineError::FileReadFailed {
                path: "unknown".to_string(),
                details: format!(
                    "Texture native data too small: {} bytes (need at least 87)",
                    data.len()
                ),
            });
        }

        eprintln!(
            "[TXD DEBUG] parse_texture_native: data len={}, first 72 bytes: {:02X?}",
            data.len(),
            &data[0..std::cmp::min(72, data.len())]
        );

        let mut pos = 0;

        // Read platform_id (4 bytes)
        let platform_id =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        eprintln!(
            "[TXD DEBUG] platform_id at pos {}: 0x{:08X}",
            pos, platform_id
        );
        pos += 4;

        // Read filter_mode (1 byte)
        let filter_mode_byte = data[pos];
        pos += 1;

        // Read uv_addressing (1 byte)
        let uv_addressing_byte = data[pos];
        pos += 1;

        // Skip unk (2 bytes - unsigned short in Python format)
        let _unk = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        // Parse texture name (32 bytes, null-terminated)
        let name_bytes = &data[pos..pos + 32];
        eprintln!("[TXD DEBUG] name bytes at pos {}: {:02X?}", pos, name_bytes);
        let name_end = name_bytes.iter().position(|&b| b == 0).unwrap_or(32);
        let name = String::from_utf8_lossy(&name_bytes[0..name_end]).to_string();
        eprintln!("[TXD DEBUG] parsed name: '{}' (end at {})", name, name_end);
        pos += 32;

        // Parse mask name
        // Note: Format says 32 bytes, but actual files may use 20 bytes (0x70 - 0x5C = 20)
        // Try to detect: if next 4 bytes after 32-byte mask look like raster_format_flags, use 32
        // Otherwise use 20 bytes
        let mask_len = if pos + 36 <= data.len() {
            // Check what's 32 bytes ahead (should be raster_format_flags if mask is 32 bytes)
            let test_raster = u32::from_le_bytes([
                data[pos + 32],
                data[pos + 33],
                data[pos + 34],
                data[pos + 35],
            ]);
            // Raster format flags are usually small values (0x00000200, etc.)
            // If it's a reasonable value, mask is 32 bytes, otherwise 20
            if test_raster < 0x1000000 && test_raster != 0 {
                32
            } else {
                20
            }
        } else {
            20 // Default to 20 bytes for GTA SA TXD files
        };

        let mask_bytes = &data[pos..pos + mask_len];
        let mask_end = mask_bytes.iter().position(|&b| b == 0).unwrap_or(mask_len);
        let _mask = String::from_utf8_lossy(&mask_bytes[0..mask_end]).to_string();
        eprintln!(
            "[TXD DEBUG] mask at pos {} (len {}): '{}'",
            pos, mask_len, _mask
        );
        pos += mask_len;

        // Now read raster format flags, d3d_format, dimensions, etc.
        if data.len() < pos + 15 {
            return Err(RengineError::FileReadFailed {
                path: "unknown".to_string(),
                details: format!(
                    "Texture native data too small for raster info: {} bytes (need at least {})",
                    data.len(),
                    pos + 15
                ),
            });
        }

        // Parse raster format flags (4 bytes)
        let raster_format_flags =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        // Parse d3d_format (4 bytes)
        let d3d_format =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        // Parse dimensions and format
        let width_raw = u16::from_le_bytes([data[pos], data[pos + 1]]);
        eprintln!(
            "[TXD DEBUG] width raw bytes at pos {}: {:02X?} = {}",
            pos,
            &data[pos..pos + 2],
            width_raw
        );
        let width = width_raw;
        pos += 2;
        let height_raw = u16::from_le_bytes([data[pos], data[pos + 1]]);
        eprintln!(
            "[TXD DEBUG] height raw bytes at pos {}: {:02X?} = {}",
            pos,
            &data[pos..pos + 2],
            height_raw
        );
        let height = height_raw;
        pos += 2;
        let depth = data[pos];
        pos += 1;
        let mipmap_count = data[pos];
        pos += 1;
        let _raster_type_byte = data[pos];

        let format = match (raster_format_flags >> 8) & 0xFF {
            0x00 => TextureFormat::RGBA32,
            0x01 => TextureFormat::RGBA16,
            0x02 => TextureFormat::Luminance8,
            0x03 => TextureFormat::LuminanceAlpha8,
            0x04 => TextureFormat::Palette4,
            0x05 => TextureFormat::Palette8,
            0x06 => TextureFormat::Compressed,
            _ => TextureFormat::RGBA32, // Default
        };

        // Parse raster format (use d3d_format or raster_format_flags)
        let raster_type = d3d_format;

        // Calculate data size (we don't need to read it, it's in the section size)
        let data_size = 0; // Will be calculated from section size if needed

        // Extract filter mode and addressing from filter_mode_byte and uv_addressing_byte
        let filter_mode = match filter_mode_byte {
            0x00 => TextureFilterMode::Nearest,
            0x01 => TextureFilterMode::Linear,
            0x02 => TextureFilterMode::MipNearest,
            0x03 => TextureFilterMode::MipLinear,
            0x04 => TextureFilterMode::LinearMipNearest,
            0x05 => TextureFilterMode::LinearMipLinear,
            _ => TextureFilterMode::Linear,
        };

        let addressing_u = match (uv_addressing_byte >> 4) & 0x0F {
            0x00 => TextureAddressingMode::Wrap,
            0x01 => TextureAddressingMode::Mirror,
            0x02 => TextureAddressingMode::Clamp,
            0x03 => TextureAddressingMode::Border,
            _ => TextureAddressingMode::Wrap,
        };

        let addressing_v = match uv_addressing_byte & 0x0F {
            0x00 => TextureAddressingMode::Wrap,
            0x01 => TextureAddressingMode::Mirror,
            0x02 => TextureAddressingMode::Clamp,
            0x03 => TextureAddressingMode::Border,
            _ => TextureAddressingMode::Wrap,
        };

        eprintln!("[TXD DEBUG] Parsed texture native: name='{}', width={}, height={}, format={:?}, platform_id=0x{:08X}", name, width, height, format, platform_id);

        Ok(TextureInfo {
            name,
            width,
            height,
            depth,
            format,
            mipmap_count,
            raster_type,
            filter_mode,
            addressing_u,
            addressing_v,
            data_size,
            data_offset: 0, // Will be calculated during parsing
            renderware_version: None,
            platform_flags: raster_type,
        })
    }

    /// Get texture data by name
    #[allow(dead_code)]
    pub fn get_texture_data(&self, texture_name: &str) -> Result<Vec<u8>, RengineError> {
        let texture = self
            .textures
            .iter()
            .find(|t| t.name == texture_name)
            .ok_or_else(|| RengineError::FileReadFailed {
                path: texture_name.to_string(),
                details: "Texture not found in TXD archive".to_string(),
            })?;

        let mut file = File::open(&self.file_path).map_err(|e| RengineError::FileReadFailed {
            path: self.file_path.clone(),
            details: e.to_string(),
        })?;

        // Seek to texture data offset
        file.seek(SeekFrom::Start(texture.data_offset as u64))
            .map_err(|e| RengineError::FileReadFailed {
                path: self.file_path.clone(),
                details: format!("Seek error: {}", e),
            })?;

        // Read texture data
        let mut data = vec![0u8; texture.data_size as usize];
        file.read_exact(&mut data)
            .map_err(|e| RengineError::FileReadFailed {
                path: self.file_path.clone(),
                details: format!("Read error: {}", e),
            })?;

        Ok(data)
    }

    /// Export texture to PNG format (placeholder - would need image processing library)
    pub fn export_texture_to_png(
        &self,
        _texture_name: &str,
        output_path: &str,
    ) -> Result<(), RengineError> {
        // This would require an image processing library like image-rs
        // For now, just return an error indicating this feature needs implementation
        Err(RengineError::FileReadFailed {
            path: output_path.to_string(),
            details: "PNG export not yet implemented - requires image processing library"
                .to_string(),
        })
    }

    /// Get texture info by name
    pub fn get_texture_info(&self, texture_name: &str) -> Option<&TextureInfo> {
        self.textures.iter().find(|t| t.name == texture_name)
    }

    /// Get all texture names
    #[allow(dead_code)]
    pub fn get_texture_names(&self) -> Vec<String> {
        self.textures.iter().map(|t| t.name.clone()).collect()
    }

    /// Get texture statistics
    pub fn get_statistics(&self) -> serde_json::Value {
        let total_size: u32 = self.textures.iter().map(|t| t.data_size).sum();
        let avg_width = if !self.textures.is_empty() {
            self.textures.iter().map(|t| t.width as u32).sum::<u32>() / self.textures.len() as u32
        } else {
            0
        };
        let avg_height = if !self.textures.is_empty() {
            self.textures.iter().map(|t| t.height as u32).sum::<u32>() / self.textures.len() as u32
        } else {
            0
        };

        let format_counts: std::collections::HashMap<String, usize> =
            self.textures
                .iter()
                .fold(std::collections::HashMap::new(), |mut acc, texture| {
                    let format_str = format!("{:?}", texture.format);
                    *acc.entry(format_str).or_insert(0) += 1;
                    acc
                });

        serde_json::json!({
            "total_textures": self.total_textures,
            "total_size_bytes": total_size,
            "average_width": avg_width,
            "average_height": avg_height,
            "format_counts": format_counts,
            "renderware_version": self.renderware_version
        })
    }
}
