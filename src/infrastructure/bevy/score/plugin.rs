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
    use bevy_test::TestAppBuilder;

    #[test]
    fn plugin_loads_successfully() {
        let _app = TestAppBuilder::new()
            .with_assets()
            .with_plugin(HeaderPlugin)
            .with_plugin(ScorePlugin)
            .with_message::<EnemyKilledMessage>()
            .build();
    }
}
