use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_simple_tilemap::prelude::*;
use crate::constants::*;
use crate::game::map::components::*;

#[derive(Bundle)]
struct BlockBundle{
    tile: MyTile,
    collider: Collider,
    friction: Friction,
    transform: TransformBundle,
}

pub const BLOCK_IDS: [fn(
    &mut Commands,
    IVec3,
    &mut TileMap
)->Entity;
    4] = [
    grass,
    rock,
    interact,
    grass
];



fn grass(
    commands: &mut Commands,
    position:IVec3,
    tilemap: &mut TileMap,
)->Entity
{

    tilemap.set_tile(position, Some(Tile { sprite_index: 1, color: Color::WHITE ,..default()}));
    commands
        .spawn(RigidBody::Fixed)
        .insert( BlockBundle{
            tile: MyTile {
                id: 0,
            },
            collider: Collider::cuboid(TILE_SIZE, TILE_SIZE),
            friction: Friction{ coefficient: NORMAL_FRICTION, combine_rule: CoefficientCombineRule::Multiply },
            transform: TransformBundle::from_transform(Transform {
                translation: position.as_vec3() * PIXELS_PER_METERS * TILE_SIZE,
                scale: Vec3::splat(PIXELS_PER_METERS/2.0),
                ..default()
            }),
        })
        .insert(ActiveCollisionTypes::all())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution {
            coefficient: 0.2,
            combine_rule: CoefficientCombineRule::Multiply,
        })
        .insert(Name::new("Grass"))
        .id()
}

fn rock(
    commands: &mut Commands,
    position:IVec3,
    tilemap: &mut TileMap,
)->Entity
{

    tilemap.set_tile(position, Some(Tile { sprite_index: 0, color: Color::WHITE ,..default()}));

    commands.spawn(RigidBody::Fixed)
        .insert( BlockBundle{
            tile: MyTile {
                id: 1,
            },
            collider: Collider::cuboid(TILE_SIZE, TILE_SIZE),
            friction: Friction{ coefficient: NORMAL_FRICTION * 2.0, combine_rule: CoefficientCombineRule::Multiply },
            transform: TransformBundle::from_transform(Transform {
                translation: position.as_vec3() * PIXELS_PER_METERS * TILE_SIZE,
                scale: Vec3::splat(PIXELS_PER_METERS/2.0),
                ..default()
            }),
        })
        .insert(ActiveCollisionTypes::all())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Multiply,
        })
        .insert(Name::new("Rock"))

        .id()
}

fn interact(
    commands: &mut Commands,
    position:IVec3,
    tilemap: &mut TileMap,
)->Entity
{

    tilemap.set_tile(position, Some(Tile { sprite_index: 4, color: Color::WHITE ,..default()}));
    commands.spawn(RigidBody::Fixed)
        .insert( BlockBundle{
            tile: MyTile {
                id: 2,
            },
            collider:Collider::cuboid(TILE_SIZE, TILE_SIZE / 2.0),
            friction: Friction{ coefficient: NORMAL_FRICTION, combine_rule: CoefficientCombineRule::Multiply },
            transform: TransformBundle::from_transform(Transform {
                translation: Vec3::new(position.x as f32 * PIXELS_PER_METERS * TILE_SIZE,
                                       (position.y as f32 - 1.0 / 4.0) * PIXELS_PER_METERS * TILE_SIZE ,
                                       position.z as f32),
                scale: Vec3::splat(PIXELS_PER_METERS/2.0),
                ..default()
            }),
        })
        .insert(ActiveCollisionTypes::all())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Multiply,
        })
        .insert(Name::new("Interact"))
        .id()
}