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
    use bevy_test::TestAppBuilder;

    #[test]
    fn spawning_header_creates_ui_node_with_correct_layout() {
        let mut app = TestAppBuilder::new().build();

        app.world_mut().spawn(HeaderBundle::new());

        let mut query = app.world_mut().query::<(&HeaderComponent, &Node, &BackgroundColor)>();
        let (header, node, bg_color) = query.single(app.world()).expect("Header not found");

        assert_eq!(*header, HeaderComponent);
        assert_eq!(node.width, Val::Percent(100.0));
        assert_eq!(node.height, Val::Px(HEADER_HEIGHT));
        assert_eq!(node.flex_direction, FlexDirection::Row);
        assert_eq!(bg_color.0, Color::srgb_u8(0, 0, 0));
    }
}
