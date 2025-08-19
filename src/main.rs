use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct InputVector(Vec2);

#[derive(Resource)]
struct DirectionSpriteHandles {
    handles: HashMap<Direction, Handle<Image>>,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn get_sprite_path(&self) -> &str {
        match self {
            Direction::North => "characters/knight_hero/rotations/north.png",
            Direction::NorthEast => "characters/knight_hero/rotations/north-east.png",
            Direction::East => "characters/knight_hero/rotations/east.png",
            Direction::SouthEast => "characters/knight_hero/rotations/south-east.png",
            Direction::South => "characters/knight_hero/rotations/south.png",
            Direction::SouthWest => "characters/knight_hero/rotations/south-west.png",
            Direction::West => "characters/knight_hero/rotations/west.png",
            Direction::NorthWest => "characters/knight_hero/rotations/north-west.png",
        }
    }
    
    fn from_input_vector(input: Vec2) -> Option<Self> {
        // deadzone 檢查在正規化之前，避免微小輸入被放大
        if input.length() < 0.1 {
            return None;
        }
        
        let angle = input.y.atan2(input.x).to_degrees();
        let normalized_angle = if angle < 0.0 { angle + 360.0 } else { angle };
        
        let direction = match normalized_angle as i32 {
            0..=22 | 338..=360 => Direction::East,
            23..=67 => Direction::NorthEast,
            68..=112 => Direction::North,
            113..=157 => Direction::NorthWest,
            158..=202 => Direction::West,
            203..=247 => Direction::SouthWest,
            248..=292 => Direction::South,
            293..=337 => Direction::SouthEast,
            _ => Direction::South,
        };
        
        Some(direction)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // 設定像素藝術使用點採樣
        .add_systems(Startup, (setup, load_sprite_handles, spawn_player).chain())
        .add_systems(Update, (input_system, movement_system, sprite_direction_system, health_system))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn load_sprite_handles(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn spawn_player(mut commands: Commands, sprite_handles: Res<DirectionSpriteHandles>) {
    let initial_direction = Direction::South;
    let sprite_handle = sprite_handles.handles.get(&initial_direction)
        .expect("初始方向的 Sprite Handle 應該已預載").clone();
        
    commands.spawn((
        Player,
        Sprite::from_image(sprite_handle),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
            .with_scale(Vec3::splat(3.0)), // 放大 3 倍展現像素藝術的格子感
        Health {
            current: 100,
            max: 100,
        },
        Velocity(Vec2::ZERO),
        InputVector(Vec2::ZERO),
        Speed(300.0),
        initial_direction,
    ));
    info!("騎士英雄已生成（3倍放大）！");
}

fn input_system(
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
        input_vector.0 = if raw_input.length() > 0.1 {
            raw_input.normalize()  // 只有超過 deadzone 才正規化
        } else {
            Vec2::ZERO
        };
    }
}

fn movement_system(
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

fn sprite_direction_system(
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

fn health_system(query: Query<&Health, With<Player>>) {
    for health in &query {
        if health.current <= 0 {
            info!("玩家死亡！");
        }
    }
}
