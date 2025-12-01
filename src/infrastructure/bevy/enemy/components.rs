use crate::infrastructure::bevy::enemy::resources::{
    ENEMY_COLOR, ENEMY_HEIGHT, ENEMY_IMAGE, ENEMY_WIDTH,
};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Component, Entity, Message, Sprite, Transform};

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

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: EnemyComponent,
    pub sprite: Sprite,
    pub transform: Transform,
}

#[derive(Component, Clone, PartialEq, Debug)]
pub struct EnemyComponent {
    pub id: usize,
}

impl EnemyComponent {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl EnemyBundle {
    pub fn new(id: usize, x: f32, y: f32, asset_server: &AssetServer) -> Self {
        Self {
            enemy: EnemyComponent::new(id),
            sprite: Sprite {
                image: asset_server.load(ENEMY_IMAGE),
                custom_size: Some(Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT)),
                color: ENEMY_COLOR,
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::{AssetPlugin, AssetServer, Handle};
    use bevy::image::Image;
    use bevy::prelude::*;
    use bevy_test::{get_resource_or_fail, minimal_app};

    #[test]
    fn should_create_the_enemy_bundle() {
        let mut app = minimal_app();

        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>();

        let asset_server = get_resource_or_fail::<AssetServer>(&mut app);

        let expected_id = 99;
        let expected_x = 100.0;
        let expected_y = 200.0;

        let bundle = EnemyBundle::new(expected_id, expected_x, expected_y, asset_server);

        assert_eq!(bundle.enemy, EnemyComponent::new(expected_id));
        assert_eq!(bundle.transform.translation.x, expected_x);
        assert_eq!(bundle.transform.translation.y, expected_y);
        assert_eq!(bundle.transform.translation.z, 0.0);

        assert_eq!(
            bundle.sprite.custom_size,
            Some(Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT))
        );

        assert_eq!(bundle.sprite.color, ENEMY_COLOR);
        assert!(matches!(bundle.sprite.image, Handle::Strong(_)));
    }
}
