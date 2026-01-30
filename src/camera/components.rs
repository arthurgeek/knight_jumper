use bevy::prelude::*;

#[derive(Component)]
pub struct FollowCamera {
    pub smoothing_speed: f32,
}
