use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
use crate::infrastructure::bevy::header::components::HeaderComponent;
use crate::infrastructure::bevy::header::resources::FONT;
use crate::infrastructure::bevy::score::components::{
    ScoreContainerBundle, ScoreLabelBundle, ScoreValueBundle, ScoreValueComponent,
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
    use crate::infrastructure::bevy::header::components::HeaderComponent;
    use crate::infrastructure::bevy::score::resources::ScoreResource;
    use bevy::app::{App, Startup, Update};
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::prelude::{AssetApp, Text};
    use bevy::text::Font;
    use bevy_test::{
        contains_component, get_resource_mut, get_resource_or_fail, minimal_app, send_message,
        spawn_dummy_entity,
    };

    fn setup() -> App {
        let mut app = minimal_app(false);
        app.add_plugins(AssetPlugin::default())
            .insert_resource(ScoreResource(Score::new()))
            .init_asset::<Image>()
            .init_asset::<Font>();
        app
    }

    #[cfg(test)]
    mod spawn_score_system {
        use super::*;
        use crate::infrastructure::bevy::score::components::ScoreLabelComponent;

        #[test]
        fn should_spawn_score_components() {
            let mut app = setup();
            app.world_mut().spawn(HeaderComponent);
            app.add_systems(Startup, spawn_score_system);

            app.update();

            assert!(contains_component::<ScoreLabelComponent>(&mut app));
            assert!(contains_component::<ScoreValueComponent>(&mut app));
        }
    }

    #[cfg(test)]
    mod update_score_text_system {
        use super::*;

        #[test]
        fn should_update_score_text() {
            let mut app = setup();
            app.add_systems(Update, update_score_text_system);

            app.world_mut().spawn((ScoreValueComponent, Text::new("0")));

            get_resource_mut::<ScoreResource>(&mut app).0.increment(50);

            app.update();

            let text = app
                .world_mut()
                .query_filtered::<&Text, With<ScoreValueComponent>>()
                .single(app.world())
                .expect("Score value text not found");

            assert_eq!(text.0, "50");
        }
    }

    #[cfg(test)]
    mod handle_enemy_killed_system {
        use super::*;

        #[test]
        fn should_increase_score_when_enemy_is_killed() {
            let mut app = setup();
            app.add_message::<EnemyKilledMessage>()
                .add_systems(Update, handle_enemy_killed_system);

            let dummy = spawn_dummy_entity(&mut app);
            send_message(&mut app, EnemyKilledMessage::new(dummy, 1, dummy));

            app.update();

            let res = get_resource_or_fail::<ScoreResource>(&mut app);
            assert_eq!(res.0.get_current(), 10);
        }
    }
}
