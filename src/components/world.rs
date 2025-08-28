use bevy::prelude::*;

#[derive(Component)]
pub struct GridTile;

#[derive(Component, Debug)]
pub struct RoomTile {
    pub tile_type: RoomTileType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoomTileType {
    Floor,
    // 北牆（上方，面向玩家）
    WallNInnerCornerW,  // 北牆內側左角
    WallNInnerMid,      // 北牆內側中段
    WallNInnerCornerE,  // 北牆內側右角
    // 南牆（下方）
    WallSInnerCapL,     // 南牆內側左端
    WallSInnerMid,      // 南牆內側中段
    WallSInnerCapR,     // 南牆內側右端
    WallSOuterCapL,     // 南牆外側左端
    WallSOuterMid,      // 南牆外側中段
    WallSOuterCapR,     // 南牆外側右端
    // 左右側牆
    WallWSide,          // 西側牆
    WallESide,          // 東側牆
}

#[derive(Component, Debug)]
pub struct Room {
    pub width: usize,
    pub height: usize,
    pub x: i32,
    pub y: i32,
}

// 複合房間結構
#[derive(Debug, Clone)]
pub struct RoomRect {
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize,
}

#[derive(Component, Debug)]
pub struct CompoundRoom {
    pub rectangles: Vec<RoomRect>,
    pub room_type: CompoundRoomType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompoundRoomType {
    Rectangle,  // 基本矩形
    LShape,     // L 形
    TShape,     // T 形
    Plus,       // 十字形
    Custom,     // 自定義形狀
}