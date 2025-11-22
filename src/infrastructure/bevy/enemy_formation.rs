use crate::domain::enemy_formation::{COLUMNS, EnemyFormation, X_STEPS};
use crate::infrastructure::bevy::game_area::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use crate::infrastructure::bevy::header::HEADER_HEIGHT;
use bevy::prelude::*;

pub const ONE_ERA_IN_SECONDS: f32 = 0.6;

struct FormationConfig {
    enemy_width: f32,
    enemy_height: f32,
    space_between_enemies_x: f32,
    space_between_enemies_y: f32,
    vertical_drop: f32,
}

const CONFIG: FormationConfig = FormationConfig {
    enemy_width: 60.0,
    enemy_height: 40.0,
    space_between_enemies_x: 15.0,
    space_between_enemies_y: 15.0,
    vertical_drop: 15.0,
};

#[derive(Resource)]
pub struct EnemyFormationResource(pub EnemyFormation);

#[derive(Resource)]
pub struct EnemyFormationMovementTimer(pub Timer);

#[derive(Component)]
pub struct EnemyFormationView;

#[derive(Component)]
pub struct Enemy;

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
        enemy_query: Query<Entity, With<Enemy>>,
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

    fn calculate_step_x(alien_width: f32, gap_x: f32) -> f32 {
        let n_aliens = COLUMNS as f32;
        let n_gaps = (COLUMNS - 1) as f32;

        let n_steps = (X_STEPS - COLUMNS) as f32;

        let block_width = (n_aliens * alien_width) + (n_gaps * gap_x);

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

        let step_size_x =
            Self::calculate_step_x(CONFIG.enemy_width, CONFIG.space_between_enemies_x);

        let enemy_formation_start_x = -(GAME_AREA_WIDTH / 2.0);
        let enemy_formation_start_y = (GAME_AREA_HEIGHT / 2.0) - HEADER_HEIGHT;

        let enemy_formation_width =
            enemy_formation_start_x + (enemy_formation_x as f32 * step_size_x);
        let enemy_formation_height =
            enemy_formation_start_y - (enemy_formation_y as f32 * CONFIG.vertical_drop);

        for (row_index, row) in enemies.iter().enumerate() {
            for (column_index, enemy_slot) in row.iter().enumerate() {
                if enemy_slot.is_some() {
                    let new_x = enemy_formation_width
                        + (column_index as f32
                            * (CONFIG.enemy_width + CONFIG.space_between_enemies_x))
                        + (CONFIG.enemy_width / 2.0);

                    let new_y = enemy_formation_height
                        - (row_index as f32
                            * (CONFIG.enemy_height + CONFIG.space_between_enemies_y))
                        - (CONFIG.enemy_height / 2.0);

                    commands.spawn((
                        Enemy,
                        Sprite {
                            image: asset_server.load("red.png"),
                            custom_size: Some(Vec2::new(CONFIG.enemy_width, CONFIG.enemy_height)),
                            ..default()
                        },
                        Transform::from_xyz(new_x, new_y, 0.0),
                    ));
                }
            }
        }
    }
}
