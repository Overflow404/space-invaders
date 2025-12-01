use crate::infrastructure::bevy::header::components::HeaderBundle;
use bevy::prelude::Commands;

pub fn spawn_header_system(mut commands: Commands) {
    commands.spawn(HeaderBundle::new());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::header::resources::HEADER_HEIGHT;
    use bevy::app::{App, Startup};
    use bevy::color::Color;
    use bevy::prelude::UiRect;
    use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val};
    use bevy_test::{contains_component, get_component, minimal_app};

    fn setup() -> App {
        minimal_app()
    }

    #[cfg(test)]
    mod spawn_header_system {
        use super::*;
        use crate::infrastructure::bevy::header::components::HeaderComponent;

        #[test]
        fn should_spawn_header() {
            let mut app = setup();
            app.add_systems(Startup, spawn_header_system);
            app.update();

            assert!(contains_component::<HeaderComponent>(&mut app));
        }

        #[test]
        fn should_display_the_header_with_correct_properties() {
            let mut app = setup();
            app.add_systems(Startup, spawn_header_system);
            app.update();

            let entity = app
                .world_mut()
                .query::<(bevy::prelude::Entity, &HeaderComponent)>()
                .single(app.world())
                .expect("Header not found")
                .0;

            let node = get_component::<Node>(&mut app, entity);
            assert_eq!(node.width, Val::Percent(100.0));
            assert_eq!(node.height, Val::Px(HEADER_HEIGHT));
            assert_eq!(node.flex_direction, FlexDirection::Row);
            assert_eq!(node.justify_content, JustifyContent::SpaceBetween);
            assert_eq!(node.align_items, AlignItems::Center);
            assert_eq!(node.padding, UiRect::horizontal(Val::Px(20.0)));

            let background_color = get_component::<BackgroundColor>(&mut app, entity);
            assert_eq!(background_color.0, Color::srgb_u8(0, 0, 0));
        }
    }
}
