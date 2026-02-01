use super::components::SpriteAnimation;
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
