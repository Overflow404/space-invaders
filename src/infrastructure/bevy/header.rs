use bevy::prelude::UiRect;
use bevy::{
    color::Color,
    ecs::{component::Component, system::Commands},
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val},
    utils::default,
};

pub const HEADER_HEIGHT: f32 = 40.0;
pub const FONT: &str = "pixeled.ttf";
#[derive(Component)]
pub struct HeaderView;

impl HeaderView {
    pub fn spawn_header(mut commands: Commands) {
        commands.spawn((
            Self,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(HEADER_HEIGHT),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(0, 0, 0)),
        ));
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::{App, Startup};
    use bevy::MinimalPlugins;
    use bevy::ui::{BackgroundColor, Node};
    use crate::infrastructure::bevy::header::HeaderView;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, HeaderView::spawn_header);
        app.update();
        app
    }

    #[test]
    fn should_display_the_header() {
        let mut app = setup();

        let mut query = app
            .world_mut()
            .query::<(&HeaderView, &Node, &BackgroundColor)>();

        let query_result = query.single(app.world());

        assert!(query_result.is_ok());
    }
}
