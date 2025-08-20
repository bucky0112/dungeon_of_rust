use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::camera::CameraFollow;

pub fn camera_follow_system(
    player_query: Query<&Transform, (With<Player>, Without<CameraFollow>)>,
    mut camera_query: Query<(&mut Transform, &CameraFollow), (With<CameraFollow>, Without<Player>)>,
    time: Res<Time>,
) {
    if let (Ok(player_transform), Ok((mut camera_transform, camera_follow))) = 
        (player_query.single(), camera_query.single_mut()) {
        
        let target_translation = player_transform.translation;
        let current_translation = camera_transform.translation;
        
        // 使用線性插值進行平滑跟隨，速度可配置
        let lerp_factor = camera_follow.speed * time.delta_secs();
        let lerp_factor = lerp_factor.min(1.0); // 確保不會超過 1.0
        
        // 保持 Z 座標不變，只跟隨 X 和 Y
        let new_translation = Vec3::new(
            current_translation.x.lerp(target_translation.x, lerp_factor),
            current_translation.y.lerp(target_translation.y, lerp_factor),
            current_translation.z, // 保持相機的 Z 位置
        );
        
        camera_transform.translation = new_translation;
    }
}