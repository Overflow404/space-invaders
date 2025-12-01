use crate::domain::enemy_formation::EnemyFormation;
use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
use crate::infrastructure::bevy::enemy::resources::{
    EnemyFireProbability, EnemyProjectileMovementTimer,
};
use crate::infrastructure::bevy::enemy_formation::resources::{
    EnemyFormationMovementTimer, EnemyFormationResource, ENEMY_FIRE_PROBABILITY,
    ENEMY_FORMATION_STEP_DURATION,
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
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::input::ButtonInput;
    use bevy::prelude::{AssetApp, KeyCode};
    use bevy_test::{contains_system, get_resource_or_fail, minimal_app};

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = minimal_app();

        app.add_plugins(AssetPlugin::default())
            .add_plugins(EnemyFormationPlugin)
            .init_asset::<Image>()
            .init_resource::<ButtonInput<KeyCode>>();

        app.update();

        get_resource_or_fail::<EnemyFormationResource>(&mut app);

        let fire_probability = get_resource_or_fail::<EnemyFireProbability>(&mut app);
        assert_eq!(fire_probability.0, 0.2);

        let formation_timer = get_resource_or_fail::<EnemyFormationMovementTimer>(&mut app);
        assert_eq!(formation_timer.0.duration().as_secs_f32(), 0.6);
        assert_eq!(formation_timer.0.mode(), TimerMode::Repeating);

        let formation_timer = get_resource_or_fail::<EnemyProjectileMovementTimer>(&mut app);
        assert_eq!(formation_timer.0.duration().as_secs_f32(), 1.2);
        assert_eq!(formation_timer.0.mode(), TimerMode::Repeating);

        assert!(contains_system(
            &app,
            Startup,
            "spawn_enemy_formation_system"
        ));
        assert!(contains_system(
            &app,
            Update,
            "enemy_formation_lifecycle_system"
        ));
        assert!(contains_system(&app, Update, "collisions_system"));
        assert!(contains_system(
            &app,
            Update,
            "enemy_formation_movement_system"
        ));
        assert!(contains_system(
            &app,
            Update,
            "spawn_random_projectiles_system"
        ));
    }
}
