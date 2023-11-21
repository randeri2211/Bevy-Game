use bevy::prelude::*;


#[derive(Component)]
pub struct Tile {
    pub(crate) id: char,
    pub(crate) color: Color,
}
