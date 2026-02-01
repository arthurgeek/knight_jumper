use super::components::{DeathTimer, KillZone};
use super::systems::{detect_killzone_collision, setup_killzone_sensors, tick_death_timer};
use bevy::prelude::*;

pub struct KillZonePlugin;

impl Plugin for KillZonePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<KillZone>()
            .register_type::<DeathTimer>()
            .add_systems(
                Update,
                (
                    setup_killzone_sensors,
                    detect_killzone_collision,
                    tick_death_timer,
                ),
            );
    }
}
