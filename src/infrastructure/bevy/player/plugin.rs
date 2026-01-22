use crate::domain::player::Player;
use crate::infrastructure::bevy::player::resources::PlayerResource;
use crate::infrastructure::bevy::player::systems::{
    on_enemy_projectile_hitting_player_system, player_fire_system, player_movement_system,
    reload_player_weapon_system, respawn_player_system, spawn_player_system,
};
use bevy::app::{App, Plugin, Startup, Update};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerResource(Player::new()))
            .add_systems(Startup, spawn_player_system)
            .add_systems(
                Update,
                (
                    player_movement_system,
                    player_fire_system,
                    reload_player_weapon_system,
                    respawn_player_system,
                    on_enemy_projectile_hitting_player_system,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
    use crate::infrastructure::bevy::enemy_projectile::components::PlayerKilledMessage;
    use crate::infrastructure::bevy::player_projectile::plugin::PlayerProjectilePlugin;
    use bevy_test::TestAppBuilder;

    #[test]
    fn plugin_loads_successfully() {
        let _app = TestAppBuilder::new()
            .with_assets()
            .with_input()
            .with_plugin(PlayerProjectilePlugin)
            .with_plugin(PlayerPlugin)
            .with_message::<EnemyKilledMessage>()
            .with_message::<PlayerKilledMessage>()
            .build();
    }
}
