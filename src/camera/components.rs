use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FollowCamera {
    pub smoothing_speed: f32,
    pub limit_bottom: f32,
    pub limit_top: f32,
    pub limit_left: f32,
    pub limit_right: f32,
}

impl Default for FollowCamera {
    fn default() -> Self {
        Self {
            smoothing_speed: 5.0,
            limit_bottom: -152.0,
            limit_top: 200.0,
            limit_left: -700.0,
            limit_right: 600.0,
        }
    }
}

/// Marker: camera has snapped to player at least once.
#[derive(Component)]
pub struct CameraInitialized;
