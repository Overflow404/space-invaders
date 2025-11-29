use crate::infrastructure::bevy::bevy_renderer::components::CameraBundle;
use crate::infrastructure::bevy::bevy_renderer::resources::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::{Changed, Commands, Query, ResMut, UiScale, Window};

pub fn camera_system(mut commands: Commands) {
    commands.spawn(CameraBundle::new());
}

pub fn window_scale_system(
    window_query: Query<&Window, Changed<Window>>,
    mut ui_scale: ResMut<UiScale>,
) {
    if let Ok(window) = window_query.single() {
        let scale_x = window.width() / WINDOW_WIDTH;
        let scale_y = window.height() / WINDOW_HEIGHT;
        ui_scale.0 = scale_x.min(scale_y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, camera_system)
            .add_systems(Update, window_scale_system)
            .init_resource::<UiScale>();

        app.update();
        app
    }

    #[cfg(test)]
    mod camera_system {
        use crate::infrastructure::bevy::bevy_renderer::components::CameraComponent;
        use crate::infrastructure::bevy::bevy_renderer::systems::tests::setup;
        use bevy_test::contains_component;

        #[test]
        fn camera_system_should_spawn_camera() {
            let mut app = setup();

            assert!(contains_component::<CameraComponent>(&mut app));
        }
    }

    #[cfg(test)]
    mod window_scale_system {
        use crate::infrastructure::bevy::bevy_renderer::resources::{WINDOW_HEIGHT, WINDOW_WIDTH};
        use crate::infrastructure::bevy::bevy_renderer::systems::tests::setup;
        use bevy::prelude::{default, UiScale, Window};
        use bevy::window::WindowResolution;

        #[test]
        fn window_scale_system_should_scale_uniformly() {
            let mut app = setup();

            app.world_mut().spawn(Window {
                resolution: WindowResolution::new(
                    WINDOW_WIDTH as u32 * 2u32,
                    WINDOW_HEIGHT as u32 * 2u32,
                ),
                ..default()
            });

            app.update();

            let ui_scale = app.world().resource::<UiScale>();
            assert_eq!(ui_scale.0, 2.0,);
        }

        #[test]
        fn window_scale_system_should_use_minimum_scale() {
            let mut app = setup();

            app.world_mut().spawn(Window {
                resolution: WindowResolution::new(2400, 1050),
                ..default()
            });

            app.update();

            let ui_scale = app.world().resource::<UiScale>();
            assert_eq!(ui_scale.0, 1.5);
        }

        #[test]
        fn window_scale_system_should_scale_down() {
            let mut app = setup();

            app.world_mut().spawn(Window {
                resolution: WindowResolution::new(600, 350),
                ..default()
            });

            app.update();

            let ui_scale = app.world().resource::<UiScale>();
            assert_eq!(ui_scale.0, 0.5);
        }
    }
}
