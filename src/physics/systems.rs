use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

/// Observer that adds RigidBody::Static to tile layer colliders.
pub fn make_tile_colliders_static(
    collider_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
) {
    if collider_created.event().event.source == TiledColliderSource::TilesLayer {
        commands
            .entity(collider_created.event().origin)
            .insert(RigidBody::Static);
    }
}
