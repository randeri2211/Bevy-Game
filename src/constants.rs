use bevy::math::f32;

// Global constants
pub const PIXELS_PER_METERS: f32 = 100.0;
pub const TILE_SIZE:f32 = 1.0/2.0; //Meters
pub const TILES:f32 = 4.0;
pub const NORMAL_FRICTION:f32 = 0.2;

// Skill constants
pub const SKILL_CD: f32 = 1.0;
pub const SKILL_SPEED: f32 = 2.0;

// Player constants
pub const PLAYER_RADIUS: f32 = 0.5;
pub const PLAYER_JUMP: f32 = PIXELS_PER_METERS * 2.0; // Jump at 2 m/s speed
pub const PLAYER_MAX_SPEED: f32 = PIXELS_PER_METERS; // Speed of 1 m/s (2 blocks a second)
pub const PLAYER_ACCELERATION: f32 = PIXELS_PER_METERS * 2.0; // Speeds up at 2 m/s^2
