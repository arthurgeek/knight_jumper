use super::components::{Enemy, Slime};
use super::systems::{spawn_slime_at_spawn_point, update_patrol_movement};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .register_type::<Slime>()
            .add_systems(Update, spawn_slime_at_spawn_point)
            .add_systems(FixedUpdate, update_patrol_movement);
    }
}
