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
    pub rotation: Vector3, // Euler angles (converted from quaternion)
    pub rotation_quat: Option<Quaternion>, // Original quaternion if available
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
        let content = std::fs::read_to_string(path).map_err(|e| IplError::Io(e))?;

        Self::parse_text_ipl(&content)
    }

    fn parse_text_ipl(content: &str) -> Result<Self, IplError> {
        let mut instances = Vec::new();
        let mut culls = Vec::new();
        let mut zones = Vec::new();
        let mut picks = Vec::new();
        let mut path_count = Vec::new();

        let mut current_section: Option<&str> = None;

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let line_lower = line.to_lowercase();

            // Check for section end
            if line_lower == "end" {
                current_section = None;
                continue;
            }

            // Check for section headers (case-insensitive)
            if line_lower == "inst" {
                current_section = Some("inst");
                continue;
            } else if line_lower == "cull" {
                current_section = Some("cull");
                continue;
            } else if line_lower == "zone" {
                current_section = Some("zone");
                continue;
            } else if line_lower == "pick" {
                current_section = Some("pick");
                continue;
            } else if line_lower == "path" {
                current_section = Some("path");
                continue;
            } else if line_lower == "cars" {
                current_section = Some("cars");
                continue;
            } else if line_lower == "enex" {
                current_section = Some("enex");
                continue;
            } else if line_lower == "grge" {
                current_section = Some("grge");
                continue;
            } else if line_lower == "occl" {
                current_section = Some("occl");
                continue;
            } else if line_lower == "auzo" {
                current_section = Some("auzo");
                continue;
            } else if line_lower == "jump" {
                current_section = Some("jump");
                continue;
            } else if line_lower == "tcyc" {
                current_section = Some("tcyc");
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
                // Other sections are parsed but not stored yet
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

        // SA format: id, model_name, interior, posX, posY, posZ, rotX, rotY, rotZ, rotW, lod
        // GTA III/VC format: id, model_name, posX, posY, posZ, scaleX, scaleY, scaleZ, rotX, rotY, rotZ, rotW

        if parts.len() < 11 {
            return Err(IplError::ParseError(format!(
                "Invalid instance line (need at least 11 fields): {}",
                line
            )));
        }

        let id = parts[0]
            .parse()
            .map_err(|_| IplError::ParseError("Invalid ID".to_string()))?;
        let model_name = parts[1].to_string();
        let interior = parts[2]
            .parse()
            .map_err(|_| IplError::ParseError("Invalid interior".to_string()))?;

        let position = Vector3 {
            x: parts[3]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid X position".to_string()))?,
            y: parts[4]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid Y position".to_string()))?,
            z: parts[5]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid Z position".to_string()))?,
        };

        // Parse quaternion rotation (SA format: rotX, rotY, rotZ, rotW)
        let qx: f32 = parts[6]
            .parse()
            .map_err(|_| IplError::ParseError("Invalid rotX".to_string()))?;
        let qy: f32 = parts[7]
            .parse()
            .map_err(|_| IplError::ParseError("Invalid rotY".to_string()))?;
        let qz: f32 = parts[8]
            .parse()
            .map_err(|_| IplError::ParseError("Invalid rotZ".to_string()))?;
        let qw: f32 = parts[9]
            .parse()
            .map_err(|_| IplError::ParseError("Invalid rotW".to_string()))?;

        let rotation_quat = Quaternion {
            x: qx,
            y: qy,
            z: qz,
            w: qw,
        };

        // Convert quaternion to Euler angles (degrees)
        let rotation = Self::quaternion_to_euler(&rotation_quat);

        let lod = parts[10].parse().unwrap_or(-1);

        Ok(IPLInstance {
            id,
            model_name,
            interior,
            position,
            rotation,
            rotation_quat: Some(rotation_quat),
            lod,
        })
    }

    /// Convert quaternion to Euler angles in degrees
    fn quaternion_to_euler(q: &Quaternion) -> Vector3 {
        // Roll (x-axis rotation)
        let sinr_cosp = 2.0 * (q.w * q.x + q.y * q.z);
        let cosr_cosp = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        // Pitch (y-axis rotation)
        let sinp = 2.0 * (q.w * q.y - q.z * q.x);
        let pitch = if sinp.abs() >= 1.0 {
            std::f32::consts::FRAC_PI_2.copysign(sinp)
        } else {
            sinp.asin()
        };

        // Yaw (z-axis rotation)
        let siny_cosp = 2.0 * (q.w * q.z + q.x * q.y);
        let cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        // Convert to degrees
        Vector3 {
            x: roll.to_degrees(),
            y: pitch.to_degrees(),
            z: yaw.to_degrees(),
        }
    }

    fn parse_cull_line(line: &str) -> Result<IPLCull, IplError> {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if parts.len() < 8 {
            return Err(IplError::ParseError(format!("Invalid cull line: {}", line)));
        }

        let center = Vector3 {
            x: parts[0]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid center X".to_string()))?,
            y: parts[1]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid center Y".to_string()))?,
            z: parts[2]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid center Z".to_string()))?,
        };

        let min = Vector3 {
            x: parts[3]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid min X".to_string()))?,
            y: parts[4]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid min Y".to_string()))?,
            z: parts[5]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid min Z".to_string()))?,
        };

        let max = Vector3 {
            x: parts[6]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid max X".to_string()))?,
            y: parts[7]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid max Y".to_string()))?,
            z: parts[8]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid max Z".to_string()))?,
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
        let zone_type = parts[1]
            .parse()
            .map_err(|_| IplError::ParseError("Invalid zone type".to_string()))?;

        let min = Vector3 {
            x: parts[2]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid min X".to_string()))?,
            y: parts[3]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid min Y".to_string()))?,
            z: parts[4]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid min Z".to_string()))?,
        };

        let max = Vector3 {
            x: parts[5]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid max X".to_string()))?,
            y: parts[6]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid max Y".to_string()))?,
            z: parts[7]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid max Z".to_string()))?,
        };

        let island = parts[8]
            .parse()
            .map_err(|_| IplError::ParseError("Invalid island".to_string()))?;
        let name_hash = parts[9]
            .parse()
            .map_err(|_| IplError::ParseError("Invalid name hash".to_string()))?;

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
            x: parts[0]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid X position".to_string()))?,
            y: parts[1]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid Y position".to_string()))?,
            z: parts[2]
                .parse()
                .map_err(|_| IplError::ParseError("Invalid Z position".to_string()))?,
        };

        let unknown = parts[3]
            .parse()
            .map_err(|_| IplError::ParseError("Invalid unknown value".to_string()))?;

        Ok(IPLPick { position, unknown })
    }

    fn parse_path_count_line(line: &str) -> Result<i32, IplError> {
        line.trim()
            .parse()
            .map_err(|_| IplError::ParseError("Invalid path count".to_string()))
    }

    pub fn save_to_path(&self, path: &str) -> Result<(), IplError> {
        let content = self.to_text_ipl();
        std::fs::write(path, content).map_err(|e| IplError::Io(e))
    }

    fn to_text_ipl(&self) -> String {
        let mut output = String::new();

        // Write instances (using quaternion format for SA compatibility)
        if !self.instances.is_empty() {
            output.push_str("inst\n");
            for instance in &self.instances {
                // Use original quaternion if available, otherwise convert Euler to quaternion
                let (qx, qy, qz, qw) = if let Some(ref q) = instance.rotation_quat {
                    (q.x, q.y, q.z, q.w)
                } else {
                    // Convert Euler to quaternion (simplified, assumes rotation order)
                    let (rx, ry, rz) = (
                        instance.rotation.x.to_radians(),
                        instance.rotation.y.to_radians(),
                        instance.rotation.z.to_radians(),
                    );
                    let cy = (rz * 0.5).cos();
                    let sy = (rz * 0.5).sin();
                    let cp = (ry * 0.5).cos();
                    let sp = (ry * 0.5).sin();
                    let cr = (rx * 0.5).cos();
                    let sr = (rx * 0.5).sin();

                    (
                        sr * cp * cy - cr * sp * sy,
                        cr * sp * cy + sr * cp * sy,
                        cr * cp * sy - sr * sp * cy,
                        cr * cp * cy + sr * sp * sy,
                    )
                };
                output.push_str(&format!(
                    "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}\n",
                    instance.id,
                    instance.model_name,
                    instance.interior,
                    instance.position.x,
                    instance.position.y,
                    instance.position.z,
                    qx,
                    qy,
                    qz,
                    qw,
                    instance.lod
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
                    pick.position.x, pick.position.y, pick.position.z, pick.unknown
                ));
            }
            output.push_str("end\n\n");
        }

        output
    }
}
