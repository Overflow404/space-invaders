use bevy::asset::Handle;
use bevy::color::Color;
use bevy::prelude::{default, Bundle, Component};
use bevy::text::{Font, TextColor, TextFont};
use bevy::ui::widget::Text;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, Node, UiRect, Val};

#[derive(Component, PartialEq, Debug)]
pub struct ScoreValueComponent;

#[derive(Component, PartialEq, Debug)]
pub struct ScoreLabelComponent;

#[derive(Bundle)]
pub struct ScoreViewBundle {
    pub node: Node,
}

impl ScoreViewBundle {
    pub fn new() -> Self {
        Self {
            node: Node {
                width: Val::Percent(50.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }
}

impl Default for ScoreViewBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Bundle)]
pub struct ScoreLabelBundle {
    pub score_label: ScoreLabelComponent,
    pub node: Node,
    pub text: Text,
    pub text_font: TextFont,
    pub text_color: TextColor,
}

impl ScoreLabelBundle {
    pub fn new(font: Handle<Font>) -> Self {
        Self {
            score_label: ScoreLabelComponent,
            node: Node {
                height: Val::Percent(50.0),
                margin: UiRect::right(Val::Px(20.0)),
                ..default()
            },
            text: Text::new("Score: "),
            text_font: TextFont {
                font,
                font_size: 14.0,
                ..default()
            },
            text_color: TextColor(Color::WHITE),
        }
    }
}

#[derive(Bundle)]
pub struct ScoreValueBundle {
    pub score_value: ScoreValueComponent,
    pub node: Node,
    pub text: Text,
    pub text_font: TextFont,
    pub text_color: TextColor,
}

impl ScoreValueBundle {
    pub fn new(font: Handle<Font>, score: u32) -> Self {
        Self {
            score_value: ScoreValueComponent,
            node: Node {
                height: Val::Percent(50.0),
                ..default()
            },
            text: Text::new(score.to_string()),
            text_font: TextFont {
                font,
                font_size: 14.0,
                ..default()
            },
            text_color: TextColor(Color::srgb_u8(51, 255, 3)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::asset::{AssetApp, AssetPlugin, AssetServer};
    use bevy::MinimalPlugins;

    #[test]
    fn should_create_score_view_bundle() {
        let bundle = ScoreViewBundle::new();

        assert_eq!(bundle.node.width, Val::Percent(50.0));
        assert_eq!(bundle.node.height, Val::Px(50.0));
        assert_eq!(bundle.node.flex_direction, FlexDirection::Row);
        assert_eq!(bundle.node.justify_content, JustifyContent::Center);
        assert_eq!(bundle.node.align_items, AlignItems::Center);
    }

    #[test]
    fn should_create_score_label_bundle() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Font>();

        let asset_server = app.world().resource::<AssetServer>().clone();
        let font = asset_server.load("test.ttf");

        let bundle = ScoreLabelBundle::new(font.clone());

        assert_eq!(bundle.score_label, ScoreLabelComponent);

        assert_eq!(bundle.node.height, Val::Percent(50.0));
        assert_eq!(bundle.node.margin, UiRect::right(Val::Px(20.0)));

        assert_eq!(bundle.text.0, "Score: ");
        assert_eq!(bundle.text_font.font, font);
        assert_eq!(bundle.text_font.font_size, 14.0);
        assert_eq!(bundle.text_color.0, Color::WHITE);
    }

    #[test]
    fn should_create_score_value_bundle() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Font>();

        let asset_server = app.world().resource::<AssetServer>().clone();
        let font = asset_server.load("test.ttf");
        let score = 42;

        let bundle = ScoreValueBundle::new(font.clone(), score);

        assert_eq!(bundle.score_value, ScoreValueComponent);

        assert_eq!(bundle.node.height, Val::Percent(50.0));

        assert_eq!(bundle.text.0, "42");
        assert_eq!(bundle.text_font.font, font);
        assert_eq!(bundle.text_font.font_size, 14.0);
        assert_eq!(bundle.text_color.0, Color::srgb_u8(51, 255, 3));
    }
}
