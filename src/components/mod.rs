// Components module declarations
pub mod player;
pub mod movement;
pub mod camera;
pub mod world;

// Re-export all components for easy importing
pub use player::*;
pub use movement::*;
pub use camera::*;
pub use world::*;