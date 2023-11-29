use bevy_simple_tilemap::prelude::*;
use std::fs;
// use bevy_ecs_tilemap::prelude::*;
// use bevy_simple_tilemap::prelude::*;
use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::prelude::ColliderMassProperties::Mass;
use crate::block_id::BLOCK_IDS;
use crate::constants::*;
use crate::game::entities::components::*;
use crate::game::entities::player::components::*;
use crate::game::map::components::Map;
use crate::game::skills::skills::*;


pub fn load_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    // new tilemap

    let texture_handle = asset_server.load("tilesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1, Some(Vec2::new(1.0, 1.0)), None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut tilemap = TileMap::default();

    let contents = fs::read_to_string("World/map.txt")
        .expect("cant read the map");


    let mut start_x = 0;
    let mut start_y = 0;
    for line in contents.lines(){
        start_x = 0;
        for _ in line.chars(){
            start_x += 1;
        }
        start_y += 1;
    }

    start_x = - start_x / 2;
    start_y = - start_y / 2;
    let mut tiles:Vec<Entity> = Vec::new();
    let mut row = -start_y;
    for line in contents.lines(){
        let mut col = start_x;

        for tile in line.chars(){
            let position_x = -TILES / 2 + col;
            let position_y = - row ;
            let mut tile_e:Entity = Entity::from_raw(0);
            if tile.is_digit(10) {
                let index:usize = tile.to_digit(10).unwrap() as usize;
                tile_e = BLOCK_IDS[index](
                    &mut commands,
                    IVec3::new(position_x, position_y, 0),
                    // &asset_server,
                    // &mut meshes,
                    // &mut materials,
                    &mut tilemap,
                );
            }else if tile == 'x' {
                tile_e = spawn_player(&mut commands,Vec3::new(position_x as f32 * PIXELS_PER_METERS * TILE_SIZE,position_y as f32 * PIXELS_PER_METERS * TILE_SIZE,0.0));
            }
            tiles.push(tile_e);
            col += 1;
        }
        row += 1;
    }

    // Set up tilemap
    let tilemap_bundle = TileMapBundle {
        tilemap,
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            scale: Vec3::splat(3.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    };

    // Spawn tilemap
    commands.spawn(tilemap_bundle)
        .insert(Name::new("map"))
        // .insert(Transform::default())
        // .insert(GlobalTransform::default())
        .insert(SpatialBundle::default())
        .insert(Map{})
        .push_children(&tiles)
    ;
}


pub fn cam_follow_player(
    mut c_query: Query<&mut Transform,With<Camera>>,
    p_query: Query<&Transform,(Without<Camera>,With<Player>)>
){
    let mut c_transform = c_query.get_single_mut().unwrap();
    let p_transform = p_query.get_single().unwrap();

    c_transform.translation = p_transform.translation;
}

pub fn spawn_player(commands: &mut Commands, translation_vec:Vec3)->Entity{
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
        .insert(Collider::capsule_y(TILE_SIZE / 2.0,TILE_SIZE / 2.0))
        .insert(TransformBundle::from_transform(Transform {
            translation: translation_vec,
            scale: Vec3::splat(PIXELS_PER_METERS),
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
        .insert(GravityScale{ 0: 1.0 })
        .insert(Mass(PLAYER_MASS))
        .insert(LockedAxes::ROTATION_LOCKED)
        // Player Related //
        .insert(Player { ..default() })
        .insert(Skills { skills_vec: player_skills })
        .insert(MageBundle{
            health: HealthBar{
                max_health: 100.0,
                current_health: 100.0,
                health_regen: 1.0,
            },
            mana: ManaBar{
                max_mana: 100.0,
                current_mana: 100.0,
                mana_regen: 10.0,
            }
        })
        .insert(Name::new("Player"))
        .id()
}