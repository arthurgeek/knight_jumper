use super::components::{Music, Sfx};
use super::resources::SfxHandles;
use crate::pickups::messages::CoinCollected;
use bevy::audio::Volume;
use bevy::prelude::*;

/// Loads sound effects into a resource.
pub fn load_sfx(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SfxHandles {
        coin: asset_server.load("sounds/coin.wav"),
    });
}

/// Spawns background music that loops forever.
pub fn spawn_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    // -12 dB ≈ 0.25 linear
    commands.spawn((
        Name::new("Background Music"),
        Music,
        AudioPlayer::new(asset_server.load("music/time_for_adventure.mp3")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.25)),
    ));
}

/// Plays coin pickup sound when CoinCollected is received.
pub fn play_coin_sound(
    mut messages: MessageReader<CoinCollected>,
    mut commands: Commands,
    sfx: Res<SfxHandles>,
) {
    for _ in messages.read() {
        commands.spawn((Sfx, AudioPlayer::new(sfx.coin.clone())));
    }
}
