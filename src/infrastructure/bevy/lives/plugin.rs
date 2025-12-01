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
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::prelude::AssetApp;
    use bevy::text::Font;
    use bevy_test::{contains_system_or_fail, get_resource_or_fail, minimal_app};

    fn setup() -> App {
        let mut app = minimal_app(false);
        app.add_plugins(AssetPlugin::default())
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

        get_resource_or_fail::<LivesResource>(&mut app);

        assert!(contains_system_or_fail(&app, Startup, "spawn_header_system"));
        assert!(contains_system_or_fail(&app, Startup, "spawn_lives_system"));
    }
}
