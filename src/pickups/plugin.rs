use super::components::Coin;
use super::systems::{collect_coins, setup_coin_sensors};
use bevy::prelude::*;

pub struct PickupsPlugin;

impl Plugin for PickupsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Coin>()
            .add_systems(Update, (setup_coin_sensors, collect_coins));
    }
}
