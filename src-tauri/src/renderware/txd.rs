use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use crate::RengineError;

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
        file.read_to_end(&mut buffer).map_err(|e| RengineError::FileReadFailed {
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
        let rw_version = super::versions::RenderWareVersionManager::new().get_rw_version(Some(version as i32));

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
        let mut textures = Vec::new();
        let mut offset = 12; // Skip TEXDICTIONARY header

        // Skip TEXDICTIONARY data section (usually empty or contains version info)
        if offset + 4 <= buffer.len() {
            let data_size = u32::from_le_bytes([
                buffer[offset], buffer[offset + 1], buffer[offset + 2], buffer[offset + 3]
            ]) as usize;
            offset += 4 + data_size;
        }

        // Parse texture native sections
        while offset + 12 <= buffer.len() {
            let section_type = u32::from_le_bytes([
                buffer[offset], buffer[offset + 1], buffer[offset + 2], buffer[offset + 3]
            ]);

            if section_type != 0x0253FF01 { // TEXTURENATIVE
                break; // Not a texture section
            }

            let section_size = u32::from_le_bytes([
                buffer[offset + 4], buffer[offset + 5], buffer[offset + 6], buffer[offset + 7]
            ]) as usize;

            let version = u32::from_le_bytes([
                buffer[offset + 8], buffer[offset + 9], buffer[offset + 10], buffer[offset + 11]
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
        if data.len() < 80 {
            return Err(RengineError::FileReadFailed {
                path: "unknown".to_string(),
                details: "Texture native data too small".to_string(),
            });
        }

        // Parse texture name (32 bytes, null-terminated)
        let name_bytes = &data[0..32];
        let name_end = name_bytes.iter().position(|&b| b == 0).unwrap_or(32);
        let name = String::from_utf8_lossy(&name_bytes[0..name_end]).to_string();

        // Skip filter addressing (4 bytes)
        let filter_addressing = u32::from_le_bytes([data[32], data[33], data[34], data[35]]);

        // Parse dimensions and format
        let width = u16::from_le_bytes([data[36], data[37]]);
        let height = u16::from_le_bytes([data[38], data[39]]);
        let depth = data[40];
        let mipmap_count = data[41];

        let format = match data[42] {
            0x00 => TextureFormat::RGBA32,
            0x01 => TextureFormat::RGBA16,
            0x02 => TextureFormat::Luminance8,
            0x03 => TextureFormat::LuminanceAlpha8,
            0x04 => TextureFormat::Palette4,
            0x05 => TextureFormat::Palette8,
            0x06 => TextureFormat::Compressed,
            _ => TextureFormat::RGBA32, // Default
        };

        // Parse raster format (4 bytes)
        let raster_type = u32::from_le_bytes([data[44], data[45], data[46], data[47]]);

        // Parse data size
        let data_size = u32::from_le_bytes([data[48], data[49], data[50], data[51]]);

        // Extract filter mode and addressing from filter_addressing
        let filter_mode = match (filter_addressing >> 24) & 0xFF {
            0x00 => TextureFilterMode::Nearest,
            0x01 => TextureFilterMode::Linear,
            0x02 => TextureFilterMode::MipNearest,
            0x03 => TextureFilterMode::MipLinear,
            0x04 => TextureFilterMode::LinearMipNearest,
            0x05 => TextureFilterMode::LinearMipLinear,
            _ => TextureFilterMode::Linear,
        };

        let addressing_u = match (filter_addressing >> 16) & 0xFF {
            0x00 => TextureAddressingMode::Wrap,
            0x01 => TextureAddressingMode::Mirror,
            0x02 => TextureAddressingMode::Clamp,
            0x03 => TextureAddressingMode::Border,
            _ => TextureAddressingMode::Wrap,
        };

        let addressing_v = match (filter_addressing >> 8) & 0xFF {
            0x00 => TextureAddressingMode::Wrap,
            0x01 => TextureAddressingMode::Mirror,
            0x02 => TextureAddressingMode::Clamp,
            0x03 => TextureAddressingMode::Border,
            _ => TextureAddressingMode::Wrap,
        };

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
        let texture = self.textures.iter().find(|t| t.name == texture_name)
            .ok_or_else(|| RengineError::FileReadFailed {
                path: texture_name.to_string(),
                details: "Texture not found in TXD archive".to_string(),
            })?;

        let mut file = File::open(&self.file_path).map_err(|e| RengineError::FileReadFailed {
            path: self.file_path.clone(),
            details: e.to_string(),
        })?;

        // Seek to texture data offset
        file.seek(SeekFrom::Start(texture.data_offset as u64)).map_err(|e| RengineError::FileReadFailed {
            path: self.file_path.clone(),
            details: format!("Seek error: {}", e),
        })?;

        // Read texture data
        let mut data = vec![0u8; texture.data_size as usize];
        file.read_exact(&mut data).map_err(|e| RengineError::FileReadFailed {
            path: self.file_path.clone(),
            details: format!("Read error: {}", e),
        })?;

        Ok(data)
    }

    /// Export texture to PNG format (placeholder - would need image processing library)
    pub fn export_texture_to_png(&self, _texture_name: &str, output_path: &str) -> Result<(), RengineError> {
        // This would require an image processing library like image-rs
        // For now, just return an error indicating this feature needs implementation
        Err(RengineError::FileReadFailed {
            path: output_path.to_string(),
            details: "PNG export not yet implemented - requires image processing library".to_string(),
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
        } else { 0 };
        let avg_height = if !self.textures.is_empty() {
            self.textures.iter().map(|t| t.height as u32).sum::<u32>() / self.textures.len() as u32
        } else { 0 };

        let format_counts: std::collections::HashMap<String, usize> = self.textures.iter()
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