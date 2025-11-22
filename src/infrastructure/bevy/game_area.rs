use crate::infrastructure::bevy::screen::ScreenView;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Sprite, Transform};
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

pub const GAME_AREA_WIDTH: f32 = 1120.0;
pub const GAME_AREA_HEIGHT: f32 = 690.0;
#[derive(Resource)]
pub struct GameAreaResource;

#[derive(Component)]
pub struct GameAreaView;

impl GameAreaView {
    pub fn spawn_game_area(mut commands: Commands) {
        commands.spawn((
            GameAreaView,
            Sprite {
                color: Color::srgb_u8(20, 30, 20),
                custom_size: Some(Vec2::new(GAME_AREA_WIDTH, GAME_AREA_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -1.0),
        ));
    }
}
