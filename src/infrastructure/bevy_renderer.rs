use crate::{
    domain::{enemy::Enemy, player::Player},
    infrastructure::renderer::Renderer,
};
use bevy::{prelude::*, window::WindowResolution};

pub struct BevyRenderer;

impl Renderer for BevyRenderer {
    fn start_game_loop(&self) {
        App::new()
            .add_systems(Startup, Self::render)
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1200, 800),
                    title: "Space invaders".to_string(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }))
            .run();
    }
}

impl BevyRenderer {
    pub fn new() -> Self {
        BevyRenderer
    }

    fn render(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Camera2d);
        let font = asset_server.load("pixeled.ttf");

        commands
            .spawn(Self::main_window())
            .with_children(Self::header(&asset_server, &font))
            .with_children(Self::game_area(&asset_server));
    }

    fn main_window() -> (Node, BackgroundColor) {
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            BackgroundColor(Color::BLACK),
        )
    }

    fn header(
        asset_server: &Res<'_, AssetServer>,
        font: &Handle<Font>,
    ) -> impl FnOnce(&mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>) {
        |window| {
            window
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect {
                            left: Val::Px(15.0),
                            right: Val::Px(15.0),
                            ..default()
                        },
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(Self::score_header(font))
                .with_children(Self::lives_header(asset_server, font));
        }
    }

    fn lives_header(
        asset_server: &Res<'_, AssetServer>,
        font: &Handle<Font>,
    ) -> impl FnOnce(&mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>) {
        |header| {
            header
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(|lives_section| {
                    lives_section.spawn((
                        Node {
                            height: Val::Percent(50.0),
                            margin: UiRect::right(Val::Px(20.0)),
                            ..default()
                        },
                        Text::new("LIVES"),
                        TextFont {
                            font: font.clone(),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    for _ in 0..3 {
                        lives_section.spawn((
                            ImageNode {
                                image: asset_server.load("player-green.png"),
                                ..default()
                            },
                            Node {
                                height: Val::Percent(35.0),
                                margin: UiRect::right(Val::Px(12.0)),
                                ..default()
                            },
                        ));
                    }
                });
        }
    }

    fn score_header(
        font: &Handle<Font>,
    ) -> impl FnOnce(&mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>) {
        |header| {
            header
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(|score_section| {
                    score_section.spawn((
                        Node {
                            height: Val::Percent(50.0),
                            margin: UiRect::right(Val::Px(20.0)),
                            ..default()
                        },
                        Text::new("SCORE"),
                        TextFont {
                            font: font.clone(),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    score_section.spawn((
                        Node {
                            height: Val::Percent(50.0),
                            ..default()
                        },
                        Text::new("0"),
                        TextFont {
                            font: font.clone(),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb_u8(51, 255, 3)),
                    ));
                });
        }
    }

    fn game_area(
        asset_server: &Res<'_, AssetServer>,
    ) -> impl FnOnce(&mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>) {
        |window| {
            window
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::top(Val::Px(30.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(Self::enemies_game_area(asset_server))
                .with_children(Self::shields_game_area())
                .with_children(Self::player_game_area(asset_server));
        }
    }

    fn player_game_area(
        asset_server: &Res<'_, AssetServer>,
    ) -> impl FnOnce(&mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>) {
        |player_area| {
            player_area
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(10.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(|game_area| {
                    game_area.spawn((
                        Player::new(35.0, 35.0),
                        ImageNode {
                            image: asset_server.load("player-green.png"),
                            ..default()
                        },
                        Node {
                            height: Val::Px(35.0),
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                    ));
                });
        }
    }

    fn shields_game_area()
    -> impl FnOnce(&mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>) {
        |shields_area| {
            shields_area.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ));
        }
    }

    fn enemies_game_area(
        asset_server: &Res<'_, AssetServer>,
    ) -> impl FnOnce(&mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>) {
        |enemy_area| {
            enemy_area
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(70.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(|game_area| {
                    for _row in 0..5 {
                        game_area
                            .spawn((
                                Node {
                                    width: Val::Percent(70.0),
                                    height: Val::Percent(15.0),
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceAround,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                            ))
                            .with_children(|row_container| {
                                for _ in 0..11 {
                                    row_container.spawn((
                                        Enemy::new(1),
                                        ImageNode {
                                            image: asset_server.load("red.png"),
                                            ..default()
                                        },
                                        Node {
                                            width: Val::Px(30.0),
                                            height: Val::Px(30.0),
                                            ..default()
                                        },
                                    ));
                                }
                            });
                    }
                });
        }
    }
}
