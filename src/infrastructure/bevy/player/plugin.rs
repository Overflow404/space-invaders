use crate::domain::player::Player;
use crate::infrastructure::bevy::player::resources::PlayerResource;
use crate::infrastructure::bevy::player::systems::{
    player_fire_system, player_movement_system, spawn_player_system,
    sync_player_firing_state_system,
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
                    sync_player_firing_state_system,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
    use crate::infrastructure::bevy::player::components::PlayerComponent;
    use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileExpiredMessage;
    use crate::infrastructure::bevy::player_projectile::plugin::PlayerProjectilePlugin;
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::input::ButtonInput;
    use bevy::prelude::{AssetApp, KeyCode};
    use bevy::MinimalPlugins;
    use bevy_test::{count_components, get_resource, get_update_systems};

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .add_plugins(PlayerPlugin)
            .add_plugins(PlayerProjectilePlugin)
            .add_message::<EnemyKilledMessage>()
            .add_message::<PlayerProjectileExpiredMessage>()
            .init_asset::<Image>()
            .init_resource::<ButtonInput<KeyCode>>();

        app.update();
        app
    }

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = setup();

        let player_resource = get_resource::<PlayerResource>(&mut app);
        assert!(!player_resource.0.is_firing());

        assert_eq!(get_update_systems(&app).count(), 5);

        let player_count = count_components::<PlayerComponent>(&mut app);
        assert_eq!(player_count, 1);
    }
}
