use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Component,Reflect,Serialize,Deserialize,Default)]
#[reflect(Component,Serialize,Deserialize)]
pub struct MyTile {
    pub(crate) id:i32,
}

#[derive(Component,Reflect,Serialize,Deserialize,Default)]
#[reflect(Component,Serialize,Deserialize)]
pub struct Map {
}

#[derive(Component,Reflect,Serialize,Deserialize,Default)]
#[reflect(Component,Serialize,Deserialize)]
pub struct Interactable {
    pub(crate) has_inventory:bool,
}