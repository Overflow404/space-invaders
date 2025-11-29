use crate::infrastructure::bevy::header::components::{HeaderBundle, HeaderComponent};
use bevy::prelude::Commands;

pub fn spawn_header_system(mut commands: Commands) {
    commands.spawn(HeaderBundle::new());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::header::resources::HEADER_HEIGHT;
    use bevy::app::App;
    use bevy::color::Color;
    use bevy::prelude::UiRect;
    use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val};
    use bevy::MinimalPlugins;
    use bevy_test::{contains_component, count_components, get_component, run_system};

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app
    }

    #[test]
    fn should_spawn_header() {
        let mut app = setup();

        run_system(&mut app, spawn_header_system).expect("System should run");

        assert!(contains_component::<HeaderComponent>(&mut app));
        assert_eq!(count_components::<HeaderComponent>(&mut app), 1);
    }

    #[test]
    fn should_display_the_header_with_correct_properties() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut app = setup();

        run_system(&mut app, spawn_header_system)?;

        let mut query = app
            .world_mut()
            .query::<(bevy::prelude::Entity, &HeaderComponent)>();
        let (entity, _) = query.single(app.world())?;

        let node = get_component::<Node>(&mut app, entity);
        assert_eq!(node.width, Val::Percent(100.0));
        assert_eq!(node.height, Val::Px(HEADER_HEIGHT));
        assert_eq!(node.flex_direction, FlexDirection::Row);
        assert_eq!(node.justify_content, JustifyContent::SpaceBetween);
        assert_eq!(node.align_items, AlignItems::Center);
        assert_eq!(node.padding, UiRect::horizontal(Val::Px(20.0)));

        let background_color = get_component::<BackgroundColor>(&mut app, entity);
        assert_eq!(background_color.0, Color::srgb_u8(0, 0, 0));

        Ok(())
    }
}
