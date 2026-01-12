use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use thiserror::Error;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    pub flags: u32,
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub uv_layers: Vec<Vec<UVLayer>>,
    pub triangles: Vec<Triangle>,
    pub materials: Vec<Material>,
    pub extensions: HashMap<String, ExtensionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionData {
    Skin(SkinData),
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
pub struct DffModel {
    pub rw_version: u32,
    pub frames: Vec<Frame>,
    pub geometries: Vec<Geometry>,
    pub atomics: Vec<Atomic>,
    pub file_path: String,
}

impl DffModel {
    pub fn load_from_path(path: &str) -> Result<Self, DffError> {
        let mut file = std::fs::File::open(path)?;
        Self::load_from_reader(&mut file, path)
    }

    pub fn load_from_reader<R: Read + Seek>(reader: &mut R, path: &str) -> Result<Self, DffError> {
        // Read the main CLUMP section
        let (section_type, _section_size, rw_version) = Self::read_section_header(reader)?;

        if section_type != 0x0010 { // CLUMP
            return Err(DffError::InvalidFormat("Expected CLUMP section".to_string()));
        }

        // Read CLUMP data
        let _num_atomics = Self::read_u32(reader)?;
        let _num_lights = Self::read_u32(reader)?;
        let _num_cameras = Self::read_u32(reader)?;

        // Skip to frame list
        Self::skip_to_section(reader, 0x000E)?; // FRAME_LIST

        let frames = Self::read_frame_list(reader)?;

        // Skip to geometry list
        Self::skip_to_section(reader, 0x001A)?; // GEOMETRY_LIST

        let geometries = Self::read_geometry_list(reader)?;

        // Skip to atomic list
        Self::skip_to_section(reader, 0x0014)?; // ATOMIC_LIST

        let atomics = Self::read_atomic_list(reader)?;

        Ok(DffModel {
            rw_version,
            frames,
            geometries,
            atomics,
            file_path: path.to_string(),
        })
    }

    fn read_section_header<R: Read>(reader: &mut R) -> Result<(u32, u32, u32), DffError> {
        let section_type = Self::read_u32(reader)?;
        let section_size = Self::read_u32(reader)?;
        let rw_version = Self::read_u32(reader)?;
        Ok((section_type, section_size, rw_version))
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
        String::from_utf8(buf).map_err(|e| DffError::InvalidFormat(format!("Invalid UTF-8 string: {}", e)))
    }

    fn skip_to_section<R: Read + Seek>(reader: &mut R, target_type: u32) -> Result<(), DffError> {
        loop {
            let (section_type, section_size, _) = Self::read_section_header(reader)?;
            if section_type == target_type {
                return Ok(());
            }
            // Skip the section data
            reader.seek(SeekFrom::Current(section_size as i64))?;
        }
    }

    fn read_frame_list<R: Read + Seek>(reader: &mut R) -> Result<Vec<Frame>, DffError> {
        let (_section_type, _section_size, _rw_version) = Self::read_section_header(reader)?;
        let num_frames = Self::read_u32(reader)?;

        let mut frames = Vec::new();

        for _ in 0..num_frames {
            let (_frame_section_type, _frame_section_size, _frame_rw_version) = Self::read_section_header(reader)?;

            // Read frame data
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

            let position = Vector3 {
                x: Self::read_f32(reader)?,
                y: Self::read_f32(reader)?,
                z: Self::read_f32(reader)?,
            };

            let parent = Self::read_u32(reader)? as i32;
            let _unknown = Self::read_u32(reader)?;

            // Try to read frame name (variable length)
            let mut name = String::new();
            loop {
                let byte = {
                    let mut buf = [0u8; 1];
                    reader.read_exact(&mut buf)?;
                    buf[0]
                };
                if byte == 0 {
                    break;
                }
                name.push(byte as char);
            }

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
        let (_section_type, _section_size, _rw_version) = Self::read_section_header(reader)?;
        let num_geometries = Self::read_u32(reader)?;

        let mut geometries = Vec::new();

        for _ in 0..num_geometries {
            geometries.push(Self::read_geometry(reader)?);
        }

        Ok(geometries)
    }

    fn read_geometry<R: Read + Seek>(reader: &mut R) -> Result<Geometry, DffError> {
        let (_section_type, _section_size, _rw_version) = Self::read_section_header(reader)?;

        let flags = Self::read_u32(reader)?;
        let num_triangles = Self::read_u32(reader)?;
        let num_vertices = Self::read_u32(reader)?;
        let _num_morph_targets = Self::read_u32(reader)?;

        // Read vertices
        let mut vertices = Vec::new();
        for _ in 0..num_vertices {
            vertices.push(Vector3 {
                x: Self::read_f32(reader)?,
                y: Self::read_f32(reader)?,
                z: Self::read_f32(reader)?,
            });
        }

        // Read normals (if present)
        let mut normals = Vec::new();
        if flags & 0x10000000 != 0 { // Has normals
            for _ in 0..num_vertices {
                normals.push(Vector3 {
                    x: Self::read_f32(reader)?,
                    y: Self::read_f32(reader)?,
                    z: Self::read_f32(reader)?,
                });
            }
        }

        // Skip prelighting colors if present
        if flags & 0x20000000 != 0 {
            for _ in 0..num_vertices {
                let _r = Self::read_u8(reader)?;
                let _g = Self::read_u8(reader)?;
                let _b = Self::read_u8(reader)?;
                let _a = Self::read_u8(reader)?;
            }
        }

        // Read UV coordinates
        let mut uv_layers = Vec::new();
        let num_uv_layers = ((flags >> 16) & 0xFF) as usize;
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

        // Read triangles
        let mut triangles = Vec::new();
        for _ in 0..num_triangles {
            let a = Self::read_u16(reader)?;
            let b = Self::read_u16(reader)?;
            let c = Self::read_u16(reader)?;
            let _material_index = Self::read_u16(reader)?;
            triangles.push(Triangle { a, b, c });
        }

        // Read materials
        let num_materials = Self::read_u32(reader)?;
        let mut materials = Vec::new();
        for _ in 0..num_materials {
            materials.push(Self::read_material(reader)?);
        }

        Ok(Geometry {
            flags,
            vertices,
            normals,
            uv_layers,
            triangles,
            materials,
            extensions: HashMap::new(),
        })
    }

    fn read_u8<R: Read>(reader: &mut R) -> Result<u8, DffError> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_material<R: Read + Seek>(reader: &mut R) -> Result<Material, DffError> {
        let (_section_type, _section_size, _rw_version) = Self::read_section_header(reader)?;

        let _unknown1 = Self::read_u32(reader)?;
        let color = Color {
            r: Self::read_u8(reader)?,
            g: Self::read_u8(reader)?,
            b: Self::read_u8(reader)?,
            a: Self::read_u8(reader)?,
        };
        let _unknown2 = Self::read_u32(reader)?;
        let _is_textured = Self::read_u32(reader)?;
        let ambient = Self::read_f32(reader)?;
        let specular = Self::read_f32(reader)?;
        let diffuse = Self::read_f32(reader)?;

        let mut textures = Vec::new();
        if _is_textured != 0 {
            // Read texture section
            let (_tex_section_type, _tex_section_size, _tex_rw_version) = Self::read_section_header(reader)?;
            let texture_name = Self::read_string(reader, 32)?;
            let mask_name = Self::read_string(reader, 32)?;
            textures.push(Texture {
                name: texture_name,
                mask: mask_name,
            });
        }

        Ok(Material {
            color,
            textures,
            surface_properties: SurfaceProperties {
                ambient,
                specular,
                diffuse,
            },
        })
    }

    fn read_atomic_list<R: Read + Seek>(reader: &mut R) -> Result<Vec<Atomic>, DffError> {
        let (_section_type, _section_size, _rw_version) = Self::read_section_header(reader)?;
        let num_atomics = Self::read_u32(reader)?;

        let mut atomics = Vec::new();

        for _ in 0..num_atomics {
            let (_atomic_section_type, _atomic_section_size, _atomic_rw_version) = Self::read_section_header(reader)?;

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
}