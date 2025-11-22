use bevy::asset::AssetServer;
use bevy::camera::{Camera2d, Projection};
use bevy::math::Vec2;
use bevy::prelude::{Changed, Query, Res, Sprite, Transform, Window, With};
use bevy::{
    ecs::{component::Component, resource::Resource, system::Commands},
    utils::default,
};

pub const GAME_AREA_WIDTH: f32 = 1120.0;
pub const GAME_AREA_HEIGHT: f32 = 690.0;
const BACKGROUND_IMAGE: &str = "tv.png";
#[derive(Resource)]
pub struct GameAreaResource;

#[derive(Component)]
pub struct GameAreaView;

impl GameAreaView {
    pub fn spawn_game_area(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        window_query: Query<&Window, Changed<Window>>,
    ) {
        let window = window_query.single().unwrap();

        commands.spawn((
            GameAreaView,
            Sprite {
                image: asset_server.load(BACKGROUND_IMAGE),
                custom_size: Some(Vec2::new(window.width(), window.height())),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -1.0),
        ));
    }

    pub fn resize_game_area(
        camera_query: Query<&Projection, (With<Camera2d>, Changed<Projection>)>,
        mut background_query: Query<&mut Sprite, With<GameAreaView>>,
    ) {
        if let Ok(projection) = camera_query.single()
            && let Projection::Orthographic(orthographic_projection) = projection
        {
            let width = orthographic_projection.area.width();
            let height = orthographic_projection.area.height();

            for mut sprite in background_query.iter_mut() {
                sprite.custom_size = Some(Vec2::new(width, height));
            }
        }
    }
}
