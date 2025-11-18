use crate::{
    domain::{enemy_formation::EnemyFormation, lives::Lives, player::Player, score::Score},
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
            shield_formation::ShieldFormationView,
        },
        renderer::Renderer,
    },
};
use bevy::{
    ecs::{error::info, relationship::Relationship},
    prelude::*,
    window::WindowResolution,
};

#[derive(Default)]
pub struct BevyRenderer;

impl Renderer for BevyRenderer {
    fn render(&self) {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1200, 700),
                    title: "Space Invaders".to_string(),
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
                    Self::on_advance_enemies,
                    Self::update_enemy_formation_display,
                ),
            )
            .run();
    }
}

impl BevyRenderer {
    fn update_enemy_formation_display(
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
    fn on_advance_enemies(
        time: Res<Time>,
        mut enemy_formation_res: ResMut<EnemyFormationResource>,
        mut timer: ResMut<EnemyFormationMovementTimer>,
    ) {
        if timer.0.tick(time.delta()).just_finished() {
            enemy_formation_res.0.advance_enemies();
        }
    }

    fn on_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Camera2d);

        commands.insert_resource(ScoreResource(Score::new()));
        commands.insert_resource(LivesResource(Lives::new()));
        commands.insert_resource(PlayerResource(Player::new()));
        commands.insert_resource(EnemyFormationResource(EnemyFormation::new()));
        commands.insert_resource(EnemyFormationMovementTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )));

        ScreenView::render(&mut commands, &asset_server);
    }

    pub fn on_player_move(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut player_query: Query<(&mut Node, &ComputedNode), With<PlayerView>>,
        container_query: Query<
            (&ComputedNode, &Node),
            (With<PlayerContainerView>, Without<PlayerView>),
        >,
        windows: Query<&Window>,
        time: Res<Time>,
    ) {
        let window = windows.single().unwrap();
        let scale_factor = window.scale_factor();

        let (container_computed, container_style) = if let Ok(res) = container_query.single() {
            res
        } else {
            return;
        };

        let unscaled_container_width = container_computed.size().x / scale_factor;

        let get_val_px = |val: &Val| match val {
            Val::Px(px) => *px,
            _ => 0.0,
        };

        let pad_left = get_val_px(&container_style.padding.left);
        let pad_right = get_val_px(&container_style.padding.right);

        for (mut node, player_computed) in player_query.iter_mut() {
            let current_left = get_val_px(&node.left);

            let speed = 300.0;
            let delta = speed * time.delta_secs();

            let mut new_left = current_left;

            if keyboard.pressed(KeyCode::ArrowLeft) {
                new_left -= delta;
            }
            if keyboard.pressed(KeyCode::ArrowRight) {
                new_left += delta;
            }

            let player_width_logical = player_computed.size().x / scale_factor;
            let half_container = unscaled_container_width / 2.0;
            let half_player = player_width_logical / 2.0;

            let min_bound = -half_container + pad_left + half_player;

            let max_bound = half_container - pad_right - half_player;

            new_left = new_left.clamp(min_bound, max_bound);

            node.left = Val::Px(new_left);
        }
    }
}
