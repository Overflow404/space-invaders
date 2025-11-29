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
    use crate::infrastructure::bevy::header::components::HeaderComponent;
    use bevy::MinimalPlugins;
    use bevy_test::contains_component;

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, HeaderPlugin));

        app.update();

        assert!(contains_component::<HeaderComponent>(&mut app));
    }
}
