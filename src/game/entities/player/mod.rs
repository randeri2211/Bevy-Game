use bevy::prelude::*;

pub mod components;
pub mod systems;
use systems::*;
use crate::AppState;
use crate::game::GameState;

pub struct PlayerPlugin{}

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Update,ability_system.run_if(in_state(AppState::Game))
            //     .run_if(in_state(GameState::Running)))
            .add_systems(Update,player_input.run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)))
            .add_systems(Update,swap_ability.run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)))
        ;
    }
}