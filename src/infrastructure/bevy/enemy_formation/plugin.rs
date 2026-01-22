use crate::domain::enemy_formation::EnemyFormation;
use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
use crate::infrastructure::bevy::enemy::resources::{
    EnemyFireProbability, EnemyProjectileMovementTimer,
};
use crate::infrastructure::bevy::enemy_formation::resources::{
    ENEMY_FIRE_PROBABILITY, ENEMY_FORMATION_STEP_DURATION, EnemyFormationMovementTimer,
    EnemyFormationResource,
};
use crate::infrastructure::bevy::enemy_formation::systems::{
    collisions_system, enemy_formation_lifecycle_system, enemy_formation_movement_system,
    spawn_enemy_formation_system, spawn_random_projectiles_system,
};
use crate::infrastructure::bevy::enemy_projectile::resources::ENEMY_PROJECTILE_DURATION;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{Timer, TimerMode};

pub struct EnemyFormationPlugin;

impl Plugin for EnemyFormationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyFormationResource(EnemyFormation::new()))
            .insert_resource(EnemyFireProbability(ENEMY_FIRE_PROBABILITY))
            .insert_resource(EnemyFormationMovementTimer(Timer::from_seconds(
                ENEMY_FORMATION_STEP_DURATION,
                TimerMode::Repeating,
            )))
            .insert_resource(EnemyProjectileMovementTimer(Timer::from_seconds(
                ENEMY_PROJECTILE_DURATION,
                TimerMode::Repeating,
            )))
            .add_message::<EnemyKilledMessage>()
            .add_systems(Startup, spawn_enemy_formation_system)
            .add_systems(
                Update,
                (
                    enemy_formation_lifecycle_system,
                    collisions_system,
                    enemy_formation_movement_system,
                    spawn_random_projectiles_system,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_loads_successfully() {
        let _app = bevy_test::smoke_test_plugin_with_assets(EnemyFormationPlugin);
    }
}
