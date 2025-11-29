use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{default, Component, Entity, Message, Resource, Timer, Transform};
use bevy::sprite::Sprite;

pub const ENEMY_WIDTH: f32 = 60.0;
pub const ENEMY_HEIGHT: f32 = 40.0;

const ENEMY_IMAGE: &str = "red.png";
#[derive(Component, Clone)]
pub struct EnemyView {
    pub id: usize,
}

#[derive(Resource)]
pub struct EnemyProjectileMovementTimer(pub Timer);

#[derive(Resource)]
pub struct EnemyFireProbability(pub f64);

#[derive(Message)]
pub struct EnemyKilledMessage {
    pub enemy_entity: Entity,
    pub enemy_id: usize,
    pub projectile_entity: Entity,
}

impl EnemyKilledMessage {
    pub fn new(enemy_entity: Entity, enemy_id: usize, projectile_entity: Entity) -> Self {
        EnemyKilledMessage {
            enemy_entity,
            enemy_id,
            projectile_entity,
        }
    }
}

impl EnemyView {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn make_enemy(
        id: usize,
        x: f32,
        y: f32,
        asset_server: &AssetServer,
    ) -> (Self, Sprite, Transform) {
        (
            EnemyView::new(id),
            Sprite {
                image: asset_server.load(ENEMY_IMAGE),
                custom_size: Some(Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT)),
                color: Color::srgb(255.0, 255.0, 255.0),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;
    use bevy::prelude::*;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.init_asset::<Image>();

        app
    }

    #[test]
    fn should_create_the_enemy_bundle() {
        let app = setup();

        let asset_server = app.world().resource::<AssetServer>();

        let expected_id = 99;
        let expected_x = 100.0;
        let expected_y = 200.0;

        let (view, sprite, transform) =
            EnemyView::make_enemy(expected_id, expected_x, expected_y, asset_server);

        assert_eq!(view.id, expected_id);
        assert_eq!(transform.translation.x, expected_x);
        assert_eq!(transform.translation.y, expected_y);
        assert_eq!(transform.translation.z, 0.0);

        assert_eq!(
            sprite.custom_size,
            Some(Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT))
        );

        assert_eq!(sprite.color, Color::srgb(255.0, 255.0, 255.0));
        assert!(matches!(sprite.image, Handle::Strong(_)));
    }
}
