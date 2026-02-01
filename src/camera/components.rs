use bevy::prelude::*;

#[derive(Component)]
pub struct FollowCamera {
    pub smoothing_speed: f32,
    /// Camera won't go below this Y value (smoothed, set from map at runtime)
    pub limit_bottom: f32,
}

impl Default for FollowCamera {
    fn default() -> Self {
        Self {
            smoothing_speed: 5.0,
            limit_bottom: -120.0,
        }
    }
}

/// Marker: camera has snapped to player at least once.
#[derive(Component)]
pub struct CameraInitialized;
