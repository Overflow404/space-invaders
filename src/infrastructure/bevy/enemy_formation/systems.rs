use crate::domain::enemy_formation::{COLUMNS, NUMBER_OF_STEPS_ON_X_AXE};
use crate::infrastructure::bevy::enemy::components::{
    EnemyBundle, EnemyComponent, EnemyKilledMessage,
};
use crate::infrastructure::bevy::enemy::resources::{
    EnemyFireProbability, EnemyProjectileMovementTimer,
};
use crate::infrastructure::bevy::enemy::resources::{ENEMY_HEIGHT, ENEMY_WIDTH};
use crate::infrastructure::bevy::enemy_formation::resources::{
    EnemyFormationMovementTimer, EnemyFormationResource, SPACE_BETWEEN_ENEMIES_X,
    SPACE_BETWEEN_ENEMIES_Y, VERTICAL_DROP,
};
use crate::infrastructure::bevy::enemy_projectile::components::EnemyProjectileBundle;
use crate::infrastructure::bevy::game_area::resources::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use crate::infrastructure::bevy::header::resources::HEADER_HEIGHT;
use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileComponent;
use bevy::prelude::*;
use rand::prelude::IteratorRandom;
use rand::Rng;

pub fn spawn_enemy_formation_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    enemy_formation_res: Res<EnemyFormationResource>,
) {
    spawn_enemies(commands, &asset_server, &enemy_formation_res);
}

pub fn enemy_formation_movement_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemy_formation_res: Res<EnemyFormationResource>,
    enemy_query: Query<Entity, With<EnemyComponent>>,
) {
    if enemy_formation_res.is_changed() {
        for entity in enemy_query.iter() {
            commands.entity(entity).despawn();
        }
        spawn_enemies(commands, &asset_server, &enemy_formation_res);
    }
}

pub fn enemy_formation_lifecycle_system(
    time: Res<Time>,
    mut enemy_formation_res: ResMut<EnemyFormationResource>,
    mut timer: ResMut<EnemyFormationMovementTimer>,
) {
    if enemy_formation_res.0.has_breached() || enemy_formation_res.0.is_annihilated() {
        timer.0.finish();
    } else if timer.0.tick(time.delta()).just_finished() {
        enemy_formation_res.0.advance();
    }
}

fn calculate_step_x(enemy_width: f32, gap_x: f32) -> f32 {
    let n_enemies = COLUMNS as f32;
    let n_gaps = (COLUMNS - 1) as f32;

    let n_steps = (NUMBER_OF_STEPS_ON_X_AXE - COLUMNS) as f32;

    let block_width = (n_enemies * enemy_width) + (n_gaps * gap_x);

    let remaining_screen = GAME_AREA_WIDTH - block_width;

    let step = remaining_screen / n_steps;

    step.max(1.0)
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: &AssetServer,
    enemy_formation: &EnemyFormationResource,
) {
    let enemies = enemy_formation.0.get_enemies();
    let (enemy_formation_x, enemy_formation_y) = enemy_formation.0.get_position();

    if enemies.is_empty() {
        return;
    }

    let step_size_x = calculate_step_x(ENEMY_WIDTH, SPACE_BETWEEN_ENEMIES_X);

    let enemy_formation_start_x = -(GAME_AREA_WIDTH / 2.0);
    let enemy_formation_start_y = (GAME_AREA_HEIGHT / 2.0) - HEADER_HEIGHT;

    let enemy_formation_width = enemy_formation_start_x + (enemy_formation_x as f32 * step_size_x);
    let enemy_formation_height =
        enemy_formation_start_y - (enemy_formation_y as f32 * VERTICAL_DROP);

    for (row_index, row) in enemies.iter().enumerate() {
        for (column_index, enemy_slot) in row.iter().enumerate() {
            if let Some(enemy) = enemy_slot {
                let new_x = enemy_formation_width
                    + (column_index as f32 * (ENEMY_WIDTH + SPACE_BETWEEN_ENEMIES_X))
                    + (ENEMY_WIDTH / 2.0);

                let new_y = enemy_formation_height
                    - (row_index as f32 * (ENEMY_HEIGHT + SPACE_BETWEEN_ENEMIES_Y))
                    - (ENEMY_HEIGHT / 2.0);

                commands.spawn(EnemyBundle::new(enemy.get_id(), new_x, new_y, asset_server));
            }
        }
    }
}

pub fn collisions_system(
    player_projectile_query: Query<(Entity, &Transform, &Sprite), With<PlayerProjectileComponent>>,
    enemy_query: Query<(Entity, &Transform, &Sprite, &EnemyComponent), With<EnemyComponent>>,
    mut despawn_enemy_message_writer: MessageWriter<EnemyKilledMessage>,
) {
    for (player_projectile_entity, player_projectile_transform, player_projectile_sprite) in
        player_projectile_query.iter()
    {
        let player_projectile_size = player_projectile_sprite.custom_size.unwrap_or(Vec2::ONE);

        for (enemy_entity, enemy_transform, enemy_sprite, enemy_component) in enemy_query.iter() {
            let enemy_size = enemy_sprite.custom_size.unwrap_or(Vec2::ONE);

            let collision = player_projectile_transform.translation.x
                < enemy_transform.translation.x + enemy_size.x / 2.0
                && player_projectile_transform.translation.x + player_projectile_size.x
                    > enemy_transform.translation.x - enemy_size.x / 2.0
                && player_projectile_transform.translation.y
                    < enemy_transform.translation.y + enemy_size.y / 2.0
                && player_projectile_transform.translation.y + player_projectile_size.y
                    > enemy_transform.translation.y - enemy_size.y / 2.0;

            if collision {
                despawn_enemy_message_writer.write(EnemyKilledMessage::new(
                    enemy_entity,
                    enemy_component.id,
                    player_projectile_entity,
                ));
                break;
            }
        }
    }
}

pub fn spawn_random_projectiles_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemyProjectileMovementTimer>,
    enemy_query: Query<&Transform, With<EnemyComponent>>,
    enemy_fire_probability: ResMut<EnemyFireProbability>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let mut rng = rand::rng();

    enemy_query
        .iter()
        .choose_multiple(&mut rng, 5)
        .iter()
        .for_each(|chosen| {
            let should_shoot = rng.random_bool(enemy_fire_probability.0);

            if should_shoot {
                let start_x = chosen.translation.x;
                let start_y = chosen.translation.y;

                commands.spawn(EnemyProjectileBundle::new(start_x, start_y));
            }
        })
}

pub fn sync_enemy_formation_state_system(
    mut enemy_killed_message: MessageReader<EnemyKilledMessage>,
    mut enemy_formation_resource: ResMut<EnemyFormationResource>,
) {
    for enemy in enemy_killed_message.read() {
        enemy_formation_resource.0.kill(enemy.enemy_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::enemy_formation::EnemyFormation;
    use crate::domain::player::Player;
    use crate::infrastructure::bevy::enemy::components::{EnemyComponent, EnemyKilledMessage};
    use crate::infrastructure::bevy::enemy_formation::resources::EnemyFormationResource;
    use crate::infrastructure::bevy::enemy_formation::systems::{
        enemy_formation_movement_system, spawn_enemy_formation_system,
    };
    use crate::infrastructure::bevy::player::resources::PlayerResource;
    use bevy::app::{App, Startup, Update};
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::prelude::{IntoScheduleConfigs, Transform, With};
    use bevy::text::Font;
    use bevy::MinimalPlugins;
    use std::error::Error;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .add_systems(Startup, spawn_enemy_formation_system.chain())
            .add_systems(Update, enemy_formation_movement_system.chain())
            .insert_resource(EnemyFormationResource(EnemyFormation::new()))
            .insert_resource(PlayerResource(Player::new()))
            .init_asset::<Image>()
            .init_asset::<Font>()
            .add_message::<EnemyKilledMessage>();

        app.update();
        app
    }

    fn get_first_enemy_coordinates(app: &mut App) -> Result<(f32, f32), Box<dyn Error>> {
        let translation = app
            .world_mut()
            .query_filtered::<&Transform, With<EnemyComponent>>()
            .iter(app.world())
            .next()
            .ok_or("First enemy coordinates not found")?
            .translation;
        Ok((translation.x, translation.y))
    }

    #[test]
    fn should_display_the_enemy_formation() -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        let enemy_count = bevy_test::count_components::<EnemyComponent>(&mut app);

        assert_eq!(enemy_count, 55);
        Ok(())
    }

    #[cfg(test)]
    mod movement_system {
        use crate::infrastructure::bevy::enemy_formation::resources::{
            EnemyFormationMovementTimer, EnemyFormationResource,
        };
        use crate::infrastructure::bevy::enemy_formation::systems::enemy_formation_lifecycle_system;
        use crate::infrastructure::bevy::enemy_formation::systems::tests::{
            get_first_enemy_coordinates, setup,
        };
        use bevy::prelude::{Time, Timer, TimerMode};
        use bevy_test::{advance_time_by_seconds, run_system};
        use std::error::Error;

        #[test]
        fn should_move_to_the_right_when_there_is_enough_space() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            let first_enemy_x_t0 = get_first_enemy_coordinates(&mut app)?.0;

            let mut resource = app.world_mut().resource_mut::<EnemyFormationResource>();
            resource.0.advance();

            app.update();

            let first_enemy_x_t1 = get_first_enemy_coordinates(&mut app)?.0;

            assert!(first_enemy_x_t1 > first_enemy_x_t0);
            Ok(())
        }

        #[test]
        fn should_move_to_the_left_when_there_is_enough_space() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            for _ in 0..31 {
                app.world_mut()
                    .resource_mut::<EnemyFormationResource>()
                    .0
                    .advance();
            }

            app.update();

            let first_enemy_x_t0 = get_first_enemy_coordinates(&mut app)?.0;

            app.world_mut()
                .resource_mut::<EnemyFormationResource>()
                .0
                .advance();

            app.update();

            let first_enemy_x_t1 = get_first_enemy_coordinates(&mut app)?.0;

            assert!(first_enemy_x_t1 < first_enemy_x_t0);
            Ok(())
        }

        #[test]
        fn should_drop_down_when_there_is_not_enough_right_space() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            let first_enemy_y_t0 = get_first_enemy_coordinates(&mut app)?.1;

            for _ in 0..31 {
                app.world_mut()
                    .resource_mut::<EnemyFormationResource>()
                    .0
                    .advance();
            }

            app.update();

            let first_enemy_y_t1 = get_first_enemy_coordinates(&mut app)?.1;

            assert!(first_enemy_y_t1 < first_enemy_y_t0);
            Ok(())
        }

        #[test]
        fn should_drop_down_when_there_is_not_enough_left_space() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            for _ in 0..31 {
                app.world_mut()
                    .resource_mut::<EnemyFormationResource>()
                    .0
                    .advance();
            }

            app.update();

            let first_enemy_y_t0 = get_first_enemy_coordinates(&mut app)?.1;

            for _ in 0..31 {
                app.world_mut()
                    .resource_mut::<EnemyFormationResource>()
                    .0
                    .advance();
            }

            app.update();

            let first_enemy_y_t1 = get_first_enemy_coordinates(&mut app)?.1;

            assert!(first_enemy_y_t1 < first_enemy_y_t0);
            Ok(())
        }

        #[test]
        fn should_advance_on_tick() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            let first_enemy_x_t0 = get_first_enemy_coordinates(&mut app)?.0;

            app.init_resource::<Time>();
            app.insert_resource(EnemyFormationMovementTimer(Timer::from_seconds(
                1.0,
                TimerMode::Once,
            )));

            advance_time_by_seconds(&mut app, 1.0);

            run_system(&mut app, enemy_formation_lifecycle_system)?;

            app.update();

            let first_enemy_x_t1 = get_first_enemy_coordinates(&mut app)?.0;

            assert!(first_enemy_x_t1 > first_enemy_x_t0);
            Ok(())
        }
    }

    #[cfg(test)]
    mod collision_system {
        use crate::domain::score::Score;
        use crate::infrastructure::bevy::enemy::components::{EnemyComponent, EnemyKilledMessage};
        use crate::infrastructure::bevy::enemy_formation::systems::collisions_system;
        use crate::infrastructure::bevy::enemy_formation::systems::tests::setup;
        use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileBundle;
        use crate::infrastructure::bevy::score::resources::ScoreResource;
        use bevy::app::Update;
        use bevy::prelude::Transform;
        use bevy_test::verify_message_fired;
        use std::error::Error;

        #[test]
        fn should_trigger_an_event_when_killing_an_enemy() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            app.add_systems(Update, collisions_system);
            app.insert_resource(ScoreResource(Score::new()));

            let enemy_info = app
                .world_mut()
                .query::<(&Transform, &EnemyComponent)>()
                .iter(app.world())
                .next()
                .map(|(t, c)| (t.translation, c.id))
                .ok_or("EnemyComponent not found")?;

            let enemy_x = enemy_info.0.x;
            let enemy_y = enemy_info.0.y;

            app.world_mut()
                .spawn(PlayerProjectileBundle::new(enemy_x, enemy_y));

            app.update();

            verify_message_fired::<EnemyKilledMessage>(&mut app)?;

            Ok(())
        }
    }

    #[cfg(test)]
    mod projectile_spawning_system {
        use crate::infrastructure::bevy::enemy::resources::{
            EnemyFireProbability, EnemyProjectileMovementTimer,
        };
        use crate::infrastructure::bevy::enemy_formation::systems::spawn_random_projectiles_system;
        use crate::infrastructure::bevy::enemy_formation::systems::tests::setup;
        use crate::infrastructure::bevy::enemy_projectile::components::EnemyProjectileComponent;
        use bevy::prelude::{Time, Timer, TimerMode};
        use bevy_test::{advance_time_by_seconds, run_system};
        use std::error::Error;

        #[test]
        fn enemy_formation_should_randomly_spawn_projectiles() -> Result<(), Box<dyn Error>> {
            let mut app = setup();

            app.init_resource::<Time>();
            app.insert_resource(EnemyProjectileMovementTimer(Timer::from_seconds(
                1.0,
                TimerMode::Once,
            )));
            app.insert_resource(EnemyFireProbability(1.0));

            advance_time_by_seconds(&mut app, 1.0);

            run_system(&mut app, spawn_random_projectiles_system)?;

            let projectiles = app
                .world_mut()
                .query::<&EnemyProjectileComponent>()
                .iter(app.world())
                .len();

            assert!(projectiles > 0);

            Ok(())
        }
    }

    #[cfg(test)]
    mod domain_sync_system {
        use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
        use crate::infrastructure::bevy::enemy_formation::resources::EnemyFormationResource;
        use crate::infrastructure::bevy::enemy_formation::systems::sync_enemy_formation_state_system;
        use crate::infrastructure::bevy::enemy_formation::systems::tests::setup;
        use bevy::app::Update;
        use bevy_test::{send_message, spawn_dummy_entity};

        #[test]
        fn should_sync_domain() {
            let mut app = setup();
            app.add_message::<EnemyKilledMessage>();
            app.add_systems(Update, sync_enemy_formation_state_system);

            let killed_enemy_id = 2;

            let enemy_entity = spawn_dummy_entity(&mut app);
            let player_projectile_entity = spawn_dummy_entity(&mut app);

            send_message(
                &mut app,
                EnemyKilledMessage::new(enemy_entity, killed_enemy_id, player_projectile_entity),
            );

            app.update();

            let post_update_enemy_formation_resource = app
                .world_mut()
                .get_resource::<EnemyFormationResource>()
                .unwrap_or_else(|| panic!("EnemyFormationResource missing"));

            assert!(
                post_update_enemy_formation_resource
                    .0
                    .get_enemies()
                    .get(0)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .is_none()
            );
        }
    }
}
