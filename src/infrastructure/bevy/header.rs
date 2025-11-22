use crate::infrastructure::bevy::screen::ScreenView;
use bevy::prelude::UiRect;
use bevy::{
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val},
    utils::default,
};

pub const HEADER_HEIGHT: f32 = 40.0;
#[derive(Component)]
pub struct HeaderView;

impl HeaderView {
    pub fn spawn_header(mut commands: Commands, root_query: Query<Entity, With<ScreenView>>) {
        if let Ok(root) = root_query.single() {
            commands.entity(root).with_children(|parent| {
                parent.spawn((
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
                    BackgroundColor(Color::srgb_u8(140, 12, 12)),
                ));
            });
        }
    }
}
