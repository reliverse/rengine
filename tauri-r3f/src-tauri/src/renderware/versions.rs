use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RenderWareVersion {
    pub version_hex: String,
    pub version_string: String,
    pub version_display_string: String,
    pub game_info: Option<GameInfo>,
    pub all_games_info: Vec<GameInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub name: String,
    pub display_name: String,
    pub icon_name: Option<String>,
    pub platform: String,
    pub data_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub platform: String,
    pub rw_version: Option<String>,
    pub rw_version_min: Option<String>,
    pub rw_version_max: Option<String>,
    pub data_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSet {
    pub name: String,
    pub display_name: String,
    pub icon_name: Option<String>,
    pub platforms: Vec<PlatformInfo>,
}

pub struct RenderWareVersionManager {
    versionsets_data: Vec<GameSet>,
    platform_info: HashMap<String, Vec<String>>,
}

impl RenderWareVersionManager {
    pub fn new() -> Self {
        Self {
            versionsets_data: Vec::new(),
            platform_info: HashMap::new(),
        }
    }

    pub fn load_from_json(&mut self, json_path: &str) -> Result<(), String> {
        let content = fs::read_to_string(json_path)
            .map_err(|e| format!("Failed to read versionsets file: {}", e))?;

        let data: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        // Parse version sets
        if let Some(sets) = data.get("version_sets").and_then(|v| v.as_array()) {
            for set in sets {
                if let Some(game_set) = Self::parse_game_set(set) {
                    self.versionsets_data.push(game_set);
                }
            }
        }

        // Parse platform info
        if let Some(platforms) = data.get("platform_info").and_then(|v| v.as_object()) {
            for (platform, types) in platforms {
                if let Some(types_array) = types.as_array() {
                    let type_strings: Vec<String> = types_array
                        .iter()
                        .filter_map(|t| t.as_str().map(|s| s.to_string()))
                        .collect();
                    self.platform_info.insert(platform.clone(), type_strings);
                }
            }
        }

        Ok(())
    }

    fn parse_game_set(value: &serde_json::Value) -> Option<GameSet> {
        let name = value.get("name")?.as_str()?.to_string();
        let display_name = value
            .get("display_name")
            .and_then(|v| v.as_str())
            .unwrap_or(&name)
            .to_string();
        let icon_name = value
            .get("icon_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let platforms = value
            .get("platforms")?
            .as_array()?
            .iter()
            .filter_map(|p| Self::parse_platform_info(p))
            .collect();

        Some(GameSet {
            name,
            display_name,
            icon_name,
            platforms,
        })
    }

    fn parse_platform_info(value: &serde_json::Value) -> Option<PlatformInfo> {
        let platform = value.get("platform")?.as_str()?.to_string();
        let rw_version = value
            .get("rw_version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let rw_version_min = value
            .get("rw_version_min")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let rw_version_max = value
            .get("rw_version_max")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let data_types = value
            .get("data_types")
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|t| t.as_str().map(|s| s.to_string()))
            .collect();

        Some(PlatformInfo {
            platform,
            rw_version,
            rw_version_min,
            rw_version_max,
            data_types,
        })
    }

    pub fn get_rw_version(&self, library_id: Option<i32>) -> i32 {
        library_id.unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn get_library_id(&self, version: i32, build: i32) -> i32 {
        ((version & 0x3FF) << 14) | (build & 0x3FFF)
    }

    pub fn version_string_to_hex(&self, version_str: &str) -> i32 {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() >= 3 {
            if let (Ok(major), Ok(minor), Ok(patch1)) = (
                parts[0].parse::<i32>(),
                parts[1].parse::<i32>(),
                parts[2].parse::<i32>(),
            ) {
                let patch2 = parts
                    .get(3)
                    .and_then(|p| p.parse::<i32>().ok())
                    .unwrap_or(0);
                return (major << 16) | (minor << 12) | (patch1 << 8) | patch2;
            }
        }
        0
    }

    pub fn hex_to_version_string(&self, version_hex: i32) -> String {
        let major = (version_hex >> 16) & 0xFF;
        let minor = (version_hex >> 12) & 0xF;
        let patch1 = (version_hex >> 8) & 0xF;
        let patch2 = version_hex & 0xFF;

        if patch2 > 0 {
            format!("{}.{}.{}.{}", major, minor, patch1, patch2)
        } else {
            format!("{}.{}.{}", major, minor, patch1)
        }
    }

    #[allow(dead_code)]
    pub fn get_version_info(&self, version_value: i32) -> RenderWareVersion {
        let version_str = self.hex_to_version_string(version_value);
        let game_info = self.find_game_by_version(version_value);
        let all_games_info = self.find_all_games_by_version(version_value);

        let version_display_string = self.get_version_display_string(version_value, true);

        RenderWareVersion {
            version_hex: format!("0x{:X}", version_value),
            version_string: version_str,
            version_display_string,
            game_info,
            all_games_info,
        }
    }

    pub fn is_valid_rw_version(&self, version_value: i32) -> bool {
        // Check typical RW version ranges
        (0x30000..=0x3FFFF).contains(&version_value)
            || [
                0x0800FFFF, 0x1003FFFF, 0x1005FFFF, 0x1401FFFF, 0x1400FFFF, 0x1803FFFF, 0x1C020037,
            ]
            .contains(&version_value)
    }

    #[allow(dead_code)]
    pub fn find_game_by_version(&self, version_value: i32) -> Option<GameInfo> {
        self.find_all_games_by_version(version_value)
            .into_iter()
            .next()
    }

    pub fn find_all_games_by_version(&self, version_value: i32) -> Vec<GameInfo> {
        let version_str = self.hex_to_version_string(version_value);
        let mut matches = Vec::new();

        for game_set in &self.versionsets_data {
            for platform in &game_set.platforms {
                let is_match = if let Some(rw_version) = &platform.rw_version {
                    *rw_version == version_str
                } else if let (Some(min_ver), Some(max_ver)) =
                    (&platform.rw_version_min, &platform.rw_version_max)
                {
                    let min_hex = self.version_string_to_hex(min_ver);
                    let max_hex = self.version_string_to_hex(max_ver);
                    (min_hex..=max_hex).contains(&version_value)
                } else {
                    false
                };

                if is_match {
                    matches.push(GameInfo {
                        name: game_set.name.clone(),
                        display_name: game_set.display_name.clone(),
                        icon_name: game_set.icon_name.clone(),
                        platform: platform.platform.clone(),
                        data_types: platform.data_types.clone(),
                    });
                }
            }
        }

        matches
    }

    pub fn get_version_display_string(
        &self,
        version_value: i32,
        include_platforms: bool,
    ) -> String {
        if !include_platforms {
            return self.hex_to_version_string(version_value);
        }

        let matches = self.find_all_games_by_version(version_value);
        if matches.is_empty() {
            return self.hex_to_version_string(version_value);
        }

        let version_str = self.hex_to_version_string(version_value);

        // Group matches by game
        let mut game_groups: HashMap<String, Vec<String>> = HashMap::new();
        for game_match in matches {
            game_groups
                .entry(game_match.display_name)
                .or_insert_with(Vec::new)
                .push(game_match.platform);
        }

        // Format display string
        let mut game_parts = Vec::new();
        for (game_name, platforms) in game_groups {
            let mut sorted_platforms = platforms;
            sorted_platforms.sort();

            let platform_str = if sorted_platforms.len() >= 3 {
                format!("({}) All", game_name)
            } else if sorted_platforms.len() == 1 {
                format!("({}) {}", game_name, sorted_platforms[0])
            } else {
                format!("({}) {}", game_name, sorted_platforms.join(" / "))
            };

            game_parts.push(platform_str);
        }

        if game_parts.is_empty() {
            version_str
        } else {
            format!("{} {}", version_str, game_parts.join(" / "))
        }
    }

    #[allow(dead_code)]
    pub fn get_games_list(&self) -> Vec<GameSet> {
        self.versionsets_data.clone()
    }

    #[allow(dead_code)]
    pub fn get_platform_data_types(&self, platform: &str) -> Vec<String> {
        self.platform_info
            .get(platform)
            .cloned()
            .unwrap_or_default()
    }

    pub fn detect_file_format_version(
        &self,
        file_data: &[u8],
        filename: &str,
    ) -> Result<(String, String, i32), String> {
        if file_data.len() < 12 {
            return Ok(("Unknown".to_string(), "Unknown".to_string(), 0));
        }

        let filename_lower = filename.to_lowercase();

        // Check for COL files first
        if filename_lower.ends_with(".col") || file_data.len() >= 4 {
            if let Ok(fourcc) = std::str::from_utf8(&file_data[0..4]) {
                match fourcc {
                    "COLL" => return Ok(("COL".to_string(), "COL1 (GTA III/VC)".to_string(), 0)),
                    "COL2" => return Ok(("COL".to_string(), "COL2 (GTA SA)".to_string(), 0)),
                    "COL3" => {
                        return Ok(("COL".to_string(), "COL3 (GTA SA Advanced)".to_string(), 0))
                    }
                    "COL4" => return Ok(("COL".to_string(), "COL4 (Extended)".to_string(), 0)),
                    _ => {}
                }
            }
        }

        // Check for RenderWare files
        if file_data.len() >= 12 {
            // Read section type and version
            let section_type =
                u32::from_le_bytes([file_data[0], file_data[1], file_data[2], file_data[3]]);
            let version =
                u32::from_le_bytes([file_data[8], file_data[9], file_data[10], file_data[11]]);

            let rw_version = self.get_rw_version(Some(version as i32));

            match section_type {
                0x0010 => {
                    // CLUMP (DFF)
                    let version_str = self.get_version_display_string(rw_version, true);
                    Ok(("DFF".to_string(), version_str, rw_version))
                }
                0x0016 => {
                    // TEXDICTIONARY (TXD)
                    let version_str = self.get_version_display_string(rw_version, true);
                    Ok(("TXD".to_string(), version_str, rw_version))
                }
                _ if self.is_valid_rw_version(rw_version) => {
                    let ext = filename.split('.').last().unwrap_or("RW").to_uppercase();
                    let version_str = self.get_version_display_string(rw_version, true);
                    Ok((ext, version_str, rw_version))
                }
                _ => Ok(("Unknown".to_string(), "Unknown format".to_string(), 0)),
            }
        } else {
            let ext = filename
                .split('.')
                .last()
                .unwrap_or("Unknown")
                .to_uppercase();
            Ok((ext, "Unknown format".to_string(), 0))
        }
    }
}

impl Default for RenderWareVersionManager {
    fn default() -> Self {
        Self::new()
    }
}
