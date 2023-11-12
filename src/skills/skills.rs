use bevy::prelude::*;
use std::time::Duration;
use bevy::math::*;
use bevy_rapier2d::prelude::*;
use crate::constants::*;
use crate::skills::skill_proj::*;

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
    pub(crate) shoot:fn(Commands, Vec3, Vec2, &SkillBase) ->(),
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
        .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
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
        .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(SkillProj::initiate(new_mouse_position, new_player_position, skill.speed))
    ;
}
