use std::time::Duration;
use crate::constants::*;
use crate::game::player::components::*;
use crate::game::skills::skills::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;




pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player), With<Player>>,
    time: Res<Time>,
) {
  
    let (mut velocity, mut player) = query.get_single_mut().unwrap().into();
    let mut x_pressed = false;

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
    }

    if !x_pressed && velocity.linvel.x.abs() <= PLAYER_MAX_SPEED / 2.0 {
        //if not trying to move in the linear x direction
        velocity.linvel.x = 0.0;
    }
}

pub fn ability_system(
    buttons: Res<Input<MouseButton>>,
    commands: Commands,
    time: Res<Time>,
    mut q_windows: Query<&Window, With<PrimaryWindow>>,
    mut p_query: Query<(&Transform, &mut Skills, Entity), With<Player>>,
    // mut s_query: Query<&mut Skills, With<Player>>,
    c_query: Query<(&Camera, &GlobalTransform)>
) {

    let window = q_windows.get_single_mut().unwrap();

    let (transform,mut skills,entity) = p_query.get_single_mut().unwrap();

    // let mut skills = s_query.get_single_mut().unwrap();

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
