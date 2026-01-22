use crate::infrastructure::bevy::score::resources::{
    SCORE_CONTAINER_HEIGHT, SCORE_CONTAINER_WIDTH, SCORE_LABEL_FONT_COLOR, SCORE_LABEL_FONT_SIZE,
    SCORE_LABEL_HEIGHT, SCORE_LABEL_MARGIN_RIGHT, SCORE_LABEL_TEXT, SCORE_VALUE_FONT_COLOR,
    SCORE_VALUE_FONT_SIZE, SCORE_VALUE_HEIGHT,
};
use bevy::asset::Handle;
use bevy::prelude::{default, Bundle, Component};
use bevy::text::{Font, TextColor, TextFont};
use bevy::ui::widget::Text;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, Node, UiRect};

#[derive(Component, PartialEq, Debug)]
pub struct ScoreValueComponent;

#[derive(Component, PartialEq, Debug)]
pub struct ScoreLabelComponent;

#[derive(Bundle)]
pub struct ScoreContainerBundle {
    pub node: Node,
}

impl ScoreContainerBundle {
    pub fn new() -> Self {
        Self {
            node: Node {
                width: SCORE_CONTAINER_WIDTH,
                height: SCORE_CONTAINER_HEIGHT,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }
}

impl Default for ScoreContainerBundle {
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
                height: SCORE_LABEL_HEIGHT,
                margin: UiRect::right(SCORE_LABEL_MARGIN_RIGHT),
                ..default()
            },
            text: Text::new(SCORE_LABEL_TEXT),
            text_font: TextFont {
                font,
                font_size: SCORE_LABEL_FONT_SIZE,
                ..default()
            },
            text_color: TextColor(SCORE_LABEL_FONT_COLOR),
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
                height: SCORE_VALUE_HEIGHT,
                ..default()
            },
            text: Text::new(score.to_string()),
            text_font: TextFont {
                font,
                font_size: SCORE_VALUE_FONT_SIZE,
                ..default()
            },
            text_color: TextColor(SCORE_VALUE_FONT_COLOR),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::color::Color;
    use bevy::ui::Val;
    use bevy_test::{dummy_font, TestAppBuilder};

    #[test]
    fn should_create_score_score_container_bundle() {
        let bundle = ScoreContainerBundle::new();

        assert_eq!(bundle.node.width, Val::Percent(50.0));
        assert_eq!(bundle.node.height, Val::Px(50.0));
        assert_eq!(bundle.node.flex_direction, FlexDirection::Row);
        assert_eq!(bundle.node.justify_content, JustifyContent::Center);
        assert_eq!(bundle.node.align_items, AlignItems::Center);
    }

    #[test]
    fn should_create_score_label_bundle() {
        let mut app = TestAppBuilder::new().build();
        app.add_plugins(AssetPlugin::default()).init_asset::<Font>();

        let font = dummy_font(&app);

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
        let mut app = TestAppBuilder::new().build();
        app.add_plugins(AssetPlugin::default()).init_asset::<Font>();

        let font = dummy_font(&app);
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
