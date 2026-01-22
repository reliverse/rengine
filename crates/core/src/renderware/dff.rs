use crate::renderware::dfx;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use thiserror::Error;

// RenderWare section types
#[allow(dead_code)]
const RW_SECTION_STRUCT: u32 = 1;
#[allow(dead_code)]
const RW_SECTION_STRING: u32 = 2;
#[allow(dead_code)]
const RW_SECTION_EXTENSION: u32 = 3;
#[allow(dead_code)]
const RW_SECTION_TEXTURE: u32 = 6;
#[allow(dead_code)]
const RW_SECTION_MATERIAL: u32 = 7;
#[allow(dead_code)]
const RW_SECTION_MATERIAL_LIST: u32 = 8;
#[allow(dead_code)]
const RW_SECTION_FRAME_LIST: u32 = 14;
#[allow(dead_code)]
const RW_SECTION_GEOMETRY: u32 = 15;
#[allow(dead_code)]
const RW_SECTION_CLUMP: u32 = 16;
#[allow(dead_code)]
const RW_SECTION_ATOMIC: u32 = 20;
#[allow(dead_code)]
const RW_SECTION_TEXTURE_NATIVE: u32 = 21;
#[allow(dead_code)]
const RW_SECTION_TEXTURE_DICTIONARY: u32 = 22;
#[allow(dead_code)]
const RW_SECTION_GEOMETRY_LIST: u32 = 26;
#[allow(dead_code)]
const RW_SECTION_ANIMATION_ANIM: u32 = 43;
#[allow(dead_code)]
const RW_SECTION_UV_ANIMATION_DICTIONARY: u32 = 43;
#[allow(dead_code)]
const RW_SECTION_USER_DATA_PLG: u32 = 287;
#[allow(dead_code)]
const RW_SECTION_MATERIAL_EFFECTS_PLG: u32 = 288;
#[allow(dead_code)]
const RW_SECTION_UV_ANIMATION_PLG: u32 = 309;
#[allow(dead_code)]
const RW_SECTION_SPECULAR_MATERIAL: u32 = 39056118;
#[allow(dead_code)]
const RW_SECTION_REFLECTION_MATERIAL: u32 = 39056124;
#[allow(dead_code)]
const RW_SECTION_2D_EFFECT: u32 = 39056120;

// Geometry flags
#[allow(dead_code)]
const RP_GEOMETRY_POSITIONS: u32 = 0x00000002;
#[allow(dead_code)]
const RP_GEOMETRY_TEXTURED: u32 = 0x00000004;
#[allow(dead_code)]
const RP_GEOMETRY_PRELIT: u32 = 0x00000008;
#[allow(dead_code)]
const RP_GEOMETRY_NORMALS: u32 = 0x00000010;
#[allow(dead_code)]
const RP_GEOMETRY_LIGHT: u32 = 0x00000020;
#[allow(dead_code)]
const RP_GEOMETRY_MODULATE_MATERIAL_COLOR: u32 = 0x00000040;
#[allow(dead_code)]
const RP_GEOMETRY_TEXTURED2: u32 = 0x00000080;

#[derive(Debug, Error)]
pub enum DffError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid DFF format: {0}")]
    InvalidFormat(String),
    #[error("Unsupported RenderWare version: {0}")]
    #[allow(dead_code)]
    UnsupportedVersion(u32),
    #[error("Section not found: {0}")]
    #[allow(dead_code)]
    SectionNotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matrix3x3 {
    pub right: Vector3,
    pub up: Vector3,
    pub at: Vector3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub name: String,
    pub parent: i32,
    pub position: Vector3,
    pub rotation_matrix: Matrix3x3,
    pub bone_data: Option<BoneData>,
    pub user_data: Option<UserData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneData {
    pub bone_id: i32,
    pub bone_index: i32,
    pub bone_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub sections: Vec<UserDataSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataSection {
    pub name: String,
    pub data: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UVLayer {
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Triangle {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub color: Color,
    pub textures: Vec<Texture>,
    pub surface_properties: SurfaceProperties,
    pub effects: Vec<MaterialEffect>,
    pub user_data: Option<MaterialUserData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Texture {
    pub name: String,
    pub mask: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceProperties {
    pub ambient: f32,
    pub specular: f32,
    pub diffuse: f32,
}

// Material Effects (MatFX) System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlendMode {
    NoBlend = 0x00,
    Zero = 0x01,
    One = 0x02,
    SrcColor = 0x03,
    InvSrcColor = 0x04,
    SrcAlpha = 0x05,
    InvSrcAlpha = 0x06,
    DestAlpha = 0x07,
    InvDestAlpha = 0x08,
    DestColor = 0x09,
    InvDestColor = 0x0A,
    SrcAlphaSat = 0x0B,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BumpMapEffect {
    pub intensity: f32,
    pub bump_texture: Option<Texture>,
    pub height_texture: Option<Texture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentMapEffect {
    pub coefficient: f32,
    pub use_fb_alpha: bool,
    pub env_texture: Option<Texture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualTextureEffect {
    pub src_blend: BlendMode,
    pub dst_blend: BlendMode,
    pub texture: Option<Texture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecularMaterial {
    pub level: f32,
    pub texture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionMaterial {
    pub scale_x: f32,
    pub scale_y: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UVFrame {
    pub time: f32,
    pub uv: [f32; 8], // UV coordinate values
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlatformType {
    PC,
    PS2,
    Xbox,
    GameCube,
    PSP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialEffect {
    BumpMap(BumpMapEffect),
    EnvironmentMap(EnvironmentMapEffect),
    DualTexture(DualTextureEffect),
    Specular(SpecularMaterial),
    Reflection(ReflectionMaterial),
    UVAnimation(Vec<UVFrame>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformNativeGeometry {
    pub platform: PlatformType,
    pub vertex_data: Vec<u8>,
    pub index_data: Vec<u8>,
    pub shader_data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialUserData {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    pub flags: u32,
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub uv_layers: Vec<Vec<UVLayer>>,
    pub triangles: Vec<Triangle>,
    pub materials: Vec<Material>,
    pub native_geometry: Option<PlatformNativeGeometry>,
    pub extensions: HashMap<String, ExtensionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionData {
    Skin(SkinData),
    Raw(Vec<u8>),
    // Add other extension types as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinData {
    pub num_bones: u32,
    pub max_weights_per_vertex: u32,
    pub used_bones: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Atomic {
    pub frame: u32,
    pub geometry: u32,
    pub flags: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UVAnimation {
    pub type_id: u32,
    pub flags: u32,
    pub duration: f32,
    pub name: String,
    pub node_to_uv: [f32; 8],
    pub frames: Vec<UVFrame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DffModel {
    pub rw_version: u32,
    pub frames: Vec<Frame>,
    pub geometries: Vec<Geometry>,
    pub atomics: Vec<Atomic>,
    pub uv_animations: Vec<UVAnimation>,
    pub effects_2dfx: Option<dfx::Effects2DFX>,
    pub file_path: String,
}

impl DffModel {
    /// Quickly check if a file appears to be a valid DFF file by reading the header
    pub fn is_valid_dff_file(path: &str) -> Result<bool, DffError> {
        // Basic file validation before attempting to load
        let metadata = std::fs::metadata(path)?;
        if metadata.len() < 12 {
            // Minimum size for a CLUMP header (type + size + version = 12 bytes)
            return Ok(false);
        }

        let mut file = std::fs::File::open(path)?;
        let (section_type, _, _) = Self::read_section_header(&mut file)?;

        // Check if it starts with a CLUMP section (0x0010)
        Ok(section_type == 0x0010)
    }

    pub fn load_from_path(path: &str) -> Result<Self, DffError> {
        // Basic file validation before attempting to load
        let metadata = std::fs::metadata(path)?;
        if metadata.len() < 12 {
            // Minimum size for a CLUMP header (type + size + version = 12 bytes)
            return Err(DffError::InvalidFormat(
                format!("File too small to be a valid DFF ({} bytes, minimum 12 bytes)", metadata.len())
            ));
        }

        let mut file = std::fs::File::open(path)?;
        Self::load_from_reader(&mut file, path)
    }

    pub fn load_from_reader<R: Read + Seek>(reader: &mut R, path: &str) -> Result<Self, DffError> {
        // Read the main CLUMP section header
        let (section_type, _section_size, rw_version) = match Self::read_section_header(reader) {
            Ok(header) => header,
            Err(DffError::Io(e)) => {
                match e.kind() {
                    std::io::ErrorKind::UnexpectedEof => {
                        return Err(DffError::InvalidFormat(
                            format!("File appears to be truncated or corrupted (unable to read header from {})", path)
                        ));
                    }
                    _ => {
                        return Err(DffError::InvalidFormat(
                            format!("IO error while reading DFF header from {}: {}", path, e)
                        ));
                    }
                }
            }
            Err(e) => return Err(e),
        };

        if section_type != 0x0010 {
            // CLUMP
            let section_name = Self::get_section_name(section_type);
            return Err(DffError::InvalidFormat(
                format!("Invalid DFF format: Expected CLUMP section (0x0010), found {} (0x{:04X}). This appears to be a {} file, not a DFF model file: {}",
                    section_name, section_type, section_name, path),
            ));
        }

        // Read the STRUCT child section header
        let (_struct_type, _struct_size, _struct_version) = Self::read_section_header(reader)?;

        // Read CLUMP STRUCT data (atomics, lights, cameras)
        let _num_atomics = Self::read_u32(reader)?;
        let _num_lights = Self::read_u32(reader)?;
        let _num_cameras = Self::read_u32(reader)?;

        // Skip to frame list
        match Self::skip_to_section(reader, 0x000E) { // FRAME_LIST
            Ok(_) => {}
            Err(DffError::InvalidFormat(msg)) if msg.contains("not found") => {
                return Err(DffError::InvalidFormat(format!(
                    "FRAME_LIST section not found in DFF file: {}. File may be corrupted or truncated.", msg
                )));
            }
            Err(e) => return Err(e),
        }

        let frames = match Self::read_frame_list(reader) {
            Ok(frames) => frames,
            Err(e) => {
                return Err(DffError::InvalidFormat(format!(
                    "Failed to read frame list: {}. File may be corrupted.", e
                )));
            }
        };

        // Skip to geometry list
        match Self::skip_to_section(reader, 0x001A) { // GEOMETRY_LIST
            Ok(_) => {}
            Err(DffError::InvalidFormat(msg)) if msg.contains("not found") => {
                return Err(DffError::InvalidFormat(format!(
                    "GEOMETRY_LIST section not found in DFF file: {}. File may be corrupted or truncated.", msg
                )));
            }
            Err(e) => return Err(e),
        }

        let geometries = match Self::read_geometry_list(reader) {
            Ok(geometries) => geometries,
            Err(e) => {
                return Err(DffError::InvalidFormat(format!(
                    "Failed to read geometry list: {}. File may be corrupted.", e
                )));
            }
        };

        // Try to skip to atomic list (may not be present in corrupted files)
        let atomics = match Self::skip_to_section(reader, 0x0014) { // ATOMIC_LIST
            Ok(_) => {
                match Self::read_atomic_list(reader) {
                    Ok(atomics) => atomics,
                    Err(_e) => {
                        // If atomic list reading fails, continue with empty list
                        Vec::new()
                    }
                }
            }
            Err(_) => {
                // ATOMIC_LIST not found, continue with empty list
                Vec::new()
            }
        };

        // Try to parse 2DFX effects from the last geometry's extensions
        let effects_2dfx = Self::extract_2dfx_effects(&geometries);

        // Try to read UV animations (seek back to beginning and look for UV Animation Dictionary)
        let uv_animations = Self::read_uv_animations(reader)?;

        Ok(DffModel {
            rw_version,
            frames,
            geometries,
            atomics,
            uv_animations,
            effects_2dfx,
            file_path: path.to_string(),
        })
    }

    fn read_section_header<R: Read>(reader: &mut R) -> Result<(u32, u32, u32), DffError> {
        let section_type = Self::read_u32(reader)?;
        let section_size = Self::read_u32(reader)?;
        let rw_version = Self::read_u32(reader)?;
        Ok((section_type, section_size, rw_version))
    }

    fn get_section_name(section_type: u32) -> &'static str {
        match section_type {
            0x0001 => "STRUCT",
            0x0002 => "STRING",
            0x0003 => "EXTENSION",
            0x0006 => "TEXTURE",
            0x0007 => "MATERIAL",
            0x0008 => "MATERIAL_LIST",
            0x000E => "FRAME_LIST",
            0x000F => "GEOMETRY",
            0x0010 => "CLUMP",
            0x0014 => "ATOMIC",
            0x0015 => "TEXTURE_NATIVE",
            0x0016 => "TEXTURE_DICTIONARY",
            0x001A => "GEOMETRY_LIST",
            0x002B => "ANIMATION/UV_ANIMATION",
            0x011F => "USER_DATA_PLG",
            0x0120 => "MATERIAL_EFFECTS_PLG",
            0x0139 => "UV_ANIMATION_PLG",
            0x1803FFF7 => "SPECULAR_MATERIAL",
            0x1803FFF8 => "REFLECTION_MATERIAL",
            0x1803FFF9 => "2D_EFFECT",
            _ => "UNKNOWN",
        }
    }

    fn read_u32<R: Read>(reader: &mut R) -> Result<u32, DffError> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn read_u16<R: Read>(reader: &mut R) -> Result<u16, DffError> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    fn read_f32<R: Read>(reader: &mut R) -> Result<f32, DffError> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    fn read_string<R: Read>(reader: &mut R, len: usize) -> Result<String, DffError> {
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        // Remove null terminators
        if let Some(null_pos) = buf.iter().position(|&c| c == 0) {
            buf.truncate(null_pos);
        }
        String::from_utf8(buf)
            .map_err(|e| DffError::InvalidFormat(format!("Invalid UTF-8 string: {}", e)))
    }

    fn skip_to_section<R: Read + Seek>(reader: &mut R, target_type: u32) -> Result<(), DffError> {
        // Get current position to track bounds
        let start_pos = reader.stream_position()?;

        // Get file size if possible (for bounds checking)
        let file_size = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(start_pos))?;

        // Maximum iterations to prevent infinite loops
        const MAX_ITERATIONS: usize = 10000;
        let mut iterations = 0;

        loop {
            // Check iteration limit
            if iterations >= MAX_ITERATIONS {
                return Err(DffError::InvalidFormat(format!(
                    "Section type 0x{:X} not found after {} iterations",
                    target_type, MAX_ITERATIONS
                )));
            }
            iterations += 1;

            // Check if we've gone past file bounds
            let current_pos = reader.stream_position()?;
            if current_pos >= file_size {
                return Err(DffError::InvalidFormat(format!(
                    "Section type 0x{:X} not found before end of file",
                    target_type
                )));
            }

            let (section_type, section_size, _) = Self::read_section_header(reader)?;

            if section_type == target_type {
                return Ok(());
            }

            // Skip the section data
            let new_pos = reader.stream_position()? + section_size as u64;
            if new_pos > file_size {
                return Err(DffError::InvalidFormat(format!(
                    "Section size {} exceeds file bounds",
                    section_size
                )));
            }
            reader.seek(SeekFrom::Current(section_size as i64))?;
        }
    }

    fn read_frame_list<R: Read + Seek>(reader: &mut R) -> Result<Vec<Frame>, DffError> {
        // Read the STRUCT child section header
        let (_struct_type, _struct_size, _struct_version) = Self::read_section_header(reader)?;

        let num_frames = Self::read_u32(reader)?;

        let mut frames = Vec::new();

        // Frames are stored contiguously - 56 bytes each, NO section headers per frame
        for _i in 0..num_frames {
            // Read frame header (exactly 56 bytes as per RenderWare format)
            // Matrix (36 bytes: 9 floats)
            let rotation_matrix = Matrix3x3 {
                right: Vector3 {
                    x: Self::read_f32(reader)?,
                    y: Self::read_f32(reader)?,
                    z: Self::read_f32(reader)?,
                },
                up: Vector3 {
                    x: Self::read_f32(reader)?,
                    y: Self::read_f32(reader)?,
                    z: Self::read_f32(reader)?,
                },
                at: Vector3 {
                    x: Self::read_f32(reader)?,
                    y: Self::read_f32(reader)?,
                    z: Self::read_f32(reader)?,
                },
            };

            // Position (12 bytes: 3 floats)
            let position = Vector3 {
                x: Self::read_f32(reader)?,
                y: Self::read_f32(reader)?,
                z: Self::read_f32(reader)?,
            };

            // Parent (4 bytes: 1 i32)
            let parent = Self::read_u32(reader)? as i32;

            // Creation flags (4 bytes: 1 u32)
            let _creation_flags = Self::read_u32(reader)?;

            let name = String::new();

            frames.push(Frame {
                name,
                parent: parent as i32,
                position,
                rotation_matrix,
                bone_data: None,
                user_data: None,
            });
        }

        Ok(frames)
    }

    fn read_geometry_list<R: Read + Seek>(reader: &mut R) -> Result<Vec<Geometry>, DffError> {
        // Read the STRUCT child section header
        let (_struct_type, _struct_size, _struct_version) = Self::read_section_header(reader)?;

        let num_geometries = Self::read_u32(reader)?;

        // Check if num_geometries is reasonable
        if num_geometries > 1000 {
            return Err(DffError::InvalidFormat(format!(
                "Unreasonable number of geometries: {}. File may be corrupted.", num_geometries
            )));
        }

        let mut geometries = Vec::new();

        for _i in 0..num_geometries {
            // Check if we have enough data for another geometry header
            let current_pos = reader.stream_position()?;
            let file_size = reader.seek(SeekFrom::End(0))?;
            reader.seek(SeekFrom::Start(current_pos))?;

            if current_pos + 12 > file_size {
                // Not enough data for another geometry, stop here
                break;
            }

            match Self::read_geometry(reader) {
                Ok(geometry) => geometries.push(geometry),
                Err(_e) => {
                    // If we can't read this geometry, stop and return what we have
                    // This handles truncated files gracefully
                    break;
                }
            }
        }

        Ok(geometries)
    }

    fn read_geometry<R: Read + Seek>(reader: &mut R) -> Result<Geometry, DffError> {
        // First read the GEOMETRY section header
        let (_geom_type, _geom_size, _geom_version) = Self::read_section_header(reader)?;

        // Then read the STRUCT child section header
        let (_struct_type, _struct_size, _struct_version) = Self::read_section_header(reader)?;

        let flags = Self::read_u32(reader)?;
        let num_triangles = Self::read_u32(reader)?;
        let num_vertices = Self::read_u32(reader)?;
        let _num_morph_targets = Self::read_u32(reader)?;

        // If native geometry, data is stored differently
        let is_native = flags & 0x01000000 != 0;

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uv_layers = Vec::new();
        let mut triangles = Vec::new();

        if !is_native {
            // Read prelit colors if present (rpGEOMETRYPRELIT = 0x08)
            if flags & 0x00000008 != 0 {
                for _ in 0..num_vertices {
                    let _r = Self::read_u8(reader)?;
                    let _g = Self::read_u8(reader)?;
                    let _b = Self::read_u8(reader)?;
                    let _a = Self::read_u8(reader)?;
                }
            }

            // Read UV coordinates (rpGEOMETRYTEXTURED = 0x04 or rpGEOMETRYTEXTURED2 = 0x80)
            let has_tex = flags & 0x00000004 != 0;
            let has_tex2 = flags & 0x00000080 != 0;
            let mut num_uv_layers = ((flags >> 16) & 0xFF) as usize;
            if num_uv_layers == 0 {
                num_uv_layers = if has_tex2 {
                    2
                } else if has_tex {
                    1
                } else {
                    0
                };
            }

            for _ in 0..num_uv_layers {
                let mut uv_layer = Vec::new();
                for _ in 0..num_vertices {
                    uv_layer.push(UVLayer {
                        u: Self::read_f32(reader)?,
                        v: Self::read_f32(reader)?,
                    });
                }
                uv_layers.push(uv_layer);
            }

            // Read triangles (8 bytes each: b, a, material, c in RW format)
            for _ in 0..num_triangles {
                let b = Self::read_u16(reader)?;
                let a = Self::read_u16(reader)?;
                let _material_index = Self::read_u16(reader)?;
                let c = Self::read_u16(reader)?;
                triangles.push(Triangle { a, b, c });
            }
        }

        // Read morph target data (bounding sphere + vertices + normals)
        // Bounding sphere (16 bytes: x, y, z, radius)
        let _bsphere_x = Self::read_f32(reader)?;
        let _bsphere_y = Self::read_f32(reader)?;
        let _bsphere_z = Self::read_f32(reader)?;
        let _bsphere_r = Self::read_f32(reader)?;

        // Has positions and has normals flags
        let has_positions = Self::read_u32(reader)?;
        let has_normals_flag = Self::read_u32(reader)?;

        // Read vertices from morph target
        if has_positions != 0 && !is_native {
            for _ in 0..num_vertices {
                vertices.push(Vector3 {
                    x: Self::read_f32(reader)?,
                    y: Self::read_f32(reader)?,
                    z: Self::read_f32(reader)?,
                });
            }
        }

        // Read normals from morph target
        if has_normals_flag != 0 && !is_native {
            for _ in 0..num_vertices {
                normals.push(Vector3 {
                    x: Self::read_f32(reader)?,
                    y: Self::read_f32(reader)?,
                    z: Self::read_f32(reader)?,
                });
            }
        }

        // Try to read the MATERIAL_LIST section (it may not be present)
        let mut materials = Vec::new();
        let current_pos = reader.stream_position()?;

        // Check if we have enough data for a potential MATERIAL_LIST header
        let file_size = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(current_pos))?;

        if current_pos + 12 <= file_size {
            // Try to read what might be a MATERIAL_LIST header
            let (potential_type, potential_size, _potential_version) = Self::read_section_header(reader)?;

            if potential_type == RW_SECTION_MATERIAL_LIST {
                // We found a MATERIAL_LIST, read it
                let matlist_end_pos = current_pos + 12 + potential_size as u64;
                if matlist_end_pos > file_size {
                    return Err(DffError::InvalidFormat(format!(
                        "MATERIAL_LIST section size {} exceeds file bounds (would end at 0x{:X}, file size 0x{:X})",
                        potential_size, matlist_end_pos, file_size
                    )));
                }

                // Read MATERIAL_LIST STRUCT
                let (_ml_struct_type, _ml_struct_size, _ml_struct_version) =
                    Self::read_section_header(reader)?;

                // Read materials count
                let num_materials = Self::read_u32(reader)?;

                // Check if num_materials is reasonable (prevent excessive memory allocation)
                if num_materials > 1000 {
                    return Err(DffError::InvalidFormat(format!(
                        "Unreasonable number of materials: {}. File may be corrupted.", num_materials
                    )));
                }

                // Skip material indices array (num_materials * 4 bytes)
                for i in 0..num_materials {
                    let current_pos = reader.stream_position()?;
                    if current_pos + 4 > matlist_end_pos {
                        return Err(DffError::InvalidFormat(format!(
                            "Material index {} truncated - not enough data in MATERIAL_LIST section", i
                        )));
                    }
                    let _mat_idx = Self::read_u32(reader)?;
                }

                for i in 0..num_materials {
                    let current_pos = reader.stream_position()?;
                    if current_pos >= matlist_end_pos {
                        return Err(DffError::InvalidFormat(format!(
                            "Material {} starts beyond MATERIAL_LIST section bounds (at 0x{:X}, section ends at 0x{:X})",
                            i, current_pos, matlist_end_pos
                        )));
                    }
                    match Self::read_material(reader) {
                        Ok(material) => materials.push(material),
                        Err(e) => {
                            return Err(DffError::InvalidFormat(format!(
                                "Failed to read material {}: {}. File may be corrupted.", i, e
                            )));
                        }
                    }
                }
            } else {
                // Not a MATERIAL_LIST, rewind to the previous position
                reader.seek(SeekFrom::Start(current_pos))?;
            }
        }

        // Check for native geometry
        let native_geometry = if is_native {
            // Check if we have enough data for native geometry header
            let current_pos = reader.stream_position()?;
            let file_size = reader.seek(SeekFrom::End(0))?;
            reader.seek(SeekFrom::Start(current_pos))?;

            if current_pos + 12 <= file_size {
                // Try to read native geometry
                match Self::read_native_geometry(reader, flags) {
                    Ok(native) => Some(native),
                    Err(_) => {
                        // If native geometry reading fails, continue without it
                        // Some files may have the native flag set but corrupted data
                        reader.seek(SeekFrom::Start(current_pos))?; // Rewind
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(Geometry {
            flags,
            vertices,
            normals,
            uv_layers,
            triangles,
            materials,
            native_geometry,
            extensions: HashMap::new(),
        })
    }

    fn read_u8<R: Read>(reader: &mut R) -> Result<u8, DffError> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_material<R: Read + Seek>(reader: &mut R) -> Result<Material, DffError> {
        // Check if we can read the section header (12 bytes)
        let current_pos = reader.stream_position()?;
        let file_size = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(current_pos))?;

        if current_pos + 12 > file_size {
            return Err(DffError::InvalidFormat(
                format!("Material section header truncated - need 12 bytes, only {} available", file_size - current_pos)
            ));
        }

        let (section_type, section_size, _rw_version) = Self::read_section_header(reader)?;

        if section_type != RW_SECTION_MATERIAL {
            return Err(DffError::InvalidFormat(format!(
                "Expected Material section (0x{:04X}), got 0x{:04X} at position 0x{:X}. File may be corrupted.",
                RW_SECTION_MATERIAL, section_type, current_pos
            )));
        }

        // Check if the section size is reasonable and doesn't exceed file bounds
        if section_size == 0 {
            return Err(DffError::InvalidFormat("Material section has zero size".to_string()));
        }

        let section_end_pos = current_pos + 12 + section_size as u64;
        if section_end_pos > file_size {
            return Err(DffError::InvalidFormat(format!(
                "Material section size {} exceeds file bounds (would end at 0x{:X}, file size 0x{:X})",
                section_size, section_end_pos, file_size
            )));
        }

        let material_start_pos = reader.stream_position()?;
        let material_end_pos = material_start_pos + section_size as u64;

        // Check if we have enough data for the basic material struct
        let remaining_in_section = material_end_pos - material_start_pos;
        if remaining_in_section < 12 { // STRUCT header
            return Err(DffError::InvalidFormat(format!(
                "Material section too small for STRUCT header: {} bytes available", remaining_in_section
            )));
        }

        // Read struct section
        let (_struct_type, struct_size, _struct_version) = Self::read_section_header(reader)?;

        // Basic material data should be at least 28 bytes (4 u32 + 4 u8 + 3 f32)
        if struct_size < 28 {
            return Err(DffError::InvalidFormat(format!(
                "Material STRUCT section too small: {} bytes, expected at least 28", struct_size
            )));
        }

        let _unknown1 = Self::read_u32(reader)?;
        let color = Color {
            r: Self::read_u8(reader)?,
            g: Self::read_u8(reader)?,
            b: Self::read_u8(reader)?,
            a: Self::read_u8(reader)?,
        };
        let _unknown2 = Self::read_u32(reader)?;
        let is_textured = Self::read_u32(reader)?;
        let ambient = Self::read_f32(reader)?;
        let specular = Self::read_f32(reader)?;
        let diffuse = Self::read_f32(reader)?;

        let mut textures = Vec::new();
        let mut effects = Vec::new();
        let mut user_data = None;

        // Read texture if present
        if is_textured != 0 {
            textures.push(Self::read_texture(reader)?);
        }

        // Read extension section
        let current_pos = reader.stream_position()?;
        if current_pos < material_end_pos {
            let (ext_type, ext_size, _ext_version) = Self::read_section_header(reader)?;

            if ext_type == RW_SECTION_EXTENSION && ext_size > 0 {
                let ext_end_pos = reader.stream_position()? + ext_size as u64;

                while reader.stream_position()? < ext_end_pos {
                    let (plugin_type, plugin_size, _plugin_version) =
                        Self::read_section_header(reader)?;
                    let plugin_end_pos = reader.stream_position()? + plugin_size as u64;

                    match plugin_type {
                        RW_SECTION_MATERIAL_EFFECTS_PLG => {
                            Self::read_material_effects(reader, &mut effects)?;
                        }
                        RW_SECTION_SPECULAR_MATERIAL => {
                            let specular_mat = Self::read_specular_material(reader)?;
                            effects.push(MaterialEffect::Specular(specular_mat));
                        }
                        RW_SECTION_REFLECTION_MATERIAL => {
                            let reflection_mat = Self::read_reflection_material(reader)?;
                            effects.push(MaterialEffect::Reflection(reflection_mat));
                        }
                        RW_SECTION_UV_ANIMATION_PLG => {
                            let uv_frames = Self::read_uv_animation_frames(reader)?;
                            effects.push(MaterialEffect::UVAnimation(uv_frames));
                        }
                        RW_SECTION_USER_DATA_PLG => {
                            user_data = Some(Self::read_user_data(reader)?);
                        }
                        _ => {
                            // Skip unknown plugins
                            reader.seek(SeekFrom::Current(plugin_size as i64))?;
                        }
                    }

                    // Ensure we're at the end of this plugin
                    reader.seek(SeekFrom::Start(plugin_end_pos))?;
                }
            }
        }

        // Ensure we're at the end of the material section
        // This handles any unread data (like extension sections we didn't process)
        reader.seek(SeekFrom::Start(material_end_pos))?;

        Ok(Material {
            color,
            textures,
            surface_properties: SurfaceProperties {
                ambient,
                specular,
                diffuse,
            },
            effects,
            user_data,
        })
    }

    fn read_texture<R: Read + Seek>(reader: &mut R) -> Result<Texture, DffError> {
        let (_tex_type, tex_size, _tex_version) = Self::read_section_header(reader)?;

        // Calculate texture end position to ensure we consume all bytes
        let tex_start_pos = reader.stream_position()?;
        let tex_end_pos = tex_start_pos + tex_size as u64;

        // Skip texture struct
        let (_struct_type, struct_size, _struct_version) = Self::read_section_header(reader)?;
        reader.seek(SeekFrom::Current(struct_size as i64))?;

        // Read texture name
        let (_name_type, name_size, _name_version) = Self::read_section_header(reader)?;
        let texture_name = Self::read_string(reader, name_size as usize)?;

        // Read mask name
        let (_mask_type, mask_size, _mask_version) = Self::read_section_header(reader)?;
        let mask_name = Self::read_string(reader, mask_size as usize)?;

        // Skip any remaining data (like EXTENSION section inside texture)
        // by seeking to the end of the texture section
        reader.seek(SeekFrom::Start(tex_end_pos))?;

        Ok(Texture {
            name: texture_name,
            mask: mask_name,
        })
    }

    fn read_material_effects<R: Read + Seek>(
        reader: &mut R,
        effects: &mut Vec<MaterialEffect>,
    ) -> Result<(), DffError> {
        let effect_type = Self::read_u32(reader)?;

        match effect_type {
            1 => {
                // Bump map
                let intensity = Self::read_f32(reader)?;
                let has_bump_texture = Self::read_u32(reader)?;
                let bump_texture = if has_bump_texture != 0 {
                    Some(Self::read_texture(reader)?)
                } else {
                    None
                };
                let has_height_texture = Self::read_u32(reader)?;
                let height_texture = if has_height_texture != 0 {
                    Some(Self::read_texture(reader)?)
                } else {
                    None
                };

                effects.push(MaterialEffect::BumpMap(BumpMapEffect {
                    intensity,
                    bump_texture,
                    height_texture,
                }));
            }
            2 => {
                // Environment map
                let coefficient = Self::read_f32(reader)?;
                let use_fb_alpha = Self::read_u32(reader)? != 0;
                let has_env_texture = Self::read_u32(reader)?;
                let env_texture = if has_env_texture != 0 {
                    Some(Self::read_texture(reader)?)
                } else {
                    None
                };

                effects.push(MaterialEffect::EnvironmentMap(EnvironmentMapEffect {
                    coefficient,
                    use_fb_alpha,
                    env_texture,
                }));
            }
            4 => {
                // Dual texture
                let src_blend = Self::read_blend_mode(reader)?;
                let dst_blend = Self::read_blend_mode(reader)?;
                let has_texture = Self::read_u32(reader)?;
                let texture = if has_texture != 0 {
                    Some(Self::read_texture(reader)?)
                } else {
                    None
                };

                effects.push(MaterialEffect::DualTexture(DualTextureEffect {
                    src_blend,
                    dst_blend,
                    texture,
                }));
            }
            _ => {
                // Skip unknown effect types
            }
        }

        Ok(())
    }

    fn read_blend_mode<R: Read + Seek>(reader: &mut R) -> Result<BlendMode, DffError> {
        let blend_value = Self::read_u32(reader)?;
        match blend_value {
            0x00 => Ok(BlendMode::NoBlend),
            0x01 => Ok(BlendMode::Zero),
            0x02 => Ok(BlendMode::One),
            0x03 => Ok(BlendMode::SrcColor),
            0x04 => Ok(BlendMode::InvSrcColor),
            0x05 => Ok(BlendMode::SrcAlpha),
            0x06 => Ok(BlendMode::InvSrcAlpha),
            0x07 => Ok(BlendMode::DestAlpha),
            0x08 => Ok(BlendMode::InvDestAlpha),
            0x09 => Ok(BlendMode::DestColor),
            0x0A => Ok(BlendMode::InvDestColor),
            0x0B => Ok(BlendMode::SrcAlphaSat),
            _ => Ok(BlendMode::NoBlend), // Default fallback
        }
    }

    fn read_specular_material<R: Read + Seek>(
        reader: &mut R,
    ) -> Result<SpecularMaterial, DffError> {
        let level = Self::read_f32(reader)?;
        let has_texture = Self::read_u32(reader)?;
        let texture = if has_texture != 0 {
            Some(Self::read_string(reader, 32)?)
        } else {
            None
        };

        Ok(SpecularMaterial { level, texture })
    }

    fn read_reflection_material<R: Read + Seek>(
        reader: &mut R,
    ) -> Result<ReflectionMaterial, DffError> {
        let scale_x = Self::read_f32(reader)?;
        let scale_y = Self::read_f32(reader)?;
        let offset_x = Self::read_f32(reader)?;
        let offset_y = Self::read_f32(reader)?;
        let intensity = Self::read_f32(reader)?;

        Ok(ReflectionMaterial {
            scale_x,
            scale_y,
            offset_x,
            offset_y,
            intensity,
        })
    }

    fn read_uv_animation_frames<R: Read + Seek>(reader: &mut R) -> Result<Vec<UVFrame>, DffError> {
        let (_struct_type, _struct_size, _struct_version) = Self::read_section_header(reader)?;
        let frame_count = Self::read_u32(reader)?;

        let mut frames = Vec::new();
        for _ in 0..frame_count {
            let time = Self::read_f32(reader)?;
            let mut uv_values = [0.0f32; 8];
            for i in 0..8 {
                uv_values[i] = Self::read_f32(reader)?;
            }

            frames.push(UVFrame {
                time,
                uv: uv_values,
            });
        }

        Ok(frames)
    }

    fn read_user_data<R: Read + Seek>(reader: &mut R) -> Result<MaterialUserData, DffError> {
        let (_struct_type, _struct_size, _struct_version) = Self::read_section_header(reader)?;

        // Read user data name
        let (_name_type, name_size, _name_version) = Self::read_section_header(reader)?;
        let name = Self::read_string(reader, name_size as usize)?;

        // Read user data
        let (_data_type, data_size, _data_version) = Self::read_section_header(reader)?;
        let mut data = vec![0u8; data_size as usize];
        reader.read_exact(&mut data)?;

        Ok(MaterialUserData { name, data })
    }

    fn read_atomic_list<R: Read + Seek>(reader: &mut R) -> Result<Vec<Atomic>, DffError> {
        let (_section_type, _section_size, _rw_version) = Self::read_section_header(reader)?;
        let num_atomics = Self::read_u32(reader)?;

        let mut atomics = Vec::new();

        for _ in 0..num_atomics {
            let (_atomic_section_type, _atomic_section_size, _atomic_rw_version) =
                Self::read_section_header(reader)?;

            let frame = Self::read_u32(reader)?;
            let geometry = Self::read_u32(reader)?;
            let flags = Self::read_u32(reader)?;
            let _unknown = Self::read_u32(reader)?;

            atomics.push(Atomic {
                frame,
                geometry,
                flags,
            });
        }

        Ok(atomics)
    }

    fn read_uv_animations<R: Read + Seek>(reader: &mut R) -> Result<Vec<UVAnimation>, DffError> {
        // Save current position
        let current_pos = reader.stream_position()?;

        // Seek to beginning to look for UV Animation Dictionary
        reader.seek(SeekFrom::Start(0))?;

        let mut uv_animations = Vec::new();

        // Read through the file looking for UV Animation Dictionary sections
        while let Ok((section_type, section_size, _)) = Self::read_section_header(reader) {
            if section_type == RW_SECTION_UV_ANIMATION_DICTIONARY {
                // Found UV Animation Dictionary
                let dict_start = reader.stream_position()?;

                // Read struct section
                let (_struct_type, _struct_size, _) = Self::read_section_header(reader)?;
                let num_animations = Self::read_u32(reader)?;

                for _ in 0..num_animations {
                    let animation = Self::read_uv_animation(reader)?;
                    uv_animations.push(animation);
                }

                // Skip to end of dictionary section
                reader.seek(SeekFrom::Start(dict_start + section_size as u64))?;
            } else {
                // Skip this section
                reader.seek(SeekFrom::Current(section_size as i64))?;
            }

            // Check if we've reached the end or gone too far
            if reader.stream_position()? >= current_pos {
                break;
            }
        }

        // Restore original position
        reader.seek(SeekFrom::Start(current_pos))?;

        Ok(uv_animations)
    }

    fn read_uv_animation<R: Read + Seek>(reader: &mut R) -> Result<UVAnimation, DffError> {
        // Read Animation Anim section
        let (section_type, _section_size, _) = Self::read_section_header(reader)?;

        if section_type != RW_SECTION_ANIMATION_ANIM {
            return Err(DffError::InvalidFormat(
                "Expected Animation Anim section".to_string(),
            ));
        }

        // Skip version (4 bytes)
        Self::read_u32(reader)?;

        let type_id = Self::read_u32(reader)?;
        let num_frames = Self::read_u32(reader)?;
        let flags = Self::read_u32(reader)?;
        let duration = Self::read_f32(reader)?;

        // Skip 4 bytes
        Self::read_u32(reader)?;

        let name = Self::read_string(reader, 32)?;
        let name = name.trim_end_matches('\0').to_string();

        // Read node_to_uv mapping (8 floats)
        let mut node_to_uv = [0.0f32; 8];
        for i in 0..8 {
            node_to_uv[i] = Self::read_f32(reader)?;
        }

        // Read UV frames
        let mut frames = Vec::new();
        for _ in 0..num_frames {
            let time = Self::read_f32(reader)?;
            let mut uv_values = [0.0f32; 8];
            for i in 0..8 {
                uv_values[i] = Self::read_f32(reader)?;
            }
            frames.push(UVFrame {
                time,
                uv: uv_values,
            });
        }

        Ok(UVAnimation {
            type_id,
            flags,
            duration,
            name,
            node_to_uv,
            frames,
        })
    }

    fn read_native_geometry<R: Read + Seek>(
        reader: &mut R,
        _flags: u32,
    ) -> Result<PlatformNativeGeometry, DffError> {
        // Read the native data PLG section
        let (section_type, section_size, _rw_version) = Self::read_section_header(reader)?;

        if section_type != 0x0000050E {
            // Native Data PLG
            return Err(DffError::InvalidFormat(
                "Expected Native Data PLG section".to_string(),
            ));
        }

        let native_data_start = reader.stream_position()?;
        let native_data_end = native_data_start + section_size as u64;

        // Read platform identifier (first 4 bytes after header)
        let platform_id = Self::read_u32(reader)?;

        let platform = match platform_id {
            1 => PlatformType::PS2,
            2 => PlatformType::Xbox,
            5 => PlatformType::GameCube,
            6 => PlatformType::PSP,
            _ => PlatformType::PC, // Default to PC
        };

        // Read the rest of the native data
        let mut native_data = Vec::new();
        while reader.stream_position()? < native_data_end {
            let mut buffer = [0u8; 1024];
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            native_data.extend_from_slice(&buffer[..bytes_read]);
        }

        // For now, we store all native data as vertex_data
        // In a full implementation, this would be parsed according to platform-specific formats
        Ok(PlatformNativeGeometry {
            platform,
            vertex_data: native_data,
            index_data: Vec::new(), // TODO: Extract index data if present
            shader_data: None,      // TODO: Extract shader data if present
        })
    }

    fn extract_2dfx_effects(geometries: &[Geometry]) -> Option<dfx::Effects2DFX> {
        // Look for 2DFX effects in the last geometry's extensions
        if let Some(last_geometry) = geometries.last() {
            if let Some(extension_data) = last_geometry.extensions.get("2dfx") {
                match extension_data {
                    ExtensionData::Raw(data) => {
                        return dfx::Effects2DFX::load_from_data(data).ok();
                    }
                    _ => {}
                }
            }
        }
        None
    }
}
