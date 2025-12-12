use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
use crate::infrastructure::bevy::enemy_projectile::components::PlayerKilledMessage;
use crate::infrastructure::bevy::game_area::resources::GAME_AREA_WIDTH;
use crate::infrastructure::bevy::player::components::{PlayerBundle, PlayerComponent};
use crate::infrastructure::bevy::player::resources::{
    PlayerResource, DISTANCE_BETWEEN_PLAYER_AND_PROJECTILE, PLAYER_SPEED, PLAYER_WIDTH,
};
use crate::infrastructure::bevy::player_projectile::components::{
    PlayerProjectileBundle, PlayerProjectileExpiredMessage,
};
use crate::infrastructure::bevy::player_projectile::resources::PlayerProjectileMovementTimerResource;
use bevy::input::ButtonInput;
use bevy::prelude::{AssetServer, Commands, Entity, KeyCode, MessageReader, Query, Res, ResMut, Time, Transform, With};

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle::new(&asset_server));
}

pub fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerComponent>>,
    time: Res<Time>,
) {
    let delta = PLAYER_SPEED * time.delta_secs();

    for mut transform in player_query.iter_mut() {
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            transform.translation.x -= delta;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            transform.translation.x += delta;
        }

        let boundary = (GAME_AREA_WIDTH / 2.0) - (PLAYER_WIDTH / 2.0);
        transform.translation.x = transform.translation.x.clamp(-boundary, boundary);
    }
}

pub fn player_fire_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_resource: ResMut<PlayerResource>,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut timer: ResMut<PlayerProjectileMovementTimerResource>,
) {
    if keyboard.pressed(KeyCode::Space) && !player_resource.0.is_firing() {
        for transform in player_query.iter() {
            let translation = transform.translation;

            commands.spawn(PlayerProjectileBundle::new(
                translation.x,
                translation.y + DISTANCE_BETWEEN_PLAYER_AND_PROJECTILE,
            ));

            player_resource.0.toggle_fire();
            timer.0.reset();
        }
    }
}

pub fn reload_player_weapon_system(
    mut enemy_killed_message: MessageReader<EnemyKilledMessage>,
    mut projectile_expired_message: MessageReader<PlayerProjectileExpiredMessage>,
    mut player_resource: ResMut<PlayerResource>,
) {
    let mut should_reload = false;

    if enemy_killed_message.read().count() > 0 {
        should_reload = true;
    }

    if projectile_expired_message.read().count() > 0 {
        should_reload = true;
    }

    if should_reload {
        player_resource.0.toggle_fire();
    }
}

pub fn respawn_player_system(
    mut commands: Commands,
    mut player_killed_message: MessageReader<PlayerKilledMessage>,
    asset_server: Res<AssetServer>,
) {
    for _ in player_killed_message.read() {
        commands.spawn(PlayerBundle::new(&asset_server));
    }
}

pub fn on_enemy_projectile_hitting_player_system(
    mut commands: Commands,
    mut player_killed_event_writer: MessageReader<PlayerKilledMessage>,
    player_query: Query<Entity, With<PlayerComponent>>,

) {
    for _ in player_killed_event_writer.read() {
        for player_entity in player_query.iter() {
            commands.entity(player_entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::player::Player;
    use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
    use crate::infrastructure::bevy::player::components::PlayerComponent;
    use crate::infrastructure::bevy::player::resources::PlayerResource;
    use crate::infrastructure::bevy::player_projectile::components::{
        PlayerProjectileComponent, PlayerProjectileExpiredMessage,
    };
    use crate::infrastructure::bevy::player_projectile::resources::PlayerProjectileMovementTimerResource;
    use bevy::app::{App, Update};
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::input::ButtonInput;
    use bevy::prelude::{AssetApp, KeyCode, Time, Timer, TimerMode};
    use bevy_test::{
        advance_time_by_seconds, contains_single_component, count_components, get_resource_mut_or_fail,
        get_resource_or_fail, minimal_app, send_message, spawn_dummy_entity,
    };

    fn setup() -> App {
        let mut app = minimal_app(true);
        app.add_plugins(AssetPlugin::default())
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<Time>()
            .init_asset::<Image>()
            .insert_resource(PlayerResource(Player::new()))
            .insert_resource(PlayerProjectileMovementTimerResource(Timer::from_seconds(
                1.0,
                TimerMode::Once,
            )));
        app
    }

    #[cfg(test)]
    mod spawn_player_system {
        use super::*;
        use bevy::prelude::Startup;

        #[test]
        fn should_spawn_player() {
            let mut app = setup();
            app.add_systems(Startup, spawn_player_system);
            app.update();

            assert!(contains_single_component::<PlayerComponent>(&mut app));

            let mut query = app.world_mut().query::<(&PlayerComponent, &Transform)>();
            let (_, transform) = query
                .single(app.world())
                .expect("Player transform not found");

            assert_eq!(transform.translation.x, 0.0);
        }
    }

    #[cfg(test)]
    mod player_movement_system {
        use super::*;
        use crate::infrastructure::bevy::game_area::resources::GAME_AREA_WIDTH;
        use crate::infrastructure::bevy::player::resources::PLAYER_WIDTH;

        fn setup_movement(x_pos: f32) -> App {
            let mut app = setup();
            app.world_mut()
                .spawn((PlayerComponent, Transform::from_xyz(x_pos, 0.0, 0.0)));
            app.add_systems(Update, player_movement_system);
            app
        }

        #[test]
        fn should_move_right_on_right_arrow() {
            let mut app = setup_movement(0.0);

            get_resource_mut_or_fail::<ButtonInput<KeyCode>>(&mut app).press(KeyCode::ArrowRight);

            advance_time_by_seconds(&mut app, 0.1);

            app.update();

            let transform = app
                .world_mut()
                .query::<&Transform>()
                .single(app.world())
                .expect("Transform not found");

            assert!(transform.translation.x > 0.0);
        }

        #[test]
        fn should_move_right_on_d_key() {
            let mut app = setup_movement(0.0);

            get_resource_mut_or_fail::<ButtonInput<KeyCode>>(&mut app).press(KeyCode::KeyD);
            advance_time_by_seconds(&mut app, 0.1);

            app.update();

            let transform = app
                .world_mut()
                .query::<&Transform>()
                .single(app.world())
                .expect("Transform not found");

            assert!(transform.translation.x > 0.0);
        }

        #[test]
        fn should_move_left_on_left_arrow() {
            let mut app = setup_movement(0.0);

            get_resource_mut_or_fail::<ButtonInput<KeyCode>>(&mut app).press(KeyCode::ArrowLeft);
            advance_time_by_seconds(&mut app, 0.1);

            app.update();

            let transform = app
                .world_mut()
                .query::<&Transform>()
                .single(app.world())
                .expect("Transform not found");

            assert!(transform.translation.x < 0.0);
        }

        #[test]
        fn should_move_left_on_a_key() {
            let mut app = setup_movement(0.0);

            get_resource_mut_or_fail::<ButtonInput<KeyCode>>(&mut app).press(KeyCode::KeyA);
            advance_time_by_seconds(&mut app, 0.1);

            app.update();

            let transform = app
                .world_mut()
                .query::<&Transform>()
                .single(app.world())
                .expect("Transform not found");

            assert!(transform.translation.x < 0.0);
        }

        #[test]
        fn should_clamp_at_boundaries() {
            let boundary = (GAME_AREA_WIDTH / 2.0) - (PLAYER_WIDTH / 2.0);
            let mut app = setup_movement(boundary);

            get_resource_mut_or_fail::<ButtonInput<KeyCode>>(&mut app).press(KeyCode::ArrowRight);
            advance_time_by_seconds(&mut app, 1.0);

            app.update();

            let transform = app
                .world_mut()
                .query::<&Transform>()
                .single(app.world())
                .expect("Transform not found");

            let diff = (transform.translation.x - boundary).abs();
            assert!(diff < 0.001);
        }
    }

    #[cfg(test)]
    mod player_fire_system {
        use super::*;

        fn setup_fire() -> App {
            let mut app = setup();
            app.world_mut()
                .spawn((PlayerComponent, Transform::from_xyz(0.0, 0.0, 0.0)));
            app.add_systems(Update, player_fire_system);
            app
        }

        #[test]
        fn should_spawn_projectile() {
            let mut app = setup_fire();

            get_resource_mut_or_fail::<ButtonInput<KeyCode>>(&mut app).press(KeyCode::Space);

            app.update();

            assert_eq!(count_components::<PlayerProjectileComponent>(&mut app), 1);
            assert!(
                get_resource_or_fail::<PlayerResource>(&mut app)
                    .0
                    .is_firing()
            );
        }

        #[test]
        fn should_not_spawn_if_cooldown_active() {
            let mut app = setup_fire();

            get_resource_mut_or_fail::<PlayerResource>(&mut app).0.toggle_fire();
            get_resource_mut_or_fail::<ButtonInput<KeyCode>>(&mut app).press(KeyCode::Space);

            app.update();

            assert_eq!(count_components::<PlayerProjectileComponent>(&mut app), 0);
        }
    }

    #[cfg(test)]
    mod reload_player_weapon_system {
        use super::*;

        fn setup_reload() -> App {
            let mut app = setup();
            app.add_message::<EnemyKilledMessage>()
                .add_message::<PlayerProjectileExpiredMessage>()
                .add_systems(Update, reload_player_weapon_system);

            get_resource_mut_or_fail::<PlayerResource>(&mut app).0.toggle_fire();
            app
        }

        #[test]
        fn should_reload_on_enemy_killed() {
            let mut app = setup_reload();
            let dummy = spawn_dummy_entity(&mut app);

            send_message(&mut app, EnemyKilledMessage::new(dummy, 1, dummy));
            app.update();

            assert!(
                !get_resource_or_fail::<PlayerResource>(&mut app)
                    .0
                    .is_firing()
            );
        }

        #[test]
        fn should_reload_on_projectile_expiry() {
            let mut app = setup_reload();

            send_message(&mut app, PlayerProjectileExpiredMessage);
            app.update();

            assert!(
                !get_resource_or_fail::<PlayerResource>(&mut app)
                    .0
                    .is_firing()
            );
        }
    }

    #[cfg(test)]
    mod respawn_player_system {
        use crate::infrastructure::bevy::enemy_projectile::components::{
            EnemyProjectileComponent, PlayerKilledMessage,
        };
        use crate::infrastructure::bevy::player::components::PlayerComponent;
        use crate::infrastructure::bevy::player::systems::respawn_player_system;
        use crate::infrastructure::bevy::player::systems::tests::setup;
        use bevy::app::Update;
        use bevy::prelude::{Entity, With};
        use bevy_test::{contains_single_component, count_components, despawn, send_message};

        #[test]
        fn should_respawn_player_when_killed() {
            let mut app = setup();
            app.add_message::<PlayerKilledMessage>()
                .add_systems(Update, respawn_player_system);

            app.world_mut().spawn(PlayerComponent);
            app.world_mut().spawn(EnemyProjectileComponent);

            let player_entity = app
                .world_mut()
                .query_filtered::<Entity, With<PlayerComponent>>()
                .single(app.world())
                .expect("World not found");

            let projectile_entity = app
                .world_mut()
                .query_filtered::<Entity, With<EnemyProjectileComponent>>()
                .single(app.world())
                .expect("World not found");

            despawn(&mut app, player_entity);

            assert_eq!(count_components::<PlayerComponent>(&mut app), 0);

            send_message(&mut app, PlayerKilledMessage::new(projectile_entity));

            app.update();

            assert!(contains_single_component::<PlayerComponent>(&mut app));
        }
    }
}
