use crate::infrastructure::bevy::footer::components::FooterBundle;
use bevy::prelude::Commands;

pub fn spawn_footer_system(mut commands: Commands) {
    commands.spawn(FooterBundle::new());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::footer::components::FooterComponent;
    use bevy::app::{App, Startup};
    use bevy_test::{TestAppBuilder, contains_single_component};

    fn setup() -> App {
        TestAppBuilder::new().build()
    }

    #[cfg(test)]
    mod spawn_footer_system {
        use super::*;

        #[test]
        fn should_spawn_footer() {
            let mut app = setup();
            app.add_systems(Startup, spawn_footer_system);
            app.update();

            assert!(contains_single_component::<FooterComponent>(&mut app));
        }
    }
}
