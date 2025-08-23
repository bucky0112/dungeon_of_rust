use bevy::prelude::*;

// 戰鬥狀態 (參考 GitHub 專案)
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum CombatState {
    #[default]
    Idle,           // 非戰鬥狀態
    PlayerSelecting, // 玩家選擇攻擊
    PlayerAttacking, // 玩家攻擊中
    EnemyAttacking,  // 敵人攻擊中
}

// 攻擊時機 (參考 GitHub 專案)
#[derive(Debug, Clone, Copy)]
pub enum AttackTiming {
    Early,
    Critical,  // 完美時機
    Late,
}

// 攻擊階段 (參考 GitHub 專案)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttackStage {
    Warmup,   // 準備階段
    Action,   // 執行階段 (可以輸入timing)
    CoolDown, // 冷卻階段
}

// 攻擊事件
#[derive(Event)]
pub struct AttackEvent {
    pub position: Vec3,
    pub direction: Vec2,
    pub damage: i32,
    pub weapon_type: WeaponType,
    pub timing: Option<AttackTiming>, // 新增時機判定
}

// 武器類型
#[derive(Clone, Copy, Debug)]
pub enum WeaponType {
    Sword,
    Magic,
    Arrow,
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

// 武器效果組件
#[derive(Component)]
pub struct WeaponEffect {
    pub lifetime: Timer,
    pub weapon_type: WeaponType,
    pub scale_animation: bool,
}

// 攻擊時機系統 (參考 GitHub 專案)
#[derive(Component)]
pub struct TimingAttack {
    pub stage: AttackStage,
    pub warmup_timer: Timer,
    pub action_timer: Timer,
    pub cooldown_timer: Timer,
    pub damage_multiplier: f32, // 基於時機的傷害倍數
}

// 武器選擇 UI (參考 GitHub 專案)
#[derive(Component)]
pub struct WeaponSelector {
    pub available_weapons: Vec<WeaponType>,
    pub selected_index: usize,
}

// === 新的視覺化攻擊系統 ===

// 武器組件 - 作為子物件
#[derive(Component)]
pub struct Weapon {
    pub weapon_type: WeaponType,
}

// 武器圖片資源組件 - 儲存左右兩側的圖片Handle
#[derive(Component)]
pub struct WeaponSprites {
    pub right_sprite: Handle<Image>,  // 右側/默認圖片
    pub left_sprite: Handle<Image>,   // 左側圖片
}

// 攻擊動畫組件 - 處理武器揮擊
#[derive(Component)]
pub struct WeaponSwingAnimation {
    pub timer: Timer,
    pub from_angle: f32,    // 起始角度（弧度）
    pub to_angle: f32,      // 結束角度（弧度）
    pub is_attacking: bool,
}

// 玩家面向組件 - 記錄最後移動方向
#[derive(Component)]
pub struct PlayerFacing {
    pub direction: Vec2,    // 最後的移動方向
}

// 武器偏移配置
#[derive(Component)]
pub struct WeaponOffset {
    pub position: Vec2,     // 相對於玩家的位置偏移
    pub base_angle: f32,    // 基礎角度（弧度）
    pub z_layer: f32,       // Z 層級（前景/背景）
}