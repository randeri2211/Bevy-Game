use std::fs;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use crate::constants::*;
use crate::map::components::*;


pub fn load_map(mut map: ResMut<Mapp>,
                mut commands: Commands,
                mut meshes: ResMut<Assets<Mesh>>,
                mut materials: ResMut<Assets<ColorMaterial>>,
){
    let contents = fs::read_to_string("World/map.txt")
        .expect("cant read the map");

    map.tiles.clear();

    let mut row:usize = 0;

    for line in contents.lines(){
        let mut col:usize = 0;
        let mut temp_vec = Vec::new();

        for tile in line.chars(){
            if tile == '\n'{
                println!("continue");
                continue;
            }

            let position_x = -TILES / 2.0 + col as f32 * TILE_SIZE * PIXELS_PER_METERS;
            let position_y = 0.0 - row as f32 * TILE_SIZE * PIXELS_PER_METERS ;

            let tile_color = match tile{
                '1' => Color::PURPLE,
                '0' => Color::GREEN,
                _ => Color::WHITE
            };

            let entity = commands.spawn(Collider::cuboid(TILE_SIZE / 2.0,TILE_SIZE / 2.0))
                .insert(Tile{
                    id: tile,
                    color: tile_color,
                })
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(TILE_SIZE).into()).into(),
                    material: materials.add(ColorMaterial::from(tile_color)),
                    ..default()
                })
                .insert(TransformBundle::from_transform(Transform{
                    translation: Vec3::new(position_x, position_y, 0.0),
                    scale: Vec3::new(PIXELS_PER_METERS, PIXELS_PER_METERS, 1.0),
                    ..default()
                }))
                // .insert(ActiveEvents::COLLISION_EVENTS)
                // .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
                .insert(ActiveCollisionTypes::all())
                .id();

            temp_vec.insert(col,entity);

            col += 1;
        }
        map.tiles.insert(row,temp_vec);
        row += 1;
    }
}
