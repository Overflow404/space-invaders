use crate::domain::lives::Lives;
use crate::infrastructure::bevy::header::systems::spawn_header_system;
use crate::infrastructure::bevy::lives::resources::LivesResource;
use crate::infrastructure::bevy::lives::systems::{
    handle_player_killed_system, spawn_lives_system, update_lives_system,
};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::IntoScheduleConfigs;

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LivesResource(Lives::new()))
            .add_systems(Startup, spawn_lives_system.after(spawn_header_system))
            .add_systems(Update, handle_player_killed_system)
            .add_systems(Update, update_lives_system.after(spawn_lives_system));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy_projectile::components::PlayerKilledMessage;
    use crate::infrastructure::bevy::header::plugin::HeaderPlugin;
    use bevy_test::TestAppBuilder;

    #[test]
    fn plugin_loads_successfully() {
        let _app = TestAppBuilder::new()
            .with_assets()
            .with_plugin(HeaderPlugin)
            .with_plugin(LivesPlugin)
            .with_message::<PlayerKilledMessage>()
            .build();
    }
}
