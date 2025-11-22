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
