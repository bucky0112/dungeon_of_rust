use bevy::prelude::*;
use crate::systems::input::input_system;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input_system);
    }
}