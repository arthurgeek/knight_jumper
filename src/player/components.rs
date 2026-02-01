use super::resources::KnightAtlas;
use crate::core::components::{Speed, SpriteAnimation};
use avian2d::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    sprite::Anchor,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name = "Player",
    Sprite,
    Anchor = Anchor::from(Vec2::new(0.0, -0.175)),
    SpriteAnimation = SpriteAnimation::new(0, 3, 10),  // Idle animation
    RigidBody::Dynamic,
    Collider = Collider::capsule(3.0, 5.0),
    ShapeCaster = ShapeCaster::new(Collider::capsule(2.97, 4.95), Vec2::ZERO, 0.0, Dir2::NEG_Y).with_max_distance(2.0).with_max_hits(10),
    Friction::ZERO,
    LockedAxes = LockedAxes::ROTATION_LOCKED,
    Speed = Speed(130.0),
    JumpVelocity = JumpVelocity(300.0),
  )]
#[component(on_add = Self::on_add)]
pub struct Player;

impl Player {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;

        // Get the atlas resource
        let atlas = world
            .get_resource::<KnightAtlas>()
            .expect("KnightAtlas resource must be present");

        let texture = atlas.texture.clone();
        let layout = atlas.layout.clone();

        // Configure the sprite
        if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
            sprite.image = texture;
            sprite.texture_atlas = Some(TextureAtlas { layout, index: 0 });
        }
    }
}

#[derive(Component, Default)]
pub struct Grounded;

/// Allows jumping for a short window after leaving ground.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CoyoteTimer(pub Timer);

impl Default for CoyoteTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

/// Buffers jump input so pressing jump slightly before landing still works.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct JumpBuffer(pub Timer);

impl Default for JumpBuffer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

/// Velocity inherited from the platform the player is standing on.
#[derive(Component, Default)]
pub struct PlatformVelocity(pub Vec2);

#[derive(Component, Default)]
pub struct WallContactLeft;

#[derive(Component, Default)]
pub struct WallContactRight;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct JumpVelocity(pub f32);
