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
    use bevy_test::{contains_system_or_fail, minimal_app};

    #[test]
    fn should_initialize_the_footer_plugin() {
        let mut app = minimal_app(false);
        app.add_plugins(FooterPlugin);

        app.update();

        assert!(contains_system_or_fail(
            &app,
            Startup,
            "spawn_footer_system"
        ));
    }
}
