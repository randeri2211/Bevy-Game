use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component,Reflect,Serialize,Deserialize,Default)]
#[reflect(Component,Serialize,Deserialize)]
pub struct Player {
    pub(crate) last_speed: f32,
}
