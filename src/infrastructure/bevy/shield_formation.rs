use bevy::{
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val, ZIndex},
    utils::default,
};

use crate::infrastructure::bevy::game_area::GameAreaView;

#[derive(Component)]
pub struct ShieldFormationView;

impl ShieldFormationView {
    pub fn spawn_shields(
        mut commands: Commands,
        game_area_query: Query<Entity, With<GameAreaView>>,
    ) {
        if let Ok(game_area) = game_area_query.single() {
            commands.entity(game_area).with_children(|parent| {
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(10.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb_u8(60, 60, 60)),
                    ZIndex(-1),
                ));
            });
        }
    }
}
