use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use crate::player::components::*;

pub fn spawn_camera(mut commands: Commands) {

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0,1.0,1.0)),
        ..default()
    });
}

pub fn cam_follow_player(
    mut c_query: Query<&mut Transform,With<Camera>>,
    p_query: Query<&Transform,(Without<Camera>,With<Player>)>
){
    let mut c_transform = c_query.get_single_mut().unwrap();
    let p_transform = p_query.get_single().unwrap();

    c_transform.translation = p_transform.translation;
}

pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    p_query: Query<Entity,&Player>,
    mut commands: Commands
) {
    let entity = p_query.get_single().unwrap();

    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1,entity2,CollisionEventFlags::SENSOR) => if *entity1 != entity {
                commands.get_entity(*entity1).unwrap().despawn();
                commands.get_entity(*entity2).unwrap().despawn();
            },
            CollisionEvent::Stopped(_, _, _) => {},
            _ => {}
        };
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {

        println!("Received contact force event: {:?}", contact_force_event);
    }
}

// pub fn display_intersection_info(rapier_context: Res<RapierContext>) {
//     let entity = ...; // Entity with a collider attached.
//
//     /* Iterate through all the intersection pairs involving a specific collider. */
//     for (collider1, collider2, intersecting) in rapier_context.intersections_with(entity) {
//         if intersecting {
//             println!("The entities {:?} and {:?} have intersecting colliders!", collider1, collider2);
//         }
//     }
// }