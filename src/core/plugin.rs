use super::components::{Speed, SpriteAnimation};
use super::systems::animate_sprites;
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Speed>()
            .register_type::<SpriteAnimation>()
            .add_systems(Update, animate_sprites);
    }
}
