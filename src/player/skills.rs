use bevy::prelude::*;
use std::time::Duration;
use bevy_rapier2d::prelude::Velocity;
use crate::constants::*;

pub const SKILL_CD:f32 = 1.0;
pub const SKILL_SPEED:f32 = 2.0;

#[derive(Component)]
pub struct Skill{
    pub(crate) lvl:i8,
    pub(crate) exp:f32,
    pub(crate) width:f32,
    pub(crate) cd:Timer,
    pub(crate) speed:f32,
}

impl Default for Skill{
    fn default() -> Self {
        Skill {
            lvl:1,
            exp:0.0,
            width:0.1,
            cd:Timer::new(Duration::from_secs_f32(SKILL_CD), TimerMode::Once),
            speed: SKILL_SPEED,
        }
    }
}

#[derive(Component)]
pub struct SkillProj {
    // pub(crate) transform:Transform,
    // pub(crate) velocity:Velocity,
}

// impl Default for SkillProj{
//     fn default() -> Self {
//         SkillProj {
//             // transform:Transform{..default()},
//             // velocity:Velocity{..default()},
//         }
//     }
// }

impl SkillProj{
    pub fn initiate(player_position: Vec3, mouse_position: Vec2,speed:f32) -> (SkillProj, TransformBundle,Velocity) {
        // Mouse position already fixed for the y orientation
        (SkillProj{

        },
         TransformBundle::from_transform(Transform{
             translation: player_position,
             scale:Vec3::new(PIXELS_PER_METERS,PIXELS_PER_METERS,0.0),
             ..default()
         }),
         Velocity {
             // Y position is reversed on mouse position(top is 0) from transform(bottom is 0)
             linvel: Vec2::new(mouse_position.x - player_position.x, mouse_position.y - player_position.y).normalize()*speed*PIXELS_PER_METERS,
             angvel: 0.0,
         }
        )
    }
}