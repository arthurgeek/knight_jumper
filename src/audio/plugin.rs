use super::components::{Music, Sfx};
use super::systems::{load_sfx, play_coin_sound, spawn_music};
use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Music>()
            .register_type::<Sfx>()
            .add_systems(Startup, (load_sfx, spawn_music))
            .add_systems(Update, play_coin_sound);
    }
}
