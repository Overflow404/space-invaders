use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        resource::Resource,
        system::{Commands, Query, Res},
    },
    ui::{
        AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, UiRect, Val,
        widget::ImageNode,
    },
    utils::default,
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
}
