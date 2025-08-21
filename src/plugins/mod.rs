// Plugins module declarations
pub mod attack;
pub mod player;
pub mod camera;
pub mod world;
pub mod input;

// Re-export all plugins for easy importing
pub use attack::*;
pub use player::*;
pub use camera::*;
pub use world::*;
pub use input::*;