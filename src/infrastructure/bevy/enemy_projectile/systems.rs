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
    use super::*;
    use crate::infrastructure::bevy::enemy_projectile::components::EnemyProjectileComponent;
    use crate::infrastructure::bevy::enemy_projectile::resources::ENEMY_PROJECTILE_SPEED;
    use bevy::app::{App, PluginGroup, Update};
    use bevy::prelude::{Time, Transform};
    use bevy::time::TimePlugin;
    use bevy::MinimalPlugins;
    use bevy_test::{advance_time_by_seconds, get_component_or_fail};

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins.build().disable::<TimePlugin>())
            .init_resource::<Time>();
        app
    }

    #[cfg(test)]
    mod enemy_projectile_movement_system {
        use super::*;

        #[test]
        fn should_move_projectiles_downwards() {
            let mut app = setup();
            app.add_systems(Update, enemy_projectile_movement_system);

            let projectile = app
                .world_mut()
                .spawn((EnemyProjectileComponent, Transform::from_xyz(0.0, 0.0, 0.0)))
                .id();

            let delta_time = 0.1;
            advance_time_by_seconds(&mut app, delta_time);

            app.update();

            let transform = get_component_or_fail::<Transform>(&mut app, projectile);
            let expected_y = -ENEMY_PROJECTILE_SPEED * delta_time;

            assert!((transform.translation.y - expected_y).abs() < 0.001);
        }

        #[test]
        fn should_not_move_when_time_delta_is_zero() {
            let mut app = setup();
            app.add_systems(Update, enemy_projectile_movement_system);

            let projectile = app
                .world_mut()
                .spawn((
                    EnemyProjectileComponent,
                    Transform::from_xyz(0.0, 100.0, 0.0),
                ))
                .id();

            advance_time_by_seconds(&mut app, 0.0);

            app.update();

            let transform = get_component_or_fail::<Transform>(&mut app, projectile);
            assert_eq!(transform.translation.y, 100.0);
        }
    }
}
