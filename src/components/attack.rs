use bevy::prelude::*;

// 攻擊事件
#[derive(Event)]
pub struct AttackEvent {
    pub position: Vec3,
    pub direction: Vec2,
    pub damage: i32,
}

// 子彈/投射物相關 Components
#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub damage: i32,
}

#[derive(Component)]
pub struct AttackAnimation {
    pub timer: Timer,
    pub is_attacking: bool,
}

// 攻擊類型
#[derive(Component)]
pub enum AttackType {
    Melee,   // 近戰攻擊
    Ranged,  // 遠程攻擊（子彈）
}

// 近戰攻擊組件
#[derive(Component)]
pub struct MeleeAttack {
    pub lifetime: Timer,
    pub damage: i32,
}