use crate::infrastructure::bevy::enemy_projectile::resources::{
    ENEMY_PROJECTILE_COLOR, ENEMY_PROJECTILE_DURATION, ENEMY_PROJECTILE_HEIGHT,
    ENEMY_PROJECTILE_WIDTH,
};
use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Component, Entity, Message, Sprite, TimerMode, Transform};
use bevy::time::Timer;

#[derive(Bundle)]
pub struct EnemyProjectileBundle {
    pub projectile: EnemyProjectileComponent,
    pub sprite: Sprite,
    pub transform: Transform,
    pub timer: EnemyProjectileTimer,
}

#[derive(Component)]
pub struct EnemyProjectileTimer(pub Timer);

impl EnemyProjectileTimer {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
}

#[derive(Message)]
pub struct EnemyProjectileExpiredMessage;

#[derive(Message)]
pub struct PlayerKilledMessage {
    pub projectile_entity: Entity,
}

impl PlayerKilledMessage {
    pub fn new(projectile_entity: Entity) -> Self {
        PlayerKilledMessage { projectile_entity }
    }
}

#[derive(Component, PartialEq, Debug)]
pub struct EnemyProjectileComponent;

impl EnemyProjectileBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            projectile: EnemyProjectileComponent,
            sprite: Sprite {
                color: ENEMY_PROJECTILE_COLOR,
                custom_size: Some(Vec2::new(ENEMY_PROJECTILE_WIDTH, ENEMY_PROJECTILE_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            timer: EnemyProjectileTimer::new(Timer::from_seconds(
                ENEMY_PROJECTILE_DURATION,
                TimerMode::Repeating,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_test::TestAppBuilder;

    #[test]
    fn spawning_enemy_projectile_creates_entity_at_specified_position() {
        let mut app = TestAppBuilder::new().build();

        let start_x = 100.0;
        let start_y = 200.0;

        app.world_mut()
            .spawn(EnemyProjectileBundle::new(start_x, start_y));

        let mut query = app
            .world_mut()
            .query::<(&EnemyProjectileComponent, &Transform, &Sprite, &EnemyProjectileTimer)>();
        let (projectile, transform, sprite, timer) = query
            .single(app.world())
            .expect("EnemyProjectile not found");

        assert_eq!(*projectile, EnemyProjectileComponent);
        assert_eq!(transform.translation.x, start_x);
        assert_eq!(transform.translation.y, start_y);
        assert_eq!(
            sprite.custom_size,
            Some(Vec2::new(ENEMY_PROJECTILE_WIDTH, ENEMY_PROJECTILE_HEIGHT))
        );
        assert_eq!(sprite.color, ENEMY_PROJECTILE_COLOR);
        assert_eq!(timer.0.mode(), TimerMode::Repeating);
    }
}
