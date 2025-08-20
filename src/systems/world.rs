use bevy::prelude::*;
use crate::components::world::GridTile;
use crate::constants::*;

pub fn spawn_grid(mut commands: Commands) {
    let tile_size = TILE_SIZE;
    let grid_size = GRID_SIZE;
    let half_grid = grid_size as f32 / 2.0;
    
    for x in 0..grid_size {
        for y in 0..grid_size {
            let world_x = (x as f32 - half_grid + 0.5) * tile_size;
            let world_y = (y as f32 - half_grid + 0.5) * tile_size;
            
            // 創建簡單的格子視覺效果 - 棋盤式顏色
            let color = if (x + y) % 2 == 0 {
                Color::srgb(GRID_DARK_COLOR.0, GRID_DARK_COLOR.1, GRID_DARK_COLOR.2)
            } else {
                Color::srgb(GRID_LIGHT_COLOR.0, GRID_LIGHT_COLOR.1, GRID_LIGHT_COLOR.2)
            };
            
            commands.spawn((
                Sprite::from_color(color, Vec2::new(tile_size, tile_size)),
                Transform::from_translation(Vec3::new(world_x, world_y, Z_LAYER_GRID)),
                GridTile,
            ));
        }
    }
    
    // 在中心加上特殊標記
    commands.spawn((
        Sprite::from_color(Color::srgb(CENTER_MARKER_COLOR.0, CENTER_MARKER_COLOR.1, CENTER_MARKER_COLOR.2), Vec2::new(tile_size, tile_size)),
        Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_MARKERS)),
        GridTile,
    ));
    
    // 在四個角落加上特殊標記
    let corners = [
        (-half_grid * tile_size, -half_grid * tile_size),
        (half_grid * tile_size, -half_grid * tile_size),
        (-half_grid * tile_size, half_grid * tile_size),
        (half_grid * tile_size, half_grid * tile_size),
    ];
    
    for (corner_x, corner_y) in corners.iter() {
        commands.spawn((
            Sprite::from_color(Color::srgb(CORNER_MARKER_COLOR.0, CORNER_MARKER_COLOR.1, CORNER_MARKER_COLOR.2), Vec2::new(tile_size * CORNER_MARKER_SCALE, tile_size * CORNER_MARKER_SCALE)),
            Transform::from_translation(Vec3::new(*corner_x, *corner_y, Z_LAYER_MARKERS)),
            GridTile,
        ));
    }
    
    info!("格子地板已生成（20x20 + 參考點）");
}