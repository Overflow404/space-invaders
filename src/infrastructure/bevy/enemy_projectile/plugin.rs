use crate::infrastructure::bevy::enemy_projectile::systems::enemy_projectile_movement_system;
use bevy::app::{App, Plugin, Update};

pub struct EnemyProjectilePlugin;

impl Plugin for EnemyProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_projectile_movement_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_test::{contains_system, minimal_app};

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = minimal_app();

        app.add_plugins(EnemyProjectilePlugin);

        app.update();

        assert!(contains_system(
            &app,
            Update,
            "enemy_projectile_movement_system"
        ));
    }
}
