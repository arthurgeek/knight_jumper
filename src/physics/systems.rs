use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

/// Observer that adds RigidBody::Static to all Tiled colliders.
pub fn make_colliders_static(
    collider_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
) {
    commands
        .entity(collider_created.event().origin)
        .insert(RigidBody::Static);
}
