use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use crate::AppState;
use crate::game::map::MapPlugin;
use crate::game::entities::player::PlayerPlugin;
use crate::game::systems::toggle_game_state;
use crate::systems::*;

pub mod map;
pub mod skills;
pub mod entities;
pub mod systems;

pub struct GamePlugin{}

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_plugins(MapPlugin{})
            .add_plugins(PlayerPlugin{})
            .add_systems(Update, (block_collisions_handler,entity_collisions_handler).run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)))
            .add_systems(Update,toggle_game_state.run_if(in_state(AppState::Game)))
        ;
    }
}


#[derive(States, Debug, Eq, PartialEq, Clone, Copy, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}
