use bevy::prelude::*;
use crate::systems::{
    setup::setup,
    camera::camera_follow_system,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, camera_follow_system);
    }
}