// Bevy app setup and initialization
// This module handles the Bevy ECS app lifecycle

use bevy::prelude::*;
use crate::bevy_bridge::{BevyCommandReceiver, BevyStateSender, BevyCommand, BevyState};
use crate::bevy_scene::{sync_scene_system, camera_control_system, process_camera_updates, setup_scene_system, ObjectEntityMap, CameraController};
use std::time::{SystemTime, UNIX_EPOCH};

/// Game state resource
#[derive(Resource, Default)]
pub struct GameState {
    /// Frame counter
    pub frame: u64,
    /// Custom game data
    pub data: serde_json::Value,
}

/// Example component for demonstration
#[derive(Component)]
pub struct ExampleComponent {
    pub value: f32,
}

/// System that processes commands from Tauri
/// Note: Most scene commands are handled by sync_scene_system in bevy_scene.rs
#[allow(unused_variables)]
fn process_commands_system(
    commands_receiver: ResMut<BevyCommandReceiver>,
    _game_state: ResMut<GameState>,
) {
    // Try to receive commands from Tauri (non-blocking)
    // Note: Scene sync commands are handled by sync_scene_system in bevy_scene.rs
    // This system only handles commands that aren't scene-related
    while let Ok(cmd) = commands_receiver.0.try_recv() {
        match cmd {
            BevyCommand::GetState => {
                // State will be sent by the state update system
            }
            BevyCommand::InputEvent { event: _, data: _ } => {
                // Process input events here
                // Input events can be handled by input systems
            }
            BevyCommand::SyncScene { .. }
            | BevyCommand::UpdateObject { .. }
            | BevyCommand::AddObject { .. }
            | BevyCommand::RemoveObject { .. }
            | BevyCommand::CameraUpdate { .. } => {
                // These are handled by sync_scene_system and process_camera_updates
                // We don't process them here to avoid double-processing
            }
            BevyCommand::Custom { command: _, data: _ } => {
                // Process custom commands
            }
        }
    }
}

/// System that updates game state every frame
fn update_game_state_system(mut game_state: ResMut<GameState>) {
    game_state.frame = game_state.frame.wrapping_add(1);
    
    // Update custom data
    game_state.data = serde_json::json!({
        "frame": game_state.frame,
        "entities": 0, // Can be updated with actual entity count
    });
}

/// System that sends state updates to Tauri
fn send_state_system(
    game_state: Res<GameState>,
    state_sender: Res<BevyStateSender>,
) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();

    let state = BevyState {
        frame: game_state.frame,
        timestamp,
        data: game_state.data.clone(),
    };

    // Send state update (non-blocking, ignore if channel is full)
    let _ = state_sender.0.try_send(state);
}

/// Example system that demonstrates ECS functionality
fn example_system(query: Query<&ExampleComponent>) {
    for component in query.iter() {
        // This system runs every frame for entities with ExampleComponent
        // For now, we just verify it's working
        if component.value > 0.0 {
            // Component is valid
        }
    }
}

/// Initialize the Bevy app
pub fn create_bevy_app(
    command_receiver: BevyCommandReceiver,
    state_sender: BevyStateSender,
    target_fps: Option<u32>,
    headless: bool,
) -> App {
    let mut app = App::new();

    if headless {
        // In headless mode, use MinimalPlugins to avoid any winit/window initialization
        // This prevents the "event loop outside main thread" error on Linux
        // MinimalPlugins includes: CorePlugin, TransformPlugin, HierarchyPlugin, DiagnosticPlugin
        app.add_plugins(MinimalPlugins);
        
        // Add asset plugin for loading resources (doesn't require windowing)
        // This is needed for loading meshes, textures, etc.
        app.add_plugins(AssetPlugin::default());
        
        // Manually insert Assets resources that are normally provided by RenderPlugin
        // These are just storage containers and don't require actual rendering or windowing
        app.insert_resource(Assets::<Mesh>::default());
        app.insert_resource(Assets::<StandardMaterial>::default());
        
        // Note: We don't add RenderPlugin in headless mode to avoid winit initialization
        // If offscreen rendering is needed later, it should be configured separately
    } else {
        // Add default plugins (includes windowing and rendering)
        app.add_plugins(DefaultPlugins);
    }

    // Configure schedule runner for controlled update loop
    let _fps = target_fps.unwrap_or(60);

    // Add resources
    app.insert_resource(GameState::default())
        .insert_resource(command_receiver)
        .insert_resource(state_sender)
        .insert_resource(ObjectEntityMap::default())
        .insert_resource(CameraController::default());

    // Add scene systems
    // Setup scene (camera, lighting) - works in both headless and windowed mode
    // In headless mode, we still need camera and lighting for offscreen rendering
    app.add_systems(Startup, setup_scene_system);

    // Add systems
    app.add_systems(Update, process_commands_system)
        .add_systems(Update, update_game_state_system)
        .add_systems(Update, send_state_system)
        .add_systems(Update, example_system)
        .add_systems(Update, sync_scene_system)
        .add_systems(Update, process_camera_updates);

    // Camera control system - works in both headless and windowed mode
    app.add_systems(Update, camera_control_system);

    // Spawn an example entity to demonstrate ECS
    app.world_mut().spawn(ExampleComponent { value: 1.0 });

    app
}
