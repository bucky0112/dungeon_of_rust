use bevy::prelude::*;
use crate::systems::door_interaction::{door_interaction_system, auto_close_door_system, door_collision_system};
use crate::systems::input::DoorInteractionEvent;

pub struct DoorInteractionPlugin;

impl Plugin for DoorInteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DoorInteractionEvent>()
            .add_systems(Update, (
                door_interaction_system,
                door_collision_system,
                // auto_close_door_system, // 暫時禁用自動關門
            ));
    }
}