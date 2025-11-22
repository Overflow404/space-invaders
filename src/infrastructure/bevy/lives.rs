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
    text::{TextColor, TextFont},
    ui::{
        widget::{ImageNode, Text}, AlignItems, FlexDirection, JustifyContent, Node, UiRect,
        Val,
    },
    utils::default,
};

use crate::infrastructure::bevy::header::FONT;
use crate::infrastructure::bevy::player::PLAYER_IMAGE;
use crate::{domain::lives::Lives, infrastructure::bevy::header::HeaderView};
#[derive(Resource)]
pub struct LivesResource(pub Lives);

#[derive(Component)]
pub struct LivesView;

impl LivesView {
    pub fn spawn_lives(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        lives_res: Res<LivesResource>,
        header_query: Query<Entity, With<HeaderView>>,
    ) {
        if let Ok(header) = header_query.single() {
            let font = asset_server.load(FONT);
            let remaining_lives = lives_res.0.get_current();

            commands.entity(header).with_children(|parent| {
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
                        for _ in 0..remaining_lives {
                            lives_section.spawn((
                                ImageNode {
                                    image: asset_server.load(PLAYER_IMAGE),
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
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::lives::Lives;
    use crate::infrastructure::bevy::header::HeaderView;
    use crate::infrastructure::bevy::lives::{LivesResource, LivesView};
    use bevy::app::{App, Startup};
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::prelude::{Children, Entity, ImageNode, IntoScheduleConfigs, Text};
    use bevy::text::Font;
    use bevy::MinimalPlugins;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));

        app.add_systems(
            Startup,
            (HeaderView::spawn_header, LivesView::spawn_lives).chain(),
        );

        app.insert_resource(LivesResource(Lives::new()));

        app.init_asset::<Image>();
        app.init_asset::<Font>();

        app.update();

        app
    }

    #[test]
    fn should_display_the_lives() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        let mut query = app.world_mut().query::<(&LivesView, &Children)>();
        let (_, children) = query.single(app.world())?;

        let label = children
            .iter()
            .filter(|child| {
                if let Some(text) = app.world().get::<Text>(**child)
                    && text.0 == "LIVES"
                {
                    return true;
                }
                false
            })
            .collect::<Vec<&Entity>>();

        let lives = children
            .iter()
            .filter(|child| app.world().get::<ImageNode>(**child).is_some())
            .collect::<Vec<&Entity>>();

        assert!(!label.is_empty());
        assert_eq!(lives.len(), 3);

        Ok(())
    }
}
