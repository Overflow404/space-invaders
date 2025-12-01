use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
use crate::infrastructure::bevy::header::components::HeaderComponent;
use crate::infrastructure::bevy::header::resources::FONT;
use crate::infrastructure::bevy::score::components::{
    ScoreContainerBundle, ScoreLabelBundle, ScoreLabelComponent, ScoreValueBundle,
    ScoreValueComponent,
};
use crate::infrastructure::bevy::score::resources::ScoreResource;
use bevy::asset::AssetServer;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::prelude::{DetectChanges, MessageReader, ResMut};
use bevy::ui::widget::Text;

pub fn update_score_text_system(
    score_resource: Res<ScoreResource>,
    mut score_query: Query<&mut Text, With<ScoreValueComponent>>,
) {
    if score_resource.is_changed() {
        for mut text in &mut score_query {
            text.0 = score_resource.0.get_current().to_string();
        }
    }
}

pub fn handle_enemy_killed_system(
    mut enemy_killed_message: MessageReader<EnemyKilledMessage>,
    mut score_resource: ResMut<ScoreResource>,
) {
    for _ in enemy_killed_message.read() {
        score_resource.0.increment(10);
    }
}

pub fn spawn_score_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score_res: Res<ScoreResource>,
    header_query: Query<Entity, With<HeaderComponent>>,
) {
    if let Ok(header) = header_query.single() {
        let font = asset_server.load(FONT);
        let current_score = score_res.0.get_current();

        commands.entity(header).with_children(|parent| {
            parent
                .spawn(ScoreContainerBundle::new())
                .with_children(|score_section| {
                    score_section.spawn(ScoreLabelBundle::new(font.clone()));
                    score_section.spawn(ScoreValueBundle::new(font, current_score));
                });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::score::Score;
    use crate::infrastructure::bevy::header::systems::spawn_header_system;
    use crate::infrastructure::bevy::score::resources::ScoreResource;
    use bevy::app::{App, Startup, Update};
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::prelude::With;
    use bevy::text::Font;
    use bevy::MinimalPlugins;
    use bevy_test::contains_component;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .add_systems(Startup, spawn_header_system)
            .insert_resource(ScoreResource(Score::new()))
            .init_asset::<Image>()
            .init_asset::<Font>();

        app.update();

        app
    }

    #[cfg(test)]
    mod should_spawn_score_system {
        use super::*;
        use bevy_test::run_system;

        #[test]
        fn should_spawn_score_components() {
            let mut app = setup();

            run_system(&mut app, spawn_score_system).expect("System should run");

            assert!(contains_component::<ScoreLabelComponent>(&mut app));
            assert!(contains_component::<ScoreValueComponent>(&mut app));
        }
    }

    #[cfg(test)]
    mod update_score_text_system {
        use super::*;
        use bevy_test::{dummy_font, get_resource_mut, run_system};
        use std::error::Error;

        #[test]
        fn should_update_score_text() -> Result<(), Box<dyn Error>> {
            let mut app = setup();
            app.add_systems(Startup, spawn_score_system);

            let font = dummy_font(&app);

            app.world_mut().spawn(ScoreValueBundle::new(font, 0));

            let mut score_resource = get_resource_mut::<ScoreResource>(&mut app);
            score_resource.0.increment(50);

            run_system(&mut app, update_score_text_system).expect("System should run");

            app.update();

            let mut score_view_value_query = app
                .world_mut()
                .query_filtered::<&Text, With<ScoreValueComponent>>();

            let score_value = score_view_value_query.single(app.world())?;

            assert_eq!(score_value.0, String::from("50"));

            Ok(())
        }
    }

    #[cfg(test)]
    mod handle_enemy_killed_system {
        use super::*;
        use bevy_test::{get_resource, send_message, spawn_dummy_entity};

        #[test]
        fn should_increase_score_when_enemy_is_killed() {
            let mut app = setup();
            app.add_message::<EnemyKilledMessage>()
                .add_systems(Update, handle_enemy_killed_system);

            let enemy_entity = spawn_dummy_entity(&mut app);
            let player_projectile_entity = spawn_dummy_entity(&mut app);

            send_message(
                &mut app,
                EnemyKilledMessage::new(enemy_entity, 1, player_projectile_entity),
            );

            app.update();

            let post_update_score_resource = get_resource::<ScoreResource>(&mut app);

            assert_eq!(post_update_score_resource.0.get_current(), 10);
        }
    }
}
