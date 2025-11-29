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
    use crate::infrastructure::bevy::footer::components::FooterComponent;
    use bevy::MinimalPlugins;
    use bevy_test::contains_component;

    #[test]
    fn should_initialize_the_footer_plugin() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, FooterPlugin));

        app.update();

        assert!(contains_component::<FooterComponent>(&mut app));
    }
}
