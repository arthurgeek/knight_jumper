use avian2d::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    /// Walls, platforms, ground - things that block movement
    Wall,
}
