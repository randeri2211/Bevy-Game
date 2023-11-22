use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;
use crate::constants::*;
use crate::game::map::components::*;

#[derive(Bundle)]
struct BlockBundle{
    tile: Tile,
    collider: Collider,
    friction: Friction,
    global:GlobalTransform,
}


pub const ASSETS: &str = "assets/";




// pub const BLOCK_IDS: &[BlockBundle; 3] = &[
//     BlockBundle{
//         tile: Tile { id: 1 },
//         collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
//         friction: Friction{ coefficient: 0.2, combine_rule: CoefficientCombineRule::Multiply },
//         global: GlobalTransform::IDENTITY,
//     },
//     BlockBundle{
//         tile: Tile { id: 2 },
//         collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
//         friction: Friction{ coefficient: 1.0, combine_rule: CoefficientCombineRule::Multiply },
//         global: GlobalTransform::IDENTITY,
//     },
//     BlockBundle{
//         tile: Tile { id: 3 },
//         collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
//         friction: Friction{ coefficient: 10.0, combine_rule: CoefficientCombineRule::Multiply },
//         global: GlobalTransform::IDENTITY,
//     },
//
// ];
pub const BLOCK_IDS: [fn(&mut Commands, Vec3, asset_server: &Res<AssetServer>,&mut ResMut<Assets<Mesh>>, &mut ResMut<Assets<ColorMaterial>>)->();
    4] = [
    rock,
    grass,
    interact,
    grass
];

fn grass(
    commands: &mut Commands,
    position:Vec3,
    asset_server: &Res<AssetServer>,
    meshes:&mut ResMut<Assets<Mesh>>,
    materials:&mut ResMut<Assets<ColorMaterial>>,
)
{
    let texture_handle:Handle<Image> = asset_server.load("grass.png");
    let mesh = Mesh::from(shape::Quad::default());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

    commands.spawn(Collider::cuboid(TILE_SIZE, TILE_SIZE))
        // .insert(RigidBody::Fixed)
        .insert(Tile {
            id: 1,
        })
        .insert(MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(texture_handle)),
            ..default()
        })
        .insert(TransformBundle::from_transform(Transform {
            translation: position,
            scale: Vec3::splat(PIXELS_PER_METERS/2.0),
            ..default()
        }))
        .insert(ActiveCollisionTypes::all())
        .insert(Friction{ coefficient: NORMAL_FRICTION, combine_rule: CoefficientCombineRule::Multiply })
        .insert(ActiveEvents::COLLISION_EVENTS)
    ;
}

fn rock(
    commands: &mut Commands,
    position:Vec3,
    asset_server: &Res<AssetServer>,
    meshes:&mut ResMut<Assets<Mesh>>,
    materials:&mut ResMut<Assets<ColorMaterial>>,
)
{
    let texture_handle:Handle<Image> = asset_server.load("paneling.png");
    let mesh = Mesh::from(shape::Quad::default());
    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

    commands.spawn(Collider::cuboid(TILE_SIZE, TILE_SIZE))
        // .insert(RigidBody::Fixed)
        .insert(Tile {
            id: 1,
        })
        .insert(MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(texture_handle)),
            ..default()
        })
        .insert(TransformBundle::from_transform(Transform {
            translation: position,
            scale: Vec3::splat(PIXELS_PER_METERS/2.0),
            ..default()
        }))
        .insert(ActiveCollisionTypes::all())
        .insert(Friction{ coefficient: 0.0, combine_rule: CoefficientCombineRule::Multiply })
        .insert(ActiveEvents::COLLISION_EVENTS)
    ;
}

fn interact(
    commands: &mut Commands,
    position:Vec3,
    asset_server: &Res<AssetServer>,
    meshes:&mut ResMut<Assets<Mesh>>,
    materials:&mut ResMut<Assets<ColorMaterial>>,
)
{
    let texture_handle:Handle<Image> = asset_server.load("paneling_small.png");
    let mesh = Mesh::from(shape::Quad::default());

    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

    commands.spawn(Collider::cuboid(TILE_SIZE, TILE_SIZE))
        // .insert(RigidBody::Fixed)
        .insert(Tile {
            id: 1,
        })
        .insert(MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(texture_handle)),
            ..default()
        })
        .insert(TransformBundle::from_transform(Transform {
            translation: Vec3::new(position.x,position.y - PIXELS_PER_METERS * TILE_SIZE / 4.0,position.z),
            scale: Vec3::new(PIXELS_PER_METERS/2.0,PIXELS_PER_METERS/4.0,1.0),
            ..default()
        }))
        .insert(ActiveCollisionTypes::all())
        .insert(Friction{ coefficient: 0.0, combine_rule: CoefficientCombineRule::Multiply })
        .insert(Interactable{ has_inventory: true })
        .insert(ActiveEvents::COLLISION_EVENTS)

    ;
}