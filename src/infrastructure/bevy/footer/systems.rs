use crate::infrastructure::bevy::footer::components::FooterBundle;
use bevy::prelude::Commands;

pub fn spawn_footer_system(mut commands: Commands) {
    commands.spawn(FooterBundle::new());
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::bevy::footer::components::FooterComponent;
    use crate::infrastructure::bevy::footer::systems::spawn_footer_system;
    use bevy::app::{App, Startup};
    use bevy::MinimalPlugins;
    use bevy_test::contains_component;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_footer_system);

        app.update();
        app
    }

    #[test]
    fn should_display_the_footer() {
        let mut app = setup();

        assert!(contains_component::<FooterComponent>(&mut app));
    }
}
