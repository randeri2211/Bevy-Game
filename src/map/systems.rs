use bevy::asset::LoadAssets;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;


pub fn load_map(server: Res<AssetServer>
){
    let map: Handle<T> = server.load("World/map.txt");
}

pub fn load_block(load_assets: LoadAssets){

}

