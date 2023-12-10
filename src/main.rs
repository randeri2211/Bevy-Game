use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_simple_tilemap::prelude::*;

pub mod constants;
pub mod systems;

mod game;
pub mod block_id;
pub mod settings;

use crate::systems::*;
use crate::constants::*;
use crate::game::entities::components::*;
use crate::game::entities::player::components::*;
use crate::game::*;
use crate::game::map::components::*;
use crate::game::skills::skill_proj::*;
use crate::settings::*;
use crate::settings::systems::*;

fn main() {
    App::new()
        .add_state::<AppState>()
        // Given Plugins //
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METERS))
        // .add_plugins(RapierDebugRenderPlugin::default())

        // Game Plugin(Contains Map and Entites) //
        .add_plugins(GamePlugin{})

        // System Plugins //
        .add_plugins(SettingsPlugin{})

        // FPS Counter Plugins //
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())

        // Tile Map Plugin //
        .add_plugins(SimpleTileMapPlugin)

        // Systems //
        .add_systems(PostStartup,spawn_camera.after(load_settings))
        .add_systems(Update, toggle_app_state)
        .add_systems(Update,mana_regen)

        // Temp //
        .register_type::<Player>()
        .register_type::<HealthBar>()
        .register_type::<ManaBar>()
        .register_type::<MyTile>()
        .register_type::<Map>()
        .register_type::<Interactable>()
        .register_type::<SkillProjBase>()
        .register_type::<SkillProj>()


        .run();
}

#[derive(States, Debug, Eq, PartialEq, Clone, Copy, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}
