use crate::infrastructure::bevy::footer::systems::spawn_footer_system;
use bevy::app::{App, Plugin, Startup};

pub struct FooterPlugin;

impl Plugin for FooterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_footer_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_loads_successfully() {
        let _app = bevy_test::smoke_test_plugin(FooterPlugin);
    }
}
