use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod constants;
pub mod systems;
mod player;
mod map;
mod skills;

use map::systems::*;
use player::systems::*;
use systems::*;
use crate::constants::*;
use crate::map::components::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METERS))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup,load_map)
        .add_systems(Startup,spawn_camera)
        .add_systems(Startup,spawn_player)
        .add_systems(Update,cam_follow_player)
        .add_systems(Update,player_input)
        .add_systems(Update,ability_system)
        .add_systems(Update,swap_ability)
        .add_systems(Update,display_events)
        .insert_resource(Mapp {
            tiles: Vec::new(),
        })
        .run();
}
