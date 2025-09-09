use bevy::prelude::*;
use crate::components::player::{Player, Velocity};
use crate::components::world::{RoomTile, RoomTileType, Door};
use crate::constants::*;
use std::collections::HashMap;

/// 牆壁碰撞檢測系統 - 阻止玩家穿牆
pub fn wall_collision_system(
    wall_query: Query<(Entity, &RoomTile, &Transform), Without<Player>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    door_query: Query<&Door>,
) {
    let mut player_transform = match player_query.single_mut() {
        Ok(transform) => transform,
        Err(_) => return,
    };
    
    let tile_size = ROOM_TILE_SIZE * PLAYER_SCALE;
    let collision_threshold = tile_size * 0.45; // 45% 的瓷磚尺寸作為碰撞檢測閾值
    
    for (entity, room_tile, wall_transform) in &wall_query {
        // 檢查是否為牆壁類型（排除地板）
        if is_wall_tile(&room_tile.tile_type) {
            // 對於門，需要檢查是否開啟
            if matches!(room_tile.tile_type, RoomTileType::DoorClosed | RoomTileType::DoorOpen) {
                // 檢查門的狀態
                if let Ok(door) = door_query.get(entity) {
                    if door.is_open {
                        continue; // 開啟的門不阻擋玩家
                    }
                }
            }
            
            let wall_pos = wall_transform.translation.truncate();
            let player_pos = player_transform.translation.truncate();
            let distance = wall_pos.distance(player_pos);
            
            // 如果玩家太靠近牆壁
            if distance < collision_threshold {
                // 計算推開玩家的方向
                let push_direction = (player_pos - wall_pos).normalize_or_zero();
                
                // 將玩家推到安全距離
                let safe_distance = collision_threshold + 1.0;
                let new_position = wall_pos + push_direction * safe_distance;
                
                player_transform.translation.x = new_position.x;
                player_transform.translation.y = new_position.y;
            }
        }
    }
}

/// 判斷是否為牆壁瓷磚類型
fn is_wall_tile(tile_type: &RoomTileType) -> bool {
    match tile_type {
        RoomTileType::Floor => false,
        RoomTileType::DoorOpen => false, // 開啟的門不阻擋（但我們會在上面單獨處理門的狀態）
        _ => true, // 所有其他類型都是牆壁
    }
}

/// 簡化的牆壁碰撞檢測系統 - 直接檢測和推回玩家
pub fn simple_wall_collision_system(
    wall_query: Query<(Entity, &RoomTile, &Transform), Without<Player>>,
    door_query: Query<&Door>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = match player_query.single_mut() {
        Ok(transform) => transform,
        Err(_) => return,
    };
    
    let tile_size = ROOM_TILE_SIZE * PLAYER_SCALE; // 48 像素
    let collision_threshold = tile_size * 0.7; // 約33.6像素的碰撞檢測範圍，允許更近的接觸
    
    let player_pos = player_transform.translation.truncate();
    
    // 系統正常運行，無需調試輸出
    
    // 首先建立位置到瓷磚的映射，優先考慮地板瓷磚
    let mut position_tiles: HashMap<(i32, i32), (Entity, &RoomTile, &Transform)> = HashMap::new();
    
    for (entity, room_tile, wall_transform) in &wall_query {
        let pos_key = (
            (wall_transform.translation.x / tile_size).round() as i32,
            (wall_transform.translation.y / tile_size).round() as i32
        );
        
        // 如果這個位置已經有瓷磚了
        if let Some((existing_entity, existing_tile, existing_transform)) = position_tiles.get(&pos_key) {
            // 如果現有的是牆壁，新的是地板，則替換
            if is_wall_tile(&existing_tile.tile_type) && room_tile.tile_type == RoomTileType::Floor {
                position_tiles.insert(pos_key, (entity, room_tile, wall_transform));
            }
            // 如果現有的是地板，新的是牆壁，則保持現有的（不替換）
            // 其他情況也不替換
        } else {
            // 如果位置是空的，直接插入
            position_tiles.insert(pos_key, (entity, room_tile, wall_transform));
        }
    }
    
    // 檢查碰撞，使用優先級處理後的瓷磚
    for (_pos_key, (entity, room_tile, wall_transform)) in position_tiles {
        if is_wall_tile(&room_tile.tile_type) {
            // 對於門，需要檢查是否開啟
            if matches!(room_tile.tile_type, RoomTileType::DoorClosed | RoomTileType::DoorOpen) {
                if let Ok(door) = door_query.get(entity) {
                    if door.is_open {
                        // debug: 確認開啟的門被跳過
                        info!("🚪 跳過開啟的門，允許玩家通過");
                        continue; // 開啟的門不阻擋玩家
                    } else {
                        info!("🚪 門是關閉的，阻擋玩家");
                    }
                } else {
                    info!("❌ 門瓷磚沒有Door組件！");
                }
            }
            
            let wall_pos = wall_transform.translation.truncate();
            let distance = wall_pos.distance(player_pos);
            
            // 如果玩家太靠近牆壁
            if distance < collision_threshold {
                // 計算推開玩家的方向
                let push_direction = (player_pos - wall_pos).normalize_or_zero();
                
                // 將玩家推到安全距離
                let safe_distance = collision_threshold + 1.0;
                let new_position = wall_pos + push_direction * safe_distance;
                
                player_transform.translation.x = new_position.x;
                player_transform.translation.y = new_position.y;
                
                return; // 一次只處理一個碰撞
            }
        }
    }
}