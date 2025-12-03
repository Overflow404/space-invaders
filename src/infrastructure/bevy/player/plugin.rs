use crate::domain::player::Player;
use crate::infrastructure::bevy::player::resources::PlayerResource;
use crate::infrastructure::bevy::player::systems::{player_fire_system, player_movement_system, reload_player_weapon_system, respawn_player_system, spawn_player_system};
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
                    respawn_player_system
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
    use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileExpiredMessage;
    use crate::infrastructure::bevy::player_projectile::plugin::PlayerProjectilePlugin;
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::input::ButtonInput;
    use bevy::prelude::{AssetApp, KeyCode};
    use bevy_test::{contains_system_or_fail, get_resource_or_fail, minimal_app};
    use crate::infrastructure::bevy::enemy_projectile::components::PlayerKilledMessage;

    fn setup() -> App {
        let mut app = minimal_app(false);
        app.add_plugins(AssetPlugin::default())
            .add_plugins(PlayerPlugin)
            .add_plugins(PlayerProjectilePlugin)
            .add_message::<EnemyKilledMessage>()
            .add_message::<PlayerProjectileExpiredMessage>()
            .add_message::<PlayerKilledMessage>()
            .init_asset::<Image>()
            .init_resource::<ButtonInput<KeyCode>>();

        app.update();
        app
    }

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = setup();

        get_resource_or_fail::<PlayerResource>(&mut app);
        assert!(contains_system_or_fail(
            &app,
            Startup,
            "spawn_player_system"
        ));
        assert!(contains_system_or_fail(
            &app,
            Update,
            "player_movement_system"
        ));
        assert!(contains_system_or_fail(&app, Update, "player_fire_system"));
        assert!(contains_system_or_fail(&app, Update, "respawn_player_system"));
        assert!(contains_system_or_fail(
            &app,
            Update,
            "reload_player_weapon_system"
        ));
    }
}
