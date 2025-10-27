use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectType {
    Cube,
    Sphere,
    Plane,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapObject {
    pub id: String,
    #[serde(rename = "objectType")]
    pub object_type: ObjectType,
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
    pub color: String,
    pub name: String,
    pub visible: bool,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapSettings {
    #[serde(rename = "gridSize")]
    pub grid_size: f64,
    #[serde(rename = "snapToGrid")]
    pub snap_to_grid: bool,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    pub id: String,
    pub name: String,
    pub version: String,
    pub objects: Vec<MapObject>,
    pub settings: MapSettings,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl Default for MapData {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "Untitled Map".to_string(),
            version: "1.0.0".to_string(),
            objects: Vec::new(),
            settings: MapSettings {
                grid_size: 1.0,
                snap_to_grid: true,
                background_color: "#1a1a1a".to_string(),
            },
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl Default for MapObject {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            object_type: ObjectType::Cube,
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            color: "#ffffff".to_string(),
            name: "New Object".to_string(),
            visible: true,
            metadata: None,
        }
    }
}
