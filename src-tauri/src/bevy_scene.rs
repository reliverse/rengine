// Bevy scene management
// Handles scene objects, lights, and camera in Bevy ECS
// Note: This module requires Bevy rendering features to be enabled

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]

use bevy::prelude::*;
use crate::bevy_bridge::{BevyCommandReceiver, BevyCommand, SceneObjectData};
use std::collections::HashMap;

/// Component that links frontend object ID to Bevy entity
#[derive(Component, Debug, Clone)]
pub struct SceneObjectEntity {
    pub frontend_id: String,
}

/// Component that stores object properties
#[derive(Component, Debug, Clone)]
pub struct SceneObjectProperties {
    pub name: String,
    pub object_type: String,
    pub color: String,
    pub visible: bool,
}

/// Component for scene lights
#[derive(Component, Debug, Clone)]
pub struct SceneLightComponent {
    pub frontend_id: String,
    pub name: String,
    pub light_type: String,
    pub color: String,
    pub intensity: f32,
    pub visible: bool,
}

/// Resource that maps frontend object IDs to Bevy entities
#[derive(Resource, Default)]
pub struct ObjectEntityMap {
    pub map: HashMap<String, Entity>,
}

/// Resource for camera controller state
#[derive(Resource)]
pub struct CameraController {
    pub position: Vec3,
    pub target: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            position: Vec3::new(5.0, 5.0, 5.0),
            target: Vec3::ZERO,
        }
    }
}

/// System that processes scene sync commands from frontend
pub fn sync_scene_system(
    mut commands: Commands,
    mut commands_receiver: ResMut<BevyCommandReceiver>,
    mut object_map: ResMut<ObjectEntityMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Process all pending commands
    while let Ok(cmd) = commands_receiver.0.try_recv() {
        match cmd {
            BevyCommand::SyncScene { scene } => {
                // Clear existing objects
                // Note: In a full implementation, we'd want to update existing entities
                // For now, we'll just spawn new ones
                
                // Spawn objects
                for object in scene.objects {
                    spawn_scene_object(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &mut object_map,
                        object,
                    );
                }
                
                // TODO: Handle lights and camera updates
            }
            BevyCommand::UpdateObject { object } => {
                // Update existing object or spawn if doesn't exist
                if let Some(entity) = object_map.map.get(&object.id) {
                    // Update existing entity
                    commands.entity(*entity).insert((
                        Transform::from_translation(Vec3::from_array(object.position))
                            .with_rotation(Quat::from_euler(
                                EulerRot::XYZ,
                                object.rotation[0],
                                object.rotation[1],
                                object.rotation[2],
                            ))
                            .with_scale(Vec3::from_array(object.scale)),
                        SceneObjectProperties {
                            name: object.name.clone(),
                            object_type: object.object_type.clone(),
                            color: object.color.clone(),
                            visible: object.visible,
                        },
                    ));
                } else {
                    // Spawn new entity
                    spawn_scene_object(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &mut object_map,
                        object,
                    );
                }
            }
            BevyCommand::AddObject { object } => {
                spawn_scene_object(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut object_map,
                    object,
                );
            }
            BevyCommand::RemoveObject { object_id } => {
                if let Some(entity) = object_map.map.remove(&object_id) {
                    commands.entity(entity).despawn();
                }
            }
            _ => {
                // Other commands handled elsewhere
            }
        }
    }
}

/// Helper function to spawn a scene object
fn spawn_scene_object(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    object_map: &mut ResMut<ObjectEntityMap>,
    object: SceneObjectData,
) {
    // Create mesh based on object type
    // Note: In Bevy 0.17, primitives can be converted to Mesh using Mesh::from()
    let mesh_handle = match object.object_type.as_str() {
        "cube" => {
            // Create a cube mesh
            meshes.add(Cuboid::new(1.0, 1.0, 1.0))
        }
        "sphere" => {
            // Create a sphere mesh
            meshes.add(Sphere::new(0.5))
        }
        "plane" => {
            // Create a plane mesh - use a thin box for a 3D plane
            meshes.add(Cuboid::new(1.0, 0.01, 1.0))
        }
        _ => {
            // Default to cube
            meshes.add(Cuboid::new(1.0, 1.0, 1.0))
        }
    };

    // Parse color (assuming hex format like "#ff0000")
    let color = parse_color(&object.color).unwrap_or(Color::srgb(1.0, 1.0, 1.0));
    let material_handle = materials.add(StandardMaterial {
        base_color: color,
        ..default()
    });

    // Create transform
    let transform = Transform::from_translation(Vec3::from_array(object.position))
        .with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            object.rotation[0],
            object.rotation[1],
            object.rotation[2],
        ))
        .with_scale(Vec3::from_array(object.scale));

    // Spawn entity
    // Note: In Bevy 0.17, use Mesh3d and MeshMaterial3d components
    let entity = commands
        .spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(material_handle),
            transform,
            SceneObjectEntity {
                frontend_id: object.id.clone(),
            },
            SceneObjectProperties {
                name: object.name,
                object_type: object.object_type,
                color: object.color,
                visible: object.visible,
            },
            Visibility::default(),
        ))
        .id();

    // Store in map
    object_map.map.insert(object.id, entity);
}

/// Helper function to parse hex color string
fn parse_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color::srgb(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        ))
    } else {
        None
    }
}

/// System that updates camera based on camera controller resource
pub fn camera_control_system(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<SceneObjectEntity>)>,
    camera_controller: Res<CameraController>,
) {
    if let Some(mut transform) = camera_query.iter_mut().next() {
        transform.translation = camera_controller.position;
        transform.look_at(camera_controller.target, Vec3::Y);
    }
}

/// System that processes camera update commands
pub fn process_camera_updates(
    mut commands_receiver: ResMut<BevyCommandReceiver>,
    mut camera_controller: ResMut<CameraController>,
) {
    while let Ok(cmd) = commands_receiver.0.try_recv() {
        if let BevyCommand::CameraUpdate { position, target } = cmd {
            camera_controller.position = Vec3::from_array(position);
            camera_controller.target = Vec3::from_array(target);
        }
    }
}

/// System that sets up basic scene (camera, lighting)
pub fn setup_scene_system(
    mut commands: Commands,
    mut camera_controller: ResMut<CameraController>,
) {
    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_controller.position)
            .looking_at(camera_controller.target, Vec3::Y),
    ));

    // Spawn basic lighting
    commands.spawn(DirectionalLight {
        color: Color::srgb(1.0, 1.0, 1.0),
        illuminance: 1000.0,
        shadows_enabled: true,
        ..default()
    });

    commands.spawn(AmbientLight {
        color: Color::srgb(1.0, 1.0, 1.0),
        brightness: 0.3,
        affects_lightmapped_meshes: true,
    });
}
 