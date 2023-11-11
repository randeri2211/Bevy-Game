use crate::constants::*;
use crate::player::components::*;
use crate::player::skills::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub const PLAYER_RADIUS: f32 = 0.5;
pub const PLAYER_JUMP: f32 = 100.0;
pub const PLAYER_MAX_SPEED: f32 = 100.0;
pub const PLAYER_ACCELERATION: f32 = 100.0;
pub const PLAYER_MASS: f32 = 100.0;

pub fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    // Ground
    commands
        .spawn(Collider::cuboid(5.0, 1.0))
        .insert(TransformBundle::from_transform(Transform {
            translation: Vec3::new(window.width() / 2.0, 0.0, 0.0),
            scale: Vec3::new(PIXELS_PER_METERS, PIXELS_PER_METERS, 1.0),
            ..default()
        }));

    let mut player_skills:Vec<SkillBase> = Vec::new();
    player_skills.insert(0, SkillBase::default());

    let mut another_skill = SkillBase::default();
    another_skill.shoot = reverse;
    another_skill.active = false;
    player_skills.insert(0, another_skill);

    // Player
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(PLAYER_RADIUS))
        .insert(TransformBundle::from_transform(Transform {
            translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            scale: Vec3::new(PIXELS_PER_METERS, PIXELS_PER_METERS, 1.0),
            ..default()
        }))
        .insert(Restitution::coefficient(0.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0 * PIXELS_PER_METERS, 0.0),
            angvel: 0.0,
        })
        // .insert(ColliderMassProperties::Mass(PLAYER_MASS))
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Player { ..default() })
        .insert(Skills { skills_vec: player_skills });
}

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

    if !x_pressed && velocity.linvel.x.abs() <= PLAYER_MAX_SPEED {
        //if not trying to move in the linear x direction
        velocity.linvel.x = 0.0;
    }
}

pub fn ability_system(
    buttons: Res<Input<MouseButton>>,
    mut q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&Transform, With<Player>>,
    commands: Commands,
    time: Res<Time>,
    mut s_query: Query<&mut Skills, With<Player>>,
) {
    let window = q_windows.get_single_mut().unwrap();

    let transform = query.get_single_mut().unwrap();

    let mut skills = s_query.get_single_mut().unwrap();

    if buttons.pressed(MouseButton::Left) {
        // a whole loop just to tick all the skills
        for skill in skills.skills_vec.iter_mut() {
            skill.cd.tick(Duration::from_secs_f32(time.delta_seconds()));
        }

        // a separate loop for procing the active skill
        for skill in skills.skills_vec.iter_mut() {
            if skill.active {
                if skill.cd.finished() {
                    skill.cd.reset();
                    if let Some(mut mouse_position) = window.cursor_position() {
                        println!("shooting");
                        mouse_position.y = window.height() - mouse_position.y;
                        (skill.shoot)(commands, transform.translation, mouse_position, skill);
                        break;
                    }
                }
            }
        }
    }
}

pub fn swap_ability(
    buttons: Res<Input<MouseButton>>,
    mut s_query: Query<&mut Skills, With<Player>>,
) {
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
