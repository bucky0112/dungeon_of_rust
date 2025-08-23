use bevy::prelude::*;
use std::f32::consts::PI;
use crate::components::{
    player::{Player, InputVector, Speed},
    attack::{PlayerFacing, Weapon, WeaponOffset, WeaponSwingAnimation, WeaponType, WeaponSprites},
};

// 系統：處理輸入
pub fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut InputVector, With<Player>>,
) {
    if let Ok(mut input_vector) = query.single_mut() {
        let mut direction = Vec2::ZERO;
        
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        
        input_vector.0 = direction.normalize_or_zero();
    }
}

// 系統：更新玩家面向（記錄最後移動方向）
pub fn update_player_facing_system(
    mut player_query: Query<(&mut PlayerFacing, &InputVector), With<Player>>,
) {
    for (mut facing, input) in &mut player_query {
        // 如果有移動輸入，更新面向
        if input.0.length() > 0.1 {
            facing.direction = input.0.normalize();
        }
    }
}

// 系統：移動玩家
pub fn player_movement_system(
    mut query: Query<(&mut Transform, &InputVector, &Speed), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, input, speed) in &mut query {
        if input.0.length() > 0.1 {
            let movement = input.0 * speed.0 * time.delta_secs();
            transform.translation += movement.extend(0.0);
        }
    }
}

// 系統：根據面向更新武器偏移
pub fn update_weapon_offset_system(
    player_query: Query<&PlayerFacing, (With<Player>, Changed<PlayerFacing>)>,
    mut weapon_query: Query<(&mut WeaponOffset, &mut Transform, &mut Sprite, &WeaponSprites), With<Weapon>>,
) {
    if let Ok(facing) = player_query.single() {
        for (mut offset, mut transform, mut sprite, weapon_sprites) in &mut weapon_query {
            // 根據面向計算武器位置和角度
            let (position, angle, z_layer, is_left_side) = calculate_weapon_position_and_rotation(&facing.direction);
            
            offset.position = position;
            offset.base_angle = angle;
            offset.z_layer = z_layer;
            
            // 根據面向切換圖片
            sprite.image = if is_left_side {
                weapon_sprites.left_sprite.clone()
            } else {
                weapon_sprites.right_sprite.clone()
            };
            
            // 移除翻轉，因為我們用不同圖片
            sprite.flip_x = false;
            sprite.flip_y = false;
            
            // 立即應用到 Transform（如果沒有攻擊動畫）
            transform.translation = position.extend(z_layer);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

// 系統：處理攻擊輸入和動畫觸發
pub fn visual_attack_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut weapon_query: Query<&mut WeaponSwingAnimation, With<Weapon>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut swing_animation in &mut weapon_query {
            if !swing_animation.is_attacking {
                // 開始攻擊動畫
                swing_animation.is_attacking = true;
                swing_animation.timer.reset();
                
                info!("開始武器揮擊動畫！");
            }
        }
    }
}

// 系統：更新武器揮擊動畫
pub fn update_weapon_swing_animation_system(
    mut weapon_query: Query<(&mut WeaponSwingAnimation, &mut Transform, &WeaponOffset), With<Weapon>>,
    player_query: Query<&PlayerFacing, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(facing) = player_query.single() {
        for (mut swing, mut transform, offset) in &mut weapon_query {
            if swing.is_attacking {
                swing.timer.tick(time.delta());
                
                // 計算插值進度 (0.0 到 1.0)
                let progress = swing.timer.elapsed_secs() / swing.timer.duration().as_secs_f32();
                
                // 獲取當前面向狀態
                let (_, _, _, is_left_side) = calculate_weapon_position_and_rotation(&facing.direction);
                
                if swing.timer.finished() {
                    // 動畫結束
                    swing.is_attacking = false;
                    // 回到基礎角度
                    transform.rotation = Quat::from_rotation_z(offset.base_angle);
                    info!("武器揮擊動畫完成！");
                } else {
                    // 插值計算當前角度
                    let current_angle = lerp_angle(swing.from_angle, swing.to_angle, progress);
                    
                    // 根據左右側調整攻擊角度
                    let adjusted_angle = if is_left_side {
                        // 左側需要相對於基礎角度來計算攻擊角度
                        offset.base_angle + current_angle
                    } else {
                        // 右側正常計算
                        offset.base_angle + current_angle
                    };
                    
                    transform.rotation = Quat::from_rotation_z(adjusted_angle);
                }
            }
        }
    }
}

// 輔助函數：根據面向方向計算武器位置和旋轉
fn calculate_weapon_position_and_rotation(facing_direction: &Vec2) -> (Vec2, f32, f32, bool) {
    let angle = facing_direction.y.atan2(facing_direction.x);
    let octant = get_direction_octant(angle);
    
    // 根據面向調整武器位置偏移（往上調整3個像素單位）
    let hand_offset = match octant {
        0 => Vec2::new(8.0, 1.0),     // 向右 (從-2.0調到1.0)
        1 => Vec2::new(6.0, 7.0),     // 右上 (從4.0調到7.0)
        2 => Vec2::new(-2.0, 11.0),   // 向上 (從8.0調到11.0)
        3 => Vec2::new(-6.0, 7.0),    // 左上 (從4.0調到7.0)
        4 => Vec2::new(-8.0, 1.0),    // 向左 (從-2.0調到1.0)
        5 => Vec2::new(-6.0, -3.0),   // 左下 (從-6.0調到-3.0)
        6 => Vec2::new(-2.0, -5.0),   // 向下 (從-8.0調到-5.0)
        7 => Vec2::new(6.0, -3.0),    // 右下 (從-6.0調到-3.0)
        _ => Vec2::new(8.0, 1.0),     // 默認向右
    };
    
    // 武器基礎角度
    let weapon_angle = angle;
    
    // Z 層級：所有方向都在前景，讓武器始終可見
    let z_layer = 1.0;
    
    // 判斷是否為左側方向（需要使用左側圖片）
    let is_left_side = matches!(octant, 3 | 4 | 5);
    
    (hand_offset, weapon_angle, z_layer, is_left_side)
}

// 輔助函數：將角度轉換為8方向
fn get_direction_octant(angle: f32) -> u8 {
    let normalized = (angle + 2.0 * PI) % (2.0 * PI);
    ((normalized + PI / 8.0) / (PI / 4.0)) as u8 % 8
}

// 輔助函數：角度插值（處理角度環繞）
fn lerp_angle(from: f32, to: f32, t: f32) -> f32 {
    let diff = ((to - from + PI) % (2.0 * PI)) - PI;
    from + diff * t
}

// 系統：生成玩家和武器的階層結構
pub fn spawn_player_with_weapon_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 生成玩家（父實體）
    let player_entity = commands.spawn((
        // 玩家 Sprite
        Sprite::from_image(asset_server.load("sprites/characters/knight_lv1.png")),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
            .with_scale(Vec3::splat(4.0)), // 增大角色
        Player,
        PlayerFacing { direction: Vec2::X }, // 默認面向右
        crate::components::player::InputVector(Vec2::ZERO),
        crate::components::player::Health { current: 100, max: 100 },
        crate::components::player::Speed(200.0),
        crate::components::player::AttackDamage(25),
        crate::components::player::AttackCooldown { 
            timer: Timer::from_seconds(0.5, TimerMode::Once) 
        },
        crate::components::player::CurrentWeapon { weapon_type: WeaponType::Sword },
        crate::components::attack::AttackAnimation {
            timer: Timer::from_seconds(0.3, TimerMode::Once),
            is_attacking: false,
        },
    )).id();
    
    // 載入武器圖片資源
    let right_sprite_handle = asset_server.load("sprites/weapons/sword.png");
    let left_sprite_handle = asset_server.load("sprites/weapons/sword_left.png");
    
    // 生成武器（子實體）
    let weapon_entity = commands.spawn((
        // 武器 Sprite（默認使用右側圖片）
        Sprite::from_image(right_sprite_handle.clone()),
        Transform::from_translation(Vec3::new(8.0, 1.0, 1.0))  // 更新初始位置往上移
            .with_scale(Vec3::splat(0.8)), // 因為會繼承父物件4x縮放，所以用0.8相當於3.2x
        Weapon { weapon_type: WeaponType::Sword },
        WeaponSprites {
            right_sprite: right_sprite_handle,
            left_sprite: left_sprite_handle,
        },
        WeaponOffset {
            position: Vec2::new(8.0, 1.0),   // 更新位置以匹配初始位置
            base_angle: 0.0,
            z_layer: 1.0,
        },
        WeaponSwingAnimation {
            timer: Timer::from_seconds(0.5, TimerMode::Once), // 0.5秒揮擊動畫
            from_angle: -PI / 4.0,  // 起始角度 -45度
            to_angle: PI / 4.0,     // 結束角度 +45度  
            is_attacking: false,
        },
    )).id();
    
    // 建立父子關係
    commands.entity(player_entity).add_child(weapon_entity);
    
    info!("玩家和武器階層已生成！");
}
