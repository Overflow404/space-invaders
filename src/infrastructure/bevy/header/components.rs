use crate::infrastructure::bevy::header::resources::HEADER_HEIGHT;
use bevy::color::Color;
use bevy::prelude::{default, Bundle, Component, UiRect};
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val};

#[derive(Component, PartialEq, Debug)]
pub struct HeaderComponent;

#[derive(Bundle)]
pub struct HeaderBundle {
    pub header: HeaderComponent,
    pub node: Node,
    pub background_color: BackgroundColor,
}

impl HeaderBundle {
    pub fn new() -> Self {
        Self {
            header: HeaderComponent,
            node: Node {
                width: Val::Percent(100.0),
                height: Val::Px(HEADER_HEIGHT),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgb_u8(0, 0, 0)),
        }
    }
}

impl Default for HeaderBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_the_bundle() {
        let bundle = HeaderBundle::new();

        assert_eq!(bundle.header, HeaderComponent);

        assert_eq!(bundle.node.width, Val::Percent(100.0));
        assert_eq!(bundle.node.height, Val::Px(HEADER_HEIGHT));
        assert_eq!(bundle.node.flex_direction, FlexDirection::Row);
        assert_eq!(bundle.node.justify_content, JustifyContent::SpaceBetween);
        assert_eq!(bundle.node.align_items, AlignItems::Center);
        assert_eq!(bundle.node.padding, UiRect::horizontal(Val::Px(20.0)));

        assert_eq!(bundle.background_color.0, Color::srgb_u8(0, 0, 0));
    }
}
