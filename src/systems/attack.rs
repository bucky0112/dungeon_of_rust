use bevy::prelude::*;
use crate::components::player::{Player, AttackCooldown, AttackDamage, InputVector, CurrentWeapon};
use crate::components::movement::Direction;
use crate::components::attack::{
    AttackEvent, AttackAnimation, Projectile, AttackType, MeleeAttack, WeaponType, 
    WeaponEffect, CombatState, TimingAttack, AttackStage, AttackTiming, WeaponSelector
};

pub fn attack_input_system(
    mut attack_input_events: EventReader<crate::systems::input::AttackInputEvent>,
    mut attack_events: EventWriter<AttackEvent>,
    mut query: Query<(Entity, &Transform, &mut AttackCooldown, &AttackDamage, &Direction, &InputVector, &mut AttackAnimation, &CurrentWeapon), With<Player>>,
    time: Res<Time>,
) {
    // 每幀檢查是否有玩家實體
    if query.is_empty() {
        return; // 沒有玩家實體就退出
    }
    
    // 更新所有玩家的攻擊冷卻時間
    for (_entity, transform, mut cooldown, damage, direction, input_vector, mut animation, current_weapon) in &mut query {
        cooldown.timer.tick(time.delta());
    }
    
    // 檢查攻擊事件
    for _event in attack_input_events.read() {
        for (_entity, transform, mut cooldown, damage, direction, input_vector, mut animation, current_weapon) in &mut query {
            info!("收到攻擊輸入事件！冷卻狀態: {}", cooldown.timer.finished());
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
                    weapon_type: current_weapon.weapon_type,
                    timing: None, // 基礎攻擊沒有時機判定
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
    asset_server: Res<AssetServer>,
) {
    for event in attack_events.read() {
        info!("生成武器攻擊效果 - 位置: {:?}, 方向: {:?}, 武器: {:?}", 
              event.position, event.direction, event.weapon_type);
        
        match event.weapon_type {
            WeaponType::Sword => {
                spawn_sword_effect(&mut commands, &asset_server, event);
            }
            WeaponType::Magic => {
                spawn_magic_effect(&mut commands, &asset_server, event);
            }
            WeaponType::Arrow => {
                spawn_arrow_projectile(&mut commands, &asset_server, event);
            }
        }
    }
}

fn spawn_sword_effect(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    event: &AttackEvent,
) {
    let attack_offset = event.direction * 50.0;
    
    // Try to load sword swing sprite, fall back to colored rectangle if not found
    let sprite_bundle = if std::path::Path::new("assets/weapons/sword_swing.png").exists() {
        Sprite::from_image(asset_server.load("weapons/sword_swing.png"))
    } else {
        // Fallback to colored sprite with sword-like shape
        Sprite::from_color(Color::srgb(1.0, 0.8, 0.0), Vec2::new(80.0, 30.0))
    };
    
    commands.spawn((
        sprite_bundle,
        Transform::from_translation(event.position + attack_offset.extend(2.0))
            .with_rotation(Quat::from_rotation_z(event.direction.y.atan2(event.direction.x))),
        WeaponEffect {
            lifetime: Timer::from_seconds(0.3, TimerMode::Once),
            weapon_type: WeaponType::Sword,
            scale_animation: true,
        },
        MeleeAttack {
            lifetime: Timer::from_seconds(0.3, TimerMode::Once),
            damage: event.damage,
        },
        AttackType::Melee,
    ));
}

fn spawn_magic_effect(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    event: &AttackEvent,
) {
    let projectile_offset = event.direction * 30.0;
    
    let sprite_bundle = if std::path::Path::new("assets/weapons/magic_fireball.png").exists() {
        Sprite::from_image(asset_server.load("weapons/magic_fireball.png"))
    } else {
        // Fallback to orange circle
        Sprite::from_color(Color::srgb(1.0, 0.5, 0.0), Vec2::new(24.0, 24.0))
    };
    
    commands.spawn((
        sprite_bundle,
        Transform::from_translation(event.position + projectile_offset.extend(1.0)),
        Projectile {
            velocity: event.direction * 200.0,
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            damage: event.damage,
        },
        WeaponEffect {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            weapon_type: WeaponType::Magic,
            scale_animation: false,
        },
        AttackType::Ranged,
    ));
}

fn spawn_arrow_projectile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    event: &AttackEvent,
) {
    let projectile_offset = event.direction * 30.0;
    
    let sprite_bundle = if std::path::Path::new("assets/weapons/arrow.png").exists() {
        Sprite::from_image(asset_server.load("weapons/arrow.png"))
    } else {
        // Fallback to brown rectangle
        Sprite::from_color(Color::srgb(0.6, 0.3, 0.1), Vec2::new(20.0, 4.0))
    };
    
    commands.spawn((
        sprite_bundle,
        Transform::from_translation(event.position + projectile_offset.extend(1.0))
            .with_rotation(Quat::from_rotation_z(event.direction.y.atan2(event.direction.x))),
        Projectile {
            velocity: event.direction * 300.0,
            lifetime: Timer::from_seconds(3.0, TimerMode::Once),
            damage: event.damage,
        },
        WeaponEffect {
            lifetime: Timer::from_seconds(3.0, TimerMode::Once),
            weapon_type: WeaponType::Arrow,
            scale_animation: false,
        },
        AttackType::Ranged,
    ));
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

// 新增：更新武器效果動畫
pub fn update_weapon_effects(
    mut commands: Commands,
    mut weapon_query: Query<(Entity, &mut WeaponEffect, &mut Transform, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut effect, mut transform, mut sprite) in &mut weapon_query {
        effect.lifetime.tick(time.delta());
        
        if effect.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        
        // 根據武器類型播放不同動畫
        match effect.weapon_type {
            WeaponType::Sword => {
                if effect.scale_animation {
                    // 劍氣漸漸縮小並變透明
                    let progress = effect.lifetime.elapsed_secs() / effect.lifetime.duration().as_secs_f32();
                    let scale = 1.0 + (1.0 - progress) * 0.5; // 從1.5倍縮小到1.0倍
                    transform.scale = Vec3::splat(scale);
                    sprite.color.set_alpha(1.0 - progress);
                }
            }
            WeaponType::Magic => {
                // 魔法效果：輕微旋轉和閃爍
                transform.rotate_z(time.delta_secs() * 2.0);
                let flicker = (time.elapsed_secs() * 10.0).sin().abs();
                sprite.color.set_alpha(0.7 + flicker * 0.3);
            }
            WeaponType::Arrow => {
                // 箭矢無特殊效果，保持穩定
            }
        }
    }
}

// 新增：更新投射物移動
pub fn update_projectiles(
    mut projectile_query: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in &mut projectile_query {
        // 根據速度移動投射物
        let movement = projectile.velocity * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}

// 新增：武器切換系統
pub fn weapon_switching_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CurrentWeapon, With<Player>>,
) {
    if let Ok(mut current_weapon) = query.single_mut() {
        if keyboard_input.just_pressed(KeyCode::Digit1) {
            current_weapon.weapon_type = WeaponType::Sword;
            info!("切換到劍！");
        } else if keyboard_input.just_pressed(KeyCode::Digit2) {
            current_weapon.weapon_type = WeaponType::Magic;
            info!("切換到魔法！");
        } else if keyboard_input.just_pressed(KeyCode::Digit3) {
            current_weapon.weapon_type = WeaponType::Arrow;
            info!("切換到弓箭！");
        }
    }
}

// 戰鬥狀態管理系統 (參考 GitHub 專案)
pub fn combat_state_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<CombatState>>,
    mut next_state: ResMut<NextState<CombatState>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    match current_state.get() {
        CombatState::Idle => {
            // 按下 T 鍵進入戰鬥模式 (Timing Attack)
            if keyboard_input.just_pressed(KeyCode::KeyT) {
                info!("進入時機攻擊模式！");
                next_state.set(CombatState::PlayerSelecting);
                
                // 為玩家添加武器選擇器
                if let Ok(player_entity) = player_query.single() {
                    commands.entity(player_entity).insert(WeaponSelector {
                        available_weapons: vec![WeaponType::Sword, WeaponType::Magic, WeaponType::Arrow],
                        selected_index: 0,
                    });
                }
            }
        },
        CombatState::PlayerSelecting => {
            // 在這個狀態下可以選擇武器和確認攻擊
            // (具體實作在武器選擇系統中)
        },
        CombatState::PlayerAttacking => {
            // 攻擊完成後回到空閒狀態
            // (由時機攻擊系統管理)
        },
        CombatState::EnemyAttacking => {
            // 敵人攻擊邏輯
            // TODO: 實作敵人攻擊系統
        },
    }
}

// 時機攻擊系統 (參考 GitHub 專案)
pub fn timing_attack_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    current_state: Res<State<CombatState>>,
    mut next_state: ResMut<NextState<CombatState>>,
    mut timing_query: Query<(Entity, &mut TimingAttack, &Transform, &AttackDamage, &CurrentWeapon), With<Player>>,
    selector_query: Query<&WeaponSelector, With<Player>>,
    mut attack_events: EventWriter<AttackEvent>,
    time: Res<Time>,
) {
    // 武器選擇階段
    if *current_state.get() == CombatState::PlayerSelecting {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if let Ok(selector) = selector_query.single() {
                let selected_weapon = selector.available_weapons[selector.selected_index];
                
                // 開始時機攻擊
                if let Ok(player_entity) = timing_query.single() {
                    let (entity, _, _, _, _) = player_entity;
                    commands.entity(entity).insert(TimingAttack {
                        stage: AttackStage::Warmup,
                        warmup_timer: Timer::from_seconds(1.0, TimerMode::Once),
                        action_timer: Timer::from_seconds(0.5, TimerMode::Once),
                        cooldown_timer: Timer::from_seconds(0.5, TimerMode::Once),
                        damage_multiplier: 1.0,
                    });
                    
                    next_state.set(CombatState::PlayerAttacking);
                    info!("開始時機攻擊 - 武器: {:?}", selected_weapon);
                }
            }
        }
        return;
    }

    // 攻擊執行階段
    if *current_state.get() == CombatState::PlayerAttacking {
        for (entity, mut timing_attack, transform, damage, current_weapon) in &mut timing_query {
            match timing_attack.stage {
                AttackStage::Warmup => {
                    timing_attack.warmup_timer.tick(time.delta());
                    if timing_attack.warmup_timer.finished() {
                        timing_attack.stage = AttackStage::Action;
                        info!("進入行動階段！按空白鍵攻擊！");
                    }
                },
                AttackStage::Action => {
                    timing_attack.action_timer.tick(time.delta());
                    
                    // 檢查時機輸入
                    if keyboard_input.just_pressed(KeyCode::Space) {
                        let elapsed_ratio = timing_attack.action_timer.elapsed_secs() / 
                                          timing_attack.action_timer.duration().as_secs_f32();
                        
                        let timing = match elapsed_ratio {
                            ratio if ratio < 0.3 => AttackTiming::Early,
                            ratio if ratio > 0.7 => AttackTiming::Late,
                            _ => AttackTiming::Critical,
                        };
                        
                        // 根據時機調整傷害
                        timing_attack.damage_multiplier = match timing {
                            AttackTiming::Critical => 2.0, // 完美時機雙倍傷害
                            AttackTiming::Early | AttackTiming::Late => 0.5, // 差時機減半傷害
                        };
                        
                        // 發送攻擊事件
                        attack_events.write(AttackEvent {
                            position: transform.translation,
                            direction: Vec2::Y, // 預設向上攻擊
                            damage: (damage.0 as f32 * timing_attack.damage_multiplier) as i32,
                            weapon_type: current_weapon.weapon_type,
                            timing: Some(timing),
                        });
                        
                        timing_attack.stage = AttackStage::CoolDown;
                        info!("攻擊執行！時機: {:?}, 傷害倍數: {:.1}", timing, timing_attack.damage_multiplier);
                    }
                    
                    // 如果時間到了還沒按，算作差時機
                    if timing_attack.action_timer.finished() {
                        attack_events.write(AttackEvent {
                            position: transform.translation,
                            direction: Vec2::Y,
                            damage: (damage.0 as f32 * 0.5) as i32,
                            weapon_type: current_weapon.weapon_type,
                            timing: Some(AttackTiming::Late),
                        });
                        
                        timing_attack.stage = AttackStage::CoolDown;
                        info!("時間到！自動執行攻擊（Late timing）");
                    }
                },
                AttackStage::CoolDown => {
                    timing_attack.cooldown_timer.tick(time.delta());
                    if timing_attack.cooldown_timer.finished() {
                        // 移除時機攻擊組件並回到空閒狀態
                        commands.entity(entity).remove::<TimingAttack>();
                        commands.entity(entity).remove::<WeaponSelector>();
                        next_state.set(CombatState::Idle);
                        info!("攻擊完成，回到空閒狀態");
                    }
                },
            }
        }
    }
}