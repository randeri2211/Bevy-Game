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
    transform: TransformBundle,
}

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

    commands
        // .insert(RigidBody::Fixed)
        .spawn(MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::from(texture_handle)),
            ..default()
        })
        .insert( BlockBundle{
            tile: Tile {
                id: 0,
            },
            collider: Collider::cuboid(TILE_SIZE, TILE_SIZE),
            friction: Friction{ coefficient: NORMAL_FRICTION, combine_rule: CoefficientCombineRule::Multiply },
            transform: TransformBundle::from_transform(Transform {
                translation: position,
                scale: Vec3::splat(PIXELS_PER_METERS/2.0),
                ..default()
            }),
        })
        .insert(ActiveCollisionTypes::all())
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

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handle,
        material: materials.add(ColorMaterial::from(texture_handle)),
        ..default()
    })
        // .insert(RigidBody::Fixed)
        .insert( BlockBundle{
            tile: Tile {
                id: 1,
            },
            collider: Collider::cuboid(TILE_SIZE, TILE_SIZE),
            friction: Friction{ coefficient: NORMAL_FRICTION * 2.0, combine_rule: CoefficientCombineRule::Multiply },
            transform: TransformBundle::from_transform(Transform {
                translation: position,
                scale: Vec3::splat(PIXELS_PER_METERS/2.0),
                ..default()
            }),
        })

        .insert(ActiveCollisionTypes::all())
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


    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handle,
        material: materials.add(ColorMaterial::from(texture_handle)),
        ..default()
    })
        // .insert(RigidBody::Fixed)
        .insert( BlockBundle{
            tile:Tile {
                id: 2,
            },
            collider:Collider::cuboid(TILE_SIZE, TILE_SIZE),
            friction: Friction{ coefficient: NORMAL_FRICTION, combine_rule: CoefficientCombineRule::Multiply },
            transform: TransformBundle::from_transform(Transform {
                translation: Vec3::new(position.x,position.y - PIXELS_PER_METERS * TILE_SIZE / 4.0,position.z),
                scale: Vec3::new(PIXELS_PER_METERS/2.0,PIXELS_PER_METERS/4.0,1.0),
                ..default()
            }),
        })

        .insert(ActiveCollisionTypes::all())
        .insert(ActiveEvents::COLLISION_EVENTS)
    ;
}