use crate::infrastructure::bevy::enemy_projectile::components::{
    EnemyProjectileComponent, EnemyProjectileExpiredMessage,
};
use crate::infrastructure::bevy::enemy_projectile::resources::{
    EnemyProjectileMovementTimerResource, ENEMY_PROJECTILE_SPEED,
};
use crate::infrastructure::bevy::game_area::resources::GAME_AREA_HEIGHT;
use bevy::prelude::{Commands, Entity, MessageWriter, Query, Res, ResMut, Time, Transform, With};

pub fn enemy_projectile_movement_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<EnemyProjectileComponent>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= ENEMY_PROJECTILE_SPEED * time.delta_secs();
    }
}

pub fn enemy_projectile_lifecycle_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<EnemyProjectileComponent>>,
    time: Res<Time>,
    mut timer: ResMut<EnemyProjectileMovementTimerResource>,
    mut message_writer: MessageWriter<EnemyProjectileExpiredMessage>,
) {
    timer.0.tick(time.delta());
    let mut reset_needed = false;

    for (entity, transform) in query.iter_mut() {
        let out_of_bound_y = -GAME_AREA_HEIGHT / 2.0;
        if transform.translation.y < out_of_bound_y {
            commands.entity(entity).despawn();
            reset_needed = true;
        }
    }

    if reset_needed {
        message_writer.write(EnemyProjectileExpiredMessage);
        timer.0.finish();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy_projectile::components::{
        EnemyProjectileComponent, EnemyProjectileExpiredMessage,
    };
    use crate::infrastructure::bevy::enemy_projectile::resources::ENEMY_PROJECTILE_SPEED;
    use bevy::app::{App, Update};
    use bevy::prelude::{Time, Timer, TimerMode, Transform};
    use bevy_test::{advance_time_by_seconds, get_component_or_fail, minimal_app};

    fn setup() -> App {
        let mut app = minimal_app(true);
        app.init_resource::<Time>()
            .add_message::<EnemyProjectileExpiredMessage>()
            .insert_resource(EnemyProjectileMovementTimerResource(Timer::from_seconds(
                1.0,
                TimerMode::Once,
            )));
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

    #[cfg(test)]
    mod enemy_projectile_lifecycle_system {
        use crate::infrastructure::bevy::enemy_projectile::components::{
            EnemyProjectileBundle, EnemyProjectileComponent, EnemyProjectileExpiredMessage,
        };
        use crate::infrastructure::bevy::enemy_projectile::resources::EnemyProjectileMovementTimerResource;
        use crate::infrastructure::bevy::enemy_projectile::systems::enemy_projectile_lifecycle_system;
        use crate::infrastructure::bevy::enemy_projectile::systems::tests::setup;
        use crate::infrastructure::bevy::game_area::resources::GAME_AREA_HEIGHT;
        use bevy::app::Update;
        use bevy_test::{
            advance_time_by_seconds, did_component_despawn, did_message_fire, get_resource_mut,
        };

        #[test]
        fn should_notify_and_despawn_when_out_of_bound() {
            let mut app = setup();
            app.add_systems(Update, enemy_projectile_lifecycle_system);

            let out_of_bounds_y = -(GAME_AREA_HEIGHT / 2.0) - 10.0;
            let out_of_bound_projectile = EnemyProjectileBundle::new(0.0, out_of_bounds_y);
            app.world_mut().spawn(out_of_bound_projectile);

            advance_time_by_seconds(&mut app, 0.01);
            app.update();

            let timer = get_resource_mut::<EnemyProjectileMovementTimerResource>(&mut app);
            assert!(timer.0.is_finished());

            assert!(did_component_despawn::<EnemyProjectileComponent>(&mut app));
            assert!(did_message_fire::<EnemyProjectileExpiredMessage>(&mut app));
        }
    }
}
