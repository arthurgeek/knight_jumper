use bevy::prelude::*;

#[derive(Message, Default)]
pub struct PlayerMovement {
    pub is_moving: bool,
    pub facing_left: bool,
}
