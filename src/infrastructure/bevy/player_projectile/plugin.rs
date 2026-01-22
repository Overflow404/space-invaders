use crate::infrastructure::bevy::enemy_formation::systems::collisions_system;
use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileExpiredMessage;
use crate::infrastructure::bevy::player_projectile::resources::{
    PLAYER_PROJECTILE_DURATION, PlayerProjectileMovementTimerResource,
};
use crate::infrastructure::bevy::player_projectile::systems::{
    player_projectile_lifecycle_system, player_projectile_movement_system,
};
use bevy::app::{App, Plugin};
use bevy::prelude::{IntoScheduleConfigs, Timer, TimerMode, Update};

pub struct PlayerProjectilePlugin;

impl Plugin for PlayerProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerProjectileMovementTimerResource(Timer::from_seconds(
            PLAYER_PROJECTILE_DURATION,
            TimerMode::Once,
        )))
        .add_message::<PlayerProjectileExpiredMessage>()
        .add_systems(
            Update,
            (
                player_projectile_movement_system.after(collisions_system),
                player_projectile_lifecycle_system.after(collisions_system),
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy::components::EnemyKilledMessage;
    use bevy_test::TestAppBuilder;

    #[test]
    fn plugin_loads_successfully() {
        let _app = TestAppBuilder::new()
            .with_plugin(PlayerProjectilePlugin)
            .with_message::<EnemyKilledMessage>()
            .build();
    }
}
