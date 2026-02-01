use bevy::prelude::*;

/// Marker for tip text entities spawned from Tiled.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TipText;
