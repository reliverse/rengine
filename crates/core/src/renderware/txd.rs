use crate::RengineError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use bcndecode::bcndecode::{decode, BcnEncoding, BcnDecoderFormat};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextureFormat {
    RGBA32 = 0x00,
    RGBA16 = 0x01,
    Luminance8 = 0x02,
    LuminanceAlpha8 = 0x03,
    Palette4 = 0x04,
    Palette8 = 0x05,
    Compressed = 0x06,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextureFilterMode {
    Nearest = 0x00,
    Linear = 0x01,
    MipNearest = 0x02,
    MipLinear = 0x03,
    LinearMipNearest = 0x04,
    LinearMipLinear = 0x05,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextureAddressingMode {
    Wrap = 0x00,
    Mirror = 0x01,
    Clamp = 0x02,
    Border = 0x03,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativePlatformType {
    PC = 0,      // Default PC platform
    OGL = 2,     // OpenGL
    PS2 = 4,     // PlayStation 2
    XBOX = 5,    // Xbox
    D3D8 = 8,    // Direct3D 8
    D3D9 = 9,    // Direct3D 9
    ATC = 11,    // AMD Texture Compression
    PVR = 12,    // PowerVR
    GC = 13,     // GameCube
    PSP = 14,    // PlayStation Portable
    DxtMobile = 15, // S3TC Mobile
    UncMobile = 16, // Uncompressed Mobile
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum D3DFormat {
    D3D8888 = 21,
    D3D888 = 22,
    D3D565 = 23,
    D3D555 = 24,
    D3D1555 = 25,
    D3D4444 = 26,
    D3DFMTL8 = 50,
    D3DFMTA8L8 = 51,
    D3DDXT1 = 827611204,
    D3DDXT2 = 844388420,
    D3DDXT3 = 861165636,
    D3DDXT4 = 877942852,
    D3DDXT5 = 894720068,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RasterFormat {
    RasterDefault = 0x00,
    Raster1555 = 0x01,
    Raster565 = 0x02,
    Raster4444 = 0x03,
    RasterLum = 0x04,
    Raster8888 = 0x05,
    Raster888 = 0x06,
    Raster16 = 0x07,
    Raster24 = 0x08,
    Raster32 = 0x09,
    Raster555 = 0x0a,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum D3DCompressType {
    DXT1 = 1,
    DXT2 = 2,
    DXT3 = 3,
    DXT4 = 4,
    DXT5 = 5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaletteType {
    PaletteNone = 0,
    Palette8 = 1,
    Palette4 = 2,
    Palette4Lsb = 3,
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
    pub palette_data: Option<Vec<u8>>, // Palette data for paletted textures
    pub platform_id: u32, // Platform identifier (D3D8, D3D9, etc.)
    pub d3d_format: u32, // Direct3D format identifier
    pub raster_format_flags: u32, // Raster format flags
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxdArchive {
    pub file_path: String,
    pub textures: Vec<TextureInfo>,
    pub total_textures: usize,
    pub renderware_version: Option<String>,
}

impl NativePlatformType {
    /// Convert platform ID to enum value
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            0 => Some(Self::PC),
            2 => Some(Self::OGL),
            4 => Some(Self::PS2),
            5 => Some(Self::XBOX),
            8 => Some(Self::D3D8),
            9 => Some(Self::D3D9),
            11 => Some(Self::ATC),
            12 => Some(Self::PVR),
            13 => Some(Self::GC),
            14 => Some(Self::PSP),
            15 => Some(Self::DxtMobile),
            16 => Some(Self::UncMobile),
            _ => None,
        }
    }

    /// Get platform name
    pub fn name(&self) -> &str {
        match self {
            Self::PC => "PC",
            Self::OGL => "OpenGL",
            Self::PS2 => "PlayStation 2",
            Self::XBOX => "Xbox",
            Self::D3D8 => "Direct3D 8",
            Self::D3D9 => "Direct3D 9",
            Self::ATC => "AMD TC",
            Self::PVR => "PowerVR",
            Self::GC => "GameCube",
            Self::PSP => "PlayStation Portable",
            Self::DxtMobile => "S3TC Mobile",
            Self::UncMobile => "Uncompressed Mobile",
        }
    }
}

impl TxdArchive {
    /// Create a new empty TXD archive
    pub fn new() -> Self {
        Self {
            file_path: String::new(),
            textures: Vec::new(),
            total_textures: 0,
            renderware_version: None,
        }
    }

    /// Create a new TXD archive with a file path
    pub fn with_path(file_path: String) -> Self {
        Self {
            file_path,
            textures: Vec::new(),
            total_textures: 0,
            renderware_version: None,
        }
    }

    /// Add a texture to the TXD archive
    pub fn add_texture(&mut self, texture: TextureInfo) {
        self.textures.push(texture);
        self.total_textures = self.textures.len();
    }

    /// Remove a texture by name
    pub fn remove_texture(&mut self, name: &str) -> bool {
        let initial_len = self.textures.len();
        self.textures.retain(|t| t.name != name);
        let removed = self.textures.len() < initial_len;
        if removed {
            self.total_textures = self.textures.len();
        }
        removed
    }

    /// Get mutable reference to a texture by name
    pub fn get_texture_mut(&mut self, name: &str) -> Option<&mut TextureInfo> {
        self.textures.iter_mut().find(|t| t.name == name)
    }

    /// Clear all textures
    pub fn clear(&mut self) {
        self.textures.clear();
        self.total_textures = 0;
    }

    /// Set RenderWare version for the archive
    pub fn set_renderware_version(&mut self, version: String) {
        self.renderware_version = Some(version);
    }

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

    /// Parse texture entries from TXD data using proper RenderWare structure
    fn parse_textures(buffer: &[u8]) -> Result<Vec<TextureInfo>, RengineError> {
        let mut textures = Vec::new();
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

        // Check for STRUCT section (0x01 or variations)
        let is_struct = struct_type == 0x01 || struct_type == 0x00000001;

        if !is_struct {
            eprintln!(
                "[TXD DEBUG] No STRUCT section found (type 0x{:08X}), using direct TEXTURENATIVE scanning",
                struct_type
            );
            return Self::parse_textures_by_scanning(buffer, offset);
        }

        let struct_size = u32::from_le_bytes([
            buffer[offset + 4],
            buffer[offset + 5],
            buffer[offset + 6],
            buffer[offset + 7],
        ]) as usize;
        offset += 12; // Skip STRUCT header

        eprintln!("[TXD DEBUG] STRUCT section found, size: {}", struct_size);

        // Parse STRUCT data to get texture count
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

        eprintln!("[TXD DEBUG] Texture count from STRUCT: {}", texture_count);

        // Skip the STRUCT data
        offset += struct_size;

        // Sanity check the texture count - RenderWare TXD files don't have millions of textures
        let texture_count = if texture_count > 10000 {
            eprintln!("[TXD DEBUG] Texture count {} seems too high, likely parsing error. Will scan for actual textures.", texture_count);
            0 // Will use scanning mode
        } else {
            texture_count
        };

        // If texture_count is 0, use scanning mode. Otherwise parse the specified number.
        if texture_count > 0 {
            // Parse the specified number of TEXTURENATIVE sections
            for i in 0..texture_count {
                eprintln!("[TXD DEBUG] Looking for texture {} at offset {}", i, offset);

                if offset + 12 > buffer.len() {
                    eprintln!("[TXD DEBUG] Not enough data for texture {} header", i);
                    break;
                }

                let section_type = u32::from_le_bytes([
                    buffer[offset],
                    buffer[offset + 1],
                    buffer[offset + 2],
                    buffer[offset + 3],
                ]);

                // TEXTURENATIVE sections
                if section_type == 0x15 || section_type == 0x00000015 {
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

                    eprintln!(
                        "[TXD DEBUG] Found TEXTURENATIVE section at offset {}, size: {}, version: 0x{:08X}",
                        offset, section_size, version
                    );

                    let texture_data_start = offset + 12;
                    if texture_data_start + section_size <= buffer.len() {
                        let texture_data = &buffer[texture_data_start..texture_data_start + section_size];
                        match Self::parse_texture_native(texture_data, version, texture_data_start) {
                            Ok(texture) => {
                                eprintln!(
                                    "[TXD DEBUG] Successfully parsed texture {}: '{}' ({}x{})",
                                    i, texture.name, texture.width, texture.height
                                );
                                textures.push(texture);
                            }
                            Err(e) => {
                                eprintln!("[TXD DEBUG] Failed to parse texture {}: {}", i, e);
                                // Continue with other textures
                            }
                        }
                    } else {
                        eprintln!(
                            "[TXD DEBUG] Texture {} data extends beyond buffer: start={}, size={}, buffer_len={}",
                            i, texture_data_start, section_size, buffer.len()
                        );
                    }

                    // Move to next section
                    offset += 12 + section_size;
                } else {
                    eprintln!(
                        "[TXD DEBUG] Unexpected section type 0x{:08X} at offset {} for texture {}",
                        section_type, offset, i
                    );
                    // Try to skip this section if possible
                    if offset + 8 <= buffer.len() {
                        let section_size = u32::from_le_bytes([
                            buffer[offset + 4],
                            buffer[offset + 5],
                            buffer[offset + 6],
                            buffer[offset + 7],
                        ]) as usize;
                        offset += 12 + section_size;
                    } else {
                        break;
                    }
                }
            }
        } else {
            // Use scanning mode to find all TEXTURENATIVE sections
            eprintln!("[TXD DEBUG] Using scanning mode to find all textures");
            return Self::parse_textures_by_scanning(buffer, offset);
        }

        eprintln!(
            "[TXD DEBUG] Structured parsing complete: found {} textures",
            textures.len()
        );
        Ok(textures)
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
                        match Self::parse_texture_native(texture_data, version, texture_data_start) { // Pass the absolute offset to texture data start
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

    /// Parse a single TEXTURENATIVE section
    fn calculate_texture_data_size(width: u16, height: u16, format: &TextureFormat, mipmap_count: u8) -> u32 {
        let mut total_size = 0u32;
        let mut current_width = width as u32;
        let mut current_height = height as u32;

        for _ in 0..mipmap_count {
            let mipmap_size = match format {
                TextureFormat::RGBA32 => current_width * current_height * 4,
                TextureFormat::RGBA16 => current_width * current_height * 2,
                TextureFormat::Luminance8 => current_width * current_height,
                TextureFormat::LuminanceAlpha8 => current_width * current_height * 2,
                TextureFormat::Palette4 => current_width * current_height / 2, // 4 bits per pixel
                TextureFormat::Palette8 => current_width * current_height,     // 8 bits per pixel
                TextureFormat::Compressed => {
                    // DXT compression: 4x4 blocks, 8 bytes for BC1/BC2/BC3
                    ((current_width + 3) / 4) * ((current_height + 3) / 4) * 8
                }
            };
            total_size += mipmap_size;

            // Next mipmap level is half the size (minimum 1x1)
            current_width = (current_width / 2).max(1);
            current_height = (current_height / 2).max(1);
        }

        total_size
    }

    fn parse_texture_native(data: &[u8], _version: u32, data_offset: usize) -> Result<TextureInfo, RengineError> {
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

        let mut pos = 0;

        // Read platform_id (4 bytes)
        let platform_id =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
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
        pos += 1; // Advance past raster_type

        // Read platform properties (1 byte)
        let _platform_properties = data[pos];
        pos += 1;

        let format_value = (raster_format_flags >> 8) & 0xFF;

        // Check d3d_format for known compressed formats first
        let format = match d3d_format {
            827611204 => TextureFormat::Compressed, // DXT1
            844388420 => TextureFormat::Compressed, // DXT2
            861165636 => TextureFormat::Compressed, // DXT3
            877942852 => TextureFormat::Compressed, // DXT4
            894720068 => TextureFormat::Compressed, // DXT5
            _ => {
                // Fall back to raster format
                match format_value {
                    0x00 => TextureFormat::RGBA32,
                    0x01 => TextureFormat::RGBA16,
                    0x02 => TextureFormat::Luminance8,
                    0x03 => TextureFormat::LuminanceAlpha8,
                    0x04 => TextureFormat::Palette4,
                    0x05 => TextureFormat::Palette8,
                    0x06 => TextureFormat::Compressed,
                    _ => TextureFormat::RGBA32, // Default
                }
            }
        };

        // Parse raster format (use d3d_format or raster_format_flags)
        let raster_type = d3d_format;

        // Extract palette data if this is a paletted texture
        let palette_data = if matches!(format, TextureFormat::Palette4 | TextureFormat::Palette8) {
            // Palette data comes before texture data
            // Palette4: 16 colors * 4 bytes = 64 bytes
            // Palette8: 256 colors * 4 bytes = 1024 bytes
            let palette_size = match format {
                TextureFormat::Palette4 => 64,
                TextureFormat::Palette8 => 1024,
                _ => 0,
            };

            if data.len() >= pos + palette_size {
                let palette = data[pos..pos + palette_size].to_vec();
                pos += palette_size; // Advance position past palette data
                Some(palette)
            } else {
                None
            }
        } else {
            None
        };

        // For mipmap levels, each level has a 4-byte length prefix followed by pixel data
        // For now, assume single mipmap level and skip the length prefix
        if data.len() >= pos + 4 {
            let mipmap_length = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
            pos += 4; // Skip length prefix
        }

        // Calculate data size based on texture format and dimensions
        let data_size = Self::calculate_texture_data_size(width, height, &format, mipmap_count);

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

        // Calculate the actual data offset - data_offset is absolute file offset to texture data section start,
        // pos is relative offset within section to pixel data start
        let actual_data_offset = data_offset + pos;

        eprintln!("[TXD DEBUG] Parsed texture native: name='{}', width={}, height={}, format={:?}, platform_id=0x{:08X}, data_offset={}, palette_size={}",
            name, width, height, format, platform_id, actual_data_offset,
            palette_data.as_ref().map_or(0, |p| p.len()));

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
            data_offset: actual_data_offset as u32, // Offset to texture data in the file
            renderware_version: None,
            platform_flags: raster_type,
            palette_data, // Palette data extracted for paletted textures
            platform_id,
            d3d_format,
            raster_format_flags,
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

    /// Save the TXD archive to a file
    pub fn save_to_path(&self, path: &str) -> Result<(), RengineError> {
        let mut file = std::fs::File::create(path).map_err(|e| RengineError::FileReadFailed {
            path: path.to_string(),
            details: format!("Failed to create file: {}", e),
        })?;

        // Default RenderWare version if not specified
        let rw_version = self.renderware_version.as_ref()
            .map(|v| v.parse::<u32>().unwrap_or(0x1803FFFF))
            .unwrap_or(0x1803FFFF);

        self.write_txd_archive(&mut file, rw_version)
    }

    /// Write TXD archive data to a writer
    fn write_txd_archive<W: Write>(&self, writer: &mut W, rw_version: u32) -> Result<(), RengineError> {
        // TEXDICTIONARY section header
        writer.write_all(&0x16u32.to_le_bytes())?; // CHUNK_TYPE: TEXDICTIONARY
        let mut struct_data = Vec::new();
        let mut texture_data = Vec::new();

        // Write STRUCT section
        struct_data.extend_from_slice(&1u32.to_le_bytes()); // STRUCT type
        let struct_size_pos = struct_data.len();
        struct_data.extend_from_slice(&0u32.to_le_bytes()); // placeholder for size
        struct_data.extend_from_slice(&rw_version.to_le_bytes()); // version

        // Write texture count
        struct_data.extend_from_slice(&(self.textures.len() as u32).to_le_bytes());

        // Write device ID (0 for default)
        struct_data.extend_from_slice(&0u32.to_le_bytes());

        // Update struct size
        let struct_size = (struct_data.len() - 12) as u32;
        struct_data[struct_size_pos..struct_size_pos + 4].copy_from_slice(&struct_size.to_le_bytes());

        // Write texture sections
        for texture in &self.textures {
            self.write_texture_native(&texture, &mut texture_data, rw_version)?;
        }

        // Write TEXDICTIONARY section size
        let total_size = (struct_data.len() + texture_data.len()) as u32;
        writer.write_all(&total_size.to_le_bytes())?;
        writer.write_all(&rw_version.to_le_bytes())?;

        // Write STRUCT and texture data
        writer.write_all(&struct_data)?;
        writer.write_all(&texture_data)?;

        Ok(())
    }

    /// Write a single texture in TEXTURENATIVE format
    fn write_texture_native<W: Write>(&self, texture: &TextureInfo, writer: &mut W, rw_version: u32) -> Result<(), RengineError> {
        // Calculate texture data size
        let texture_data_size = Self::calculate_texture_data_size(texture.width, texture.height, &texture.format, texture.mipmap_count);

        // TEXTURENATIVE section header
        writer.write_all(&0x15u32.to_le_bytes())?; // TEXTURENATIVE
        let texture_size: u32 = 128 + texture_data_size; // Header + data
        writer.write_all(&texture_size.to_le_bytes())?;
        writer.write_all(&rw_version.to_le_bytes())?;

        // Platform ID
        writer.write_all(&texture.platform_id.to_le_bytes())?;

        // Filter mode and addressing
        let filter_byte = texture.filter_mode.clone() as u8;
        writer.write_all(&[filter_byte])?;

        let addressing_byte = ((texture.addressing_u.clone() as u8) << 4) | (texture.addressing_v.clone() as u8);
        writer.write_all(&[addressing_byte])?;

        // Unknown/reserved
        writer.write_all(&[0u8, 0u8])?;

        // Texture name (32 bytes, null-terminated)
        let mut name_bytes = [0u8; 32];
        let name_len = texture.name.len().min(31);
        name_bytes[..name_len].copy_from_slice(texture.name.as_bytes());
        writer.write_all(&name_bytes)?;

        // Mask name (32 bytes, null-terminated) - using empty for now
        writer.write_all(&[0u8; 32])?;

        // Raster format flags
        writer.write_all(&texture.raster_format_flags.to_le_bytes())?;

        // D3D format
        writer.write_all(&texture.d3d_format.to_le_bytes())?;

        // Dimensions
        writer.write_all(&texture.width.to_le_bytes())?;
        writer.write_all(&texture.height.to_le_bytes())?;
        writer.write_all(&[texture.depth])?;
        writer.write_all(&[texture.mipmap_count])?;
        writer.write_all(&texture.raster_type.to_le_bytes())?; // raster type
        writer.write_all(&[0u8])?; // platform properties

        // Palette data (if paletted)
        if let Some(palette) = &texture.palette_data {
            writer.write_all(palette)?;
        }

        // Mipmap data
        // For each mipmap level, write size prefix + data
        let mut current_width = texture.width as u32;
        let mut current_height = texture.height as u32;
        let mut remaining_data = texture_data_size;

        for _ in 0..texture.mipmap_count {
            let level_size = match texture.format {
                TextureFormat::RGBA32 => current_width * current_height * 4,
                TextureFormat::RGBA16 => current_width * current_height * 2,
                TextureFormat::Luminance8 => current_width * current_height,
                TextureFormat::LuminanceAlpha8 => current_width * current_height * 2,
                TextureFormat::Palette4 => current_width * current_height / 2,
                TextureFormat::Palette8 => current_width * current_height,
                TextureFormat::Compressed => {
                    ((current_width + 3) / 4) * ((current_height + 3) / 4) * 8
                }
            };

            // Write mipmap size prefix
            writer.write_all(&(level_size as u32).to_le_bytes())?;

            // For now, write zeros as placeholder data
            // In a real implementation, you'd have the actual texture data
            let zeros = vec![0u8; level_size as usize];
            writer.write_all(&zeros)?;

            remaining_data -= level_size;

            // Next mipmap level
            current_width = (current_width / 2).max(1);
            current_height = (current_height / 2).max(1);
        }

        Ok(())
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

impl TextureInfo {
    /// Convert texture data to RGBA format for a specific mipmap level
    /// This implements comprehensive texture decoding similar to the Python TXD library
    pub fn to_rgba(&self, archive: &TxdArchive, level: usize) -> Result<Vec<u8>, RengineError> {
        // Get the raw texture data
        let texture_data = archive.get_texture_data(&self.name)?;

        // For now, we'll implement a basic version that handles common formats
        // This is a simplified version - a full implementation would need all the decoder functions

        match self.platform_id {
            // D3D8 platform
            1 => self.decode_d3d8_texture(&texture_data, level),
            // D3D9 platform
            2 => self.decode_d3d9_texture(&texture_data, level),
            // Other platforms - fall back to raster format
            _ => self.decode_raster_texture(&texture_data, level),
        }
    }

    fn decode_d3d8_texture(&self, data: &[u8], level: usize) -> Result<Vec<u8>, RengineError> {
        // Simplified D3D8 decoding - in practice this would need more complex logic
        match self.d3d_format {
            // DXT formats
            827611204 => self.decode_bc1(data, self.width, self.height), // DXT1
            844388420 => self.decode_bc2(data, self.width, self.height, true), // DXT2
            861165636 => self.decode_bc2(data, self.width, self.height, false), // DXT3
            877942852 => self.decode_bc3(data, self.width, self.height, true), // DXT4
            894720068 => self.decode_bc3(data, self.width, self.height, false), // DXT5
            _ => Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("Unsupported D3D8 format: 0x{:08X}", self.d3d_format),
            }),
        }
    }

    fn decode_d3d9_texture(&self, data: &[u8], level: usize) -> Result<Vec<u8>, RengineError> {
        match self.d3d_format {
            21 => self.decode_bgra8888(data, self.width, self.height), // D3D_8888
            22 => self.decode_bgra888(data, self.width, self.height), // D3D_888
            23 => self.decode_bgra565(data, self.width, self.height), // D3D_565
            24 => self.decode_bgra555(data, self.width, self.height), // D3D_555
            25 => self.decode_bgra1555(data, self.width, self.height), // D3D_1555
            26 => self.decode_bgra4444(data, self.width, self.height), // D3D_4444
            50 => self.decode_lum8(data, self.width, self.height), // D3DFMT_L8
            51 => self.decode_lum8a8(data, self.width, self.height), // D3DFMT_A8L8
            // DXT formats
            827611204 => self.decode_bc1(data, self.width, self.height), // DXT1
            844388420 => self.decode_bc2(data, self.width, self.height, true), // DXT2
            861165636 => self.decode_bc2(data, self.width, self.height, false), // DXT3
            877942852 => self.decode_bc3(data, self.width, self.height, true), // DXT4
            894720068 => self.decode_bc3(data, self.width, self.height, false), // DXT5
            _ => Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("Unsupported D3D9 format: 0x{:08X}", self.d3d_format),
            }),
        }
    }

    fn decode_raster_texture(&self, data: &[u8], level: usize) -> Result<Vec<u8>, RengineError> {
        // Check if this is a compressed texture first
        if self.format == TextureFormat::Compressed {
            return self.try_decode_compressed(data, self.width, self.height);
        }

        let raster_format = (self.raster_format_flags >> 8) & 0xFF;
        match raster_format {
            0x01 => self.decode_bgra1555(data, self.width, self.height), // RASTER_1555
            0x02 => self.decode_bgra565(data, self.width, self.height), // RASTER_565
            0x03 => self.decode_bgra4444(data, self.width, self.height), // RASTER_4444
            0x04 => self.decode_lum8(data, self.width, self.height), // RASTER_LUM
            0x05 => self.decode_bgra8888(data, self.width, self.height), // RASTER_8888
            0x06 => self.decode_bgra888(data, self.width, self.height), // RASTER_888
            0x0a => self.decode_bgra555(data, self.width, self.height), // RASTER_555
            _ => {
                // Fall back to palette or compressed if we have palette data
                if self.palette_data.is_some() {
                    self.decode_paletted(data, self.width, self.height)
                } else {
                    // Try compressed formats
                    self.try_decode_compressed(data, self.width, self.height)
                }
            }
        }
    }

    // Decoder implementations (simplified versions)
    fn decode_bgra8888(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        let expected_size = (width as usize * height as usize * 4) as usize;
        if data.len() != expected_size {
            return Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("BGRA8888 data size mismatch: expected {}, got {}", expected_size, data.len()),
            });
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for chunk in data.chunks_exact(4) {
            let b = chunk[0];
            let g = chunk[1];
            let r = chunk[2];
            let a = chunk[3];
            rgba.extend_from_slice(&[r, g, b, a]);
        }
        Ok(rgba)
    }

    fn decode_bgra888(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        let expected_size = (width as usize * height as usize * 3) as usize;
        if data.len() != expected_size {
            return Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("BGRA888 data size mismatch: expected {}, got {}", expected_size, data.len()),
            });
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for chunk in data.chunks_exact(3) {
            let b = chunk[0];
            let g = chunk[1];
            let r = chunk[2];
            rgba.extend_from_slice(&[r, g, b, 255]);
        }
        Ok(rgba)
    }

    fn decode_bgra565(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        let expected_size = (width as usize * height as usize * 2) as usize;
        if data.len() != expected_size {
            return Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("BGRA565 data size mismatch: expected {}, got {}", expected_size, data.len()),
            });
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for chunk in data.chunks_exact(2) {
            let pixel = u16::from_le_bytes([chunk[0], chunk[1]]);
            let r = ((pixel >> 11) & 0x1F) as u8;
            let g = ((pixel >> 5) & 0x3F) as u8;
            let b = (pixel & 0x1F) as u8;

            // Scale to 8-bit
            let r = (r << 3) | (r >> 2);
            let g = (g << 2) | (g >> 4);
            let b = (b << 3) | (b >> 2);

            rgba.extend_from_slice(&[r, g, b, 255]);
        }
        Ok(rgba)
    }

    fn decode_bgra555(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        let expected_size = (width as usize * height as usize * 2) as usize;
        if data.len() != expected_size {
            return Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("BGRA555 data size mismatch: expected {}, got {}", expected_size, data.len()),
            });
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for chunk in data.chunks_exact(2) {
            let pixel = u16::from_le_bytes([chunk[0], chunk[1]]);
            let r = ((pixel >> 10) & 0x1F) as u8;
            let g = ((pixel >> 5) & 0x1F) as u8;
            let b = (pixel & 0x1F) as u8;

            // Scale to 8-bit
            let r = (r << 3) | (r >> 2);
            let g = (g << 3) | (g >> 2);
            let b = (b << 3) | (b >> 2);

            rgba.extend_from_slice(&[r, g, b, 255]);
        }
        Ok(rgba)
    }

    fn decode_bgra1555(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        let expected_size = (width as usize * height as usize * 2) as usize;
        if data.len() != expected_size {
            return Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("BGRA1555 data size mismatch: expected {}, got {}", expected_size, data.len()),
            });
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for chunk in data.chunks_exact(2) {
            let pixel = u16::from_le_bytes([chunk[0], chunk[1]]);
            let r = ((pixel >> 10) & 0x1F) as u8;
            let g = ((pixel >> 5) & 0x1F) as u8;
            let b = (pixel & 0x1F) as u8;
            let a = ((pixel >> 15) & 0x01) as u8;

            // Scale to 8-bit
            let r = (r << 3) | (r >> 2);
            let g = (g << 3) | (g >> 2);
            let b = (b << 3) | (b >> 2);
            let a = if a != 0 { 255 } else { 0 };

            rgba.extend_from_slice(&[r, g, b, a]);
        }
        Ok(rgba)
    }

    fn decode_bgra4444(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        let expected_size = (width as usize * height as usize * 2) as usize;
        if data.len() != expected_size {
            return Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("BGRA4444 data size mismatch: expected {}, got {}", expected_size, data.len()),
            });
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for chunk in data.chunks_exact(2) {
            let pixel = u16::from_le_bytes([chunk[0], chunk[1]]);
            let r = ((pixel >> 8) & 0x0F) as u8;
            let g = ((pixel >> 4) & 0x0F) as u8;
            let b = (pixel & 0x0F) as u8;
            let a = ((pixel >> 12) & 0x0F) as u8;

            // Scale to 8-bit
            let r = (r << 4) | r;
            let g = (g << 4) | g;
            let b = (b << 4) | b;
            let a = (a << 4) | a;

            rgba.extend_from_slice(&[r, g, b, a]);
        }
        Ok(rgba)
    }

    fn decode_lum8(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        let expected_size = width as usize * height as usize;
        if data.len() != expected_size {
            return Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("LUM8 data size mismatch: expected {}, got {}", expected_size, data.len()),
            });
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for &luminance in data {
            rgba.extend_from_slice(&[luminance, luminance, luminance, 255]);
        }
        Ok(rgba)
    }

    fn decode_lum8a8(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        let expected_size = width as usize * height as usize * 2;
        if data.len() != expected_size {
            return Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("LUM8A8 data size mismatch: expected {}, got {}", expected_size, data.len()),
            });
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for chunk in data.chunks_exact(2) {
            let luminance = chunk[0];
            let alpha = chunk[1];
            rgba.extend_from_slice(&[luminance, luminance, luminance, alpha]);
        }
        Ok(rgba)
    }

    fn decode_paletted(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        let palette = self.palette_data.as_ref().ok_or_else(|| RengineError::FileReadFailed {
            path: self.name.clone(),
            details: "Palette data required for paletted texture".to_string(),
        })?;

        let expected_pixels = width as usize * height as usize;

        match self.depth {
            4 => {
                // 4-bit palette
                let expected_data_size = (expected_pixels + 1) / 2;
                if data.len() != expected_data_size {
                    return Err(RengineError::FileReadFailed {
                        path: self.name.clone(),
                        details: format!("Palette4 data size mismatch: expected {}, got {}", expected_data_size, data.len()),
                    });
                }
                if palette.len() != 64 { // 16 colors * 4 bytes
                    return Err(RengineError::FileReadFailed {
                        path: self.name.clone(),
                        details: format!("Palette4 palette size mismatch: expected 64, got {}", palette.len()),
                    });
                }

                let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
                for &byte in data {
                    let idx1 = (byte & 0x0F) as usize;
                    let idx2 = ((byte >> 4) & 0x0F) as usize;
                    rgba.extend_from_slice(&palette[idx1 * 4..idx1 * 4 + 4]);
                    rgba.extend_from_slice(&palette[idx2 * 4..idx2 * 4 + 4]);
                }
                Ok(rgba)
            }
            8 => {
                // 8-bit palette
                if data.len() != expected_pixels {
                    return Err(RengineError::FileReadFailed {
                        path: self.name.clone(),
                        details: format!("Palette8 data size mismatch: expected {}, got {}", expected_pixels, data.len()),
                    });
                }
                if palette.len() != 1024 { // 256 colors * 4 bytes
                    return Err(RengineError::FileReadFailed {
                        path: self.name.clone(),
                        details: format!("Palette8 palette size mismatch: expected 1024, got {}", palette.len()),
                    });
                }

                let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
                for &idx in data {
                    let color_idx = idx as usize * 4;
                    rgba.extend_from_slice(&palette[color_idx..color_idx + 4]);
                }
                Ok(rgba)
            }
            _ => Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("Unsupported palette depth: {}", self.depth),
            }),
        }
    }

    fn try_decode_compressed(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        // Try BC1 (DXT1) first, then BC3 (DXT5)
        match self.decode_bc1(data, width, height) {
            Ok(rgba) => Ok(rgba),
            Err(_) => self.decode_bc3(data, width, height, false),
        }
    }

    fn decode_bc1(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>, RengineError> {
        // BC1/DXT1 decompression using bcndecode
        match decode(data, width as usize, height as usize, BcnEncoding::Bc1, BcnDecoderFormat::RGBA) {
            Ok(rgba_data) => Ok(rgba_data),
            Err(e) => Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("BC1/DXT1 decoding failed: {}", e),
            }),
        }
    }

    fn decode_bc2(&self, data: &[u8], width: u16, height: u16, premultiplied: bool) -> Result<Vec<u8>, RengineError> {
        // BC2/DXT2/DXT3 decompression
        Err(RengineError::FileReadFailed {
            path: self.name.clone(),
            details: "BC2/DXT2/DXT3 decoding not implemented - requires bcndecode or similar library".to_string(),
        })
    }

    fn decode_bc3(&self, data: &[u8], width: u16, height: u16, premultiplied: bool) -> Result<Vec<u8>, RengineError> {
        // BC3/DXT4/DXT5 decompression using bcndecode
        let encoding = if premultiplied { BcnEncoding::Bc2 } else { BcnEncoding::Bc3 };
        match decode(data, width as usize, height as usize, encoding, BcnDecoderFormat::RGBA) {
            Ok(rgba_data) => Ok(rgba_data),
            Err(e) => Err(RengineError::FileReadFailed {
                path: self.name.clone(),
                details: format!("BC3/DXT4/DXT5 decoding failed: {}", e),
            }),
        }
    }
}
