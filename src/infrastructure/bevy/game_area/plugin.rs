use crate::infrastructure::bevy::game_area::systems::{
    resize_game_area_system, spawn_game_area_system,
};
use bevy::app::{App, Plugin, PostUpdate, Startup};

pub struct GameAreaPlugin;

impl Plugin for GameAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_area_system)
            .add_systems(PostUpdate, resize_game_area_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::game_area::components::GameAreaComponent;
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::prelude::{AssetApp, Window};
    use bevy::utils::default;
    use bevy::window::WindowResolution;
    use bevy_test::{contains_component, contains_system, minimal_app};

    fn setup() -> App {
        let mut app = minimal_app();
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>();

        app.world_mut().spawn(Window {
            resolution: WindowResolution::new(800, 600),
            ..default()
        });

        app.add_plugins(GameAreaPlugin);

        app.update();
        app
    }

    #[test]
    fn should_initialize_the_game_area_plugin() {
        let mut app = setup();
        assert!(contains_component::<GameAreaComponent>(&mut app));
        assert!(contains_system(&app, PostUpdate, "resize_game_area_system"));
    }
}
