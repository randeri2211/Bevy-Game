use bevy_simple_tilemap::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use crate::AppState;
use crate::constants::*;
use crate::game::entities::components::ManaBar;
use crate::game::map::components::*;
use crate::game::skills::skill_proj::*;
use crate::settings::components::Settings;


pub fn spawn_camera(mut commands: Commands,
                    settings: Res<Settings>) {

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(settings.zoom)),
        ..default()
    });
}


pub fn block_collisions_handler(
    mut collision_events: EventReader<CollisionEvent>,
    tile_query: Query<(Entity,&Transform),With<MyTile>>,
    mut commands: Commands,
    mut map_query: Query<&mut TileMap>,
) {
    let mut tilemap = map_query.get_single_mut().unwrap();
    let mut hit_this_frame = Vec::new();
    // Iterate through all collision events
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1,entity2,CollisionEventFlags::SENSOR) =>
            {
                for (tile,transform) in tile_query.iter(){
                    // Order of entities is not obsolete, check if one of the entities is a tile
                    if tile == *entity1{
                        if !hit_this_frame.contains(entity2) {
                            // tilemap.set_tile(position, Some(Tile { sprite_index: 0, color: Color::WHITE ,..default()}));
                            tilemap.set_tile(Vec3::new(transform.translation.x / TILE_SIZE / PIXELS_PER_METERS,transform.translation.y  / TILE_SIZE / PIXELS_PER_METERS,transform.translation.z).as_ivec3(),None);

                            commands.get_entity(*entity2).unwrap().despawn();
                            commands.get_entity(*entity1).unwrap().remove::<Mesh2dHandle>().remove::<Collider>();

                            hit_this_frame.insert(0,*entity2);
                        }
                        break;
                    }
                    else if tile == *entity2{
                        if !hit_this_frame.contains(entity1){
                            tilemap.set_tile(Vec3::new(transform.translation.x  / TILE_SIZE / PIXELS_PER_METERS,transform.translation.y  / TILE_SIZE / PIXELS_PER_METERS,transform.translation.z).as_ivec3(),None);

                            commands.get_entity(*entity1).unwrap().despawn();
                            commands.get_entity(*entity2).unwrap().remove::<Mesh2dHandle>().remove::<Collider>();
                            hit_this_frame.insert(0,*entity1);
                        }
                        break;
                    }
                }
            }
            ,
            CollisionEvent::Stopped(_, _, _) => {},
            _ => {}
        };
    }
}

pub fn entity_collisions_handler(
    mut collision_events: EventReader<CollisionEvent>,
    skill_query: Query<(&SkillProjBase,Entity)>,
) {
    fn check_entity(entity1:Entity,entity2:Entity,entity:Entity,skill:&SkillProjBase){
        // one of the collision entities is the projectile entity
        if entity1 == entity {
            // the other collision entity isn't the target entity
            if skill.id != entity2.index() || skill.generation != entity2.generation() {
                // TODO: add entity hit behaviour
                // println!("Collided with enemy!");
            }
        }
    }


    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, CollisionEventFlags::SENSOR) =>
                {
                    for (skill,entity) in skill_query.iter(){
                        check_entity(*entity1,*entity2,entity,skill);
                        check_entity(*entity2,*entity1,entity,skill);
                    }
                }
            ,
            CollisionEvent::Stopped(_, _, _) => {},
            _ => {}
        }
    }
}


pub fn toggle_app_state(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    entity_query: Query<Entity,With<Collider>>,
    map_query: Query<Entity,With<Map>>,
){
    if keys.just_pressed(KeyCode::G){
        if *app_state.get() == AppState::Game{
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            // Despawn everything on the map
            for entity in entity_query.iter(){
                commands.get_entity(entity).unwrap().despawn();
            }
            commands.get_entity(map_query.get_single().unwrap()).unwrap().despawn();

            println!("in mainmenu");
        }
        else if *app_state.get() == AppState::MainMenu{
            commands.insert_resource(NextState(Some(AppState::Game)));
            println!("in game");
        }
    }
}

pub fn mana_regen(mut mages:Query<&mut ManaBar>,time:Res<Time>){
    for mut mage in mages.iter_mut(){
        // if mage.current_mana < mage.max_mana{
        //     println!("{}:{}",mage.current_mana,mage.max_mana);
        // }
        mage.current_mana = f32::min(mage.current_mana + mage.mana_regen*time.delta_seconds(),mage.max_mana);
    }
}

