use bevy::prelude::*;
use crate::components::player::{Player, AttackCooldown, AttackDamage, InputVector};
use crate::components::movement::Direction;
use crate::components::attack::{AttackEvent, AttackAnimation, Projectile, AttackType, MeleeAttack};

pub fn attack_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut attack_events: EventWriter<AttackEvent>,
    mut query: Query<(Entity, &Transform, &mut AttackCooldown, &AttackDamage, &Direction, &InputVector, &mut AttackAnimation), With<Player>>,
    time: Res<Time>,
) {
    // 每幀檢查是否有玩家實體
    if query.is_empty() {
        return; // 沒有玩家實體就退出
    }
    
    for (_entity, transform, mut cooldown, damage, direction, input_vector, mut animation) in &mut query {
        // 更新攻擊冷卻時間
        cooldown.timer.tick(time.delta());

        // 檢查攻擊按鍵 (空白鍵)
        if keyboard_input.just_pressed(KeyCode::Space) {
            info!("空白鍵被按下！冷卻狀態: {}", cooldown.timer.finished());
            if cooldown.timer.finished() {
                info!("攻擊觸發！");
                // 決定攻擊方向：如果有移動輸入就用移動方向，否則用面向方向
                let attack_direction = if input_vector.0.length() > 0.1 {
                    input_vector.0.normalize()
                } else {
                    direction.to_vec2()
                };

                // 發送攻擊事件
                attack_events.write(AttackEvent {
                    position: transform.translation,
                    direction: attack_direction,
                    damage: damage.0,
                });

                // 開始攻擊動畫
                animation.is_attacking = true;
                animation.timer.reset();

                // 重新開始冷卻計時
                cooldown.timer = Timer::from_seconds(0.5, TimerMode::Once);
            }
        }
    }
}

pub fn handle_attack_events(
    mut commands: Commands,
    mut attack_events: EventReader<AttackEvent>,
    _asset_server: Res<AssetServer>,
) {
    for event in attack_events.read() {
        info!("生成近戰攻擊效果 - 位置: {:?}, 方向: {:?}", event.position, event.direction);
        
        // 創建近戰攻擊視覺效果（劍氣）
        let attack_offset = event.direction * 50.0; // 攻擊範圍50像素
        
        commands.spawn((
            Sprite::from_color(Color::srgb(1.0, 0.8, 0.0), Vec2::new(80.0, 30.0)), // 金黃色劍氣，更大更明顯
            Transform::from_translation(event.position + attack_offset.extend(2.0)) // Z層級提高
                .with_rotation(Quat::from_rotation_z(event.direction.y.atan2(event.direction.x))), // 根據攻擊方向旋轉
            MeleeAttack {
                lifetime: Timer::from_seconds(0.3, TimerMode::Once), // 0.3秒後消失
                damage: event.damage,
            },
            AttackType::Melee,
        ));
    }
}


pub fn update_melee_attacks(
    mut commands: Commands,
    mut melee_query: Query<(Entity, &mut MeleeAttack)>,
    time: Res<Time>,
) {
    for (entity, mut melee_attack) in &mut melee_query {
        // 更新生命週期
        melee_attack.lifetime.tick(time.delta());

        // 如果超時就刪除攻擊效果
        if melee_attack.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_attack_animations(
    mut query: Query<(&mut AttackAnimation, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut animation, mut sprite) in &mut query {
        if animation.is_attacking {
            animation.timer.tick(time.delta());
            
            // 簡單的攻擊動畫效果：改變顏色
            if animation.timer.finished() {
                animation.is_attacking = false;
                // 恢復原始顏色
                sprite.color = Color::WHITE;
            } else {
                // 攻擊時微微變紅
                sprite.color = Color::srgb(1.0, 0.8, 0.8);
            }
        }
    }
}