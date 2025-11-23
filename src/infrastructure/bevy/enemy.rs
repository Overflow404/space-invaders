use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{default, Component, Transform};
use bevy::sprite::Sprite;

pub const ENEMY_WIDTH: f32 = 60.0;
pub const ENEMY_HEIGHT: f32 = 40.0;

const ENEMY_IMAGE: &str = "red.png";
#[derive(Component)]
pub struct EnemyView {
    pub id: usize,
}

impl EnemyView {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn spawn_enemy(
        id: usize,
        x: f32,
        y: f32,
        asset_server: &AssetServer,
    ) -> (Self, Sprite, Transform) {
        (
            EnemyView::new(id),
            Sprite {
                image: asset_server.load(ENEMY_IMAGE),
                custom_size: Some(Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT)),
                color: Color::srgb(255.0, 255.0, 255.0),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        )
    }
}
