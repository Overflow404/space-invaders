use crate::infrastructure::bevy::game_area::components::{GameAreaBundle, GameAreaComponent};
use bevy::asset::AssetServer;
use bevy::camera::{Camera2d, Projection};
use bevy::math::Vec2;
use bevy::prelude::{Changed, Commands, Query, Res, Sprite, Window, With};

pub fn spawn_game_area_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
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
    if let Ok(projection) = camera_query.single() {
        if let Projection::Orthographic(orthographic_projection) = projection {
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
    use bevy::app::App;
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::utils::default;
    use bevy::window::WindowResolution;
    use bevy_test::{get_component_or_fail, minimal_app};

    fn setup() -> App {
        let mut app = minimal_app(false);
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>()
            .world_mut()
            .spawn(Window {
                resolution: WindowResolution::new(800, 600),
                ..default()
            });
        app
    }

    #[cfg(test)]
    mod spawn_game_area_system {
        use super::*;
        use bevy::app::Startup;
        use bevy_test::contains_single_component;

        #[test]
        fn should_spawn_game_area() {
            let mut app = setup();
            app.add_systems(Startup, spawn_game_area_system);
            app.update();

            assert!(contains_single_component::<GameAreaComponent>(&mut app));
        }
    }

    #[cfg(test)]
    mod resize_game_area_system {
        use super::*;
        use bevy::app::{Startup, Update};
        use bevy::camera::{Camera2d, OrthographicProjection, Projection};

        #[test]
        fn should_resize_game_area_when_camera_projection_changes() {
            let mut app = setup();
            app.add_systems(Startup, spawn_game_area_system);
            app.add_systems(Update, resize_game_area_system);
            app.update();

            let new_width = 1024.0;
            let new_height = 768.0;

            app.world_mut().spawn((
                Camera2d,
                Projection::Orthographic(OrthographicProjection {
                    area: bevy::math::Rect::new(0.0, 0.0, new_width, new_height),
                    ..OrthographicProjection::default_2d()
                }),
            ));

            app.update();

            let mut query = app
                .world_mut()
                .query::<(bevy::prelude::Entity, &GameAreaComponent)>();
            let (entity, _) = query
                .single(app.world())
                .expect("GameAreaComponent not found");

            let sprite = get_component_or_fail::<Sprite>(&mut app, entity);
            assert_eq!(sprite.custom_size, Some(Vec2::new(new_width, new_height)));
        }
    }
}
