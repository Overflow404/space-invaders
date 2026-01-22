use crate::infrastructure::bevy::player::resources::{
    PLAYER_HEIGHT, PLAYER_IMAGE, PLAYER_WIDTH, PLAYER_X, PLAYER_Y,
};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, Sprite, Transform, default};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: PlayerComponent,
    pub sprite: Sprite,
    pub transform: Transform,
}

#[derive(Component, PartialEq, Debug)]
pub struct PlayerComponent;

impl PlayerBundle {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            player: PlayerComponent,
            sprite: Sprite {
                image: asset_server.load(PLAYER_IMAGE),
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(PLAYER_X, PLAYER_Y, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::prelude::*;
    use bevy_test::TestAppBuilder;

    fn setup() -> App {
        let mut app = TestAppBuilder::new().build();
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>();

        app
    }

    #[test]
    fn spawning_player_creates_entity_with_correct_components() {
        let mut app = setup();

        let asset_server = app.world().resource::<AssetServer>().clone();
        app.world_mut().spawn(PlayerBundle::new(&asset_server));

        let mut query = app
            .world_mut()
            .query::<(&PlayerComponent, &Transform, &Sprite)>();
        let (player, transform, sprite) = query.single(app.world()).expect("Player not found");

        assert_eq!(*player, PlayerComponent);
        assert_eq!(transform.translation.x, PLAYER_X);
        assert_eq!(transform.translation.y, PLAYER_Y);
        assert_eq!(
            sprite.custom_size,
            Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT))
        );
    }
}
