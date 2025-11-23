use crate::domain::enemy_formation::{EnemyFormation, COLUMNS, NUMBER_OF_STEPS_ON_X_AXE};
use crate::infrastructure::bevy::game_area::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use crate::infrastructure::bevy::header::HEADER_HEIGHT;
use bevy::prelude::*;

pub const ENEMY_FORMATION_STEP_DURATION: f32 = 0.01;
pub const SPACE_BETWEEN_ENEMIES_X: f32 = 15.0;

const ENEMY_WIDTH: f32 = 60.0;
const ENEMY_HEIGHT: f32 = 40.0;
const SPACE_BETWEEN_ENEMIES_Y: f32 = 15.0;
const VERTICAL_DROP: f32 = 15.0;
const ENEMY_IMAGE: &str = "red.png";

#[derive(Resource)]
pub struct EnemyFormationResource(pub EnemyFormation);

#[derive(Resource)]
pub struct EnemyFormationMovementTimer(pub Timer);

#[derive(Component)]
pub struct EnemyFormationView;

#[derive(Component)]
pub struct EnemyView;

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
        if timer.0.tick(time.delta()).just_finished() {
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
            for (column_index, _) in row.iter().enumerate() {
                let new_x = enemy_formation_width
                    + (column_index as f32 * (ENEMY_WIDTH + SPACE_BETWEEN_ENEMIES_X))
                    + (ENEMY_WIDTH / 2.0);

                let new_y = enemy_formation_height
                    - (row_index as f32 * (ENEMY_HEIGHT + SPACE_BETWEEN_ENEMIES_Y))
                    - (ENEMY_HEIGHT / 2.0);

                commands.spawn((
                    EnemyView,
                    Sprite {
                        image: asset_server.load(ENEMY_IMAGE),
                        custom_size: Some(Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT)),
                        color: Color::srgb(255.0, 255.0, 255.0),
                        ..default()
                    },
                    Transform::from_xyz(new_x, new_y, 0.0),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::enemy_formation::EnemyFormation;
    use crate::infrastructure::bevy::enemy_formation::{
        EnemyFormationResource, EnemyFormationView, EnemyView,
    };
    use bevy::app::{App, Startup, Update};
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::prelude::{IntoScheduleConfigs, Transform, With};
    use bevy::text::Font;
    use bevy::MinimalPlugins;
    use std::error::Error;

    fn get_first_enemy_coordinates(app: &mut App) -> Result<(f32, f32), Box<dyn Error>> {
        let translation = app
            .world_mut()
            .query_filtered::<&Transform, With<EnemyView>>()
            .iter(app.world())
            .next()
            .ok_or("First enemy Y at t1 not found")?
            .translation;
        Ok((translation.x, translation.y))
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));

        app.add_systems(Startup, EnemyFormationView::spawn_enemy_formation.chain());
        app.add_systems(Update, EnemyFormationView::on_move.chain());

        app.insert_resource(EnemyFormationResource(EnemyFormation::new()));

        app.init_asset::<Image>();
        app.init_asset::<Font>();

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
}
