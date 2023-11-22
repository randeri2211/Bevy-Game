use bevy::prelude::*;


#[derive(Component)]
pub struct Tile {
    pub(crate) id:i32,
}

#[derive(Component)]
pub struct Interactable {
    pub(crate) has_inventory:bool,
}