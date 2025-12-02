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
    use bevy::color::Color;

    #[test]
    fn should_create_the_bundle() {
        let start_x = 100.0;
        let start_y = 200.0;

        let bundle = EnemyProjectileBundle::new(start_x, start_y);

        assert_eq!(bundle.projectile, EnemyProjectileComponent);

        assert_eq!(bundle.transform.translation.x, start_x);
        assert_eq!(bundle.transform.translation.y, start_y);
        assert_eq!(bundle.transform.translation.z, 0f32);

        assert_eq!(
            bundle.sprite.custom_size,
            Some(Vec2::new(ENEMY_PROJECTILE_WIDTH, ENEMY_PROJECTILE_HEIGHT)),
        );

        assert_eq!(bundle.sprite.color, Color::srgb(1.0, 1.0, 1.0));
        //TODO timer tests
    }
}
