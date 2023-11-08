use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use crate::player::components::*;
use crate::player::skills::*;
use crate::constants::*;

pub const PLAYER_RADIUS:f32 = 0.5;
pub const PLAYER_JUMP:f32 = 100.0;
pub const PLAYER_MAX_SPEED:f32 = 100.0;
pub const PLAYER_ACCELERATION:f32 = 100.0;
pub const PLAYER_MASS:f32 = 100.0;

pub fn spawn_player(mut commands: Commands,
                    window_query: Query<&Window,With<PrimaryWindow>>){

    let window = window_query.get_single().unwrap();

    // Ground
    commands
        .spawn(Collider::cuboid(5.0, 1.0))
        .insert(TransformBundle::from_transform(
            Transform
            {
                translation:Vec3::new(window.width() / 2.0, 0.0, 0.0),
                scale:Vec3::new(PIXELS_PER_METERS,PIXELS_PER_METERS,1.0),
                ..default()
            }
        ))
    ;

    // Player
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(PLAYER_RADIUS))
        .insert(TransformBundle::from_transform(
            Transform
            {
                translation:Vec3::new(window.width()/2.0,window.height()/2.0,0.0),
                scale:Vec3::new(PIXELS_PER_METERS,PIXELS_PER_METERS,1.0),
                ..default()
            }
        ))
        .insert(Restitution::coefficient(0.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0*PIXELS_PER_METERS, 0.0),
            angvel: 0.0,
        })
        // .insert(ColliderMassProperties::Mass(PLAYER_MASS))
        .insert(Friction{
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Player{..default()})
        .insert(Skill{..default()})
    ;
}


pub fn player_input(keys: Res<Input<KeyCode>>,
                    mut query:Query<(&mut Velocity,&mut Player,&mut Skill)>,
                    time: Res<Time>
){

    let (mut velocity,mut player, mut skill) = query.get_single_mut().unwrap().into();
    skill.cd.tick(time.delta()); // tick active skill cd
    let mut x_pressed = false;

    if keys.pressed(KeyCode::Space) {
        // W is being held down
        if player.last_speed < 0.0 && velocity.linvel.y >= 0.0{
            // if the player just touched the ground or is currently touching it
            velocity.linvel.y = PLAYER_JUMP;
        }
        player.last_speed = velocity.linvel.y
    }

    if keys.pressed(KeyCode::A) {
        // A is being held down
        x_pressed = true;

        if velocity.linvel.x > -PLAYER_MAX_SPEED{
            velocity.linvel.x -= PLAYER_ACCELERATION * time.delta_seconds();
            if velocity.linvel.x < -PLAYER_MAX_SPEED{
                velocity.linvel.x = -PLAYER_MAX_SPEED;
            }
        }
    }


    if keys.pressed(KeyCode::D) {
        // D is being held down
        x_pressed = true;

        if velocity.linvel.x < PLAYER_MAX_SPEED{
            velocity.linvel.x += PLAYER_ACCELERATION * time.delta_seconds();
            if velocity.linvel.x > PLAYER_MAX_SPEED{
                velocity.linvel.x = PLAYER_MAX_SPEED;
            }
        }
    }

    if !x_pressed && velocity.linvel.x.abs() <= PLAYER_MAX_SPEED{
        //if not trying to move in the linear x direction
        velocity.linvel.x = 0.0;
    }
}

pub fn ability_system(buttons: Res<Input<MouseButton>>,
                      mut q_windows: Query<&Window, With<PrimaryWindow>>,
                      mut query:Query<(&Transform,&mut Skill)>,
                      mut commands: Commands
){
    let (transform, mut skill) = query.get_single_mut().unwrap().into();
    if buttons.pressed(MouseButton::Left) {
        // Left mouse button pressed
        if let Some(mut mouse_position) = q_windows.single().cursor_position() {
            //found cursor position
            let window = q_windows.get_single_mut().unwrap();
            mouse_position.y = window.height() - mouse_position.y;
            if skill.cd.finished() {
                skill.cd.reset(); // reset cd timer
                commands.spawn(Collider::ball(skill.width))
                    .insert(Sensor)
                    .insert(RigidBody::Dynamic)
                    .insert(GravityScale(0.0))
                    .insert(Friction::coefficient(0.0))
                    .insert(SkillProj::initiate(transform.translation,mouse_position,skill.speed))
                ;
            }
        }
    }
    if buttons.pressed(MouseButton::Right) {
        // Right mouse button pressed

    }
}
