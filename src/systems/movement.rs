use bevy::prelude::*;
use crate::components::player::{Player, Velocity, InputVector, Speed};
use crate::components::movement::Direction;
use crate::resources::sprites::DirectionSpriteHandles;

pub fn movement_system(
    mut query: Query<(&mut Transform, &mut Velocity, &InputVector, &Speed), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, input_vector, speed) in &mut query {
        // 根據正規化的輸入向量和速度計算實際速度
        velocity.0 = input_vector.0 * speed.0;

        // 應用移動
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

pub fn sprite_direction_system(
    mut query: Query<(&InputVector, &mut Direction, &mut Sprite), With<Player>>,
    sprite_handles: Res<DirectionSpriteHandles>,
) {
    for (input_vector, mut direction, mut sprite) in &mut query {
        // 使用輸入向量而非速度來決定方向，停下來時保持原方向
        if let Some(new_direction) = Direction::from_input_vector(input_vector.0) {
            if *direction != new_direction {
                *direction = new_direction;
                if let Some(handle) = sprite_handles.handles.get(&new_direction) {
                    sprite.image = handle.clone();
                }
            }
        }
        // 如果沒有輸入（返回 None），保持當前方向不變
    }
}