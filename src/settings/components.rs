use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Copy, Serialize, Deserialize,Debug,Reflect,InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Settings {
    pub(crate) master_volume: f32,
    pub(crate) auto_step:bool,
    #[inspector(min = 0.5, max = 1.0)]
    pub(crate) zoom:f32
}

impl Default for Settings{
    fn default() -> Self {
        Settings{
            master_volume: 100.0,
            auto_step: true,
            zoom: 0.5,
        }
    }
}