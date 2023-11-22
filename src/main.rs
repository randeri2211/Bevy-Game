use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod constants;
pub mod systems;

mod game;
pub mod block_id;


use crate::systems::*;
use crate::constants::*;
use crate::game::GamePlugin;


fn main() {
    App::new()
        .add_state::<AppState>()
        // Given Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METERS))
        .add_plugins(RapierDebugRenderPlugin::default())
        // Player Plugin
        .add_plugins(GamePlugin{})
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // Systems
        .add_systems(Startup,spawn_camera)
        .add_systems(Update, block_collisions_handler)
        .add_systems(Update,entity_collisions_handler)
        .add_systems(Update, toggle_app_state)
        .run();
}

#[derive(States, Debug, Eq, PartialEq, Clone, Copy, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}
