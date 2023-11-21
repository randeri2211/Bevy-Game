use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub(crate) last_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player { last_speed: 0.0 }
    }
}
