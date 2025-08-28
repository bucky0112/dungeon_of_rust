use bevy::prelude::*;
use rand::Rng;
use crate::components::world::{GridTile, RoomTile, Room, RoomTileType, CompoundRoom, CompoundRoomType, RoomRect};
use crate::resources::RoomAssets;
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
    
    info!("格子地板已生成（20x20）");
}

pub fn spawn_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 初始化房間資源
    let room_assets = RoomAssets::load_all(&asset_server);
    commands.insert_resource(room_assets);
    
    // 隨機選擇房間類型
    let mut rng = rand::thread_rng();
    let room_type_choice = rng.gen_range(0..4);
    
    match room_type_choice {
        0 => {
            // 基本矩形房間 (30% 機率)
            let room_width = rng.gen_range(8..15);
            let room_height = rng.gen_range(6..10);
            let room_x = -(room_width as i32) / 2;
            let room_y = -(room_height as i32) / 2;
            
            generate_room_tiles(&mut commands, &asset_server, room_width, room_height, room_x, room_y);
            info!("矩形房間已生成 ({}x{})", room_width, room_height);
        },
        1 => {
            // L 形房間 (25% 機率)
            let compound_room = generate_l_shape_room(&mut rng);
            spawn_compound_room(&mut commands, &asset_server, compound_room);
            info!("L 形房間已生成");
        },
        2 => {
            // T 形房間 (25% 機率)
            let compound_room = generate_t_shape_room(&mut rng);
            spawn_compound_room(&mut commands, &asset_server, compound_room);
            info!("T 形房間已生成");
        },
        _ => {
            // 十字形房間 (20% 機率)
            let compound_room = generate_plus_shape_room(&mut rng);
            spawn_compound_room(&mut commands, &asset_server, compound_room);
            info!("十字形房間已生成");
        }
    }
}

fn generate_room_tiles(
    commands: &mut Commands,
    asset_server: &AssetServer,
    width: usize,
    height: usize,
    start_x: i32,
    start_y: i32,
) {
    let tile_size = ROOM_TILE_SIZE * PLAYER_SCALE;  // 使用房間瓷磚專用尺寸並考慮縮放
    let room_assets = RoomAssets::load_all(asset_server);
    
    // 生成房間結構：包含南牆外側 + 內部 + 北牆
    // 南牆外側（y=0）-> 南牆內側（y=1）-> 地板（y=2..height-1）-> 北牆（y=height-1）
    let total_height = height + 1; // 增加一行給南牆外側
    
    for y in 0..total_height {
        for x in 0..width {
            let world_x = (start_x + x as i32) as f32 * tile_size;
            let world_y = (start_y + y as i32 - 1) as f32 * tile_size; // -1 調整南牆外側位置
            
            // 決定瓷磚類型和對應的圖片
            let (tile_type, texture_handle) = if y == total_height - 1 {
                // 北牆（上方，面向玩家）
                if x == 0 {
                    (RoomTileType::WallNInnerCornerW, room_assets.wall_n_inner_corner_w.clone())
                } else if x == width - 1 {
                    (RoomTileType::WallNInnerCornerE, room_assets.wall_n_inner_corner_e.clone())
                } else {
                    (RoomTileType::WallNInnerMid, room_assets.wall_n_inner_mid.clone())
                }
            } else if y == 1 {
                // 南牆內側
                if x == 0 {
                    (RoomTileType::WallSInnerCapL, room_assets.wall_s_inner_cap_l.clone())
                } else if x == width - 1 {
                    (RoomTileType::WallSInnerCapR, room_assets.wall_s_inner_cap_r.clone())
                } else {
                    (RoomTileType::WallSInnerMid, room_assets.wall_s_inner_mid.clone())
                }
            } else if y == 0 {
                // 南牆外側
                if x == 0 {
                    (RoomTileType::WallSOuterCapL, room_assets.wall_s_outer_cap_l.clone())
                } else if x == width - 1 {
                    (RoomTileType::WallSOuterCapR, room_assets.wall_s_outer_cap_r.clone())
                } else {
                    (RoomTileType::WallSOuterMid, room_assets.wall_s_outer_mid.clone())
                }
            } else if x == 0 {
                // 西側牆
                (RoomTileType::WallWSide, room_assets.wall_w_side.clone())
            } else if x == width - 1 {
                // 東側牆
                (RoomTileType::WallESide, room_assets.wall_e_side.clone())
            } else {
                // 內部地板
                (RoomTileType::Floor, room_assets.floor_indoor.clone())
            };
            
            // 生成瓷磚實體
            commands.spawn((
                Sprite::from_image(texture_handle),
                Transform::from_translation(Vec3::new(world_x, world_y, Z_LAYER_GRID + 0.1))
                    .with_scale(Vec3::splat(PLAYER_SCALE)), // 使用與玩家相同的縮放
                RoomTile { tile_type },
            ));
        }
    }
}

// 走廊連接系統
fn generate_corridors(
    commands: &mut Commands,
    asset_server: &AssetServer,
    compound_room: &CompoundRoom,
) {
    if compound_room.rectangles.len() < 2 {
        return; // 不需要走廊
    }
    
    let room_assets = RoomAssets::load_all(asset_server);
    let tile_size = ROOM_TILE_SIZE * PLAYER_SCALE;
    
    // 為每對相鄰房間創建連接
    match compound_room.room_type {
        CompoundRoomType::LShape => {
            create_l_shape_corridor(commands, &room_assets, tile_size, &compound_room.rectangles);
        },
        CompoundRoomType::TShape => {
            create_t_shape_corridor(commands, &room_assets, tile_size, &compound_room.rectangles);
        },
        CompoundRoomType::Plus => {
            create_plus_shape_corridors(commands, &room_assets, tile_size, &compound_room.rectangles);
        },
        _ => {}
    }
}

// L 形房間走廊
fn create_l_shape_corridor(
    commands: &mut Commands,
    room_assets: &RoomAssets,
    tile_size: f32,
    rectangles: &[RoomRect],
) {
    if rectangles.len() != 2 {
        return;
    }
    
    let main_rect = &rectangles[0];  // 主房間（垂直）
    let ext_rect = &rectangles[1];   // 擴展房間（水平）
    
    // 在兩個房間的連接處移除牆壁，創建開口
    // 找到重疊區域的中心位置
    let corridor_x = main_rect.x + main_rect.width as i32 - 1; // 主房間右邊界
    let corridor_y = ext_rect.y + 1; // 擴展房間內部
    
    // 移除牆壁，用地板取代（簡單版本：直接放地板）
    let world_x = corridor_x as f32 * tile_size;
    let world_y = corridor_y as f32 * tile_size;
    
    commands.spawn((
        Sprite::from_image(room_assets.floor_indoor.clone()),
        Transform::from_translation(Vec3::new(world_x, world_y, Z_LAYER_GRID + 0.2)) // 更高層級覆蓋牆壁
            .with_scale(Vec3::splat(PLAYER_SCALE)),
        RoomTile { tile_type: RoomTileType::Floor },
    ));
}

// T 形房間走廊
fn create_t_shape_corridor(
    commands: &mut Commands,
    room_assets: &RoomAssets,
    tile_size: f32,
    rectangles: &[RoomRect],
) {
    if rectangles.len() != 2 {
        return;
    }
    
    let _top_rect = &rectangles[0];    // 上橫梁
    let stem_rect = &rectangles[1];   // 下豎梁
    
    // 在上橫梁和下豎梁的連接處創建開口
    let corridor_x = stem_rect.x + stem_rect.width as i32 / 2; // 豎梁中心
    let corridor_y = stem_rect.y + stem_rect.height as i32 - 1; // 豎梁上部
    
    let world_x = corridor_x as f32 * tile_size;
    let world_y = corridor_y as f32 * tile_size;
    
    commands.spawn((
        Sprite::from_image(room_assets.floor_indoor.clone()),
        Transform::from_translation(Vec3::new(world_x, world_y, Z_LAYER_GRID + 0.2))
            .with_scale(Vec3::splat(PLAYER_SCALE)),
        RoomTile { tile_type: RoomTileType::Floor },
    ));
}

// 十字形房間走廊
fn create_plus_shape_corridors(
    commands: &mut Commands,
    room_assets: &RoomAssets,
    tile_size: f32,
    rectangles: &[RoomRect],
) {
    if rectangles.len() != 5 {
        return;
    }
    
    let center_rect = &rectangles[0]; // 中心區域
    
    // 為每個臂膀創建到中心的連接
    for arm_rect in &rectangles[1..] {
        let (corridor_x, corridor_y) = get_connection_point(center_rect, arm_rect);
        
        let world_x = corridor_x as f32 * tile_size;
        let world_y = corridor_y as f32 * tile_size;
        
        commands.spawn((
            Sprite::from_image(room_assets.floor_indoor.clone()),
            Transform::from_translation(Vec3::new(world_x, world_y, Z_LAYER_GRID + 0.2))
                .with_scale(Vec3::splat(PLAYER_SCALE)),
            RoomTile { tile_type: RoomTileType::Floor },
        ));
    }
}

// 獲取兩個房間之間的連接點
fn get_connection_point(center: &RoomRect, arm: &RoomRect) -> (i32, i32) {
    let center_cx = center.x + center.width as i32 / 2;
    let center_cy = center.y + center.height as i32 / 2;
    let arm_cx = arm.x + arm.width as i32 / 2;
    let arm_cy = arm.y + arm.height as i32 / 2;
    
    // 根據臂膀的相對位置決定連接點
    if arm_cx == center_cx {
        // 上下臂膀
        if arm_cy > center_cy {
            // 上臂
            (center_cx, center.y + center.height as i32 - 1)
        } else {
            // 下臂
            (center_cx, center.y)
        }
    } else {
        // 左右臂膀
        if arm_cx > center_cx {
            // 右臂
            (center.x + center.width as i32 - 1, center_cy)
        } else {
            // 左臂
            (center.x, center_cy)
        }
    }
}

// L 形房間生成器
fn generate_l_shape_room(rng: &mut impl Rng) -> CompoundRoom {
    let main_width = rng.gen_range(6..10);
    let main_height = rng.gen_range(8..12);
    let extension_width = rng.gen_range(5..9);
    let extension_height = rng.gen_range(4..7);
    
    // 確保擴展部分與主房間有足夠重疊
    let extension_height = extension_height.min(main_height - 1);
    
    // 主房間（垂直部分）
    let main_rect = RoomRect {
        x: -(main_width as i32) / 2,
        y: -(main_height as i32) / 2,
        width: main_width,
        height: main_height,
    };
    
    // 擴展房間（水平部分）- 與主房間重疊 1 格確保連通
    let extension_rect = RoomRect {
        x: main_rect.x + (main_width as i32) - 1, // 重疊 1 格
        y: main_rect.y,
        width: extension_width + 1, // 加 1 格用於重疊
        height: extension_height,
    };
    
    CompoundRoom {
        rectangles: vec![main_rect, extension_rect],
        room_type: CompoundRoomType::LShape,
    }
}

// T 形房間生成器
fn generate_t_shape_room(rng: &mut impl Rng) -> CompoundRoom {
    let main_width = rng.gen_range(10..14);
    let main_height = rng.gen_range(4..6);
    let stem_width = rng.gen_range(4..7);
    let stem_height = rng.gen_range(6..10);
    
    // 確保豎梁寬度不超過橫梁寬度
    let stem_width = stem_width.min(main_width - 2);
    
    // 上橫梁（T 形的橫向部分）
    let top_rect = RoomRect {
        x: -(main_width as i32) / 2,
        y: (stem_height as i32) / 2 - 1, // 與豎梁重疊 1 格確保連通
        width: main_width,
        height: main_height,
    };
    
    // 下豎梁（T 形的豎向部分）
    let stem_rect = RoomRect {
        x: -(stem_width as i32) / 2,
        y: -(stem_height as i32) / 2,
        width: stem_width,
        height: stem_height,
    };
    
    CompoundRoom {
        rectangles: vec![top_rect, stem_rect],
        room_type: CompoundRoomType::TShape,
    }
}

// 十字形房間生成器
fn generate_plus_shape_room(rng: &mut impl Rng) -> CompoundRoom {
    let center_width = rng.gen_range(6..8);
    let center_height = rng.gen_range(6..8);
    let arm_length = rng.gen_range(4..7);
    let arm_width = rng.gen_range(3..5);
    
    // 確保臂膀寬度不超過中心區域
    let arm_width = arm_width.min(center_width - 1).min(center_height - 1);
    
    // 中心區域
    let center_rect = RoomRect {
        x: -(center_width as i32) / 2,
        y: -(center_height as i32) / 2,
        width: center_width,
        height: center_height,
    };
    
    // 上臂 - 與中心重疊 1 格確保連通
    let top_arm = RoomRect {
        x: -(arm_width as i32) / 2,
        y: (center_height as i32) / 2 - 1,
        width: arm_width,
        height: arm_length + 1,
    };
    
    // 下臂 - 與中心重疊 1 格確保連通
    let bottom_arm = RoomRect {
        x: -(arm_width as i32) / 2,
        y: -((center_height as i32) / 2 + (arm_length as i32)),
        width: arm_width,
        height: arm_length + 1,
    };
    
    // 左臂 - 與中心重疊 1 格確保連通
    let left_arm = RoomRect {
        x: -((center_width as i32) / 2 + (arm_length as i32)),
        y: -(arm_width as i32) / 2,
        width: arm_length + 1,
        height: arm_width,
    };
    
    // 右臂 - 與中心重疊 1 格確保連通
    let right_arm = RoomRect {
        x: (center_width as i32) / 2 - 1,
        y: -(arm_width as i32) / 2,
        width: arm_length + 1,
        height: arm_width,
    };
    
    CompoundRoom {
        rectangles: vec![center_rect, top_arm, bottom_arm, left_arm, right_arm],
        room_type: CompoundRoomType::Plus,
    }
}

// 複合房間生成函數 - 使用走廊連接方法
fn spawn_compound_room(
    commands: &mut Commands,
    asset_server: &AssetServer,
    compound_room: CompoundRoom,
) {
    // 1. 為每個矩形生成完整房間
    for rect in &compound_room.rectangles {
        generate_room_tiles(
            commands,
            asset_server,
            rect.width,
            rect.height,
            rect.x,
            rect.y,
        );
    }
    
    // 2. 生成連接走廊
    generate_corridors(commands, asset_server, &compound_room);
    
    // 創建複合房間實體
    commands.spawn((
        compound_room,
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Visibility::Visible,
    ));
}