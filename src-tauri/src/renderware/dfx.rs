use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DfxError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid DFX format: {0}")]
    InvalidFormat(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// 2DFX Effects System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect2DFX {
    Light(LightEffect),
    Particle(ParticleEffect),
    PedAttractor(PedAttractorEffect),
    SunGlare(SunGlareEffect),
    EnterExit(EnterExitEffect),
    RoadSign(RoadSignEffect),
    TriggerPoint(TriggerPointEffect),
    CoverPoint(CoverPointEffect),
    Escalator(EscalatorEffect),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightEffect {
    pub position: Vector3,
    pub color: Color,
    pub corona_far_clip: f32,
    pub pointlight_range: f32,
    pub corona_size: f32,
    pub shadow_size: f32,
    pub corona_show_mode: u8,
    pub corona_enable_reflection: u8,
    pub corona_flare_type: u8,
    pub shadow_color_multiplier: u8,
    pub flags1: u8,
    pub corona_tex_name: String,
    pub shadow_tex_name: String,
    pub shadow_z_distance: u8,
    pub flags2: u8,
    pub look_direction: Option<Vector3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleEffect {
    pub position: Vector3,
    pub particle_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedAttractorEffect {
    pub position: Vector3,
    pub attractor_type: u8,
    pub queue_direction_x: f32,
    pub queue_direction_y: f32,
    pub external_script: String,
    pub unknown1: u16,
    pub unknown2: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunGlareEffect {
    pub position: Vector3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterExitEffect {
    pub position: Vector3,
    pub enter_angle: f32,
    pub approach_angle: f32,
    pub exit_angle: f32,
    pub interior_id: u16,
    pub flags: u8,
    pub name: String,
    pub time_on: u8,
    pub time_off: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadSignEffect {
    pub position: Vector3,
    pub size_x: f32,
    pub size_y: f32,
    pub rotation: f32,
    pub flags: u16,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerPointEffect {
    pub position: Vector3,
    pub point_id: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverPointEffect {
    pub position: Vector3,
    pub direction_x: f32,
    pub direction_y: f32,
    pub direction_z: f32,
    pub type_: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalatorEffect {
    pub bottom: Vector3,
    pub top: Vector3,
    pub end: Vector3,
    pub direction: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effects2DFX {
    pub effects: Vec<Effect2DFX>,
}

impl Effects2DFX {
    pub fn load_from_data(data: &[u8]) -> Result<Self, DfxError> {
        if data.len() < 4 {
            return Err(DfxError::InvalidFormat("Data too short for 2DFX".to_string()));
        }

        let entries_count = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
        let mut effects = Vec::new();
        let mut pos = 4;

        for _ in 0..entries_count {
            if pos + 20 > data.len() {
                break;
            }

            let position = Vector3 {
                x: f32::from_le_bytes(data[pos..pos+4].try_into().unwrap()),
                y: f32::from_le_bytes(data[pos+4..pos+8].try_into().unwrap()),
                z: f32::from_le_bytes(data[pos+8..pos+12].try_into().unwrap()),
            };

            let effect_type = u32::from_le_bytes(data[pos+12..pos+16].try_into().unwrap());
            let size = u32::from_le_bytes(data[pos+16..pos+20].try_into().unwrap()) as usize;

            pos += 20;

            if pos + size > data.len() {
                break;
            }

            let effect_data = &data[pos..pos + size];

            match effect_type {
                0 => { // Light
                    if effect_data.len() >= 76 {
                        let color = Color {
                            r: effect_data[0],
                            g: effect_data[1],
                            b: effect_data[2],
                            a: effect_data[3],
                        };

                        let corona_far_clip = f32::from_le_bytes(effect_data[4..8].try_into().unwrap());
                        let pointlight_range = f32::from_le_bytes(effect_data[8..12].try_into().unwrap());
                        let corona_size = f32::from_le_bytes(effect_data[12..16].try_into().unwrap());
                        let shadow_size = f32::from_le_bytes(effect_data[16..20].try_into().unwrap());
                        let corona_show_mode = effect_data[20];
                        let corona_enable_reflection = effect_data[21];
                        let corona_flare_type = effect_data[22];
                        let shadow_color_multiplier = effect_data[23];
                        let flags1 = effect_data[24];

                        let corona_tex_name = Self::read_string(&effect_data[25..49]);
                        let shadow_tex_name = Self::read_string(&effect_data[49..73]);
                        let shadow_z_distance = effect_data[73];
                        let flags2 = effect_data[74];

                        let look_direction = if effect_data.len() >= 77 {
                            Some(Vector3 {
                                x: effect_data[75] as f32,
                                y: effect_data[76] as f32,
                                z: effect_data[77] as f32,
                            })
                        } else {
                            None
                        };

                        effects.push(Effect2DFX::Light(LightEffect {
                            position,
                            color,
                            corona_far_clip,
                            pointlight_range,
                            corona_size,
                            shadow_size,
                            corona_show_mode,
                            corona_enable_reflection,
                            corona_flare_type,
                            shadow_color_multiplier,
                            flags1,
                            corona_tex_name,
                            shadow_tex_name,
                            shadow_z_distance,
                            flags2,
                            look_direction,
                        }));
                    }
                }
                1 => { // Particle
                    let particle_type = String::from_utf8_lossy(effect_data).to_string();
                    effects.push(Effect2DFX::Particle(ParticleEffect {
                        position,
                        particle_type,
                    }));
                }
                3 => { // Ped Attractor
                    if effect_data.len() >= 28 {
                        let attractor_type = effect_data[0];
                        let queue_direction_x = f32::from_le_bytes(effect_data[1..5].try_into().unwrap());
                        let queue_direction_y = f32::from_le_bytes(effect_data[5..9].try_into().unwrap());
                        let external_script = Self::read_string(&effect_data[9..25]);
                        let unknown1 = u16::from_le_bytes(effect_data[25..27].try_into().unwrap());
                        let unknown2 = u16::from_le_bytes(effect_data[27..29].try_into().unwrap());

                        effects.push(Effect2DFX::PedAttractor(PedAttractorEffect {
                            position,
                            attractor_type,
                            queue_direction_x,
                            queue_direction_y,
                            external_script,
                            unknown1,
                            unknown2,
                        }));
                    }
                }
                4 => { // Sun Glare
                    effects.push(Effect2DFX::SunGlare(SunGlareEffect {
                        position,
                    }));
                }
                6 => { // Enter/Exit
                    if effect_data.len() >= 44 {
                        let enter_angle = f32::from_le_bytes(effect_data[0..4].try_into().unwrap());
                        let approach_angle = f32::from_le_bytes(effect_data[4..8].try_into().unwrap());
                        let exit_angle = f32::from_le_bytes(effect_data[8..12].try_into().unwrap());
                        let interior_id = u16::from_le_bytes(effect_data[12..14].try_into().unwrap());
                        let flags = effect_data[14];
                        let name = Self::read_string(&effect_data[15..39]);
                        let time_on = effect_data[39];
                        let time_off = effect_data[40];

                        effects.push(Effect2DFX::EnterExit(EnterExitEffect {
                            position,
                            enter_angle,
                            approach_angle,
                            exit_angle,
                            interior_id,
                            flags,
                            name,
                            time_on,
                            time_off,
                        }));
                    }
                }
                7 => { // Road Sign
                    if effect_data.len() >= 26 {
                        let size_x = f32::from_le_bytes(effect_data[0..4].try_into().unwrap());
                        let size_y = f32::from_le_bytes(effect_data[4..8].try_into().unwrap());
                        let rotation = f32::from_le_bytes(effect_data[8..12].try_into().unwrap());
                        let flags = u16::from_le_bytes(effect_data[12..14].try_into().unwrap());
                        let text = Self::read_string(&effect_data[14..]);

                        effects.push(Effect2DFX::RoadSign(RoadSignEffect {
                            position,
                            size_x,
                            size_y,
                            rotation,
                            flags,
                            text,
                        }));
                    }
                }
                8 => { // Trigger Point
                    if effect_data.len() >= 2 {
                        let point_id = u16::from_le_bytes(effect_data[0..2].try_into().unwrap());
                        effects.push(Effect2DFX::TriggerPoint(TriggerPointEffect {
                            position,
                            point_id,
                        }));
                    }
                }
                9 => { // Cover Point
                    if effect_data.len() >= 13 {
                        let direction_x = f32::from_le_bytes(effect_data[0..4].try_into().unwrap());
                        let direction_y = f32::from_le_bytes(effect_data[4..8].try_into().unwrap());
                        let direction_z = f32::from_le_bytes(effect_data[8..12].try_into().unwrap());
                        let type_ = effect_data[12];

                        effects.push(Effect2DFX::CoverPoint(CoverPointEffect {
                            position,
                            direction_x,
                            direction_y,
                            direction_z,
                            type_,
                        }));
                    }
                }
                10 => { // Escalator
                    if effect_data.len() >= 37 {
                        let bottom = Vector3 {
                            x: f32::from_le_bytes(effect_data[0..4].try_into().unwrap()),
                            y: f32::from_le_bytes(effect_data[4..8].try_into().unwrap()),
                            z: f32::from_le_bytes(effect_data[8..12].try_into().unwrap()),
                        };
                        let top = Vector3 {
                            x: f32::from_le_bytes(effect_data[12..16].try_into().unwrap()),
                            y: f32::from_le_bytes(effect_data[16..20].try_into().unwrap()),
                            z: f32::from_le_bytes(effect_data[20..24].try_into().unwrap()),
                        };
                        let end = Vector3 {
                            x: f32::from_le_bytes(effect_data[24..28].try_into().unwrap()),
                            y: f32::from_le_bytes(effect_data[28..32].try_into().unwrap()),
                            z: f32::from_le_bytes(effect_data[32..36].try_into().unwrap()),
                        };
                        let direction = effect_data[36];

                        effects.push(Effect2DFX::Escalator(EscalatorEffect {
                            bottom,
                            top,
                            end,
                            direction,
                        }));
                    }
                }
                _ => {
                    // Unknown effect type, skip
                }
            }

            pos += size;
        }

        Ok(Effects2DFX { effects })
    }

    fn read_string(data: &[u8]) -> String {
        // Find null terminator
        let end = data.iter().position(|&x| x == 0).unwrap_or(data.len());
        String::from_utf8_lossy(&data[0..end]).to_string()
    }
}