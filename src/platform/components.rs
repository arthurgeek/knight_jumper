use super::resources::PlatformTexture;
use avian2d::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

fn setup_platform_sprite(mut world: DeferredWorld, ctx: HookContext) {
    let entity = ctx.entity;

    let texture = world
        .get_resource::<PlatformTexture>()
        .expect("PlatformTexture resource must be present")
        .texture
        .clone();

    if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
        sprite.image = texture;
        sprite.rect = Some(Rect::new(16.0, 0.0, 48.0, 9.0));
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(
    Sprite,
    RigidBody::Static,
    Collider::rectangle(32.0, 9.0),
    ActiveCollisionHooks::MODIFY_CONTACTS
)]
#[component(on_add = setup_platform_sprite)]
pub struct OneWayPlatform;

/// Tiled polyline for a moving platform.
/// First point = left edge at start, last point = right edge at end.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
#[require(
    Sprite,
    RigidBody::Kinematic,
    Collider::rectangle(32.0, 9.0),
    LinearVelocity,
    ActiveCollisionHooks::MODIFY_CONTACTS
)]
#[component(on_add = setup_platform_sprite)]
pub struct MovingPlatform {
    /// Speed in pixels per second
    pub speed: f32,
    /// Start position (center) - computed from polyline
    #[reflect(ignore)]
    pub start: Vec2,
    /// End position (center) - computed from polyline
    #[reflect(ignore)]
    pub end: Vec2,
    /// 1.0 = toward end, -1.0 = toward start
    #[reflect(ignore)]
    pub direction: f32,
}

impl Default for MovingPlatform {
    fn default() -> Self {
        Self {
            speed: 50.0,
            start: Vec2::ZERO,
            end: Vec2::ZERO,
            direction: 1.0,
        }
    }
}
