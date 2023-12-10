use bevy::prelude::*;
use crate::AppState;
use crate::game::GameState;
use crate::settings::components::Settings;
use crate::settings::systems::*;

pub mod components;
pub mod systems;

pub struct SettingsPlugin{}

impl Plugin for SettingsPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_settings)
            .register_type::<Settings>()
            .add_systems(Update,update_zoom.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Paused)))
            // .add_systems(Update,print_settings)
        ;
    }
}