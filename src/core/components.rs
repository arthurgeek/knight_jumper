use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed(pub f32);

/// Simple looping sprite animation.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpriteAnimation {
    pub first: usize,
    pub last: usize,
    pub timer: Timer,
}

impl SpriteAnimation {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first,
            last,
            timer: Timer::new(
                Duration::from_secs_f32(1.0 / fps as f32),
                TimerMode::Repeating,
            ),
        }
    }
}
