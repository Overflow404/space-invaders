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
    use bevy::MinimalPlugins;
    use bevy_test::get_update_systems;

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, EnemyProjectilePlugin));

        app.update();

        assert_eq!(get_update_systems(&app).count(), 1);
    }
}
