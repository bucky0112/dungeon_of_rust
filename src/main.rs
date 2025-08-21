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
            PlayerPlugin,
            InputPlugin,
            AttackPlugin,
        ))
        .run();
}

