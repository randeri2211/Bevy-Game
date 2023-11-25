use bevy::prelude::*;
use crate::game::map::MapPlugin;
use crate::game::entities::player::PlayerPlugin;

pub mod map;
pub mod skills;
pub mod entities;

pub struct GamePlugin{}

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_plugins(MapPlugin{})
            .add_plugins(PlayerPlugin{})
        ;
    }
}


#[derive(States, Debug, Eq, PartialEq, Clone, Copy, Hash, Default)]
enum GameState {
    #[default]
    Running,
    Paused,
}
