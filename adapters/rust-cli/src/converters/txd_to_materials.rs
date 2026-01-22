use rengine_core::renderware::txd::{TxdArchive, TextureInfo, TextureFormat};
use std::{fs, path::Path};
use anyhow::{Result, anyhow};
use tracing::{info, error};

use crate::ImageFormat;

extern crate image;

pub struct TxdToMaterialsConverter {
    verbose: bool,
    quiet: bool,
}

impl TxdToMaterialsConverter {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        Self { verbose, quiet }
    }

    pub async fn convert(&self, file: &str, output: &str, format: ImageFormat) -> Result<i32> {
        if self.verbose {
            info!("Extracting materials from TXD: {} -> {}", file, output);
        }

        // Load TXD archive
        let archive = match TxdArchive::load_from_path(file) {
            Ok(archive) => archive,
            Err(e) => {
                error!("Failed to load TXD file {}: {}", file, e);
                return Err(anyhow!("Failed to load TXD file: {}", e));
            }
        };

        if archive.textures.is_empty() {
            if !self.quiet {
                println!("No textures found in TXD file");
            }
            return Ok(0);
        }

        // Create output directory
        fs::create_dir_all(output)?;

        let mut extracted = 0;

        for texture in &archive.textures {
            let extension = match format {
                ImageFormat::Png => "png",
                ImageFormat::Jpg => "jpg",
            };

            let texture_filename = format!("{}.{}", texture.name, extension);
            let texture_path = Path::new(output).join(&texture_filename);

            // Try to decode and save the actual texture
            match self.decode_and_save_texture(&archive, texture, &texture_path, &format) {
                Ok(_) => {
                    if self.verbose {
                        info!("Extracted texture: {} ({}x{})", texture.name, texture.width, texture.height);
                    }
                    extracted += 1;
                }
                Err(e) => {
                    error!("Failed to extract texture {}: {}", texture.name, e);
                    // Create an error file instead of placeholder
                    let error_content = format!("Failed to decode texture: {}\nFormat: {:?}\nDimensions: {}x{}\nError: {}",
                        texture.name, texture.format, texture.width, texture.height, e);
                    if let Err(write_err) = fs::write(&texture_path, error_content) {
                        error!("Failed to write error file for {}: {}", texture.name, write_err);
                    }
                }
            }
        }

        // Generate material files
        let material_result = self.generate_material_file(&archive, output, format);
        let materials_txt_result = self.generate_materials_txt_file(&archive, output);

        if !self.quiet {
            println!("Extracted {} textures from {} to {}", extracted, file, output);
            if let Ok(material_file) = &material_result {
                println!("Created material file: {}", material_file);
            }
            if let Ok(materials_txt_file) = &materials_txt_result {
                println!("Created detailed materials file: {}", materials_txt_file);
            }
        }

        Ok(0)
    }

    fn generate_material_file(&self, archive: &TxdArchive, output_dir: &str, format: ImageFormat) -> Result<String> {
        let extension = match format {
            ImageFormat::Png => "png",
            ImageFormat::Jpg => "jpg",
        };

        let material_filename = format!("materials.mtl");
        let material_path = Path::new(output_dir).join(&material_filename);

        let mut mtl_content = String::new();
        mtl_content.push_str("# Material definitions extracted from TXD\n");
        mtl_content.push_str(&format!("# Total textures: {}\n", archive.textures.len()));
        if let Some(version) = &archive.renderware_version {
            mtl_content.push_str(&format!("# RenderWare version: {}\n", version));
        }

        for (i, texture) in archive.textures.iter().enumerate() {
            let material_name = format!("texture_{}", i);
            mtl_content.push_str(&format!("\nnewmtl {}\n", material_name));
            mtl_content.push_str(&format!("  # Original texture: {}\n", texture.name));
            mtl_content.push_str(&format!("  # Dimensions: {}x{}\n", texture.width, texture.height));
            mtl_content.push_str(&format!("  # Format: {:?}, Mipmaps: {}\n", texture.format, texture.mipmap_count));
            mtl_content.push_str(&format!("  # Filter: {:?}, Addressing: U={:?} V={:?}\n",
                texture.filter_mode, texture.addressing_u, texture.addressing_v));

            // Diffuse texture
            mtl_content.push_str(&format!("  map_Kd {}.{}\n", texture.name, extension));

            // Enhanced material properties based on texture format
            match texture.format {
                TextureFormat::RGBA32 | TextureFormat::RGBA16 => {
                    // Full color textures
                    mtl_content.push_str("  Kd 1.0 1.0 1.0\n");
                    mtl_content.push_str("  Ks 0.0 0.0 0.0\n");
                    mtl_content.push_str("  Ka 0.2 0.2 0.2\n");
                    mtl_content.push_str("  Ns 0.0\n");
                    mtl_content.push_str("  illum 1\n");
                }
                TextureFormat::Luminance8 | TextureFormat::LuminanceAlpha8 => {
                    // Grayscale textures
                    mtl_content.push_str("  Kd 0.8 0.8 0.8\n");
                    mtl_content.push_str("  Ks 0.0 0.0 0.0\n");
                    mtl_content.push_str("  Ka 0.5 0.5 0.5\n");
                    mtl_content.push_str("  Ns 0.0\n");
                    mtl_content.push_str("  illum 1\n");
                }
                TextureFormat::Palette4 | TextureFormat::Palette8 => {
                    // Paletted textures
                    mtl_content.push_str("  Kd 1.0 1.0 1.0\n");
                    mtl_content.push_str("  Ks 0.0 0.0 0.0\n");
                    mtl_content.push_str("  Ka 0.2 0.2 0.2\n");
                    mtl_content.push_str("  Ns 0.0\n");
                    mtl_content.push_str("  illum 1\n");
                    mtl_content.push_str("  # Note: Paletted texture format - may need palette data\n");
                }
                TextureFormat::Compressed => {
                    // Compressed textures (DXT)
                    mtl_content.push_str("  Kd 1.0 1.0 1.0\n");
                    mtl_content.push_str("  Ks 0.0 0.0 0.0\n");
                    mtl_content.push_str("  Ka 0.2 0.2 0.2\n");
                    mtl_content.push_str("  Ns 0.0\n");
                    mtl_content.push_str("  illum 1\n");
                    mtl_content.push_str("  # Note: Compressed texture format (DXT) - may need decompression\n");
                }
            }
        }

        fs::write(&material_path, &mtl_content)?;
        Ok(material_path.to_str().unwrap().to_string())
    }

    fn generate_materials_txt_file(&self, archive: &TxdArchive, output_dir: &str) -> Result<String> {
        if archive.textures.is_empty() {
            return Ok(String::new());
        }

        let materials_filename = "materials.txt";
        let materials_path = Path::new(output_dir).join(&materials_filename);

        let mut txt_content = String::new();
        txt_content.push_str("# Material definitions extracted from TXD\n");
        if let Some(version) = &archive.renderware_version {
            txt_content.push_str(&format!("# RenderWare version: {}\n", version));
        }
        txt_content.push_str(&format!("# Texture Count: {}\n", archive.textures.len()));
        txt_content.push_str(&format!("# Source file: {}\n\n", archive.file_path));

        for (idx, texture) in archive.textures.iter().enumerate() {
            txt_content.push_str(&format!("Material_{}:\n", idx));
            txt_content.push_str(&format!("  Name: {}\n", texture.name));
            txt_content.push_str(&format!("  Width: {}\n", texture.width));
            txt_content.push_str(&format!("  Height: {}\n", texture.height));
            txt_content.push_str(&format!("  Depth: {}\n", texture.depth));
            txt_content.push_str(&format!("  Format: {:?}\n", texture.format));
            txt_content.push_str(&format!("  Mipmaps: {}\n", texture.mipmap_count));
            txt_content.push_str(&format!("  Filter Mode: {:?}\n", texture.filter_mode));
            txt_content.push_str(&format!("  Addressing U: {:?}\n", texture.addressing_u));
            txt_content.push_str(&format!("  Addressing V: {:?}\n", texture.addressing_v));
            txt_content.push_str(&format!("  Raster Type: 0x{:08X}\n", texture.raster_type));
            txt_content.push_str(&format!("  Platform Flags: 0x{:08X}\n", texture.platform_flags));
            txt_content.push_str(&format!("  Data Size: {} bytes\n", texture.data_size));

            let extension = if texture.format == TextureFormat::Compressed {
                // For compressed textures, we'll try to save as PNG after decompression
                "png"
            } else {
                "png" // Default to PNG for now, could be configurable
            };
            txt_content.push_str(&format!("  File: {}.{}", texture.name, extension));

            // Add texture format specific notes
            match texture.format {
                TextureFormat::RGBA32 => txt_content.push_str("  # 32-bit RGBA texture\n"),
                TextureFormat::RGBA16 => txt_content.push_str("  # 16-bit RGBA texture (5551 format)\n"),
                TextureFormat::Luminance8 => txt_content.push_str("  # 8-bit grayscale texture\n"),
                TextureFormat::LuminanceAlpha8 => txt_content.push_str("  # 8-bit grayscale with alpha texture\n"),
                TextureFormat::Palette4 => txt_content.push_str("  # 4-bit paletted texture (palette extraction supported)\n"),
                TextureFormat::Palette8 => txt_content.push_str("  # 8-bit paletted texture (palette extraction supported)\n"),
                TextureFormat::Compressed => txt_content.push_str("  # Compressed texture (DXT format - BC1/BC3 decompression supported)\n"),
            }

            txt_content.push_str("\n");
        }

        fs::write(&materials_path, &txt_content)?;
        Ok(materials_path.to_str().unwrap().to_string())
    }

    fn decode_and_save_texture(
        &self,
        archive: &TxdArchive,
        texture: &TextureInfo,
        output_path: &Path,
        format: &ImageFormat,
    ) -> Result<()> {
        // Use the comprehensive to_rgba method from TextureInfo
        let rgba_data = texture.to_rgba(archive, 0)?; // Use mipmap level 0 (highest quality)

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

}