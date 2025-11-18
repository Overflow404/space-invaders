use crate::{
    domain::{
        enemy_formation::EnemyFormation, lives::Lives, player::Player, score::Score,
        shield_formation::ShieldFormation,
    },
    infrastructure::{
        bevy::{
            enemy_formation::{
                EnemyFormationMovementTimer, EnemyFormationResource, EnemyFormationView,
            },
            game_area::GameAreaView,
            header::HeaderView,
            lives::{LivesResource, LivesView},
            player::{PlayerContainerView, PlayerResource, PlayerView},
            score::{ScoreResource, ScoreView},
            screen::ScreenView,
            shield_formation::{ShieldFormationResource, ShieldFormationView},
        },
        renderer::Renderer,
    },
};
use bevy::{prelude::*, window::WindowResolution};

#[derive(Default)]
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
                    Self::on_player_move,
                    Self::advance_enemies_on_tick,
                    Self::on_enemy_formation_move,
                ),
            )
            .run();
    }
}

impl BevyRenderer {
    fn on_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Camera2d);

        commands.insert_resource(ScoreResource(Score::new()));
        commands.insert_resource(LivesResource(Lives::new()));
        commands.insert_resource(PlayerResource(Player::new()));
        commands.insert_resource(ShieldFormationResource(ShieldFormation::new()));
        commands.insert_resource(EnemyFormationResource(EnemyFormation::new()));
        commands.insert_resource(EnemyFormationMovementTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )));

        ScreenView::render(&mut commands, &asset_server);
    }

    fn on_enemy_formation_move(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        enemy_formation_res: Res<EnemyFormationResource>,
        container_query: Query<Entity, With<EnemyFormationView>>,
    ) {
        if enemy_formation_res.is_changed() {
            if let Ok(container) = container_query.single() {
                commands.entity(container).despawn_children();
                commands
                    .entity(container)
                    .with_children(|formation_container| {
                        EnemyFormationView::on_update(
                            formation_container,
                            &asset_server,
                            &enemy_formation_res,
                        );
                    });
            }
        }
    }

    fn advance_enemies_on_tick(
        time: Res<Time>,
        mut enemy_formation_res: ResMut<EnemyFormationResource>,
        mut timer: ResMut<EnemyFormationMovementTimer>,
    ) {
        if timer.0.tick(time.delta()).just_finished() {
            enemy_formation_res.0.advance_enemies();
        }
    }

    pub fn on_player_move(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut player_query: Query<(&mut Node, &ComputedNode), With<PlayerView>>,
        parent_query: Query<
            (&ComputedNode, &Node),
            (With<PlayerContainerView>, Without<PlayerView>),
        >,
        windows: Query<&Window>,
        time: Res<Time>,
    ) {
        let window = windows.single().unwrap();
        let scale_factor = window.scale_factor();

        let (parent_computed, parent) = if let Ok(res) = parent_query.single() {
            res
        } else {
            return;
        };

        let scaled_parent_width = parent_computed.size().x / scale_factor;

        let get_val_from_px = |val: &Val| match val {
            Val::Px(px) => *px,
            _ => 0.0,
        };

        let pad_left = get_val_from_px(&parent.padding.left);
        let pad_right = get_val_from_px(&parent.padding.right);

        for (mut player, player_computed) in player_query.iter_mut() {
            let current_left = get_val_from_px(&player.left);

            let speed = 300.0;
            let delta = speed * time.delta_secs();

            let mut new_left = current_left;

            if keyboard.pressed(KeyCode::ArrowLeft) {
                new_left -= delta;
            }
            if keyboard.pressed(KeyCode::ArrowRight) {
                new_left += delta;
            }

            let scaled_player_width = player_computed.size().x / scale_factor;
            let half_container = scaled_parent_width / 2.0;
            let half_player = scaled_player_width / 2.0;

            let min_bound = -half_container + pad_left + half_player;
            let max_bound = half_container - pad_right - half_player;

            new_left = new_left.clamp(min_bound, max_bound);

            player.left = Val::Px(new_left);
        }
    }
}
