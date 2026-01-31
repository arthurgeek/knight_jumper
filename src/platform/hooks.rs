use super::components::OneWayPlatform;
use avian2d::prelude::*;
use bevy::{
    ecs::system::{SystemParam, lifetimeless::Read},
    prelude::*,
};

#[derive(SystemParam)]
pub struct PlatformHooks<'w, 's> {
    one_way_platforms_query: Query<'w, 's, (Read<OneWayPlatform>, Read<GlobalTransform>)>,
}

impl CollisionHooks for PlatformHooks<'_, '_> {
    fn modify_contacts(&self, contacts: &mut ContactPair, _: &mut Commands) -> bool {
        // This is the contact modification hook, called after collision detection,
        // but before constraints are created for the solver. Mutable access to the ECS
        // is not allowed, but we can queue commands to perform deferred changes.

        // Differentiate between which normal of the manifold we should use
        enum RelevantNormal {
            Normal1,
            Normal2,
        }

        // First, figure out which entity is the one-way platform, and which is the other.
        // Choose the appropriate normal for pass-through depending on which is which.
        let (_, _, platform_transform, _, relevant_normal) =
            if let Ok((one_way_platform, platform_transform)) =
                self.one_way_platforms_query.get(contacts.collider1)
            {
                (
                    contacts.collider1,
                    one_way_platform,
                    platform_transform,
                    contacts.collider2,
                    RelevantNormal::Normal1,
                )
            } else if let Ok((one_way_platform, platform_transform)) =
                self.one_way_platforms_query.get(contacts.collider2)
            {
                (
                    contacts.collider2,
                    one_way_platform,
                    platform_transform,
                    contacts.collider1,
                    RelevantNormal::Normal2,
                )
            } else {
                // Neither is a one-way-platform, so accept the collision:
                // we're done here.
                return true;
            };

        // Get the manifold and check the normal
        for manifold in contacts.manifolds.iter() {
            // Get normal pointing toward the "other" entity
            let dominated_normal = match relevant_normal {
                RelevantNormal::Normal1 => manifold.normal, // points from collider1 to collider2
                RelevantNormal::Normal2 => -manifold.normal, // flip it
            };

            // Platform's "up" in world space (truncate to 2D)
            let platform_up = platform_transform.up().truncate();

            // If approaching from below (normal opposite to platform's up), allow pass-through
            if dominated_normal.dot(platform_up) < -0.7 {
                return false; // disable contact
            }
        }

        // Otherwise
        true
    }
}
