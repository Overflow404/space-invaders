use crate::infrastructure::bevy::enemy_projectile::components::{
    EnemyProjectileExpiredMessage, PlayerKilledMessage,
};
use crate::infrastructure::bevy::enemy_projectile::systems::{
    collision_system, enemy_projectile_lifecycle_system, enemy_projectile_movement_system,
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
            ),
        )
        .add_message::<EnemyProjectileExpiredMessage>()
        .add_message::<PlayerKilledMessage>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_test::{contains_system_or_fail, minimal_app};

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = minimal_app(false);

        app.add_plugins(EnemyProjectilePlugin);

        app.update();

        assert!(contains_system_or_fail(
            &app,
            Update,
            "enemy_projectile_movement_system"
        ));

        assert!(contains_system_or_fail(
            &app,
            Update,
            "enemy_projectile_lifecycle_system"
        ));
    }
}
