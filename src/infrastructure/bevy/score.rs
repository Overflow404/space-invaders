use crate::infrastructure::bevy::enemy::EnemyKilledMessage;
use crate::infrastructure::bevy::header::FONT;
use crate::{domain::score::Score, infrastructure::bevy::header::HeaderView};
use bevy::prelude::{DetectChanges, MessageReader, ResMut};
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

#[derive(Resource)]
pub struct ScoreResource(pub Score);

#[derive(Component)]
pub struct ScoreViewValue;

#[derive(Component)]
pub struct ScoreViewLabel;

impl ScoreViewValue {
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
                            ScoreViewLabel,
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
                            ScoreViewValue,
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

    pub fn on_change(
        score_resource: Res<ScoreResource>,
        mut score_query: Query<&mut Text, With<ScoreViewValue>>,
    ) {
        if score_resource.is_changed() {
            for mut text in &mut score_query {
                text.0 = score_resource.0.get_current().to_string();
            }
        }
    }

    pub fn on_enemy_killed_message(
        mut enemy_killed_message: MessageReader<EnemyKilledMessage>,
        mut score_resource: ResMut<ScoreResource>,
    ) {
        for _ in enemy_killed_message.read() {
            score_resource.0.increment(10);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::score::Score;
    use crate::infrastructure::bevy::enemy::EnemyKilledMessage;
    use crate::infrastructure::bevy::header::HeaderView;
    use crate::infrastructure::bevy::score::{ScoreResource, ScoreViewLabel, ScoreViewValue};
    use bevy::app::{App, Startup, Update};
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::prelude::{IntoScheduleConfigs, Text, With};
    use bevy::text::Font;
    use bevy::MinimalPlugins;
    use std::error::Error;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));

        app.add_systems(
            Startup,
            (HeaderView::spawn_header, ScoreViewValue::spawn_score).chain(),
        );

        app.add_systems(Update, (ScoreViewValue::on_change,).chain());

        app.insert_resource(ScoreResource(Score::new()));

        app.init_asset::<Image>();
        app.init_asset::<Font>();

        app.update();

        app
    }

    #[test]
    fn should_display_the_score() -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        let mut score_view_label_query = app
            .world_mut()
            .query_filtered::<&Text, With<ScoreViewLabel>>();

        let mut score_view_value_query = app
            .world_mut()
            .query_filtered::<&Text, With<ScoreViewValue>>();

        let score_label = score_view_label_query.single(app.world())?;
        let score_value = score_view_value_query.single(app.world())?;

        assert_eq!(score_label.0, String::from("Score: "));
        assert_eq!(score_value.0, String::from("0"));

        Ok(())
    }

    #[test]
    fn should_update_score() -> Result<(), Box<dyn Error>> {
        let mut app = setup();

        let mut score_resource = app.world_mut().resource_mut::<ScoreResource>();
        score_resource.0.increment(50);

        app.update();

        let mut score_view_value_query = app
            .world_mut()
            .query_filtered::<&Text, With<ScoreViewValue>>();

        let score_value = score_view_value_query.single(app.world())?;

        assert_eq!(
            score_value.0,
            String::from("50"),
            "Score should have increased"
        );

        Ok(())
    }

    #[test]
    fn should_sync_domain() {
        let mut app = setup();
        app.add_message::<EnemyKilledMessage>();
        app.add_systems(Update, ScoreViewValue::on_enemy_killed_message);

        let dummy_entity = app.world_mut().spawn_empty().id();

        app.world_mut()
            .write_message(EnemyKilledMessage(dummy_entity, 1));

        app.update();

        let post_update_score_resource = app
            .world_mut()
            .get_resource::<ScoreResource>()
            .unwrap_or_else(|| panic!("ScoreResource missing"));

        assert_eq!(post_update_score_resource.0.get_current(), 10);
    }
}
