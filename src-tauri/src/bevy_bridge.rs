// Communication bridge between Bevy and Tauri
// Uses crossbeam channels for thread-safe message passing

use crossbeam_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};

/// Scene object data structure (matches frontend SceneObject)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneObjectData {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String, // "cube" | "sphere" | "plane" | "imported"
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub color: String,
    pub visible: bool,
}

/// Scene light data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneLightData {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub light_type: String,
    pub position: Option<[f32; 3]>,
    pub color: String,
    pub intensity: f32,
    pub visible: bool,
}

/// Full scene state for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneState {
    pub objects: Vec<SceneObjectData>,
    pub lights: Vec<SceneLightData>,
    pub camera_position: [f32; 3],
    pub camera_target: [f32; 3],
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    pub fog_enabled: bool,
    pub fog_color: String,
    pub fog_near: f32,
    pub fog_far: f32,
}

/// Messages sent from Tauri to Bevy (commands/input)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BevyCommand {
    /// Request current game state
    GetState,
    /// Send input event (e.g., keyboard, mouse)
    InputEvent { event: String, data: serde_json::Value },
    /// Sync full scene state from frontend
    SyncScene { scene: SceneState },
    /// Update individual object
    UpdateObject { object: SceneObjectData },
    /// Add new object to scene
    AddObject { object: SceneObjectData },
    /// Remove object from scene
    RemoveObject { object_id: String },
    /// Update camera position/target
    CameraUpdate { position: [f32; 3], target: [f32; 3] },
    /// Custom command for future use
    Custom { command: String, data: serde_json::Value },
}

/// Messages sent from Bevy to Tauri (state updates)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BevyState {
    /// Frame counter to verify Bevy is running
    pub frame: u64,
    /// Timestamp of last update
    pub timestamp: f64,
    /// Custom state data (can be extended)
    pub data: serde_json::Value,
}

impl Default for BevyState {
    fn default() -> Self {
        Self {
            frame: 0,
            timestamp: 0.0,
            data: serde_json::json!({}),
        }
    }
}

/// Resource for sending commands to Bevy from Tauri
pub struct BevyCommandSender(pub Sender<BevyCommand>);

/// Resource for receiving state updates from Bevy in Tauri
pub struct BevyStateReceiver(pub Receiver<BevyState>);

/// Resource for sending state updates from Bevy to Tauri
#[derive(bevy::prelude::Resource)]
pub struct BevyStateSender(pub Sender<BevyState>);

/// Resource for receiving commands in Bevy from Tauri
#[derive(bevy::prelude::Resource)]
pub struct BevyCommandReceiver(pub Receiver<BevyCommand>);
