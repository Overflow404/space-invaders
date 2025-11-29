use crate::infrastructure::bevy::enemy_projectile::components::EnemyProjectileComponent;
use crate::infrastructure::bevy::enemy_projectile::resources::ENEMY_PROJECTILE_SPEED;
use bevy::prelude::{Query, Res, Time, Transform, With};

pub fn enemy_projectile_movement_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<EnemyProjectileComponent>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= ENEMY_PROJECTILE_SPEED * time.delta_secs();
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::bevy::enemy::resources::EnemyProjectileMovementTimer;
    use bevy::app::App;
    use bevy::prelude::{Time, Timer, TimerMode};
    use bevy::MinimalPlugins;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.init_resource::<Time>();
        app.insert_resource(EnemyProjectileMovementTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )));

        app
    }

    #[cfg(test)]
    mod movement_system {
        use crate::infrastructure::bevy::enemy_projectile::components::EnemyProjectileComponent;
        use crate::infrastructure::bevy::enemy_projectile::systems::enemy_projectile_movement_system;
        use crate::infrastructure::bevy::enemy_projectile::systems::tests::setup;
        use bevy::prelude::Transform;
        use bevy_test::{advance_time_by_seconds, get_component, run_system};
        use std::error::Error;

        #[test]
        fn should_advance_projectiles_downwards() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            let projectile = app
                .world_mut()
                .spawn((EnemyProjectileComponent, Transform::from_xyz(0.0, 0.0, 0.0)))
                .id();

            advance_time_by_seconds(&mut app, 0.1);

            run_system(&mut app, enemy_projectile_movement_system)?;

            let transform = get_component::<Transform>(&mut app, projectile);

            assert!(
                (transform.translation.y.abs() - 50.0).abs() < 0.001,
                "Projectile should have moved down by 50 units"
            );

            Ok(())
        }
    }
}
