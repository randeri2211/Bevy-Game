use bevy::prelude::*;
use crate::settings::systems::*;

pub mod components;
pub mod systems;

pub struct SettingsPlugin{}

impl Plugin for SettingsPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_settings)
            // .add_systems(Update,print_settings)
        ;
    }
}