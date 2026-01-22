use crate::infrastructure::bevy::enemy_projectile::components::{
    EnemyProjectileExpiredMessage, PlayerKilledMessage,
};
use crate::infrastructure::bevy::enemy_projectile::systems::{
    collision_system, enemy_projectile_lifecycle_system, enemy_projectile_movement_system,
    on_enemy_projectile_hitting_player_system,
};
use bevy::app::{App, Plugin, Update};

pub struct EnemyProjectilePlugin;

impl Plugin for EnemyProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                collision_system,
                enemy_projectile_movement_system,
                enemy_projectile_lifecycle_system,
                on_enemy_projectile_hitting_player_system,
            ),
        )
        .add_message::<EnemyProjectileExpiredMessage>()
        .add_message::<PlayerKilledMessage>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_loads_successfully() {
        let _app = bevy_test::smoke_test_plugin(EnemyProjectilePlugin);
    }
}
