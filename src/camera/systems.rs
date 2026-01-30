use super::components::FollowCamera;
use crate::player::Player;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    // Spawn a 2D camera with proper scaling
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::splat(0.5)),
        FollowCamera {
            smoothing_speed: 5.0,
        },
    ));
}

pub fn follow_player(
    time: Res<Time>,
    player: Query<&Transform, (Without<Camera2d>, With<Player>)>,
    mut camera: Query<(&mut Transform, &FollowCamera), With<Camera2d>>,
    mut initialized: Local<bool>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let Ok((mut cam_transform, follow)) = camera.single_mut() else {
        return;
    };

    let target = player_transform.translation.truncate();

    // Snap to player on first frame, then smooth follow
    if !*initialized {
        cam_transform.translation.x = target.x;
        cam_transform.translation.y = target.y;
        *initialized = true;
        return;
    }

    let current = cam_transform.translation.truncate();
    let new_pos = current.lerp(target, follow.smoothing_speed * time.delta_secs());
    cam_transform.translation.x = new_pos.x;
    cam_transform.translation.y = new_pos.y;
}
