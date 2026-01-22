use crate::infrastructure::bevy::enemy::resources::{
    ENEMY_COLOR, ENEMY_HEIGHT, ENEMY_IMAGE, ENEMY_WIDTH,
};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, Entity, Message, Sprite, Transform, default};

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
    use bevy::asset::AssetServer;
    use bevy_test::TestAppBuilder;

    #[test]
    fn spawning_enemy_creates_entity_at_correct_position() {
        let mut app = TestAppBuilder::new().with_assets().build();

        let asset_server = app.world().resource::<AssetServer>().clone();

        let expected_id = 99;
        let expected_x = 100.0;
        let expected_y = 200.0;

        app.world_mut().spawn(EnemyBundle::new(
            expected_id,
            expected_x,
            expected_y,
            &asset_server,
        ));

        let mut query = app
            .world_mut()
            .query::<(&EnemyComponent, &Transform, &Sprite)>();
        let (enemy, transform, sprite) = query.single(app.world()).expect("Enemy not found");

        assert_eq!(enemy.id, expected_id);
        assert_eq!(transform.translation.x, expected_x);
        assert_eq!(transform.translation.y, expected_y);
        assert_eq!(
            sprite.custom_size,
            Some(Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT))
        );
        assert_eq!(sprite.color, ENEMY_COLOR);
    }
}
