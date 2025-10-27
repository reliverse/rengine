use crate::types::{MapData, MapObject, ObjectType, Vector3};
use serde::Deserialize;
use serde_json;
use std::fs;
use tauri::{async_runtime::Mutex, State};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AddObjectParams {
    pub object_type: String,
    pub position: Vector3,
}

pub struct AppState {
    pub current_map: Option<MapData>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { current_map: None }
    }
}

#[tauri::command]
pub async fn create_new_map(state: State<'_, Mutex<AppState>>) -> Result<MapData, String> {
    let mut app_state = state.lock().await;
    let new_map = MapData::default();
    app_state.current_map = Some(new_map.clone());
    Ok(new_map)
}

#[tauri::command]
pub async fn load_map(state: State<'_, Mutex<AppState>>, path: String) -> Result<MapData, String> {
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    let map_data: MapData =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse map data: {}", e))?;

    let mut app_state = state.lock().await;
    app_state.current_map = Some(map_data.clone());

    Ok(map_data)
}

#[tauri::command]
pub async fn save_map(state: State<'_, Mutex<AppState>>, path: String) -> Result<(), String> {
    let app_state = state.lock().await;

    if let Some(map_data) = &app_state.current_map {
        let content = serde_json::to_string_pretty(map_data)
            .map_err(|e| format!("Failed to serialize map data: {}", e))?;

        fs::write(&path, content).map_err(|e| format!("Failed to write file: {}", e))?;
    } else {
        return Err("No map data to save".to_string());
    }

    Ok(())
}

#[tauri::command]
pub async fn add_object(
    state: State<'_, Mutex<AppState>>,
    params: AddObjectParams,
) -> Result<MapObject, String> {
    let mut app_state = state.lock().await;

    let obj_type = match params.object_type.as_str() {
        "cube" => ObjectType::Cube,
        "sphere" => ObjectType::Sphere,
        "plane" => ObjectType::Plane,
        "custom" => ObjectType::Custom,
        _ => return Err("Invalid object type".to_string()),
    };

    let mut new_object = MapObject::default();
    new_object.object_type = obj_type;
    new_object.position = params.position;
    new_object.id = Uuid::new_v4().to_string();

    // If no map is loaded, create a new one
    if app_state.current_map.is_none() {
        app_state.current_map = Some(MapData::default());
    }

    if let Some(ref mut map_data) = app_state.current_map {
        map_data.objects.push(new_object.clone());
        map_data.updated_at = chrono::Utc::now().to_rfc3339();
    }

    Ok(new_object)
}

#[tauri::command]
pub async fn update_object(
    state: State<'_, Mutex<AppState>>,
    object_id: String,
    position: Option<Vector3>,
    rotation: Option<Vector3>,
    scale: Option<Vector3>,
    color: Option<String>,
    name: Option<String>,
    object_type: Option<String>,
    visible: Option<bool>,
) -> Result<(), String> {
    let mut app_state = state.lock().await;

    if let Some(ref mut map_data) = app_state.current_map {
        if let Some(obj) = map_data.objects.iter_mut().find(|o| o.id == object_id) {
            if let Some(pos) = position {
                obj.position = pos;
            }
            if let Some(rot) = rotation {
                obj.rotation = rot;
            }
            if let Some(scl) = scale {
                obj.scale = scl;
            }
            if let Some(col) = color {
                obj.color = col;
            }
            if let Some(nm) = name {
                obj.name = nm;
            }
            if let Some(vis) = visible {
                obj.visible = vis;
            }
            if let Some(ot) = object_type {
                let obj_type = match ot.as_str() {
                    "cube" => ObjectType::Cube,
                    "sphere" => ObjectType::Sphere,
                    "plane" => ObjectType::Plane,
                    "custom" => ObjectType::Custom,
                    _ => return Err("Invalid object type".to_string()),
                };
                obj.object_type = obj_type;
            }
            map_data.updated_at = chrono::Utc::now().to_rfc3339();
        } else {
            return Err("Object not found".to_string());
        }
    } else {
        return Err("No map loaded".to_string());
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_object(
    state: State<'_, Mutex<AppState>>,
    object_id: String,
) -> Result<(), String> {
    let mut app_state = state.lock().await;

    if let Some(ref mut map_data) = app_state.current_map {
        map_data.objects.retain(|o| o.id != object_id);
        map_data.updated_at = chrono::Utc::now().to_rfc3339();
    } else {
        return Err("No map loaded".to_string());
    }

    Ok(())
}

#[tauri::command]
pub async fn get_current_map(state: State<'_, Mutex<AppState>>) -> Result<Option<MapData>, String> {
    let app_state = state.lock().await;
    Ok(app_state.current_map.clone())
}

#[tauri::command]
pub async fn set_current_map(
    state: State<'_, Mutex<AppState>>,
    map_data: MapData,
) -> Result<(), String> {
    let mut app_state = state.lock().await;
    app_state.current_map = Some(map_data);
    Ok(())
}

#[tauri::command]
pub async fn sync_map_data(
    state: State<'_, Mutex<AppState>>,
    map_data: MapData,
) -> Result<(), String> {
    let mut app_state = state.lock().await;
    app_state.current_map = Some(map_data);
    Ok(())
}
