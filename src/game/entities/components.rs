use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component,Reflect,Serialize,Deserialize,Default)]
#[reflect(Component,Serialize,Deserialize)]
pub struct HealthBar{
    pub(crate) max_health:f32,
    pub(crate) current_health:f32,
    pub(crate) health_regen:f32,
}

#[derive(Component,Reflect,Serialize,Deserialize,Default)]
#[reflect(Component,Serialize,Deserialize)]
pub struct ManaBar{
    pub(crate) max_mana:f32,
    pub(crate) current_mana:f32,
    pub(crate) mana_regen:f32,
}


#[derive(Bundle)]
pub struct MageBundle {
    pub(crate) health:HealthBar,
    pub(crate) mana:ManaBar,
}