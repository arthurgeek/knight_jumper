use crate::core::components::Score;
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    Reloading,
}

/// Resets game state and transitions back to Playing.
pub fn restart_game(mut next_state: ResMut<NextState<GameState>>, mut score: ResMut<Score>) {
    score.0 = 0;
    next_state.set(GameState::Playing);
}
