use bevy::prelude::*;
use crate::components::player::{Player, InputVector};
use crate::components::world::{Door, RoomTile};
use crate::constants::*;

#[derive(Event)]
pub struct DoorInteractionEvent;

#[derive(Event)]
pub struct AttackInputEvent;

pub fn input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut InputVector, With<Player>>,
    player_query: Query<&Transform, With<Player>>,
    door_query: Query<(&Door, &Transform), (With<RoomTile>, Without<Player>)>,
    mut door_events: EventWriter<DoorInteractionEvent>,
    mut attack_events: EventWriter<AttackInputEvent>,
) {
    for mut input_vector in &mut query {
        let mut raw_input = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            raw_input.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            raw_input.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            raw_input.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            raw_input.x += 1.0;
        }

        // deadzone 檢查在正規化之前，避免微小輸入被放大
        input_vector.0 = if raw_input.length() > INPUT_DEADZONE {
            raw_input.normalize()  // 只有超過 deadzone 才正規化
        } else {
            Vec2::ZERO
        };
    }

    // 檢測空白鍵按下事件 - 優先處理門交互
    if keyboard_input.just_pressed(KeyCode::Space) {
        // 檢查玩家是否在門附近
        let player_transform = match player_query.single() {
            Ok(transform) => transform,
            Err(_) => return,
        };
        
        let mut near_door = false;
        let interaction_distance = ROOM_TILE_SIZE * PLAYER_SCALE * 10.0; // 10個瓷磚的距離，約480像素交互範圍
        
        for (_door, door_transform) in &door_query {
            let distance = player_transform.translation.distance(door_transform.translation);
            if distance <= interaction_distance {
                near_door = true;
                break;
            }
        }
        
        // 根據是否在門附近發送不同事件
        if near_door {
            door_events.write(DoorInteractionEvent);
        } else {
            attack_events.write(AttackInputEvent);
        }
    }
}