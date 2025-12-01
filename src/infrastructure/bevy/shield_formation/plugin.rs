use crate::domain::shield_formation::ShieldFormation;
use crate::infrastructure::bevy::shield_formation::resources::ShieldFormationResource;
use crate::infrastructure::bevy::shield_formation::systems::spawn_shields_system;
use bevy::app::{App, Plugin, Startup};

pub struct ShieldFormationPlugin;

impl Plugin for ShieldFormationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShieldFormationResource(ShieldFormation::new()))
            .add_systems(Startup, spawn_shields_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::prelude::AssetApp;
    use bevy_test::{contains_system, get_resource_or_fail, minimal_app};

    fn setup() -> App {
        let mut app = minimal_app();
        app.add_plugins(AssetPlugin::default())
            .add_plugins(ShieldFormationPlugin)
            .init_asset::<Image>();

        app.update();
        app
    }

    #[test]
    fn should_initialize_the_shield_formation_plugin() {
        let mut app = setup();

        get_resource_or_fail::<ShieldFormationResource>(&mut app);
        assert!(contains_system(&app, Startup, "spawn_shields_system"));
    }
}
