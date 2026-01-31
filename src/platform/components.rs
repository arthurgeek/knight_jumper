use super::resources::PlatformTexture;
use avian2d::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(
    Sprite,
    RigidBody::Static,
    Collider::rectangle(32.0, 9.0),
    ActiveCollisionHooks::MODIFY_CONTACTS
)]
#[component(on_add = Self::on_add)]
pub struct OneWayPlatform;

impl OneWayPlatform {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;

        let atlas = world
            .get_resource::<PlatformTexture>()
            .expect("PlatformTexture resource must be present");

        let texture = atlas.texture.clone();

        if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
            sprite.image = texture;
            sprite.rect = Some(Rect::new(16.0, 0.0, 48.0, 9.0));
        }
    }
}

/// Tiled object configuration for spawning a moving platform.
/// Place in Tiled with type="MovingPlatformSpawn" and properties:
/// - "end_point": object reference to the destination point
/// - "duration": float for travel time (default 1.5)
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct MovingPlatformSpawn {
    /// Entity reference to the end point (auto-populated by bevy_ecs_tiled)
    pub end_point: Option<Entity>,
    /// Duration of one-way travel in seconds
    #[reflect(default = "MovingPlatformSpawn::default_duration")]
    pub duration: f32,
}

impl MovingPlatformSpawn {
    fn default_duration() -> f32 {
        1.5
    }
}

/// A platform that moves back and forth between two points
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    Sprite,
    RigidBody::Kinematic,
    Collider::rectangle(32.0, 9.0),
    ActiveCollisionHooks::MODIFY_CONTACTS
)]
#[component(on_add = Self::on_add)]
pub struct MovingPlatform {
    pub start: Vec2,
    pub end: Vec2,
    pub duration: f32,
    pub progress: f32,
    pub direction: f32, // 1.0 = forward, -1.0 = backward
}

impl MovingPlatform {
    pub fn new(start: Vec2, end: Vec2, duration: f32) -> Self {
        Self {
            start,
            end,
            duration,
            progress: 0.0,
            direction: 1.0,
        }
    }

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;

        let atlas = world
            .get_resource::<PlatformTexture>()
            .expect("PlatformTexture resource must be present");

        let texture = atlas.texture.clone();

        if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
            sprite.image = texture;
            sprite.rect = Some(Rect::new(16.0, 0.0, 48.0, 9.0));
        }
    }
}
