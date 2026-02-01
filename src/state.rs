use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    Reloading,
}

/// Immediately transition back to Playing.
pub fn restart_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}
