use crate::infrastructure::bevy::enemy_formation::EnemyFormationView;
use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileExpiredMessage;
use crate::infrastructure::bevy::player_projectile::resources::{
    PlayerProjectileMovementTimerResource, PLAYER_PROJECTILE_DURATION,
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
                player_projectile_movement_system.after(EnemyFormationView::handle_collisions),
                player_projectile_lifecycle_system.after(EnemyFormationView::handle_collisions),
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::enemy::EnemyKilledMessage;
    use bevy::MinimalPlugins;
    use bevy_test::{contains_message, get_resource, get_update_systems};

    fn setup() -> App {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, PlayerProjectilePlugin))
            .add_message::<EnemyKilledMessage>();

        app
    }
    #[test]
    fn should_initialize_the_plugin() {
        let mut app = setup();

        app.update();

        let timer = get_resource::<PlayerProjectileMovementTimerResource>(&mut app);

        assert_eq!(timer.0.duration().as_millis(), 1200);
        assert_eq!(timer.0.mode(), TimerMode::Once);

        assert_eq!(get_update_systems(&app).count(), 2);
        assert!(contains_message::<PlayerProjectileExpiredMessage>(&app));
    }
}
