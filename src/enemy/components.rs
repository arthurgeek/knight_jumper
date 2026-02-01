use crate::core::components::{Speed, SpriteAnimation};
use crate::core::layers::GameLayer;
use crate::killzone::components::KillZone;
use avian2d::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    sprite::Anchor,
};

/// Marker for all enemy types.
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
#[require(
    Speed = Speed(30.0),
    RigidBody::Kinematic,
    LinearVelocity,
    Collider = Collider::rectangle(10.0, 12.0),
    Sensor,
    CollisionEventsEnabled,
    KillZone,
    RayCaster = RayCaster::new(Vec2::ZERO, Dir2::X)
        .with_max_hits(1)
        .with_max_distance(7.0)
        .with_query_filter(SpatialQueryFilter::from_mask(GameLayer::Wall)),
)]
pub struct Enemy;

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
#[require(
    Enemy,
    Sprite,
    Anchor = Anchor::from(Vec2::new(0.0, -0.25)),
    SpriteAnimation = SpriteAnimation::new(4, 7, 10),  // Second row, 10fps
)]
#[component(on_add = Self::on_add)]
pub struct Slime;

impl Slime {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;

        let asset_server = world.resource::<AssetServer>();
        let texture = asset_server.load("sprites/slime_green.png");

        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let layout = layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(24),
            4,
            3,
            None,
            None,
        ));

        if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
            sprite.image = texture;
            sprite.texture_atlas = Some(TextureAtlas {
                layout,
                index: 4, // Second row, first frame
            });
        }
    }
}
