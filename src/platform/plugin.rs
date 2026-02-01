use super::components::{MovingPlatform, OneWayPlatform};
use super::resources::PlatformTexture;
use super::systems::{
    load_platform_texture, setup_moving_platforms, spawn_platform_at_spawn_point,
    update_moving_platforms,
};
use bevy::prelude::*;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OneWayPlatform>()
            .register_type::<MovingPlatform>()
            .register_type::<PlatformTexture>()
            .add_systems(Startup, load_platform_texture)
            .add_systems(
                Update,
                (spawn_platform_at_spawn_point, setup_moving_platforms),
            )
            .add_systems(FixedUpdate, update_moving_platforms);
    }
}
