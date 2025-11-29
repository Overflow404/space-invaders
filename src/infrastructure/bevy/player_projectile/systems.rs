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
) {
    timer.0.tick(time.delta());
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
pub fn player_projectile_despawn_system(
    mut commands: Commands,
    mut despawn_player_projectile_message: MessageReader<EnemyKilledMessage>,
) {
    for message in despawn_player_projectile_message.read() {
        commands.entity(message.projectile_entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::bevy::enemy::EnemyKilledMessage;
    use crate::infrastructure::bevy::game_area::GAME_AREA_HEIGHT;
    use crate::infrastructure::bevy::player_projectile::components::{
        PlayerProjectileComponent, PlayerProjectileExpiredMessage,
    };
    use crate::infrastructure::bevy::player_projectile::resources::PlayerProjectileMovementTimerResource;
    use crate::infrastructure::bevy::player_projectile::systems::{
        player_projectile_despawn_system, player_projectile_lifecycle_system,
        player_projectile_movement_system,
    };
    use bevy::app::{App, Update};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{MessageReader, Time, Timer, TimerMode, Transform};
    use bevy::MinimalPlugins;
    use std::time::Duration;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.init_resource::<Time>();
        app.add_message::<PlayerProjectileExpiredMessage>();
        app.insert_resource(PlayerProjectileMovementTimerResource(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )));

        app
    }

    #[test]
    fn should_despawn_after_hitting_the_enemy() {
        let mut app = setup();
        app.add_message::<EnemyKilledMessage>();
        app.add_systems(Update, player_projectile_despawn_system);

        let enemy_entity = app.world_mut().spawn_empty().id();
        let player_projectile_entity = app.world_mut().spawn_empty().id();
        assert!(app.world().get_entity(player_projectile_entity).is_ok());

        app.world_mut().write_message(EnemyKilledMessage::new(
            enemy_entity,
            1,
            player_projectile_entity,
        ));

        app.update();

        assert!(app.world().get_entity(player_projectile_entity).is_err());
    }

    #[test]
    fn should_move_projectile_upwards() -> bevy::prelude::Result<(), Box<dyn std::error::Error>> {
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

        let transform = app
            .world()
            .get::<Transform>(projectile)
            .ok_or("Cannot get transform")?;

        assert!(
            (transform.translation.y - 50.0).abs() < 0.001,
            "Projectile should have moved up by 50 units"
        );

        Ok(())
    }

    #[test]
    fn should_despawn_when_out_of_bounds() -> bevy::prelude::Result<(), Box<dyn std::error::Error>>
    {
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

        let count = app
            .world_mut()
            .query::<&PlayerProjectileComponent>()
            .iter(app.world())
            .len();
        assert_eq!(count, 0, "Projectile should be despawned");

        app.world_mut()
            .run_system_once(
                move |mut reader: MessageReader<PlayerProjectileExpiredMessage>| {
                    let mut iterator = reader.read();

                    iterator.next().unwrap_or_else(|| {
                        panic!("Player projectile expired message did not arrive!")
                    });
                },
            )
            .map_err(|e| format!("Cannot run system: {e}"))?;

        Ok(())
    }

    #[test]
    fn should_despawn_when_timer_finishes() -> bevy::prelude::Result<(), Box<dyn std::error::Error>>
    {
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

        let count = app
            .world_mut()
            .query::<&PlayerProjectileComponent>()
            .iter(app.world())
            .len();

        assert_eq!(count, 0, "Projectile should be despawned due to timeout");

        app.world_mut()
            .run_system_once(
                move |mut reader: MessageReader<PlayerProjectileExpiredMessage>| {
                    let mut iterator = reader.read();

                    iterator.next().unwrap_or_else(|| {
                        panic!("Player projectile expired message did not arrive!")
                    });
                },
            )
            .map_err(|e| format!("Cannot run system: {e}"))?;

        Ok(())
    }
}
