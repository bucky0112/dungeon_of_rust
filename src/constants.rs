// Game constants extracted from main.rs

// Grid and world constants
pub const TILE_SIZE: f32 = 64.0;
pub const GRID_SIZE: i32 = 20;

// Player constants
pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_MAX_HEALTH: i32 = 100;
pub const PLAYER_SCALE: f32 = 3.0;

// Camera constants
pub const CAMERA_FOLLOW_SPEED: f32 = 3.0;

// Input constants
pub const INPUT_DEADZONE: f32 = 0.1;

// Grid visual constants
pub const GRID_DARK_COLOR: (f32, f32, f32) = (0.3, 0.3, 0.3);
pub const GRID_LIGHT_COLOR: (f32, f32, f32) = (0.4, 0.4, 0.4);
pub const CENTER_MARKER_COLOR: (f32, f32, f32) = (0.8, 0.2, 0.2);
pub const CORNER_MARKER_COLOR: (f32, f32, f32) = (0.2, 0.8, 0.2);
pub const CORNER_MARKER_SCALE: f32 = 0.5;

// Z-layer constants for proper rendering order
pub const Z_LAYER_GRID: f32 = -1.0;
pub const Z_LAYER_MARKERS: f32 = -0.5;
pub const Z_LAYER_PLAYER: f32 = 0.0;