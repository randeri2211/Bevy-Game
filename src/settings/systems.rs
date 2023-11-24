use fs::read_to_string;
use std::fs;
use std::path::Path;
use bevy::prelude::*;
use crate::settings::components::*;

pub fn save_settings(settings: Settings){
    let res = serde_json::to_string_pretty(&settings);
    // println!("{}",res.unwrap());
    let res = fs::write("settings.json", res.unwrap()).expect("Couldnt save settings file");
    println!("save_settings:{:?}",res);
}

pub fn load_settings(mut commands: Commands)
{
    // Load settings if exists,otherwise create default settings
    if !Path::new("settings.json").is_file(){
        save_settings(Settings::default());
    }
    let binding = read_to_string("settings.json").expect("Couldnt read settings file");
    let set = binding.as_str();
    let res:Settings = serde_json::from_str(set).unwrap();
    commands.insert_resource(res);
}

pub fn print_settings(settings: ResMut<Settings>){
    println!("{:?}",settings.into_inner());
}

