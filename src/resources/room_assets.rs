use bevy::prelude::*;

#[derive(Resource)]
pub struct RoomAssets {
    // Floor assets
    pub floor_indoor: Handle<Image>,
    
    // North wall assets (上方，面向玩家)
    pub wall_n_inner_corner_w: Handle<Image>,
    pub wall_n_inner_mid: Handle<Image>,
    pub wall_n_inner_corner_e: Handle<Image>,
    
    // South wall assets (下方)
    pub wall_s_inner_cap_l: Handle<Image>,
    pub wall_s_inner_mid: Handle<Image>,
    pub wall_s_inner_cap_r: Handle<Image>,
    pub wall_s_outer_cap_l: Handle<Image>,
    pub wall_s_outer_mid: Handle<Image>,
    pub wall_s_outer_cap_r: Handle<Image>,
    
    // Side wall assets (左右側)
    pub wall_w_side: Handle<Image>,
    pub wall_e_side: Handle<Image>,
    
    // Door assets (for future use)
    pub door_closed: Handle<Image>,
    pub door_open: Handle<Image>,
}

impl RoomAssets {
    pub fn load_all(asset_server: &AssetServer) -> Self {
        Self {
            // Load floor assets
            floor_indoor: asset_server.load("floors/floor_indoor.png"),
            
            // Load north wall assets
            wall_n_inner_corner_w: asset_server.load("walls/wall_N_inner_corner_W.png"),
            wall_n_inner_mid: asset_server.load("walls/wall_N_inner_mid.png"),
            wall_n_inner_corner_e: asset_server.load("walls/wall_N_inner_corner_E.png"),
            
            // Load south wall assets
            wall_s_inner_cap_l: asset_server.load("walls/wall_S_inner_cap_L.png"),
            wall_s_inner_mid: asset_server.load("walls/wall_S_inner_mid.png"),
            wall_s_inner_cap_r: asset_server.load("walls/wall_S_inner_cap_R.png"),
            wall_s_outer_cap_l: asset_server.load("walls/wall_S_outer_cap_L.png"),
            wall_s_outer_mid: asset_server.load("walls/wall_S_outer_mid.png"),
            wall_s_outer_cap_r: asset_server.load("walls/wall_S_outer_cap_R.png"),
            
            // Load side wall assets
            wall_w_side: asset_server.load("walls/wall_W_side.png"),
            wall_e_side: asset_server.load("walls/wall_E_side.png"),
            
            // Load door assets
            door_closed: asset_server.load("doors/door_closed.png"),
            door_open: asset_server.load("doors/door_open.png"),
        }
    }
}