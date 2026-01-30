use bevy::prelude::*;

#[derive(Message, Default)]
pub struct PlayerMovement {
    pub is_moving: bool,
    pub is_grounded: bool,
    pub facing_left: bool,
}
