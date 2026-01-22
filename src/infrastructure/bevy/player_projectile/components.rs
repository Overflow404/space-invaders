use crate::infrastructure::bevy::player_projectile::resources::{
    PLAYER_PROJECTILE_COLOR, PLAYER_PROJECTILE_HEIGHT, PLAYER_PROJECTILE_WIDTH,
};
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Sprite, Transform, default};
use bevy::prelude::{Component, Message};

#[derive(Message)]
pub struct PlayerProjectileExpiredMessage;

#[derive(Bundle)]
pub struct PlayerProjectileBundle {
    pub projectile: PlayerProjectileComponent,
    pub sprite: Sprite,
    pub transform: Transform,
}

#[derive(Component, PartialEq, Debug)]
pub struct PlayerProjectileComponent;

impl PlayerProjectileBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            projectile: PlayerProjectileComponent,
            sprite: Sprite {
                color: PLAYER_PROJECTILE_COLOR,
                custom_size: Some(Vec2::new(PLAYER_PROJECTILE_WIDTH, PLAYER_PROJECTILE_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_test::TestAppBuilder;

    #[test]
    fn spawning_projectile_creates_entity_at_specified_position() {
        let mut app = TestAppBuilder::new().build();

        let start_x = 100.0;
        let start_y = 50.0;

        app.world_mut()
            .spawn(PlayerProjectileBundle::new(start_x, start_y));

        let mut query = app
            .world_mut()
            .query::<(&PlayerProjectileComponent, &Transform, &Sprite)>();
        let (projectile, transform, sprite) = query
            .single(app.world())
            .expect("PlayerProjectile not found");

        assert_eq!(*projectile, PlayerProjectileComponent);
        assert_eq!(transform.translation.x, start_x);
        assert_eq!(transform.translation.y, start_y);
        assert_eq!(
            sprite.custom_size,
            Some(Vec2::new(PLAYER_PROJECTILE_WIDTH, PLAYER_PROJECTILE_HEIGHT))
        );
        assert_eq!(sprite.color, PLAYER_PROJECTILE_COLOR);
    }
}
