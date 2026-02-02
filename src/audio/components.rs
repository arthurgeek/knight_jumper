use bevy::prelude::*;

/// Marker for background music.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// Marker for sound effects.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Sfx;
