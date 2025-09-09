// Plugins module declarations
pub mod attack;
pub mod player;
pub mod camera;
pub mod world;
pub mod input;
pub mod visual_combat;  // 新的視覺化戰鬥插件
pub mod door_interaction;  // 門交互插件
pub mod wall_collision;  // 牆壁碰撞插件
pub mod room_transition;  // 房間切換插件

// Re-export all plugins for easy importing
pub use attack::*;
pub use player::*;
pub use camera::*;
pub use world::*;
pub use input::*;
pub use visual_combat::*;
pub use door_interaction::*;
pub use wall_collision::*;
pub use room_transition::*;