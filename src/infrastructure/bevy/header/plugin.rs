use crate::infrastructure::bevy::header::systems::spawn_header_system;
use bevy::app::{App, Plugin, Startup};

pub struct HeaderPlugin;

impl Plugin for HeaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_header_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_loads_successfully() {
        let _app = bevy_test::smoke_test_plugin(HeaderPlugin);
    }
}
