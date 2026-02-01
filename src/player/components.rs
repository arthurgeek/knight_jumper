use super::resources::KnightAtlas;
use avian2d::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    sprite::Anchor,
};
use std::time::Duration;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name = "Player",
    Sprite,
    Anchor = Anchor::from(Vec2::new(0.0, -0.175)),
    AnimationState,
    AnimationConfig = AnimationConfig::new(0, 3, 10, true),
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

#[derive(Component, Default)]
pub struct WallContactLeft;

#[derive(Component, Default)]
pub struct WallContactRight;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed(pub f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct JumpVelocity(pub f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub enum AnimationState {
    #[default]
    Idle,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub looping: bool,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8, looping: bool) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            looping,
            frame_timer: Self::timer_from_fps(fps, looping),
        }
    }

    pub fn timer_from_fps(fps: u8, looping: bool) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / fps as f32),
            if looping {
                TimerMode::Repeating
            } else {
                TimerMode::Once
            },
        )
    }
}
