use crate::{
    domain::{
        enemy_formation::EnemyFormation, lives::Lives, player::Player, score::Score,
        shield_formation::ShieldFormation,
    },
    infrastructure::{
        bevy::{
            enemy_formation::{
                EnemyFormationMovementTimer, EnemyFormationResource, EnemyFormationView,
                ONE_ERA_IN_SECONDS,
            },
            game_area::GameAreaView,
            header::HeaderView,
            lives::{LivesResource, LivesView},
            player::{PlayerResource, PlayerView},
            projectile::{PROJECTILE_TIME_IN_SECONDS, ProjectileMovementTimer},
            score::{ScoreResource, ScoreView},
            screen::ScreenView,
            shield_formation::{ShieldFormationResource, ShieldFormationView},
        },
        renderer::Renderer,
    },
};
use bevy::{prelude::*, window::WindowResolution};

pub struct BevyRenderer;

const WINDOW_NAME: &str = "Space Invaders";
const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 700;

impl Renderer for BevyRenderer {

    fn render(&self) {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    title: WINDOW_NAME.to_string(),
                    resizable: false,
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
            .run();
    }
}

impl BevyRenderer {

    pub fn new() -> Self {
        Self
    }
    fn on_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Camera2d);

        commands.insert_resource(ScoreResource(Score::new()));
        commands.insert_resource(LivesResource(Lives::new()));
        commands.insert_resource(PlayerResource(Player::new()));
        commands.insert_resource(ShieldFormationResource(ShieldFormation::new()));
        commands.insert_resource(EnemyFormationResource(EnemyFormation::new()));
        commands.insert_resource(EnemyFormationMovementTimer(Timer::from_seconds(
            ONE_ERA_IN_SECONDS,
            TimerMode::Repeating,
        )));
        commands.insert_resource(ProjectileMovementTimer(Timer::from_seconds(
            PROJECTILE_TIME_IN_SECONDS,
            TimerMode::Once,
        )));

        ScreenView::render(&mut commands, &*asset_server);
    }
}
