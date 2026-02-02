use super::components::Coin;
use super::messages::CoinCollected;
use crate::player::Player;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

/// Makes coin colliders into sensors when they're created by bevy_ecs_tiled.
pub fn setup_coin_sensors(
    mut collider_events: MessageReader<TiledEvent<ColliderCreated>>,
    coins: Query<(), With<Coin>>,
    mut commands: Commands,
) {
    for evt in collider_events.read() {
        if coins.get(*evt.event.collider_of).is_ok() {
            commands
                .entity(evt.origin)
                .insert((Sensor, CollisionEventsEnabled));
        }
    }
}

/// Collects coins when the player touches them.
pub fn collect_coins(
    mut collision_events: MessageReader<CollisionStart>,
    mut commands: Commands,
    mut coin_events: MessageWriter<CoinCollected>,
    coins: Query<(), With<Coin>>,
    players: Query<(), With<Player>>,
    collider_query: Query<&TiledColliderOf>,
) {
    for evt in collision_events.read() {
        // Check both colliders - coin could be either one depending on collision direction
        let (coin_entity, player_body) = match (
            collider_query
                .get(evt.collider1)
                .ok()
                .filter(|c| coins.contains(c.0)),
            collider_query
                .get(evt.collider2)
                .ok()
                .filter(|c| coins.contains(c.0)),
        ) {
            (Some(coin), None) => (Some(coin.0), evt.body2),
            (None, Some(coin)) => (Some(coin.0), evt.body1),
            _ => (None, None),
        };

        // Check if the other body is a player
        let is_player = player_body.is_some_and(|body| players.contains(body));

        if let Some(coin) = coin_entity
            && is_player
        {
            coin_events.write(CoinCollected);
            commands.entity(coin).despawn();
        }
    }
}
