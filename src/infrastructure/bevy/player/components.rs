use crate::infrastructure::bevy::player::resources::{
    PLAYER_HEIGHT, PLAYER_IMAGE, PLAYER_WIDTH, PLAYER_X, PLAYER_Y,
};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Component, Sprite, Transform};

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
    use bevy::asset::{AssetPlugin, AssetServer, Handle};
    use bevy::image::Image;
    use bevy::prelude::*;
    use bevy_test::{get_resource_or_fail, minimal_app};

    fn setup() -> App {
        let mut app = minimal_app(false);
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>();

        app
    }

    #[test]
    fn should_create_the_bundle() {
        let mut app = setup();

        let asset_server = get_resource_or_fail::<AssetServer>(&mut app);

        let bundle = PlayerBundle::new(asset_server);

        assert_eq!(bundle.player, PlayerComponent);
        assert_eq!(bundle.transform.translation.x, PLAYER_X);
        assert_eq!(bundle.transform.translation.y, PLAYER_Y);
        assert_eq!(bundle.transform.translation.z, 0.0);

        assert_eq!(
            bundle.sprite.custom_size,
            Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT))
        );

        assert!(matches!(bundle.sprite.image, Handle::Strong(_)));
    }
}
