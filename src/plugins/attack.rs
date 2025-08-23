use bevy::prelude::*;
use crate::systems::attack::{
    attack_input_system, handle_attack_events, update_melee_attacks, 
    update_attack_animations, update_weapon_effects, update_projectiles, 
    weapon_switching_system, combat_state_system, timing_attack_system
};
use crate::components::attack::{AttackEvent, CombatState};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<CombatState>() // 新增戰鬥狀態
            .add_event::<AttackEvent>()
            .add_systems(Update, (
                weapon_switching_system,
                combat_state_system,     // 新增戰鬥狀態管理
                timing_attack_system,    // 新增時機攻擊系統
                attack_input_system,
                handle_attack_events,
                update_melee_attacks,
                update_attack_animations,
                update_weapon_effects,
                update_projectiles,
            ).chain()); // chain() 確保系統按順序執行
    }
}