// Systems module declarations
pub mod attack;
pub mod input;
pub mod movement;
pub mod camera;
pub mod health;
pub mod world;
pub mod setup;
pub mod visual_combat;  // 新的視覺化戰鬥系統
pub mod door_interaction;  // 門交互系統
pub mod wall_collision;  // 牆壁碰撞檢測系統
pub mod room_transition;  // 房間切換系統

// Re-export all systems for easy importing
pub use attack::*;
pub use input::*;
pub use movement::*;
pub use camera::*;
pub use health::*;
pub use world::*;
pub use setup::*;
pub use visual_combat::*;
pub use door_interaction::*;
pub use wall_collision::*;
pub use room_transition::*;