use crate::domain::enemy_formation::{EnemyFormation, COLUMNS, NUMBER_OF_STEPS_ON_X_AXE};
use crate::infrastructure::bevy::enemy::{
    DespawnEnemyMessage, EnemyFireProbability, EnemyProjectileMovementTimer,
};
pub(crate) use crate::infrastructure::bevy::enemy::{EnemyView, ENEMY_HEIGHT, ENEMY_WIDTH};
use crate::infrastructure::bevy::enemy_projectile::EnemyProjectileView;
use crate::infrastructure::bevy::game_area::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use crate::infrastructure::bevy::header::HEADER_HEIGHT;
use crate::infrastructure::bevy::player::PlayerResource;
use crate::infrastructure::bevy::player_projectile::PlayerProjectileView;
use crate::infrastructure::bevy::score::ScoreResource;
use bevy::prelude::*;
use rand::prelude::IteratorRandom;
use rand::Rng;

pub const ENEMY_FORMATION_STEP_DURATION: f32 = 0.6;
pub const SPACE_BETWEEN_ENEMIES_X: f32 = 15.0;

const SPACE_BETWEEN_ENEMIES_Y: f32 = 15.0;
const VERTICAL_DROP: f32 = 15.0;

#[derive(Resource)]
pub struct EnemyFormationResource(pub EnemyFormation);

#[derive(Resource)]
pub struct EnemyFormationMovementTimer(pub Timer);

#[derive(Component)]
pub struct EnemyFormationView;

impl EnemyFormationView {
    pub fn spawn_enemy_formation(
        commands: Commands,
        asset_server: Res<AssetServer>,
        enemy_formation_res: Res<EnemyFormationResource>,
    ) {
        Self::spawn_enemies(commands, &asset_server, &enemy_formation_res);
    }

    pub fn on_move(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        enemy_formation_res: Res<EnemyFormationResource>,
        enemy_query: Query<Entity, With<EnemyView>>,
    ) {
        if enemy_formation_res.is_changed() {
            for entity in enemy_query.iter() {
                commands.entity(entity).despawn();
            }
            Self::spawn_enemies(commands, &asset_server, &enemy_formation_res);
        }
    }

    pub fn advance_on_tick(
        time: Res<Time>,
        mut enemy_formation_res: ResMut<EnemyFormationResource>,
        mut timer: ResMut<EnemyFormationMovementTimer>,
    ) {
        if enemy_formation_res.0.has_breached() || enemy_formation_res.0.is_annihilated() {
            timer.0.finish();
        } else if timer.0.tick(time.delta()).just_finished() {
            enemy_formation_res.0.advance_enemies();
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

        let step_size_x = Self::calculate_step_x(ENEMY_WIDTH, SPACE_BETWEEN_ENEMIES_X);

        let enemy_formation_start_x = -(GAME_AREA_WIDTH / 2.0);
        let enemy_formation_start_y = (GAME_AREA_HEIGHT / 2.0) - HEADER_HEIGHT;

        let enemy_formation_width =
            enemy_formation_start_x + (enemy_formation_x as f32 * step_size_x);
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

                    commands.spawn(EnemyView::make_enemy(
                        enemy.get_id(),
                        new_x,
                        new_y,
                        asset_server,
                    ));
                }
            }
        }
    }

    pub fn handle_collisions(
        mut commands: Commands,
        mut enemy_formation_resource: ResMut<EnemyFormationResource>,
        player_projectile_query: Query<(Entity, &Transform, &Sprite), With<PlayerProjectileView>>,
        enemy_query: Query<(Entity, &Transform, &Sprite, &EnemyView), With<EnemyView>>,
        mut player_resource: ResMut<PlayerResource>,
        mut score_resource: ResMut<ScoreResource>,
    ) {
        for (player_projectile_entity, player_projectile_transform, player_projectile_sprite) in
            player_projectile_query.iter()
        {
            let player_projectile_size = player_projectile_sprite.custom_size.unwrap_or(Vec2::ONE);

            for (enemy_entity, enemy_transform, enemy_sprite, enemy_view) in enemy_query.iter() {
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
                    commands.write_message(DespawnEnemyMessage(enemy_entity, enemy_view.clone()));
                    commands.entity(player_projectile_entity).despawn();
                    player_resource.0.toggle_fire();
                    score_resource.0.increment(10);
                    enemy_formation_resource.0.kill(enemy_view.id);
                    break;
                }
            }
        }
    }

    pub fn spawn_random_projectiles(
        mut commands: Commands,
        time: Res<Time>,
        mut timer: ResMut<EnemyProjectileMovementTimer>,
        enemy_view_query: Query<&Transform, With<EnemyView>>,
        enemy_fire_probability: ResMut<EnemyFireProbability>,
    ) {
        if !timer.0.tick(time.delta()).just_finished() {
            return;
        }

        let mut rng = rand::rng();

        enemy_view_query
            .iter()
            .choose_multiple(&mut rng, 5)
            .iter()
            .for_each(|chosen| {
                let should_shoot = rng.random_bool(enemy_fire_probability.0);

                if should_shoot {
                    let start_x = chosen.translation.x;
                    let start_y = chosen.translation.y;

                    let projectile_view = EnemyProjectileView::new(start_x, start_y);

                    commands.spawn(projectile_view.make_projectile());
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::enemy_formation::EnemyFormation;
    use crate::domain::player::Player;
    use crate::domain::score::Score;
    use crate::infrastructure::bevy::enemy::{
        DespawnEnemyMessage, EnemyFireProbability, EnemyProjectileMovementTimer,
    };
    use crate::infrastructure::bevy::enemy_formation::{
        EnemyFormationMovementTimer, EnemyFormationResource, EnemyFormationView, EnemyView,
    };
    use crate::infrastructure::bevy::enemy_projectile::EnemyProjectileView;
    use crate::infrastructure::bevy::player::PlayerResource;
    use crate::infrastructure::bevy::player_projectile::PlayerProjectileView;
    use crate::infrastructure::bevy::score::ScoreResource;
    use bevy::app::{App, Startup, Update};
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::image::Image;
    use bevy::math::Vec2;
    use bevy::prelude::{IntoScheduleConfigs, MessageReader, Timer, TimerMode, Transform, With};
    use bevy::sprite::Sprite;
    use bevy::text::Font;
    use bevy::time::Time;
    use bevy::utils::default;
    use bevy::MinimalPlugins;
    use std::error::Error;
    use std::time::Duration;

    fn get_first_enemy_coordinates(app: &mut App) -> Result<(f32, f32), Box<dyn Error>> {
        let translation = app
            .world_mut()
            .query_filtered::<&Transform, With<EnemyView>>()
            .iter(app.world())
            .next()
            .ok_or("First enemy coordinates not found")?
            .translation;
        Ok((translation.x, translation.y))
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));

        app.add_systems(Startup, EnemyFormationView::spawn_enemy_formation.chain());
        app.add_systems(Update, EnemyFormationView::on_move.chain());

        app.insert_resource(EnemyFormationResource(EnemyFormation::new()));
        app.insert_resource(PlayerResource(Player::new()));

        app.init_asset::<Image>();
        app.init_asset::<Font>();
        app.add_message::<DespawnEnemyMessage>();

        app.update();

        app
    }

    #[test]
    fn should_display_the_enemy_formation() -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        let mut query = app.world_mut().query::<&EnemyView>();
        let enemy_count = query.iter(app.world()).count();

        assert_eq!(enemy_count, 55);
        Ok(())
    }

    #[test]
    fn enemy_formation_should_move_to_the_right_when_there_is_enough_space()
    -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        let first_enemy_x_t0 = get_first_enemy_coordinates(&mut app)?.0;

        let mut resource = app.world_mut().resource_mut::<EnemyFormationResource>();
        resource.0.advance_enemies();

        app.update();

        let first_enemy_x_t1 = get_first_enemy_coordinates(&mut app)?.0;

        assert!(first_enemy_x_t1 > first_enemy_x_t0);
        Ok(())
    }

    #[test]
    fn enemy_formation_should_move_to_the_left_when_there_is_enough_space()
    -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        for _ in 0..31 {
            app.world_mut()
                .resource_mut::<EnemyFormationResource>()
                .0
                .advance_enemies();
        }

        app.update();

        let first_enemy_x_t0 = get_first_enemy_coordinates(&mut app)?.0;

        app.world_mut()
            .resource_mut::<EnemyFormationResource>()
            .0
            .advance_enemies();

        app.update();

        let first_enemy_x_t1 = get_first_enemy_coordinates(&mut app)?.0;

        assert!(first_enemy_x_t1 < first_enemy_x_t0);
        Ok(())
    }

    #[test]
    fn enemy_formation_should_drop_down_when_there_is_not_enough_right_space()
    -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        let first_enemy_y_t0 = get_first_enemy_coordinates(&mut app)?.1;

        for _ in 0..31 {
            app.world_mut()
                .resource_mut::<EnemyFormationResource>()
                .0
                .advance_enemies();
        }

        app.update();

        let first_enemy_y_t1 = get_first_enemy_coordinates(&mut app)?.1;

        assert!(first_enemy_y_t1 < first_enemy_y_t0);
        Ok(())
    }

    #[test]
    fn enemy_formation_should_drop_down_when_there_is_not_enough_left_space()
    -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        for _ in 0..31 {
            app.world_mut()
                .resource_mut::<EnemyFormationResource>()
                .0
                .advance_enemies();
        }

        app.update();

        let first_enemy_y_t0 = get_first_enemy_coordinates(&mut app)?.1;

        for _ in 0..31 {
            app.world_mut()
                .resource_mut::<EnemyFormationResource>()
                .0
                .advance_enemies();
        }

        app.update();

        let first_enemy_y_t1 = get_first_enemy_coordinates(&mut app)?.1;

        assert!(first_enemy_y_t1 < first_enemy_y_t0);
        Ok(())
    }

    #[test]
    fn enemy_formation_should_advance_on_tick() -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        let first_enemy_x_t0 = get_first_enemy_coordinates(&mut app)?.0;

        app.init_resource::<Time>();
        app.insert_resource(EnemyFormationMovementTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )));

        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_secs_f32(1.0));

        app.world_mut()
            .run_system_once(EnemyFormationView::advance_on_tick)
            .map_err(|e| format!("Cannot run system: {e}"))?;

        app.update();

        let first_enemy_x_t1 = get_first_enemy_coordinates(&mut app)?.0;

        assert!(first_enemy_x_t1 > first_enemy_x_t0);
        Ok(())
    }

    #[test]
    fn should_fire_despawn_event_on_collision() -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        app.add_systems(Update, EnemyFormationView::handle_collisions);
        app.insert_resource(ScoreResource(Score::new()));

        let enemy_info = app
            .world_mut()
            .query::<(&Transform, &EnemyView)>()
            .iter(app.world())
            .next()
            .map(|(t, v)| (t.translation, v.id))
            .ok_or("EnemyView not found")?;

        let enemy_x = enemy_info.0;
        let enemy_id = enemy_info.1;

        app.world_mut().spawn((
            PlayerProjectileView::new(0.0, 0.0),
            Sprite {
                custom_size: Some(Vec2::new(5.0, 15.0)),
                ..default()
            },
            Transform::from_translation(enemy_x),
        ));

        app.update();

        app.world_mut()
            .run_system_once(move |mut reader: MessageReader<DespawnEnemyMessage>| {
                let message = reader
                    .read()
                    .next()
                    .unwrap_or_else(|| panic!("Despawn enemy message did not arrive!"));
                assert_eq!(message.1.id, enemy_id);
            })
            .map_err(|e| format!("Cannot run system: {e}"))?;

        Ok(())
    }

    #[test]
    fn should_randomly_spawn_projectiles() -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        app.init_resource::<Time>();
        app.insert_resource(EnemyProjectileMovementTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )));
        app.insert_resource(EnemyFireProbability(1.0));

        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_secs_f32(1.0));

        app.world_mut()
            .run_system_once(EnemyFormationView::spawn_random_projectiles)
            .map_err(|e| format!("Cannot run system: {e}"))?;

        let projectiles = app
            .world_mut()
            .query::<&EnemyProjectileView>()
            .iter(app.world())
            .len();

        assert!(projectiles > 0);

        Ok(())
    }
}
