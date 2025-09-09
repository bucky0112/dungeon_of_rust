use bevy::prelude::*;

mod constants;
mod components;
mod resources;
mod systems;
mod plugins;

use plugins::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // 設定像素藝術使用點採樣
        .add_plugins((
            CameraPlugin,
            WorldPlugin,
            // PlayerPlugin,        // 暫時禁用舊的 PlayerPlugin
            InputPlugin,
            // AttackPlugin,        // 暫時禁用舊的 AttackPlugin
            VisualCombatPlugin,     // 使用新的視覺化戰鬥系統
            DoorInteractionPlugin,  // 門交互系統
            WallCollisionPlugin,    // 牆壁碰撞檢測系統
            RoomTransitionPlugin,   // 房間切換系統
        ))
        .run();
}

