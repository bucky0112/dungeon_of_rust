use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::movement::Direction;

#[derive(Resource)]
pub struct DirectionSpriteHandles {
    pub handles: HashMap<Direction, Handle<Image>>,
}