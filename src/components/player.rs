use bevy::prelude::*;
use crate::components::attack::WeaponType;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct InputVector(pub Vec2);

// 攻擊相關 Components
#[derive(Component)]
pub struct AttackCooldown {
    pub timer: Timer,
}

#[derive(Component)]
pub struct AttackDamage(pub i32);

#[derive(Component)]
pub struct AttackRange(pub f32);

#[derive(Component)]
pub struct CurrentWeapon {
    pub weapon_type: WeaponType,
}