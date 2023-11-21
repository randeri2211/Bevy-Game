use bevy::math::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::constants::*;

#[derive(Component)]
pub struct SkillProjBase {
    pub(crate) id:u32,
    pub(crate) generation:u32,
}

#[derive(Component)]
pub struct SkillProj {
}

impl SkillProj{
    pub fn initiate(player_position: Vec3, mouse_position: Vec2,speed:f32,entity_id:u32,entity_gen:u32) -> (SkillProj,SkillProjBase, TransformBundle,Velocity) {
        // Mouse position already fixed for the y orientation
        (
            SkillProj {},
            SkillProjBase{
                id: entity_id,
                generation:entity_gen
            },
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
