use bevy::prelude::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PlatformTexture {
    pub texture: Handle<Image>,
}
