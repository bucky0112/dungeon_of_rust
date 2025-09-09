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

/// 房間切換系統 - 當玩家通過開啟的門時傳送到新房間
pub fn room_transition_system(
    door_query: Query<(&Door, &Transform), Without<Player>>,
    mut player_query: Query<(&mut Transform, &InputVector), With<Player>>,
    mut transition_cooldown: ResMut<TransitionCooldown>,
    time: Res<Time>,
) {
    // 更新冷卻計時器
    transition_cooldown.timer.tick(time.delta());
    
    let (mut player_transform, input_vector) = match player_query.single_mut() {
        Ok(result) => result,
        Err(_) => return,
    };
    
    let tile_size = ROOM_TILE_SIZE * PLAYER_SCALE;
    let transition_threshold = tile_size * 0.8; // 80% 的瓷磚尺寸作為傳送觸發距離
    
    for (door, door_transform) in &door_query {
        let door_pos = door_transform.translation.truncate();
        let player_pos = player_transform.translation.truncate();
        let distance = door_pos.distance(player_pos);
        
        // 添加調試信息
        static mut DEBUG_COUNTER: u32 = 0;
        unsafe {
            DEBUG_COUNTER += 1;
            if DEBUG_COUNTER % 300 == 0 { // 每5秒檢查一次
                info!("🚪 門狀態: {} (位置: {:?}), 玩家位置: {:?}, 距離: {:.1}", 
                      if door.is_open { "開啟" } else { "關閉" }, 
                      door_pos, player_pos, distance);
            }
        }
        
        // 只檢查開啟的門
        if door.is_open {
            // 如果玩家靠近開啟的門且還在冷卻中，跳過
            if distance < transition_threshold {
                if !transition_cooldown.timer.finished() {
                    continue;
                }
                
                info!("玩家位置: {:?}, 門位置: {:?}, 距離: {:.1}, 輸入向量: {:?}", player_pos, door_pos, distance, input_vector.0);
                
                // 判斷玩家移動方向與門的關係
                let door_to_player = player_pos - door_pos;
                let is_moving_towards_door = input_vector.0.length() > 0.1;
                
                if is_moving_towards_door {
                    // 根據玩家當前位置和移動方向決定傳送
                    if door_to_player.y < 0.0 && input_vector.0.y > 0.0 {
                        // 玩家在門的下方且向上移動 - 進入房間
                        let new_position = door_pos + Vec2::new(0.0, tile_size * 1.5);
                        player_transform.translation.x = new_position.x;
                        player_transform.translation.y = new_position.y;
                        
                        transition_cooldown.timer.reset();
                        info!("玩家進入房間！傳送到: {:?}", new_position);
                    } else if door_to_player.y > 0.0 && input_vector.0.y < 0.0 {
                        // 玩家在門的上方且向下移動 - 離開房間
                        let new_position = door_pos + Vec2::new(0.0, -tile_size * 1.5);
                        player_transform.translation.x = new_position.x;
                        player_transform.translation.y = new_position.y;
                        
                        transition_cooldown.timer.reset();
                        info!("玩家離開房間！傳送到: {:?}", new_position);
                    }
                }
            }
        }
    }
}