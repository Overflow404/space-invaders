use crate::{
    domain::{lives::Lives, player::Player, score::Score},
    infrastructure::{
        bevy::{
            game_area::player::{PlayerComponent, PlayerView},
            header::{
                lives::{LivesComponent, LivesView},
                score::{ScoreComponent, ScoreView},
            },
        },
        renderer::Renderer,
    },
};
use bevy::{prelude::*, window::WindowResolution};

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
                (Self::setup_camera, Self::spawn_components, Self::startup).chain(),
            )
            .add_systems(Update, Self::move_player_with_bounds)
            // .add_systems(
            //     Update,
            //     (Self::handle_input, Self::update_score_display).chain(),
            // )
            .run();
    }
}

impl BevyRenderer {
    fn setup_camera(mut commands: Commands) {
        commands.spawn(Camera2d);
    }

    fn spawn_components(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            ScoreComponent(Score::new()),
            LivesComponent(Lives::new()),
            PlayerComponent(Player::new()),
        ));
    }

    fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
        Self::render_background(&mut commands, &asset_server);
        Self::render_game(&mut commands, &asset_server);
    }

    fn render_game(commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands
            .spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    left: Val::Px(45.0),
                    right: Val::Px(45.0),
                    top: Val::Px(45.0),
                    bottom: Val::Px(45.0),
                    ..default()
                },
                ..default()
            },))
            .with_children(|parent| {
                Self::render_header(parent, asset_server);
                Self::render_game_area(parent, asset_server);
            });
    }

    // fn update_score_display(
    //     score_header: Query<&ScoreHeader, Changed<ScoreHeader>>,
    //     mut text_query: Query<&mut Text, With<ScoreText>>,
    // ) {
    //     if let Ok(score_header) = score_header.single() {
    //         for mut text in text_query.iter_mut() {
    //             **text = score_header.0.get_current().to_string();
    //         }
    //     }
    // }

    // fn handle_player_movement(
    //     keyboard: Res<ButtonInput<KeyCode>>,
    //     mut player_component: Query<&mut PlayerComponent>,
    // ) {
    //     if keyboard.just_pressed(KeyCode::ArrowRight) {
    //         if let Ok(mut score_header) = player_component.single_mut() {
    //             score_header.0.increment(1);
    //         }
    //     }
    // }
    //
    fn move_player_with_bounds(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut player_query: Query<&mut Node, With<PlayerView>>,
        time: Res<Time>,
        windows: Query<&Window>,
    ) {
        if let Ok(window) = windows.single() {
            for mut node in player_query.iter_mut() {
                let current_left = match node.margin.left {
                    Val::Px(px) => px,
                    _ => 0.0,
                };

                let speed = 300.0;
                let movement = speed * time.delta_secs();

                // Calculate new position
                let mut new_left = current_left;

                if keyboard.pressed(KeyCode::ArrowLeft) {
                    new_left -= movement;
                }
                if keyboard.pressed(KeyCode::ArrowRight) {
                    new_left += movement;
                }

                // Get player width (assuming 35px from your setup)
                let player_width = 35.0;

                // Clamp position to screen bounds
                // let min_left = 0.0;
                // let max_left = window.width() - player_width;
                // new_left = new_left.clamp(min_left, max_left);

                node.margin.left = Val::Px(new_left);
            }
        }
    }

    fn render_background(commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn((
            Sprite {
                image: asset_server.load("tv.png"),
                custom_size: Some(Vec2::new(1200.0, 700.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -1.0),
        ));
    }

    fn render_header(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>) {
        let font = asset_server.load("pixeled.ttf");

        parent
            .spawn((Node {
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
            },))
            .with_children(|parent| {
                Self::render_score_header(parent, &font);
                Self::render_lives_header(parent, asset_server, &font);
            });
    }

    fn render_score_header(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
        parent
            .spawn((Node {
                width: Val::Percent(50.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },))
            .with_children(|score_section| {
                score_section.spawn((
                    Node {
                        height: Val::Percent(50.0),
                        margin: UiRect::right(Val::Px(20.0)),
                        ..default()
                    },
                    Text::new("Score: "),
                    TextFont {
                        font: font.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                score_section.spawn((
                    ScoreView,
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

    fn render_lives_header(
        parent: &mut ChildSpawnerCommands,
        asset_server: &Res<AssetServer>,
        font: &Handle<Font>,
    ) {
        parent
            .spawn((
                LivesView,
                Node {
                    width: Val::Percent(50.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
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
                            margin: UiRect::right(Val::Px(25.0)),
                            ..default()
                        },
                    ));
                }
            });
    }

    fn render_game_area(parent: &mut ChildSpawnerCommands, asset_server: &Res<AssetServer>) {
        parent
            .spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                margin: UiRect::top(Val::Px(30.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },))
            .with_children(|parent| {
                Self::enemies_game_area(parent, asset_server);
                Self::shields_game_area(parent, asset_server);
                Self::player_game_area(parent, asset_server);
            });
    }

    fn enemies_game_area(parent: &mut ChildSpawnerCommands, asset_server: &Res<'_, AssetServer>) {
        parent
            .spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },))
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

    fn player_game_area(parent: &mut ChildSpawnerCommands, asset_server: &Res<'_, AssetServer>) {
        parent
            .spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..default()
            },))
            .with_children(|game_area| {
                game_area.spawn((
                    PlayerView,
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

    fn shields_game_area(parent: &mut ChildSpawnerCommands, asset_server: &Res<'_, AssetServer>) {
        parent.spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(20.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..default()
        },));
    }
}
