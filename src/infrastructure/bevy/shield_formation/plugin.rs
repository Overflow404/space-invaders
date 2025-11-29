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
    use crate::infrastructure::bevy::shield::components::ShieldComponent;
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::prelude::AssetApp;
    use bevy::MinimalPlugins;
    use bevy_test::{count_components, get_resource};

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .add_plugins(ShieldFormationPlugin)
            .init_asset::<Image>();

        app.update();
        app
    }

    #[test]
    fn should_initialize_the_shield_formation_plugin() {
        let mut app = setup();

        let shield_formation_resource = get_resource::<ShieldFormationResource>(&mut app);

        assert_eq!(shield_formation_resource.0.get_shields().len(), 4);
        assert_eq!(count_components::<ShieldComponent>(&mut app), 4);
    }
}
