use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IplError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid IPL format: {0}")]
    InvalidFormat(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPLInstance {
    pub id: i32,
    pub model_name: String,
    pub interior: i32,
    pub position: Vector3,
    pub rotation: Vector3, // Euler angles
    pub lod: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPLCull {
    pub center: Vector3,
    pub min: Vector3,
    pub max: Vector3,
    pub unknown: i32,
    pub flags: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPLZone {
    pub name: String,
    pub zone_type: i32,
    pub min: Vector3,
    pub max: Vector3,
    pub island: i32,
    pub name_hash: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPLPick {
    pub position: Vector3,
    pub unknown: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPLFile {
    pub instances: Vec<IPLInstance>,
    pub culls: Vec<IPLCull>,
    pub zones: Vec<IPLZone>,
    pub picks: Vec<IPLPick>,
    pub path_count: Vec<i32>,
}

impl IPLFile {
    pub fn load_from_path(path: &str) -> Result<Self, IplError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| IplError::Io(e))?;

        Self::parse_text_ipl(&content)
    }

    fn parse_text_ipl(content: &str) -> Result<Self, IplError> {
        let mut instances = Vec::new();
        let mut culls = Vec::new();
        let mut zones = Vec::new();
        let mut picks = Vec::new();
        let mut path_count = Vec::new();

        let mut current_section = None;

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Check for section headers
            if line.starts_with("inst") && line.ends_with("end") {
                current_section = Some("inst");
                continue;
            } else if line.starts_with("cull") && line.ends_with("end") {
                current_section = Some("cull");
                continue;
            } else if line.starts_with("zone") && line.ends_with("end") {
                current_section = Some("zone");
                continue;
            } else if line.starts_with("pick") && line.ends_with("end") {
                current_section = Some("pick");
                continue;
            } else if line.starts_with("path") && line.ends_with("end") {
                current_section = Some("path");
                continue;
            }

            // Parse section data
            match current_section {
                Some("inst") => {
                    if let Ok(instance) = Self::parse_instance_line(line) {
                        instances.push(instance);
                    }
                }
                Some("cull") => {
                    if let Ok(cull) = Self::parse_cull_line(line) {
                        culls.push(cull);
                    }
                }
                Some("zone") => {
                    if let Ok(zone) = Self::parse_zone_line(line) {
                        zones.push(zone);
                    }
                }
                Some("pick") => {
                    if let Ok(pick) = Self::parse_pick_line(line) {
                        picks.push(pick);
                    }
                }
                Some("path") => {
                    if let Ok(count) = Self::parse_path_count_line(line) {
                        path_count.push(count);
                    }
                }
                _ => {}
            }
        }

        Ok(IPLFile {
            instances,
            culls,
            zones,
            picks,
            path_count,
        })
    }

    fn parse_instance_line(line: &str) -> Result<IPLInstance, IplError> {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if parts.len() < 11 {
            return Err(IplError::ParseError(format!("Invalid instance line: {}", line)));
        }

        let id = parts[0].parse().map_err(|_| IplError::ParseError("Invalid ID".to_string()))?;
        let model_name = parts[1].to_string();
        let interior = parts[2].parse().map_err(|_| IplError::ParseError("Invalid interior".to_string()))?;

        let position = Vector3 {
            x: parts[3].parse().map_err(|_| IplError::ParseError("Invalid X position".to_string()))?,
            y: parts[4].parse().map_err(|_| IplError::ParseError("Invalid Y position".to_string()))?,
            z: parts[5].parse().map_err(|_| IplError::ParseError("Invalid Z position".to_string()))?,
        };

        let rotation = Vector3 {
            x: parts[6].parse().map_err(|_| IplError::ParseError("Invalid X rotation".to_string()))?,
            y: parts[7].parse().map_err(|_| IplError::ParseError("Invalid Y rotation".to_string()))?,
            z: parts[8].parse().map_err(|_| IplError::ParseError("Invalid Z rotation".to_string()))?,
        };

        let lod = parts[9].parse().map_err(|_| IplError::ParseError("Invalid LOD".to_string()))?;

        Ok(IPLInstance {
            id,
            model_name,
            interior,
            position,
            rotation,
            lod,
        })
    }

    fn parse_cull_line(line: &str) -> Result<IPLCull, IplError> {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if parts.len() < 8 {
            return Err(IplError::ParseError(format!("Invalid cull line: {}", line)));
        }

        let center = Vector3 {
            x: parts[0].parse().map_err(|_| IplError::ParseError("Invalid center X".to_string()))?,
            y: parts[1].parse().map_err(|_| IplError::ParseError("Invalid center Y".to_string()))?,
            z: parts[2].parse().map_err(|_| IplError::ParseError("Invalid center Z".to_string()))?,
        };

        let min = Vector3 {
            x: parts[3].parse().map_err(|_| IplError::ParseError("Invalid min X".to_string()))?,
            y: parts[4].parse().map_err(|_| IplError::ParseError("Invalid min Y".to_string()))?,
            z: parts[5].parse().map_err(|_| IplError::ParseError("Invalid min Z".to_string()))?,
        };

        let max = Vector3 {
            x: parts[6].parse().map_err(|_| IplError::ParseError("Invalid max X".to_string()))?,
            y: parts[7].parse().map_err(|_| IplError::ParseError("Invalid max Y".to_string()))?,
            z: parts[8].parse().map_err(|_| IplError::ParseError("Invalid max Z".to_string()))?,
        };

        let unknown = parts.get(9).and_then(|s| s.parse().ok()).unwrap_or(0);
        let flags = parts.get(10).and_then(|s| s.parse().ok()).unwrap_or(0);
        let name = parts.get(11).unwrap_or(&"").to_string();

        Ok(IPLCull {
            center,
            min,
            max,
            unknown,
            flags,
            name,
        })
    }

    fn parse_zone_line(line: &str) -> Result<IPLZone, IplError> {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if parts.len() < 10 {
            return Err(IplError::ParseError(format!("Invalid zone line: {}", line)));
        }

        let name = parts[0].to_string();
        let zone_type = parts[1].parse().map_err(|_| IplError::ParseError("Invalid zone type".to_string()))?;

        let min = Vector3 {
            x: parts[2].parse().map_err(|_| IplError::ParseError("Invalid min X".to_string()))?,
            y: parts[3].parse().map_err(|_| IplError::ParseError("Invalid min Y".to_string()))?,
            z: parts[4].parse().map_err(|_| IplError::ParseError("Invalid min Z".to_string()))?,
        };

        let max = Vector3 {
            x: parts[5].parse().map_err(|_| IplError::ParseError("Invalid max X".to_string()))?,
            y: parts[6].parse().map_err(|_| IplError::ParseError("Invalid max Y".to_string()))?,
            z: parts[7].parse().map_err(|_| IplError::ParseError("Invalid max Z".to_string()))?,
        };

        let island = parts[8].parse().map_err(|_| IplError::ParseError("Invalid island".to_string()))?;
        let name_hash = parts[9].parse().map_err(|_| IplError::ParseError("Invalid name hash".to_string()))?;

        Ok(IPLZone {
            name,
            zone_type,
            min,
            max,
            island,
            name_hash,
        })
    }

    fn parse_pick_line(line: &str) -> Result<IPLPick, IplError> {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if parts.len() < 4 {
            return Err(IplError::ParseError(format!("Invalid pick line: {}", line)));
        }

        let position = Vector3 {
            x: parts[0].parse().map_err(|_| IplError::ParseError("Invalid X position".to_string()))?,
            y: parts[1].parse().map_err(|_| IplError::ParseError("Invalid Y position".to_string()))?,
            z: parts[2].parse().map_err(|_| IplError::ParseError("Invalid Z position".to_string()))?,
        };

        let unknown = parts[3].parse().map_err(|_| IplError::ParseError("Invalid unknown value".to_string()))?;

        Ok(IPLPick { position, unknown })
    }

    fn parse_path_count_line(line: &str) -> Result<i32, IplError> {
        line.trim().parse().map_err(|_| IplError::ParseError("Invalid path count".to_string()))
    }

    pub fn save_to_path(&self, path: &str) -> Result<(), IplError> {
        let content = self.to_text_ipl();
        std::fs::write(path, content).map_err(|e| IplError::Io(e))
    }

    fn to_text_ipl(&self) -> String {
        let mut output = String::new();

        // Write instances
        if !self.instances.is_empty() {
            output.push_str("inst\n");
            for instance in &self.instances {
                output.push_str(&format!(
                    "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}\n",
                    instance.id,
                    instance.model_name,
                    instance.interior,
                    instance.position.x,
                    instance.position.y,
                    instance.position.z,
                    instance.rotation.x,
                    instance.rotation.y,
                    instance.rotation.z,
                    instance.lod,
                    -1 // LOD index, default to -1
                ));
            }
            output.push_str("end\n\n");
        }

        // Write culls
        if !self.culls.is_empty() {
            output.push_str("cull\n");
            for cull in &self.culls {
                output.push_str(&format!(
                    "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}\n",
                    cull.center.x,
                    cull.center.y,
                    cull.center.z,
                    cull.min.x,
                    cull.min.y,
                    cull.min.z,
                    cull.max.x,
                    cull.max.y,
                    cull.max.z,
                    cull.unknown
                ));
            }
            output.push_str("end\n\n");
        }

        // Write zones
        if !self.zones.is_empty() {
            output.push_str("zone\n");
            for zone in &self.zones {
                output.push_str(&format!(
                    "{}, {}, {}, {}, {}, {}, {}, {}, {}\n",
                    zone.name,
                    zone.zone_type,
                    zone.min.x,
                    zone.min.y,
                    zone.min.z,
                    zone.max.x,
                    zone.max.y,
                    zone.max.z,
                    zone.island
                ));
            }
            output.push_str("end\n\n");
        }

        // Write picks
        if !self.picks.is_empty() {
            output.push_str("pick\n");
            for pick in &self.picks {
                output.push_str(&format!(
                    "{}, {}, {}, {}\n",
                    pick.position.x,
                    pick.position.y,
                    pick.position.z,
                    pick.unknown
                ));
            }
            output.push_str("end\n\n");
        }

        output
    }
}