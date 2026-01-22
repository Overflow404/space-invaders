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

    #[test]
    fn plugin_loads_successfully() {
        let _app = bevy_test::smoke_test_plugin_with_assets(ShieldFormationPlugin);
    }
}
