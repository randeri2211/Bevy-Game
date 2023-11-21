use bevy::prelude::*;
use crate::game::map::MapPlugin;
use crate::game::player::PlayerPlugin;

pub mod player;
pub mod map;
pub mod skills;

pub struct GamePlugin{}

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_plugins(MapPlugin{})
            .add_plugins(PlayerPlugin{});
    }
}


#[derive(States, Debug, Eq, PartialEq, Clone, Copy, Hash, Default)]
enum GameState {
    #[default]
    Running,
    Paused,
}
