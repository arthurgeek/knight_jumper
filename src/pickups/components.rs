use bevy::prelude::*;

/// Marker component for collectible coins.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Coin;
