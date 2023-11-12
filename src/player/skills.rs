use crate::constants::*;
use bevy::prelude::*;
use std::time::Duration;
use bevy::math::*;
use bevy_rapier2d::prelude::*;
use crate::constants::*;

pub const SKILL_CD: f32 = 1.0;
pub const SKILL_SPEED: f32 = 2.0;

// #[bevy_trait_query::queryable]
// pub trait Shootable{
//     fn shoot(&self, commands: Commands, player_position: Vec3, mouse_position: Vec2);
//     fn tick(&mut self,time:f32);
//     fn reset(&mut self);
//     fn finish(&self) -> bool;
// }

#[derive(Component)]

pub struct Skills{
    pub(crate) skills_vec:Vec<SkillBase>
}

pub struct SkillBase{
    pub(crate) lvl:i8,
    pub(crate) exp:f32,
    pub(crate) size:f32,
    pub(crate) cd:Timer,
    pub(crate) speed:f32,
    pub(crate) active:bool,
    pub(crate) key:char,
    pub(crate) shoot:fn(Commands,Vec3,Vec2,&SkillBase)->(),
}

impl Default for SkillBase{

    fn default() -> Self {
        SkillBase {lvl:1,
            exp:0.0,
            size:0.1,
            cd:Timer::new(Duration::from_secs_f32(SKILL_CD), TimerMode::Once),
            speed: SKILL_SPEED,
            active: true,
            key: 'r',
            shoot: default_shoot,
        }
    }
}

pub fn default_shoot(mut commands: Commands, player_position: Vec3, mouse_position: Vec2, skill:&SkillBase) {
    commands.spawn(Collider::ball(skill.size))
        .insert(Sensor)
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(Friction::coefficient(0.0))
        .insert(SkillProj::initiate(player_position, mouse_position, skill.speed))
    ;
}

pub fn reverse(mut commands: Commands, player_position: Vec3, mouse_position: Vec2,skill:&SkillBase) {

    let new_mouse_position = vec3(mouse_position.x,mouse_position.y,0.0);
    let new_player_position = vec2(player_position.x,player_position.y);

    commands.spawn(Collider::ball(skill.size))
        .insert(Sensor)
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(Friction::coefficient(0.0))
        .insert(SkillProj::initiate(new_mouse_position, new_player_position, skill.speed))
    ;
}


#[derive(Component)]
pub struct SkillProj {
}


impl SkillProj{
    pub fn initiate(player_position: Vec3, mouse_position: Vec2,speed:f32) -> (SkillProj, TransformBundle,Velocity) {
        // Mouse position already fixed for the y orientation
        (
            SkillProj {},
            TransformBundle::from_transform(Transform {
                translation: player_position,
                scale: Vec3::new(PIXELS_PER_METERS, PIXELS_PER_METERS, 0.0),
                ..default()
            }),
            Velocity {
                // Y position is reversed on mouse position(top is 0) from transform(bottom is 0)
                linvel: Vec2::new(
                    mouse_position.x - player_position.x,
                    mouse_position.y - player_position.y,
                )
                .normalize()
                    * speed
                    * PIXELS_PER_METERS,
                angvel: 0.0,
            },
        )
    }
}
