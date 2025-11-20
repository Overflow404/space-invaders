use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    time::Time,
    transform::components::GlobalTransform,
    ui::{
        widget::ImageNode, AlignItems, BackgroundColor, ComputedNode, FlexDirection, JustifyContent, Node,
        UiRect, Val,
    },
    utils::default,
    window::Window,
};
use tracing::info;

use crate::{
    domain::player::Player,
    infrastructure::bevy::{
        game_area::GameAreaView,
        projectile::{ProjectileMovementTimer, ProjectileView},
    },
};

#[derive(Resource)]
pub struct PlayerResource(pub Player);

#[derive(Component)]
pub struct PlayerView;

#[derive(Component)]
pub struct PlayerContainerView;

impl PlayerView {
    pub fn new() -> Self {
        PlayerView
    }

    pub fn spawn_player(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        game_area_query: Query<Entity, With<GameAreaView>>,
    ) {
        if let Ok(game_area) = game_area_query.single() {
            commands.entity(game_area).with_children(|parent| {
                parent
                    .spawn((
                        PlayerContainerView,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(10.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::FlexEnd,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb_u8(200, 200, 200)),
                    ))
                    .with_children(|player_container| {
                        player_container.spawn((
                            Self,
                            ImageNode::new(asset_server.load("player-green.png")),
                            Node {
                                height: Val::Px(35.0),
                                width: Val::Px(70.0),
                                margin: UiRect::bottom(Val::Px(20.0)),
                                ..default()
                            },
                        ));
                    });
            });
        }
    }

    fn get_val_from_px(val: &Val) -> f32 {
        match val {
            Val::Px(px) => *px,
            _ => 0.0,
        }
    }

    pub fn on_move(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut player_query: Query<(&mut Node, &ComputedNode), With<PlayerView>>,
        parent_query: Query<(&ComputedNode), (With<PlayerContainerView>, Without<PlayerView>)>,
        windows: Query<&Window>,
        time: Res<Time>,
    ) {
        let window = windows.single().unwrap();
        let scale_factor = window.scale_factor();

        let (parent_computed) = if let Ok(res) = parent_query.single() {
            res
        } else {
            return;
        };

        let scaled_parent_width = parent_computed.size().x / scale_factor;

        for (mut player, player_computed) in player_query.iter_mut() {
            let current_left = Self::get_val_from_px(&player.left);

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

            let min_bound = -half_container + half_player;
            let max_bound = half_container - half_player;

            new_left = new_left.clamp(min_bound, max_bound);

            player.left = Val::Px(new_left);
            info!("moved to {:?}", player.left);
        }
    }

    pub fn on_fire(
        mut commands: Commands,
        time: Res<Time>,
        keyboard: Res<ButtonInput<KeyCode>>,
        mut player_res: ResMut<PlayerResource>,
        mut timer: ResMut<ProjectileMovementTimer>,
        player_query: Query<&Node, With<PlayerView>>,
        parent_query: Query<&ComputedNode, (With<PlayerContainerView>, Without<PlayerView>)>,
        windows: Query<&Window>,
    ) {
        let window = windows.single().unwrap();
        let scale_factor = window.scale_factor();

        let parent_computed = if let Ok(res) = parent_query.single() {
            res
        } else {
            return;
        };

        let scaled_parent_width = parent_computed.size().x / scale_factor;

        let half_container = scaled_parent_width / 2.0;

        if keyboard.pressed(KeyCode::Space) && !player_res.0.is_firing() {
            for player_node in player_query.iter() {
                let player_left = Self::get_val_from_px(&player_node.left);

                let projectile_x = player_left + half_container + 10.0;

                let projectile_view = ProjectileView::new(projectile_x, 150.0);
                commands.spawn(projectile_view.spawn_projectile());
                player_res.0.toggle_fire();
            }
        }

        if player_res.0.is_firing() && !timer.0.just_finished() {
            info!("ADVANCING PROJECTILE");
            timer.0.tick(time.delta());
        }

        if timer.0.just_finished() {
            player_res.0.toggle_fire();
            timer.0.reset();
            info!("END PROJECTILE");
        }
    }
}
