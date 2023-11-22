use std::time::Duration;
use crate::constants::*;
use crate::game::player::components::*;
use crate::game::skills::skills::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use crate::game::map::components::*;


pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player,&mut Transform, &Collider,Entity), With<Player>>,
    time: Res<Time>,
    mut collision_events: EventReader<CollisionEvent>,
    tile_collider_query: Query<(&Transform,&Collider,Entity),(With<Tile>,Without<Player>)>,
) {
    // println!("before unwrap");

    let (mut velocity, mut player, mut player_transform, player_collider,player_entity) = query.get_single_mut().unwrap().into();
    // println!("after unwrap");
    let mut x_pressed = false;

    fn check_auto_step(
        player_transform:&mut Transform,
        player_collider:&Collider,
        player_entity:Entity,
        collision_events: &mut EventReader<CollisionEvent>,
        tile_collider_query: &Query<(&Transform,&Collider,Entity),(With<Tile>,Without<Player>)>,
    ){
        // Check for auto-step
        let mut collisions_amm = 0;
        let old_transform = player_transform.translation.clone();

        for collision in collision_events.read(){
            match collision {
                CollisionEvent::Started(entity1, entity2, _) =>
                    {
                        if collisions_amm == 2{
                            println!("broke");
                            break;
                        }
                        let offset:f32 = 1.0;
                        // Player is one of the colliders
                        if *entity1 == player_entity{
                            for (collider_transform,collider,entity) in tile_collider_query.iter(){
                                if *entity2 == entity{
                                    // if player_transform.translation.y = middle of the capsule_y
                                    let tile_collider = collider.as_cuboid().unwrap();
                                    let p_collider = player_collider.as_capsule().unwrap();
                                    // Center of player is higher than top of the block,and bottom of player is lower than top of block
                                    if player_transform.translation.y >= tile_collider.half_extents().y + collider_transform.translation.y - offset&&
                                        player_transform.translation.y - p_collider.half_height() - p_collider.radius() <= tile_collider.half_extents().y + collider_transform.translation.y - offset
                                    {
                                        collisions_amm += 1;
                                        player_transform.translation = Vec3::new(player_transform.translation.x,
                                                                                 tile_collider.half_extents().y + collider_transform.translation.y + p_collider.half_height() + p_collider.radius(),
                                                                                 player_transform.translation.z
                                        );
                                    }
                                }
                            }
                        }
                        else if *entity2 == player_entity{
                            for (collider_transform,collider,entity) in tile_collider_query.iter(){
                                if *entity1 == entity{
                                    let tile_collider = collider.as_cuboid().unwrap();
                                    let p_collider = player_collider.as_capsule().unwrap();
                                    if player_transform.translation.y >= tile_collider.half_extents().y + collider_transform.translation.y - offset&&
                                        player_transform.translation.y - p_collider.half_height() - p_collider.radius() <= tile_collider.half_extents().y + collider_transform.translation.y - offset
                                    {
                                        collisions_amm += 1;
                                        player_transform.translation = Vec3::new(player_transform.translation.x,
                                                                                 tile_collider.half_extents().y + collider_transform.translation.y + p_collider.half_height() + p_collider.radius(),
                                                                                 player_transform.translation.z
                                        );
                                    }
                                }
                            }
                        }

                    }
                ,
                CollisionEvent::Stopped(_, _, _) => {},
                _ => {}
            }
        }
        // println!("amm:{}",collisions_amm);
        if collisions_amm > 1{
            player_transform.translation = old_transform;
        }
        if collisions_amm > 0 {
            println!("amm:{}",collisions_amm);
        }
    }


    if keys.pressed(KeyCode::Space) {
        // W is being held down
        if player.last_speed < 0.0 && velocity.linvel.y >= 0.0 {
            // if the player just touched the ground or is currently touching it
            velocity.linvel.y = PLAYER_JUMP;
        }
        player.last_speed = velocity.linvel.y
    }

    if keys.pressed(KeyCode::A) {
        // A is being held down
        x_pressed = true;

        if velocity.linvel.x > -PLAYER_MAX_SPEED {
            velocity.linvel.x -= PLAYER_ACCELERATION * time.delta_seconds();
            if velocity.linvel.x < -PLAYER_MAX_SPEED {
                velocity.linvel.x = -PLAYER_MAX_SPEED;
            }
        }
        check_auto_step(&mut player_transform, &player_collider, player_entity, &mut collision_events, &tile_collider_query);

    }

    if keys.pressed(KeyCode::D) {
        // D is being held down
        x_pressed = true;

        if velocity.linvel.x < PLAYER_MAX_SPEED {
            velocity.linvel.x += PLAYER_ACCELERATION * time.delta_seconds();
            if velocity.linvel.x > PLAYER_MAX_SPEED {
                velocity.linvel.x = PLAYER_MAX_SPEED;
            }
        }

        check_auto_step(&mut player_transform, &player_collider, player_entity, &mut collision_events, &tile_collider_query);
    }

    // Stop movements caused by dynamic rigid body
    if !x_pressed && velocity.linvel.x.abs() <= PLAYER_MAX_SPEED / 2.0 {
        // if not trying to move in the linear x direction
        velocity.linvel.x = 0.0;
    }
}

pub fn ability_system(
    buttons: Res<Input<MouseButton>>,
    commands: Commands,
    time: Res<Time>,
    mut q_windows: Query<&Window, With<PrimaryWindow>>,
    mut p_query: Query<(&Transform, &mut Skills, Entity), With<Player>>,
    c_query: Query<(&Camera, &GlobalTransform)>
) {

    let window = q_windows.get_single_mut().unwrap();

    let (transform,mut skills,entity) = p_query.get_single_mut().unwrap();


    let (camera, camera_transform) = c_query.single();

    // a whole loop just to tick all the skills
    for skill in skills.skills_vec.iter_mut() {
        skill.cd.tick(Duration::from_secs_f32(time.delta_seconds()));
    }

    if buttons.pressed(MouseButton::Left) {
        // a separate loop for procing the active skill
        for skill in skills.skills_vec.iter_mut() {
            if skill.active {
                if skill.cd.finished() {
                    skill.cd.reset();
                    if let Some(mouse_position) = window.cursor_position().and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                        .map(|ray| ray.origin.truncate()) {
                        // println!("shooter id:{},{}",entity.index(),entity.generation());
                        (skill.shoot)(commands, transform.translation, mouse_position, skill,entity.index(),entity.generation());
                        break;
                    }
                }
            }
        }
    }
}

pub fn swap_ability(buttons: Res<Input<MouseButton>>, mut s_query: Query<&mut Skills, With<Player>>) {
    if buttons.just_pressed(MouseButton::Right) {

        let mut found = false;
        let mut finished = false;

        let mut skills = s_query.get_single_mut().unwrap();

        //goes through all the skillBases in the skills vector
        for skill in skills.skills_vec.iter_mut() {
            if found{
                skill.active = true;
                finished = true;
                break;
            }
            if skill.active{
                skill.active = false;
                found = true;
            }
        }
        if !finished{
            skills.skills_vec.get_mut(0).unwrap().active = true;
        }
    }
}
