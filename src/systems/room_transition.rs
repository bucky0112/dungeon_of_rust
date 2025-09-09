use bevy::prelude::*;
use crate::components::player::{Player, InputVector};
use crate::components::world::{Door, RoomTile, RoomTileType};
use crate::constants::*;

#[derive(Resource)]
pub struct TransitionCooldown {
    pub timer: Timer,
}

impl Default for TransitionCooldown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

/// 房間切換系統 - 基於鍵盤輸入的簡單切換機制
pub fn room_transition_system(
    door_query: Query<(&Door, &Transform), Without<Player>>,
    mut player_query: Query<(&mut Transform, &InputVector), With<Player>>,
    mut transition_cooldown: ResMut<TransitionCooldown>,
    time: Res<Time>,
) {
    // 更新冷卻計時器
    transition_cooldown.timer.tick(time.delta());
    
    // 如果還在冷卻中，不執行傳送
    if !transition_cooldown.timer.finished() {
        return;
    }
    
    let (mut player_transform, input_vector) = match player_query.single_mut() {
        Ok(result) => result,
        Err(_) => return,
    };
    
    let tile_size = ROOM_TILE_SIZE * PLAYER_SCALE;
    let trigger_distance = tile_size * 3.0; // 3個瓷磚距離作為觸發範圍
    
    for (door, door_transform) in &door_query {
        let door_pos = door_transform.translation.truncate();
        let player_pos = player_transform.translation.truncate();
        let distance = door_pos.distance(player_pos);
        
        // 只有在玩家靠近門且門是開啟的情況下才檢查
        if door.is_open && distance < trigger_distance {
            let door_to_player = player_pos - door_pos;
            let player_movement = input_vector.0;
            
            // 檢查玩家是否在向門的方向移動
            let movement_threshold = 0.1;
            let moving_up = player_movement.y > movement_threshold;
            let moving_down = player_movement.y < -movement_threshold;
            
            // 簡化的切換邏輯：
            // 1. 如果玩家在門下方且向上移動 -> 傳送到門上方（進入房間）
            // 2. 如果玩家在門上方且向下移動 -> 傳送到門下方（離開房間）
            
            if door_to_player.y < -20.0 && moving_up {
                // 玩家在門下方，向上移動 - 進入房間
                let new_position = door_pos + Vec2::new(0.0, 80.0);
                player_transform.translation.x = new_position.x;
                player_transform.translation.y = new_position.y;
                
                transition_cooldown.timer.reset();
                info!("✅ 玩家進入房間！從 {:?} 傳送到 {:?}", player_pos, new_position);
                
            } else if door_to_player.y > 20.0 && moving_down {
                // 玩家在門上方，向下移動 - 離開房間
                let new_position = door_pos + Vec2::new(0.0, -80.0);
                player_transform.translation.x = new_position.x;
                player_transform.translation.y = new_position.y;
                
                transition_cooldown.timer.reset();
                info!("✅ 玩家離開房間！從 {:?} 傳送到 {:?}", player_pos, new_position);
            }
        }
    }
}