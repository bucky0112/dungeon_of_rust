use bevy::prelude::*;
use crate::systems::visual_combat::{
    player_input_system,
    player_movement_system,
    update_player_facing_system,
    update_weapon_offset_system,
    visual_attack_input_system,
    update_weapon_swing_animation_system,
    spawn_player_with_weapon_system,
};

pub struct VisualCombatPlugin;

impl Plugin for VisualCombatPlugin {
    fn build(&self, app: &mut App) {
        app
            // 在遊戲開始時生成玩家和武器
            .add_systems(Startup, spawn_player_with_weapon_system)
            // 更新系統
            .add_systems(Update, (
                player_input_system,
                player_movement_system,
                update_player_facing_system,
                update_weapon_offset_system,
                visual_attack_input_system,
                update_weapon_swing_animation_system,
            ).chain()); // 確保系統按順序執行
    }
}