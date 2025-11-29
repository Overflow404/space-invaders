use crate::infrastructure::bevy::header::components::HeaderComponent;
use crate::infrastructure::bevy::header::resources::FONT;
use crate::infrastructure::bevy::lives::components::{
    LivesComponent, LivesLabelBundle, LivesViewBundle,
};
use crate::infrastructure::bevy::lives::resources::LivesResource;
use crate::infrastructure::bevy::player::resources::PLAYER_IMAGE;
use bevy::asset::AssetServer;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::ui::widget::ImageNode;
use bevy::ui::{Node, UiRect, Val};
use bevy::utils::default;

pub fn spawn_lives_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lives_res: Res<LivesResource>,
    header_query: Query<Entity, With<HeaderComponent>>,
) {
    if let Ok(header) = header_query.single() {
        let font = asset_server.load(FONT);
        let remaining_lives = lives_res.0.get_current();

        commands.entity(header).with_children(|parent| {
            parent
                .spawn(LivesViewBundle::new())
                .with_children(|lives_section| {
                    lives_section.spawn(LivesLabelBundle::new(font));
                    for _ in 0..remaining_lives {
                        lives_section.spawn((
                            ImageNode {
                                image: asset_server.load(PLAYER_IMAGE),
                                ..default()
                            },
                            Node {
                                height: Val::Percent(35.0),
                                margin: UiRect::right(Val::Px(25.0)),
                                ..default()
                            },
                        ));
                    }
                });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::lives::Lives;
    use crate::infrastructure::bevy::header::systems::spawn_header_system;
    use crate::infrastructure::bevy::lives::resources::LivesResource;
    use bevy::app::App;
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::prelude::{Children, ImageNode};
    use bevy::text::Font;
    use bevy::ui::widget::Text;
    use bevy::MinimalPlugins;
    use bevy_test::{contains_component, count_components, run_system};

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .insert_resource(LivesResource(Lives::new()))
            .init_asset::<Image>()
            .init_asset::<Font>();
        app
    }

    #[test]
    fn should_spawn_lives_component() {
        let mut app = setup();

        run_system(&mut app, spawn_header_system).expect("System should run");
        run_system(&mut app, spawn_lives_system).expect("System should run");

        assert!(contains_component::<LivesComponent>(&mut app));
        assert_eq!(count_components::<LivesComponent>(&mut app), 1);
    }

    #[test]
    fn should_display_the_lives_with_label_and_icons() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        run_system(&mut app, spawn_header_system)?;
        run_system(&mut app, spawn_lives_system)?;

        let mut query = app.world_mut().query::<(&LivesComponent, &Children)>();
        let (_, children) = query.single(app.world())?;

        let label = children
            .iter()
            .filter(|child| {
                if let Some(text) = app.world().get::<Text>(**child)
                    && text.0 == "LIVES"
                {
                    return true;
                }
                false
            })
            .collect::<Vec<&Entity>>();

        let lives = children
            .iter()
            .filter(|child| app.world().get::<ImageNode>(**child).is_some())
            .collect::<Vec<&Entity>>();

        assert!(!label.is_empty());
        assert_eq!(lives.len(), 3);

        Ok(())
    }
}
