use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod player;
pub mod systems;
pub mod constants;

use systems::*;
use player::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(500.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup,spawn_camera)
        .add_systems(Startup,spawn_player)
        .add_systems(Update,player_input)
        .add_systems(Update,ability_system)
        .run();
}


