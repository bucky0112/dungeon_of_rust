use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::player::*;
use crate::components::movement::Direction;
use crate::components::camera::CameraFollow;
use crate::resources::sprites::DirectionSpriteHandles;
use crate::constants::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        CameraFollow { speed: CAMERA_FOLLOW_SPEED },
    ));
}

pub fn load_sprite_handles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handles = HashMap::new();
    
    // 預載所有方向的 Handle
    handles.insert(Direction::North, asset_server.load(Direction::North.get_sprite_path()));
    handles.insert(Direction::NorthEast, asset_server.load(Direction::NorthEast.get_sprite_path()));
    handles.insert(Direction::East, asset_server.load(Direction::East.get_sprite_path()));
    handles.insert(Direction::SouthEast, asset_server.load(Direction::SouthEast.get_sprite_path()));
    handles.insert(Direction::South, asset_server.load(Direction::South.get_sprite_path()));
    handles.insert(Direction::SouthWest, asset_server.load(Direction::SouthWest.get_sprite_path()));
    handles.insert(Direction::West, asset_server.load(Direction::West.get_sprite_path()));
    handles.insert(Direction::NorthWest, asset_server.load(Direction::NorthWest.get_sprite_path()));
    
    commands.insert_resource(DirectionSpriteHandles { handles });
    info!("角色方向 Sprite 已預載完成");
}

pub fn spawn_player(mut commands: Commands, sprite_handles: Res<DirectionSpriteHandles>) {
    let initial_direction = Direction::South;
    let sprite_handle = sprite_handles.handles.get(&initial_direction)
        .expect("初始方向的 Sprite Handle 應該已預載").clone();
        
    commands.spawn((
        Player,
        Sprite::from_image(sprite_handle),
        Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_PLAYER))
            .with_scale(Vec3::splat(PLAYER_SCALE)),
        Health {
            current: PLAYER_MAX_HEALTH,
            max: PLAYER_MAX_HEALTH,
        },
        Velocity(Vec2::ZERO),
        InputVector(Vec2::ZERO),
        Speed(PLAYER_SPEED),
        initial_direction,
    ));
    info!("騎士英雄已生成（3倍放大）！");
}