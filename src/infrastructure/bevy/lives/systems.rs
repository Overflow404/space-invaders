use crate::infrastructure::bevy::enemy_projectile::components::PlayerKilledMessage;
use crate::infrastructure::bevy::header::components::HeaderComponent;
use crate::infrastructure::bevy::header::resources::FONT;
use crate::infrastructure::bevy::lives::components::{
    LivesLabelBundle, LivesValueBundle, LivesValueComponent, LivesViewBundle,
};
use crate::infrastructure::bevy::lives::resources::LivesResource;
use crate::infrastructure::bevy::player::resources::PLAYER_IMAGE;
use bevy::asset::AssetServer;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::prelude::{DetectChanges, MessageReader, ResMut};

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
                        let image = asset_server.load(PLAYER_IMAGE);
                        lives_section.spawn(LivesValueBundle::new(image));
                    }
                });
        });
    }
}

pub fn handle_player_killed_system(
    mut lives_res: ResMut<LivesResource>,
    mut player_killed_message: MessageReader<PlayerKilledMessage>,
) {
    for _ in player_killed_message.read() {
        lives_res.0.decrement();
    }
}

pub fn update_lives_system(
    lives_resource: Res<LivesResource>,
    mut commands: Commands,
    lives_icons_query: Query<(Entity, &LivesValueComponent)>,
) {
    if lives_resource.is_changed() {
        let target_lives_count = lives_resource.0.get_current() as usize;
        let current_icon_count = lives_icons_query.iter().count();

        if current_icon_count > target_lives_count {
            let diff = current_icon_count - target_lives_count;

            for (entity, _) in lives_icons_query.iter().take(diff) {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::lives::Lives;
    use crate::infrastructure::bevy::header::components::HeaderComponent;
    use crate::infrastructure::bevy::lives::components::LivesViewComponent;
    use crate::infrastructure::bevy::lives::resources::LivesResource;
    use bevy::app::{App, Startup};
    use bevy::image::Image;
    use bevy::prelude::{AssetApp, Children, ImageNode, Text};
    use bevy::text::Font;
    use bevy_test::{contains_single_component, TestAppBuilder};

    fn setup() -> App {
        TestAppBuilder::new()
            .with_assets()
            .with_setup(|app| {
                app.init_asset::<Image>()
                    .init_asset::<Font>()
                    .insert_resource(LivesResource(Lives::new()));

                app.world_mut().spawn(HeaderComponent);
            })
            .build()
    }

    #[cfg(test)]
    mod spawn_lives_system {
        use super::*;

        #[test]
        fn should_spawn_lives_component() {
            let mut app = setup();
            app.add_systems(Startup, spawn_lives_system);
            app.update();

            assert!(contains_single_component::<LivesViewComponent>(&mut app));
        }

        #[test]
        fn should_display_the_lives_with_label_and_icons() {
            let mut app = setup();
            app.add_systems(Startup, spawn_lives_system);
            app.update();

            let mut query = app.world_mut().query::<(&LivesViewComponent, &Children)>();
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

    #[cfg(test)]
    mod update_lives_system {
        use super::*;
        use crate::infrastructure::bevy::lives::components::LivesValueComponent;
        use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileComponent;
        use bevy::app::Update;
        use bevy_test::{get_resource_mut_or_fail, get_resource_or_fail, send_message};

        #[test]
        fn should_decrease_lives_when_player_killed() {
            let mut app = setup();
            app.add_systems(Update, handle_player_killed_system);
            app.add_message::<PlayerKilledMessage>();

            let res = get_resource_or_fail::<LivesResource>(&mut app);
            assert_eq!(res.0.get_current(), 3);

            let projectile = app.world_mut().spawn(PlayerProjectileComponent).id();

            send_message(&mut app, PlayerKilledMessage::new(projectile));

            app.update();

            let res = get_resource_or_fail::<LivesResource>(&mut app);
            assert_eq!(res.0.get_current(), 2);
        }

        #[test]
        fn should_render_the_updated_lives() {
            let mut app = setup();
            app.add_systems(Startup, spawn_lives_system);
            app.add_systems(Update, update_lives_system);
            app.update();

            let mut query = app
                .world_mut()
                .query::<(&LivesValueComponent, &ImageNode)>();
            let lives_image_count = query.iter(app.world()).count();

            assert_eq!(lives_image_count, 3);

            let mut res = get_resource_mut_or_fail::<LivesResource>(&mut app);
            res.0.decrement();

            app.update();

            let mut query = app
                .world_mut()
                .query::<(&LivesValueComponent, &ImageNode)>();
            let lives_image_count = query.iter(app.world()).count();

            assert_eq!(lives_image_count, 2);
        }
    }
}
