use std::fs;
use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;
use crate::block_id::BLOCK_IDS;
use crate::constants::*;
use crate::game::player::components::*;
use crate::game::skills::skills::*;


pub fn load_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
){

    let contents = fs::read_to_string("World/map.txt")
        .expect("cant read the map");


    let mut row:usize = 0;

    for line in contents.lines(){
        let mut col:usize = 0;

        for tile in line.chars(){
            let position_x = -TILES / 2.0 + col as f32 * TILE_SIZE * PIXELS_PER_METERS;
            let position_y = 0.0 - row as f32 * TILE_SIZE * PIXELS_PER_METERS;
            if tile.is_digit(10) {
                let index:usize = tile.to_digit(10).unwrap() as usize;
                BLOCK_IDS[index](
                    &mut commands,
                    Vec3::new(position_x, position_y, 0.0),
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                );
            }else if tile == 'x' {
                spawn_player(&mut commands,Vec3::new(position_x,position_y + PIXELS_PER_METERS / 4.0,0.0));
            }
            col += 1;
        }
        row += 1;
    }
}


pub fn cam_follow_player(
    mut c_query: Query<&mut Transform,With<Camera>>,
    p_query: Query<&Transform,(Without<Camera>,With<Player>)>
){
    let mut c_transform = c_query.get_single_mut().unwrap();
    let p_transform = p_query.get_single().unwrap();

    c_transform.translation = p_transform.translation;
}

pub fn spawn_player(commands: &mut Commands, translation_vec:Vec3) {
    // Skills init
    let mut player_skills:Vec<SkillBase> = Vec::new();
    player_skills.insert(0, SkillBase::default());

    let mut another_skill = SkillBase::default();
    another_skill.shoot = reverse;
    another_skill.active = false;
    player_skills.insert(0, another_skill);

    // Player init
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule_y(TILE_SIZE, TILE_SIZE))
        .insert(TransformBundle::from_transform(Transform {
            translation: translation_vec,
            scale: Vec3::splat(PIXELS_PER_METERS / 2.0),
            ..default()
        }))
        .insert(Restitution::coefficient(0.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0 , 0.0),
            angvel: 0.0,
        })
        .insert(Friction {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Multiply,
        })
        // .insert(GravityScale{ 0: 0.0 })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Player { ..default() })
        .insert(Skills { skills_vec: player_skills })
    ;
}