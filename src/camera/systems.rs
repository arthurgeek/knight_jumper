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

#[cfg(feature = "debug")]
pub fn draw_camera_bounds(mut gizmos: Gizmos, camera: Query<&FollowCamera>) {
    let Ok(follow) = camera.single() else {
        return;
    };
    let red = Color::srgb(1.0, 0.0, 0.0);

    // Top
    gizmos.line_2d(
        Vec2::new(-1000.0, follow.limit_top),
        Vec2::new(1000.0, follow.limit_top),
        red,
    );
    // Bottom
    gizmos.line_2d(
        Vec2::new(-1000.0, follow.limit_bottom),
        Vec2::new(1000.0, follow.limit_bottom),
        red,
    );
    // Left
    gizmos.line_2d(
        Vec2::new(follow.limit_left, -1000.0),
        Vec2::new(follow.limit_left, 1000.0),
        red,
    );
    // Right
    gizmos.line_2d(
        Vec2::new(follow.limit_right, -1000.0),
        Vec2::new(follow.limit_right, 1000.0),
        red,
    );
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

    // Get half viewport size to clamp camera edges, not center
    // Must account for camera scale (0.5 = zoomed in, sees less world space)
    let (half_width, half_height) = match projection {
        Projection::Orthographic(ortho) => (
            ortho.area.width() / 2.0 * cam_transform.scale.x,
            ortho.area.height() / 2.0 * cam_transform.scale.y,
        ),
        _ => (0.0, 0.0),
    };

    // Clamp target so camera edges stay within bounds
    let mut target = player_transform.translation.truncate();
    target.x = target.x.clamp(
        follow.limit_left + half_width,
        follow.limit_right - half_width,
    );
    target.y = target.y.clamp(
        follow.limit_bottom + half_height,
        follow.limit_top - half_height,
    );

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
