use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        resource::Resource,
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    time::Time,
    ui::{
        AlignItems, BackgroundColor, ComputedNode, FlexDirection, JustifyContent, Node, UiRect,
        Val, widget::ImageNode,
    },
    utils::default,
    window::Window,
};

use crate::{domain::player::Player, infrastructure::bevy::game_area::GameAreaView};

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

    pub fn on_move(
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

    pub fn on_fire() {
        //TODO funny part :D
    }
}
