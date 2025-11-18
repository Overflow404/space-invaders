use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        resource::Resource,
        system::{Commands, Query},
    },
    ui::{AlignItems, FlexDirection, JustifyContent, Node, Val},
    utils::default,
};

use crate::infrastructure::bevy::screen::ScreenView;

#[derive(Resource)]
pub struct GameAreaResource;

#[derive(Component)]
pub struct GameAreaView;

impl GameAreaView {
    pub fn spawn_game_area(mut commands: Commands, root_query: Query<Entity, With<ScreenView>>) {
        if let Ok(root) = root_query.single() {
            commands.entity(root).with_children(|parent| {
                parent.spawn((
                    Self,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(90.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ));
            });
        }
    }
}
