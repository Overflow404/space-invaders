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
    use bevy_test::{contains_system, minimal_app};

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = minimal_app(false);

        app.add_plugins(HeaderPlugin);

        app.update();

        assert!(contains_system(&app, Startup, "spawn_header_system"));
    }
}
