use bevy::prelude::*;
use crate::systems::world::{spawn_grid, spawn_room};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_grid, spawn_room).chain());
    }
}