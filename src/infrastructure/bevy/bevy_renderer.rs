use crate::infrastructure::bevy::header::HEADER_HEIGHT;
use crate::{
    domain::{
        enemy_formation::EnemyFormation, lives::Lives, player::Player, score::Score,
        shield_formation::ShieldFormation,
    },
    infrastructure::{
        bevy::{
            enemy_formation::{
                ENEMY_FORMATION_SPEED, EnemyFormationMovementTimer, EnemyFormationResource,
                EnemyFormationView,
            },
            game_area::GameAreaView,
            header::HeaderView,
            lives::{LivesResource, LivesView},
            player::{PlayerResource, PlayerView},
            projectile::{PROJECTILE_DURATION, ProjectileMovementTimer},
            score::{ScoreResource, ScoreView},
            shield_formation::{ShieldFormationResource, ShieldFormationView},
        },
        renderer::Renderer,
    },
};
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, PostUpdate, Startup, Update};
use bevy::camera::{Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::prelude::{
    Changed, Commands, IntoScheduleConfigs, Query, ResMut, Timer, Transform, UiScale,
};
use bevy::time::TimerMode;
use bevy::utils::default;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution};

pub struct BevyRenderer;

const WINDOW_NAME: &str = "Space Invaders";
pub(crate) const WINDOW_WIDTH: f32 = 1200.0;
pub(crate) const WINDOW_HEIGHT: f32 = 700.0;

impl Renderer for BevyRenderer {
    fn render(&self) {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                    title: WINDOW_NAME.to_string(),
                    present_mode: PresentMode::Fifo,
                    ..default()
                }),
                ..default()
            }))
            .add_systems(
                Startup,
                (
                    Self::on_startup,
                    HeaderView::spawn_header.after(Self::on_startup),
                    LivesView::spawn_lives.after(HeaderView::spawn_header),
                    ScoreView::spawn_score.after(HeaderView::spawn_header),
                    GameAreaView::spawn_game_area.after(Self::on_startup),
                    EnemyFormationView::spawn_enemy_formation.after(GameAreaView::spawn_game_area),
                    ShieldFormationView::spawn_shields
                        .after(EnemyFormationView::spawn_enemy_formation),
                    PlayerView::spawn_player.after(ShieldFormationView::spawn_shields),
                ),
            )
            .add_systems(
                Update,
                (
                    PlayerView::on_move,
                    PlayerView::on_fire,
                    EnemyFormationView::on_move,
                    EnemyFormationView::advance_on_tick,
                ),
            )
            .add_systems(
                PostUpdate,
                (GameAreaView::resize_game_area, Self::update_window_scale),
            )
            .run();
    }
}

impl Default for BevyRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl BevyRenderer {
    pub fn new() -> Self {
        Self
    }
    fn on_startup(mut commands: Commands) {
        commands.spawn((
            Camera2d,
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: WINDOW_WIDTH,
                    min_height: WINDOW_HEIGHT,
                },
                ..OrthographicProjection::default_2d()
            }),
            Transform::from_xyz(0.0, HEADER_HEIGHT, 0.0),
        ));

        commands.insert_resource(ScoreResource(Score::new()));
        commands.insert_resource(LivesResource(Lives::new()));
        commands.insert_resource(PlayerResource(Player::new()));
        commands.insert_resource(ShieldFormationResource(ShieldFormation::new()));
        commands.insert_resource(EnemyFormationResource(EnemyFormation::new()));
        commands.insert_resource(EnemyFormationMovementTimer(Timer::from_seconds(
            ENEMY_FORMATION_SPEED,
            TimerMode::Repeating,
        )));
        commands.insert_resource(ProjectileMovementTimer(Timer::from_seconds(
            PROJECTILE_DURATION,
            TimerMode::Once,
        )));
    }

    fn update_window_scale(
        window_query: Query<&Window, Changed<Window>>,
        mut ui_scale: ResMut<UiScale>,
    ) {
        if let Ok(window) = window_query.single() {
            let scale_x = window.width() / WINDOW_WIDTH;
            let scale_y = window.height() / WINDOW_HEIGHT;

            ui_scale.0 = scale_x.min(scale_y);
        }
    }
}
