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
    ui::{widget::Text, AlignItems, FlexDirection, JustifyContent, Node, UiRect, Val},
    utils::default,
};

use crate::infrastructure::bevy::header::FONT;
use crate::{domain::score::Score, infrastructure::bevy::header::HeaderView};

#[derive(Resource)]
pub struct ScoreResource(pub Score);

#[derive(Component)]
pub struct ScoreView;

impl ScoreView {
    pub fn spawn_score(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        score_res: Res<ScoreResource>,
        header_query: Query<Entity, With<HeaderView>>,
    ) {
        if let Ok(header) = header_query.single() {
            let font = asset_server.load(FONT);

            commands.entity(header).with_children(|parent| {
                parent
                    .spawn((
                        ScoreView,
                        Node {
                            width: Val::Percent(50.0),
                            height: Val::Px(50.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
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
                            Node {
                                height: Val::Percent(50.0),
                                ..default()
                            },
                            Text::new(score_res.0.get_current().to_string()),
                            TextFont {
                                font: font.clone(),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb_u8(51, 255, 3)),
                        ));
                    });
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::score::Score;
    use crate::infrastructure::bevy::header::HeaderView;
    use crate::infrastructure::bevy::score::{ScoreResource, ScoreView};
    use bevy::app::{App, Startup};
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::prelude::{Children, Entity, IntoScheduleConfigs, Text};
    use bevy::text::Font;
    use bevy::MinimalPlugins;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));

        app.add_systems(
            Startup,
            (HeaderView::spawn_header, ScoreView::spawn_score).chain(),
        );

        app.insert_resource(ScoreResource(Score::new()));

        app.init_asset::<Image>();
        app.init_asset::<Font>();

        app.update();

        app
    }

    #[test]
    fn should_display_the_score() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        let mut query = app.world_mut().query::<(&ScoreView, &Children)>();
        let (_, children) = query.single(app.world())?;

        let label = children
            .iter()
            .filter(|child| {
                if let Some(text) = app.world().get::<Text>(**child)
                    && text.0 == "Score: "
                {
                    return true;
                }
                false
            })
            .collect::<Vec<&Entity>>();

        let score = children
            .iter()
            .filter(|child| {
                if let Some(text) = app.world().get::<Text>(**child)
                    && text.0 == "0"
                {
                    return true;
                }
                false
            })
            .collect::<Vec<&Entity>>();

        assert!(!label.is_empty());
        assert!(!score.is_empty());

        Ok(())
    }
}
