use crate::infrastructure::bevy::header::components::HeaderComponent;
use crate::infrastructure::bevy::header::resources::FONT;
use crate::infrastructure::bevy::lives::components::{LivesLabelBundle, LivesViewBundle};
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
    use crate::infrastructure::bevy::header::components::HeaderComponent;
    use crate::infrastructure::bevy::lives::components::LivesComponent;
    use crate::infrastructure::bevy::lives::resources::LivesResource;
    use bevy::app::{App, Startup};
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::prelude::{AssetApp, Children, ImageNode, Text};
    use bevy::text::Font;
    use bevy_test::{contains_component, minimal_app};

    fn setup() -> App {
        let mut app = minimal_app(false);
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>()
            .init_asset::<Font>()
            .insert_resource(LivesResource(Lives::new()));

        app.world_mut().spawn(HeaderComponent);
        app
    }

    #[cfg(test)]
    mod spawn_lives_system {
        use super::*;

        #[test]
        fn should_spawn_lives_component() {
            let mut app = setup();
            app.add_systems(Startup, spawn_lives_system);
            app.update();

            assert!(contains_component::<LivesComponent>(&mut app));
        }

        #[test]
        fn should_display_the_lives_with_label_and_icons() {
            let mut app = setup();
            app.add_systems(Startup, spawn_lives_system);
            app.update();

            let mut query = app.world_mut().query::<(&LivesComponent, &Children)>();
            let (_, children) = query.single(app.world()).unwrap();

            let label_count = children
                .iter()
                .filter(|child| {
                    app.world()
                        .get::<Text>(**child)
                        .map_or(false, |text| text.0 == "LIVES")
                })
                .count();

            let icon_count = children
                .iter()
                .filter(|child| app.world().get::<ImageNode>(**child).is_some())
                .count();

            assert_eq!(label_count, 1);
            assert_eq!(icon_count, 3);
        }
    }
}
