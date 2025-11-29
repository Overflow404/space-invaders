use crate::infrastructure::bevy::player_projectile::components::DespawnPlayerProjectileMessage;
use crate::infrastructure::bevy::player_projectile::resources::{
    PlayerProjectileMovementTimerResource, PLAYER_PROJECTILE_DURATION,
};
use crate::infrastructure::bevy::player_projectile::systems::{
    player_projectile_despawn_system, player_projectile_lifecycle_system,
    player_projectile_movement_system,
};
use bevy::app::{App, Plugin};
use bevy::prelude::{Timer, TimerMode, Update};

pub struct PlayerProjectilePlugin;

impl Plugin for PlayerProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerProjectileMovementTimerResource(Timer::from_seconds(
            PLAYER_PROJECTILE_DURATION,
            TimerMode::Once,
        )))
        .add_message::<DespawnPlayerProjectileMessage>()
        .add_systems(
            Update,
            (
                player_projectile_movement_system,
                player_projectile_lifecycle_system,
                player_projectile_despawn_system,
            ),
        );
    }
}
