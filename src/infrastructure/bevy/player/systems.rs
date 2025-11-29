use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
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
use bevy::prelude::{
    AssetServer, Commands, KeyCode, MessageReader, Query, Res, ResMut, Time, Transform, With,
};

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

pub fn sync_player_firing_state_system(
    mut enemy_killed_message: MessageReader<EnemyKilledMessage>,
    mut projectile_expired_message: MessageReader<PlayerProjectileExpiredMessage>,
    mut player_resource: ResMut<PlayerResource>,
) {
    let mut should_reset_player = false;

    for _ in enemy_killed_message.read() {
        should_reset_player = true;
    }

    for _ in projectile_expired_message.read() {
        should_reset_player = true;
    }

    if should_reset_player {
        player_resource.0.toggle_fire();
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::player::Player;
    use crate::infrastructure::bevy::player::components::PlayerComponent;
    use crate::infrastructure::bevy::player::resources::PlayerResource;
    use crate::infrastructure::bevy::player::systems::{
        player_fire_system, player_movement_system, spawn_player_system,
        sync_player_firing_state_system,
    };
    use crate::infrastructure::bevy::player_projectile::resources::PlayerProjectileMovementTimerResource;
    use bevy::app::{App, Update};
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::input::ButtonInput;
    use bevy::prelude::{AssetApp, KeyCode, Time, Timer, TimerMode};
    use bevy::MinimalPlugins;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<Time>()
            .insert_resource(PlayerResource(Player::new()))
            .insert_resource(PlayerProjectileMovementTimerResource(Timer::from_seconds(
                1.0,
                TimerMode::Once,
            )))
            .init_asset::<Image>()
            .add_systems(Update, (player_movement_system, player_fire_system));

        app.update();
        app
    }

    #[cfg(test)]
    mod spawn_system {
        use super::*;
        use bevy::prelude::Transform;
        use bevy_test::{contains_component, count_components, run_system};
        use std::error::Error;

        #[test]
        fn should_spawn_player() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            run_system(&mut app, spawn_player_system)?;

            assert!(contains_component::<PlayerComponent>(&mut app));
            assert_eq!(count_components::<PlayerComponent>(&mut app), 1);

            let mut query = app.world_mut().query::<(&PlayerComponent, &Transform)>();
            let (_, transform) = query.single(app.world())?;

            assert_eq!(transform.translation.x, 0.0);

            Ok(())
        }
    }

    #[cfg(test)]
    mod movement_system {
        use super::*;
        use crate::domain::enemy_formation::MovingDirection;
        use crate::infrastructure::bevy::game_area::resources::GAME_AREA_WIDTH;
        use crate::infrastructure::bevy::player::resources::PLAYER_WIDTH;
        use bevy::input::ButtonInput;
        use bevy::prelude::{Entity, KeyCode, Transform, With};
        use bevy_test::run_system;
        use std::error::Error;

        fn player_should_move_on_input(
            key_code: KeyCode,
            moving_direction: MovingDirection,
        ) -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            run_system(&mut app, spawn_player_system)?;

            let start_x = app
                .world_mut()
                .query::<&Transform>()
                .single(app.world())?
                .translation
                .x;

            app.world_mut()
                .get_resource_mut::<ButtonInput<KeyCode>>()
                .ok_or("ButtonInput resource missing")?
                .press(key_code);

            app.update();

            let end_x = app
                .world_mut()
                .query::<&Transform>()
                .single(app.world())?
                .translation
                .x;

            match moving_direction {
                MovingDirection::ToLeft => assert!(end_x < start_x, "Player should move left"),
                MovingDirection::ToRight => assert!(end_x > start_x, "Player should move right"),
            }

            Ok(())
        }

        #[test]
        fn player_should_move_right_on_right_key_press() -> Result<(), Box<dyn Error>> {
            player_should_move_on_input(KeyCode::ArrowRight, MovingDirection::ToRight)
        }

        #[test]
        fn player_should_move_right_on_key_d_press() -> Result<(), Box<dyn Error>> {
            player_should_move_on_input(KeyCode::KeyD, MovingDirection::ToRight)
        }

        #[test]
        fn player_should_move_left_on_left_key_press() -> Result<(), Box<dyn Error>> {
            player_should_move_on_input(KeyCode::ArrowLeft, MovingDirection::ToLeft)
        }

        #[test]
        fn player_should_move_left_on_key_a_press() -> Result<(), Box<dyn Error>> {
            player_should_move_on_input(KeyCode::KeyA, MovingDirection::ToLeft)
        }

        #[test]
        fn player_should_not_move_out_of_bounds() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            run_system(&mut app, spawn_player_system)?;

            let boundary = (GAME_AREA_WIDTH / 2.0) - (PLAYER_WIDTH / 2.0);

            let player_entity = app
                .world_mut()
                .query_filtered::<Entity, With<PlayerComponent>>()
                .single(app.world())?;

            let mut transform = app
                .world_mut()
                .get_mut::<Transform>(player_entity)
                .ok_or("Player Transform missing")?;

            transform.translation.x = boundary;

            app.world_mut()
                .get_resource_mut::<ButtonInput<KeyCode>>()
                .ok_or("Input missing")?
                .press(KeyCode::ArrowRight);

            app.update();

            let end_x = app
                .world_mut()
                .query::<&Transform>()
                .single(app.world())?
                .translation
                .x;

            assert!(
                (end_x - boundary).abs() < 0.001,
                "Player should be clamped at boundary"
            );

            Ok(())
        }
    }

    #[cfg(test)]
    mod fire_system {
        use super::*;
        use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileComponent;
        use bevy::input::ButtonInput;
        use bevy::prelude::KeyCode;
        use bevy_test::{count_components, get_resource, run_system};
        use std::error::Error;

        #[test]
        fn player_should_spawn_projectile_when_firing() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            run_system(&mut app, spawn_player_system)?;

            let initial_count = count_components::<PlayerProjectileComponent>(&mut app);

            assert_eq!(initial_count, 0);

            app.world_mut()
                .get_resource_mut::<ButtonInput<KeyCode>>()
                .ok_or("ButtonInput resource missing")?
                .press(KeyCode::Space);

            app.update();

            let final_count = count_components::<PlayerProjectileComponent>(&mut app);

            assert_eq!(final_count, 1, "A projectile should spawn");

            let player_res = get_resource::<PlayerResource>(&mut app);

            assert!(
                player_res.0.is_firing(),
                "Player resource should be marked as firing"
            );

            Ok(())
        }

        #[test]
        fn player_should_not_fire_if_cooldown_is_active() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            run_system(&mut app, spawn_player_system)?;

            app.world_mut()
                .get_resource_mut::<PlayerResource>()
                .ok_or("PlayerResource missing")?
                .0
                .toggle_fire();

            app.world_mut()
                .get_resource_mut::<ButtonInput<KeyCode>>()
                .ok_or("ButtonInput resource missing")?
                .press(KeyCode::Space);

            app.update();

            let count = count_components::<PlayerProjectileComponent>(&mut app);

            assert_eq!(
                count, 0,
                "Should not spawn projectile if cooldown is active"
            );

            Ok(())
        }
    }

    #[cfg(test)]
    mod domain_sync_system {
        use super::*;
        use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
        use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileExpiredMessage;
        use bevy_test::{get_resource, run_system, send_message, spawn_dummy_entity};
        use std::error::Error;

        #[test]
        fn should_toggle_firing_when_hitting_enemy() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            run_system(&mut app, spawn_player_system)?;

            app.add_message::<EnemyKilledMessage>()
                .add_message::<PlayerProjectileExpiredMessage>()
                .add_systems(Update, sync_player_firing_state_system);

            let pre_update_player_resource = get_resource::<PlayerResource>(&mut app);

            assert!(!pre_update_player_resource.0.is_firing());

            let enemy_entity = spawn_dummy_entity(&mut app);
            let player_projectile_entity = spawn_dummy_entity(&mut app);

            send_message(
                &mut app,
                EnemyKilledMessage::new(enemy_entity, 1, player_projectile_entity),
            );

            app.update();

            let post_update_player_resource = get_resource::<PlayerResource>(&mut app);

            assert!(post_update_player_resource.0.is_firing());

            Ok(())
        }

        #[test]
        fn should_toggle_firing_when_projectile_expires() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            run_system(&mut app, spawn_player_system)?;

            app.add_message::<EnemyKilledMessage>()
                .add_message::<PlayerProjectileExpiredMessage>()
                .add_systems(Update, sync_player_firing_state_system);

            let pre_update_player_resource = get_resource::<PlayerResource>(&mut app);

            assert!(!pre_update_player_resource.0.is_firing());

            send_message(&mut app, PlayerProjectileExpiredMessage);

            app.update();

            let post_update_player_resource = get_resource::<PlayerResource>(&mut app);

            assert!(post_update_player_resource.0.is_firing());

            Ok(())
        }
    }
}
