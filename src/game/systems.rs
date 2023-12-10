use bevy::prelude::*;
use crate::game::GameState;

pub fn toggle_game_state(mut commands: Commands,
                         keys: Res<Input<KeyCode>>,
                         game_state: Res<State<GameState>>){
    if keys.just_pressed(KeyCode::Escape){
        if *game_state.get() == GameState::Running{
            commands.insert_resource(NextState(Some(GameState::Paused)));
            println!("Paused");
        }
        else if *game_state.get() == GameState::Paused{
            commands.insert_resource(NextState(Some(GameState::Running)));
            println!("Running");
        }
    }
}