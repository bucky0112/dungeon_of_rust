// Systems module declarations
pub mod input;
pub mod movement;
pub mod camera;
pub mod health;
pub mod world;
pub mod setup;

// Re-export all systems for easy importing
pub use input::*;
pub use movement::*;
pub use camera::*;
pub use health::*;
pub use world::*;
pub use setup::*;