use bevy::prelude::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct KnightAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PlayerInput {
    pub movement_direction: f32,
    pub jump_requested: bool,
}
