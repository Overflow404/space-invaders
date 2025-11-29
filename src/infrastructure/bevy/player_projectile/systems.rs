use crate::infrastructure::bevy::enemy::EnemyKilledMessage;
use crate::infrastructure::bevy::game_area::GAME_AREA_HEIGHT;
use crate::infrastructure::bevy::player_projectile::components::{
    PlayerProjectileComponent, PlayerProjectileExpiredMessage,
};
use crate::infrastructure::bevy::player_projectile::resources::{
    PlayerProjectileMovementTimerResource, PROJECTILE_SPEED,
};
use bevy::prelude::{
    Commands, Entity, MessageReader, MessageWriter, Query, Res, ResMut, Time, Transform, With,
};

pub fn player_projectile_movement_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerProjectileComponent>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y += PROJECTILE_SPEED * time.delta_secs();
    }
}

pub fn player_projectile_lifecycle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PlayerProjectileMovementTimerResource>,
    query: Query<(Entity, &mut Transform), With<PlayerProjectileComponent>>,
    mut message_writer: MessageWriter<PlayerProjectileExpiredMessage>,
    mut despawn_player_projectile_message: MessageReader<EnemyKilledMessage>,
) {
    timer.0.tick(time.delta());

    for message in despawn_player_projectile_message.read() {
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
        timer.0.reset();
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::bevy::enemy::EnemyKilledMessage;
    use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileExpiredMessage;
    use crate::infrastructure::bevy::player_projectile::resources::PlayerProjectileMovementTimerResource;
    use bevy::app::App;
    use bevy::prelude::{Time, Timer, TimerMode};
    use bevy::MinimalPlugins;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.init_resource::<Time>();
        app.add_message::<PlayerProjectileExpiredMessage>();
        app.add_message::<EnemyKilledMessage>();
        app.insert_resource(PlayerProjectileMovementTimerResource(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )));

        app
    }

    #[cfg(test)]
    mod lifecycle_system {
        use crate::infrastructure::bevy::enemy::EnemyKilledMessage;
        use crate::infrastructure::bevy::game_area::GAME_AREA_HEIGHT;
        use crate::infrastructure::bevy::player_projectile::components::{
            PlayerProjectileComponent, PlayerProjectileExpiredMessage,
        };
        use crate::infrastructure::bevy::player_projectile::systems::player_projectile_lifecycle_system;
        use crate::infrastructure::bevy::player_projectile::systems::tests::setup;
        use bevy::app::Update;
        use bevy::ecs::system::RunSystemOnce;
        use bevy::prelude::{Time, Transform};
        use bevy_test::{component_despawned, contains_entity, verify_message_fired};
        use std::error::Error;
        use std::time::Duration;

        #[test]
        fn should_despawn_when_enemy_is_killed() {
            let mut app = setup();

            app.add_message::<EnemyKilledMessage>();
            app.add_systems(Update, player_projectile_lifecycle_system);

            let enemy_entity = app.world_mut().spawn_empty().id();
            let player_projectile_entity = app.world_mut().spawn_empty().id();

            assert!(contains_entity(&app, player_projectile_entity));

            app.world_mut().write_message(EnemyKilledMessage::new(
                enemy_entity,
                1,
                player_projectile_entity,
            ));

            app.update();

            assert!(!contains_entity(&app, player_projectile_entity));
        }

        #[test]
        fn should_despawn_when_out_of_bounds() -> Result<(), Box<dyn Error>> {
            let mut app = setup();
            app.add_systems(Update, player_projectile_lifecycle_system);

            let out_of_bounds_y = (GAME_AREA_HEIGHT / 2.0) + 10.0;

            app.world_mut().spawn((
                PlayerProjectileComponent,
                Transform::from_xyz(0.0, out_of_bounds_y, 0.0),
            ));

            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_secs_f32(0.01));

            app.update();

            assert!(component_despawned::<PlayerProjectileComponent>(&mut app));
            verify_message_fired::<PlayerProjectileExpiredMessage>(&mut app)?;

            Ok(())
        }

        #[test]
        fn should_despawn_when_timer_finishes() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            app.world_mut().spawn((
                PlayerProjectileComponent,
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));

            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_secs_f32(2.0));

            app.world_mut()
                .run_system_once(player_projectile_lifecycle_system)
                .map_err(|e| format!("Cannot run system: {e}"))?;

            assert!(component_despawned::<PlayerProjectileComponent>(&mut app));
            verify_message_fired::<PlayerProjectileExpiredMessage>(&mut app)?;

            Ok(())
        }
    }

    #[cfg(test)]
    mod movement_system {
        use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileComponent;
        use crate::infrastructure::bevy::player_projectile::systems::player_projectile_movement_system;
        use crate::infrastructure::bevy::player_projectile::systems::tests::setup;
        use bevy::ecs::system::RunSystemOnce;
        use bevy::prelude::{Time, Transform};
        use bevy_test::get_component;
        use std::error::Error;
        use std::time::Duration;

        #[test]
        fn should_move_projectile_upwards() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            let projectile = app
                .world_mut()
                .spawn((
                    PlayerProjectileComponent,
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ))
                .id();

            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_secs_f32(0.1));

            app.world_mut()
                .run_system_once(player_projectile_movement_system)
                .map_err(|e| format!("Cannot run system: {e}"))?;

            let transform = get_component::<Transform>(&mut app, projectile);

            assert!(
                (transform.translation.y - 50.0).abs() < 0.001,
                "Projectile should have moved up by 50 units"
            );

            Ok(())
        }
    }
}
