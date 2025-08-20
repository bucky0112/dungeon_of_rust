use bevy::prelude::*;
use crate::components::player::{Player, InputVector};
use crate::constants::INPUT_DEADZONE;

pub fn input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut InputVector, With<Player>>,
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
}