use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::world::{Door, RoomTile, RoomTileType};
use crate::systems::input::DoorInteractionEvent;
use crate::resources::RoomAssets;
use crate::constants::*;

/// 門交互系統 - 處理玩家與門的碰撞檢測和開關邏輯
pub fn door_interaction_system(
    mut door_query: Query<(Entity, &mut Door, &mut RoomTile, &Transform, &mut Sprite), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut door_events: EventReader<DoorInteractionEvent>,
    room_assets: Res<RoomAssets>,
) {
    let player_transform = match player_query.single() {
        Ok(transform) => transform,
        Err(_) => return, // 沒有玩家就退出
    };
    
    // 調試：檢查是否有門存在
    static mut DEBUG_COUNTER: u32 = 0;
    unsafe {
        DEBUG_COUNTER += 1;
        if DEBUG_COUNTER % 300 == 0 { // 每10秒檢查一次
            let door_count = door_query.iter().count();
            info!("門交互系統運行中，找到 {} 個門", door_count);
            for (entity, door, room_tile, door_transform, sprite) in door_query.iter() {
                info!("  門位置: {:?}, 開啟狀態: {}", door_transform.translation.truncate(), door.is_open);
            }
        }
    }

    // 只有在收到門交互事件時才處理
    for _event in door_events.read() {
        let mut closest_door = None;
        let mut closest_distance = f32::INFINITY;
        
        // 找到最近的門
        for (entity, door, room_tile, door_transform, sprite) in door_query.iter() {
            let distance = player_transform.translation.distance(door_transform.translation);
            let interaction_distance = ROOM_TILE_SIZE * PLAYER_SCALE * 10.0; // 10個瓷磚的距離，約480像素
            
            if distance <= interaction_distance && distance < closest_distance {
                closest_distance = distance;
                closest_door = Some((entity, door, room_tile, door_transform, sprite));
            }
        }
        
        // 如果找到最近的門，切換其狀態
        if let Some((entity, _door, _room_tile, _door_transform, _sprite)) = closest_door {
            // 通過entity重新獲取可變引用
            if let Ok((_, mut door, mut room_tile, _, mut sprite)) = door_query.get_mut(entity) {
                // 切換門的狀態
                door.is_open = !door.is_open;
                
                // 更新視覺效果
                if door.is_open {
                    room_tile.tile_type = RoomTileType::DoorOpen;
                    sprite.image = room_assets.door_open.clone();
                    info!("🚪 門已開啟！玩家現在可以通過");
                } else {
                    room_tile.tile_type = RoomTileType::DoorClosed;
                    sprite.image = room_assets.door_closed.clone();
                    info!("🚪 門已關閉！玩家無法通過");
                }
            }
        }
    }
}

/// 門碰撞檢測系統 - 阻止玩家穿過關閉的門
pub fn door_collision_system(
    door_query: Query<(&Door, &Transform), Without<Player>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = match player_query.single_mut() {
        Ok(transform) => transform,
        Err(_) => return,
    };
    
    let tile_size = ROOM_TILE_SIZE * PLAYER_SCALE;
    let collision_threshold = tile_size * 0.4; // 40% 的瓷磚尺寸作為碰撞檢測閾值
    
    for (door, door_transform) in &door_query {
        // 只檢查關閉的門
        if !door.is_open {
            let door_pos = door_transform.translation.truncate();
            let player_pos = player_transform.translation.truncate();
            let distance = door_pos.distance(player_pos);
            
            // 如果玩家太靠近關閉的門
            if distance < collision_threshold {
                // 計算推開玩家的方向
                let push_direction = (player_pos - door_pos).normalize_or_zero();
                
                // 將玩家推到安全距離
                let safe_distance = collision_threshold + 2.0;
                let new_position = door_pos + push_direction * safe_distance;
                
                player_transform.translation.x = new_position.x;
                player_transform.translation.y = new_position.y;
            }
        }
    }
}

/// 自動門關閉系統 - 玩家離開後自動關閉門
pub fn auto_close_door_system(
    mut door_query: Query<(Entity, &mut Door, &mut RoomTile, &Transform, &mut Sprite)>,
    player_query: Query<&Transform, With<Player>>,
    room_assets: Res<RoomAssets>,
    _time: Res<Time>,
) {
    let player_transform = match player_query.single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    for (_entity, mut door, mut room_tile, door_transform, mut sprite) in &mut door_query {
        // 只處理開啟的門
        if door.is_open {
            let distance = player_transform.translation.distance(door_transform.translation);
            let auto_close_distance = ROOM_TILE_SIZE * PLAYER_SCALE * 15.0; // 15個瓷磚距離自動關閉
            
            // 如果玩家離得夠遠，自動關閉門
            if distance > auto_close_distance {
                door.is_open = false;
                room_tile.tile_type = RoomTileType::DoorClosed;
                sprite.image = room_assets.door_closed.clone();
                info!("門自動關閉！");
            }
        }
    }
}