use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use crate::AppState;
use crate::game::map::components::*;
use crate::game::skills::skill_proj::*;


pub fn spawn_camera(mut commands: Commands) {

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0,1.0,1.0)),
        ..default()
    });
}


pub fn block_collisions_handler(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    tile_query: Query<Entity,With<Tile>>,
    mut commands: Commands
) {
    let mut hit_this_frame = Vec::new();
    // Iterate through all collision events
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1,entity2,CollisionEventFlags::SENSOR) =>
            {
                for tile in tile_query.iter(){
                    // Order of entities is not obsolete, check if one of the entities is a tile
                    if tile == *entity1{
                        if !hit_this_frame.contains(entity2) {
                            println!("normal");
                            commands.get_entity(*entity2).unwrap().despawn();
                            commands.get_entity(*entity1).unwrap().remove::<Mesh2dHandle>().remove::<Collider>();
                            hit_this_frame.insert(0,*entity2);
                        }
                        break;
                    }
                    if tile == *entity2{
                        if !hit_this_frame.contains(entity1){
                            println!("reversed");
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

    //mainly for debug purposes
    //TODO:remove
    for contact_force_event in contact_force_events.read() {

        println!("Received contact force event: {:?}", contact_force_event);
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
        println!("{:?}",collision_event);
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
){
    if keys.just_pressed(KeyCode::Escape){
        if *app_state.get() == AppState::Game{
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            for entity in entity_query.iter(){
                commands.get_entity(entity).unwrap().despawn();
            }
            println!("in mainmenu");
        }
    }

    if keys.just_pressed(KeyCode::Escape){
        if *app_state.get() == AppState::MainMenu{
            commands.insert_resource(NextState(Some(AppState::Game)));
            println!("in game");
        }
    }
}