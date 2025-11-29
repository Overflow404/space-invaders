use crate::infrastructure::bevy::game_area::components::{GameAreaBundle, GameAreaComponent};
use bevy::asset::AssetServer;
use bevy::camera::{Camera2d, Projection};
use bevy::math::Vec2;
use bevy::prelude::{Changed, Commands, Query, Res, Sprite, Window, With};

pub fn spawn_game_area_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, Changed<Window>>,
) {
    if let Ok(window) = window_query.single() {
        commands.spawn(GameAreaBundle::new(
            &asset_server,
            window.width(),
            window.height(),
        ));
    }
}

pub fn resize_game_area_system(
    camera_query: Query<&Projection, (With<Camera2d>, Changed<Projection>)>,
    mut background_query: Query<&mut Sprite, With<GameAreaComponent>>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::game_area::systems::{
        resize_game_area_system, spawn_game_area_system,
    };
    use bevy::app::App;
    use bevy::asset::{AssetApp, AssetPlugin, Handle};
    use bevy::camera::Projection;
    use bevy::image::Image;
    use bevy::math::Vec2;
    use bevy::prelude::{Camera2d, OrthographicProjection, Sprite, Transform};
    use bevy::utils::default;
    use bevy::window::WindowResolution;
    use bevy::MinimalPlugins;
    use bevy_test::{contains_component, count_components, get_component, run_system};

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Image>()
            .world_mut()
            .spawn(Window {
                resolution: WindowResolution::new(800, 600),
                ..default()
            });
        app
    }

    #[test]
    fn should_spawn_game_area() {
        let mut app = setup();

        run_system(&mut app, spawn_game_area_system).expect("System should run");

        assert!(contains_component::<GameAreaComponent>(&mut app));
        assert_eq!(count_components::<GameAreaComponent>(&mut app), 1);
    }

    #[test]
    fn should_spawn_background_image_at_correct_z_index() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut app = setup();

        run_system(&mut app, spawn_game_area_system)?;

        let mut query = app
            .world_mut()
            .query::<(bevy::prelude::Entity, &GameAreaComponent)>();
        let (entity, _) = query.single(app.world())?;

        let transform = get_component::<Transform>(&mut app, entity);
        assert_eq!(transform.translation.z, -1.0);
        assert_eq!(transform.translation.x, 0.0);
        assert_eq!(transform.translation.y, 0.0);

        let sprite = get_component::<Sprite>(&mut app, entity);
        assert!(matches!(sprite.image, Handle::Strong(_)));
        assert_eq!(sprite.custom_size, Some(Vec2::new(800.0, 600.0)));

        Ok(())
    }

    #[test]
    fn should_resize_game_area_when_camera_projection_changes()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        run_system(&mut app, spawn_game_area_system)?;

        let new_width = 1024.0;
        let new_height = 768.0;

        app.world_mut().spawn((
            Camera2d,
            Projection::Orthographic(OrthographicProjection {
                area: bevy::math::Rect::new(0.0, 0.0, new_width, new_height),
                ..OrthographicProjection::default_2d()
            }),
        ));

        run_system(&mut app, resize_game_area_system)?;

        let mut query = app
            .world_mut()
            .query::<(bevy::prelude::Entity, &GameAreaComponent)>();
        let (entity, _) = query.single(app.world())?;

        let sprite = get_component::<Sprite>(&mut app, entity);
        assert_eq!(sprite.custom_size, Some(Vec2::new(new_width, new_height)));

        Ok(())
    }
}
