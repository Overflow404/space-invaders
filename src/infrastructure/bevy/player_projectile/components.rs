use crate::infrastructure::bevy::player_projectile::resources::{
    PLAYER_PROJECTILE_COLOR, PROJECTILE_HEIGHT, PROJECTILE_WIDTH,
};
use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Sprite, Transform};
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
                custom_size: Some(Vec2::new(PROJECTILE_WIDTH, PROJECTILE_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::bevy::player_projectile::components::{
        PlayerProjectileBundle, PlayerProjectileComponent,
    };
    use bevy::color::Color;
    use bevy::math::Vec2;

    #[test]
    fn should_create_the_bundle() {
        let start_x = 100.0;
        let start_y = 50.0;

        let bundle = PlayerProjectileBundle::new(start_x, start_y);

        assert_eq!(bundle.projectile, PlayerProjectileComponent);

        assert_eq!(bundle.transform.translation.x, start_x);
        assert_eq!(bundle.transform.translation.y, start_y);
        assert_eq!(bundle.transform.translation.z, 0f32);

        assert_eq!(bundle.sprite.color, Color::srgb(1.0, 1.0, 1.0));
        assert_eq!(bundle.sprite.custom_size, Some(Vec2::new(5.0, 15.0)),);
    }
}
