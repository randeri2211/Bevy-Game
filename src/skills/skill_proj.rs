use bevy::math::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::constants::*;

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
