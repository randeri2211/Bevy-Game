use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_simple_tilemap::prelude::*;

pub mod constants;
pub mod systems;

mod game;
pub mod block_id;
pub mod settings;

use crate::systems::*;
use crate::constants::*;
use crate::game::GamePlugin;
use crate::settings::SettingsPlugin;
fn main() {
    App::new()
        .add_state::<AppState>()
        // Given Plugins //
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METERS))
        // .add_plugins(RapierDebugRenderPlugin::default())
        // Player Plugin //
        .add_plugins(GamePlugin{})
        // System Plugins //
        .add_plugins(SettingsPlugin{})
        // FPS Counter Plugins //
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(SimpleTileMapPlugin)

        // Systems //
        .add_systems(Startup,spawn_camera)
        .add_systems(Update, toggle_app_state)
        .add_systems(Update,mana_regen)
        .run();
}

#[derive(States, Debug, Eq, PartialEq, Clone, Copy, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}
