use bevy::prelude::*;
use crate::systems::{
    setup::{load_sprite_handles, spawn_player},
    movement::{movement_system, sprite_direction_system},
    health::health_system,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (load_sprite_handles, spawn_player).chain())
            .add_systems(Update, (movement_system, sprite_direction_system, health_system));
    }
}