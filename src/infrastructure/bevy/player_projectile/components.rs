use crate::infrastructure::bevy::player_projectile::resources::PLAYER_PROJECTILE_COLOR;
use bevy::math::Vec2;
use bevy::math::Vec3;
use bevy::prelude::Component;
use bevy::prelude::{default, Bundle, Sprite, Transform};
use bevy::prelude::{Entity, Message};

#[derive(Message)]
pub struct DespawnPlayerProjectileMessage(pub Entity);

#[derive(Bundle)]
pub struct PlayerProjectileBundle {
    pub projectile: PlayerProjectileComponent,
    pub sprite: Sprite,
    pub transform: Transform,
}

#[derive(Component)]
pub struct PlayerProjectileComponent {
    start_position: Vec3,
}

impl PlayerProjectileBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            projectile: PlayerProjectileComponent::new(x, y),
            sprite: Sprite {
                color: PLAYER_PROJECTILE_COLOR,
                custom_size: Some(Vec2::new(5.0, 15.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
        }
    }
}

impl PlayerProjectileComponent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            start_position: Vec3::new(x, y, 0.0),
        }
    }

    pub fn get_position(&self) -> Vec3 {
        self.start_position
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::bevy::player_projectile::components::PlayerProjectileBundle;
    use crate::infrastructure::bevy::player_projectile::resources::{
        PROJECTILE_HEIGHT, PROJECTILE_WIDTH,
    };
    use bevy::color::Color;
    use bevy::math::{Vec2, Vec3};

    #[test]
    fn should_create_the_player_projectile_bundle() {
        let start_x = 100.0;
        let start_y = 50.0;

        let bundle = PlayerProjectileBundle::new(start_x, start_y);

        assert_eq!(bundle.transform.translation.x, start_x);
        assert_eq!(bundle.transform.translation.y, start_y);
        assert_eq!(
            bundle.sprite.custom_size,
            Some(Vec2::new(PROJECTILE_WIDTH, PROJECTILE_HEIGHT))
        );
        assert_eq!(bundle.sprite.color, Color::srgb(1.0, 1.0, 1.0));
        assert_eq!(
            bundle.projectile.get_position(),
            Vec3::new(start_x, start_y, 0.0)
        );
    }
}
