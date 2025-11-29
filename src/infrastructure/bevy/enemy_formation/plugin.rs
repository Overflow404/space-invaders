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
    sync_enemy_formation_state_system,
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
                    sync_enemy_formation_state_system,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy::components::EnemyComponent;
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::input::ButtonInput;
    use bevy::prelude::{AssetApp, KeyCode};
    use bevy::MinimalPlugins;
    use bevy_test::{count_components, get_resource, get_update_systems};

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .add_plugins(EnemyFormationPlugin)
            .init_asset::<Image>()
            .init_resource::<ButtonInput<KeyCode>>();

        app.update();
        app
    }

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = setup();

        let formation_resource = get_resource::<EnemyFormationResource>(&mut app);
        assert!(!formation_resource.0.is_annihilated());

        let formation_timer = get_resource::<EnemyFormationMovementTimer>(&mut app);
        assert_eq!(
            formation_timer.0.duration().as_secs_f32(),
            ENEMY_FORMATION_STEP_DURATION
        );
        assert_eq!(formation_timer.0.mode(), TimerMode::Repeating);

        let fire_probability = get_resource::<EnemyFireProbability>(&mut app);
        assert_eq!(fire_probability.0, ENEMY_FIRE_PROBABILITY);

        assert_eq!(get_update_systems(&app).count(), 5);

        let enemy_count = count_components::<EnemyComponent>(&mut app);
        assert_eq!(enemy_count, 55);
    }
}
