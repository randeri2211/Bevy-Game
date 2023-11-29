use bevy::prelude::*;


#[derive(Component)]
pub struct MyTile {
    pub(crate) id:i32,
}

#[derive(Component)]
pub struct Map {
}

#[derive(Component)]
pub struct Interactable {
    pub(crate) has_inventory:bool,
}