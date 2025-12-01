use crate::infrastructure::bevy::game_area::resources::BACKGROUND_IMAGE;
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Component, Sprite, Transform};

#[derive(Component, PartialEq, Debug)]
pub struct GameAreaComponent;

#[derive(Bundle)]
pub struct GameAreaBundle {
    pub game_area: GameAreaComponent,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl GameAreaBundle {
    pub fn new(asset_server: &AssetServer, width: f32, height: f32) -> Self {
        Self {
            game_area: GameAreaComponent,
            sprite: Sprite {
                image: asset_server.load(BACKGROUND_IMAGE),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::{AssetApp, AssetPlugin, Handle};
    use bevy::image::Image;
    use bevy_test::minimal_app;

    #[test]
    fn should_create_the_bundle() {
        let mut app = minimal_app(false);
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>();

        let asset_server = app.world().resource::<AssetServer>().clone();
        let width = 800.0;
        let height = 600.0;

        let bundle = GameAreaBundle::new(&asset_server, width, height);

        assert_eq!(bundle.game_area, GameAreaComponent);

        assert_eq!(bundle.transform.translation.x, 0.0);
        assert_eq!(bundle.transform.translation.y, 0.0);
        assert_eq!(bundle.transform.translation.z, -1.0);

        assert!(matches!(bundle.sprite.image, Handle::Strong(_)));
        assert_eq!(bundle.sprite.custom_size, Some(Vec2::new(width, height)));
    }
}
