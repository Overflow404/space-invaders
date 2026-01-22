use crate::infrastructure::bevy::enemy_projectile::components::{
    EnemyProjectileComponent, EnemyProjectileExpiredMessage, EnemyProjectileTimer,
    PlayerKilledMessage,
};
use crate::infrastructure::bevy::enemy_projectile::resources::ENEMY_PROJECTILE_SPEED;
use crate::infrastructure::bevy::game_area::resources::GAME_AREA_HEIGHT;
use crate::infrastructure::bevy::player::components::PlayerComponent;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Entity, MessageReader, MessageWriter, Query, Res, Sprite, Time, Transform, With,
};

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
    time: Res<Time>,
    mut query: Query<
        (Entity, &Transform, &mut EnemyProjectileTimer),
        With<EnemyProjectileComponent>,
    >,
    mut message_writer: MessageWriter<EnemyProjectileExpiredMessage>,
) {
    for (entity, transform, mut timer_component) in query.iter_mut() {
        let mut reset_needed = false;
        let out_of_bound_y = -GAME_AREA_HEIGHT / 2.0;

        if transform.translation.y < out_of_bound_y {
            reset_needed = true;
        }

        timer_component.0.tick(time.delta());

        if timer_component.0.is_finished() {
            reset_needed = true;
        }

        if reset_needed {
            commands.entity(entity).despawn();
            message_writer.write(EnemyProjectileExpiredMessage);
            timer_component.0.reset();
        }
    }
}

pub fn on_enemy_projectile_hitting_player_system(
    mut commands: Commands,
    mut player_killed_event_writer: MessageReader<PlayerKilledMessage>,
) {
    for msg in player_killed_event_writer.read() {
        commands.entity(msg.projectile_entity).despawn();
    }
}

pub fn collision_system(
    mut projectile_query: Query<(Entity, &Transform, &Sprite), With<EnemyProjectileComponent>>,
    player_query: Query<(&Transform, &Sprite, &PlayerComponent), With<PlayerComponent>>,
    mut player_killed_message_writer: MessageWriter<PlayerKilledMessage>,
) {
    for (player_transform, player_sprite, _) in player_query.iter() {
        for (projectile_entity, projectile_transform, projectile_sprite) in
            projectile_query.iter_mut()
        {
            let player_size = player_sprite.custom_size.unwrap_or(Vec2::ONE);
            let projectile_size = projectile_sprite.custom_size.unwrap_or(Vec2::ONE);

            let collision = projectile_transform.translation.x
                < player_transform.translation.x + player_size.x / 2.0
                && projectile_transform.translation.x + projectile_size.x
                    > player_transform.translation.x - player_size.x / 2.0
                && projectile_transform.translation.y
                    < player_transform.translation.y + player_size.y / 2.0
                && projectile_transform.translation.y + projectile_size.y
                    > player_transform.translation.y - player_size.y / 2.0;

            if collision {
                player_killed_message_writer.write(PlayerKilledMessage::new(projectile_entity));
                break;
            }
        }
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
    use bevy::prelude::Transform;
    use bevy_test::{TestAppBuilder, advance_time_by_seconds, get_component_or_fail};

    fn setup() -> App {
        TestAppBuilder::with_time_disabled()
            .with_time()
            .with_setup(|app| {
                app.add_message::<EnemyProjectileExpiredMessage>();
            })
            .build()
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
        use crate::infrastructure::bevy::enemy_projectile::systems::enemy_projectile_lifecycle_system;
        use crate::infrastructure::bevy::enemy_projectile::systems::tests::setup;
        use crate::infrastructure::bevy::game_area::resources::GAME_AREA_HEIGHT;
        use bevy::app::Update;
        use bevy_test::{advance_time_by_seconds, did_component_despawn, did_message_fire};

        #[test]
        fn should_notify_and_despawn_when_out_of_bound() {
            let mut app = setup();
            app.add_systems(Update, enemy_projectile_lifecycle_system);

            let out_of_bounds_y = -(GAME_AREA_HEIGHT / 2.0) - 10.0;
            let out_of_bound_projectile = EnemyProjectileBundle::new(0.0, out_of_bounds_y);
            app.world_mut().spawn(out_of_bound_projectile);

            advance_time_by_seconds(&mut app, 0.01);
            app.update();

            assert!(did_component_despawn::<EnemyProjectileComponent>(&mut app));
            assert!(did_message_fire::<EnemyProjectileExpiredMessage>(&mut app));
        }

        #[test]
        fn should_notify_and_despawn_when_timer_finishes() {
            let mut app = setup();
            app.add_systems(Update, enemy_projectile_lifecycle_system);

            app.world_mut().spawn(EnemyProjectileBundle::new(0.0, 0.0));

            advance_time_by_seconds(&mut app, 2.0);
            app.update();

            assert!(did_component_despawn::<EnemyProjectileComponent>(&mut app));
            assert!(did_message_fire::<EnemyProjectileExpiredMessage>(&mut app));
        }
    }

    #[cfg(test)]
    mod collision_system {
        use crate::domain::player::Player;
        use crate::infrastructure::bevy::enemy_projectile::components::{
            EnemyProjectileBundle, PlayerKilledMessage,
        };
        use crate::infrastructure::bevy::enemy_projectile::systems::collision_system;
        use crate::infrastructure::bevy::enemy_projectile::systems::tests::setup;
        use crate::infrastructure::bevy::player::components::PlayerComponent;
        use crate::infrastructure::bevy::player::resources::PlayerResource;
        use crate::infrastructure::bevy::player::systems::spawn_player_system;
        use bevy::app::{Startup, Update};
        use bevy::asset::{AssetApp, AssetPlugin};
        use bevy::image::Image;
        use bevy::prelude::Transform;
        use bevy_test::did_message_fire;

        #[test]
        fn should_trigger_an_event_hitting_the_player() {
            let mut app = setup();
            app.add_systems(Startup, spawn_player_system);
            app.add_systems(Update, collision_system);
            app.add_plugins(AssetPlugin::default());
            app.add_message::<PlayerKilledMessage>();
            app.init_asset::<Image>();

            app.insert_resource(PlayerResource(Player::new()));

            app.update();

            let player_info = app
                .world_mut()
                .query::<(&Transform, &PlayerComponent)>()
                .iter(app.world())
                .next()
                .map(|(t, _)| t.translation)
                .expect("PlayerComponent not found");

            app.world_mut()
                .spawn(EnemyProjectileBundle::new(player_info.x, player_info.y));

            app.update();

            assert!(did_message_fire::<PlayerKilledMessage>(&mut app));
        }
    }

    #[cfg(test)]
    mod on_enemy_projectile_hitting_player_system {
        use crate::infrastructure::bevy::enemy_projectile::components::{
            EnemyProjectileBundle, EnemyProjectileComponent, PlayerKilledMessage,
        };
        use crate::infrastructure::bevy::enemy_projectile::systems::on_enemy_projectile_hitting_player_system;
        use crate::infrastructure::bevy::enemy_projectile::systems::tests::setup;
        use bevy::app::Update;
        use bevy::asset::{AssetApp, AssetPlugin};
        use bevy::image::Image;
        use bevy_test::send_message;

        #[test]
        fn should_despawn_the_enemy_projectile() {
            let mut app = setup();
            app.add_plugins(AssetPlugin::default())
                .init_asset::<Image>();
            app.add_systems(Update, on_enemy_projectile_hitting_player_system);
            app.add_message::<PlayerKilledMessage>();

            let projectile = app
                .world_mut()
                .spawn(EnemyProjectileBundle::new(0.0, 0.0))
                .id();

            app.update();

            let enemy_projectile_info = app
                .world_mut()
                .query::<&EnemyProjectileComponent>()
                .iter(app.world())
                .next();

            assert!(enemy_projectile_info.is_some());

            send_message(&mut app, PlayerKilledMessage::new(projectile));

            app.update();

            let enemy_projectile_info = app
                .world_mut()
                .query::<&EnemyProjectileComponent>()
                .iter(app.world())
                .next();

            assert!(enemy_projectile_info.is_none());
        }
    }
}
