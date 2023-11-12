use bevy::prelude::*;

#[derive(Resource)]
pub struct Mapp {
    pub(crate) tiles:Vec<Vec<Entity>>
}


#[derive(Component)]
pub struct Tile {
    pub(crate) id: char,
    pub(crate) color: Color,
}
