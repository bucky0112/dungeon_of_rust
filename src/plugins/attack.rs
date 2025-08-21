use bevy::prelude::*;
use crate::systems::attack::{attack_input_system, handle_attack_events, update_melee_attacks, update_attack_animations};
use crate::components::attack::AttackEvent;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AttackEvent>()
            .add_systems(Update, (
                attack_input_system,
                handle_attack_events,
                update_melee_attacks,
                update_attack_animations,
            ).chain()); // chain() 確保系統按順序執行
    }
}