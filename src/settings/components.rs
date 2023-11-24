use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Copy, Serialize, Deserialize,Debug)]
pub struct Settings {
    pub(crate) master_volume: f64,
    pub(crate) auto_step:bool,
}

impl Default for Settings{
    fn default() -> Self {
        Settings{
            master_volume: 100.0,
            auto_step: true,
        }
    }
}