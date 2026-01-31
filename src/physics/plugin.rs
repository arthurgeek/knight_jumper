use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use super::systems::make_tile_colliders_static;
use crate::platform::PlatformHooks;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default().with_collision_hooks::<PlatformHooks>())
            .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default())
            .insert_resource(Gravity(Vec2::new(0.0, -980.0)))
            .add_observer(make_tile_colliders_static);
    }
}
