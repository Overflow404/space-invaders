use crate::infrastructure::bevy::enemy_projectile::components::EnemyProjectileExpiredMessage;
use crate::infrastructure::bevy::enemy_projectile::resources::{
    EnemyProjectileMovementTimerResource, ENEMY_PROJECTILE_DURATION,
};
use crate::infrastructure::bevy::enemy_projectile::systems::{
    enemy_projectile_lifecycle_system, enemy_projectile_movement_system,
};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Timer, TimerMode};

pub struct EnemyProjectilePlugin;

impl Plugin for EnemyProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyProjectileMovementTimerResource(Timer::from_seconds(
            ENEMY_PROJECTILE_DURATION,
            TimerMode::Once,
        )))
        .add_systems(
            Update,
            (
                enemy_projectile_movement_system,
                enemy_projectile_lifecycle_system,
            ),
        )
        .add_message::<EnemyProjectileExpiredMessage>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy_projectile::resources::EnemyProjectileMovementTimerResource;
    use bevy::prelude::TimerMode;
    use bevy_test::{contains_system_or_fail, get_resource_or_fail, minimal_app};

    #[test]
    fn should_initialize_the_plugin() {
        let mut app = minimal_app(false);

        app.add_plugins(EnemyProjectilePlugin);

        let timer = get_resource_or_fail::<EnemyProjectileMovementTimerResource>(&mut app);
        assert_eq!(timer.0.duration().as_millis(), 1200);
        assert_eq!(timer.0.mode(), TimerMode::Once);

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
