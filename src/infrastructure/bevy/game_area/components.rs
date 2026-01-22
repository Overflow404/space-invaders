use crate::infrastructure::bevy::game_area::resources::BACKGROUND_IMAGE;
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, Sprite, Transform, default};

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
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy_test::TestAppBuilder;

    #[test]
    fn spawning_game_area_creates_background_with_specified_dimensions() {
        let mut app = TestAppBuilder::new().build();
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>();

        let asset_server = app.world().resource::<AssetServer>().clone();
        let width = 800.0;
        let height = 600.0;

        app.world_mut()
            .spawn(GameAreaBundle::new(&asset_server, width, height));

        let mut query = app
            .world_mut()
            .query::<(&GameAreaComponent, &Transform, &Sprite)>();
        let (game_area, transform, sprite) = query.single(app.world()).expect("GameArea not found");

        assert_eq!(*game_area, GameAreaComponent);
        assert_eq!(transform.translation.z, -1.0);
        assert_eq!(sprite.custom_size, Some(Vec2::new(width, height)));
    }
}
