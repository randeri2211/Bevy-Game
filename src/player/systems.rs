use std::time::Duration;
use crate::constants::*;
use crate::player::components::*;
use crate::skills::skills::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;


pub fn spawn_player(mut commands: Commands) {

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
        .insert(Collider::capsule_y(TILE_SIZE / 2.0, TILE_SIZE / 2.0))
        .insert(TransformBundle::from_transform(Transform {
            translation: Vec3::new(0.0, TILE_SIZE * PIXELS_PER_METERS, 0.0),
            scale: Vec3::new(PIXELS_PER_METERS, PIXELS_PER_METERS, 1.0),
            ..default()
        }))
        .insert(Restitution::coefficient(0.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0 , 0.0),
            angvel: 0.0,
        })
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        // .insert(GravityScale{ 0: 0.0 })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Player { ..default() })
        .insert(Skills { skills_vec: player_skills });
}

pub fn player_input(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &mut Player), With<Player>>, time: Res<Time>,) {
  
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

    if !x_pressed && velocity.linvel.x.abs() <= PLAYER_MAX_SPEED {
        //if not trying to move in the linear x direction
        velocity.linvel.x = 0.0;
    }
}

pub fn ability_system(buttons: Res<Input<MouseButton>>, commands: Commands, time: Res<Time>, mut q_windows: Query<&Window, With<PrimaryWindow>>, mut query: Query<&Transform, With<Player>>, mut s_query: Query<&mut Skills, With<Player>>,c_query: Query<(&Camera, &GlobalTransform)>) {

    let window = q_windows.get_single_mut().unwrap();

    let transform = query.get_single_mut().unwrap();

    let mut skills = s_query.get_single_mut().unwrap();

    let (camera, camera_transform) = c_query.single();
    // println!("{}",transform.translation);

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
                    if let Some(mut mouse_position) = window.cursor_position().and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                        .map(|ray| ray.origin.truncate()) {
                        println!("shooting");
                        (skill.shoot)(commands, transform.translation, mouse_position, skill);
                        break;
                    }
                }
            }
        }
    }
}

pub fn swap_ability(buttons: Res<Input<MouseButton>>, mut s_query: Query<&mut Skills, With<Player>>,) {
    if buttons.just_pressed(MouseButton::Right) {
        println!("swapped");
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
