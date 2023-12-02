
// Global constants
pub const PIXELS_PER_METERS: f32 = 16.0;
pub const TILE_SIZE: f32 = 1.0; //Meters
pub const TILES: i32 = 4;
pub const NORMAL_FRICTION: f32 = 0.2;
pub const ZOOM: f32 = 0.5;

// Skill constants
pub const SKILL_CD: f32 = 1.0;
pub const SKILL_SPEED: f32 = 2.0;

// Player constants
pub const PLAYER_RADIUS: f32 = 0.5;
pub const PLAYER_JUMP: f32 = PIXELS_PER_METERS * TILE_SIZE * 4.0; // Jump at 3 m/s speed
pub const PLAYER_MAX_SPEED: f32 = PIXELS_PER_METERS * 2.0; // Speed of 1 m/s (2 blocks a second)
pub const PLAYER_ACCELERATION: f32 = PIXELS_PER_METERS * 4.0; // Speeds up at 2 m/s^2
pub const PLAYER_MASS: f32 = 1.0;
pub const PLAYER_RIGHT_ATLAS_INDEX: usize = 1;
pub const PLAYER_LEFT_ATLAS_INDEX: usize = 0;
