use bevy::asset::Handle;
use bevy::color::Color;
use bevy::prelude::{default, Bundle, Component};
use bevy::text::{Font, TextColor, TextFont};
use bevy::ui::widget::Text;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, Node, UiRect, Val};

#[derive(Component, PartialEq, Debug)]
pub struct LivesComponent;

#[derive(Bundle)]
pub struct LivesViewBundle {
    pub lives: LivesComponent,
    pub node: Node,
}

impl LivesViewBundle {
    pub fn new() -> Self {
        Self {
            lives: LivesComponent,
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

impl Default for LivesViewBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Bundle)]
pub struct LivesLabelBundle {
    pub node: Node,
    pub text: Text,
    pub text_font: TextFont,
    pub text_color: TextColor,
}

impl LivesLabelBundle {
    pub fn new(font: Handle<Font>) -> Self {
        Self {
            node: Node {
                height: Val::Percent(50.0),
                margin: UiRect::right(Val::Px(20.0)),
                ..default()
            },
            text: Text::new("LIVES"),
            text_font: TextFont {
                font,
                font_size: 14.0,
                ..default()
            },
            text_color: TextColor(Color::WHITE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy_test::{dummy_font, minimal_app};

    #[test]
    fn should_create_lives_view_bundle() {
        let bundle = LivesViewBundle::new();

        assert_eq!(bundle.lives, LivesComponent);

        assert_eq!(bundle.node.width, Val::Percent(50.0));
        assert_eq!(bundle.node.height, Val::Px(50.0));
        assert_eq!(bundle.node.flex_direction, FlexDirection::Row);
        assert_eq!(bundle.node.justify_content, JustifyContent::Center);
        assert_eq!(bundle.node.align_items, AlignItems::Center);
    }

    #[test]
    fn should_create_lives_label_bundle() {
        let mut app = minimal_app(false);
        app.add_plugins(AssetPlugin::default()).init_asset::<Font>();

        let font = dummy_font(&app);

        let bundle = LivesLabelBundle::new(font.clone());

        assert_eq!(bundle.node.height, Val::Percent(50.0));
        assert_eq!(bundle.node.margin, UiRect::right(Val::Px(20.0)));

        assert_eq!(bundle.text.0, "LIVES");
        assert_eq!(bundle.text_font.font, font);
        assert_eq!(bundle.text_font.font_size, 14.0);
        assert_eq!(bundle.text_color.0, Color::WHITE);
    }
}
