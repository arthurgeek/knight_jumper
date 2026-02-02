use super::components::{Score, SpriteAnimation};
use crate::pickups::messages::CoinCollected;
use bevy::prelude::*;

/// Ticks sprite animations and advances frames.
pub fn animate_sprites(time: Res<Time>, mut query: Query<(&mut SpriteAnimation, &mut Sprite)>) {
    for (mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index >= anim.last {
                anim.first
            } else {
                atlas.index + 1
            };
        }
    }
}

/// Increments score when coins are collected.
pub fn increment_score(mut messages: MessageReader<CoinCollected>, mut score: ResMut<Score>) {
    for _ in messages.read() {
        score.0 += 1;
    }
}
