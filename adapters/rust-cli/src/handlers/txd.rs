use rengine_core::renderware::txd::{TxdArchive, TextureInfo, TextureFormat};
use std::fs;
use anyhow::{Result, anyhow};
use tracing::{info, error};
use bcndecode::{decode, BcnEncoding, BcnDecoderFormat};

use crate::{OutputFormat, ImageFormat};

extern crate image;

pub struct TXDHandler {
    verbose: bool,
    quiet: bool,
}

impl TXDHandler {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub async fn info(&self, file_path: &str, detailed: bool, format: OutputFormat) -> Result<i32> {
        if self.verbose {
            info!("Loading TXD file: {}", file_path);
        }

        let archive = match TxdArchive::load_from_path(file_path) {
            Ok(archive) => archive,
            Err(e) => {
                error!("Failed to load TXD file {}: {}", file_path, e);
                return Err(anyhow!("Failed to load TXD file: {}", e));
            }
        };

        let file_size = fs::metadata(file_path)?.len();

        let mut info = serde_json::json!({
            "file": file_path,
            "size": file_size,
            "textures": archive.total_textures,
            "rw_version": archive.renderware_version
        });

        if detailed {
            // Add detailed texture information
            let mut texture_info = Vec::new();
            for (i, texture) in archive.textures.iter().enumerate() {
                texture_info.push(serde_json::json!({
                    "index": i,
                    "name": texture.name,
                    "width": texture.width,
                    "height": texture.height,
                    "depth": texture.depth,
                    "format": format!("{:?}", texture.format),
                    "mipmap_count": texture.mipmap_count,
                    "data_size": texture.data_size,
                    "filter_mode": format!("{:?}", texture.filter_mode),
                    "addressing_u": format!("{:?}", texture.addressing_u),
                    "addressing_v": format!("{:?}", texture.addressing_v)
                }));
            }
            info["texture_details"] = serde_json::Value::Array(texture_info);

            // Add format summary
            let mut format_counts = std::collections::HashMap::new();
            for texture in &archive.textures {
                let format_str = format!("{:?}", texture.format);
                *format_counts.entry(format_str).or_insert(0) += 1;
            }
            info["format_summary"] = serde_json::to_value(format_counts)?;
        }

        match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&info)?);
            }
            OutputFormat::Text => {
                if !self.quiet {
                    println!("TXD File: {}", info["file"]);
                    println!("Size: {} bytes", info["size"]);
                    println!("Textures: {}", info["textures"]);
                    if let Some(rw_version) = info["rw_version"].as_str() {
                        println!("RenderWare Version: {}", rw_version);
                    }

                    if detailed {
                        if let Some(texture_details) = info.get("texture_details") {
                            if let Some(textures) = texture_details.as_array() {
                                println!("\nTexture Details:");
                                for texture in textures {
                                    let name = texture["name"].as_str().unwrap_or("unknown");
                                    let width = texture["width"].as_u64().unwrap_or(0);
                                    let height = texture["height"].as_u64().unwrap_or(0);
                                    let format = texture["format"].as_str().unwrap_or("unknown");
                                    let mipmaps = texture["mipmap_count"].as_u64().unwrap_or(0);
                                    println!("  {}: {}x{} ({}), {} mipmaps",
                                            name, width, height, format, mipmaps);
                                }
                            }
                        }

                        if let Some(format_summary) = info.get("format_summary") {
                            println!("\nFormat Summary:");
                            if let Some(obj) = format_summary.as_object() {
                                for (format_name, count) in obj {
                                    println!("  {}: {}", format_name, count);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(0)
    }

    pub async fn create(&self, output_path: &str) -> Result<i32> {
        if self.verbose {
            info!("Creating new TXD file: {}", output_path);
        }

        let mut archive = TxdArchive::with_path(output_path.to_string());
        archive.set_renderware_version("3.6.0.3".to_string());

        match archive.save_to_path(output_path) {
            Ok(_) => {
                if !self.quiet {
                    println!("Created empty TXD file: {}", output_path);
                }
                Ok(0)
            }
            Err(e) => {
                error!("Failed to create TXD file {}: {}", output_path, e);
                Err(anyhow!("Failed to create TXD file: {}", e))
            }
        }
    }

    pub async fn extract(&self, file_path: &str, output_dir: &str, format: ImageFormat) -> Result<i32> {
        if self.verbose {
            info!("Loading TXD file for extraction: {}", file_path);
        }

        let archive = match TxdArchive::load_from_path(file_path) {
            Ok(archive) => archive,
            Err(e) => {
                error!("Failed to load TXD file {}: {}", file_path, e);
                return Err(anyhow!("Failed to load TXD file: {}", e));
            }
        };

        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)?;

        let mut extracted = 0;
        let mut skipped = 0;

        for texture in &archive.textures {
            let extension = match format {
                ImageFormat::Png => "png",
                ImageFormat::Jpg => "jpg",
            };

            let output_path = std::path::Path::new(output_dir)
                .join(format!("{}.{}", texture.name, extension));

            // Try to decode the texture
            match self.decode_and_save_texture(&archive, texture, &output_path, &format).await {
                Ok(_) => {
                    if self.verbose {
                        info!("Extracted texture: {} ({}x{})", texture.name, texture.width, texture.height);
                    }
                    extracted += 1;
                }
                Err(e) => {
                    error!("Failed to extract texture {}: {}", texture.name, e);
                    // Create a placeholder file to indicate the texture exists but couldn't be decoded
                    if let Err(write_err) = std::fs::write(&output_path, format!("Failed to decode texture: {}\nFormat: {:?}\nDimensions: {}x{}\nError: {}", texture.name, texture.format, texture.width, texture.height, e)) {
                        error!("Failed to write error placeholder for {}: {}", texture.name, write_err);
                    }
                    skipped += 1;
                }
            }
        }

        if !self.quiet {
            println!("Extracted {} textures from {} to {}", extracted, file_path, output_dir);
            if skipped > 0 {
                println!("Skipped {} textures due to decoding errors (error files created)", skipped);
            }
        }

        Ok(0)
    }

    async fn decode_and_save_texture(
        &self,
        archive: &TxdArchive,
        texture: &TextureInfo,
        output_path: &std::path::Path,
        format: &ImageFormat,
    ) -> Result<()> {
        // Get the raw texture data
        let texture_data = archive.get_texture_data(&texture.name)?;

        // Decode based on texture format
        let rgba_data = self.decode_texture_data(texture, &texture_data)?;

        // Create image buffer
        let img_buffer = image::RgbaImage::from_raw(texture.width as u32, texture.height as u32, rgba_data)
            .ok_or_else(|| anyhow!("Failed to create image buffer"))?;

        // Save the image
        match format {
            ImageFormat::Png => {
                img_buffer.save_with_format(output_path, image::ImageFormat::Png)?;
            }
            ImageFormat::Jpg => {
                img_buffer.save_with_format(output_path, image::ImageFormat::Jpeg)?;
            }
        }

        Ok(())
    }

    fn decode_texture_data(&self, texture: &TextureInfo, data: &[u8]) -> Result<Vec<u8>> {
        match texture.format {
            TextureFormat::RGBA32 => {
                // Direct RGBA32 format
                if data.len() != (texture.width as usize * texture.height as usize * 4) {
                    return Err(anyhow!("RGBA32 data size mismatch: expected {}, got {}",
                        texture.width as usize * texture.height as usize * 4, data.len()));
                }
                Ok(data.to_vec())
            }
            TextureFormat::RGBA16 => {
                // 16-bit RGBA (5551 format typically)
                self.decode_rgba16(data, texture.width, texture.height)
            }
            TextureFormat::Luminance8 => {
                // 8-bit grayscale
                self.decode_luminance8(data, texture.width, texture.height)
            }
            TextureFormat::LuminanceAlpha8 => {
                // 8-bit grayscale with alpha
                self.decode_luminance_alpha8(data, texture.width, texture.height)
            }
            TextureFormat::Palette4 | TextureFormat::Palette8 => {
                // Paletted formats - need palette data
                self.decode_paletted_texture(texture, data)
            }
            TextureFormat::Compressed => {
                // DXT compression - decode based on platform format
                self.decode_compressed_texture(texture, data)
            }
        }
    }

    fn decode_rgba16(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>> {
        let expected_size = width as usize * height as usize * 2;
        if data.len() != expected_size {
            return Err(anyhow!("RGBA16 data size mismatch: expected {}, got {}", expected_size, data.len()));
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);

        for chunk in data.chunks_exact(2) {
            let pixel = u16::from_le_bytes([chunk[0], chunk[1]]);

            // Extract 5-bit components (RGBA 5551 format)
            let r = ((pixel >> 11) & 0x1F) as u8;
            let g = ((pixel >> 6) & 0x1F) as u8;
            let b = ((pixel >> 1) & 0x1F) as u8;
            let a = (pixel & 0x01) as u8;

            // Scale to 8-bit
            let r = (r << 3) | (r >> 2);
            let g = (g << 3) | (g >> 2);
            let b = (b << 3) | (b >> 2);
            let a = if a != 0 { 255 } else { 0 };

            rgba.extend_from_slice(&[r, g, b, a]);
        }

        Ok(rgba)
    }

    fn decode_luminance8(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>> {
        let expected_size = width as usize * height as usize;
        if data.len() != expected_size {
            return Err(anyhow!("Luminance8 data size mismatch: expected {}, got {}", expected_size, data.len()));
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);

        for &luminance in data {
            rgba.extend_from_slice(&[luminance, luminance, luminance, 255]);
        }

        Ok(rgba)
    }

    fn decode_luminance_alpha8(&self, data: &[u8], width: u16, height: u16) -> Result<Vec<u8>> {
        let expected_size = width as usize * height as usize * 2;
        if data.len() != expected_size {
            return Err(anyhow!("LuminanceAlpha8 data size mismatch: expected {}, got {}", expected_size, data.len()));
        }

        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);

        for chunk in data.chunks_exact(2) {
            let luminance = chunk[0];
            let alpha = chunk[1];
            rgba.extend_from_slice(&[luminance, luminance, luminance, alpha]);
        }

        Ok(rgba)
    }

    fn decode_paletted_texture(&self, texture: &TextureInfo, data: &[u8]) -> Result<Vec<u8>> {
        // Check if we have palette data
        let palette = texture.palette_data.as_ref()
            .ok_or_else(|| anyhow!("Paletted texture missing palette data"))?;

        let width = texture.width as usize;
        let height = texture.height as usize;
        let mut rgba = Vec::with_capacity(width * height * 4);

        match texture.format {
            TextureFormat::Palette8 => {
                // 8-bit palette indices
                if data.len() != width * height {
                    return Err(anyhow!("Palette8 data size mismatch: expected {}, got {}",
                        width * height, data.len()));
                }

                for &index in data {
                    if (index as usize) < palette.len() / 4 {
                        let start = (index as usize) * 4;
                        rgba.extend_from_slice(&palette[start..start + 4]);
                    } else {
                        // Invalid palette index, use transparent
                        rgba.extend_from_slice(&[0, 0, 0, 0]);
                    }
                }
            }
            TextureFormat::Palette4 => {
                // 4-bit palette indices (2 pixels per byte)
                let expected_size = (width * height + 1) / 2;
                if data.len() != expected_size {
                    return Err(anyhow!("Palette4 data size mismatch: expected {}, got {}",
                        expected_size, data.len()));
                }

                for &byte in data {
                    // Extract two 4-bit indices from each byte
                    let index1 = (byte >> 4) & 0x0F;
                    let index2 = byte & 0x0F;

                    // Add first pixel
                    if (index1 as usize) < palette.len() / 4 {
                        let start = (index1 as usize) * 4;
                        rgba.extend_from_slice(&palette[start..start + 4]);
                    } else {
                        rgba.extend_from_slice(&[0, 0, 0, 0]);
                    }

                    // Add second pixel
                    if (index2 as usize) < palette.len() / 4 {
                        let start = (index2 as usize) * 4;
                        rgba.extend_from_slice(&palette[start..start + 4]);
                    } else {
                        rgba.extend_from_slice(&[0, 0, 0, 0]);
                    }
                }

                // If odd number of pixels, we might have added one extra - trim it
                rgba.truncate(width * height * 4);
            }
            _ => return Err(anyhow!("Invalid format for paletted texture decoding")),
        }

        Ok(rgba)
    }

    fn decode_compressed_texture(&self, texture: &TextureInfo, data: &[u8]) -> Result<Vec<u8>> {
        // Determine the DXT format from platform-specific data
        let dxt_format = match texture.d3d_format {
            // DXT1 formats (BC1)
            0x31545844 | 827611204 => BcnEncoding::Bc1,
            // DXT2 formats (BC2)
            0x32545844 | 844388420 => BcnEncoding::Bc2,
            // DXT3 formats (BC2)
            0x33545844 | 861165636 => BcnEncoding::Bc2,
            // DXT4 formats (BC3)
            0x34545844 | 877942852 => BcnEncoding::Bc3,
            // DXT5 formats (BC3)
            0x35545844 | 894720068 => BcnEncoding::Bc3,
            _ => {
                // Try to infer from raster format flags or platform ID
                match texture.raster_format_flags {
                    0x01 => BcnEncoding::Bc1, // DXT1
                    0x02 => BcnEncoding::Bc2, // DXT2/DXT3
                    0x03 => BcnEncoding::Bc3, // DXT4/DXT5
                    _ => {
                        // Try platform-specific defaults
                        match texture.platform_id {
                            8 | 9 => BcnEncoding::Bc1, // D3D8/D3D9 default to DXT1
                            4 => BcnEncoding::Bc1,     // PS2 default
                            5 => BcnEncoding::Bc1,     // XBOX default
                            _ => return Err(anyhow!("Unknown compressed texture format: d3d_format=0x{:08X}, raster_flags=0x{:08X}, platform_id={}",
                                texture.d3d_format, texture.raster_format_flags, texture.platform_id)),
                        }
                    }
                }
            }
        };

        // Calculate expected compressed size for one mipmap level
        let block_width = (texture.width as usize + 3) / 4;
        let block_height = (texture.height as usize + 3) / 4;
        let expected_size = match dxt_format {
            BcnEncoding::Bc1 => block_width * block_height * 8,  // 8 bytes per 4x4 block
            BcnEncoding::Bc2 | BcnEncoding::Bc3 => block_width * block_height * 16, // 16 bytes per 4x4 block
            _ => return Err(anyhow!("Unsupported BCn encoding")),
        };

        // For mipmap level 0, check if data matches expected size
        if data.len() < expected_size {
            return Err(anyhow!("Compressed data too small: expected at least {}, got {}",
                expected_size, data.len()));
        }

        // Decode only the first mipmap level (most textures are single-level in TXD)
        let width = texture.width as usize;
        let height = texture.height as usize;

        // Take only the first mipmap level data
        let level_data = &data[0..expected_size];

        match decode(level_data, width, height, dxt_format, BcnDecoderFormat::RGBA) {
            Ok(decoded) => Ok(decoded),
            Err(e) => Err(anyhow!("Failed to decode compressed texture: {} (size: {}x{})",
                e, width, height)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_txd_handler_creation() {
        let handler = TXDHandler::new(false, false);
        assert!(!handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_txd_handler_with_verbose() {
        let handler = TXDHandler::new(true, false);
        assert!(handler.verbose);
        assert!(!handler.quiet);
    }

    #[tokio::test]
    async fn test_txd_handler_quiet() {
        let handler = TXDHandler::new(false, true);
        assert!(!handler.verbose);
        assert!(handler.quiet);
    }
}