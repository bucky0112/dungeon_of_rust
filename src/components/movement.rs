use bevy::prelude::*;
use crate::constants::INPUT_DEADZONE;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn get_sprite_path(&self) -> &str {
        match self {
            Direction::North => "characters/knight_hero/rotations/north.png",
            Direction::NorthEast => "characters/knight_hero/rotations/north-east.png",
            Direction::East => "characters/knight_hero/rotations/east.png",
            Direction::SouthEast => "characters/knight_hero/rotations/south-east.png",
            Direction::South => "characters/knight_hero/rotations/south.png",
            Direction::SouthWest => "characters/knight_hero/rotations/south-west.png",
            Direction::West => "characters/knight_hero/rotations/west.png",
            Direction::NorthWest => "characters/knight_hero/rotations/north-west.png",
        }
    }
    
    pub fn from_input_vector(input: Vec2) -> Option<Self> {
        // deadzone 檢查在正規化之前，避免微小輸入被放大
        if input.length() < INPUT_DEADZONE {
            return None;
        }
        
        let angle = input.y.atan2(input.x).to_degrees();
        let normalized_angle = if angle < 0.0 { angle + 360.0 } else { angle };
        
        let direction = match normalized_angle as i32 {
            0..=22 | 338..=360 => Direction::East,
            23..=67 => Direction::NorthEast,
            68..=112 => Direction::North,
            113..=157 => Direction::NorthWest,
            158..=202 => Direction::West,
            203..=247 => Direction::SouthWest,
            248..=292 => Direction::South,
            293..=337 => Direction::SouthEast,
            _ => Direction::South,
        };
        
        Some(direction)
    }
}