use super::components::{CameraInitialized, FollowCamera};
use crate::player::Player;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::splat(0.5)),
        FollowCamera::default(),
    ));
}

/// Resets camera state so it snaps to player on next spawn.
pub fn reset_camera(mut commands: Commands, camera: Query<Entity, With<CameraInitialized>>) {
    for entity in &camera {
        commands.entity(entity).remove::<CameraInitialized>();
    }
}

pub fn follow_player(
    time: Res<Time>,
    player: Query<&Transform, (Without<Camera2d>, With<Player>)>,
    mut camera: Query<
        (
            Entity,
            &mut Transform,
            &FollowCamera,
            &Projection,
            Has<CameraInitialized>,
        ),
        With<Camera2d>,
    >,
    mut commands: Commands,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let Ok((cam_entity, mut cam_transform, follow, projection, initialized)) = camera.single_mut()
    else {
        return;
    };

    // Get half viewport height to clamp camera's bottom edge, not center
    // Must account for camera scale (0.5 = zoomed in, sees less world space)
    let half_height = match projection {
        Projection::Orthographic(ortho) => ortho.area.height() / 2.0 * cam_transform.scale.y,
        _ => 0.0,
    };
    let min_camera_y = follow.limit_bottom + half_height;

    // Target player position, clamped so camera bottom never goes below limit
    let mut target = player_transform.translation.truncate();
    target.y = target.y.max(min_camera_y);

    // Snap to player on first frame, then smooth follow
    if !initialized {
        cam_transform.translation.x = target.x;
        cam_transform.translation.y = target.y;
        commands.entity(cam_entity).insert(CameraInitialized);
        return;
    }

    let current = cam_transform.translation.truncate();
    let new_pos = current.lerp(target, follow.smoothing_speed * time.delta_secs());
    cam_transform.translation.x = new_pos.x;
    cam_transform.translation.y = new_pos.y;
}
