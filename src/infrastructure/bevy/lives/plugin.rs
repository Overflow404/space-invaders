use crate::domain::lives::Lives;
use crate::infrastructure::bevy::header::systems::spawn_header_system;
use crate::infrastructure::bevy::lives::resources::LivesResource;
use crate::infrastructure::bevy::lives::systems::spawn_lives_system;
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::IntoScheduleConfigs;

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LivesResource(Lives::new()))
            .add_systems(Startup, spawn_lives_system.after(spawn_header_system));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::header::plugin::HeaderPlugin;
    use crate::infrastructure::bevy::lives::components::LivesComponent;
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::prelude::AssetApp;
    use bevy::text::Font;
    use bevy::MinimalPlugins;
    use bevy_test::{count_components, get_resource_or_fail};

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .add_plugins(HeaderPlugin)
            .add_plugins(LivesPlugin)
            .init_asset::<Image>()
            .init_asset::<Font>();

        app.update();
        app
    }

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = setup();

        let lives_resource = get_resource_or_fail::<LivesResource>(&mut app);
        assert_eq!(lives_resource.0.get_current(), 3);

        assert!(count_components::<LivesComponent>(&mut app) <= 1);
    }
}
