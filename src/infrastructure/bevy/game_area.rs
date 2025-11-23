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
        if let Ok(window) = window_query.single() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;
    use bevy::prelude::*;
    use bevy::window::WindowResolution;

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.init_asset::<Image>();

        app.world_mut().spawn(Window {
            resolution: WindowResolution::new(800, 600),
            ..default()
        });

        app.add_systems(Startup, GameAreaView::spawn_game_area);
        app.update();
        app
    }

    #[test]
    fn should_spawn_background_image_at_correct_z_index() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut app = setup_app();

        let mut query = app
            .world_mut()
            .query::<(&GameAreaView, &Sprite, &Transform)>();

        let (_, sprite, transform) = query.single(app.world())?;

        assert_eq!(transform.translation.z, -1.0);
        assert!(matches!(sprite.image, Handle::Strong(_)));

        Ok(())
    }
}
