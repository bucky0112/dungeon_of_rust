use bevy::prelude::*;
use crate::systems::wall_collision::simple_wall_collision_system;

pub struct WallCollisionPlugin;

impl Plugin for WallCollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, simple_wall_collision_system);
    }
}