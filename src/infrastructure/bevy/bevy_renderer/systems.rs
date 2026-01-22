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
    use bevy::app::Startup;

    #[cfg(test)]
    mod camera_system {
        use super::*;
        use crate::infrastructure::bevy::bevy_renderer::components::CameraComponent;
        use bevy_test::{contains_single_component, TestAppBuilder};

        #[test]
        fn should_spawn_camera() {
            let mut app = TestAppBuilder::new()
                .without_auto_update()
                .with_setup(|app| {
                    app.add_systems(Startup, camera_system);
                    app.update();
                })
                .build();

            assert!(contains_single_component::<CameraComponent>(&mut app));
        }
    }

    #[cfg(test)]
    mod window_scale_system {
        use super::*;
        use bevy::prelude::{default, UiScale, Window};
        use bevy::window::WindowResolution;
        use bevy_test::TestAppBuilder;

        #[test]
        fn should_scale_up() {
            let app = TestAppBuilder::new()
                .without_auto_update()
                .with_setup(|app| {
                    app.init_resource::<UiScale>();
                    app.add_systems(Startup, window_scale_system);
                    app.world_mut().spawn(Window {
                        resolution: WindowResolution::new(
                            WINDOW_WIDTH as u32 * 2u32,
                            WINDOW_HEIGHT as u32 * 2u32,
                        ),
                        ..default()
                    });
                    app.update();
                })
                .build();

            let ui_scale = app.world().resource::<UiScale>();
            assert_eq!(ui_scale.0, 2.0);
        }

        #[test]
        fn should_use_minimum_scale() {
            let app = TestAppBuilder::new()
                .without_auto_update()
                .with_setup(|app| {
                    app.init_resource::<UiScale>();
                    app.add_systems(Startup, window_scale_system);
                    app.world_mut().spawn(Window {
                        resolution: WindowResolution::new(2400, 1050),
                        ..default()
                    });
                    app.update();
                })
                .build();

            let ui_scale = app.world().resource::<UiScale>();
            assert_eq!(ui_scale.0, 1.5);
        }

        #[test]
        fn should_scale_down() {
            let app = TestAppBuilder::new()
                .without_auto_update()
                .with_setup(|app| {
                    app.init_resource::<UiScale>();
                    app.add_systems(Startup, window_scale_system);
                    app.world_mut().spawn(Window {
                        resolution: WindowResolution::new(600, 350),
                        ..default()
                    });
                    app.update();
                })
                .build();

            let ui_scale = app.world().resource::<UiScale>();
            assert_eq!(ui_scale.0, 0.5);
        }
    }
}
