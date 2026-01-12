use serde::{Deserialize, Serialize};
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ColError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid COL format: {0}")]
    InvalidFormat(String),
    #[error("Unsupported COL version: {0}")]
    UnsupportedVersion(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Box {
    pub min: Vector3,
    pub max: Vector3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub material: u8,
    pub light: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Triangle {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surface {
    pub material: u8,
    pub flag: u8,
    pub brightness: u8,
    pub light: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColModel {
    pub model_id: u32,
    pub model_name: String,
    pub center_of_mass: Vector3,
    pub bounding_box: Box,
    pub spheres: Vec<Sphere>,
    pub boxes: Vec<Box>,
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub suspension_lines: Vec<[Vector3; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColVersion {
    Col1, // GTA III/Vice City
    Col2, // GTA San Andreas
    Col3, // GTA SA Advanced
    Col4, // Extended format
}

impl ColVersion {
    pub fn from_fourcc(fourcc: &str) -> Result<Self, ColError> {
        match fourcc {
            "COLL" => Ok(ColVersion::Col1),
            "COL2" => Ok(ColVersion::Col2),
            "COL3" => Ok(ColVersion::Col3),
            "COL4" => Ok(ColVersion::Col4),
            _ => Err(ColError::UnsupportedVersion(fourcc.to_string())),
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            ColVersion::Col1 => "COL1 (GTA III/VC)",
            ColVersion::Col2 => "COL2 (GTA SA)",
            ColVersion::Col3 => "COL3 (GTA SA Advanced)",
            ColVersion::Col4 => "COL4 (Extended)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColFile {
    pub version: ColVersion,
    pub file_path: String,
    pub models: Vec<ColModel>,
}

impl ColFile {
    pub fn load_from_path(path: &str) -> Result<Self, ColError> {
        let mut file = std::fs::File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        Self::load_from_bytes(&data, path)
    }

    pub fn load_from_bytes(data: &[u8], file_path: &str) -> Result<Self, ColError> {
        if data.len() < 4 {
            return Err(ColError::InvalidFormat("File too small".to_string()));
        }

        // Read FourCC
        let fourcc = std::str::from_utf8(&data[0..4])
            .map_err(|_| ColError::InvalidFormat("Invalid FourCC".to_string()))?;

        let version = ColVersion::from_fourcc(fourcc)?;

        let mut models = Vec::new();

        match version {
            ColVersion::Col1 => {
                if data.len() < 8 {
                    return Err(ColError::InvalidFormat("COL1 file too small".to_string()));
                }
                let model_count = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
                let mut offset = 8;

                for _ in 0..model_count {
                    if offset + 108 > data.len() {
                        return Err(ColError::InvalidFormat(
                            "COL1 model data truncated".to_string(),
                        ));
                    }

                    let model = Self::parse_col1_model(&data[offset..])?;
                    models.push(model);
                    offset += 108;
                }
            }
            ColVersion::Col2 | ColVersion::Col3 | ColVersion::Col4 => {
                if data.len() < 8 {
                    return Err(ColError::InvalidFormat(
                        "COL2/3/4 file too small".to_string(),
                    ));
                }
                let model_count = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
                let mut offset = 8;

                for _ in 0..model_count {
                    if offset + 4 > data.len() {
                        return Err(ColError::InvalidFormat(
                            "Model size field missing".to_string(),
                        ));
                    }

                    let model_size = u32::from_le_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]) as usize;
                    offset += 4;

                    if offset + model_size > data.len() {
                        return Err(ColError::InvalidFormat("Model data truncated".to_string()));
                    }

                    let model = Self::parse_col2_model(&data[offset..offset + model_size])?;
                    models.push(model);
                    offset += model_size;
                }
            }
        }

        Ok(ColFile {
            version,
            file_path: file_path.to_string(),
            models,
        })
    }

    fn parse_col1_model(data: &[u8]) -> Result<ColModel, ColError> {
        if data.len() < 108 {
            return Err(ColError::InvalidFormat(
                "COL1 model data too small".to_string(),
            ));
        }

        // COL1 has a fixed structure
        let model_id = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let center_of_mass = Vector3 {
            x: f32::from_le_bytes([data[4], data[5], data[6], data[7]]),
            y: f32::from_le_bytes([data[8], data[9], data[10], data[11]]),
            z: f32::from_le_bytes([data[12], data[13], data[14], data[15]]),
        };

        // Skip some fields and read bounding box
        let min = Vector3 {
            x: f32::from_le_bytes([data[16], data[17], data[18], data[19]]),
            y: f32::from_le_bytes([data[20], data[21], data[22], data[23]]),
            z: f32::from_le_bytes([data[24], data[25], data[26], data[27]]),
        };
        let max = Vector3 {
            x: f32::from_le_bytes([data[28], data[29], data[30], data[31]]),
            y: f32::from_le_bytes([data[32], data[33], data[34], data[35]]),
            z: f32::from_le_bytes([data[36], data[37], data[38], data[39]]),
        };

        let sphere_count = u16::from_le_bytes([data[40], data[41]]) as usize;
        let box_count = u16::from_le_bytes([data[42], data[43]]) as usize;
        let vertex_count = u16::from_le_bytes([data[44], data[45]]) as usize;
        let face_count = u16::from_le_bytes([data[46], data[47]]) as usize;

        // Parse spheres, boxes, vertices, faces based on counts
        let mut spheres = Vec::new();
        let mut offset = 48;

        for _ in 0..sphere_count {
            if offset + 16 > data.len() {
                break;
            }
            let center = Vector3 {
                x: f32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]),
                y: f32::from_le_bytes([
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]),
                z: f32::from_le_bytes([
                    data[offset + 8],
                    data[offset + 9],
                    data[offset + 10],
                    data[offset + 11],
                ]),
            };
            let radius = f32::from_le_bytes([
                data[offset + 12],
                data[offset + 13],
                data[offset + 14],
                data[offset + 15],
            ]);
            spheres.push(Sphere { center, radius });
            offset += 16;
        }

        let mut boxes = Vec::new();
        for _ in 0..box_count {
            if offset + 24 > data.len() {
                break;
            }
            let min = Vector3 {
                x: f32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]),
                y: f32::from_le_bytes([
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]),
                z: f32::from_le_bytes([
                    data[offset + 8],
                    data[offset + 9],
                    data[offset + 10],
                    data[offset + 11],
                ]),
            };
            let max = Vector3 {
                x: f32::from_le_bytes([
                    data[offset + 12],
                    data[offset + 13],
                    data[offset + 14],
                    data[offset + 15],
                ]),
                y: f32::from_le_bytes([
                    data[offset + 16],
                    data[offset + 17],
                    data[offset + 18],
                    data[offset + 19],
                ]),
                z: f32::from_le_bytes([
                    data[offset + 20],
                    data[offset + 21],
                    data[offset + 22],
                    data[offset + 23],
                ]),
            };
            boxes.push(Box { min, max });
            offset += 24;
        }

        let mut vertices = Vec::new();
        for _ in 0..vertex_count {
            if offset + 12 > data.len() {
                break;
            }
            let vertex = Vertex {
                x: f32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]),
                y: f32::from_le_bytes([
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]),
                z: f32::from_le_bytes([
                    data[offset + 8],
                    data[offset + 9],
                    data[offset + 10],
                    data[offset + 11],
                ]),
            };
            vertices.push(vertex);
            offset += 12;
        }

        let mut faces = Vec::new();
        for _ in 0..face_count {
            if offset + 8 > data.len() {
                break;
            }
            let face = Face {
                a: u16::from_le_bytes([data[offset], data[offset + 1]]),
                b: u16::from_le_bytes([data[offset + 2], data[offset + 3]]),
                c: u16::from_le_bytes([data[offset + 4], data[offset + 5]]),
                material: data[offset + 6],
                light: data[offset + 7],
            };
            faces.push(face);
            offset += 8;
        }

        Ok(ColModel {
            model_id,
            model_name: format!("Model_{}", model_id),
            center_of_mass,
            bounding_box: Box { min, max },
            spheres,
            boxes,
            vertices,
            faces,
            suspension_lines: Vec::new(),
        })
    }

    fn parse_col2_model(data: &[u8]) -> Result<ColModel, ColError> {
        if data.len() < 52 {
            return Err(ColError::InvalidFormat(
                "COL2 model header too small".to_string(),
            ));
        }

        let mut offset = 0;
        let model_id = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;

        // Model name (22 bytes, null-terminated)
        let name_end = data[offset..offset + 22]
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(22);
        let model_name = String::from_utf8_lossy(&data[offset..offset + name_end]).to_string();
        offset += 22;

        // Skip unknown field (4 bytes)
        offset += 4;

        let center_of_mass = Vector3 {
            x: f32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]),
            y: f32::from_le_bytes([
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]),
            z: f32::from_le_bytes([
                data[offset + 8],
                data[offset + 9],
                data[offset + 10],
                data[offset + 11],
            ]),
        };
        offset += 12;

        // Bounding box
        let min = Vector3 {
            x: f32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]),
            y: f32::from_le_bytes([
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]),
            z: f32::from_le_bytes([
                data[offset + 8],
                data[offset + 9],
                data[offset + 10],
                data[offset + 11],
            ]),
        };
        offset += 12;

        let max = Vector3 {
            x: f32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]),
            y: f32::from_le_bytes([
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]),
            z: f32::from_le_bytes([
                data[offset + 8],
                data[offset + 9],
                data[offset + 10],
                data[offset + 11],
            ]),
        };
        offset += 12;

        // Counts
        let sphere_count = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;
        let box_count = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;
        let face_count = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;
        let line_count = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;
        let vertex_count = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;
        let _triangle_count = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;
        let _shadow_count = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;

        // Parse spheres
        let mut spheres = Vec::new();
        for _ in 0..sphere_count {
            if offset + 16 > data.len() {
                break;
            }
            let center = Vector3 {
                x: f32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]),
                y: f32::from_le_bytes([
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]),
                z: f32::from_le_bytes([
                    data[offset + 8],
                    data[offset + 9],
                    data[offset + 10],
                    data[offset + 11],
                ]),
            };
            let radius = f32::from_le_bytes([
                data[offset + 12],
                data[offset + 13],
                data[offset + 14],
                data[offset + 15],
            ]);
            spheres.push(Sphere { center, radius });
            offset += 16;
        }

        // Parse boxes
        let mut boxes = Vec::new();
        for _ in 0..box_count {
            if offset + 24 > data.len() {
                break;
            }
            let min = Vector3 {
                x: f32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]),
                y: f32::from_le_bytes([
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]),
                z: f32::from_le_bytes([
                    data[offset + 8],
                    data[offset + 9],
                    data[offset + 10],
                    data[offset + 11],
                ]),
            };
            let max = Vector3 {
                x: f32::from_le_bytes([
                    data[offset + 12],
                    data[offset + 13],
                    data[offset + 14],
                    data[offset + 15],
                ]),
                y: f32::from_le_bytes([
                    data[offset + 16],
                    data[offset + 17],
                    data[offset + 18],
                    data[offset + 19],
                ]),
                z: f32::from_le_bytes([
                    data[offset + 20],
                    data[offset + 21],
                    data[offset + 22],
                    data[offset + 23],
                ]),
            };
            boxes.push(Box { min, max });
            offset += 24;
        }

        // Parse vertices
        let mut vertices = Vec::new();
        for _ in 0..vertex_count {
            if offset + 12 > data.len() {
                break;
            }
            let vertex = Vertex {
                x: f32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]),
                y: f32::from_le_bytes([
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]),
                z: f32::from_le_bytes([
                    data[offset + 8],
                    data[offset + 9],
                    data[offset + 10],
                    data[offset + 11],
                ]),
            };
            vertices.push(vertex);
            offset += 12;
        }

        // Parse faces
        let mut faces = Vec::new();
        for _ in 0..face_count {
            if offset + 8 > data.len() {
                break;
            }
            let face = Face {
                a: u16::from_le_bytes([data[offset], data[offset + 1]]),
                b: u16::from_le_bytes([data[offset + 2], data[offset + 3]]),
                c: u16::from_le_bytes([data[offset + 4], data[offset + 5]]),
                material: data[offset + 6],
                light: data[offset + 7],
            };
            faces.push(face);
            offset += 8;
        }

        // Parse suspension lines
        let mut suspension_lines = Vec::new();
        for _ in 0..line_count {
            if offset + 24 > data.len() {
                break;
            }
            let start = Vector3 {
                x: f32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]),
                y: f32::from_le_bytes([
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]),
                z: f32::from_le_bytes([
                    data[offset + 8],
                    data[offset + 9],
                    data[offset + 10],
                    data[offset + 11],
                ]),
            };
            let end = Vector3 {
                x: f32::from_le_bytes([
                    data[offset + 12],
                    data[offset + 13],
                    data[offset + 14],
                    data[offset + 15],
                ]),
                y: f32::from_le_bytes([
                    data[offset + 16],
                    data[offset + 17],
                    data[offset + 18],
                    data[offset + 19],
                ]),
                z: f32::from_le_bytes([
                    data[offset + 20],
                    data[offset + 21],
                    data[offset + 22],
                    data[offset + 23],
                ]),
            };
            suspension_lines.push([start, end]);
            offset += 24;
        }

        Ok(ColModel {
            model_id,
            model_name,
            center_of_mass,
            bounding_box: Box { min, max },
            spheres,
            boxes,
            vertices,
            faces,
            suspension_lines,
        })
    }

    pub fn get_statistics(&self) -> serde_json::Value {
        let total_spheres = self.models.iter().map(|m| m.spheres.len()).sum::<usize>();
        let total_boxes = self.models.iter().map(|m| m.boxes.len()).sum::<usize>();
        let total_vertices = self.models.iter().map(|m| m.vertices.len()).sum::<usize>();
        let total_faces = self.models.iter().map(|m| m.faces.len()).sum::<usize>();
        let total_suspension_lines = self
            .models
            .iter()
            .map(|m| m.suspension_lines.len())
            .sum::<usize>();

        serde_json::json!({
            "version": self.version.to_string(),
            "model_count": self.models.len(),
            "total_spheres": total_spheres,
            "total_boxes": total_boxes,
            "total_vertices": total_vertices,
            "total_faces": total_faces,
            "total_suspension_lines": total_suspension_lines,
            "file_path": self.file_path
        })
    }
}
