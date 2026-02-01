use super::components::{DeathTimer, KillZone};
use crate::player::Player;
use crate::state::GameState;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

/// Makes kill zone colliders into sensors when created by bevy_ecs_tiled.
pub fn setup_killzone_sensors(
    mut collider_events: MessageReader<TiledEvent<ColliderCreated>>,
    killzones: Query<(), With<KillZone>>,
    mut commands: Commands,
) {
    for evt in collider_events.read() {
        if killzones.get(*evt.event.collider_of).is_ok() {
            commands
                .entity(evt.origin)
                .insert((Sensor, CollisionEventsEnabled));
        }
    }
}

/// Starts death timer when player touches a kill zone.
pub fn detect_killzone_collision(
    mut collision_events: MessageReader<CollisionStart>,
    mut commands: Commands,
    killzones: Query<(), With<KillZone>>,
    players: Query<Entity, (With<Player>, Without<DeathTimer>)>,
    collider_query: Query<&TiledColliderOf>,
    mut time: ResMut<Time<Virtual>>,
) {
    for evt in collision_events.read() {
        // Check if either collider belongs to a kill zone (via TiledColliderOf parent)
        let killzone_via_parent = collider_query
            .get(evt.collider1)
            .ok()
            .filter(|c| killzones.contains(c.0))
            .or_else(|| {
                collider_query
                    .get(evt.collider2)
                    .ok()
                    .filter(|c| killzones.contains(c.0))
            });

        // Also check if collider entity itself has KillZone (for manually spawned enemies)
        let killzone_direct =
            killzones.contains(evt.collider1) || killzones.contains(evt.collider2);

        if killzone_via_parent.is_none() && !killzone_direct {
            continue;
        }

        // Check if the other body is the player (without a death timer already)
        let player_entity = [evt.body1, evt.body2]
            .into_iter()
            .flatten()
            .find(|&body| players.contains(body));

        if let Some(player) = player_entity {
            info!("Player hit kill zone! Starting death timer...");
            time.set_relative_speed(0.5);
            commands
                .entity(player)
                .remove::<Collider>()
                .insert(DeathTimer::default());
        }
    }
}

/// Ticks death timer and transitions to Reloading state when it expires.
pub fn tick_death_timer(
    time: Res<Time>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut death_query: Query<&mut DeathTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for mut timer in &mut death_query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            virtual_time.set_relative_speed(1.0);
            next_state.set(GameState::Reloading);
        }
    }
}
