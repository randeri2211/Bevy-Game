use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod constants;
pub mod systems;
mod player;
mod map;

use map::systems::*;
use player::systems::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(500.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup,load_map)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_input)
        .add_systems(Update, ability_system)
        .run();
}
