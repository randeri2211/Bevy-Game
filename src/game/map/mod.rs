use bevy::prelude::*;
use crate::game::map::systems::*;
use crate::AppState;

pub mod components;
pub mod systems;

pub struct MapPlugin{}

impl Plugin for MapPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), load_map)
            .add_systems(Update,(cam_follow_player).run_if(in_state(AppState::Game)))
        ;
    }
}