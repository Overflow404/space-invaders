use crate::domain::score::Score;
use crate::infrastructure::bevy::header::systems::spawn_header_system;
use crate::infrastructure::bevy::score::resources::ScoreResource;
use crate::infrastructure::bevy::score::systems::{
    handle_enemy_killed_system, spawn_score_system, update_score_text_system,
};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::IntoScheduleConfigs;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScoreResource(Score::new()))
            .add_systems(Startup, spawn_score_system.after(spawn_header_system))
            .add_systems(
                Update,
                (update_score_text_system, handle_enemy_killed_system),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
    use crate::infrastructure::bevy::header::plugin::HeaderPlugin;
    use crate::infrastructure::bevy::score::components::ScoreValueComponent;
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::prelude::AssetApp;
    use bevy::text::Font;
    use bevy_test::{contains_component, contains_system, get_resource_or_fail, minimal_app};

    fn setup() -> App {
        let mut app = minimal_app(false);
        app.add_plugins(AssetPlugin::default())
            .add_plugins(HeaderPlugin)
            .add_plugins(ScorePlugin)
            .add_message::<EnemyKilledMessage>()
            .init_asset::<Image>()
            .init_asset::<Font>();

        app.update();
        app
    }

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = setup();

        let score_resource = get_resource_or_fail::<ScoreResource>(&mut app);
        assert_eq!(score_resource.0.get_current(), 0);

        assert!(contains_system(&app, Startup, "spawn_score_system"));
        assert!(contains_system(&app, Update, "update_score_text_system"));
        assert!(contains_system(&app, Update, "handle_enemy_killed_system"));

        assert!(contains_component::<ScoreValueComponent>(&mut app));
    }
}
