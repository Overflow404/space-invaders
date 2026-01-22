use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
use crate::infrastructure::bevy::game_area::resources::GAME_AREA_HEIGHT;
use crate::infrastructure::bevy::player_projectile::components::{
    PlayerProjectileComponent, PlayerProjectileExpiredMessage,
};
use crate::infrastructure::bevy::player_projectile::resources::{
    PlayerProjectileMovementTimerResource, PLAYER_PROJECTILE_SPEED,
};
use bevy::prelude::{
    Commands, Entity, MessageReader, MessageWriter, Query, Res, ResMut, Time, Transform, With,
};

pub fn player_projectile_movement_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerProjectileComponent>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y += PLAYER_PROJECTILE_SPEED * time.delta_secs();
    }
}

pub fn player_projectile_lifecycle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PlayerProjectileMovementTimerResource>,
    query: Query<(Entity, &mut Transform), With<PlayerProjectileComponent>>,
    mut message_writer: MessageWriter<PlayerProjectileExpiredMessage>,
    mut message_reader: MessageReader<EnemyKilledMessage>,
) {
    timer.0.tick(time.delta());

    for message in message_reader.read() {
        commands.entity(message.projectile_entity).despawn();
        return;
    }

    let top_bound = GAME_AREA_HEIGHT / 2.0;
    let mut reset_needed = false;

    for (entity, transform) in query.iter() {
        if transform.translation.y > top_bound {
            commands.entity(entity).despawn();
            reset_needed = true;
        }
    }

    if timer.0.just_finished() {
        for (entity, _) in query.iter() {
            if !reset_needed {
                commands.entity(entity).despawn();
                reset_needed = true;
            }
        }
    }

    if reset_needed {
        message_writer.write(PlayerProjectileExpiredMessage);
        timer.0.finish();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
    use crate::infrastructure::bevy::game_area::resources::GAME_AREA_HEIGHT;
    use crate::infrastructure::bevy::player_projectile::components::{
        PlayerProjectileComponent, PlayerProjectileExpiredMessage,
    };
    use crate::infrastructure::bevy::player_projectile::resources::{
        PlayerProjectileMovementTimerResource, PLAYER_PROJECTILE_SPEED,
    };
    use bevy::app::{App, Update};
    use bevy::prelude::{Timer, TimerMode, Transform};
    use bevy_test::{
        advance_time_by_seconds, contains_entity, did_component_despawn, did_message_fire,
        get_component_or_fail, send_message, spawn_dummy_entity, TestAppBuilder,
    };

    fn setup() -> App {
        TestAppBuilder::with_time_disabled()
            .with_time()
            .with_setup(|app| {
                app.add_message::<PlayerProjectileExpiredMessage>()
                    .add_message::<EnemyKilledMessage>()
                    .insert_resource(PlayerProjectileMovementTimerResource(Timer::from_seconds(
                        1.0,
                        TimerMode::Once,
                    )));
            })
            .build()
    }

    #[cfg(test)]
    mod player_projectile_lifecycle_system {
        use super::*;
        use bevy_test::get_resource_mut_or_fail;

        #[test]
        fn should_despawn_when_enemy_is_killed() {
            let mut app = setup();
            app.add_systems(Update, player_projectile_lifecycle_system);

            let enemy_entity = spawn_dummy_entity(&mut app);
            let player_projectile_entity = spawn_dummy_entity(&mut app);

            send_message(
                &mut app,
                EnemyKilledMessage::new(enemy_entity, 1, player_projectile_entity),
            );

            app.update();

            assert!(!contains_entity(&app, player_projectile_entity));
        }

        #[test]
        fn should_notify_and_despawn_when_out_of_bounds() {
            let mut app = setup();
            app.add_systems(Update, player_projectile_lifecycle_system);

            let out_of_bounds_y = (GAME_AREA_HEIGHT / 2.0) + 10.0;

            app.world_mut().spawn((
                PlayerProjectileComponent,
                Transform::from_xyz(0.0, out_of_bounds_y, 0.0),
            ));

            advance_time_by_seconds(&mut app, 0.01);
            app.update();

            let timer = get_resource_mut_or_fail::<PlayerProjectileMovementTimerResource>(&mut app);
            assert!(timer.0.is_finished());

            assert!(did_component_despawn::<PlayerProjectileComponent>(&mut app));
            assert!(did_message_fire::<PlayerProjectileExpiredMessage>(&mut app));
        }

        #[test]
        fn should_notify_and_despawn_when_timer_finishes() {
            let mut app = setup();
            app.add_systems(Update, player_projectile_lifecycle_system);

            app.world_mut().spawn((
                PlayerProjectileComponent,
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));

            advance_time_by_seconds(&mut app, 2.0);
            app.update();

            let timer = get_resource_mut_or_fail::<PlayerProjectileMovementTimerResource>(&mut app);
            assert!(timer.0.is_finished());

            assert!(did_component_despawn::<PlayerProjectileComponent>(&mut app));
            assert!(did_message_fire::<PlayerProjectileExpiredMessage>(&mut app));
        }
    }

    #[cfg(test)]
    mod player_projectile_movement_system {
        use super::*;

        #[test]
        fn should_move_projectile_upwards() {
            let mut app = setup();
            app.add_systems(Update, player_projectile_movement_system);

            let projectile = app
                .world_mut()
                .spawn((
                    PlayerProjectileComponent,
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ))
                .id();

            let delta_time = 0.1;
            advance_time_by_seconds(&mut app, delta_time);
            app.update();

            let transform = get_component_or_fail::<Transform>(&mut app, projectile);
            let expected_y = PLAYER_PROJECTILE_SPEED * delta_time;

            assert!((transform.translation.y - expected_y).abs() < 0.001);
        }
    }
}
