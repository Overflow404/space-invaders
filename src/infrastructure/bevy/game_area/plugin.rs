use crate::infrastructure::bevy::game_area::systems::{
    resize_game_area_system, spawn_game_area_system,
};
use bevy::app::{App, Plugin, PostUpdate, Startup};

pub struct GameAreaPlugin;

impl Plugin for GameAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_area_system)
            .add_systems(PostUpdate, resize_game_area_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_loads_successfully() {
        let _app = bevy_test::smoke_test_plugin_with_assets(GameAreaPlugin);
    }
}
