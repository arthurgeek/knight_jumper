use bevy::prelude::*;

/// Preloaded sound effect handles.
#[derive(Resource)]
pub struct SfxHandles {
    pub coin: Handle<AudioSource>,
}
