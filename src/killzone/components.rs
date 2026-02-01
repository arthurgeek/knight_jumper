use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct KillZone;

#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct DeathTimer(pub Timer);

impl Default for DeathTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.6, TimerMode::Once))
    }
}
