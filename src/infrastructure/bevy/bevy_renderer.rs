use crate::infrastructure::bevy::footer::FooterView;
use crate::infrastructure::bevy::header::HEADER_HEIGHT;
use crate::infrastructure::bevy::player_projectile::PlayerProjectileView;
use crate::{
    domain::{
        enemy_formation::EnemyFormation, lives::Lives, player::Player, score::Score,
        shield_formation::ShieldFormation,
    },
    infrastructure::{
        bevy::{
            enemy_formation::{
                EnemyFormationMovementTimer, EnemyFormationResource, EnemyFormationView,
                ENEMY_FORMATION_STEP_DURATION,
            },
            game_area::GameAreaView,
            header::HeaderView,
            lives::{LivesResource, LivesView},
            player::{PlayerResource, PlayerView},
            player_projectile::{PlayerProjectileMovementTimer, PROJECTILE_DURATION},
            score::{ScoreResource, ScoreView},
            shield_formation::{ShieldFormationResource, ShieldFormationView},
        },
        renderer::Renderer,
    },
};
use bevy::app::{App, Plugin, PluginGroup, PostUpdate, Startup, Update};
use bevy::camera::{Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::TimerMode;
use bevy::utils::default;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution};
use bevy::DefaultPlugins;

pub struct BevyRenderer;

const WINDOW_NAME: &str = "Space Invaders";
pub(crate) const WINDOW_WIDTH: f32 = 1200.0;
pub(crate) const WINDOW_HEIGHT: f32 = 700.0;

pub struct SpaceInvadersPlugin;

impl Plugin for SpaceInvadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (Self::spawn_camera, Self::inject_resources))
            .add_systems(
                Startup,
                (
                    HeaderView::spawn_header,
                    ScoreView::spawn_score,
                    LivesView::spawn_lives,
                    GameAreaView::spawn_game_area,
                    EnemyFormationView::spawn_enemy_formation,
                    ShieldFormationView::spawn_shields,
                    PlayerView::spawn_player,
                    FooterView::spawn_footer,
                )
                    .chain()
                    .after(Self::spawn_camera),
            )
            .add_systems(
                Update,
                (
                    PlayerView::on_move,
                    PlayerView::on_fire,
                    PlayerProjectileView::on_move,
                    PlayerProjectileView::on_destroy,
                    EnemyFormationView::on_move,
                    EnemyFormationView::advance_on_tick,
                    EnemyFormationView::handle_collisions,
                ),
            )
            .add_systems(
                PostUpdate,
                (GameAreaView::resize_game_area, Self::update_window_scale),
            );
    }
}

impl SpaceInvadersPlugin {
    fn spawn_camera(mut commands: Commands) {
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
    }

    fn inject_resources(mut commands: Commands) {
        commands.insert_resource(ScoreResource(Score::new()));
        commands.insert_resource(LivesResource(Lives::new()));
        commands.insert_resource(PlayerResource(Player::new()));
        commands.insert_resource(ShieldFormationResource(ShieldFormation::new()));
        commands.insert_resource(EnemyFormationResource(EnemyFormation::new()));
        commands.insert_resource(EnemyFormationMovementTimer(Timer::from_seconds(
            ENEMY_FORMATION_STEP_DURATION,
            TimerMode::Repeating,
        )));
        commands.insert_resource(PlayerProjectileMovementTimer(Timer::from_seconds(
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

impl Renderer for BevyRenderer {
    fn render(&self) {
        App::new()
            .add_plugins(Self::window_plugin_config())
            .add_plugins(SpaceInvadersPlugin)
            .run();
    }
}

impl BevyRenderer {
    pub fn new() -> Self {
        Self
    }

    fn window_plugin_config() -> impl PluginGroup {
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                    title: WINDOW_NAME.to_string(),
                    present_mode: PresentMode::Fifo,
                    ..default()
                }),
                ..default()
            })
            .disable::<LogPlugin>()
    }
}

impl Default for BevyRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy_formation::EnemyView;

    fn setup() -> App {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .add_plugins(WindowPlugin::default())
            .add_plugins(SpaceInvadersPlugin)
            .init_asset::<Image>()
            .init_asset::<Font>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<UiScale>()
            .insert_resource(ScoreResource(Score::new()))
            .insert_resource(LivesResource(Lives::new()))
            .insert_resource(PlayerResource(Player::new()))
            .insert_resource(ShieldFormationResource(ShieldFormation::new()))
            .insert_resource(EnemyFormationResource(EnemyFormation::new()))
            .insert_resource(EnemyFormationMovementTimer(Timer::from_seconds(
                ENEMY_FORMATION_STEP_DURATION,
                TimerMode::Repeating,
            )))
            .insert_resource(PlayerProjectileMovementTimer(Timer::from_seconds(
                PROJECTILE_DURATION,
                TimerMode::Once,
            )));

        app.update();
        app
    }

    #[test]
    fn should_spawn_all_the_elements() {
        let mut app = setup();

        let world = &mut app.world_mut();

        fn assert_exists<T: Component>(world: &mut World, name: &str, expected_count: usize) {
            let actual_count = world.query::<&T>().iter(world).count();
            assert_eq!(
                actual_count, expected_count,
                "Expected {} entities with component `{}`",
                expected_count, name
            );
        }

        assert_exists::<HeaderView>(world, "HeaderView", 1);

        assert_exists::<ScoreView>(world, "ScoreView", 1);
        assert_exists::<LivesView>(world, "LivesView", 1);

        assert_exists::<GameAreaView>(world, "GameAreaView", 1);

        assert_exists::<EnemyView>(world, "EnemyView", 55);
        assert_exists::<PlayerView>(world, "PlayerView", 1);
        assert_exists::<FooterView>(world, "FooterView", 1);
    }
}
